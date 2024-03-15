struct User {
    username: String,
    password: String,
}

impl User {
    fn say_hi(&mut self) {
        println!("Hi, I'm {}", self.username);
    }

    fn change_password(&mut self, new_password: String) {
        self.password = new_password;
    }
}

fn main() {
    let mut user1 = User {
        username: String::from("user1"),
        password: String::from("password1"),
    };

    user1.say_hi();
    user1.change_password(String::from("password2"));

    println!("The new password is {}", user1.password);
}
