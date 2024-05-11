use std::{rc::Rc, sync::Arc, thread, time::Duration};

fn main() {
    // Rc
    let count1: Rc<i32> = Rc::new(10);
    // 增加引用计数器
    let count2: Rc<i32> = count1.clone();
    println!("count1:{count1} count2:{count2}");
    println!("reference count:{} ", Rc::strong_count(&count1));
    {
        let count3 = count2.clone();
        println!("reference count:{} ", Rc::strong_count(&count3));
    }
    println!("reference count:{} ", Rc::strong_count(&count1));

    // Arc
    let s = Arc::new(10);
    let mut handles = Vec::new();
    for _i in 0..10 {
        // 增加引用计数器
        let tmp = Arc::clone(&s);
        let handle = thread::spawn(move || {
            println!("atomic reference count:{}", Arc::strong_count(&tmp));
            thread::sleep(Duration::from_secs(2));
        });
        handles.push(handle);
    }
    println!("atomic reference count:{}", Arc::strong_count(&s));
    thread::sleep(Duration::from_secs(2));
    println!("atomic reference count:{}", Arc::strong_count(&s));
}
