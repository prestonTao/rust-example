
/*
	并发编程
*/

use std::thread;
use std::time::Duration;

pub fn run() {

}


fn thread_ten(){
	for i in 0..10{
		thread::spawn(|| {
			println!("Hello from a thread!");
		});
	}
	thread::sleep(Duration::from_millis(50));
}

static mut a: i32 = 3;
fn changeStatic(){
	unsafe{
		a = 5;
	}
}