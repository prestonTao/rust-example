
//模式
fn Patterns(){
	let (x, y) = (1, 2);
}


//可变性
fn Mutability(){
	let mut x = 1;
	x = 2;

	let mut x: i32 = 1;
	x = 7;
	let x = x; // x is now immutable and is bound to 7
	let y = 4;
	let y = "I can also be bound to text!"; // y is now of a different type
}

//函数，调用方法：print_sum(5, 6);
fn print_sum(x: i32, y: i32) {
	println!("sum is: {}", x + y);
}

//有返回值的函数
fn add_one(x: i32) -> i32 {
	x + 1
}

//提早返回
fn EarlyReturns(x: i32) -> i32 {
	return x;
	// we never run this code!
	x + 1
}

/*
	发散函数，发散函数可以被用作任何类型
	let x: i32 = DivergingFunctions();
	let x: String = DivergingFunctions();
*/
fn DivergingFunctions() -> ! {
	panic!("This function never returns!");
}

/*
	函数指针
	// without type inference
	let f: fn(i32) -> i32 = plus_one;
	// with type inference
	let f = plus_one;

	你可以用 f 来调用这个函数：
	let six = f(5);
*/

//原生类型
fn PrimitiveTypes(){
	//布尔型
	let a = true;
	let b: bool = false;
	//char   Rust的char并不是1个字节，而是4个
	let c = 'x';
	// let two_hearts = '';

	//数字类型
	//这里有一个不同数字类型的列表，以及它们在标注库中的文档：
	// i8  i16  i32  i64  u8  u16  u32  u64  isize  usize  f32  f64
	let d = 42; // x has type i32
	let e = 1.0; // y has type f64

	//数组
	let f = [1, 2, 3];     // a: [i32; 3]
	let mut g = [1, 2, 3]; // m: [i32; 3]
	let h = [0; 20]; // a: [i32; 20]
	println!("a has {} elements", f.len());//获取数组长度
	let one = f[1]; //通过下标访问特定元素

	//切片
	let i = [0, 1, 2, 3, 4];
	let complete = &i[..]; // i中所有的元素
	let middle = &i[1..4]; // 下标为1-4的所有元素：1, 2, and 3

	//字符串
	let mut timeStr = "haha";
	// &timeStr.push_str("-".to_string());

	//元祖
	let x = (1, "hello");
	let x: (i32, &str) = (1, "hello");
	//你可以把一个元组赋值给另一个，如果它们包含相同的类型和数量。当元组有相同的长度时它们有相同的数量。
	let mut x = (1, 2); // x: (i32, i32)
	let y = (2, 3); // y: (i32, i32)
	x = y;

	let (x, y, z) = (1, 2, 3);
	println!("x is {}", x);

	/*
		你可以一个逗号来消除一个单元素元组和一个括号中的值的歧义：
		(0,); // single-element tuple
		(0); // zero in parentheses
	*/
	let tuple = (1, 2, 3);
	let t0 = tuple.0;
	let t1 = tuple.1;
	let t2 = tuple.2;

	


}



// if 语句
fn If(){
	let x = 5;
	if x == 5 {
		println!("x is five!");
	} else if x == 6 {
		println!("x is six!");
	} else {
		println!("x is not five or six :(");
	}
	//也可以这样写
	let y = if x == 5 { 10 } else { 15 }; // y: i32
}

// 循环
fn Loop(){
	loop{
		println!("Loop forever!");
	}

	let mut x = 5; // mut x: i32
	let mut done = false; // mut done: bool
	while !done {
		x += x - 3;
		println!("{}", x);
		if x % 5 == 0 {
			done = true;
		}
	}

	for x in 0..10 {
		println!("{}", x); // x: i32
		break;
	}

	//当你需要记录你已经循环了多少次了的时候，你可以使用 .enumerate() 函数。
	for (i,j) in (5..10).enumerate() {
		println!("i = {} and j = {}", i, j);
	}
	let lines = "hello\nworld".lines();
	for (linenumber, line) in lines.enumerate() {
		println!("{}: {}", linenumber, line);
	}
	/*
		'outer: for x in 0..10 {
			'inner: for y in 0..10 {
				if x % 2 == 0 { continue 'outer; } // continues the loop over x
				if y % 2 == 0 { continue 'inner; } // continues the loop over y
				println!("x: {}, y: {}", x, y);
			}
		}
	*/
}

//移动语义
fn MoveSemantics(){
	let v1 = vec![1, 2, 3];
	let v2 = vec![1, 2, 3];
	let (v1, v2, answer) = MoveSemantics_foo(v1, v2);
	let answer = MoveSemantics_foo_too(v1, v2);
}

//在方法的返回交还所有权
fn MoveSemantics_foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
	// do stuff with v1 and v2
	// hand back ownership, and the result of our function
	(v1, v2, 42)
}
//使用借用
fn MoveSemantics_foo_too(v1: Vec<i32>, v2: Vec<i32>) -> i32 {
	// v1.push(5);
	42
}

//借用
fn JieYong(){
	let mut x = 5;
	let mut xs = vec![1, 2, 3];
	{
		let y = &mut x; // -+ &mut borrow starts here
		*y += 1; // |

		let ys = &mut xs;
		ys.push(4);
	} // -+ ... and ends here
	println!("{}", x); // <- try to borrow x here
}


//迭代器失效
fn IteratorInvalidation(){
	let mut v = vec![1, 2, 3];
	for i in &v {
		println!("{}", i);
	}
}


//生命周期
fn LifeTime(){
	let y = &5; // this is the same as `let _y = 5; let y = &_y;`
	let f = Foo { x: y };
	println!("x: {}, x: {}", f.x, f.x());

	let x: &'static str = "Hello, world.";
	let x: &'static i32 = &FOO;
}

// implicit
fn LifeTime_foo(x: &i32) {
}
// explicit
fn LifeTime_bar<'a, 'b>(x: &'a i32, y: &'b i32) {
}

struct Foo<'a> {
	x: &'a i32,
}

impl<'a> Foo<'a> {
	fn x(&self) -> &'a i32 { self.x }
}

static FOO: i32 = 5;



//生命周期省略（Lifetime Elision）

// fn print(s: &str); // elided
// fn print<'a>(s: &'a str); // expanded
// fn debug(lvl: u32, s: &str); // elided
// fn debug<'a>(lvl: u32, s: &'a str); // expanded
// // In the preceding example, `lvl` doesn’t need a lifetime because it’s not a
// // reference (`&`). Only things relating to references (such as a `struct`
// // which contains a reference) need lifetimes.
// fn substr(s: &str, until: u32) -> &str; // elided
// fn substr<'a>(s: &'a str, until: u32) -> &'a str; // expanded
// fn get_str() -> &str; // ILLEGAL, no inputs
// fn frob(s: &str, t: &str) -> &str; // ILLEGAL, two inputs
// fn frob<'a, 'b>(s: &'a str, t: &'b str) -> &str; // Expanded: Output lifetime is ambigu
// ous
// fn get_mut(&mut self) -> &mut T; // elided
// fn get_mut<'a>(&'a mut self) -> &'a mut T; // expanded
// fn args<T:ToCStr>(&mut self, args: &[T]) -> &mut Command; // elided
// fn args<'a, 'b, T:ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command; // expanded
// fn new(buf: &mut [u8]) -> BufWriter; // elided
// fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>; // expanded



//可变性
use std::sync::Arc;
use std::cell::Cell;

fn KeBianXin(){
	let mut x = 5;
	let mut y = &mut x;

	let (mut x, y) = (5, 6);

	let xarc = Arc::new(5);
	let yarc = xarc.clone();

	//结构体可变性
	let mut a = Point { x: 5, y: 6 };
	a.x = 10;

	//模拟字段级别的可变性
	let point = PointToo { x: 5, y: Cell::new(6) };
	point.y.set(7);
	println!("y: {:?}", point.y);
}

struct Point{
	x: i32,
	y: i32,
}

struct PointToo {
	x: i32,
	y: Cell<i32>,
}


//结构体
fn Struct(){
	let mut point = Point { x: 0, y: 0 };
	{
		let r = PointRef { x: &mut point.x, y: &mut point.y };
		*r.x = 5;
		*r.y = 6;
	}
	assert_eq!(5, point.x);
	assert_eq!(6, point.y);
}
//带可变指针的结构体
struct PointRef<'a> {
	x: &'a mut i32,
	y: &'a mut i32,
}


//更新语法（Update syntax）
//一个包含 .. 的 struct 表明你想要使用一些其它结构体的拷贝的一些值
fn UpdateSyntax(){
	let mut point = Point3d { x: 0, y: 0, z: 0 };
	point = Point3d { y: 1, .. point };

	let origin = Point3d { x: 0, y: 0, z: 0 };
	point = Point3d { z: 1, x: 2, .. origin };
}
struct Point3d {
	x: i32,
	y: i32,
	z: i32,
}
