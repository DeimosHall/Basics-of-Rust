use std::fmt::Display;

// Lifetimes of variables are implicit most of the time.
// They are the reason why we can't use the reference of x
// after it goes out of scope, it's because the lifetime of
// x ('b) is shorter than the lifetime of r ('a)
fn invalid_lifetimes() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    // println!("r: {r}");//          |
}                         // ---------+

// In this function, rust doesn't know the lifetime of the return
// value, it could be the lifetime of x, or maybe the one of y, but
// even we don't know it because we don't know the answer of the
// comparision in first place.
// 
// The compiler shows this error:
// this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
// help: consider introducing a named lifetime parameter
//   |
// 16 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//   |           ++++     ++          ++          ++
// 
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

// As we see in the error message, the compiler suggest to add a lifetime
// paramter to indicate the lifetime of the return value will be as long
// as than the smaller one from any of the parameters
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// Elision rules
// The reason why the `first_word` function works without lifetime annotations
// is because of the Elision rules.
// 
// Rule 1
// The compiler assign a lifetime to every input argument, so we don't have to
// specify them.
// Rule 2
// If there's only one input lifetime parameter, that lifetime will be for the output
// Rule 3
// If one of the input lifetimes are &self (e.g. methods), that lifetime will be for the output

// In fact, annotating static is not needed here
static APP_VERSION: &'static str = "0.1.0";

fn main() {
    println!("Lifetimes!");
    
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    println!("The longest string is {}", longest(string1.as_str(), string2));
    
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // This won't work because the lifetime of the result is the same
    // as the smaller one of the parameters, in this case string2 is
    // in an inner scope, so it isn't valid here
    // println!("The longest string is {result}");
    
    // Structs also require lifetime annotation when holding reference values
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    
    // Lifetime annotations are required here because they are part of the
    // struct's type
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Part: {}", i.part);
    
    println!("First word: {}", first_word("first word"));
    
    // The static lifetime means it lasts for the entire life of the program
    // Useful for string configurations, check APP_VERSION
    let SECONDS_IN_A_MINUTE: &'static str = "60";
    println!("Seconds: {}", SECONDS_IN_A_MINUTE);
    println!("App version: {}", APP_VERSION);
    
    println!("{} ", longest_with_an_announcement("hola", "hello", "Lifetimes!"))
}

// To finish, a function with a generic argument, trait bounds and lifetimes
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
