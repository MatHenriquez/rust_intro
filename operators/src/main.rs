fn main() {
    let number_one: i32 = 10;
    let number_two: i32 = 200;

    let sum = number_one + number_two;
    let sub = number_one - number_two;
    let mul = number_one * number_two;
    let div = number_one / number_two;
    let rem = number_one % number_two;
    let pow = number_one.pow(2);
    let sqrt = (number_one as f64).sqrt();
    let is_major = number_one > number_two;
    let is_minor = number_one < number_two;
    let is_equal = number_one == number_two;
    let is_major_or_equal = number_one >= number_two;
    let is_minor_or_equal = number_one <= number_two;
    let is_different = number_one != number_two;
    let result: bool = 20 + 10 > 100 || true;
    let result2: bool = 20 + 10 > 100 && true;

    println!("Sum: {}, Sub: {}, Mul: {}, Div: {}, Rem: {}", sum, sub, mul, div, rem);
    println!("Pow: {}, Sqrt: {}", pow, sqrt);
    println!("Is major: {}, Is minor: {}, Is equal: {}", is_major, is_minor, is_equal);
    println!("Is major or equal: {}, Is minor or equal: {}", is_major_or_equal, is_minor_or_equal);
    println!("Is different: {}", is_different);
    println!("Result: {}", result);
    println!("Result2: {}", result2);
}
