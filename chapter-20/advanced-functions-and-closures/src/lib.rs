// We can pass functions as arguments to other functions, just as we
// can do it with closures

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// Here we pass add_one as an argument
fn function_as_arg() {
    let answer = do_twice(add_one, 5);

    // Answer is 12
    println!("The answer is: {answer}");
}
