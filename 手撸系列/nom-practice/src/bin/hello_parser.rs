fn main() {
    let msg = "hello,rust";
    let hello = hello_parser(msg);
    println!("parse result:{:?}", hello);
}

/// hello解析器
/// 返回参数:剩余字符串，解析出来的字符串
fn hello_parser(msg: &str) -> Result<(&str, &str), &str> {
    if !msg.is_empty() && msg.starts_with("hello") {
        Ok((&msg["hello".len()..], "hello"))
    } else {
        Err("parse error")
    }
}
