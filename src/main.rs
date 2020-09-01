// extern crate rust_example;
// extern crate std;
// extern crate time;

// use rust_example::time_example;
pub mod time_example;
pub mod libother;

use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	time_example::run();
	rust_example::fmt::run();
	rust_example::concurrency::run();
	rust_example::mongodb::run();
	rust_example::collections::run();
	rust_example::file::run();
	rust_example::net::run();
	// rust_example::futures::run();
	// rust_example::libother::tokio::example::run();
	rust_example::test::run();
	libother::tokio::tokioasync2::run().await;
	libother::tokio::tokiotcpserver::run().await;

	libother::tokio::tokioasync::run().await;
	Ok(())
}






