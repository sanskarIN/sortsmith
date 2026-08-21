use crate::error::io;
use crate::models::OperationJournal;
use crate::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub fn journal_path(dir: &Path, id: uuid::Uuid) -> PathBuf {
    dir.join(format!("{id}.journal.json"))
}

pub fn save_journal(dir: &Path, journal: &OperationJournal) -> Result<PathBuf> {
    fs::create_dir_all(dir).map_err(|e| io(dir, e))?;
    let target = journal_path(dir, journal.id);
    let temp = target.with_extension("journal.json.tmp");
    let bytes = serde_json::to_vec_pretty(journal)?;
    fs::write(&temp, bytes).map_err(|e| io(&temp, e))?;
    fs::rename(&temp, &target).map_err(|e| io(&target, e))?;
    Ok(target)
}

pub fn load_journal(path: &Path) -> Result<OperationJournal> {
    let bytes = fs::read(path).map_err(|e| io(path, e))?;
    Ok(serde_json::from_slice(&bytes)?)
}
