fn main() {
    // Iterator are lazy, meaning they need to be consume to be
    // applied, for example:
    let vector = vec![1, 2, 3];
    let iterator = vector.iter();  // Does anything at this point
    
    // Use it to consume it
    for item in iterator {
        println!("item: {}", item);
    }
    
    // There are methods that consume the iterator, for example,
    // the sum method:
    let iterator2 = vector.iter();
    let total: u32 = iterator2.sum();  // iterator2 is not available after this
    println!("total: {}", total);
    
    // There are methods that produces new iterators, for example map
    vector.iter().map(|x| x + 1);
    println!("vector: {:?}", vector);  // prints [1, 2, 3], nothing has change yet
    // We need to consume the iterator, and to do so, we can use the collect method
    let new_vector: Vec<_> = vector.iter().map(|x| x + 1).collect();
    println!("new vector: {:?}", new_vector);  // prints [2, 3, 4]
}
