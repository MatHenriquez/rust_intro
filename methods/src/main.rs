struct User {
    username: String,
    password: String,
}

impl User {
    fn say_hi(&mut self) {
        println!("Hi, I'm {}", self.username);
    }
}

fn main() {
    let mut user1 = User {
        username: String::from("user1"),
        password: String::from("password1"),
    };

    user1.say_hi();
}
