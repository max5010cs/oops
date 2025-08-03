use crate::matchers::Rule;

pub fn load() -> Vec<Rule> {
    const RULES_JSON: &str = include_str!("../../rules/bad_practices.json");

    let rules: Vec<Rule> = serde_json::from_str(RULES_JSON).unwrap_or_else(|e| {
        eprintln!("[oops::bad_practices] ❌ Failed to parse embedded bad_practices.json: {}", e);
        vec![]
    });

    if rules.is_empty() {
        eprintln!("[oops::bad_practices] ⚠️ No rules loaded from embedded bad_practices.json");
    } else {
        println!("[oops::bad_practices] ✅ Loaded {} embedded rule(s)", rules.len());
    }

    rules
}



