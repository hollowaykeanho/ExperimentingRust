// Copyright 2022 "Holloway" Chew, Kean Ho <hollowaykeanho@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

// declare a trait (list of methods compliance)
pub trait Summary {
	fn summarize(&self) -> String;
}

// use trait as parameter
pub fn notify(channel: &impl Summary) {
	println!("Breaking News: {}", channel.summarize());
}

// use trait bound for generics
pub fn blast<X: Summary, Y: Summary>(ch1: &X, ch2: &Y) {
	println!("Announcement: {} AND {}", ch1.summarize(), ch2.summarize());
}

// alternative trade bound (for clarity)
pub fn ultra_blast<X, Y>(ch1: &X, ch2: &Y)
where
	X: Summary,
	Y: Summary,
{
	println!("Blasting: {} AND {}", ch1.summarize(), ch2.summarize());
}

// return trait pattern value
pub fn new_summarizable_tweet(username: String, tweet: String) -> impl Summary {
	return Tweet {
		username: username,
		content: tweet,
		reply: false,
		retweet: false,
	};
}

pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String,
}

// implement methods complying to Summary trait
impl Summary for NewsArticle {
	fn summarize(&self) -> String {
		return format!("{}, by {} ({})", self.headline, self.author, self.location);
	}
}

pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool,
}

// implement methods complying to Summary trait
impl Summary for Tweet {
	fn summarize(&self) -> String {
		return format!("{}: {}", self.username, self.content);
	}
}

fn main() {
	let tweet = new_summarizable_tweet(
		String::from("chant"),
		String::from("Unexplored area, forbidden wall."),
	);
	let article = NewsArticle {
		headline: String::from("Penguins win the Stanley Cup Championship!"),
		location: String::from("Pittsburgh, PA, USA"),
		author: String::from("Iceburgh"),
		content: String::from(
			"The Pittsburgh Penguins once again are the best \
			hockey team in the NHL.",
		),
	};

	println!("Here is a tweet: {}", tweet.summarize());
	println!("Here is a news: {}", article.summarize());
	notify(&tweet);
	notify(&article);
	blast(&tweet, &article);
	ultra_blast(&tweet, &article);
}
