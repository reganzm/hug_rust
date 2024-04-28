use lazy_static::lazy_static;
use std::{sync::Mutex, thread};

lazy_static! {
    pub static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(Vec::new());
}

pub fn do_a_call() {
    // 验证内存地址
    println!("addr:{:p}", &*ARRAY.lock().unwrap());
    ARRAY.lock().unwrap().push(1);
}

fn main() {
    // 获取单例实例,自定义
    let handle1 = thread::spawn(|| {
        do_a_call();
    });

    let handle2 = thread::spawn(|| {
        do_a_call();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Called {} times", ARRAY.lock().unwrap().len());
}
