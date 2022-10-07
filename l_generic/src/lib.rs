/* Trait Summary, Interface */
pub trait Summary {
	/* Summarize author */
	fn summarize_author(&self) -> String;

	/* Summarize */
	fn summarize(&self) -> String {
		format!("(Read more from {}...)", self.summarize_author()) // Default
	}
}

/* Struct NewsArticle */
pub struct NewsArticle {
	pub headline: String, 
	pub location: String, 
	pub author: String, 
	pub content: String, 
}

/* Implement interface Summary to struct NewsArticle */
impl Summary for NewsArticle {
	fn summarize_author(&self) -> String {
		format!("@{}", self.author)
	}
	// fn summarize(&self) -> String {
	// 	format!("{}, by {} ({})", self.headline, self.author, self.location)
	// }
}

/* Struct Tweet */
pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool,
}

/* Implement interface Summary to struct Tweet */
impl Summary for Tweet {
	fn summarize_author(&self) -> String {
		format!("@{}", self.username)
	}
	fn summarize(&self) -> String {
		format!("{}: {}", self.username, self.content)
	}
}

/* Notify */
// pub fn notify(item: &impl Summary) {
// pub fn notify<T: Summary>(item1: &T, item2: &T) {
pub fn notify<T: Summary>(item: &T) {
	println!("Breaking news! {}", item.summarize());
}

/* Return summarizable */
// fn return_summarizable(switch: bool) -> impl Summary {
// 	if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                 hockey team in the NHL.",
//             ),
//         }
// 	} else {
// 		Tweet {
// 	        username: String::from("horse_ebooks"),
// 	        content: String::from(
// 	            "of course, as you probably already know, people",
// 	        ),
// 	        reply: false,
// 	        retweet: false,
// 		}
// 	}
// }