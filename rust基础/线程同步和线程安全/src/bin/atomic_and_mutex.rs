use std::ops::Sub;
use std::{
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Instant,
};

// 使用Atomic和Mutex做比较
fn main() {
    // 原子类型作为全局变量
    const N_TIMES: u32 = 1000000;
    const CAP: usize = 100;
    static RESULT: AtomicU32 = AtomicU32::new(0);

    fn add_n_times(n: u32) -> JoinHandle<()> {
        thread::spawn(move || {
            for _ in 1..=n {
                RESULT.fetch_add(1, Ordering::Relaxed);
            }
        })
    }
    fn add_n_times_2(n: u32, lock: Arc<Mutex<i32>>) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut num = lock.lock().unwrap();
            for _ in 1..=n {
                *num += 1;
            }
        })
    }

    let start_time = Instant::now();
    let mut handles = Vec::with_capacity(CAP);
    for _ in 1..=CAP {
        handles.push(add_n_times(N_TIMES));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", RESULT.load(Ordering::Relaxed));
    println!("原子变量耗时:{:?}", Instant::now().sub(start_time));
    // 使用Mutex
    let start_time = Instant::now();
    let mutex = Arc::new(Mutex::new(0));

    let mut handles = Vec::with_capacity(CAP);
    for _ in 1..=CAP {
        let m = mutex.clone();
        handles.push(add_n_times_2(N_TIMES, m));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", *mutex);
    println!("Mutex耗时:{:?}", Instant::now().sub(start_time));

    // 内部可变性
    show_change();
}

// Atomic和Mutex的值具有内部可变性
fn show_change() {
    let lock1 = Mutex::new(10);
    // 修改内部值
    *lock1.lock().unwrap() += 10;
    let lock2 = AtomicU32::new(10);
    let before_value = lock2.fetch_add(10, Ordering::Relaxed);
    println!(
        "before value:{before_value} current value:{:?}",
        lock2.load(Ordering::Relaxed)
    );
}
