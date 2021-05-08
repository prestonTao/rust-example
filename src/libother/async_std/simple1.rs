// use std::thread;
use std::time;
use async_std;
use futures::future::{join, join_all};
use async_std::task::sleep;



pub fn run(){
    let start = time::Instant::now();
    async_std::task::block_on(main_exe());
    println!("执行时间：{:?}", start.elapsed());
}

async fn main_exe(){
    hello().await;
    // let fnItems = vec![common_db(), open_file()];
    let (db, file) = join(common_db(), open_file()).await;
    println!("{} {}",db, file);

    //
    let users: Vec<&str> = vec!["zhao", "qian", "shun", "li"];
    let user_info: Vec<String> = join_all(users.into_iter().map(|user: &str|{
        select_db(user)
    })).await;
    println!("user info: {:?}", user_info);
}

//模拟异步链接数据库
async fn common_db() -> String {
    
    // thread::sleep(time::Duration::from_secs(1));//这种方式睡眠会阻塞线程
    sleep(time::Duration::from_secs(1)).await;

    println!("common db");
    String::from("db client")
}

//模拟打开文件
async fn open_file() -> String {
    // thread::sleep(time::Duration::from_secs(1));
    sleep(time::Duration::from_secs(1)).await;
    println!("open file");
    String::from("open file")
}

async fn select_db(name: &str) -> String {
    sleep(time::Duration::from_secs(1)).await;
    String::from(name)
}

async fn hello(){
    println!("hello");
}
