
// the compiler will enforce that any type that has the `Summary` trait will have the method `summarize` defined with this signature exactly
// pub trait Summary {
//     fn summarize(&self) -> String;
// }

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}



// implementing a `Trait` on a Type

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }
impl Summary for NewsArticle { }

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
