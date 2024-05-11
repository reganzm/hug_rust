use std::{fs,time::Duration};
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::task;
use futures::stream::StreamExt;
use async_std::prelude::*;


#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:12345").await.unwrap();
    listener
    .incoming()
    .for_each_concurrent(None, |tcpstream| async move {
        let tcpstream = tcpstream.unwrap();
        handle_connection(tcpstream).await;
    })
    .await;
}

async fn handle_connection(mut stream:TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).await.unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "sleep.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{status_line}{contents}");
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();

}