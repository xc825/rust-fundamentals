use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write; // Add missing import for BufWriter
use std::io::BufWriter; // Add missing import for BufWriter


fn read_file(fname: &str) {
    //let file = File::open("non_existent_file.txt");
    let file = File::open(&fname);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                std::io::ErrorKind::PermissionDenied => {
                    panic!("Permission denied(matched error): {}", error)
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


fn write_file(fname: &str) { // Modify function signature to accept &str instead of &fname: String
    let file = File::create(fname);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            panic!("Error creating file: {}", error)
        }
    };
    let mut writer = BufWriter::new(file);
    match writer.write_all(b"Hello, world!\n") {
        Ok(()) => println!("File [{}] written successfully" , fname),
        Err(error) => {
            panic!("Error writing to file[{}]: {}", fname, error)
        }
    }
}

fn main() {
    let fname = "file.txt";
    write_file(&fname);
    read_file(&fname);
    read_file("file2.txt");
}
