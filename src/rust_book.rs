
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