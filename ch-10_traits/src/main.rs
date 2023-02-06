pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summerize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    
}
pub struct NewsArticle {
    pub auther: String,
    pub headline: String,
    pub content: String,
}
impl Summary for NewsArticle {
    // fn summerize(&self) -> String {
    //     format!("{}, by {}", self.headline, self.auther)
    // }
    fn summarize_author(&self) -> String {
        format!("{}", self.auther)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    // Default Implementation. If no implementation is not there in struct, then this will consider.
    fn summerize(&self) -> String {
        format!("Default Impl: Read More!!!! {}", self.summarize_author())
    }
    
}
// Traits as Parameter
// pub fn notify(item: &impl Summary) {
//     println!("Breaking News: {}", item.summerize());
// }
// Alternate way
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking News: {}", item.summerize());
}
// 
/*
    fn some_func<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

    }
    fn some_func<T, U>(t: &T, u: &U) -> i32
        where T: Display + Clone,
              U: Clone + Debug {
    }

*/
// Traits as Return Type
fn returns_summerizable() -> impl Summary {
    Tweet {
        username: String::from("ReturnUser"),
        content: String::from("Retrurn Contect"),
        reply: false,
        retweet: false,
    }
}
fn basic_trait_check() {
    let tweet = Tweet {
        username: String::from("Hrudaya"),
        content: String::from("Hello World!!!"),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle {
        auther: String::from("Ranjan"),
        headline: String::from("The sky is falling!!!"),
        content: String::from("Sky is actually not falling!!!!"), 
    };
    println!("Tweet Summary: {}", tweet.summerize());
    println!("NewArticle Summary: {}", article.summerize());
    notify(&article);
}
fn main() {
    basic_trait_check();
    // Using trait as return type
    println!("{}", returns_summerizable().summerize());
}