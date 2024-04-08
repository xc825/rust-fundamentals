use std::io;

fn main() {
    println!("Please enter a greeting:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    // use of match expression to pattern match against variable "name"
    match name.trim().to_lowercase().as_str() {
        "good bye" => println!("Sorry to see you go."),
        "hello" => println!("Hi, nice to meet you!"),
        "good morning" => println!("Good morning to You!"),
        "good evening" => println!("It is late!"),
        _ => println!("I can't find a greeting, good bye."),
    }
}
