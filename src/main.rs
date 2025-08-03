mod matchers;
mod scanner;
mod reporter;
mod rules;

use std::env;
use std::path::Path;
use walkdir::WalkDir;
use crate::rules::load_all_rules;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = args.get(1).map(String::as_str).unwrap_or(".");
    let rules = load_all_rules();

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "rs" || ext == "js" || ext == "py" || ext == "ts" || ext == "go" {
                    let matches = scanner::scan_file(path, &rules);
                    if !matches.is_empty() {
                        println!("\nFile: {}", path.display());
                        reporter::print_report(&matches);
                    }
                }
            }
        }
    }
}
