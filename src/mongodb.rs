
//https://github.com/mongodb-labs/mongo-rust-driver-prototype

// use bson::Bson;
// use mongodb::{Client, ThreadedClient};
// use mongodb::db::ThreadedDatabase;


pub fn run(){
	// example()
}



// fn example(){
// 	let client = Client::connect("localhost", 27017)
//         .ok().expect("链接服务器错误");

//     let coll = client.db("test").collection("movies");

//     let doc = doc! { "title" => "Jaws",
//                       "array" => [ 1, 2, 3 ] };

//     // Insert document into 'test.movies' collection
//     coll.insert_one(doc.clone(), None)
//         .ok().expect("插入文档错误");

//     // Find the document and receive a cursor
//     let mut cursor = coll.find(Some(doc.clone()), None)
//         .ok().expect("查询错误.");

//     let item = cursor.next();

//     // cursor.next() returns an Option<Result<Document>>
//     match item {
//         Some(Ok(doc)) => match doc.get("title") {
//             Some(&Bson::String(ref title)) => println!("{}", title),
//             _ => panic!("Expected title to be a string!"),
//         },
//         Some(Err(_)) => panic!("Failed to get next from server!"),
//         None => panic!("Server returned no results!"),
//     }
// }
