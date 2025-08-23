#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width >= rectangle.width && self.height >= rectangle.height
    }
    // Constructor
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
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
    println!("The are of the rectangle is {} square pixels.",
        rectangle.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(8);
    println!("square created: {:#?}", square);
}

fn area(rectangle: &Rectangle) -> u32 { 
    rectangle.width * rectangle.height
}