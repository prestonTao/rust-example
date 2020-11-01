
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
	
	runTimeout();
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



//下面是一个全局超时回调定时器

fn program_main() {
    println!("So we start the program here!");
    set_timeout(200, || {
        println!("We create tasks with a callback that runs once the task finished!");
    });
    set_timeout(100, || {
        println!("We can even chain sub-tasks...");
        set_timeout(50, || {
            println!("...like this!");
        })
    });
    println!("While our tasks are executing we can do other stuff instead of waiting.");
}

fn runTimeout() {
    RT.with(|rt| rt.run(program_main));
}

use std::sync::mpsc::{channel, Receiver, Sender};
use std::{cell::RefCell, collections::HashMap, thread};

thread_local! {
    static RT: Runtime = Runtime::new();
}

struct Runtime {
    callbacks: RefCell<HashMap<usize, Box<dyn FnOnce() -> ()>>>,
    next_id: RefCell<usize>,
    evt_sender: Sender<usize>,
    evt_reciever: Receiver<usize>,
}

fn set_timeout(ms: u64, cb: impl FnOnce() + 'static) {
    RT.with(|rt| {
        let id = *rt.next_id.borrow();
        *rt.next_id.borrow_mut() += 1;
        rt.callbacks.borrow_mut().insert(id, Box::new(cb));
        let evt_sender = rt.evt_sender.clone();
        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_millis(ms));
            evt_sender.send(id).unwrap();
        });
    });
}

impl Runtime {
    fn new() -> Self {
        let (evt_sender, evt_reciever) = channel();
        Runtime {
            callbacks: RefCell::new(HashMap::new()),
            next_id: RefCell::new(1),
            evt_sender,
            evt_reciever,
        }
    }

    fn run(&self, program: fn()) {
        program();
        for evt_id in &self.evt_reciever {
            let cb = self.callbacks.borrow_mut().remove(&evt_id).unwrap();
            cb();
            if self.callbacks.borrow().is_empty() {
                break;
            }
        }
    }
}
