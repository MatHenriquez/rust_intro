use std::io;

fn main() {
    println!("Enter your nickname: ");
    let mut nickname = String::new(); // new() is an associated function of the String type that returns a new instance of a String type (an empty string in this case)

    io::stdin().read_line(&mut nickname); // read_line() is a method of the standard input (io::stdin()) that reads a line from the standard input and stores it in the variable nickname

    let nickname = nickname.trim(); // trim() is a method of the String type that removes the leading and trailing whitespaces from the string
    println!("Your nickname is: {}", nickname);
}