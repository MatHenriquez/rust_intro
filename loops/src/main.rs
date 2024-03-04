fn main() {

    let mut count = 0;

    loop {
        count += 1;
        println!("Count is: {}", count);

        if count == 5 {
            break;
        }
    }

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    for number in numbers.iter() {
        println!("Number[i] is: {}", number);
    }

    for number in (1..4) {
        println!("Number[i] is: {}", number);
    }

    let mut count = 10;
    while count != 0 {
        println!("Count is: {}", count);
        count -= 1;
    }
}
