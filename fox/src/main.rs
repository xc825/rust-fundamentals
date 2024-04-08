use std::io;
fn longest_word(sentence: &str) -> &str {
    let words = sentence.split_whitespace();
    let mut longest = "";
    for word in words {
        if word.len() > longest.len() {
            longest = word;
        }
    }
    longest
}

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    //println!("{}", &sentence[0..=4]);

    // concatenate using format!
    let _description = format!("Title: Quick story\n{}", sentence);
    //println!("{}", description);

    // //iterate over the characters in the sentence
    // for c in sentence.chars() {
    //     match c {
    //         'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel!"),
    //         _ => continue,
    //     }
    // }

    // Split and collect into a vector
    //let words: Vec<&str> = sentence.split_whitespace().collect();
    let _words = sentence.split(' ').collect::<Vec<&str>>();
    //println!("{:?}", words);

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);

    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
    for vow in vowels {
        println!("{}:{}", vow, sentence.matches(vow).count());
    }

    let mut input = String::new();
    println!("Please enter your input:");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Longest word is: {}", longest_word(input.trim()));
        }
        Err(error) => println!("Error: {}", error),
    }
}
