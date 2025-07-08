fn main() {
    let msg = "hello,rust";
    let mut parse_generator = hello_parser_generator("hello");
    let hello1 = parse_generator(msg);
    println!("parse result:{:?}", hello1);
}

/// hello解析器生成器
/// 返回参数:剩余字符串，解析出来的字符串
fn hello_parser_generator<'a>(
    tag: &'a str,
) -> impl FnMut(&'a str) -> Result<(&'a str, &'a str), &'a str> {
    move |msg| {
        if !msg.is_empty() && msg.starts_with(tag) {
            Ok((&msg[tag.len()..], tag))
        } else {
            Err("parse error")
        }
    }
}
