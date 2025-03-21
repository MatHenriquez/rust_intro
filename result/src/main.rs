// enum Result<T, E> { // T: Type, E: Error
//     Ok(T),
//     Err(E),
// }

fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(x / y)
    }
}

#[derive(Debug)]
enum ErrorDivide {
    DivideByZero,
    DivideByNegative, // Add a new arbitrary error
}

fn new_divide(x: f64, y: f64) -> Result<f64, ErrorDivide> {
    if y == 0.0 {
        return Err(ErrorDivide::DivideByZero);
    }

    if y < 0.0 {
        return Err(ErrorDivide::DivideByNegative);
    }

    Ok(x / y)
}

fn main() {
    let result = divide(2.0, 0.0);
    println!("{:?}", result); // Err("Cannot divide by zero")

    let result = divide(6.0, 3.0);
    println!("{:?}", result); // Ok(2.0)

    let result = new_divide(2.0, 0.0);

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => match error {
            ErrorDivide::DivideByZero => println!("Cannot divide by zero"),
            ErrorDivide::DivideByNegative => println!("Cannot divide by negative"),
        },
    }

    // Result methods: unwrap, unwrap_or, expect

    // let result = divide(2.0, 0.0).unwrap(); // panic! with "Cannot divide by zero"
    let result = divide(2.0, 0.0).unwrap_or(0.0); // 0.0
    let result = divide(2.0, -2.0).expect("Cannot divide by negative"); // panic! with "Cannot divide by zero"
}
