
// extern crate model;


// #[warn(non_snake_code)]
// pub fn callP(){
// 	super::model::p();
// }

// pub fn p(){
// 	println!("hello time");
// }

// #[test]
// pub fn time_example(){

// }

// #[test]
// pub fn time_two(){

// }

// extern crate std;
extern crate time;
use std::mem;
// use std::thread;
// use time::*;

/*
	获取系统当前时间
*/
pub fn getSysTimeTest(){
	let start = time::now();//获取开始时间
	println!("系统当前时间为：{:?}", start);
	
	let timeStr = start.tm_sec.to_string();
	println!("当前时间为：{:?}",timeStr);

	// let mut b = "";
	// unsafe {
	// 	// let a = [0u8, 0u8, 0u8, 0u8];
	// 	let b = mem::transmute::<i32, String>(timeStr);
	// }


    let end = time::now();//获取结束时间
    println!("耗时:{:?}",end-start);
}

pub fn haha(){
	println!("haha");
}


// pub fn 