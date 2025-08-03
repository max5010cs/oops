mod rules;

fn main() {
    let secret_rules = rules::secrets::load_secret_rules();

    println!("Loaded {} secret rules:", secret_rules.len());
    for rule in &secret_rules {
        println!("- [{}] {}", rule.name, rule.description);
    }
}

