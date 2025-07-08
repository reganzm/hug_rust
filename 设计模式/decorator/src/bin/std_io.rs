use std::io::{BufReader, Cursor, Read};

fn main() {
    // 标准库中的io模块使用了装饰器模式
    let mut input = BufReader::new(Cursor::new("study rust"));
    let mut buf = [0; 20];
    input.read(&mut buf).ok();
    for byte in buf {
        print!("{}", char::from(byte));
    }
}
