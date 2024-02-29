fn main() {
    let vector = vec![1, 2, 3, 4, 5]; // Vector can store same type and change size
    println!("Vector: {:?}", vector);

    let mut vector = vec![1, 2, 3, 4, 5];
    println!("Vector: {:?}", vector);

    let first_element = vector[0]; // Accessing elements by index
    let last_element = vector[vector.len() - 1];
    println!("First element: {}, Last element: {}", first_element, last_element);

    vector[1] = 10; // Updating vector element
    println!("Vector: {:?}", vector);

    vector.push(6); // Adding element to the end
    println!("Vector: {:?}", vector);

    let last_element = vector.pop(); // Removing element from the end
    println!("Vector: {:?}, Last element: {:?}", vector, last_element);

    let first_element = vector.remove(0); // Removing element by index
    println!("Vector: {:?}, First element: {}", vector, first_element);

    vector.insert(0, -1); // Inserting element with value: -1 at index 0 
    println!("Vector: {:?}", vector);

    let last_number = vector.pop().unwrap(); // Removing last element and unwrapping the result
    println!("Number popped: {}", last_number);
    println!("New vector is: {:?}", vector);
}
