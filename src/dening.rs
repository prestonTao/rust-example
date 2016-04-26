
use std::thread;

struct Philosopher {
	name: String,
}

impl Philosopher {
	fn new(name: &str) -> Philosopher {
		Philosopher {
			name: name.to_string(),
		}
	}

	fn eat(&self){
		println!("{} 吃饭", self.name);
		thread::sleep_ms(1000);
	}
}

fn run() {
	//let p1 = Philosopher::new("Baruch Spinoza");      // 译者注：朱迪斯·巴特勒
	//let p2 = Philosopher::new("Gilles Deleuze");      // 译者注：吉尔·德勒兹
	//let p3 = Philosopher::new("Karl Marx");           // 译者注：卡尔·马克思
	//let p4 = Philosopher::new("Friedrich Nietzsche"); // 译者注：弗里德里希·威廉·尼采
	//let p5 = Philosopher::new("Michel Foucault");     // 译者注：米歇尔·福柯

	let philosophers = vec![
		Philosopher::new("Baruch Spinoza"),
		Philosopher::new("Gilles Deleuze"),
		Philosopher::new("Karl Marx"),
		Philosopher::new("Friedrich Nietzsche"),
		Philosopher::new("Michel Foucault"),
	];
	for p in philosophers{
		p.eat();
	}
}

