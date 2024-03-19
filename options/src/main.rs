// enum Option<T> { // Option -> is an enum
//     Some(T), // Some -> is a variant of Option
//     None, // None -> is a variant of Option
// }

fn get_value(flag: bool) -> Option<String> {
    if flag {
        Some(String::from("Hello, world! From get_value!"))
    } else {
        None
    }
}

fn main() {
    // Option -> is an enum that can have two values: Some and None
    // Result -> is an enum that can have two values: Ok and Err

    let message = get_value(true); // This will return Some("Hello, world! From get_value!") because the function returns a value
    match message {
        Some(msg) => println!("{}", msg), // This will print the message
        None => println!("No message"), // This will not be executed
    }

    let message = get_value(false); // This will return None because the function returns no value
    match message {
        Some(msg) => println!("{}", msg), // This will not be executed
        None => println!("No message"), // This will print "No message"
    }

    // unwrap method -> is a method that returns the value if it's Some or panics if it's None
    let message = get_value(true).unwrap(); // This will return "Hello, world! From get_value!" because the function returns a value
    println!("{}", message); // This will print the message

    let message = get_value(false).unwrap(); // This will panic because the function returns no value
    println!("{}", message); // This will not be executed
}
