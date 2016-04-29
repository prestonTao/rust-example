
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
	
}