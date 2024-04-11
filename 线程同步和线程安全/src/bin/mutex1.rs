use std::sync::Mutex;

fn main() {
    // 使用Mutex创建互斥锁实例
    let m = Mutex::new(10);
    {
        // lock方法获取MutexGuard
        // MutexGuard实现了Deref特征和Drop特征
        let mut lock: std::sync::MutexGuard<i32> = m.lock().unwrap();
        // 通过*解引用获取值，并修改值
        *lock = *lock - 1;
    }// <--- num离开作用域时，自动调用drop释放锁
    println!("m:{:?}", m);
}
