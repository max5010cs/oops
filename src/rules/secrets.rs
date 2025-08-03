use crate::matchers::Rule;

pub fn load() -> Vec<Rule> {
    const RULES_JSON: &str = include_str!("../../rules/secrets.json");

    let rules: Vec<Rule> = serde_json::from_str(RULES_JSON).unwrap_or_else(|e| {
        eprintln!("[oops::secrets] ❌ Failed to parse embedded secrets.json: {}", e);
        vec![]
    });

    if rules.is_empty() {
        eprintln!("[oops::secrets] ⚠️ No rules loaded from embedded secrets.json");
    } else {
        println!("[oops::secrets] ✅ Loaded {} embedded rule(s)", rules.len());
    }

    rules
}
