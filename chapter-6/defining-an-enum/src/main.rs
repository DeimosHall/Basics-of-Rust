#[derive(Debug)]
enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: u32, y: u32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Calling message");
    }
}

fn main() {
    let ipv4 = IpAddressKind::V4(127, 0, 0, 1);
    let loopback = IpAddressKind::V6("::1".to_string());

    println!("ipv4: {:?}", ipv4);
    println!("loopback: {:?}", loopback);

    let message: Message = Message::Write("Hello".to_string());
    message.call();

    // Option
    let five = Some(5);
    let hello = Some("Hello");
    // We have to specify the type when it's None
    let not_a_number: Option<i32> = None;
}
