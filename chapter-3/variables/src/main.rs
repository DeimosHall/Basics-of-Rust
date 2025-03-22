fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // error[E0384]: cannot assign twice to immutable variable `x`
    println!("The value of x is: {}", x);

    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    // Constants
    const DAY_HOURS: u32 = 24;
    println!("The value of DAY_HOURS is: {}", DAY_HOURS);

    // Shadowing
    let z = 5;
    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of z is: {}", z); // 12
    }
    println!("The value of z is: {}", z); // 6

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces); // 3
}
