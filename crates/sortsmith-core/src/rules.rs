use crate::models::{FileEntry, Rule, RuleAction, RuleCriterion};
use crate::{Result, SortSmithError};
use chrono::{Duration, Utc};
use regex::Regex;
use std::path::{Path, PathBuf};

pub fn rule_matches(rule: &Rule, file: &FileEntry) -> Result<bool> {
    if !rule.enabled || rule.criteria.is_empty() {
        return Ok(false);
    }
    let mut matches = Vec::with_capacity(rule.criteria.len());
    for criterion in &rule.criteria {
        let matched = match criterion {
            RuleCriterion::Extension { values } => file.extension.as_ref().is_some_and(|ext| values.iter().any(|v| v.trim_start_matches('.').eq_ignore_ascii_case(ext))),
            RuleCriterion::MimePrefix { values } => file.mime.as_ref().is_some_and(|mime| values.iter().any(|prefix| mime.to_ascii_lowercase().starts_with(&prefix.to_ascii_lowercase()))),
            RuleCriterion::ModifiedOlderThanDays { days } => file.modified_at.is_some_and(|m| m < Utc::now() - Duration::days(i64::from(*days))),
            RuleCriterion::SizeRange { min_bytes, max_bytes } => min_bytes.is_none_or(|m| file.size >= m) && max_bytes.is_none_or(|m| file.size <= m),
            RuleCriterion::NameRegex { pattern } => {
                let regex = Regex::new(pattern).map_err(|e| SortSmithError::InvalidRule(format!("invalid regex in '{}': {e}", rule.name)))?;
                file.path.file_name().and_then(|n| n.to_str()).is_some_and(|name| regex.is_match(name))
            }
        };
        matches.push(matched);
    }
    Ok(if rule.match_all { matches.into_iter().all(|m| m) } else { matches.into_iter().any(|m| m) })
}

pub fn destination_for(root: &Path, file: &FileEntry, rule: &Rule) -> Result<PathBuf> {
    let name = file.path.file_name().ok_or_else(|| SortSmithError::InvalidRule("file has no name".into()))?;
    match &rule.action {
        RuleAction::MoveTo { subdirectory } => Ok(crate::safety::safe_subdirectory(root, subdirectory)?.join(name)),
        RuleAction::RenamePrefix { prefix } => Ok(file.path.with_file_name(format!("{prefix}{}", name.to_string_lossy()))),
        RuleAction::RenameTemplate { template } => {
            let stem = file.path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
            let ext = file.extension.as_deref().unwrap_or("");
            let mut rendered = template.replace("{name}", stem).replace("{ext}", ext);
            if rendered.contains('/') || rendered.contains('\\') || rendered.contains("..") {
                return Err(SortSmithError::InvalidRule("rename template cannot contain path separators or '..'".into()));
            }
            if rendered.trim().is_empty() {
                return Err(SortSmithError::InvalidRule("rename template cannot produce an empty name".into()));
            }
            if !ext.is_empty() && !rendered.ends_with(&format!(".{ext}")) {
                rendered.push('.');
                rendered.push_str(ext);
            }
            Ok(file.path.with_file_name(rendered))
        }
    }
}
