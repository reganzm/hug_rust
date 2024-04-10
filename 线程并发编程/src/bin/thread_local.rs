use std::cell::Cell;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use thread_local::ThreadLocal;
fn main() {
    // 使用标准库中的thread_local!宏创建线程局部变量
    // 使用Cell提供内部可变性
    thread_local! { static INIT:Cell<i32> = Cell::new(100)};
    INIT.with(|i| {
        println!("main thread i={}", i.get());
        i.set(i.get() - 1);
        println!("main thread i={}", i.get());
    });
    println!("main thread INIT VALUE:{}", INIT.get());

    // 在线程中使用线程局部变量
    let handle = thread::spawn(move || {
        INIT.with(|i| {
            println!("child thread i={}", i.get());
            i.set(i.get() - 100);
            println!("child thread i={}", i.get());
        });
    });
    handle.join().unwrap();
    println!("main thread INIT VALUE:{}", INIT.get());

    // 在结构体中使用线程局部变量
    struct Task;
    impl Task {
        thread_local! { static INIT:Cell<i32> = Cell::new(100)}

        fn set_init(&self, num: i32) {
            INIT.set(num);
        }
        fn get_init(&self) -> i32 {
            INIT.get()
        }
    }
    let task = Task;
    task.set_init(1000);
    println!("main thread struct INIT:{}", task.get_init());
    let handle = thread::spawn(move || {
        task.set_init(99);
        println!("child thread struct INIT:{}", task.get_init());
    });
    handle.join().unwrap();
    // 下面编译错误，因为task所有权已经移动到了子线程中
    // println!("main thread struct INIT:{}", task.get_init());

    // 使用第三方库thread_local创建本地变量
    // Arc在多个线程间安全的共享所有权
    let tl = Arc::new(ThreadLocal::new());
    // 主线程中初始化为99
    tl.get_or(|| Cell::new(99));
    let mut handles = vec![];
    // 创建多个线程
    for _i in 0..=4 {
        let tl_i = tl.clone();
        let handle = thread::spawn(move || {
            let cell = tl_i.get_or(|| Cell::new(100));
            cell.set(cell.get() + 1);
        });
        handles.push(handle);
    }

    println!("引用计数:{}", Arc::strong_count(&tl));
    for handle in handles {
        handle.join().unwrap();
    }
    println!("引用计数:{}", Arc::strong_count(&tl));
    println!("main thread tl:{}", tl.get().unwrap().get());
}
