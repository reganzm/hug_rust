use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

/// mutex和condvar一起控制线程同步
fn main() {
    // 在线程之间共享所有权
    let lock = Arc::new(Mutex::new(false));
    let con = Arc::new(Condvar::new());

    let lock_1 = lock.clone();
    let con_1 = con.clone();

    let handle = thread::spawn(move || {
        let mut locked = lock_1.lock().unwrap();
        let mut counter = 0;
        while counter < 3 {
            // CondVar等待条件变为true，此时本线程阻塞
            // CondVar的wait方法会释放Mutex上的锁，其它线程有机会获得锁
            // 由其它的线程通知唤醒
            locked = con_1.wait(locked).unwrap();
            // 唤醒后立即将锁置为false
            *locked = false;
            counter += 1;
            println!("子线程counter:{counter}");
        }
    });

    let mut counter = 0;
    loop {
        thread::sleep(Duration::from_secs(1));
        // 获取锁并置为true
        *lock.lock().unwrap() = true;
        counter += 1;
        if counter > 3 {
            break;
        }
        println!("主线程counter:{counter}");
        // 通过条件通知其它线程
        con.notify_one();
    }

    handle.join().unwrap();
    println!("lock:{:?}", lock);
}
