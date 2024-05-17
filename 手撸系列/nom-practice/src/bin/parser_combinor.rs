fn main() {
    let msg = "hello,rust";
    let result = parser_combinor(msg, hello_parser, comma_parser, rust_parser);
    println!("result:{:?}", result);
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

/// 逗号解析器
/// 返回参数:剩余字符串，解析出来的字符串
fn comma_parser(msg: &str) -> Result<(&str, &str), &str> {
    if !msg.is_empty() && msg.starts_with(",") {
        Ok((&msg[",".len()..], ","))
    } else {
        Err("parse error")
    }
}

/// rust解析器解析器
/// 返回参数:剩余字符串，解析出来的字符串
fn rust_parser(msg: &str) -> Result<(&str, &str), &str> {
    if !msg.is_empty() && msg.starts_with("rust") {
        Ok((&msg["rust".len()..], "rust"))
    } else {
        Err("parse error")
    }
}
fn parser_combinor<F, F1, F2>(
    msg: &str,
    mut hello_parser: F,
    mut comma_parser: F1,
    mut rust_parser: F2,
) -> Vec<&str>
where
    F: FnMut(&str) -> Result<(&str, &str), &str>,
    F1:FnMut(&str) -> Result<(&str, &str), &str>,
    F2:FnMut(&str) -> Result<(&str, &str), &str>,
{
    let mut result = vec![];
    if let Ok((input, res)) = hello_parser(msg) {
        result.push(res);
        if let Ok((input, res)) = comma_parser(&input) {
            result.push(res);
            if let Ok((_, res)) = rust_parser(&input) {
                result.push(res);
            }
        }
    } else {
        result.push("parse error");
    }

    result
}
