use std::thread;

/// 为裸指针实现Send特征
fn main() {
    let num = 100;
    let p = num as *mut i32;
    let p = SandBox(p);
    let handler = thread::spawn(move || {
        println!("{:?}", p);
    });
    // 下面打印会报错，因为p的所有权已经被转移到子线程中去了
    // println!("{:?}",p);
    handler.join().unwrap();
}

#[derive(Debug)]
struct SandBox(*mut i32);
unsafe impl Send for SandBox {}
