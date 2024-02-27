fn main() {
    let tuple = (1, false, 5.5); // Tuple can store different types
    println!("Tuple: {:?}", tuple);

    let mut tuple: (i32, bool, f64) = (1, false, 5.5);
    println!("Tuple: {:?}", tuple);

    let (x, y, z) = tuple; // Destructuring
    println!("x: {}, y: {}, z: {}", x, y, z);

    let first_element = tuple.0; // Accessing elements by index
    let last_element = tuple.2;
    println!("First element: {}, Last element: {}", first_element, last_element);

    tupla.1 = true; // Updating tuple element
    println!("Tuple: {:?}", tuple);
}
