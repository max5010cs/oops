use crate::matchers::Rule;

pub fn load() -> Vec<Rule> {
    const RULES_JSON: &str = include_str!("../../rules/misconfig.json");

    let rules: Vec<Rule> = serde_json::from_str(RULES_JSON).unwrap_or_else(|e| {
        eprintln!("[oops::misconfig] ❌ Failed to parse embedded misconfig.json: {}", e);
        vec![]
    });

    if rules.is_empty() {
        eprintln!("[oops::misconfig] ⚠️ No rules loaded from embedded misconfig.json");
    } else {
        println!("[oops::misconfig] ✅ Loaded {} embedded rule(s)", rules.len());
    }

    rules
}
