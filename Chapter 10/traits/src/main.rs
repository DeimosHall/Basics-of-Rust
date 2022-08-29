use traits::{Summary, Tweet, NewsArticle};

pub mod lib;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as probably you already know people"
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburg"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL."
        ),
    };

    println!("New article available: {}", article.summarize());
}
