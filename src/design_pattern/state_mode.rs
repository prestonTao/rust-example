// 状态模式：允许对象在内部状态改变时改变它的行为，对象看起来好像修改了它的类

// 以状态模式而言，我们将一群行为封装在状态对象中，context的行为随时可委托到那些状态对象中的一个。
// 随着时间的流逝，当前状态在状态对象集合中游走改变，以反映出context内部的状态，
// 因此，context的行为也会跟着改变。但是context的客户对于状态对象了解不多，甚至根本是浑然不觉。

// 我们把状态模式想成是不用在context中放置许多条件判断的替代方案。通过将行为包装进状态对象中，
// 你可以通过在context内简单地改变状态对象来改变context的行为

// rust中处理循环引用比较麻烦，要用RC和Week，多线程引用要用ARC。
// 因此不按《headfirst设计模式》中写法来做，而是每次新的状态都是来自上一个状态方法的返回值

struct GumballMachine {
    // 不用Option会有问题，有兴趣的话可尝试去掉Option
    current_state: Option<Box<State>>,
}
impl GumballMachine {
    // 没有&外边无法连续调用该struct的各个方法，会报 use of moved value
    fn insert_quarter(&mut self) {
        // 不用Option会有问题 ^^^^ cannot move out of borrowed content  cannot move out of borrowed content
        if let Some(s) = self.current_state.take() {
            self.current_state = Some(s.insert_quarter())
        }
    }
    fn eject_quarter(&mut self) {
        if let Some(s) = self.current_state.take() {
            self.current_state = Some(s.eject_quarter())
        }
    }
    fn turn_crank(&mut self) {
        if let Some(s) = self.current_state.take() {
            self.current_state = Some(s.turn_crank())
        }
    }
}
trait State {
    fn insert_quarter(self: Box<Self>) -> Box<State>;
    fn eject_quarter(self: Box<Self>) -> Box<State>;
    fn turn_crank(self: Box<Self>) -> Box<State>;
}

struct NoQuarterState {}
impl State for NoQuarterState {
    fn insert_quarter(self: Box<Self>) -> Box<State> {
        println!("receive quarter");
        Box::new(HasQuarterState{})
    }

    fn eject_quarter(self: Box<Self>) -> Box<State> {
        println!("not have quarter, can't eject");
        self
    }

    fn turn_crank(self: Box<Self>) -> Box<State> {
        println!("not have quarter, can't eject");
        self
    }
}

struct HasQuarterState {}
impl State for HasQuarterState {
    fn insert_quarter(self: Box<Self>) -> Box<State> {
        println!("already have quarter");
        self
    }

    fn eject_quarter(self: Box<Self>) -> Box<State> {
        println!("eject quarter");
        Box::new(NoQuarterState{})
    }

    fn turn_crank(self: Box<Self>) -> Box<State> {
        println!("turn crank");
        Box::new(NoQuarterState{})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn state_pattern_test() {
        let mut gumball = GumballMachine{current_state: Some(Box::new(NoQuarterState{}))};

        gumball.turn_crank();

        gumball.insert_quarter();
        gumball.insert_quarter();
        gumball.eject_quarter();

        gumball.insert_quarter();
        gumball.turn_crank();
    }
}