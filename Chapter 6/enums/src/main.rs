#[derive(Debug)]
enum IpAdress {
    // Only one value can be instaciated
    //V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Color {
    Rgb(i32, i32, i32),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

impl Message {
    fn call(&self) {
        println!("Message: {:?}", self);
    }
}

// <T> means it can be any type of data
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    //let home_ip = IpAdress::V4(String::from("127.0.0.1"));
    let home_ip = IpAdress::V4(127, 0, 0, 1);
    let loopback = IpAdress::V6(String::from("::1"));

    dbg!(&home_ip);
    dbg!(&loopback);

    let msg = Message::Write(String::from("Hello"));
    msg.call();

    // Its type is Option<i32>
    let some_number = Some(5);
    // Its type is Option<&str>
    let some_string = Some("A string");
    // For None Rust can't infer the type
    let absent_number: Option<i32> = None;
}
