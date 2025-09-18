// A simple file reader app
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn open_file(path: &String) {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
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
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a file path as a command line argument")
    }

    let file_path = &args[1];
    println!("File path: {}", file_path);

    open_file(file_path);

    // The first argument is the path to that was used to call the program
    println!("My path is: {}.", args[0]);
}
