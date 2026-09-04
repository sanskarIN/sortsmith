use crate::{error::SortSmithError, Result};
use std::collections::HashSet;
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

pub fn validate_filename_fragment(fragment: &str, label: &str) -> Result<()> {
    if fragment.trim().is_empty() {
        return Err(SortSmithError::InvalidRule(format!("{label} cannot be empty")));
    }
    if contains_invalid_filename_character(fragment) {
        return Err(SortSmithError::InvalidRule(format!("{label} contains a character that is unsafe in a cross-platform filename")));
    }
    Ok(())
}

pub fn validate_filename(filename: &str, label: &str) -> Result<()> {
    if filename.is_empty() || matches!(filename, "." | "..") {
        return Err(SortSmithError::InvalidRule(format!("{label} cannot be empty or a reserved path component")));
    }
    if filename.as_bytes().len() > 255 || filename.encode_utf16().count() > 255 {
        return Err(SortSmithError::InvalidRule(format!("{label} is too long for a portable filename")));
    }
    if contains_invalid_filename_character(filename) {
        return Err(SortSmithError::InvalidRule(format!("{label} contains a character that is unsafe in a cross-platform filename")));
    }
    if filename.ends_with([' ', '.']) {
        return Err(SortSmithError::InvalidRule(format!("{label} cannot end with a space or period")));
    }
    if is_windows_reserved_name(filename) {
        return Err(SortSmithError::InvalidRule(format!("{label} uses a Windows-reserved device name")));
    }
    Ok(())
}

fn contains_invalid_filename_character(value: &str) -> bool {
    value.chars().any(|ch| ch.is_control() || matches!(ch, '/' | '\\' | '<' | '>' | ':' | '"' | '|' | '?' | '*'))
}

fn is_windows_reserved_name(filename: &str) -> bool {
    let stem = filename.split('.').next().unwrap_or(filename).trim_end_matches([' ', '.']).to_ascii_uppercase();
    matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || is_windows_numbered_device_name(&stem, "COM")
        || is_windows_numbered_device_name(&stem, "LPT")
}

fn is_windows_numbered_device_name(stem: &str, prefix: &str) -> bool {
    let Some(suffix) = stem.strip_prefix(prefix) else { return false; };
    matches!(suffix, "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³")
}

pub fn collision_safe_path(destination: &Path) -> PathBuf {
    collision_safe_path_with_reserved(destination, &HashSet::new())
}

pub fn collision_safe_path_with_reserved(destination: &Path, reserved: &HashSet<PathBuf>) -> PathBuf {
    if !destination.exists() && !reserved_contains(reserved, destination) {
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
        if !candidate.exists() && !reserved_contains(reserved, &candidate) {
            return candidate;
        }
    }

    loop {
        let suffix = uuid::Uuid::new_v4().simple();
        let filename = match ext {
            Some(ext) => format!("{stem} ({suffix}).{ext}"),
            None => format!("{stem} ({suffix})"),
        };
        let candidate = parent.join(filename);
        if !candidate.exists() && !reserved_contains(reserved, &candidate) {
            return candidate;
        }
    }
}

#[cfg(windows)]
fn reserved_contains(reserved: &HashSet<PathBuf>, candidate: &Path) -> bool {
    let candidate = candidate.to_string_lossy().to_lowercase();
    reserved.iter().any(|path| path.to_string_lossy().to_lowercase() == candidate)
}

#[cfg(not(windows))]
fn reserved_contains(reserved: &HashSet<PathBuf>, candidate: &Path) -> bool {
    reserved.contains(candidate)
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
    fn rejects_unsafe_filename_fragments() {
        assert!(validate_filename_fragment("../escape", "prefix").is_err());
        assert!(validate_filename_fragment("bad:name", "prefix").is_err());
        assert!(validate_filename_fragment("safe-prefix_", "prefix").is_ok());
    }

    #[test]
    fn rejects_non_portable_rendered_filenames() {
        assert!(validate_filename("CON.txt", "filename").is_err());
        assert!(validate_filename("LPT9", "filename").is_err());
        assert!(validate_filename("report. ", "filename").is_err());
        assert!(validate_filename("report.", "filename").is_err());
        assert!(validate_filename("report-final.txt", "filename").is_ok());
    }

    #[test]
    fn rejects_unicode_windows_reserved_device_names() {
        assert!(validate_filename("COM¹.txt", "filename").is_err());
        assert!(validate_filename("LPT²", "filename").is_err());
        assert!(validate_filename("COM0.txt", "filename").is_ok());
    }

    #[test]
    fn rejects_overlong_rendered_filenames() {
        let name = format!("{}.txt", "a".repeat(252));
        assert!(validate_filename(&name, "filename").is_err());
    }

    #[test]
    fn rejects_unicode_filename_that_exceeds_windows_utf16_limit() {
        let name = format!("{}x.txt", "😀".repeat(126));
        assert!(name.as_bytes().len() <= 255);
        assert!(name.chars().count() <= 255);
        assert!(name.encode_utf16().count() > 255);
        assert!(validate_filename(&name, "filename").is_err());
    }

    #[test]
    fn accepts_unicode_filename_within_utf16_limit() {
        let name = format!("{}x.txt", "😀".repeat(124));
        assert!(name.encode_utf16().count() <= 255);
        assert!(validate_filename(&name, "filename").is_ok());
    }

    #[test]
    fn collision_path_preserves_extension() {
        let dir = tempdir().unwrap();
        let existing = dir.path().join("report.pdf");
        std::fs::write(&existing, b"x").unwrap();
        let candidate = collision_safe_path(&existing);
        assert_eq!(candidate.file_name().unwrap(), "report (1).pdf");
    }

    #[test]
    fn reserved_collision_path_uses_next_suffix() {
        let dir = tempdir().unwrap();
        let desired = dir.path().join("report.pdf");
        let mut reserved = HashSet::new();
        reserved.insert(desired.clone());
        reserved.insert(dir.path().join("report (1).pdf"));
        let candidate = collision_safe_path_with_reserved(&desired, &reserved);
        assert_eq!(candidate.file_name().unwrap(), "report (2).pdf");
    }

    #[cfg(windows)]
    #[test]
    fn reserved_collision_path_is_case_insensitive_on_windows() {
        let dir = tempdir().unwrap();
        let desired = dir.path().join("Report.txt");
        let mut reserved = HashSet::new();
        reserved.insert(dir.path().join("report.txt"));
        reserved.insert(dir.path().join("REPORT (1).txt"));
        let candidate = collision_safe_path_with_reserved(&desired, &reserved);
        assert_eq!(candidate.file_name().unwrap(), "Report (2).txt");
    }
}
