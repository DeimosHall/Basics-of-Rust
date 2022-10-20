fn main() {
    let v1 = vec![1, 2, 3];
    // v1_iter only stores the iterator
    let v1_iter = v1.iter();

    for item in v1_iter {
        println!("{}", item);
    }

    // map is an iterator that produces other itarator
    // it also needs to be consume
    v1.iter().map(|x| x + 1);

    // We can conume the iterator with the method collect,
    // that returns the values into a collection data type
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("{:#?}", v2);
}
