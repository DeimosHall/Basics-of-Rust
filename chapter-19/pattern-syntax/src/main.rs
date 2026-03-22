fn main() {
    // Matching literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => println!("anything"),
    }

    // Matching named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }

    // The previous match is a little confusing,
    // I may think it should match the "_" case,
    // but what's happening is more clear in this version:
    match x {
        Some(50) => println!("Got 50"),
        Some(value) => println!("Matched, value = {value}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?} and y = {y}");

    // Matching multiple patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"), // This is printed
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching ranges of values with ..=
    let x = 5;
    match x {
        0..=5 => println!("from zero to five"),
        _ => println!("anything"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("Here is the letter"),
        'k'..='z' => println!("Here is not the letter"),
        _ => println!("anything"),
    }

    // Destructuring to break apart values
    // Structs
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 5, y: 10 };
    // Destructuring into x and y variables
    let Point { x, y } = point;
    assert_eq!(5, x);
    assert_eq!(10, y);

    match point {
        Point { x, y: 11 } => println!("y is not 11!"),
        Point { x: 5, y } => println!("matched!, x is 5"),
        Point { x, y } => print!("matches any other point"), // But never goes here
    }

    // Enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 255, 0);

    match msg {
        Message::Quit => println!("No Quit data to destructure"),
        Message::Move { x, y } => println!("Move in the x direction {x} and the y direction {y}"),
        Message::Write(text) => println!("Text: {text}"),
        Message::ChangeColor(r, g, b) => println!("Matched!, rgb: {r},{g},{b}"),
    }

    // Nested structs and enums
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message2::ChangeColor(Color::Hsv(0, 255, 0));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("rgb: {r},{g},{b}, but doesn't match!")
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => println!("hsv: {h},{s},{v}, matches!"),
        _ => println!("anything else"),
    }

    // Ignoring parts of a value can be done with `_`, but also with `..`
    struct Point2 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point2 { x: 0, y: 0, z: 0 };

    match origin {
        Point2 { x, .. } => println!("x is {x}"),
    }

    // Adding conditional with match guards
    // Only available on match
    let num = Some(4);

    match num {
        Some(value) if value % 2 == 0 => println!("The number {value} is even"),
        Some(value) => println!("The number {value} is odd"),
        None => (),
    }

    // Using @ Bindings
    enum Message3 {
        Hello { id: i32 },
    }

    let msg = Message3::Hello { id: 5 };

    match msg {
        // Using @ let us test the value and crete a temp variable to use it in the println
        Message3::Hello { id: id @ 3..7 } => println!("This works! id: {}", id),
        // No binding here because we are not printint the value
        Message3::Hello { id: 10..20 } => println!("Not here!"),
        // No binding here because we are not comparing the value
        Message3::Hello { id } => println!("Any other value: {}", id),
    }
}
