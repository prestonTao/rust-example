

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

pub fn convert(pack_data: &[u8]){  
    let ptr :*const u8 = pack_data.as_ptr();  
    let ptr :*const u32 = ptr as *const u32;  
    let s = unsafe{ *ptr};  
    println!("{:?}", s);  
} 