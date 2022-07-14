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

// assign lifetime (e.g 'a) in function
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		return x;
	}

	return y;
}

// assign lifetime (e.g. 'a) in generic function
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
	T: std::fmt::Display,
{
	println!("Announcement! {}", ann);
	return longest(x, y);
}

// assign lifetime (e.g. 'a) in struct
struct ImportantExcerpt<'a> {
	part: &'a str, // holds pointer so maintain the original lifetime
}

fn main() {
	let result; // declare
	let string1 = String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaa");
	let string2 = String::from("bbb");

	{
		result = longest_with_announcement(
			string1.as_str(),
			string2.as_str(),
			"Longest String!",
		); // assign
	}

	println!("The longest string is {}", result);

	// lifetime struct references
	let novel = String::from("Some times ago, there was a wizard...");
	let i = ImportantExcerpt { part: &novel };

	println!("The importance is: {}", i.part);
}
