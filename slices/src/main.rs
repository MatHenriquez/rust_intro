fn main() {
    // A slice is a reference to a contiguous sequence of elements in a collection. Are like references to arrays, but they don't have a fixed size.
    // Slices -> Heap-allocated data structures
    // Arrays -> Stack-allocated data structures

    let message = String::from("Hello, world!");
    println!("message: {}", message);
}
