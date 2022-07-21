use std::env;
use std::fs;
use std::fs::DirEntry;
use regex::Regex;

fn main() {

    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3, "Wrong number of arguments provided");
    let regex = Regex::new(args.get(1).unwrap()).expect("Invalid regex in argument 1");
    let template = args.get(2).unwrap();

    for file in fs::read_dir(".").unwrap() {
        if let Ok(file_result) = file {
            process_file(&regex, &template, file_result).expect("Error occurred");
        }
    }
}

fn process_file(regex: &Regex, template: &str, file: DirEntry) -> Result<String, String>{
    let filename = file.file_name().to_str().unwrap().to_string();
    if regex.is_match(&filename) {
        let captures = regex.captures(&filename).unwrap();
        let mut new_name = template.to_string();
        for (i, capture) in captures.iter().enumerate() {
            let backref_finder = Regex::new(format!("\\$\\({}\\)", i).as_str()).unwrap();
            new_name = backref_finder.replace_all(&new_name, capture.unwrap().as_str()).to_string();
        }
        fs::rename(filename, new_name).expect("Renaming failed");
    }
    Ok("Hello".to_string())
}