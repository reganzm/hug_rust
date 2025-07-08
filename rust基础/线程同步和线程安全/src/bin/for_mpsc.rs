use std::{sync::mpsc, thread, time::Duration};

fn main() {
    // 创建一个通道，返回元组(发送者，接收者)
    let (sender, receiver) = mpsc::channel();
    // 创建子线程向主线程发送消息
    thread::spawn(move || {
        for i in 1..=10 {
            // 使用move将sender的所有权移动到子线程中
            // send方法返回Result，因为能发送失败
            // 这里直接unwrap取出Ok
            let msg = String::from(format!("Message_{i}"));
            let result = sender.send(msg);
            // 编译报错，msg没有实现COPY特征，所有权被转移
            //println!("msg ：{msg}");
            if let Ok(()) = result {
                println!("子线程发送消息成功");
            } else {
                println!("子线程发送消息失败");
            }
            thread::sleep(Duration::from_millis(1000));
        }
    });
    // 在主线程中使用for循环接收消息
    for msg in receiver {
        println!("收到消息：{msg}");
    }
}
