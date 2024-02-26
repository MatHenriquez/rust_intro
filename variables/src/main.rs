fn main() {
    // Inmutable variables
    let first_number = 10;
    println!("The value of first_number is: {}", first_number);

    let second_number: i32 = 20; // Is optional to specify the type of the variable
    let sum = first_number + second_number;

    println!("The value of sum is: {}", sum);

    println!("The sum is: {} + {} = {}", first_number, second_number, sum);

    // Mutable variable
    let mut mutable_number = 30;
    println!("The value of mutable_number is: {}", mutable_number);

    mutable_number = 40;
    println!("The value of mutable_number is: {}", mutable_number);

    // Constants
    const PI: f32 = 3.1416; // Is mandatory to specify the type of the constant
    static LANGUAGE: &str = "Rust";
    println!("The value of PI is: {}", PI);
    println!("The value of LANGUAGE is: {}", LANGUAGE);
}
