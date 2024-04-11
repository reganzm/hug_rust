use std::sync::RwLock;

/// 读写锁
fn main() {
    // 实例化读写锁
    let lock = RwLock::new(100);
    // 同一时间允许多个读
    {
        let num1 = lock.read().unwrap();
        let num2 = lock.read().unwrap();
        assert_eq!(*num1, 100);
        assert_eq!(*num2, 100)
    } //<-------------读锁在此离开作用域，释放读锁
      // 同一时间只运行一个写
    {
        let mut num3 = lock.write().unwrap();
        *num3 += 1;
        assert_eq!(*num3, 101);
    } //<-------------写锁在此离开作用域，释放写锁
}
