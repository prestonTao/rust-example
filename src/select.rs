
use std::thread;
use std::time::Duration;
use crossbeam_channel::{select, unbounded};
use rand::Rng;
use async_io::Timer;
extern crate rand;

pub async fn run() {
    for i in 0..100 {    
        example();
    }
    example2();
}

fn example(){
    let (s1, r1) = unbounded();
    let (s2, r2) = unbounded();

    smol::spawn(async move {
		let secret_number = rand::thread_rng().gen_range(1, 101);
		Timer::after(Duration::from_micros(secret_number)).await;
        // thread::sleep(Duration::from_micros(secret_number));
        s1.send(10).unwrap();
	}).detach();

	smol::spawn(async move {
		let secret_number = rand::thread_rng().gen_range(1, 101);
		Timer::after(Duration::from_micros(secret_number)).await;
        // thread::sleep(Duration::from_micros(secret_number));
        s2.send(20).unwrap();
	}).detach();


    // thread::spawn(move || {
    //     let secret_number = rand::thread_rng().gen_range(1, 101);
    //     thread::sleep(Duration::from_micros(secret_number));
    //     s1.send(10).unwrap();
    // });
    // thread::spawn(move || {
    //     let secret_number = rand::thread_rng().gen_range(1, 101);
    //     thread::sleep(Duration::from_micros(secret_number));
    //     s2.send(20).unwrap();
    // });

    //最多执行这两个接收操作中的一个。
    select! {
        recv(r1) -> msg => {
            println!("10");
            assert_eq!(msg, Ok(10))
        },
        recv(r2) -> msg => {
            println!("20");
            assert_eq!(msg, Ok(20))
        },
        default(Duration::from_secs(1)) => println!("timed out"),
    }
}


fn example2(){
    let (s1, r1) = unbounded();
    s1.send(());
    select! {
        recv(r1) -> msg => {
            println!("1111111111111");
            // assert_eq!(msg, Ok(10))
        },
        // recv(r2) -> msg => {
        //     println!("20");
        //     assert_eq!(msg, Ok(20))
        // },
        default(Duration::from_secs(1)) => println!("timed out"),
    }
    println!("finish");
}


