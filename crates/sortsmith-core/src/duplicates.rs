use crate::error::io;
use crate::models::{DuplicateFile, DuplicateGroup, ScanOptions};
use crate::Result;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn find_duplicates(root: &Path, options: &ScanOptions) -> Result<Vec<DuplicateGroup>> {
    let depth = if options.recursive { options.max_depth.unwrap_or(32) } else { 1 };
    let canonical_root = root.canonicalize().map_err(|e| io(root, e))?;
    let mut by_size: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    for item in WalkDir::new(root)
        .follow_links(options.follow_links)
        .max_depth(depth)
        .into_iter()
        .filter_entry(|entry| {
            if !options.include_hidden && is_hidden(entry, root) {
                return false;
            }
            options.follow_links && entry_resolves_outside_root(entry, &canonical_root) == Some(true).then_some(false).unwrap_or(true)
        })
    {
        let Ok(entry) = item else { continue; };
        if !entry.file_type().is_file() { continue; }
        if options.follow_links && !entry_within_root(entry.path(), &canonical_root) { continue; }
        if let Ok(metadata) = entry.metadata() {
            by_size.entry(metadata.len()).or_default().push(entry.path().to_path_buf());
        }
    }
    let candidates: Vec<(u64, Vec<PathBuf>)> = by_size.into_iter().filter(|(_, files)| files.len() > 1).collect();
    let hashed: Vec<(u64, PathBuf, String)> = candidates
        .par_iter()
        .flat_map_iter(|(size, files)| files.iter().filter_map(move |path| hash_file(path).ok().map(|hash| (*size, path.clone(), hash))))
        .collect();
    let mut grouped: HashMap<(u64, String), Vec<PathBuf>> = HashMap::new();
    for (size, path, hash) in hashed { grouped.entry((size, hash)).or_default().push(path); }
    let mut groups: Vec<_> = grouped
        .into_iter()
        .filter(|(_, files)| files.len() > 1)
        .map(|((size, hash), mut files)| {
            files.sort();
            DuplicateGroup { hash, size, files: files.into_iter().map(|path| DuplicateFile { path, size }).collect() }
        })
        .collect();
    groups.sort_by(|a, b| b.size.cmp(&a.size).then_with(|| a.hash.cmp(&b.hash)));
    Ok(groups)
}

fn entry_resolves_outside_root(entry: &DirEntry, canonical_root: &Path) -> Option<bool> {
    if !entry.file_type().is_symlink() { return Some(false); }
    entry.path().canonicalize().ok().map(|resolved| !resolved.starts_with(canonical_root))
}

fn entry_within_root(path: &Path, canonical_root: &Path) -> bool {
    path.canonicalize().is_ok_and(|resolved| resolved.starts_with(canonical_root))
}

fn is_hidden(entry: &DirEntry, root: &Path) -> bool {
    if entry.path() == root { return false; }
    entry.file_name().to_str().is_some_and(|name| name.starts_with('.'))
}

fn hash_file(path: &Path) -> Result<String> {
    let file = File::open(path).map_err(|e| io(path, e))?;
    let mut reader = BufReader::with_capacity(1024 * 1024, file);
    let mut hasher = blake3::Hasher::new();
    let mut buffer = vec![0u8; 1024 * 1024];
    loop {
        let read = reader.read(&mut buffer).map_err(|e| io(path, e))?;
        if read == 0 { break; }
        hasher.update(&buffer[..read]);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn finds_equal_content_without_deleting() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("a.bin"), b"same").unwrap();
        std::fs::write(dir.path().join("b.bin"), b"same").unwrap();
        std::fs::write(dir.path().join("c.bin"), b"different").unwrap();
        let groups = find_duplicates(dir.path(), &ScanOptions::default()).unwrap();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].files.len(), 2);
        assert!(dir.path().join("a.bin").exists());
    }

    #[test]
    fn duplicate_files_are_sorted_by_path() {
        let dir = tempdir().unwrap();
        let first = dir.path().join("z-last.bin");
        let second = dir.path().join("a-first.bin");
        std::fs::write(&first, b"same").unwrap();
        std::fs::write(&second, b"same").unwrap();

        let groups = find_duplicates(dir.path(), &ScanOptions::default()).unwrap();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].files[0].path, second);
        assert_eq!(groups[0].files[1].path, first);
    }

    #[test]
    fn hidden_directories_are_pruned_unless_requested() {
        let dir = tempdir().unwrap();
        let hidden = dir.path().join(".cache");
        std::fs::create_dir(&hidden).unwrap();
        std::fs::write(hidden.join("a.bin"), b"same").unwrap();
        std::fs::write(hidden.join("b.bin"), b"same").unwrap();

        let mut options = ScanOptions { recursive: true, ..ScanOptions::default() };
        assert!(find_duplicates(dir.path(), &options).unwrap().is_empty());

        options.include_hidden = true;
        let groups = find_duplicates(dir.path(), &options).unwrap();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].files.len(), 2);
    }

    #[cfg(unix)]
    #[test]
    fn external_symlink_directories_are_not_scanned() {
        use std::os::unix::fs::symlink;
        let root = tempdir().unwrap();
        let outside = tempdir().unwrap();
        let linked = root.path().join("external");
        std::fs::write(outside.path().join("a.bin"), b"same").unwrap();
        std::fs::write(outside.path().join("b.bin"), b"same").unwrap();
        symlink(outside.path(), &linked).unwrap();

        let options = ScanOptions { recursive: true, include_hidden: false, follow_links: true, max_depth: Some(8) };
        let groups = find_duplicates(root.path(), &options).unwrap();
        assert!(groups.is_empty());
    }
}