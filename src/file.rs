use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub fn run(){
	// readContent();
	// readFileBytes();
}

//读文件内容
fn readContent() {
	//创建一个文件
	let mut newfile = File::create("temp/foo.txt").unwrap();
	newfile.write_all(b"Hello, world!");
	newfile.sync_all();

	let mut f = File::open("temp/log.txt").unwrap();
	let mut reader = BufReader::new(f);
	let mut line = String::new();
	let len = reader.read_line(&mut line).unwrap();
	println!("第一行有多少个字节 {}, 内容是：{}", len, line);
}


//
fn readFileBytes(){
	// let filepath = Path::new("temp/log.txt");
	//   \\\\.\\physicaldrive0
	let mut f = File::open("temp/log.txt").unwrap();
	println!("{:?}", f);
	let mut read_mem = [0; 512];
	let read_buf = &mut read_mem[0..512];
	let mbr_data = f.read(read_buf).unwrap();
	// let mbr = mbr_data.as_slice().unwrap();
	println!("多少字节 {}， 内容：{:?}", mbr_data, read_buf);
}