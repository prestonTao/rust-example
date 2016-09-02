

use std::net::{TcpListener, TcpStream};
use std::thread;



fn main(){
	example();
}

fn example(){
	let listener = TcpListener::bind("0.0.0.0:19980").unwrap();

	// match TcpListener::bind("0.0.0.0:19980"){
	// 	Ok(listener) => {
			
	// 	},
	// 	Err(e) => panic!("called `Result::unwrap()` on an `Err` value: {:?}", e),
	// }
	


	

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
    let mut buf: [u8; 1024]  = [0; 1024];
    let n = stream.read(&buf).unwrap();

    for x in &buf {
        println!("{} ", x);
    }
}