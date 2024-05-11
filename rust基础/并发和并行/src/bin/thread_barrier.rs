use std::{
    sync::{Arc, Barrier},
    thread::{self, JoinHandle},
};

fn main() {
    let cap: usize = 11;
    let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(cap);
    // 通过Arc在堆上创建多线程安全且共享所有权的Barrier，屏障初始为11个线程
    let barrier: Arc<Barrier> = Arc::new(Barrier::new(cap));
    for i in 0..=10 {
        // clone增加引用计数
        let b: Arc<Barrier> = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("thread {i} before wait");
            // 屏障等待
            b.wait();
            println!("thread {i} after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
