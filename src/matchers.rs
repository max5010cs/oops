#![allow(dead_code)]

use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Rule {
    pub name: String,
    pub pattern: String,
    pub description: String,
    pub risk: String,
    pub category: String,
    pub reference: String,
}

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub rule: Rule,
    pub line_number: usize,
    pub line_content: String,
}

impl Rule {
    pub fn to_regex(&self) -> Option<Regex> {
        Regex::new(&self.pattern).ok()
    }
}
