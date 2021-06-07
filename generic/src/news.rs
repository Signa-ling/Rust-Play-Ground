pub trait Summary {
    fn summarize_author(&self) -> String;

    // Default behavior.
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
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
        // Behaves differently from "impl Summary for Tweet {}".
        format!("{}", self.author)
    }
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
}

// Take as argument some type that implements Summary trate.
// Then call the summarize method.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Syntax Sugar for notify()
/*
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
*/