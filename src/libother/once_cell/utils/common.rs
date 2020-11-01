use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;

/*
    OnceCell 可能存储任意非Copy类型，可以最多分配一次，并提供对存储内容的直接访问
*/
static GLOBAL_DATA: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    Mutex::new(m)
});

pub fn run(){
    println!("{:?}", GLOBAL_DATA.lock().unwrap());

    {
        let mut m = GLOBAL_DATA.lock().unwrap();
        m.insert(1, "tao".to_string());
    }


    println!("{:?}", GLOBAL_DATA.lock().unwrap());
}