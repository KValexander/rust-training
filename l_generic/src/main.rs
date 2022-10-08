/* Mod */
pub mod lib;

/* Use std */
use std::fmt::Display;

/* Use */
use crate::lib::{Summary, NewsArticle, Tweet};

/* Struct */
struct Point<X1, Y1> {
    x: X1,
    y: Y1
}

/* Impl */
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other : Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

/* Lifetimes */
/* Struct */
struct ImportantExcerpt<'a> {
    part: &'a str,
}

/* Impl */
impl<'a> ImportantExcerpt<'a> {
    fn announce(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}


/* Entry point */
fn main() {

    /* Mixup */
    let p1 = Point{ x: 5, y: 10.4 };
    let p2 = Point{ x: "Hello", y: " world" };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    /* Tweet */
    let tweet = Tweet {
        username: String::from("book"),
        content: String::from("This is content"),
        reply: false,
        retweet: false,
    };

    /* Summary tweet */
    println!("1 new tweet - {}", tweet.summarize());

    /* News article */
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        )
    };

    /* Summary article */
    println!("New article available! {}", article.summarize());

    /* Lifetimes */
    let r;
    {
        let x = 5;
        r = &x; // error
    }

    let x = 5;
    let r = &x; // success

    println!("r: {}", r);

    /* Lifetimes 2 */
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // &i32 // a reference
    // &'a i32 // a reference with and explict lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

    /* Lifetimes 3 */
    let string1 = String::from("Long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result); // success
    }
    // println!("The longest string is {}", result); // error

    /* Lifetimes 4 */
    let novel = String::from("Call me Vito Scoletti. Some year ago...");
    let first = novel.split(".").next().expect("Coild not find a '.'");
    let i = ImportantExcerpt {
        part: first
    };

    /* Static lifetime */
    let s: &'static str = "I have a static lifetime";

}

/* Longest string */
// fn longest(x: &str, y: &str) -> &str {
// fn longest<'a>(x: &'a str, y: &'a str) -> & str {
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

/* First word */
// fn first_word(s: &str) -> &str {
// fn first_word<'a>(s: &'a str) -> &str {
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}


/* Longest with an announcement */
fn longest_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str where T:Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {x} else {y}
}

