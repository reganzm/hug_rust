use std::{rc::Rc, sync::Arc, thread};

/// 在线程中使用Rc智能指针
fn main() {
    let _t = Rc::new(100);
    Arc::new(10);
    let handler = thread::spawn(move || {
        // 下面打印会报错
        // println!("t:{:?}", t);
    });
    handler.join().unwrap();
}
