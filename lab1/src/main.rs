use std::io;
use std::io::Write;


fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn average(numbers: &[i32]) -> f64 {
    let sum = sum(numbers) as f64;
    let count = numbers.len() as f64;
    sum / count
}

fn ask_numbers(n: usize) -> Vec<i32> {
    let mut numbers = Vec::new();
    for i in 0..n {
        print!("Enter number {}: ", i + 1);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let number: i32 = input.trim().parse().expect("Invalid input");
        numbers.push(number);
    }
    numbers
}

fn main() {
    let numbers = ask_numbers(5);
    let result = sum(&numbers);
    println!("The sum is {}", result);
    println!("The average is {:.2}", average(&numbers));
}
