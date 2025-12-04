pub fn find_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// We can also define generics structs
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub mod Another {
    pub struct Point<T, U> {
        pub x: T,
        pub y: U,
    }
    
    impl<T, U> Point<T, U> {
        pub fn x(&self) -> &T {
            &self.x
        }
    }
    
    impl Point<f32, f32> {
        pub fn hello(&self) -> String {
            "hello".to_string()
        }
    }
}