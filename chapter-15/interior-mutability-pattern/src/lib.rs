// The RefCell<T> smart pointer allow us to mutate internal
// values even if the data is inmutable. It internally uses
// an unsafe block.
//
// - Rc<T> enables multiple owners of the same data; Box<T>
//      and RefCell<T> have single owners.
// - Box<T> allows immutable or mutable borrows checked at compile time;
//      Rc<T> allows only immutable borrows checked at compile time;
//      RefCell<T> allows immutable or mutable borrows checked at runtime.
// - Because RefCell<T> allows mutable borrows checked at runtime, you can
//      mutate the value inside the RefCell<T> even when the RefCell<T> is
//      immutable.
//
// It can be useful when we work with mock objects, let's see an example where
// we have a `Messenger` that sends messages.
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

pub fn run() {

}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    // keep tracks on the sent messages.
    struct MockMessenger {
        // sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // sent_messages: vec![],
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // The compiler tell us to change the `self` to be mutable:
            // self.sent_messages.push(String::from(message));
            // |             ^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so it cannot be borrowed as mutable
            // But we don't want to do it just for testing purposes, instead,
            // we can use the RefCell<T> smart pointer.
            // self.sent_messages.push(String::from(message));
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // assert_eq!(mock_messenger.sent_messages.len(), 1);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
