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
	// write to STDERR
	eprintln!("write to STDERR instead of STDOUT!");

	// reading CMD
	let args: Vec<String> = std::env::args().collect();
	eprintln!("list of arguments: {:?}", args);

	// utilize argument
	let arg1 = &args[0];
	eprintln!("1st argument: {:?}", arg1);

	// read a file
	let contents = std::fs::read_to_string("poem.txt").expect("error reading file");
	eprintln!("Contents are:\n{}", contents);

	// read environment variables
	let config = std::env::var("PATH").unwrap();
	eprintln!("Env. variable for PATH is: {}", config);
}
