fn main() {
    // str -> It's stored in the stack
    // String -> It's stored in the heap

    // Examples

    // str
    let name = "John";
    println!("The name is: {}", name);
    // name.push('n'); This will throw an error because we can't modify a string slice (str)

    // String
    let mut surname = String::from("Doe");
    surname.push(' '); // This will work because we can modify a String
    surname.push('J');
    surname.push('r');
    surname.push('.');

    surname.push_str(" III"); // This method appends a string slice to a String
    println!("The surname is: {}", surname);

    // Converting a str to a String
    let city = "New York";
    let city = city.to_string();
    println!("The city is: {}", city);

    // Converting a String to a str
    let country = String::from("USA");
    let country = &country[..]; // This is a string slice
}
