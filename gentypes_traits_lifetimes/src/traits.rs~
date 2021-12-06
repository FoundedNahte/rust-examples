use std::fmt::Display;

pub trait Summary { // pub allows the trait to be applied outside
    fn summarize(&self) -> String {
        String::from("(Read more...)") // default string
    }

    fn summarize_author(&self) -> String;

}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn sumamrize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {// function only accepts types that have implemented Summary trait
    println!("Breaking news! {}", item.summarize()); 
}
//above same as below
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify(item!: &impl Summary, item2: f32) {}

pub fn notify<T: Summary>(item1: &T, item2: &T) {}// trait bound required if parameters need to be same trait

pub fn notify(item: &(impl Summary + Display)) {} // parameter type must have implemented summary and display traits
//above same as below
pub fn notify<T: Summary + Display>(item: &T) {}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// instead of writing above, write below
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}

fn returns_summarizable() -> impl Summary { // returns some type that implements the summary trait
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// using trait bounds to conditionally implement methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Slef {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// blanket implementations; conditionally implement a trait for any type that implements another
// trait
impl<T: Display> ToString for T {

}
