use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3, "Wrong number of arguments provided");
    let regex_1 = Regex::new(args.get(1).unwrap()).expect("Invalid regex in argument 1");
    let regex_2 = Regex::new(args.get(2).unwrap()).expect("Invalid regex in argument 2");

    for file in fs::read_dir(".").unwrap() {
        println!("{} | {}", file.as_ref().unwrap().file_name().to_str().unwrap(), regex_1.is_match(file.as_ref().unwrap().file_name().to_str().unwrap()));
    }
}