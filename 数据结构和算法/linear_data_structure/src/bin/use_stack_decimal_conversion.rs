//!使用栈进行16进制以内的任意进制转换

use stack::Stack;

mod stack;

fn decimal_conversion(mut origin_num: u32, base: u32) -> String {
    let digits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    // 使用转辗相除法
    let mut stack = Stack::new();

    // 余数入栈
    while origin_num > 0 {
        let rem = origin_num % base;
        stack.push(rem);
        origin_num /= base;
    }

    // 出栈并拼装成字符串
    let mut result = "".to_string();
    while !stack.is_empty() {
        let rem = stack.pop().unwrap() as usize;
        result += &digits[rem].to_string();
    }
    result
}

fn main() {
    let num = 1024;
    println!("{num} 转换为2进制:{}", decimal_conversion(num, 2));
    println!("{num} 转换为3进制:{}", decimal_conversion(num, 3));
    println!("{num} 转换为4进制:{}", decimal_conversion(num, 4));
    println!("{num} 转换为5进制:{}", decimal_conversion(num, 5));
    println!("{num} 转换为6进制:{}", decimal_conversion(num, 6));
    println!("{num} 转换为7进制:{}", decimal_conversion(num, 7));
    println!("{num} 转换为8进制:{}", decimal_conversion(num, 8));
    println!("{num} 转换为9进制:{}", decimal_conversion(num, 9));
    println!("{num} 转换为10进制:{}", decimal_conversion(num, 10));
    println!("{num} 转换为11进制:{}", decimal_conversion(num, 11));
    println!("{num} 转换为12进制:{}", decimal_conversion(num, 12));
    println!("{num} 转换为13进制:{}", decimal_conversion(num, 13));
    println!("{num} 转换为14进制:{}", decimal_conversion(num, 14));
    println!("{num} 转换为15进制:{}", decimal_conversion(num, 15));
    println!("{num} 转换为16进制:{}", decimal_conversion(num, 16));
}
