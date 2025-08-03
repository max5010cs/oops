use crate::matchers::Rule;

pub fn load() -> Vec<Rule> {
    const RULES_JSON: &str = include_str!("../../rules/known_patterns.json");

    let rules: Vec<Rule> = serde_json::from_str(RULES_JSON).unwrap_or_else(|e| {
        eprintln!("[oops::known_patterns] ❌ Failed to parse embedded known_patterns.json: {}", e);
        vec![]
    });

    if rules.is_empty() {
        eprintln!("[oops::known_patterns] ⚠️ No rules loaded from embedded known_patterns.json");
    } else {
        println!("[oops::known_patterns] ✅ Loaded {} embedded rule(s)", rules.len());
    }

    rules
}
