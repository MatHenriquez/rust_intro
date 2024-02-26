fn main() {
    // Int types
    // i8, i16, i32, i64, i128 --> Are signed
    // u8, u16, u32, u64, u128 --> Are unsigned
    let first_number: i8 = -128;
    let second_number: u8 = 255;

    println!("First number: {}", first_number);
    println!("Second number: {}", second_number);

    // Char type (UTF-8)
    let letter: char = 'A';

    println!("Letter: {}", letter);

    // Float types
    // f32, f64
    let float_number: f32 = 3.14;
    let double_number: f64 = 3.141592653589793238;

    println!("Float number: {}", float_number);
    println!("Double number: {}", double_number);

    // Boolean type
    let is_true: bool = true;
    let is_false: bool = false;

    println!("Is true: {}", is_true);
    println!("Is false: {}", is_false);
}
