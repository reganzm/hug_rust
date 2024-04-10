use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
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
    // 调用子线程返回的句柄上的join方法
    // 会使父线程阻塞，直到它等待的子线程结束
    handle.join().unwrap();
}
