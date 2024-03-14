struct User { // struct definition 
    username: String, // struct fields
    password: String,
}

fn main() {
    let user = User { // struct instance
        username: String::from("user"),
        password: String::from("password"),
    };
    println!("Username: {}", user.username);
    println!("Password: {}", user.password);

    let user2 = User {
        username: String::from("user2"),
        ..user // struct update syntax
    };
    println!("Username: {}", user2.username);
    println!("Password: {}", user2.password);
}
