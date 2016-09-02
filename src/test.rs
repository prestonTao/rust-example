#![feature(box_syntax)]

extern crate rand;
// use rand::Rng;
use std::thread;


pub fn run(){
    println!("start test----------");
    example();
}




/*
	创建数组
*/
pub fn example(){
	// let secret_number = rand::thread_rng().gen_range(1, 101);
 //    println!("这个秘密数字是：{}", secret_number);
    
	// let topOne = Top{
	// 	name: "nihao".to_string(),
	// };
	// let mut temp = vec![topOne];
	// let a: [Vec<Top>; 20] = [&mut temp; 20];
	// let pool: [Vec<u8>;20] = [Vec::new();20];

	// let pool: Vec<[u8;1024*1024]> = Vec::new();

	let block = Box::new([0u8; 1024*1024]);

	thread::sleep_ms(1000*5);


    
}
// struct Top{
// 	name: String,
// }