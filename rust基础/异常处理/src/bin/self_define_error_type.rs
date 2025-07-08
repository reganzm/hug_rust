use std::{
    fmt::Debug,
    fs::File,
    io::{self, ErrorKind},
};

fn main() {
    fn get_a_err1() -> Result<(), MyError> {
        Err(MyError {
            code: 404,
            message: String::from("找不到页面"),
        })
    }
    fn get_a_err2() -> Result<(), MyError> {
        Err(MyError {
            code: 500,
            message: String::from("系统错误"),
        })
    }

    println!("{:?}", get_a_err1());
    println!("{:?}", get_a_err2());

    let r = use_my_error_unified_process();
    match r {
        Ok(_) => println!("ok"),
        Err(myerror) => println!("{:?}", myerror),
    }
}

fn use_my_error_unified_process() -> Result<(), MyError> {
    // 使用MyError进行错误类型归一化
    // open方法抛出的io::Error将自动转换为MyError
    // 因为MyError实现了From特征
    let _f = File::open("./src/bin/hello.txt")?;
    Ok(())
}

struct MyError {
    code: usize,
    message: String,
}

// 实现Debug特征
impl Debug for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error {{code:{},message:{}}}", self.code, self.message)
    }
}

// 为io::Error实现From接口
impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            ErrorKind::NotFound => MyError {
                code: 100001,
                message: error.to_string(),
            },
            ErrorKind::PermissionDenied => MyError {
                code: 100002,
                message: error.to_string(),
            },
            _ => MyError {
                code: 100003,
                message: error.to_string(),
            },
        }
    }
}
