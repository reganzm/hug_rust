use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

// 服务端地址
const LOCAL: &str = "127.0.0.1:8888";
// 消息长度
const MSG_SIZE: usize = 32;

fn main() {
    // 和客户端建立连接
    let mut client = TcpStream::connect(LOCAL).expect("连接失败");
    // 设置客户端所有的操作都为非阻塞
    client.set_nonblocking(true).expect("设置非阻塞失败");

    let (sender, receiver) = mpsc::channel::<String>();

    // 新建线程，接收服务端消息
    thread::spawn(move || loop {
        // 缓存数组
        let mut buff = vec![0; MSG_SIZE];
        // 从简历的client连接上读取服务端发送的消息
        match client.read_exact(&mut buff) {
            Ok(_) => {
                // 过滤出有效消息
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!(">>{:?}", String::from_utf8(msg).unwrap());
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("读取数据失败，终止连接");
                break;
            }
        }
        // 从channel中读取输入设备输入的消息
        match receiver.try_recv() {
            Ok(msg) => {
                // 将消息读取缓存
                let mut buff = msg.clone().into_bytes();
                // 填充为指定大小的数组
                buff.resize(MSG_SIZE, 0);
                // 将消息通过连接写到服务端
                client.write_all(&buff).expect("发送消息失败");
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }
        // 防止cpu打满
        thread::sleep(Duration::from_millis(100));
    });
    // 打印tips
    println!("使用技巧，输入 :quit 退出连接");
    println!("请发送消息:");
    loop {
        // 缓存数组
        let mut buff = String::new();
        // 从标准输入读取数据到buff
        io::stdin().read_line(&mut buff).expect("读取失败");
        // 转换为字符串
        let msg = buff.trim().to_string();
        // 发送到channel
        // 如果是:quit 或 发送消息到channel失败，退出主线程
        if msg == ":quit" || sender.send(msg).is_err() {
            break;
        }
    }
    println!("再见!");
}
