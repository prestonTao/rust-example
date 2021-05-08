
use std::thread;
use std::time::Duration;
use blocking::unblock;
// use crossbeam_channel::{select, unbounded, Sender, Receiver};
use async_channel::{unbounded, Receiver, Sender};
use rand::Rng;
use async_io::Timer;
extern crate rand;

pub async fn run() {
    example().await;
}

async fn example(){
	let (s1, r1) = unbounded();
	smol::spawn(dispatch(s1)).detach();
	r1.recv().await;
	// Timer::after(Duration::from_secs(10));
}


async fn dispatch(sender: Sender<()>){

	// Timer::after(Duration::from_secs(1));
	let (s2, r2) = unbounded();

	let count = 10;
	// for i in 0..count {
		smol::spawn(task(s2.clone())).detach();
	// }

	// for i in 0..count {
		r2.recv().await;
	// }

	
	sender.send(()).await;
}

async fn task(sender: Sender<()>){
	let secret_number = rand::thread_rng().gen_range(1, 10);
	println!("任务等待 {} 秒", secret_number);
	Timer::after(Duration::from_secs(secret_number));
	sender.send(()).await;
}
