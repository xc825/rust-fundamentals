extern crate my_library;

use my_library::read_stdin;

fn main() {
    println!("Please enter a string:");
    let mut str: String = read_stdin();
    let red_str = my_library::colors::red(&str);
    let green_str = my_library::colors::green(&str);
    let blue_str = my_library::colors::blue(&str);
    str.push_str(" Yesss!!!");
    println!("Red: {}", red_str);
    println!("Green: {}", green_str);
    println!("Blue: {}", blue_str);
    println!("Success!!!:{}", str);
}
