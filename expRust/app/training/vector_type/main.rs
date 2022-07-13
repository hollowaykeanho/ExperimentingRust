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
	// create vector list (elements stored next to each other)
	let mut v: Vec<u8> = Vec::new();

	// add elements
	v.push(5);
	v.push(6);
	v.push(7);
	v.push(8);

	// read element from list
	let ret: &u8 = &v[2];
	println!("The element read out is {}", ret);

	match v.get(2) {
		// use Some() due to vector generic nature
		| Some(ret) => println!("The third element is {}", ret),
		// use None to indicate the value is none
		| None => println!("there is no third element."),
		// reading an invalid one shall cause panic
	}

	// iterate over the values in a vector
	for i in &v {
		// read element
		println!("the value is {}", i);

		// update element value
		*i + 50;
		println!("the updated value is {}", i);
	}

	// remove element
	// NOTE: not available at the moment. The only way is to re-create
	//       new vector replacing the old one at the moment.
}
