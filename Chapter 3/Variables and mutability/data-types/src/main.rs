fn main() {
    // Rust has four primary scalar types: integers, floats, Booleans and characters.
    let decimal: u32 = 98_222; // 98222
    let hex: u32 = 0xff;
    let octal: u32 = 0o77;
    let binary: u8 = 0b1111_0000;
    let byte: u8 = b'B';

    println!("Decimal: {decimal}");
    println!("Hexadecimal: {hex}");
    println!("Octal: {octal}");
    println!("Binary: {binary}");
    println!("Byte: {byte}");

    // Float
    let x = 2.01; // f64 by default
    let y: f32 = 3.2; // f32 only if it's especified
    println!("Float");
    println!("x = {x}");
    println!("y = {y}");

    // Boolean
    let t = true;
    let f: bool = false;
    println!("Boolean");
    println!("t = {t}, f = {f}");

    // Char
    let c = 'z';
    let z: char = 'Z';
    let emoji = '😎';
    println!("Char");
    println!("c = {c}, z = {z}, emoji = {emoji}");

    // Tuple
    // Tuples can't grow or shrink in size once created
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup; // Destructuring a tuple
    let last_value = tup.2;
    println!("Tuple");
    println!("The value of y is {y}");
    println!("The value of z is {last_value}");


    // Array
    // An array has a fixed size, but vectors can grow and shrink
    let _a = [1, 2, 3, 4, 5];
    // In the type of the array, first is the type of each element
    // and then the number of elements
    let _b: [i32; 5] = [1, 2, 3, 4, 5];
    let value = 3;
    const SIZE: usize = 5;
    let new_array = [value; SIZE];
    println!("New array = [{}, {}, {}, {}, {}]", new_array[0], new_array[1], new_array[2], new_array[3],
            new_array[4]);
}
