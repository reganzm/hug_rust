fn main() {
    let msg = "hello,world";
    let result = parser(msg);
    println!("result:{:?}", result);
    // 解析器产生器
    let mut gener = generator("hello");
    let result = gener(msg);
    println!("result:{:?}", result);
}

fn parser(msg: &str) -> Result<(&str, &str), &str> {
    if msg.starts_with("hello") {
        Ok((&msg["hello".len()..], "hello"))
    } else {
        Err("parse error")
    }
}

fn generator<'a>(label: &'a str) -> impl FnMut(&'a str) -> Result<(&'a str, &'a str), &'a str> {
    move |input: &str| {
        if input.starts_with(label) {
            Ok((&input[label.len()..], label))
        } else {
            Err("parse failed")
        }
    }
}
