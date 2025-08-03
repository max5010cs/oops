use std::fs;
use std::path::Path;
use crate::matchers::{Rule, MatchResult};
use regex::Regex;

pub fn scan_file(file_path: &Path, rules: &[Rule]) -> Vec<MatchResult> {
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => return vec![],
    };

    let mut results = vec![];

    for (i, line) in content.lines().enumerate() {
        for rule in rules.iter() {
            if let Some(re) = rule.to_regex() {
                if re.is_match(line) {
                    results.push(MatchResult {
                        rule: rule.clone(),
                        line_number: i + 1,
                        line_content: line.to_string(),
                    });
                }
            }
        }
    }
    results
}