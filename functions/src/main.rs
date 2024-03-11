fn user_greeting(){
    println!("Hello, user!");
}

fn sum(number_one: i32, number_two: i32) -> i32 {
    number_one + number_two
}

fn factorial(number: u32) -> u32{
    if number == 1 {
        return number; // We use return keyword when we want to return a value from a function before the end of the function
    } else {
        number * factorial(number - 1)
    }
}

fn main() {
    user_greeting();

    let result = sum(10, 20);
    println!("The result is: {}", result);

    let factorial_of_five = factorial(5);
    println!("The factorial of 5 is: {}", factorial_of_five);
}
