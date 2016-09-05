
use std::thread;
use std::sync::Arc;
use std::cell::RefCell;

fn main(){
	example3();
	thread::sleep_ms(1000*10);
	// let mut a = Arc::new(vec![0]);
	// Arc::make_mut(&mut a).push(2);
	// println!("{:?}", a);
}



fn example3(){
	// let node = Node{name:"nihao".to_string()};
	let block = Box::new([0u8; 1024*1024*100]);

	let mut ptr = TwoPtru8{
		big: RefCell::new(Vec::new()),
		xiao: RefCell::new(Vec::new()),
	};

	thread::sleep_ms(1000*10);
	ptr.put(block);


}

struct TwoPtru8{
	big: RefCell<Vec<Arc<Box<[u8; 1024*1024*100]>>>>,
	xiao: RefCell<Vec<Arc<Box<[u8; 1024*1024*100]>>>>,
}
impl TwoPtru8{
	fn put(&mut self, block: Box<[u8; 1024*1024*100]>){
		// let boxNode = Box::new(node);
		// let rcNode = Arc::new(node);
		// self.xiao.push(Arc::make_mut(rcNode.clone()));
		let arcBlock = Arc::new(block);

		self.big.borrow_mut().push(arcBlock.clone());
		self.xiao.borrow_mut().push(arcBlock.clone());
	}
}


//------------------------------------------
fn example2(){
	let node = Node{name:"nihao".to_string()};
	let mut ptr = TwoPtr{
		big: RefCell::new(Vec::new()),
		xiao: RefCell::new(Vec::new()),
	};

	thread::sleep_ms(1000*10);
	ptr.put(node);


}

struct TwoPtr{
	big: RefCell<Vec<Arc<Node>>>,
	xiao: RefCell<Vec<Arc<Node>>>,
}
impl TwoPtr{
	fn put(&mut self, node: Node){
		// let boxNode = Box::new(node);
		let rcNode = Arc::new(node);
		// self.xiao.push(Arc::make_mut(rcNode.clone()));

		self.big.borrow_mut().push(rcNode.clone());
		self.xiao.borrow_mut().push(rcNode.clone());
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