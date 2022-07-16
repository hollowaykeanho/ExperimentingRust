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

#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

fn main() {
	// closure for immutable borrowing
	let list = vec![1, 2, 3];
	let only_borrows = || println!("From closure: {:?}", list);
	println!("Before calling closure: {:?}", list);
	only_borrows();
	println!("After calling closure: {:?}", list);

	// closure for mutable borrowing
	let mut list = vec![1, 2, 3];
	println!("Before calling closure: {:?}", list);
	let mut borrow_append = || list.push(7);
	borrow_append();
	println!("After calling closure: {:?}", list);

	// example use: processing list
	let mut list = [
		Rectangle {
			width: 10,
			height: 1,
		},
		Rectangle {
			width: 2,
			height: 1,
		},
		Rectangle {
			width: 5,
			height: 2,
		},
	];

	let mut num_sort_ops = 0;
	list.sort_by_key(|r| {
		num_sort_ops += 1;
		return r.width;
	});
	println!("{:#?}, sorted in {num_sort_ops} operations", list);
}
