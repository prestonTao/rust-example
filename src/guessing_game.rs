
extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("请猜一个1-100以内的整数。");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("这个秘密数字是：{}", secret_number);

    loop{
    	println!("请输入你猜测的数字：");

	    let mut guess = String::new();
	    io::stdin().read_line(&mut guess)
	    	.expect("读入行错误");

	    println!("你猜测的数字是：{}", guess);

	    let guess: u32 = match guess.trim().parse(){
	    	Ok(num) => num,
	    	Err(_)  => continue,
	    };

	    match guess.cmp(&secret_number){
	    	Ordering::Less    => println!("太小了！"),
	    	Ordering::Greater => println!("太大了"),
	    	Ordering::Equal   => {
	    		println!("猜中了");
	    		break;
	    	}
	    }
    }
    

}
