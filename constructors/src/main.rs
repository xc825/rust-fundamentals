#[allow(dead_code)]
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn from_email(email: String) -> Self {
        Self {
            username: email.split('@').next().unwrap().to_string(),
            email,
            uri: String::from("https://example.com"),
            active: true,
        }
    }
    fn update_uri(&mut self, uri: String) {
        println!("Updating URI for {}. [{}]->[{}]", self.username, self.uri, uri);
        self.uri = uri;
    }
    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main() {
    let mut new_user = User::new(
        String::from("alfredodeza"),
        String::from("alfreodeza@example.com"),
        String::from("https://alfredodeza.com"),
    );
    println!("Hello, {}!", new_user.username);
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status is: {}", new_user.username, new_user.active);

    let mut john = User::from_email(String::from("john@beer.com"));
    println!("Hello, {}!", john.username);
    println!("John: {:?}", john);
    john.update_uri(String::from("https://john.com"));
    println!("John's new URI: {}", john.uri);
}
