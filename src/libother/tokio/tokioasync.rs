

use tokio::time::{self, delay_for, Duration};

// #[tokio::main]
// async fn main() {
//     // delay_for(Duration::from_millis(10000)).await;
// }


use std::io;
use tokio::{task, sync::mpsc};
// use std::{thread, time};
use futures::*;
// use std::time::{Duration, Instant};
// use tokio::timer::Delay;
// use tokio_timer::Delay;
// use tokio_timer::Interval;
// use tokio::time::{self, Duration};
// use tokio::timer::Delay;

pub async fn run() {

    

    println!("11111111111");
    // producer
    let (mut tx_in, mut rx_in) = mpsc::channel::<String>(800000);
    task::spawn(async move {
        loop {
            let mut line = String::new();
            line = task::spawn_blocking(move || {
                for i in 0..10{
                    println!("1111 {}", i);
                    async {
                        delay_for(Duration::from_secs(1)).await;
                    };
                }
                io::stdin().read_line(&mut line).unwrap();
                line
            }).await.unwrap();

            tx_in.send(line.trim().to_string()).await.unwrap();
        }
    });

    println!("2222222222");

    // reporter
    let (mut tx_out, mut rx_out) = mpsc::channel::<String>(800000);
    task::spawn(async move {
        loop {
            for i in 0..10{
                println!("2222 {}", i);
                delay_for(Duration::from_secs(1)).await;
            }
            match rx_out.recv().await {
                Some(data) => {
                    task::spawn_blocking(move || {
                        println!("{}", data);
                    }).await.unwrap();
                }, 

                None => ()
            }
        }
    });

    task::spawn(input());

    println!("3333333333");

    // worker
    loop {
        match rx_in.recv().await {
            Some(data) => {
                tx_out.send(data).await.unwrap();
            }, 

            None => ()
        }
    }
}


async fn input(){
    for i in 0..10{
        println!("3333 {}", i);
        delay_for(Duration::from_secs(1)).await;
    }
    // task::spawn_blocking(output().await);

    // let t = Timer::default();
    // t.park_timeout(duration: Duration)
    // tokio_timer::sleep(tt);
    

    // Delay::new(Instant::now() + Duration::from_millis(1000))
    // .map_err(|e| panic!("timer failed; err={:?}", e))
    // .and_then(|_| {
    //     println!("1 Delayed Call Executed!");
    //     Ok(())
    // });


    task::spawn(output());
}

async fn output(){
    // let tt = time::Duration::from_secs(1);
    for i in 0..10{
        println!("4444 {}", i);
        delay_for(Duration::from_secs(1)).await;
    }
    // Delay::new(Instant::now() + Duration::from_millis(1000))
    // .map_err(|e| panic!("timer failed; err={:?}", e))
    // .and_then(|_| {
    //     println!("2 Delayed Call Executed!");
    //     Ok(())
    // });
}


// async fn five(){
//     let delayed = Delay::new(Instant::now() + Duration::from_millis(1000))
//     .map_err(|e| panic!("timer failed; err={:?}", e))
//     .and_then(|_| {
//         println!("Delayed Call Executed!");

//         Ok(())
//     });
// }



