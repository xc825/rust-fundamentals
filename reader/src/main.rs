use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(fname: &str) {
    let file = File::open(fname);
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
    // for line in reader.lines() {
    //     match line {
    //         Ok(line) => println!("{}", line),
    //         Err(error) => {
    //             panic!("Error reading line: {}", error)
    //         }
    //     }
    // }
    for (index, line) in reader.lines().enumerate() {
        let line_number = index + 1; // line numbers start at 1
        let line_content = line.unwrap(); // unwrap the Result
        println!("{}: {}", line_number, line_content);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // The first argument is the path that was used to call the program.
    read_file(&args[1]);
}