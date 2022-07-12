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

// enum in rust has a different declarations
enum IpAddr {
	V4(u8, u8, u8, u8),
	V6(String),
}

// like struct, it can be defined with its own methods
impl IpAddr {
	fn to_string(&self) -> String {
		match self {
			| Self::V4(a, b, c, d) => return format!("{}.{}.{}.{}", a, b, c, d),
			| Self::V6(a) => return format!("{}", a),
		}
	}
}

fn main() {
	let home = IpAddr::V4(127, 0, 0, 1);
	let loopback = IpAddr::V6(String::from("::1"));

	println!("{}", home.to_string());
	println!("{}", loopback.to_string());
}
