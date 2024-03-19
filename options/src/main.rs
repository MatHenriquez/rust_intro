// enum Option<T> { // Option -> is an enum
//     Some(T), // Some -> is a variant of Option
//     None, // None -> is a variant of Option
// }

fn get_value() -> Option<String> {
    Some(String::from("Hello, world! From get_value!"))
}

fn main() {
    // Option -> is an enum that can have two values: Some and None
    // Result -> is an enum that can have two values: Ok and Err

    let message = get_value(); // This will return Some("Hello, world! From get_value!") because the function returns a value
    match message {
        Some(msg) => println!("{}", msg), // This will print the message
        None => println!("No message"), // This will not be executed
    }
}
