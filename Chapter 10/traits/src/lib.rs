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