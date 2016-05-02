

use std::collections::HashMap;


pub fn Run(){

}

fn example(){
	// would be `HashMap<&str, &str>` in this example).
	let mut map = HashMap::new();
	map.insert("Foo".to_string(), 42);
	assert_eq!(map.get("Foo"), Some(&42));

	if !map.contains_key("Les Misérables") {
    	println!("We've got {} reviews, but Les Misérables ain't one.", map.len());
	}

	//便利map
	for (book, review) in &map {
    	println!("{}: \"{}\"", book, review);
	}

	//获取某个key对应的value，如果不存在就插入一个
	// let tao = "Tao";
	// let a = map.entry(tao.to_string()).or_insert(Adaptor::new(tao));
	map.remove("The Adventures of Sherlock Holmes");
}