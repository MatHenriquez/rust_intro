fn main() {
    let number: i32 = 50;

    match number {
        1 => println!("It's one"),
        2 => println!("It's two"),
        3 => println!("It's three"),
        4 | 5 => println!("It's four or five"),
        6..=100 => {
            println!("It's greater than five");
            println!("It's less than or equal to one hundred");
        },
        _ => println!("It's something else"),
    }
}
