#[derive(Debug)]
#[allow(dead_code)]
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn x(&self) -> &X1 {
        &self.x
    }
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// This method will work only for f32 types
#[allow(dead_code)]
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // Code for generic struct
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 5.5, y: 10.1 };
    let mixed = Point { x: 15.2, y: 7 };
    println!("integer: {:?}, float: {:?}, mixed: {:?}", integer, float, mixed);

    // Generic implementation
    println!("mixed.x: {}", mixed.x());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    println!("p1.x: {}, p1.y: {}", p1.x, p1.y);
    println!("p2.x: {}, p2.y: {}", p2.x, p2.y);
    
    let p3 = p1.mixup(p2);
    println!("p3.x: {}, p3.y: {}", p3.x, p3.y);
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Generic function for largest
/*
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
} */