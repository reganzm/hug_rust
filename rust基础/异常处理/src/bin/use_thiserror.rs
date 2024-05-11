use std::{fs::File, io::Read};

#[warn(dead_code)]
fn main() {
    let r = test_fn();
    match r {
        Ok(s) => println!("{s}"),
        Err(myerror) => println!("error:{:?}", myerror),
    }
}

fn test_fn() -> Result<String, MyError> {
    let data_path = std::env::var("DATA_PATH")?;
    let mut file = File::open(data_path)?;
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);
    println!("content:{}", buf);
    Ok(buf)
}

#[derive(Debug, thiserror::Error)]
enum MyError {
    #[error("没有找到环境变量")]
    EnvironmentNotFound(#[from] std::env::VarError),
    #[error("IO错误")]
    IoError(#[from] std::io::Error),
}
