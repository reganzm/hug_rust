use std::{sync::Arc, thread, time::Duration};
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    // 初始化信号量为3
    let semaphore: Arc<Semaphore> = Arc::new(Semaphore::new(3));
    let mut join_handles = Vec::new();
    // 获取一个信号量许可，信号量减1
    let permit = semaphore.clone().acquire_owned().await.unwrap();
    println!("start {:?}", permit);
    //drop(permit);
    for i in 0..5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(async move {
            println!("i:{i}");
            thread::sleep(Duration::from_secs(10));
            println!("inner {:?}", permit);
            drop(permit);
        }));
    }

    for handle in join_handles {
        handle.await.unwrap();
    }
    let permit = semaphore.clone().acquire_owned().await.unwrap();
    println!("end {:?}", permit);
}
