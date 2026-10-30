
// CODE IN lib.rs

use std::fmt::Display;

use b_traits::{NewsArticle, SocialPost, Summary};

fn main() {

    // Definition
    // Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose
    // *******

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };
    
    println!("1 new post: {}", post.summarize());
    
    // using default implementations
    // uncomment lib.rs
    
    let article = NewsArticle {
        headline: String::from("Penguins win the stanley cup champioship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    

    // using traits as parameters
    // *******
    
    // read as: "accept T for some type T that implements Summary"
//     fn notify(item: &impl Summary) {
//         println!("Breaking news! {}", item.summarize());
//     }
// 
//     notify(&post);
    
    // trait bound syntax
    // *******

    // the `impl Trait` syntax works for straighforwad cases but is syntax sugar for a longer form: `trait bound`

    // fn notify<T: Summary>(item: &T) {
    //     println!("Breaking news! {}", item.summarize());
    // }
    // 
    // notify(&post);

    // `impl Trait` is convenient and makes the code more concise

    // multiple trait bounds with the `+` syntax
    fn notify(item: &(impl Summary + Display)) {
        println!("Breaking news! {}", item.summarize());
    }
    
    notify(&post);
    
}
