fn main() {
    let message_main = "Hello, world from main() block!";
    println!("{}", message_main);

    {
        let message_inner = "Hello, world from inner block!"; // Scope: message_inner is accessible only within the block
        println!("{}", message_inner);
        println!("{} (from inner block)", message_main); // Scope: message_main is accessible from inner block but not vice versa
    }
} // When the block ends, the variable is destroyed

// Note: shadowing respect the scope of the variable. It does not affect the variable outside the block.
