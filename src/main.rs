use std::env;
use std::fs;
use std::fs::DirEntry;

use regex::Regex;

use crate::RenameOutcome::{Changed, Matched, NotMatched};

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3, "Wrong number of arguments provided");
    let regex = Regex::new(args.get(1).unwrap()).expect("Invalid regex in argument 1");
    let template = args.get(2).unwrap();

    let mut matched = 0;
    let mut changed = 0;
    for file in fs::read_dir(".").unwrap() {
        if let Ok(file_result) = file {
            let process_result = rename_file(&regex, &template, file_result);
            match process_result {
                Changed => changed += 1,
                Matched => matched += 1,
                NotMatched => {}
            }
        }
    }
    println!(
        "{} files matched, {} files renamed",
        matched + changed,
        changed
    );
}

enum RenameOutcome {
    Changed,
    Matched,
    NotMatched,
}

fn rename_file(regex: &Regex, template: &str, file: DirEntry) -> RenameOutcome {
    let filename = file
        .file_name()
        .to_str()
        .expect("Filename reading failed")
        .to_string();
    if regex.is_match(&filename) {
        let new_name = generate_new_name(regex, template, &filename);
        if filename != new_name {
            fs::rename(filename, new_name).expect("Renaming failed");
            Changed
        } else {
            Matched
        }
    } else {
        NotMatched
    }
}

fn generate_new_name(regex: &Regex, template: &str, filename: &String) -> String {
    let captures = regex.captures(&filename).expect("Capture failed");
    let mut new_name = template.to_string();
    for (i, maybe_capture) in captures.iter().enumerate() {
        if let Some(capture) = maybe_capture {
            let backref_finder =
                Regex::new(format!("\\$\\({}\\)", i).as_str()).expect("Regex failed");
            new_name = backref_finder
                .replace_all(&new_name, capture.as_str())
                .to_string();
        }
    }
    new_name
}
