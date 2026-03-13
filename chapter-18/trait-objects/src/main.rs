// Rust doesn't have inheritance, so if we want to create a gui
// library, we can use trait objects to emulate the same behavior.

// Our trait to implement
pub trait Draw {
    fn draw(&self);
}

// Our screen that tracks the components on it.
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// A method to simulate drawing the components.
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Note: We can't use generics in the case, because it would result
// in a Screen that can only holds components of one type, for example
// it could only holds Buttons, or only TextFields.
//
// The implementation with a trait uses dynamic distpach, so we can store
// components of multiple types, such as Box<Button>, or Box<Label>.

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button");
    }
}

pub struct TextField {
    pub text: String,
}

impl Draw for TextField {
    fn draw(&self) {
        println!("Drawing a text field");
    }
}

fn main() {
    let button = Button {
        width: 128,
        height: 64,
        label: String::from("Press"),
    };
    
    let text_field = TextField {
        text: String::from("Hello"),
    };

    let screen = Screen {
        components: vec![Box::from(button), Box::from(text_field)]
    };

    screen.run();
}
