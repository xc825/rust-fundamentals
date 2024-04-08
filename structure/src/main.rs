
#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    email: Option<String>,
    phone_number: Option<String>,
}

impl Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {
    let mut person = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
        email: Some("structs@google.com".to_string()),
        phone_number: None,
    };
    println!("{:?}", person);
    println!("Full name: {}", person.full_name());
    println!("Happy birthday, {}!", person.first_name);
    person.age += 1;
    println!("You are now {} years old.", person.age);
}
