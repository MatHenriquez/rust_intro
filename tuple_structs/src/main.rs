#[derive(Debug)] // to print the struct with {:?}
struct Color (u32, u32, u32); // tuple struct with 3 elements 

fn main() {
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    println!("black: {:?}", black);
    println!("white: {:?}", white);

    let mut custom_color = Color(100, 200, 150);
    println!("custom_color: {:?}", custom_color);

    // change the value of the first element
    custom_color.0 = custom_color.0 + 10;   
}
