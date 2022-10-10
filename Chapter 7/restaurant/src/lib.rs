// Modules can also hold definitions for other items, such as structs,
// enums, constants, traits or functions
#[allow(dead_code)]
mod front_of_house;

// Creating a shortchut with 'use'
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Path with shortcut
    hosting::add_to_waitlist();
}

#[allow(dead_code)]
mod customer {
    pub fn eat_at_restaurant() {
        // super access to a top level in the tree of filesystem
        super::hosting::add_to_waitlist();
    }
}