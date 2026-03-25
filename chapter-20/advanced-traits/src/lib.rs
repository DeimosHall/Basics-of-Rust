use std::ops::Add;

// traits can have associated type, which are like placeholders
// the implementors must meet. As an example, the Iterator trait
// from the standard library:
pub trait Iterator2 {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// Let's look at how to implement Iterator2 on a Counter type
struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator2 for Counter {
    type Item = u32;

    // We use Item as the return type, it's a like a generic
    // return type
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// Using default generic parameters and operator overloading

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// This is the `+` operator
impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


