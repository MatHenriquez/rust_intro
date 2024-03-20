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

fn main() {
    println!("Hello, world!");
}
