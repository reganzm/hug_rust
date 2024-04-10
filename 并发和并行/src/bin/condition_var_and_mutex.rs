use std::borrow::Borrow;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();
    println!("main 锁定的值为:{:?}", pair.0.lock().unwrap());

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("子线程修改锁定的值为true");
        *started = true;
        // 通知等在该条件上的线程
        cvar.notify_one();
    });
    //thread::sleep(Duration::from_millis(10));
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    // main线程等待锁里面的值变为true
    println!("main wait..");
    started = cvar.wait(started).unwrap();
    println!("main 线程 {started}");
}
