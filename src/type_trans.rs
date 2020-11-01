

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

//把字节数字转化为字符串例子
pub fn c(){
	use std::fmt::Write;

    let mut signature_string = String::new();
    let signature_code= [177,187,102,36,165,137,39,
	63,52,197,173,13,168,216,95,3,175,113,213,98,52,
	77,175,152,79,188,119,141,52,19,19,53,]; 
    
    //for a in signature_code().iter() { 
    for a in signature_code.iter() { 
        //println!(" N: {:x?}", a); 
        //signature_string.push(a);
        write!(signature_string, "{:02x}", a);
    }
	
    println!("the entire array HEX as a single string: {}", signature_string);
}

pub fn convertString(bs: Vec<i32>) -> String {
	let mut resultStr = String::new();
	for a in &bs{
        // write!(resultStr, "{:02x}", a);
        resultStr.push_str(&a.to_string());
	}
	resultStr
}


pub fn simple(){
    //i32 转 u32
    let x:i32 = 12;
    let y = x as u32;//y的类型u32

    //i32 转 f64
    let x:i32 = 12;
    let y = x as f64;//y的类型f64

    //i32 转 String
    let x:i32 = 12;
    let y = x.to_string();//y的类型String
   
    //String 转 i32
    let str = String::from("12");
    let y = str.parse::<i32>().unwrap();
  
    //String 转 u32
    let str = String::from("12");
    let y = str.parse::<u32>().unwrap();

    //String 转 &str
    let str = String::from("12");
    let y = str.parse::<f32>().unwrap();

    //&str 转 String
    let str:&str = "heelo";
    let string:String = str.to_string();
}