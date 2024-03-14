struct User { // struct definition 
    username: String, // struct fields
    password: String,
}

fn create_user(username: String, password: String) -> User { // function that returns a User
    User { username, password }
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

    let username = String::from("user3");
    let password = String::from("password3");
    let user3 = User { // struct instance with variables
        username, // field init shorthand syntax 
        password,
    };
    println!("Username: {}", user3.username);
    println!("Password: {}", user3.password);

    let user4 = create_user(String::from("user4"), String::from("password4")); // struct instance from function
    println!("Username: {}", user4.username);
    println!("Password: {}", user4.password);

    let mut user5 = User { // mutable struct instance
        username: String::from("user5"),
        password: String::from("password5"),
    };
    user5.username = String::from("user5");
    user5.password = String::from("password");
    println!("Username: {}", user5.username);
    println!("Password: {}", user5.password);
}
