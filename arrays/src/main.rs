fn main() {
    let numbers = [1, 2, 3, 4, 5]; // Array size is fixed
    println!("First number: {}", numbers[0]);
    println!("Numbers: {:?}", numbers);

    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Numbers: {:?}", numbers);

    let values = [5.5; 10]; // [5.5, 5.5, 5.5, 5.5, 5.5, 5.5, 5.5, 5.5, 5.5, 5.5,]
    println!("Values: {:?}", values);

    let first_number = numbers[0];
    let last_number = numbers[numbers.len() - 1];
    println!("First number: {}", first_number);
    println!("Last number: {}", last_number);

    numbers[0] = 10;
    println!("First number: {}", numbers[0]);
}
