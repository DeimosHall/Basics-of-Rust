// This is needed to use the dbg! macro
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // This is a method
    // &self means it doesn't take ownership of the Rectangle instance
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }

    // Funtions that don't depend on self aren't methods, they're called constructors
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
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

    println!("{:#?}", rect1);
    println!("The area is {}", area(&rect1));
    println!("The area is {}", rect1.area());
    println!("Width is {}", rect1.width());
    // rect1.width should be private
    println!("Width is {}", rect1.width);
    println!("dbg! ->");
    dbg!(&rect1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(3);
    println!("");
    dbg!(&square);
}

fn area(rectanle: &Rectangle) -> u32 {
    rectanle.width * rectanle.height
}