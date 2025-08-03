use crate::matchers::MatchResult;
use colored::*;

pub fn colorize_risk(risk: &str) -> ColoredString {
    match risk {
        "High" => risk.red().bold(),
        "Medium" => risk.yellow().bold(),
        "Low" => risk.green().bold(),
        _ => risk.normal(),
    }
}

pub fn print_results(results: &[(std::path::PathBuf, Vec<MatchResult>)]) {
    if results.is_empty() {
        println!("{} No issues found!", "[oops]".green().bold());
        return;
    }

    println!(
        "\n{} Found {} issue(s). Displaying results:\n",
        "[oops]".bold().purple(),
        results.iter().map(|(_, ms)| ms.len()).sum::<usize>().to_string().bold()
    );

    for (file, matches) in results {
        for m in matches {
            println!("{} {}:{}", "📁 File".blue().bold(), file.display(), m.line_number);
            println!("{} {}", "⚠️  Risk".blue().bold(), colorize_risk(&m.rule.risk));
            println!("{} {}", "📚 Description".blue().bold(), m.rule.description);
            println!("{} {}", "🔍 Pattern".blue().bold(), m.rule.pattern);
            println!("{} {}", "🧠 Line Content".blue().bold(), m.line_content.trim());
            println!("{}", "—".repeat(60).dimmed());
        }
    }

    let total_issues: usize = results.iter().map(|(_, ms)| ms.len()).sum();
    println!(
        "\n{} Scan complete — {} issues found.",
        "[oops]".green().bold(),
        total_issues.to_string().bold()
    );
}

pub fn print_intro() {
    let banner = r#"
   ____                        __         
  / __ \___  ____  ____  _____/ /__  _____
 / / / / _ \/ __ \/ __ \/ ___/ / _ \/ ___/
/ /_/ /  __/ /_/ / /_/ / /__/ /  __/ /    
\____/\___/ .___/ .___/\___/_/\___/_/     
         /_/   /_/                        

"#;
    println!("{}", banner.bright_magenta().bold());
    println!(
        "{} A real-time code security companion for your terminal.",
        "[oops]".bright_magenta().bold()
    );
    println!(
        "{} Type '{}' to get started.",
        "[oops]".bright_magenta(),
        "oops help".cyan()
    );
    println!();
}

pub fn print_help() {
    println!("{}", "[oops] Usage:".bright_yellow());
    println!("  oops run [dir] [--ignore path1 path2 ...]   Run a scan");
    println!("  oops watch [dir] [--ignore path1 path2 ...] Watch files live");
    println!("  oops about                                   Show info");
    println!("  oops help                                    Show this message");
}

pub fn print_about() {
    println!("{}", "[oops] About".bright_green().bold());
    println!("Built by Max Yuldashev in Rust.");
    println!("Real-time code scanner for catching risky patterns before you commit.");
    println!("GitHub: https://github.com/yourusername/oops");
}

