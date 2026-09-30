use crate::{Result, error::SortSmithError};
use std::collections::HashSet;
use std::path::{Component, Path, PathBuf};

pub fn safe_subdirectory(root: &Path, subdirectory: &str) -> Result<PathBuf> {
    let candidate = Path::new(subdirectory);
    if candidate.as_os_str().is_empty() || candidate.is_absolute() {
        return Err(SortSmithError::UnsafeDestination(candidate.to_path_buf()));
    }
    if candidate.components().any(|c| {
        matches!(
            c,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err(SortSmithError::UnsafeDestination(candidate.to_path_buf()));
    }
    Ok(root.join(candidate))
}

pub fn validate_filename_fragment(fragment: &str, label: &str) -> Result<()> {
    if fragment.trim().is_empty() {
        return Err(SortSmithError::InvalidRule(format!(
            "{label} cannot be empty"
        )));
    }
    if contains_invalid_filename_character(fragment) {
        return Err(SortSmithError::InvalidRule(format!(
            "{label} contains a character that is unsafe in a cross-platform filename"
        )));
    }
    Ok(())
}

pub fn validate_filename(filename: &str, label: &str) -> Result<()> {
    if filename.is_empty() || matches!(filename, "." | "..") {
        return Err(SortSmithError::InvalidRule(format!(
            "{label} cannot be empty or a reserved path component"
        )));
    }
    if filename.len() > 255 || filename.encode_utf16().count() > 255 {
        return Err(SortSmithError::InvalidRule(format!(
            "{label} is too long for a portable filename"
        )));
    }
    if contains_invalid_filename_character(filename) {
        return Err(SortSmithError::InvalidRule(format!(
            "{label} contains a character that is unsafe in a cross-platform filename"
        )));
    }
    if filename.ends_with([' ', '.']) {
        return Err(SortSmithError::InvalidRule(format!(
            "{label} cannot end with a space or period"
        )));
    }
    if is_windows_reserved_name(filename) {
        return Err(SortSmithError::InvalidRule(format!(
            "{label} uses a Windows-reserved device name"
        )));
    }
    Ok(())
}

fn contains_invalid_filename_character(value: &str) -> bool {
    value.chars().any(|ch| {
        ch.is_control() || matches!(ch, '/' | '\\' | '<' | '>' | ':' | '"' | '|' | '?' | '*')
    value.chars().any(|c| {
        matches!(c, '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*') || c.is_control()
    })
}

fn is_windows_reserved_name(filename: &str) -> bool {
    let stem = filename
        .split('.')
        .next()
        .unwrap_or(filename)
        .trim_end_matches([' ', '.'])
        .to_ascii_uppercase();
    matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || is_windows_numbered_device_name(&stem, "COM")
        || is_windows_numbered_device_name(&stem, "LPT")
}

fn is_windows_numbered_device_name(stem: &str, prefix: &str) -> bool {
    let Some(suffix) = stem.strip_prefix(prefix) else {
        return false;
    };
    matches!(
        suffix,
        "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³"
    )
}

pub fn collision_safe_path(destination: &Path) -> PathBuf {
    collision_safe_path_with_reserved(destination, &HashSet::new())
}

pub fn collision_safe_path_with_reserved(
    destination: &Path,
    reserved: &HashSet<PathBuf>,
) -> PathBuf {
    if !destination.exists() && !reserved_contains(reserved, destination) {
        return destination.to_path_buf();
    }
    let parent = destination.parent().unwrap_or_else(|| Path::new("."));
    let stem = destination
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("file");
    let ext = destination.extension().and_then(|e| e.to_str());
    for n in 1..=100_000u32 {
        let filename = collision_filename(stem, ext, &n.to_string());
        let candidate = parent.join(filename);
        if !candidate.exists() && !reserved_contains(reserved, &candidate) {
            return candidate;
        }
    }

    loop {
        let suffix = uuid::Uuid::new_v4().simple().to_string();
        let filename = collision_filename(stem, ext, &suffix);
        let candidate = parent.join(filename);
        if !candidate.exists() && !reserved_contains(reserved, &candidate) {
            return candidate;
        }
    }
}

fn collision_filename(stem: &str, ext: Option<&str>, suffix: &str) -> String {
    let extension = ext.map_or(0, |value| value.encode_utf16().count() + 1);
    let suffix_units = suffix.encode_utf16().count() + 3;
    let max_stem_units = 255usize.saturating_sub(extension + suffix_units);
    let max_stem_bytes =
        255usize.saturating_sub(ext.map_or(0, |value| value.len() + 1) + suffix.len() + 3);
    let mut fitted = String::new();
    for ch in stem.chars() {
        let next_units = fitted.encode_utf16().count() + ch.len_utf16();
        let next_bytes = fitted.len() + ch.len_utf8();
        if next_units > max_stem_units || next_bytes > max_stem_bytes {
            break;
        }
        fitted.push(ch);
    }
    let fitted = fitted.trim_end_matches([' ', '.']);
    match ext {
        Some(ext) => format!("{fitted} ({suffix}).{ext}"),
        None => format!("{fitted} ({suffix})"),
    }
}

#[cfg(windows)]
fn reserved_contains(reserved: &HashSet<PathBuf>, candidate: &Path) -> bool {
    let candidate = candidate.to_string_lossy().to_lowercase();
    reserved
        .iter()
        .any(|path| path.to_string_lossy().to_lowercase() == candidate)
}

#[cfg(not(windows))]
fn reserved_contains(reserved: &HashSet<PathBuf>, candidate: &Path) -> bool {
    reserved.contains(candidate)
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(filename)
        .split('.')
        .next()
        .unwrap_or(filename)
        .to_ascii_uppercase();
    matches!(
        stem.as_str(),
        "CON"
            | "PRN"
            | "AUX"
            | "NUL"
            | "COM1"
            | "COM2"
            | "COM3"
            | "COM4"
            | "COM5"
            | "COM6"
            | "COM7"
            | "COM8"
            | "COM9"
            | "LPT1"
            | "LPT2"
            | "LPT3"
            | "LPT4"
            | "LPT5"
            | "LPT6"
            | "LPT7"
            | "LPT8"
            | "LPT9"
    )
}

pub fn collision_safe_path(path: &Path) -> PathBuf {
    collision_safe_path_with_reserved(path, &HashSet::new())
}

pub fn collision_safe_path_with_reserved(path: &Path, reserved: &HashSet<PathBuf>) -> PathBuf {
    if !path.exists() && !reserved.contains(path) {
        return path.to_path_buf();
    }
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let stem = path.file_stem().and_then(|v| v.to_str()).unwrap_or("file");
    let extension = path.extension().and_then(|v| v.to_str());
    for index in 1..=100_000usize {
        let candidate_name = match extension {
            Some(ext) if !ext.is_empty() => format!("{stem} ({index}).{ext}"),
            _ => format!("{stem} ({index})"),
        };
        let candidate = parent.join(candidate_name);
        if !candidate.exists() && !reserved.contains(&candidate) {
            return candidate;
        }
    }
    path.to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn rejects_windows_unsafe_characters() {
        assert!(validate_filename("bad:name.txt", "file").is_err());
    }

    #[test]
    fn rejects_windows_reserved_names() {
        assert!(validate_filename("CON.txt", "file").is_err());
    }

    #[test]
    fn rejects_overlong_utf8_or_utf16_names() {
        assert!(validate_filename(&"a".repeat(256), "file").is_err());
        assert!(validate_filename(&"😀".repeat(256), "file").is_err());
    }

    #[test]
    fn collision_helper_preserves_extension() {
        let root = tempdir().unwrap();
        let path = root.path().join("note.txt");
        std::fs::write(&path, b"existing").unwrap();
        assert_eq!(collision_safe_path(&path), root.path().join("note (1).txt"));
    }
}
