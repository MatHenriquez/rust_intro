fn main() {
    // Inmutable variables
    let first_number = 10;
    println!("The value of first_number is: {}", first_number);

    let second_number: i32 = 20;
    let sum = first_number + second_number;

    println!("The value of sum is: {}", sum);

    println!("The sum is: {} + {} = {}", first_number, second_number, sum);

    // Mutable variable
    let mut mutable_number = 30;
    println!("The value of mutable_number is: {}", mutable_number);

    mutable_number = 40;
    println!("The value of mutable_number is: {}", mutable_number);
}
