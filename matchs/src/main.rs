fn main() {
    let number: i32 = 50;

    match number {
        1 => println!("It's one"),
        2 => println!("It's two"),
        3 => println!("It's three"),
        4 | 5 => println!("It's four or five"), // Match multiple patterns
        6..=100 => {
            println!("It's greater than five");
            println!("It's less than or equal to one hundred");
        },
        _ => println!("It's something else"),
    }

    let message = match number {
        1 => "It's one",
        2 => "It's two",
        3 => "It's three",
        4 | 5 => "It's four or five",
        6..=100 => "It's greater than five and less than or equal to one hundred",
        _ => "It's something else",
    };

    println!("Message is: {:?}", message);

    // FizzBuzz
    for number in 1..=100 {
        match (number % 3, number % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", number),
        }
    }
}
