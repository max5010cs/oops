mod matchers;
mod scanner;
mod reporter;
mod rules;

use std::env;
use std::path::Path;
use walkdir::WalkDir;
use crate::rules::load_all_rules;
use scanner::{scan_dir, start_watch};
use reporter::{print_intro, print_help, print_about, print_watch_message, report_all};

fn main() {
    let args: Vec<String> = env::args().collect();
    print_intro();

    if args.len() < 2 {
        print_help();
        return;
    }

    let command = args[1].as_str();
    let mut target_dir = ".";
    let mut ignore_list: Vec<String> = vec![];

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--ignore" => {
                i += 1;
                while i < args.len() && !args[i].starts_with("--") {
                    ignore_list.push(args[i].clone());
                    i += 1;
                }
            }
            other => {
                target_dir = other;
                i += 1;
            }
        }
    }

    let rules = load_all_rules();

    match command {
        "run" => {
            let results = scan_dir(Path::new(target_dir), &rules, &ignore_list);
            report_all(&results);
        }
        "watch" => {
            print_watch_message();
            start_watch(Path::new(target_dir), &rules, &ignore_list);
        }
        "about" => print_about(),
        "help" => print_help(),
        _ => print_help(),
    }
}
