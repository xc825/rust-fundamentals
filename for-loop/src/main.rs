fn main() {
    for i in 1..=10 {
        if i % 2 != 0 {
            continue;
        }
        println!("i = {}", i);
        if i == 2 {
            break;
        }
    }
    let numbers = vec![1, 2, 3, 4, 5];
    for number in numbers.iter() {
        println!("number = {}", number);
    }

    let name = "How are you?";
    match name {
        "Hello" => println!("Hi!"),
        "Goodbye" => println!("Bye!"),
        "How are you?" => println!("{}\nI'm fine, thank you!", name),
        _ => println!("I don't know what you said."),
    }

    use std::collections::HashMap;
    let mut reviews: HashMap<String, String> = HashMap::new();
    
    reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate."));
    reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet recipes."));
    reviews.insert(String::from("Programming in Rust"), String::from("Great examples."));

// Look for a specific review
let book: &str = "Programming in Rust";
println!("\nReview for \'{}\': {:?}", book, reviews.get(book));

// Remove book review
let obsolete: &str = "Ancient Roman History";
println!("\n'{}\' removed.", obsolete);
reviews.remove(obsolete);

// Confirm book review removed
println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));

let big_birds = ["ostrich", "peacock", "stork"];
for bird in big_birds.iter() {
    println!("The {} is a big bird.", bird);
}

    println!("Exit!");
}
