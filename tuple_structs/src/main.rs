#[derive(Debug)] // to print the struct with {:?}
struct Color (u32, u32, u32); // tuple struct with 3 elements 

fn main() {
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    println!("black: {:?}", black);
    println!("white: {:?}", white);
}
