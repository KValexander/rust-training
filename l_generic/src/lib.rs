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