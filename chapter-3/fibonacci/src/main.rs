fn main() {
    print!("Fibonacci sequece: ");
    gen_fibonacci(15);
    println!();
}

// 0, 1, 1, 2, 3, 5, 8, 13, 21
fn gen_fibonacci(steps: u8) {
    let mut attemps = 0;
    let mut first_number = 0;
    let mut second_number = 1;
    let mut temporal_number: i32;

    loop {
        attemps = attemps + 1;
        print!("{}, ", first_number);
        temporal_number = first_number + second_number;
        first_number = second_number;
        second_number = temporal_number;

        if attemps == steps {
            break;
        }
    }
}
