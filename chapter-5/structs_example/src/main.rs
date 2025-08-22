#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle = Rectangle {
        width: 3,
        height: 5,
    };

    // Print structs
    println!("Rectangle: {rectangle:?}");
    // Print pretty structs
    println!("Rectangle: {rectangle:#?}");
    // Using dbg
    dbg!(&rectangle);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rectangle)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}