use std::io;

fn main() {
    println!("Fibonacci succession generator");

    loop {
        println!("How many characters do you want to generate?");
        let mut size = String::new();
        io::stdin()
            .read_line(&mut size)
            .expect("Unable to read the size");

        let size: u64 = match size.trim().parse() {
            Ok(size) => size,
            Err(_) => {
                println!("Invalid value, try again");
                continue;
            }
        };
        generate_fibonacci(size);
        break;
    }
}

fn generate_fibonacci(mut size: u64) {
    let mut x: u64;
    let mut y: u64 = 1;
    let mut z: u64 = 1;

    while size > 0 {
        size -= 1;

        x = y;
        y = z;
        z = x + y;

        if size == 0 {
            print!("{x}");
        } else {
            print!("{x},");
        }
    }
    println!("");
}