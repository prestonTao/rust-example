

use std::net::{TcpListener, TcpStream};
use std::thread;



pub fn run(){
	println!("start net-------------------------");
	// example();
}

fn example(){
	

	let listener = TcpListener::bind("127.0.0.1:80").unwrap();

	

	// accept connections and process them, spawning a new thread for each one
	for stream in listener.incoming() {
	    match stream {
	        Ok(stream) => {
	            thread::spawn(move|| {
	                // connection succeeded
	                handle_client(stream)
	            });
	        }
	        Err(e) => { /* connection failed */ }
	    }
	}

	// close the socket server
	drop(listener);
}

fn handle_client(stream: TcpStream) {
    // ...
}