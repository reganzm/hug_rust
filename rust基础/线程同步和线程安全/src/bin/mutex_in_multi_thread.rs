use std::{
    sync::{Arc, Mutex},
    thread,
};

/// 多线程中使用Mutex
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    // 新建50个线程，每个线程对counter做加1操作
    for _ in 1..=50 {
        // 使用共享所有权且线程安全的智能指针Arc，如果使用Rc会报错
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            // 获得锁
            let mut num = counter.lock().unwrap();
            // 对锁住的值+1
            *num += 1;
        });
        handles.push(handle);
    }
    // 等待所有线程结束
    for handle in handles {
        handle.join().unwrap();
    }
    // 打印counter中的数值
    println!("counter:{}", *counter.lock().unwrap()); // 结果为50
}
