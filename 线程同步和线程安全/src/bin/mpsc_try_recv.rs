use std::{sync::mpsc, thread, time::Duration};
fn main() {
    // 创建一个通道，返回元组(发送者，接收者)
    let (sender, receiver) = mpsc::channel();
    // 创建子线程向主线程发送消息
    thread::spawn(move || {
        // 使用move将sender的所有权移动到子线程中
        // send方法返回Result，因为能发送失败
        // 这里直接unwrap取出Ok
        let msg = String::from("加油2024");
        let result = sender.send(msg);
        // 编译报错，msg没有实现COPY特征，所有权被转移
        //println!("msg ：{msg}");
        if let Ok(()) = result {
            println!("子线程发送消息成功");
        } else {
            println!("子线程发送消息失败");
        }
    });
    // 在主线程中接收消息
    // try_recv不会阻塞当前线程，没有消息立即返回一个错误
    thread::sleep(Duration::from_millis(1));
    let msg = receiver.try_recv();
    if let Ok(msg) = msg {
        println!("接收到子线程发送的消息:{msg}");
    } else {
        println!("没有收到消息...{:?}", msg);
    }
    // if let Ok(msg) = msg {
    //     println!("接收到子线程发送的消息:{msg}");
    // } else {
    //     println!("没有收到消息...{:?}", msg);
    // }
}
