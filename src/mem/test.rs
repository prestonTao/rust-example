
use std::thread;
fn main(){
	example2();
	// thread::sleep_ms(1000*10);
}

fn example2(){
	let node = Node{name:"nihao".to_string()};
	let mut ptr = TwoPtr{
		big: Vec::new(),
		xiao: Vec::new(),
	};
	ptr.put(&node);

}

struct TwoPtr<'a>{
	big: Vec<Node>,
	xiao: Vec<&'a Node>,
}
impl<'a> TwoPtr<'a>{
	fn put(&mut self, node: &Node){
		self.xiao.push(node);
		self.big.push(*node);
	}
}
struct Node{
	name: String,
}

//------------------------------------------
/*
	在堆上分配内存实验
*/
fn example(){
	// deref

	let mut q = Queue::new();

	while true {
		let temp = q.get();
		q.put(temp);

		thread::sleep_ms(1000);
	}
}

struct Queue{
	pool: Vec<Box<[u8; 1024*1024*100]>>,
}

impl Queue{
	fn new() -> Queue{
		return Queue{
			pool: Vec::new(),
		}
	}

	fn get(&mut self) -> Box<[u8; 1024*1024*100]>{
		match self.pool.pop(){
			Some(val) => return val,
            None => return Box::new([0u8; 1024*1024*100]),
		}
		// return self.pool.pop().expect("");
	}

	fn put(&mut self, b: Box<[u8; 1024*1024*100]>){
		self.pool.push(b);
	}
}