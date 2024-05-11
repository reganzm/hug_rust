use std::{sync::Arc, thread};
/// 为裸指针实现send和sync特征
fn main() {
    let num = 123;
    let sand_box = Arc::new(SandBox(num as *mut i32));
    let sand_box_clone = sand_box.clone();
    let handle = thread::spawn(move || {
        println!("sand box value:{:?}", sand_box_clone);
    });
    handle.join().unwrap();
    println!("sand box value:{:?}", sand_box);
}

#[derive(Debug)]
struct SandBox(*mut i32);
unsafe impl Send for SandBox {}
unsafe impl Sync for SandBox {}
