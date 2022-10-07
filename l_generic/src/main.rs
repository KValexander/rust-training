/* Mod */
pub mod lib;

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

}
