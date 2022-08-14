// Normal tuple
struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}

// Tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs
struc AlwaysEqual;

fn main() {

    // The order to assign the values does not matter
    let mut user1 = User {
        email: String::from("test@example.com"),
        username: String::from("Cayuya"),
        active: true,
        sing_in_count: 1,
    };

    user1.email = String::from("another@example.com");

    let mut user2 = User {
        active: user1.active.clone(),
        username: String::from("Francisco"),
        email: String::from("example@example.com"),
        sing_in_count: user1.sing_in_count,
    };

    // ..user1 takes ownership of the rest of the values of user1,
    // so user1 can't be used after created user3
    /*
    let mut user3 = User {
        email: String::from("user3@example.com"),
        ..user1
    }; */

    println!("User 1: {}, User 2: {}", user1.username, user2.username);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    /*
    User {
        email: email,
        username: username,
        active: true,
        sing_in_count: 1,
    } */

    // It can be written like this
    User {
        email,
        username,
        active: true,
        sing_in_count: 1,
    }
}