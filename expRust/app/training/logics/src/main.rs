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
	// nested if else loop
	let number: u8 = 6;
	if number % 4 == 0 {
		println!("{number} is divisible by 4");
	} else if number % 3 == 0 {
		println!("{number} is divisible by 3");
	} else if number % 2 == 0 {
		println!("{number} is divisible by 2");
	} else {
		println!("{number} is not divisible by 4, 3, and 2.");
	}

	// matching (replacing switch convention)
	let number: u8 = 6;
	match number {
		| 1 => println!("It is 1."),
		| 2 => println!("It is 2."),
		| 3 => println!("It is 3."),
		| 4 => println!("It is 4."),
		| 5 => println!("It is 5."),
		| 6 => println!("It is 6."),
		| _ => println!("It is unlisted"),
	}
}
