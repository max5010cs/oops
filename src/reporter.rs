use crate::matchers::Finding;
use colored::*;

pub fn report(findings: Vec<Finding>) {
    for f in findings {
        println!(
            "{} {} in {}:{}",
            "[Oops]".red().bold(),
            f.rule_name.yellow().bold(),
            f.file.blue(),
            f.line_number.to_string().bold()
        );
        println!("   {}", f.line.trim());
        println!("   {}: {}", "Risk".cyan(), f.risk);
        println!("   {}: {}", "Category".cyan(), f.category);
        println!("   {}: {}", "Info".cyan(), f.description);
        println!("   {}: {}\n", "Docs".cyan(), f.reference);
    }
}
