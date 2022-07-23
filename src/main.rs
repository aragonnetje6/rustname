
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

use clap::Parser;
use regex::Regex;

use crate::RenameOutcome::{Changed, Matched, NotMatched, Failed};

#[derive(Parser)]
#[clap(author, version, about, long_about = "Mass renames files and optionally folders based on regex and template string")]
struct Args {
    /// Include subdirectories in the search
    #[clap(short, long, action)]
    recursive: bool,

    /// Also rename folders
    #[clap(short, long, action)]
    folders: bool,

    /// Print out extra information
    #[clap(short, long, action)]
    verbose: bool,

    /// Regex to match filenames against
    #[clap(value_parser)]
    regex_str: String,

    /// Template to build new filenames from
    #[clap(value_parser)]
    template_str: String,
}

fn main() {
    let args = Args::parse();

    let regex = Regex::new(&args.regex_str).expect("Invalid regex in argument 1");

    let mut matched: u32 = 0;
    let mut changed: u32 = 0;
    let mut failed: u32 = 0;

    handle_directory(".", &args, &regex, &mut matched, &mut changed, &mut failed);
    println!(
        "{} files matched, {} files renamed, {} errors",
        matched + changed + failed,
        changed,
        failed
    );
}

fn handle_directory<P: AsRef<Path>>(path: P, args: &Args, regex: &Regex, matched: &mut u32, changed: &mut u32, failed: &mut u32) {
    for file_result in fs::read_dir(path).unwrap() {
        if let Ok(file) = file_result {
            let is_dir = file.file_type().expect("filetype reading failed").is_dir();
            if is_dir & args.recursive {
                handle_directory(file.path(), args, regex, matched, changed, failed);
            }
            if !is_dir | (is_dir & args.folders) {
                let process_result = rename(&regex, &args.template_str, file, args.verbose);
                match process_result {
                    Changed => *changed += 1,
                    Matched => *matched += 1,
                    Failed => *failed += 1,
                    NotMatched => {}
                }
            }
        }
    }
}

enum RenameOutcome {
    Changed,
    Matched,
    NotMatched,
    Failed,
}

fn rename(regex: &Regex, template: &str, file: DirEntry, verbose: bool) -> RenameOutcome {
    let filename = file
        .file_name()
        .to_str()
        .expect("Filename reading failed")
        .to_string();
    if regex.is_match(&filename) {
        let new_name = generate_new_name(regex, template, &filename);
        if filename != new_name {
            if verbose {
                println!("{} -> {}", filename, new_name);
            }
            let rename_res = fs::rename(&filename, &new_name);
            match rename_res {
                Ok(_) => Changed,
                Err(x) => {
                    println!("{}: {}", filename, x);
                    Failed
                }
            }
        } else {
            if verbose {
                println!("{} unchanged", filename);
            }
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
