use std::io;
use std::env;

// a unit function that doesn't return anything
fn print_sum(numbers: &[i32]) {
    let sum: i32 = numbers.iter().sum(); // Calculate the sum of elements in slice
    if sum % 2 == 0 {               // Check if sum is even
        println!("The sum is even.");
    } else {
        println!("The sum is odd.");
    }
}

fn enter_word() {
    let mut input = String::new();
    while  input.trim() != "stop" {
        input.clear();
        println!("Please enter a word (type 'stop' to exit): ");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        println!("You entered: {}", input);
    }
}

fn process_numbers(slice: &[i32]) {
    for (index, number) in slice.iter().enumerate() {
        if *number < 0 {
            panic!("Negative number found at index {}", index); // Stop execution and show error message
        }
    }
}

fn split_string(s:String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    let result = parts.get(field);
    result.expect("Ups! Problem! Panic!!!").to_string()
}

fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn main() {

    let numbers: [i32; 3]= [1, 2, 3];      // Define a slice of integers
    print_sum(&numbers);          // Call the unit function with the slice as an argument

    println!("sum of numbers: {}", sum(&numbers));
    //env::set_var("RUST_BACKTRACE", "1");
    //let numbers = [1, 2, 3, -5];   // Include a negative number to trigger the panic
    //process_numbers(&numbers);

    let chunk: String = split_string("Hello world".to_string(), ' ', 2);
    println!("Split string: {}", chunk);

    println!("Goodbye!");
}