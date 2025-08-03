// --- matchers.rs ---
use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Rule {
    pub name: String,
    pub pattern: String,
    pub description: String,
    pub risk: String,
    pub reference: String,
    pub category: String,
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

    pub fn severity_color(&self) -> colored::ColoredString {
        match self.risk.to_lowercase().as_str() {
            "high" => self.risk.red().bold(),
            "medium" => self.risk.yellow().bold(),
            "low" => self.risk.green().bold(),
            _ => self.risk.normal(),
        }
    }
}
