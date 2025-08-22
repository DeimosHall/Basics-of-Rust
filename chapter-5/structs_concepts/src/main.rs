use core::fmt;

struct User {
    name: String,
    age: u8,
    authenticated: bool,
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("name", &self.name)
            .field("age", &self.age)
            .field("authenticated", &self.authenticated)
            .finish()
    }
}

// Tuple struct
struct Color(i32, i32, i32);

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Color")
            .field(&self.0)
            .field(&self.1)
            .field(&self.2)
            .finish()
    }
}

// Unit-Like struct
struct AlwasyEqual;

fn main() {
    let user = User {
        name: String::from("Cayuya"),
        age: 27,
        authenticated: true,
    };

    println!("User name: {}", user.name);
    println!("{:#?}", user);
    // user.age = 26;  // We can't change values because user is not mutable

    // Mutable user struct
    let mut user = build_user("Deimos".to_string(), 24);
    println!("{:#?}", user);
    println!("Authenticating user...");
    user.authenticated = true;
    println!("{:#?}", user);

    // Create a new user based on a previous one
    let user2 = User {
        name: user.name,
        age: user.age,
        authenticated: false,
    };
    println!("user2: {:#?}", user2);

    // Create a new user based on a previous one with less code
    let user3 = User {
        name: String::from("Name"),
        ..user // Copy the rest of the fields
    };
    println!("user3: {:#?}", user3);

    let white = Color(255, 255, 255);
    println!("white: {:#?}", white);

    let subject = AlwasyEqual;
}

fn build_user(name: String, age: u8) -> User {
    User {
        name,
        age,
        authenticated: false,
    }
}
