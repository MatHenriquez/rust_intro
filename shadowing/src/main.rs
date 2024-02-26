fn main() {
    let value: i32 = 10; // Is inmutable by default
    println!("The value of value is: {}", value);

    let value: i32 = 20; // Shadowing the previous value
    println!("The new value of value is: {}", value);

    let value = false; // Shadowing the previous value with a different type
}
