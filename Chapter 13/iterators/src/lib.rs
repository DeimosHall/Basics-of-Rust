#[test]
fn iterator_demonstration() {
    let v1 = [1, 2, 3];
    let mut v1_iter = v1.iter();

    // iter() returns immutable references of the items.
    // into_iter() creates an iterator that takes ownership
    // of the variable.
    // iter_mut() itarates over mutable references.
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    // When the itarator do not find an item,
    // it returns a None value.
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // There are method like sum() that
    // takes ownership of the iterator.
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}