use tokio::time::{self, delay_for, Duration};
use tokio::task;
use std::{collections::HashMap, thread};


pub async fn run() {
    
    task::spawn(one());
    task::spawn(two());

    delay_for(Duration::from_millis(6000)).await;
}

async fn one(){
    for i in 0..10{
        println!("one {}", i);
        // delay_for(Duration::from_millis(500)).await;
        thread::sleep_ms(500);
    }
}

async fn two(){
    for i in 0..10{
        println!("two {}", i);
        // delay_for(Duration::from_millis(500)).await;
        thread::sleep_ms(500);
    }
}

fn example(x: &i32) -> &i32 {
    &x
}

fn example2() -> i32 {
    7
}
