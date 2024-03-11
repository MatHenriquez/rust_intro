fn user_greeting(){
    println!("Hello, user!");
}

fn sum(number_one: i32, number_two: i32) -> i32 {
    number_one + number_two
}

fn main() {
    user_greeting();

    let result = sum(10, 20);
    println!("The result is: {}", result);
}
