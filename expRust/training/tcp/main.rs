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

use std::net::TcpListener;

use std::io::prelude::*;
use std::net::TcpStream;

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

// implement Worker Pool for controlled multi-threading
struct Worker {
	id: usize,
	thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
	fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
		let thread = thread::spawn(move || loop {
			let msg = rx.lock().unwrap().recv().unwrap();

			match msg {
				| Message::NewJob(job) => {
					println!("Worker {} got a job; executing.", id);
					job();
				},
				| Message::Terminate => {
					println!("Worker {} was told to terminate.", id);
					break;
				},
			}
		});

		return Worker {
			id: id,
			thread: Some(thread),
		};
	}
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
	NewJob(Job),
	Terminate,
}

struct ThreadPool {
	workers: Vec<Worker>,
	sender: mpsc::Sender<Message>,
}

impl ThreadPool {
	fn new(size: usize) -> ThreadPool {
		assert!(size > 0);

		let (tx, rx) = mpsc::channel();
		let rx = Arc::new(Mutex::new(rx));
		let mut workers = Vec::with_capacity(size);

		for id in 0..size {
			workers.push(Worker::new(id, Arc::clone(&rx)));
		}

		return ThreadPool {
			workers: workers,
			sender: tx,
		};
	}

	fn execute<Fx>(&self, f: Fx)
	where
		Fx: FnOnce() + Send + 'static,
	{
		let job = Box::new(f);
		self.sender.send(Message::NewJob(job)).unwrap();
	}
}

impl Drop for ThreadPool {
	fn drop(&mut self) {
		println!("Sending termination message to all workers.");

		for _ in &self.workers {
			self.sender.send(Message::Terminate).unwrap();
		}

		println!("Shutting down all workers...");

		for worker in &mut self.workers {
			println!("Shutting down worker {}", worker.id);

			if let Some(thread) = worker.thread.take() {
				thread.join().unwrap();
			}
		}
	}
}

// process connection using specific protocols (E.g. HTTP, SSH, etc)
fn handle_connection(mut stream: TcpStream) {
	let mut buffer = [0; 1024];

	stream.read(&mut buffer).unwrap();

	println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

// starting a TCP server
fn main() {
	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	let pool = ThreadPool::new(4);

	for stream in listener.incoming() {
		pool.execute(|| {
			handle_connection(stream.unwrap());
		});
	}

	// ctrl + c to cancel the server
	println!("Shutting down...");
}
