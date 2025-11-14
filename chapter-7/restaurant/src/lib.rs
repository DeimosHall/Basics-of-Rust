// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_wait_list() {}
//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}
//         fn serve_order() {}
//         fn take_payment() {}
//     }
// }

// Separating the front_of_house module to another file

mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    // We have to define one by one the public fields
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // private
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // But in enums all fields al public if the enum is public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        self::cook_order();
        // Use functions from the parent
        super::deliver_order();
    }

    fn cook_order() {}
}

pub use crate::front_of_house::hosting;

pub fn eat_at_retaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_wait_list();

    // Relative path
    front_of_house::hosting::add_to_wait_list();

    // Using shortcut
    hosting::add_to_wait_list();

    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");

    // If enum is public, fields are public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}
