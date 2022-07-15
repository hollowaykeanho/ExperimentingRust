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
	// [1] create new thread with spawn
	let handle = std::thread::spawn(|| {
		for i in 1..10 {
			eprintln!("hi number {} from the spawned thread!", i);
			std::thread::sleep(std::time::Duration::from_millis(1));
		}
	});

	for i in 1..5 {
		eprintln!("hi number {} from the main thread!", i);
		std::thread::sleep(std::time::Duration::from_millis(1));
	}

	// wait for all handle to be completed before proceeding
	handle.join().unwrap();

	// [2] channel
	let (tx_channel, rx_channel) = std::sync::mpsc::channel();

	// spawn new thread that moves data
	std::thread::spawn(move || {
		let val = String::from("hi");
		tx_channel.send(val).unwrap();
	});

	// read from receiver channel
	let received = rx_channel.recv().unwrap(); // this blocks and waits indefinitely
	eprintln!("received channel message: {}", received);

	let received = rx_channel.try_recv(); // this does not block
	match received {
		| Ok(ret) => println!("a message is available: {:?}", ret),
		| Err(err) => println!("no message is available: {:?}", err),
	}

	// [3] atmoic mutex lock
	let counter = std::sync::Arc::new(std::sync::Mutex::new(0));
	let mut handles = vec![];

	for _ in 0..10 {
		let counter = std::sync::Arc::clone(&counter);
		let handle = std::thread::spawn(move || {
			let mut num = counter.lock().unwrap();

			*num += 1;
		});
		handles.push(handle);
	}

	for handle in handles {
		handle.join().unwrap();
	}

	eprintln!("Result: {}", *counter.lock().unwrap());

	// Fin
	eprintln!("main thread completed!");
}
