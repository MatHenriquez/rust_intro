fn main() {
    let message = String::from("Hello, world! From main!");

    { // Block 2: new scope
        println!("message from block 2: {}", message);
        let message = String::from("Hello, world! From block 2!");
        { // Block 3: new scope
            println!("message from block 3: {}", message); // This will print the message from block 2 because it's the closest scope
            let result = message.len() + 10; // This variable will be dropped at the end of this block
        }
    }

    let new_message = String::from("Hello, world! From main! Again!"); // This is a new variable, not the same as the first one

    {
        let borrowed_message = &new_message; // This is a reference to the message variable
    }

    println!("message from main: {}", new_message); // This will print the message from the main scope
}
