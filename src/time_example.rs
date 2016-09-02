
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
use std::fmt;
use std::time as t;
// use std::thread;
// use time::*;


pub fn run(){
	// example();
    println!("start time_example----------");
    getSystemTimeNow();
}

fn example(){
	let x = time::now();       //获取当前完整时间，包括日期
	let y = time::get_time();  //仅获取当前时间
}


/*
	获取系统当前时间
*/
pub fn getSysTimeTest(){
	let start = time::now();//获取开始时间
	println!("系统当前时间为：{:?}", start);
	
    let end = time::now();//获取结束时间
    println!("耗时:{:?}",end-start);
}

pub fn format(){
	let nowTime = time::now();
	let mut timeStr = (nowTime.tm_year + 1900).to_string();
	timeStr.push_str("-");
	timeStr.push_str(&nowTime.tm_mon.to_string());
	timeStr.push_str("-");
	timeStr.push_str(&nowTime.tm_mday.to_string());
	timeStr.push_str(" ");
	timeStr.push_str(&nowTime.tm_hour.to_string());
	timeStr.push_str(":");
	timeStr.push_str(&nowTime.tm_min.to_string());
	timeStr.push_str(":");
	timeStr.push_str(&nowTime.tm_sec.to_string());
    println!("当前时间：{} \n{:?}",timeStr, nowTime);

    // let format = nowTime.strftime("YY-MM--DD");
    // println!("{:?}", format);
}


pub fn getSystemTimeNow(){
    let e = t::SystemTime::now();
    println!("{:?}", e);
}