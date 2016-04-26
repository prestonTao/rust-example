#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};


fn main() {
    println!("Hello, world!");

    let mut server = Nickel::new();
    server.get("**",middleware!("hello"));
    server.listen("127.0.0.1:80");
    std::mem::
}
