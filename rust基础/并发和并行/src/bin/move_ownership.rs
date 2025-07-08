use std::{thread, time::Duration};
fn main() {
    let hello = String::from("Hello Rust");
    let handle = thread::spawn(move || {
        // 线程睡眠1000毫秒
        thread::sleep(Duration::from_millis(1000));
        // 打印出当前线程id
        println!("{} , thread_id:{:?}", hello, thread::current().id());
    });
    handle.join().unwrap();
}
