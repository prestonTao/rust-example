
use std::thread;
use std::time::Duration;
use crossbeam_channel::{select, unbounded};

pub async fn run() {
    for i in 0..100 {    
        example();
    }
}

fn example(){
    let (s1, r1) = unbounded();
    let (s2, r2) = unbounded();

    thread::spawn(move || {
        thread::sleep(Duration::from_micros(10));
        s1.send(10).unwrap();
    });
    thread::spawn(move || {
        s2.send(20).unwrap();
    });

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


