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
	// create a `String` data type.
	let mut s = String::from("hello"); // a string object

	// NOTE: &s (reference) is equivalent to:
	// let mut s = "hello";

	// [1] append using operator
	let x1 = String::from("hello ");
	let x2 = String::from("Serena");
	let x3 = x1 + &x2; // x1 was moved into here and no longer valid
	println!("x3 is '{}'", x3);

	// [2] append using `pust_str` method
	s.push_str(" world!");
	let s2 = " Quanta"; // double quote is string slice
	s.push_str(s2);

	// [3] append a single (UTF-8 compliant) character using `push` method
	s.push('!'); // single quote is character
	println!("s2 is '{}'", s);

	// [4] looping all characters in a string
	for (i, c) in s.chars().enumerate() {
		println!("character {} is: {}", i, c);
	}

	for (i, b) in s.bytes().enumerate() {
		println!("byte {} is: {}", i, b);
	}
}
