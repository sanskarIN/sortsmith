use crate::models::{FileEntry, Rule, RuleAction, RuleCriterion};
use crate::{Result, SortSmithError};
use chrono::{Duration, Utc};
use regex::Regex;
use std::path::{Path, PathBuf};

pub fn validate_rule(rule: &Rule) -> Result<()> {
    if rule.name.trim().is_empty() || rule.name.chars().count() > 128 {
        return Err(SortSmithError::InvalidRule("rule name must contain 1 to 128 characters".into()));
    }
    if rule.criteria.is_empty() || rule.criteria.len() > 16 {
        return Err(SortSmithError::InvalidRule(format!("'{}' must contain 1 to 16 criteria", rule.name)));
    }

    for criterion in &rule.criteria {
        match criterion {
            RuleCriterion::Extension { values } => validate_values(values, 64, "extension", &rule.name)?,
            RuleCriterion::MimePrefix { values } => validate_values(values, 64, "MIME prefix", &rule.name)?,
            RuleCriterion::ModifiedOlderThanDays { days } => {
                if *days > 365_000 { return Err(SortSmithError::InvalidRule(format!("'{}' has an unsupported modified-age range", rule.name))); }
            }
            RuleCriterion::SizeRange { min_bytes, max_bytes } => {
                if min_bytes.is_none() && max_bytes.is_none() {
                    return Err(SortSmithError::InvalidRule(format!("'{}' has an empty size range", rule.name)));
                }
                if let (Some(minimum), Some(maximum)) = (min_bytes, max_bytes) {
                    if minimum > maximum { return Err(SortSmithError::InvalidRule(format!("'{}' has a minimum size larger than its maximum", rule.name))); }
                }
            }
            RuleCriterion::NameRegex { pattern } => {
                if pattern.is_empty() || pattern.len() > 1_024 {
                    return Err(SortSmithError::InvalidRule(format!("'{}' has an empty or oversized filename pattern", rule.name)));
                }
                Regex::new(pattern).map_err(|e| SortSmithError::InvalidRule(format!("invalid regex in '{}': {e}", rule.name)))?;
            }
        }
    }

    match &rule.action {
        RuleAction::MoveTo { subdirectory } => {
            crate::safety::safe_subdirectory(Path::new("."), subdirectory)?;
        }
        RuleAction::RenamePrefix { prefix } => {
            crate::safety::validate_filename_fragment(prefix, "rename prefix")?;
        }
        RuleAction::RenameTemplate { template } => {
            crate::safety::validate_filename_fragment(template, "rename template")?;
            if template.contains("..") {
                return Err(SortSmithError::InvalidRule("rename template cannot contain '..'".into()));
            }
            if !template.contains("{name}") && !template.contains("{ext}") {
                return Err(SortSmithError::InvalidRule("rename template must contain {name} or {ext}".into()));
            }
            let unknown = template.replace("{name}", "").replace("{ext}", "");
            if unknown.contains('{') || unknown.contains('}') {
                return Err(SortSmithError::InvalidRule("rename template contains an unknown placeholder".into()));
            }
        }
    }
    Ok(())
}

fn validate_values(values: &[String], maximum: usize, label: &str, rule_name: &str) -> Result<()> {
    if values.is_empty() || values.len() > maximum || values.iter().any(|value| value.trim().is_empty() || value.len() > 128) {
        return Err(SortSmithError::InvalidRule(format!("'{rule_name}' has invalid {label} values")));
    }
    Ok(())
}

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
        RuleAction::RenamePrefix { prefix } => {
            crate::safety::validate_filename_fragment(prefix, "rename prefix")?;
            Ok(file.path.with_file_name(format!("{prefix}{}", name.to_string_lossy())))
        }
        RuleAction::RenameTemplate { template } => {
            crate::safety::validate_filename_fragment(template, "rename template")?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{RuleAction, RuleCriterion};
    use std::path::PathBuf;
    use uuid::Uuid;

    fn entry() -> FileEntry {
        FileEntry {
            path: PathBuf::from("/tmp/root/report.txt"),
            relative_path: PathBuf::from("report.txt"),
            size: 12,
            modified_at: None,
            mime: Some("text/plain".into()),
            extension: Some("txt".into()),
        }
    }

    fn rule(action: RuleAction) -> Rule {
        Rule {
            id: Uuid::new_v4(),
            name: "rename".into(),
            enabled: true,
            match_all: true,
            criteria: vec![RuleCriterion::Extension { values: vec!["txt".into()] }],
            action,
        }
    }

    #[test]
    fn rejects_unsafe_rename_prefix() {
        let result = destination_for(Path::new("/tmp/root"), &entry(), &rule(RuleAction::RenamePrefix { prefix: "../".into() }));
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_size_range() {
        let mut candidate = rule(RuleAction::MoveTo { subdirectory: "Text".into() });
        candidate.criteria = vec![RuleCriterion::SizeRange { min_bytes: Some(20), max_bytes: Some(10) }];
        assert!(validate_rule(&candidate).is_err());
    }

    #[test]
    fn rejects_unknown_template_placeholder() {
        let candidate = rule(RuleAction::RenameTemplate { template: "{name}-{unknown}.{ext}".into() });
        assert!(validate_rule(&candidate).is_err());
    }

    #[test]
    fn renders_safe_template_and_preserves_extension() {
        let candidate = rule(RuleAction::RenameTemplate { template: "{name}-sorted.{ext}".into() });
        validate_rule(&candidate).unwrap();
        let result = destination_for(Path::new("/tmp/root"), &entry(), &candidate).unwrap();
        assert_eq!(result.file_name().unwrap(), "report-sorted.txt");
    }
}
