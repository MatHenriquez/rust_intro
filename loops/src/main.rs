fn main() {

    let mut count = 0;

    loop {
        count += 1;
        println!("{}", count);

        if count == 5 {
            break;
        }
    }
}
