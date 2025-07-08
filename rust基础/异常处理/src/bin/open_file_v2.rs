pub mod use_thiserror;

use std::{
    fs::File,
    io::{Error, ErrorKind, Read, Write},
};

fn main() -> Result<(), Error> {
    let mut file = File::open("./src/bin/result.txt").expect("打开文件失败");
    // 写入数据
    let _ = file.write_all("2024加油!".as_bytes());
    // 读取数据
    let mut file = File::open("./src/bin/result.txt").unwrap();
    let mut content: String = String::new();
    let _ = file.read_to_string(&mut content);
    println!("{content}");

    // ?运算符向上层抛出异常
    let content = read_from_file();
    match content {
        Ok(c) => println!("content:{c}"),
        Err(e) => println!("error:{:?}", e),
    }

    Ok(())
}

fn read_from_file() -> Result<String, Error> {
    // 文件不存在，?直接返回Error
    let mut file = File::open("./src/bin/result1.txt")?;
    let mut result = String::new();
    let _ = file.read_to_string(&mut result);
    Ok(result)
}
