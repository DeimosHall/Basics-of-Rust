fn main() {
    let v1 = vec![1, 2, 3];
    // v1_iter only stores the iterator
    let v1_iter = v1.iter();

    for item in v1_iter {
        println!("{}", item);
    }
}
