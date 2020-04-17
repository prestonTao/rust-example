extern crate rust_example;
// extern crate std;
// extern crate time;

use rust_example::time_example;

fn main() {
	time_example::run();
	rust_example::fmt::run();
	rust_example::concurrency::run();
	rust_example::mongodb::run();
	rust_example::collections::run();
	rust_example::file::run();
	rust_example::net::run();
	rust_example::futures::run();
	rust_example::libother::tokio::example::run();


	rust_example::test::run();
	
}






