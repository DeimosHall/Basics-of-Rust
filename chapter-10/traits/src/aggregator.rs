use std::fmt::{Debug, Display};

// A trait defines shared behavior
pub trait Summary {
    // The method could define default behavior
    fn summarize(&self) -> String {
        format!("Read more...")
    }
}

// Traits as parameters
// Here we acept any data type that implements the Summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Another equivalent way is to use trait bound syntax
// It's the same, but more verbose
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// We can also force the parameter to implement more than one trait
// like this:
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    3
}

// We can also use traits as the return type:
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}

// Or better, like this:
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    3
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// We implement the behavior on each struct
impl Summary for NewsArticle {
    // If we don't define the havior here, we will use the default one
    // fn summarize(&self) -> String {
    //     format!("{}, {} by ({})", self.headline, self.author, self.location)
    // }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}