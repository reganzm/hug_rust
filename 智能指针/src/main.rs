use std::{mem::size_of_val, rc::Rc};
fn main() {
    let a: &str = "Rust";
    // 分别打印出&str的大小、切片变量地址、切片对应值的地址
    // 和切片对应值的大小，它和切片中的len属性大小一致
    println!(
        "size of a:{} , address of a:{:p}, value address of a:{:p} , size of data:{}",
        a.len(),
        &a,
        &(*a),
        size_of_val(&(*a))
    );

    // 指针(引用)
    let a: u8 = 8;
    let b: &u8 = &a;
    let c: *const u32;

    let a: i32 = 8;
    let b: &i32 = &a;
    // 裸指针
    let a: i32 = 8;
    let b: *const i32 = &a;
    unsafe {
        println!("a:{a} b:{}", *b);
    }
}
