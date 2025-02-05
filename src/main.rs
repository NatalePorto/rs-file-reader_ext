use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).expect("Please provide the file path as first argument");
    
    let file = File::open(file_name);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}