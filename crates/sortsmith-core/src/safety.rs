use crate::{error::SortSmithError, Result};
use std::path::{Component, Path, PathBuf};

pub fn safe_subdirectory(root: &Path, subdirectory: &str) -> Result<PathBuf> {
    let candidate = Path::new(subdirectory);
    if candidate.as_os_str().is_empty() || candidate.is_absolute() {
        return Err(SortSmithError::UnsafeDestination(candidate.to_path_buf()));
    }
    if candidate.components().any(|c| matches!(c, Component::ParentDir | Component::RootDir | Component::Prefix(_))) {
        return Err(SortSmithError::UnsafeDestination(candidate.to_path_buf()));
    }
    Ok(root.join(candidate))
}

pub fn collision_safe_path(destination: &Path) -> PathBuf {
    if !destination.exists() {
        return destination.to_path_buf();
    }
    let parent = destination.parent().unwrap_or_else(|| Path::new("."));
    let stem = destination.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
    let ext = destination.extension().and_then(|e| e.to_str());
    for n in 1..=100_000u32 {
        let filename = match ext {
            Some(ext) => format!("{stem} ({n}).{ext}"),
            None => format!("{stem} ({n})"),
        };
        let candidate = parent.join(filename);
        if !candidate.exists() {
            return candidate;
        }
    }
    destination.to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn rejects_parent_escape() {
        let root = Path::new("/tmp/root");
        assert!(safe_subdirectory(root, "../outside").is_err());
    }

    #[test]
    fn collision_path_preserves_extension() {
        let dir = tempdir().unwrap();
        let existing = dir.path().join("report.pdf");
        std::fs::write(&existing, b"x").unwrap();
        let candidate = collision_safe_path(&existing);
        assert_eq!(candidate.file_name().unwrap(), "report (1).pdf");
    }
}
