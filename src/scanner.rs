use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::matchers::{Rule, MatchResult};
use notify::{Watcher, RecursiveMode, RecommendedWatcher, Config, Event, EventKind};
use std::sync::mpsc::channel;
use colored::Colorize;
use rand::prelude::*; // ✅ includes SliceRandom and rng() in one line

const ALLOWED_EXTENSIONS: &[&str] = &[
    "rs", "js", "ts", "jsx", "tsx", "html", "css", "py", "sh", "go", "php", "c", "cpp"
];

const IGNORED_DIRS: &[&str] = &[
    "target", "node_modules", "dist", "build", ".git", ".vscode", "__pycache__"
];

const FUNNY_COMMENTS: &[&str] = &[
    "Oops! You really wanna push this?",
    "Well, that's a new flavor of disaster.",
    "Found something fishy!",
    "Did you mean to write that?",
    "Code smells like production-level panic.",
    "Nope. Try again, cowboy.",
    "Security gods are judging you right now.",
    "I wouldn’t deploy that if I were you...",
    "Imagine pushing this to main.",
    "Quick fix? More like quick fail.",
    "The bug says hello!",
    "Yikes. Hope no one sees this.",
    "404: Good practices not found.",
    "Too late to blame the intern.",
    "This is why we can’t have nice things.",
    "Oops! That line made me cringe.",
    "Code gremlins are real.",
    "A wild bug appears!",
    "Commit this and pray.",
    "Push it and run."
];

pub fn scan_dir(root: &Path, rules: &[Rule], ignore_list: &[&str]) -> Vec<(PathBuf, Vec<MatchResult>)> {
    let mut all_matches = Vec::new();
    println!("{} Scanning directory: {}", "[oops]".bright_magenta().bold(), root.display());

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            !IGNORED_DIRS.iter().any(|d| e.path().components().any(|c| c.as_os_str() == *d)) &&
            !ignore_list.iter().any(|i| e.path().display().to_string().contains(i))
        })
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map_or(false, |ext| ALLOWED_EXTENSIONS.contains(&ext))
        })
    {
        let path = entry.path();
        println!("{} {} — scanning...", "[oops]".cyan(), path.display());
        let matches = scan_file(path, rules);
        if !matches.is_empty() {
            all_matches.push((path.to_path_buf(), matches));
        }
    }

    println!("\n{} Scan complete.", "[oops]".green().bold());
    all_matches
}

pub fn scan_file(file_path: &Path, rules: &[Rule]) -> Vec<MatchResult> {
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => return vec![],
    };

    let mut results = vec![];

for rule in rules {
    if let Some(re) = rule.to_regex() {
        for mat in re.find_iter(&content) {
            let start = mat.start();
            let line_number = content[..start].matches('\n').count() + 1;

            // Get the full line containing the match
            let line_start = content[..start].rfind('\n').map_or(0, |pos| pos + 1);
            let line_end = content[start..].find('\n').map_or(content.len(), |pos| start + pos);
            let line_content = &content[line_start..line_end];

            results.push(MatchResult {
                rule: rule.clone(),
                line_number,
                line_content: line_content.to_string(),
            });
        }
    }
}

    results
}

pub fn start_watch(path: &Path, rules: &[Rule], ignore_list: &[&str]) {
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher =
        Watcher::new(tx, Config::default()).expect("Failed to initialize watcher");
    watcher
        .watch(path, RecursiveMode::Recursive)
        .expect("Failed to watch path");

    let mut rng = rand::rng(); // ✅ new API


    loop {
        match rx.recv() {
            Ok(Ok(Event { kind: EventKind::Modify(_), paths, .. })) => {
                for path in paths {
                    if !path.is_file() {
                        continue;
                    }

                    let ext_ok = path.extension()
                        .and_then(|ext| ext.to_str())
                        .map_or(false, |ext| ALLOWED_EXTENSIONS.contains(&ext));

                    let ignored = IGNORED_DIRS.iter().any(|d| path.components().any(|c| c.as_os_str() == *d)) ||
                                   ignore_list.iter().any(|i| path.display().to_string().contains(i));

                    if ext_ok && !ignored {
                        let matches = scan_file(&path, rules);
                        if !matches.is_empty() {
                            println!("\n{} {}", "[oops] Detected issue in".red().bold(), path.display());
                            for m in matches {
                                println!("{}:{} — {}", path.display(), m.line_number, m.rule.name.yellow());
                                println!("{}: {}", "Description".blue(), m.rule.description);
                                println!("{}: {}", "Risk".blue(), m.rule.risk);

                                if let Some(msg) = FUNNY_COMMENTS.choose(&mut rng) {
                                    println!("{} {}", "💬".italic().dimmed(), msg.italic().dimmed());
                                }

                                println!("{}\x07", "[oops] Alert!".bold().red()); // terminal bell
                            }
                        }
                    }
                }
            }
            Ok(_) => {}
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
