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

// declare an unsafe function.
unsafe fn hal() {
	println!("unsafe function called!");
}

// declare unsafe trait.
//unsafe trait Foo {
// ...
//}

//unsafe impl Foo for i32 {
// ...
//}

// call from external languages
#[no_mangle]
pub extern "C" fn call_from_c() {
	println!("Jsut called a rust function from C");
}

fn main() {
	let mut num = 5;

	// *const is a raw unsafe immutable pointer.
	// can be created in safe code not inaccessible in safe scope here.
	let r1 = &num as *const i32;

	// *mut is a raw unsafe mutable pointer.
	// can be created in safe code not inaccessible in safe scope here.
	let r2 = &mut num as *mut i32;

	// use unsafe keyword and scope to run unsafe codes
	unsafe {
		// access unsafe mutable & immutable pointers in unsafe scope.
		println!("r1 is {}", *r1);
		println!("r2 is {}", *r2);

		// call unsafe function
		hal();

		// call extern functiion
		call_from_c();
	}
}
