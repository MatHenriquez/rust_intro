use std::io;

fn main() {
    // let color = "red";

    let mut color = String::new();
    println!("Enter the color: ");
    io::stdin().read_line(&mut color);
    let color = color.trim().to_lowercase();

    if color == "green" {
        println!("Go ahead");
    } else if color == "red" {
        println!("STOP!");
    } else if color == "yellow" {
        println!("Be prepared to stop");
    } else {
        println!("Invalid color");
    }
}
