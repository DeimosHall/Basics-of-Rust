#[allow(dead_code)]
#[warn(unused_variables)]
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1), 
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six: {:?}, none: {:?}", six, none);

    // The _ placeholder catch the rest of the posibilities
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // The empty tuple -> () allows us to get nothing to do in the code
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}