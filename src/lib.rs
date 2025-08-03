pub mod scanner;
pub mod rules;
pub mod matchers;
pub mod reporter;

// Shared struct used across all modules
#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub pattern: String,
    pub risk: String,
    pub category: String,
    pub description: String,
    pub reference: String,
}

impl Rule {
    pub fn new(
        name: String,
        pattern: String,
        risk: String,
        category: String,
        description: String,
        reference: String,
    ) -> Self {
        Rule {
            name,
            pattern,
            risk,
            category,
            description,
            reference,
        }
    }
}
