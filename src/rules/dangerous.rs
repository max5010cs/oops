use crate::matchers::Rule;

pub fn load() -> Vec<Rule> {
    const RULES_JSON: &str = include_str!("../../rules/dangerous.json");

    let rules: Vec<Rule> = serde_json::from_str(RULES_JSON).unwrap_or_else(|e| {
        eprintln!("[oops::dangerous] ❌ Failed to parse embedded dangerous.json: {}", e);
        vec![]
    });

    if rules.is_empty() {
        eprintln!("[oops::dangerous] ⚠️ No rules loaded from embedded dangerous.json");
    } else {
        println!("[oops::dangerous] ✅ Loaded {} embedded rule(s)", rules.len());
    }

    rules
}
