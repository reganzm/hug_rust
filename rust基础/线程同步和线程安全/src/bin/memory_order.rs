use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::{hint, thread};

fn main() {
    let spinlock = Arc::new(AtomicUsize::new(1));
    let spinlock_clone = Arc::clone(&spinlock);
    let thread = thread::spawn(move || {
        spinlock_clone.store(100, Ordering::Release); // 内存屏障1↑
    });
    // 等待其它线程释放锁
    while spinlock.load(Ordering::Acquire) != 100 {
        // 内存屏障2↓
        // 自旋
        hint::spin_loop();
    }
    thread.join().unwrap();
    println!("spin lock:{:?}", spinlock);
}
