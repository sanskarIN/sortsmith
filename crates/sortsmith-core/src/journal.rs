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

    let file = File::create(&temp).map_err(|e| io(&temp, e))?;
    let mut writer = BufWriter::new(file);
    if let Err(error) = serde_json::to_writer_pretty(&mut writer, journal) {
        drop(writer);
        let _ = fs::remove_file(&temp);
        return Err(error.into());
    }
    writer.flush().map_err(|e| io(&temp, e))?;
    writer.get_ref().sync_all().map_err(|e| io(&temp, e))?;
    drop(writer);

    if let Err(error) = fs::rename(&temp, &target) {
        let _ = fs::remove_file(&temp);
        return Err(io(&target, error));
    }
    Ok(target)
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

    #[test]
    fn journal_round_trip_uses_atomic_target_file() {
        let dir = tempdir().unwrap();
        let journal = OperationJournal {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            root: dir.path().to_path_buf(),
            entries: vec![JournalEntry {
                operation_id: Uuid::new_v4(),
                from: dir.path().join("before.txt"),
                to: dir.path().join("after.txt"),
            }],
        };

        let path = save_journal(dir.path(), &journal).unwrap();
        assert!(path.exists());
        assert!(!path.with_extension("journal.json.tmp").exists());
        assert_eq!(load_journal(&path).unwrap(), journal);
    }
}
