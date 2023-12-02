use std::env;
use std::fs::read_to_string;

pub fn read_lines() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    read_to_string(filename.to_string())
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
