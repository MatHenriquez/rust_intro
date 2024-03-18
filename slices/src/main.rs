fn main() {
    // A slice is a reference to a contiguous sequence of elements in a collection. Are like references to arrays, but they don't have a fixed size.
    // Slices -> Heap-allocated data structures
    // Arrays -> Stack-allocated data structures

    let message = String::from("Hello, world!");
    println!("message: {}", message);

    let hello = &message[0..5]; // [start..end] -> start is inclusive, end is exclusive
    let world = &message[7..12];

    println!("hello: {}", hello);
    println!("world: {}", world);
}
