
// [dependencies]
// futures = "0.3.5"
// tokio = { version = "0.2", features = ["full"] }

use futures::{select, future::FutureExt, pin_mut};
use tokio::{runtime::Runtime, task};
use std::io::Result;

async fn function1() -> Result<()> {
    tokio::time::delay_for(tokio::time::Duration::from_secs(10)).await;
    println!("function1 ++++ ");
    Ok(())
}

async fn function2() -> Result<()> {
    println!("function2 ++++ ");
    Ok(())
}

async fn async_main() {
    let f1 = function1().fuse();
    let f2 = function2().fuse();

    pin_mut!(f1, f2);

    select! {
        _ = f1 => println!("task one completed first"),
        _ = f2 => println!("task two completed first"),
    }
}

pub async fn run() {
    task::spawn(async_main());

    // let mut runtime = Runtime::new().unwrap();
    // runtime.block_on(async_main());
    println!("Hello, world!");
}
