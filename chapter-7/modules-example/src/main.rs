use crate::garden::vegetables::Carrot;

pub mod garden;

fn main() {
    let vegetable = Carrot {};
    println!("I'm growing {vegetable:?}");
}
