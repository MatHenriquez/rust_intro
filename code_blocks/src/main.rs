fn main() {
    let message_main = "Hello, world from main() block!";
    println!("{}", message_main);

    {
        let message_inner = "Hello, world from inner block!"; // Scope: message_inner is accessible only within the block
        println!("{}", message_inner);
        println!("{} (from inner block)", message_main); // Scope: message_main is accessible from inner block but not vice versa
    }

    let returned_value = {
        println!("Hello from another block!");

        let variable: i32 = 200;
        println!("The value of variable is: {}", variable);

        variable // The value of variable is returned from the block.
    };

    println!("The value of returned_value is: {}", returned_value);
} // When the block ends, the variable is destroyed

// Note: shadowing respect the scope of the variable. It does not affect the variable outside the block.
