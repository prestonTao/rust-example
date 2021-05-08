// extern crate rust_example;
// extern crate std;
// extern crate time;

// use rust_example::time_example;
pub mod time_example;
pub mod libother;
pub mod smart_pointer;
pub mod thread_safe;
pub mod memory_reordering;
pub mod trait_usage;

use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

	// smart_pointer::run();


	// time_example::run();
	// rust_example::fmt::run();
	// rust_example::concurrency::run();
	// rust_example::mongodb::run();
	// rust_example::collections::run();
	// rust_example::file::run();
	// rust_example::net::run();
	// rust_example::futures::run();
	// rust_example::libother::tokio::example::run();
	// rust_example::test::run().await;
	// libother::tokio::tokioasync2::run().await;
	// libother::tokio::tokiotcpserver::run();
	// rust_example::select::run().await;
	// rust_example::time_example::run();


	// libother::once_cell::once_cell_example::run();

	// libother::async_std::simple1::run();
	// libother::pnet::simple::run();
	// thread_safe::s4::run().await;
	// memory_reordering::spinlock::run();
	trait_usage::auth::run();



	//这个方法有异步监听命令行输入，留在最后
	// libother::tokio::tokioasync::run().await;
	Ok(())
}






