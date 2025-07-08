use std::io;
use thiserror::Error as ThisError;

fn main() {
    self_error_type();
    self_error_type1();
    self_error_type2();
}

// 动态错误类型
fn self_error_type2() {
    // 自定义错误类型的定义
    #[derive(ThisError, Debug)]
    pub enum MyError {
        // FailedWithCode 的错误描述，其中 {0} 会被动态地替换为具体的代码值
        #[error("failed with code: {0}")]
        FailedWithCode(i32),
    }
    //
    fn process_data(error_code: i32) -> Result<(), MyError> {
        // 使用动态的 error_code 创建 FailedWithCode 错误
        Err(MyError::FailedWithCode(error_code))
    }
    let result = process_data(404);
    println!("result:{:?}", result);
    let result: Result<(), MyError> = process_data(500);
    println!("result:{:?}", result);
    let result = process_data(403);
    println!("result:{:?}", result);
    let result = process_data(503);
    println!("result:{:?}", result);
}

/// 自定义错误类型
fn self_error_type() {
    // 自定义错误类型的定义
    #[derive(ThisError, Debug)]
    pub enum MyError {
        // DataNotFound 错误的描述
        #[error("data not found")]
        DataNotFound,
        // InvalidInput 错误的描述
        #[error("invalid input")]
        InvalidInput,
    }
    // 示例函数，展示如何使用自定义错误
    fn search_data(query: &str) -> Result<(), MyError> {
        if query.is_empty() {
            // 当查询为空时，返回 InvalidInput 错误
            return Err(MyError::InvalidInput);
        }
        // 这里省略了实际的数据查询逻辑
        // ...
        // 数据未找到时返回 DataNotFound 错误
        Err(MyError::DataNotFound)
    }
    let result = search_data("");
    println!("result:{:#?}", result);
}

/// 自定义错误类型 - 嵌套错误
fn self_error_type1() {
    // 自定义错误类型的定义
    #[derive(ThisError, Debug)]
    pub enum MyError {
        // IoError 错误的描述，它包含一个嵌套的 io::Error
        #[error("I/O error occurred")]
        IoError(#[from] io::Error),
    }
    // 示例函数，展示如何使用嵌套的错误
    fn read_file(file_path: &str) -> Result<String, MyError> {
        // 如果 fs::read_to_string 返回错误，我们使用 MyError::from 将它转换为 MyError::IoError
        std::fs::read_to_string(file_path).map_err(MyError::from)
    }
    let result = read_file("./test.rs");
    println!("result:{:?}", result);
}
