pub fn find_largest_number(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    
    largest
}

pub fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}