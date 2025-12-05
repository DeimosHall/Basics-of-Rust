use std::fmt::Display;

use crate::aggregator::{NewsArticle, SocialPost, Summary};

mod aggregator;

// Generic pair
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// We implement this method only for the Pair that implements the
// Display and PartialOrd traits
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let post = SocialPost {
        username: "Cayuya".to_string(),
        content: "This is a post".to_string(),
        reply: false,
        repost: false,
    };
    
    println!("Summary: {}", post.summarize());
    
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
                hockey team in the NHL.",
        ),
    };
    
    println!("Summary: {}", article.summarize());
    
    let pair = Pair::new(3, 4);
    pair.cmp_display();
}
