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

// Gentle Note About Generics: **FEEL FREE AND SAFE TO USE**
// 1. Rust compiler deploy 'monomorphization' at pre-compile time = meaning that
//    all generic codes are translated back into specific code pattern before
//    compiling into binary. Hence, there **SHOULD NOT** be any runtime penalty.

// generic function with trait restrictions
fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
	let mut largest = list[0];

	for &item in list {
		if item > largest {
			largest = item;
		}
	}

	return largest;
}

// generic struct with multi fields
struct Point<X, Y> {
	x: X,
	y: Y,
}

// generic method implementation (matching generic params to params)
impl<X, Y> Point<X, Y> {
	fn x(&self) -> &X {
		return &self.x;
	}

	fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X, Y2> {
		return Point {
			x: self.x,
			y: other.y,
		};
	}
}

// generic enum
enum Core<A, B> {
	Ok(A),
	Bad(B),
}

// deploying generic enum
fn enum_core_value(is_ok: bool) -> Core<String, bool> {
	if is_ok {
		return Core::Ok(String::from("yeap"));
	}

	return Core::Bad(false);
}

fn main() {
	// using generic function with trait restrictions
	let number_list = vec![32, 50, 25, 100, 65];
	let float_list = vec![50.2, 25.3, 100.1, 72.5];
	let char_list = vec!['y', 'm', 'a', 'q'];
	println!("Largest from the list: '{}'", largest(&number_list));
	println!("Largest from the list: '{}'", largest(&float_list));
	println!("Largest from the list: '{}'", largest(&char_list));

	// using generic struct with multi fields
	let both_integer = Point { x: 5, y: 10 };
	let both_float = Point { x: 5.0, y: 10.0 };
	let integer_and_float = Point { x: 5, y: 10.0 };

	println!(
		"both int x: '{}' and y '{}'",
		both_integer.x, both_integer.y
	);
	println!("both float x: '{}' and y '{}'", both_float.x, both_float.y);
	println!(
		"int and float x: '{}' and y '{}'",
		integer_and_float.x, integer_and_float.y
	);
	println!("x via method: '{}'", integer_and_float.x());

	let mixed = both_float.mixup(both_integer);
	println!("mixed up x: '{}' and y '{}'", mixed.x, mixed.y);

	// using generic enum
	match enum_core_value(true) {
		| Core::Ok(ret) => println!("enum is: {}", ret),
		| Core::Bad(ret) => println!("enum is: {}", ret),
	}
}
