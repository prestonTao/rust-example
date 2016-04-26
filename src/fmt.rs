

//http://www.cnblogs.com/chenxizhang/p/4762975.html
pub fn run(){
    print();
}

fn example(){
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {predicate}",
             predicate="over the lazy dog",
             subject="the quick brown fox",
             verb="jumps");
    println!("{} of {:b} people know binary, the other half don't", 1, 2);
    println!("My name is {0}, {1} {0}", "Bond", "cha");
    // struct Structure(i32);
    // println!("This struct `{}` won't print...", Structure(3));
}


use std::fmt;

#[derive(Debug)]
struct Point{ //自定义一个结构体
    x:i32,
    y:i32
}

impl fmt::Display for Point{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          write!(f, "x为{}，y为{}", self.x,self.y)
      }

}

fn print() {
    let p = Point{x:3,y:5};
    println!("{}",p.x);//打印x，这会成功
    println!("{:?}",p);//直接打印整个结构体，没有实现Debug，会失败
    println!("{}",p);//直接打印整个结构体，没有实现Display，会失败

    
}