use std::fmt::{ Display, Debug };

pub trait Summary {
    // The behavior will be defined by the structs that calls this trait
    //fn summarize(&self) -> String;

    // summarize with default behavior
    /*
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    } */

    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.headline)
    }

    /*
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    } */
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // This overrides the default implementation
    /*
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    } */
}

// Returning types that implements traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("hourse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        retweet: false,
    }
}

// Traits as parameters
// item can be any type that implements the Summary trait
/*
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
} */

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// We can specify two types of bounds like this
// pub fn notify(item: &(impl Summary + Display))
// or like this
// pub fn notify<T: Summary + Display>(item: &T)

// When a function has a lot of bounds, instead of writing this:
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U)
// we can write this:
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug {
    5
}

// The impl block in the standard library looks similar to this code:
/*
impl<T: Display> ToString for T {
    // --snip--
} */
// Because the standard library has this blanket implementation, we can call the to_string
// method defined by the ToString trait on any type that implements the Display trait. For
// example, we can turn integers into their corresponding String values like this because
// integers implement Display:
// let s = 3.to_string();