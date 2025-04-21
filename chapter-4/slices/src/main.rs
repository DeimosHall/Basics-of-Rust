fn main() {
    let text = String::from("Hola Mundo");
    println!("text: {text}");
    let word = first_word(&text);
    println!("first word: {word}");

    // Slices of arrays
    let array = [1, 2, 3, 4];
    println!("array: {:?}", array);
    let slice = &array[1..4];
    println!("slice: {:?}", slice);
}

fn first_word(s: &String) -> &str {
    let bytes: &[u8] = s.as_bytes();
    println!("bytes: {:?}", bytes);
    for (index, &item) in bytes.iter().enumerate() {
        // println!("{index}: {item}");
        if item == b' ' {
            return &s[0..index];
        }
    }
    // The entire word
    &s[..]
}
