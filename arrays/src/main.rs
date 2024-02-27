fn main() {
    let numbers = [1, 2, 3, 4, 5];
    println!("First number: {}", numbers[0]);
    println!("Numbers: {:?}", numbers);

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Numbers: {:?}", numbers);

    let values = [5.5; 10]; // [5.5, 5.5, 5.5, 5.5, 5.5, 5.5, 5.5, 5.5, 5.5, 5.5,]
    println!("Values: {:?}", values);
}
