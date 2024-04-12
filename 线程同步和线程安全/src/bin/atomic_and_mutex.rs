use std::{sync::atomic::AtomicU32, thread::{self, JoinHandle}, time::Instant};

// 使用Atomic和Mutex做比较
fn main(){
    // 原子类型作为全局变量
    const N_TIMES:u32 = 1000000;
    const CAP:u8 = 100;
    static RESULT:AtomicU32 = AtomicU32::new(0);

    fn add_n_times(n:u32)->JoinHandle<()>{
        thread::spawn(move||{
            for i in 1..=n{
                TOTAL
            }
        })
    }

    
    let start_time = Instant::now();
    let mut threads = Vec::with_capacity(cap);
    for i in 1..=100{
       
    }


}