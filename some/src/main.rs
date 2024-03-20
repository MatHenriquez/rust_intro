fn main() {
    let message = Some("Hello, world!");

    match message {
        Some("Hello, world!") => println!("Message is: Hello, world!"),
        Some("Goodbye!") => println!("Message is: Goodbye!"),
        Some(_) => println!("Some other message"),
        None => println!("No message"),
    }
}
