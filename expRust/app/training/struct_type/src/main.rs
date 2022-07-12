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

// defining a struct
//
// The entire struct must be mutable at all time.
struct User {
	email: String,
	name: String,
}

// defining method for a struct
impl User {
	// the first parameter is always `&self` or `&mut self`.
	fn to_string(&mut self) -> String {
		return format!("{} is {}", self.name, self.email);
	}

	// 1 implementation can have multiple methods.
	fn to_debug(&self) {
		println!("DEBUG: The email for {} is {}", self.name, self.email);
	}
}

fn main() {
	// create struct and own it.
	let mut user1: User = User {
		email: String::from("name@domain.com"),
		name: String::from("name core"),
	};

	// modify a field
	user1.email = String::from("local@domain.com");
	println!("The email for {} is {}", user1.name, user1.email);

	// create the factory function
	let mut user2: User = build_user(
		String::from("veronica@email.com"),
		String::from("Veronica Valentine"),
	);

	// call a method from the struct
	println!("{}", user2.to_string());
	user2.to_debug();

	// housing referenced value
	// **COMING SOON** - Lifetime topic
}

fn build_user(email: String, username: String) -> User {
	return User {
		email: email,
		name: username,
	};
}
