fn main() {
    let _u8: u8 = 120;
    println!("| --- Integer Types --- |");
    println!("u8 size = {} byte", size_of(_u8));
    let _u16: u16 = 1200;
    println!("u16 size = {} bytes", size_of(_u16));
    let _u32: u32 = 12000;
    println!("u32 size = {} bytes", size_of(_u32));
    let _u64: u64 = 120000;
    println!("u64 size = {} bytes", size_of(_u64));
    let _u128: u128 = 1200000;
    println!("u128 size = {} bytes", size_of(_u128));
    let _usize: usize = 12000000;
    println!("usize size = {} bytes", size_of(_usize));
    println!("");

    let _i8: i8 = -120;
    println!("i8 size = {} byte", size_of(_i8));
    let _i16: i16 = -1200;
    println!("i16 size = {} bytes", size_of(_i16));
    let _i32: i32 = -12000;
    println!("i32 size = {} bytes", size_of(_i32));
    let _i64: i64 = -120000;
    println!("i64 size = {} bytes", size_of(_i64));
    let _i128: i128 = -1200000;
    println!("i128 size = {} bytes", size_of(_i128));
    let _isize: isize = -12000000;
    println!("isize size = {} bytes", size_of(_isize));
    println!("");

    let decimal_number: u32 = 98_222;
    println!("98_222 = {}", decimal_number);
    let hex_number: u32 = 0xff;
    println!("0xff = {}", hex_number);
    let octal_number: u32 = 0o77;
    println!("0o77 = {}", octal_number);
    let binary_number: u32 = 0b1111_0000;
    println!("0b1111_0000 = {}", binary_number);
    let byte_number: u8 = b'A';
    println!("b'A' = {}", byte_number);
    println!("");

    println!("| --- Floating-Point Types --- |");
    let _f32: f32 = 3.0;
    println!("f32 size = {} bytes", size_of(_f32));
    let _f64: f64 = 3.0;
    println!("f64 size = {} bytes", size_of(_f64));
    println!("");

    println!("| --- Boolean Type --- |");
    let _t: bool = true;
    println!("bool size = {} byte", size_of(_t));
    let is_rust_fun = true;
    println!("is_rust_fun = {}", is_rust_fun);
    let is_javascript_fun: bool = false;
    println!("is_javascript_fun: bool = {}", is_javascript_fun);
    println!("");

    println!("| --- Character Type --- |");
    let _c: char = 'z';
    println!("char size = {} bytes", size_of(_c));
    let c = 'z';
    println!("c = {}", c);
    let z: char = 'ℤ';
    println!("z: char = {}", z);
    let heart_eyed_cat: char = '😻';
    println!("heart_eyed_cat: char = {}", heart_eyed_cat);
    println!("");

    println!("| --- Compound Types --- |");
    println!("| --- Tuple Type --- |");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup: (i32, f64, u8) = {:?}", tup);
    let (_x, y, _z) = tup;
    println!("y = {}", y);
    println!("tup.0 = {}", tup.0);
    println!("");

    println!("| --- Array Type --- |");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a: [i32; 5] = {:?}", a);
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("months = {:?}", months);
    println!("First month[0] = {}", months[0]);
    println!("Last month[12] = {}", months[months.len() - 1]);
    print!("months[13] = ");
    match months.get(13) {
        Some(month) => println!("{}", month),
        None => println!("None"),
    }
    println!("");
}

// Get the size of a type in bytes
fn size_of<T>(_: T) -> usize {
    std::mem::size_of::<T>()
}
