struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 { // Borrowing a reference to a Rectangle instance
    rectangle.width * rectangle.height
}

fn main() {
    // Ownership is Rust’s most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector.

    // Ownership Rules:
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    let rectangle1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rectangle1Area = area(&rectangle1); // Borrowing a reference to a Rectangle instance

    println!("The area of the rectangle is {} square pixels.", rectangle1Area);

    // The following code will not compile because the rectangle1 is borrowed as immutable and cannot be borrowed as mutable
    // let rectangle1Mut = &mut rectangle1;
    // rectangle1Mut.width = 60;
    // println!("The area of the rectangle is {} square pixels.", area(&rectangle1Mut));

    let new_rectangle = rectangle1; // This is a move operation, the ownership of rectangle1 is moved to new_rectangle
    println!("The area of the rectangle is {} square pixels.", area(&new_rectangle));

    // The following code will not compile because the rectangle1 is moved to new_rectangle
    // println!("The area of the rectangle is {} square pixels.", area(&rectangle1));
}
