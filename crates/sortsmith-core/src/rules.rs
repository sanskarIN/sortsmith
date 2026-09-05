use crate::models::{FileEntry, Rule, RuleAction, RuleCriterion};
use crate::{Result, SortSmithError};
use chrono::{Duration, Utc};
use regex::Regex;
use std::path::{Path, PathBuf};

pub struct PreparedRule<'a> {
    rule: &'a Rule,
    regexes: Vec<Option<Regex>>,
}

impl<'a> PreparedRule<'a> {
    pub fn new(rule: &'a Rule) -> Result<Self> {
        validate_rule(rule)?;
        let regexes = rule
            .criteria
            .iter()
            .map(|criterion| match criterion {
                RuleCriterion::NameRegex { pattern } => Regex::new(pattern).ok(),
                _ => None,
            })
            .collect();
        Ok(Self { rule, regexes })
    }

    pub fn rule(&self) -> &'a Rule {
        self.rule
    }

    pub fn matches(&self, file: &FileEntry) -> bool {
        if !self.rule.enabled {
            return false;
        }
        let matches = self
            .rule
            .criteria
            .iter()
            .zip(&self.regexes)
            .map(|(criterion, prepared_regex)| match criterion {
                RuleCriterion::Extension { values } => {
                    file.extension.as_ref().is_some_and(|ext| {
                        values
                            .iter()
                            .any(|v| v.trim_start_matches('.').eq_ignore_ascii_case(ext))
                    })
                }
                RuleCriterion::MimePrefix { values } => {
                    file.mime.as_ref().is_some_and(|mime| {
                        values.iter().any(|prefix| {
                            mime.to_ascii_lowercase()
                                .starts_with(&prefix.to_ascii_lowercase())
                        })
                    })
                }
                RuleCriterion::ModifiedOlderThanDays { days } => file
                    .modified_at
                    .is_some_and(|m| m < Utc::now() - Duration::days(i64::from(*days))),
                RuleCriterion::SizeRange {
                    min_bytes,
                    max_bytes,
                } => {
                    min_bytes.is_none_or(|m| file.size >= m)
                        && max_bytes.is_none_or(|m| file.size <= m)
                }
                RuleCriterion::NameRegex { .. } => prepared_regex.as_ref().is_some_and(|regex| {
                    file.path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .is_some_and(|name| regex.is_match(name))
                }),
            });
        if self.rule.match_all {
            matches.all(|matched| matched)
        } else {
            matches.any(|matched| matched)
        }
    }
}

pub fn validate_rule(rule: &Rule) -> Result<()> {
    if rule.name.trim().is_empty() || rule.name.chars().count() > 128 {
        return Err(SortSmithError::InvalidRule(
            "rule name must contain 1 to 128 characters".into(),
        ));
    }
    if rule.criteria.is_empty() || rule.criteria.len() > 16 {
        return Err(SortSmithError::InvalidRule(format!(
            "'{}' must contain 1 to 16 criteria",
            rule.name
        )));
    }

    for criterion in &rule.criteria {
        match criterion {
            RuleCriterion::Extension { values } => {
                validate_values(values, 64, "extension", &rule.name)?
            }
            RuleCriterion::MimePrefix { values } => {
                validate_values(values, 64, "MIME prefix", &rule.name)?
            }
            RuleCriterion::ModifiedOlderThanDays { days } => {
                if *days > 365_000 {
                    return Err(SortSmithError::InvalidRule(format!(
                        "'{}' has an unsupported modified-age range",
                        rule.name
                    )));
                }
            }
            RuleCriterion::SizeRange {
                min_bytes,
                max_bytes,
            } => {
                if min_bytes.is_none() && max_bytes.is_none() {
                    return Err(SortSmithError::InvalidRule(format!(
                        "'{}' has an empty size range",
                        rule.name
                    )));
                }
                if let (Some(minimum), Some(maximum)) = (min_bytes, max_bytes)
                    && minimum > maximum
                {
                    return Err(SortSmithError::InvalidRule(format!(
                        "'{}' has a minimum size larger than its maximum",
                        rule.name
                    )));
                }
            }
            RuleCriterion::NameRegex { pattern } => {
                if pattern.is_empty() || pattern.chars().count() > 1_024 {
                    return Err(SortSmithError::InvalidRule(format!(
                        "'{}' has an empty or oversized filename pattern",
                        rule.name
                    )));
                }
                Regex::new(pattern).map_err(|e| {
                    SortSmithError::InvalidRule(format!("invalid regex in '{}': {e}", rule.name))
                })?;
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
                return Err(SortSmithError::InvalidRule(
                    "rename template cannot contain '..'".into(),
                ));
            }
            if !template.contains("{name}") && !template.contains("{ext}") {
                return Err(SortSmithError::InvalidRule(
                    "rename template must contain {name} or {ext}".into(),
                ));
            }
            let unknown = template.replace("{name}", "").replace("{ext}", "");
            if unknown.contains('{') || unknown.contains('}') {
                return Err(SortSmithError::InvalidRule(
                    "rename template contains an unknown placeholder".into(),
                ));
            }
        }
    }
    Ok(())
}

fn validate_values(values: &[String], maximum: usize, label: &str, rule_name: &str) -> Result<()> {
    if values.is_empty()
        || values.len() > maximum
        || values
            .iter()
            .any(|value| value.trim().is_empty() || value.chars().count() > 128)
    {
        return Err(SortSmithError::InvalidRule(format!(
            "'{rule_name}' has invalid {label} values"
        )));
    }
    Ok(())
}

pub fn rule_matches(rule: &Rule, file: &FileEntry) -> Result<bool> {
    Ok(PreparedRule::new(rule)?.matches(file))
}

pub fn destination_for(root: &Path, file: &FileEntry, rule: &Rule) -> Result<PathBuf> {
    let name = file
        .path
        .file_name()
        .ok_or_else(|| SortSmithError::InvalidRule("file has no name".into()))?;
    match &rule.action {
        RuleAction::MoveTo { subdirectory } => {
            Ok(crate::safety::safe_subdirectory(root, subdirectory)?.join(name))
        }
        RuleAction::RenamePrefix { prefix } => {
            crate::safety::validate_filename_fragment(prefix, "rename prefix")?;
            let rendered = format!("{prefix}{}", name.to_string_lossy());
            crate::safety::validate_filename(&rendered, "renamed file")?;
            Ok(file.path.with_file_name(rendered))
        }
        RuleAction::RenameTemplate { template } => {
            crate::safety::validate_filename_fragment(template, "rename template")?;
            let stem = file
                .path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("file");
            let ext = file.extension.as_deref().unwrap_or("");
            let mut rendered = template.replace("{name}", stem).replace("{ext}", ext);
            if rendered.contains('/') || rendered.contains('\\') || rendered.contains("..") {
                return Err(SortSmithError::InvalidRule(
                    "rename template cannot contain path separators or '..'".into(),
                ));
            }
            if rendered.trim().is_empty() {
                return Err(SortSmithError::InvalidRule(
                    "rename template cannot produce an empty name".into(),
                ));
            }
            if !ext.is_empty() && !rendered.ends_with(&format!(".{ext}")) {
                rendered.push('.');
                rendered.push_str(ext);
            }
            crate::safety::validate_filename(&rendered, "renamed file")?;
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
            criteria: vec![RuleCriterion::Extension {
                values: vec!["txt".into()],
            }],
            action,
        }
    }

    #[test]
    fn prepared_regex_matches_repeatedly() {
        let mut candidate = rule(RuleAction::MoveTo {
            subdirectory: "Text".into(),
        });
        candidate.criteria = vec![RuleCriterion::NameRegex {
            pattern: r"^report\.txt$".into(),
        }];
        let prepared = PreparedRule::new(&candidate).unwrap();
        assert!(prepared.matches(&entry()));
        assert!(prepared.matches(&entry()));
    }

    #[test]
    fn rejects_unsafe_rename_prefix() {
        let result = destination_for(
            Path::new("/tmp/root"),
            &entry(),
            &rule(RuleAction::RenamePrefix {
                prefix: "../".into(),
            }),
        );
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_size_range() {
        let mut candidate = rule(RuleAction::MoveTo {
            subdirectory: "Text".into(),
        });
        candidate.criteria = vec![RuleCriterion::SizeRange {
            min_bytes: Some(20),
            max_bytes: Some(10),
        }];
        assert!(validate_rule(&candidate).is_err());
    }

    #[test]
    fn rejects_unknown_template_placeholder() {
        let candidate = rule(RuleAction::RenameTemplate {
            template: "{name}-{unknown}.{ext}".into(),
        });
        assert!(validate_rule(&candidate).is_err());
    }

    #[test]
    fn renders_safe_template_and_preserves_extension() {
        let candidate = rule(RuleAction::RenameTemplate {
            template: "{name}-sorted.{ext}".into(),
        });
        validate_rule(&candidate).unwrap();
        let result = destination_for(Path::new("/tmp/root"), &entry(), &candidate).unwrap();
        assert_eq!(result.file_name().unwrap(), "report-sorted.txt");
    }

    #[test]
    fn rejects_reserved_rendered_filename() {
        let candidate = rule(RuleAction::RenameTemplate {
            template: "CON.{ext}".into(),
        });
        validate_rule(&candidate).unwrap();
        assert!(destination_for(Path::new("/tmp/root"), &entry(), &candidate).is_err());
    }

    #[test]
    fn rejects_trailing_period_for_extensionless_file() {
        let candidate = rule(RuleAction::RenameTemplate {
            template: "{name}.".into(),
        });
        validate_rule(&candidate).unwrap();
        let mut extensionless = entry();
        extensionless.path = PathBuf::from("/tmp/root/report");
        extensionless.relative_path = PathBuf::from("report");
        extensionless.extension = None;
        assert!(destination_for(Path::new("/tmp/root"), &extensionless, &candidate).is_err());
    }

    #[test]
    fn accepts_128_unicode_characters_in_rule_values() {
        let value = "é".repeat(128);
        let mut candidate = rule(RuleAction::MoveTo {
            subdirectory: "Text".into(),
        });
        candidate.criteria = vec![RuleCriterion::Extension {
            values: vec![value],
        }];
        assert!(validate_rule(&candidate).is_ok());
    }

    #[test]
    fn rejects_129_unicode_characters_in_rule_values() {
        let value = "é".repeat(129);
        let mut candidate = rule(RuleAction::MoveTo {
            subdirectory: "Text".into(),
        });
        candidate.criteria = vec![RuleCriterion::Extension {
            values: vec![value],
        }];
        assert!(validate_rule(&candidate).is_err());
    }
}
