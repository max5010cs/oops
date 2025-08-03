use crate::matchers::Rule;

pub fn load() -> Vec<Rule> {
    const RULES_JSON: &str = include_str!("../../rules/permissions.json");

    let rules: Vec<Rule> = serde_json::from_str(RULES_JSON).unwrap_or_else(|e| {
        eprintln!("[oops::permissions] ❌ Failed to parse embedded permissions.json: {}", e);
        vec![]
    });

    if rules.is_empty() {
        eprintln!("[oops::permissions] ⚠️ No rules loaded from embedded permissions.json");
    } else {
        println!("[oops::permissions] ✅ Loaded {} embedded rule(s)", rules.len());
    }

    rules
}
