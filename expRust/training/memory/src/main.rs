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

fn main() {
	// basic ownership and consumption
	let s = String::from("hello"); // `s` come into scope
	take_ownership(s); // `s` valued moved. No longer valid.

	let x: u8 = 5; // `x` come into scope
	make_copy(x); // x is a copy-able by default so still valid.

	println!("x is: {x}"); // `x` is valid and usable.

	// immutable referneces
	let s: String = String::from("hello");
	let l: usize = calculate_length(&s); // `s` is still valid.
	println!("s is: {s} with length {l}"); // `s` and `l` no longer valid.

	println!("s is: {s} with length {l}"); // `s` and `l` no longer valid.

	// mutable references
	let mut s: String = String::from("hello");
	change(&mut s); // send it as a mutable reference
	println!("s is: {s}");

	// slices
	let s: [u8; 5] = [1, 2, 3, 4, 5]; // an array
	let x: &[u8] = &s[1..3]; // a slice from 1 to 2 (right before 3)
	assert_eq!(x, &[2, 3]);
}

fn take_ownership(s: String) {
	println!("got: {s}");
}

fn make_copy(x: u8) {
	println!("got: {x}");
}

fn calculate_length(s: &String) -> usize {
	return s.len();
}

fn change(s: &mut String) {
	s.push_str(", world!");
}
