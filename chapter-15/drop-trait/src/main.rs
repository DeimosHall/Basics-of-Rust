struct CustomSmartPointer {
    data: String,
}

// We can customize the code of the "destructor" by
// implementing the drop trait
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Finishing {}!", self.data);
    }
}

fn main() {
    let _c = CustomSmartPointer {
        data: String::from("connection")
    };
    
    let _d = CustomSmartPointer {
        data: String::from("web socket")
    };
    
    println!("Custom smart pointers created");
}
