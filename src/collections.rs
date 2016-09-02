
use std::collections::HashMap;
use std::collections::HashSet;


pub fn run(){
	// example_hashmap();
	example_hashset();
}

fn example_hashmap(){
	let mut book_reviews: HashMap<&str, &str> = HashMap::new();

	book_reviews.insert("马",     "云");
	book_reviews.insert("刘",     "强东");
	book_reviews.insert("周",     "鸿祎");
	book_reviews.insert("马化腾", "臭婊砸");

	// 检查是否有其中一个元素
	if !book_reviews.contains_key("我") {
	    println!("长度为：{}", book_reviews.len());
	}

	let to_find = ["曾", "轶可"];
	for book in &to_find {
	    match book_reviews.get(book) {
	        Some(review) => println!("{}: {}", book, review),
	        None => println!("{} is unreviewed.", book)
	    }
	}
	// iterate over everything.
	for (book, review) in &book_reviews {
	    println!("{}: \"{}\"", book, review);
	}
	// Use a HashMap to store the vikings' health points.
	let mut vikings = HashMap::new();
	vikings.insert(Viking::new("Einar", "Norway"), 25);
	vikings.insert(Viking::new("Olaf", "Denmark"), 24);
	vikings.insert(Viking::new("Olaf", "Denmark"), 24);
	vikings.insert(Viking::new("Harald", "Iceland"), 12);
	// Use derived implementation to print the status of the vikings.
	for (viking, health) in &vikings {
	    println!("{:?} has {} hp", viking, health);
	}

}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    /// Create a new Viking.
    fn new(name: &str, country: &str) -> Viking {
        Viking { name: name.to_string(), country: country.to_string() }
    }
}


// HashSet
fn example_hashset(){
	let mut books: HashSet<&str> = HashSet::new();
	// Add some books.
	books.insert("A Dance With Dragons");
	books.insert("To Kill a Mockingbird");
	books.insert("The Odyssey");
	books.insert("The Great Gatsby");

	let mut vikings = HashSet::new();
	vikings.insert(VikingSet { name: "Einar", power: 9 });
	vikings.insert(VikingSet { name: "Einar", power: 9 });
	vikings.insert(VikingSet { name: "Olaf", power: 4 });
	vikings.insert(VikingSet { name: "Harald", power: 8 });
	// Use derived implementation to print the vikings.
	for x in &vikings {
	    println!("{:?}", x);
	}

	println!("test take() function");

	let mut s: HashSet<&str> = HashSet::new();
	s.insert("tao");
	s.insert("hong");
	s.insert("fei");
	for (index, one) in s.drain().by_ref().enumerate(){
		println!("{} {}", index, one);
	}

	// match s.get("tao"){
	// 	Some(x) => {
	// 		println!("{}", x);
	// 	},
	// 	None => {
	// 		println!("None");
	// 	},
	// }
	

}

#[derive(Hash, Eq, PartialEq, Debug)]
struct VikingSet<'a> {
    name: &'a str,
    power: usize,
}
