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

// More info: https://doc.rust-lang.org/reference/macros-by-example.html

// declare a simple procedural macro
macro_rules! say_hello {
	// () indicates macro takes no arguments.
	() => {
		// execution of the macro are here.
		println!("Hello!");
	};
}

// create function macro
macro_rules! create_function {
	// takes an argument ($func_name) and create a function (ident type)
	($func_name:ident) => {
		fn $func_name() {
			println!("You called {:?}()", stringify!($func_name));
		}
	};
}

// create expression macro
macro_rules! print_result {
	// takes an argument ($expression) and create an expression (expr type)
	($expression:expr) => {
		println!("{:?} = {:?}", stringify!($expression), $expression);
	};
}

// create an overloading macro
macro_rules! test {
	($left:expr; and $right:expr) => {
		println!(
			"{:?} and {:?} is {:?}",
			stringify!($left),
			stringify!($right),
			$left && $right
		);
	};
	($left:expr; or $right:expr) => {
		println!(
			"{:?} and {:?} is {:?}",
			stringify!($left),
			stringify!($right),
			$left || $right
		);
	};
}

// using repetitive argument macro
macro_rules! find_min {
	// 1 case
	($x:expr) => ($x);

	// multiple cases
	($x:expr, $($y:expr),+) => (
		std::cmp::min($x, find_min!($($y),+))
	);
}

// using domain specific language macro
macro_rules! calculate {
	// eval is a caller
	(eval $e:expr) => {{
		{
			let val: usize = $e;
			println!("{} = {}", stringify!{$e}, val);
		}
	}};

	// perform variadic macro
	(eval $e:expr, $(eval $es:expr),+) => {{
		calculate! { eval $e }
		calculate! { $(eval $es),+ }
	}};
}

// in main source codes
create_function!(foo); // create a foo function using macro
create_function!(bar); // create a bar function using macro

fn main() {
	say_hello!(); // call macro directory
	foo(); // call macro created function
	bar(); // call macro created function

	print_result!(1u32 + 1); // call expression

	// call overloading macro
	test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
	test!(true; or false);

	// call repetitive argument macro
	println!("min {}", find_min!(1));
	println!("min {}", find_min!(1 + 2, 2));
	println!("min {}", find_min!(5, 2 * 3, 4));

	// calling domain specific language macro
	calculate! {
		eval 1 + 2,
		eval 3 + 4,
		eval (1 + 2) * (3 / 4)
	}
}
