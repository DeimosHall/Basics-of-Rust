use std::env;
// std::env::args_os can receive arguments with invalid Unicode

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
