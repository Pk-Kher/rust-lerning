use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("there is not summary here")
    }
    fn get_author(&self) -> String;
}
pub struct NewsArtical {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: Option<String>,
}

impl Summary for NewsArtical {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
    fn get_author(&self) -> String {
        format!("{}", self.author)
    }
}

impl NewsArtical {
    pub fn new(headline: String, location: String, author: String, content: Option<String>) -> NewsArtical {
        NewsArtical {
            headline,
            location,
            author,
            content,
        }
    }
}
pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: String,
    pub repost: String,
}
impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn get_author(&self) -> String {
        format!("{}", self.username)
    }
}
//traits as args in the function
pub fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}
pub fn notify_v2<T: Summary>(item: &T) {
    println!("notify v2:{}", item.summarize());
}
// you need to pass same type that implements Summary
pub fn notify_v3<T1: Summary>(item1: &T1, item2: &T1) {
    println!("notify v3:item1->{} item2->{}", item1.summarize(), item2.summarize());
}

pub fn notify_v4(item: &(impl Summary + Display)) {
    println!("notify v4 from display traits:{}", item);
    println!(
        "notify v4 from Summary traits->summarize{}, author:{}",
        item.summarize(),
        item.get_author()
    );
}

pub fn return_summary_social() -> impl Summary {
    SocialPost {
        username: String::from("name"),
        content: String::from("content"),
        reply: String::from("reply"),
        repost: String::from("repost"),
    }
}
