use std::io;

fn main() {
    println!("Enter your nickname: ");
    let mut nickname = String::new(); // new() is an associated function of the String type that returns a new instance of a String type (an empty string in this case)

    // Result is an enum that represents either success (Ok) or failure (Err)
    io::stdin().read_line(&mut nickname); // read_line() is a method of the standard input (io::stdin()) that reads a line from the standard input and stores it in the variable nickname

    let nickname = nickname.trim(); // trim() is a method of the String type that removes the leading and trailing whitespaces from the string
    println!("Your nickname is: {}", nickname);

    println!("Enter your age: ");
    let mut age = String::new();
    io::stdin().read_line(&mut age); // &mut is a mutable reference to the variable age
    let age = age.trim();
    let age: i32 = age.parse().unwrap(); // parse() is a method of the String type that parses a string into a number. unwrap() is a method of the Result type that returns the value of the Ok variant or panics if the value is an Err variant
    println!("Your age is: {}", age);
}