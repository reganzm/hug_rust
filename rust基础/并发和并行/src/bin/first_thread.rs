use std::{thread, time::Duration};

fn main() {
    thread::spawn(|| {
        // 线程睡眠1000毫秒
        thread::sleep(Duration::from_millis(1000));
        // 打印出当前线程id
        println!(
            "Hello Rust from thread , thread_id:{:?}",
            thread::current().id()
        );
    });
    // 主线程打印出当前线程id
    println!(
        "Hello Rust from main thread , thread_id:{:?}",
        thread::current().id()
    );
}
