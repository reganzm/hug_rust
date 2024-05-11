use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// 监听地址
const LOCAL: &str = "127.0.0.1:8888";
// 消息大小
const MSG_SIZE: usize = 32;

fn main() {
    // 建立监听
    let server = TcpListener::bind(LOCAL).expect("绑定失败");
    // 设置server的accept方法为非阻塞
    server.set_nonblocking(true).expect("设置失败");
    // 接入进来的客户端集合
    let mut clients = vec![];
    // 基于channel消息传递机制，解耦建立监听和响应客户端这两件事
    let (sender, receiver) = mpsc::channel::<String>();
    loop {
        // 建立客户端连接
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);
            let sender = sender.clone();
            clients.push(socket.try_clone().expect("克隆失败"));
            // 新建线程，接收client消息，并将接收到的消息发送到channel
            thread::spawn(move || loop {
                // 初始化buff，默认值为0
                let mut buff = vec![0; MSG_SIZE];
                // 读取客户端发送的信息，存入指定长度的buff中
                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        // 过滤出有效的字节
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        // 将有效字节转换成字符串
                        let msg = String::from_utf8(msg).expect("非法字符");
                        // 打印收到的消息
                        println!("{}-: {:?}", addr, msg);
                        // 将接收到的消息发送到channel
                        sender.send(msg).expect("发送消息到channel失败");
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("关闭连接: {}", addr);
                        break;
                    }
                }
                // 防止CPU打满
                thread::sleep(Duration::from_millis(100));
            });
        }
        // channel中有消息
        if let Ok(msg) = receiver.try_recv() {
            // 向所有客户端同步刚刚收到的消息
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let mut buff = msg.clone().into_bytes();
                    buff.resize(MSG_SIZE, 0);
                    // 向每个客户端发送消息，并返回状态ok的客户端
                    client.write_all(&buff).map(|_| client).ok()
                })
                .collect::<Vec<_>>();
        }
        // 防止CPU打满
        thread::sleep(Duration::from_millis(100));
    }
}
