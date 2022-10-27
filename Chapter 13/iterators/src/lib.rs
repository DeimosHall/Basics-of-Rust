//! This is the documentation for the entire lib file.
//! The syntaxis is //!

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}
// Only public items can be documented to be viewed in HTML
/// A hello world
/// # Example
/// ```
/// say_hello();
/// ```
pub fn say_hello() {
    println!("Hello world :D");
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter takes ownership of the vector
    shoes.into_iter().filter(|shoe| shoe.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneacker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneacker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }
}
