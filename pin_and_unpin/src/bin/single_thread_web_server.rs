use std::{fs, io::{Read, Write}, net::{TcpListener, TcpStream}};

// 单线程版本的web服务
fn main() {
    // 监听本地端口 12345
    let listener = TcpListener::bind("127.0.0.1:12345").unwrap();
    // 阻塞等待请求进入
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // 从链接中读取1024个字节的数据
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    // 处理请求
    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK \r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    // 回复内容
    println!("status line:{status_line} file_name:{file_name}");
    let contents = fs::read_to_string(file_name).unwrap();
    let response = format!("{status_line}{contents}");
    println!("response {response}");
    // 写数据到stream
    stream.write_all(response.as_bytes()).unwrap();
    // 刷新
    stream.flush().unwrap();
}
