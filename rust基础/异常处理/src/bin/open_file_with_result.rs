use std::{
    fs::File,
    io::{Error, ErrorKind, Read, Write},
};

fn main() -> Result<(), Error> {
    let file = File::open("./src/bin/result.txt");
    // file是Result<File,Error>类型。现在做模式匹配
    let mut file = match file {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("./src/bin/result.txt") {
                Ok(f) => f,
                Err(e) => panic!("文件不存在且创建文件失败:{e}"),
            },
            ErrorKind::PermissionDenied => panic!("没有打开权限"),
            // 其它异常
            _ => panic!("遇到未知异常"),
        },
    };
    // 写入数据
    let _ = file.write_all("2024加油!".as_bytes());
    // 读取数据
    let mut file = File::open("./src/bin/result.txt")?;
    let mut content: String = String::new();
    let _ = file.read_to_string(&mut content);
    println!("{content}");
    Ok(())
}
