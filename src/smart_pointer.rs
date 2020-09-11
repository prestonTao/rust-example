use std::{rc::Rc, cell::RefCell, rc::Weak, cell::Cell};


//智能指针
pub fn run(){
    Box();
    Rc_share();
    loopPointer();
    Cell();
    Refcell();
}

//Box
fn Box(){
    let a = Box::new("hello");
    let b = Box::new("rust".to_string());
    let c = *a;//解引用移动，目前支持此行为的智能指针只有Box<T>
    let d = *b;
    println!("{:?}", a);
}


//共享所有权 Rc<T>和 Weak<T>
fn Rc_share(){
    let x = Rc::new(45);
    let y1 = x.clone();//增加强引用计数
    let y2 = x.clone();//增加强引用计数
    println!("强引用计数：{:?}",Rc::strong_count(&x));
    let w = Rc::downgrade(&x);//增加弱引用计数
    println!("弱引用计数：{:?}",Rc::weak_count(&x));
    let y3 = &*x;//不增加计数
    println!("{}", 100 - *x);
}

//利用Weak<T>解决循环引用的内存泄露问题
fn loopPointer(){
    let first = Rc::new(RefCell::new(Node{next:None,head:None}));
    let second = Rc::new(RefCell::new(Node{next:None,head:None}));
    let thrid = Rc::new(RefCell::new(Node{next:None,head:None}));

    first.borrow_mut().next = Some(second.clone());
    second.borrow_mut().next = Some(thrid.clone());
    thrid.borrow_mut().head = Some(Rc::downgrade(&first));

}
struct Node{
    next:Option<Rc<RefCell<Node>>>,
    head:Option<Weak<RefCell<Node>>>
}
impl Drop for Node{
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

//内部可变性 Cell<T>
//尽量不要用它包裹大的结构体，应该选择包装某个字段，因为Cell<T>内部每次get/set都会执行一次按位复制
struct Foo{
    x: u32,
    y: Cell<u32>
}
fn Cell(){
    let foo = Foo{x:1, y:Cell::new(3)};
    assert_eq!(1, foo.x);
    assert_eq!(3, foo.y.get());
    foo.y.set(5);
    assert_eq!(5, foo.y.get());
}

//内部可变性RefCell<T>
fn Refcell(){
    let x = RefCell::new(vec![1,2,3,4]);
    println!("{:?}",x.borrow());
    x.borrow_mut().push(5);
    println!("{:?}", x.borrow());

    let mut mut_v = x.borrow_mut();
    mut_v.push(6);
    println!("{:?}", mut_v);
    // let mut mut_v2 = x.borrow_mut(); //运行时异常
}




