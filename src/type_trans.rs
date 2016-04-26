

use std::mem;

fn Trans(){

	//将4个u8类型转化为一个u32
	unsafe {
		let a = [0u8, 0u8, 0u8, 0u8];
		let b = mem::transmute::<[u8; 4], u32>(a);
	}

	//i32转化为string
	let x: i32 = 32;
	let y = x.to_string();

}
