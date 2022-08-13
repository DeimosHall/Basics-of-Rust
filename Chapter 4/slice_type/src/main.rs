fn main() {
    // String slices
    let s = String::from("hello world");

    // [0..5] is equal to [..5]
    let hello = &s[0..5];
    // [6..11] is equal to [6..]
    let world = &s[6..11];

    println!("{hello} {world}");
    let word = get_first_word(&s);
    //s.clear();
    println!("{word}");

    // Slices in arrays
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    println!("{:?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}