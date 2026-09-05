use crate::error::io;
use crate::models::OperationJournal;
use crate::Result;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

pub fn journal_path(dir: &Path, id: uuid::Uuid) -> PathBuf {
    dir.join(format!("{id}.journal.json"))
}

pub fn save_journal(dir: &Path, journal: &OperationJournal) -> Result<PathBuf> {
    fs::create_dir_all(dir).map_err(|e| io(dir, e))?;
    let target = journal_path(dir, journal.id);
    let temp = target.with_extension("journal.json.tmp");
    let journal = normalize_journal_paths(journal)?;

    let file = File::create(&temp).map_err(|e| io(&temp, e))?;
    let mut writer = BufWriter::new(file);
    if let Err(error) = serde_json::to_writer_pretty(&mut writer, &journal) {
        drop(writer);
        let _ = fs::remove_file(&temp);
        return Err(error.into());
    }
    writer.flush().map_err(|e| io(&temp, e))?;
    writer.get_ref().sync_all().map_err(|e| io(&temp, e))?;
    drop(writer);

    replace_journal_target(&temp, &target)?;
    sync_journal_directory(dir)?;
    Ok(target)
}

fn normalize_journal_paths(journal: &OperationJournal) -> Result<OperationJournal> {
    let root = make_absolute(&journal.root)?;
    let entries = journal
        .entries
        .iter()
        .map(|entry| {
            Ok(crate::models::JournalEntry {
                operation_id: entry.operation_id,
                from: make_absolute(&entry.from)?,
                to: make_absolute(&entry.to)?,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(OperationJournal {
        id: journal.id,
        created_at: journal.created_at,
        root,
        entries,
    })
}

fn make_absolute(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        std::env::current_dir().map(|dir| dir.join(path)).map_err(|e| io(path, e))
    }
}

fn replace_journal_target(temp: &Path, target: &Path) -> Result<()> {
    match fs::rename(temp, target) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            fs::remove_file(target).map_err(|e| io(target, e))?;
            fs::rename(temp, target).map_err(|e| io(target, e))
        }
        Err(error) => {
            let _ = fs::remove_file(temp);
            Err(io(target, error))
        }
    }
}

#[cfg(unix)]
fn sync_journal_directory(dir: &Path) -> Result<()> {
    File::open(dir).map_err(|e| io(dir, e))?.sync_all().map_err(|e| io(dir, e))
}

#[cfg(not(unix))]
fn sync_journal_directory(_dir: &Path) -> Result<()> {
    Ok(())
}

pub fn load_journal(path: &Path) -> Result<OperationJournal> {
    let file = File::open(path).map_err(|e| io(path, e))?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::JournalEntry;
    use chrono::Utc;
    use tempfile::tempdir;
    use uuid::Uuid;

    fn journal(dir: &Path, operation_id: Uuid) -> OperationJournal {
        OperationJournal {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            root: dir.to_path_buf(),
            entries: vec![JournalEntry {
                operation_id,
                from: dir.join("before.txt"),
                to: dir.join("after.txt"),
            }],
        }
    }

    #[test]
    fn journal_round_trip_uses_atomic_target_file() {
        let dir = tempdir().unwrap();
        let journal = journal(dir.path(), Uuid::new_v4());

        let path = save_journal(dir.path(), &journal).unwrap();
        assert!(path.exists());
        assert!(!path.with_extension("journal.json.tmp").exists());
        let stored = load_journal(&path).unwrap();
        assert_eq!(stored, journal);
    }

    #[test]
    fn saving_existing_journal_replaces_previous_snapshot() {
        let dir = tempdir().unwrap();
        let operation_id = Uuid::new_v4();
        let first = journal(dir.path(), operation_id);
        let path = save_journal(dir.path(), &first).unwrap();

        let mut second = first.clone();
        second.entries.push(JournalEntry {
            operation_id: Uuid::new_v4(),
            from: dir.path().join("another-before.txt"),
            to: dir.path().join("another-after.txt"),
        });
        assert_eq!(save_journal(dir.path(), &second).unwrap(), path);
        assert_eq!(load_journal(&path).unwrap(), second);
        assert!(!path.with_extension("journal.json.tmp").exists());
    }

    #[test]
    fn relative_journal_paths_are_stored_as_absolute_paths() {
        let root = tempdir().unwrap();
        let journal = OperationJournal {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            root: root.path().to_path_buf(),
            entries: vec![JournalEntry {
                operation_id: Uuid::new_v4(),
                from: PathBuf::from("relative-before.txt"),
                to: PathBuf::from("relative-after.txt"),
            }],
        };

        let path = save_journal(root.path(), &journal).unwrap();
        let stored = load_journal(&path).unwrap();
        assert!(stored.root.is_absolute());
        assert!(stored.entries[0].from.is_absolute());
        assert!(stored.entries[0].to.is_absolute());
    }

    #[cfg(unix)]
    #[test]
    fn saving_journal_syncs_the_parent_directory_after_replace() {
        let dir = tempdir().unwrap();
        let journal = journal(dir.path(), Uuid::new_v4());

        let path = save_journal(dir.path(), &journal).unwrap();
        let directory = File::open(dir.path()).unwrap();
        directory.sync_all().unwrap();
        assert!(path.exists());
    }
}
