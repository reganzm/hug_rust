use std::{sync::mpsc, thread};
fn main() {
    // 创建一个通道，返回元组(发送者，接收者)
    let (sender, receiver) = mpsc::channel();
    // 克隆一个生产者
    let sender1 = sender.clone();
    // 生产者1
    thread::spawn(move || {
        let thread_id = thread::current().id();
        let msg = format!("加油2024 来自线程:{:?}", thread_id);
        let _ = sender.send(msg);
    });
    // 生产者2
    thread::spawn(move || {
        let thread_id = thread::current().id();
        let msg = format!("加油2024 来自线程:{:?}", thread_id);
        let _ = sender1.send(msg);
    });
    // 在主线程中使用for循环接收消息
    for msg in receiver {
        println!("收到消息：{msg}");
    }
}
