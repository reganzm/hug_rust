use std::{sync::mpsc, thread};

fn main() {
    // 创建一个通道，返回元组(发送者，接收者)
    let (sender, receiver) = mpsc::channel();
    // 创建子线程向主线程发送消息
    thread::spawn(move || {
        // 使用move将sender的所有权移动到子线程中
        // send方法返回Result，因为能发送失败
        // 这里直接unwrap取出Ok
        let result = sender.send("加油2024");
        if let Ok(()) = result {
            println!("子线程发送消息成功");
        } else {
            println!("子线程发送消息失败");
        }
    });
    // 在主线程中接收消息
    // recv会阻塞当前线程，直到有消息返回或者channel被关闭为止
    let msg = receiver.recv();
    if let Ok(msg) = msg {
        println!("接收到子线程发送的消息:{msg}");
    } else {
        println!("接收消息失败...");
    }
}
