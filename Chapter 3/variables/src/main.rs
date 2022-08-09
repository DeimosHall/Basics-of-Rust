// Variables can't be declared in the global scope
// let variable = 15;
const WEIGHT: u32 = 52;

fn main() {
    // Variables are inmutable by default
    let inmutable_x = 5;
    let mut mutable_x = 10;
    println!("Inmutable x = {inmutable_x}");
    println!("Mutable x = {mutable_x}");
    mutable_x = 12;
    println!("Mutable x = {mutable_x}");
    println!("Contant weight = {WEIGHT}");
    println!("");

    // Shadowing
    let x = 5;
    let x = x + 1; // This is a new variable

    {
        let x = x * 2; // This is a new variable that will be destroyed after the block
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // The type can be changed with shadowing
    let x = 3;
    println!("x = {x}");
    let x = "three";
    println!("x = {x}");
}
