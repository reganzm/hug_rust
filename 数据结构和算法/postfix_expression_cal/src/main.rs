//! 后缀表达式计算

use std::collections::HashMap;

use par_checker::par_checher;
use stack::Stack;
mod par_checker;
mod stack;

/// 中缀表达式转换为后缀表达式
fn infix_to_postfix(infix: &str) -> Option<String> {
    // 1.括号检测
    if !par_checher(infix) {
        return None;
    }

    // 2.设置运算符优先级
    let mut prec = HashMap::new();
    // 括号优先级最低
    prec.insert("(", 1);
    prec.insert(")", 1);
    prec.insert("+", 2);
    prec.insert("-", 2);
    // 乘除优先级最高
    prec.insert("*", 3);
    prec.insert("/", 3);

    // 使用栈保存运算符，使用postfix保存后缀表达式
    let mut ops_stack = Stack::new();
    let mut postfix = Vec::new();

    // 遍历表达式中的字符
    for token in infix.split_whitespace() {
        // 将数字0~9放入postfix
        if "0" <= token && token <= "9" {
            postfix.push(token);
        }
        // 遇到"("，入栈
        else if "(" == token {
            ops_stack.push(token);
        }
        // 遇到")",运算符出栈并追加到后缀表达式列表
        else if ")" == token {
            let mut top = ops_stack.pop().unwrap();
            // "("前的所有运算符都出栈
            while top != "(" {
                postfix.push(top);
                top = ops_stack.pop().unwrap();
            }
        } else {
            // 遇到+-*/运算符，先比较运算符的优先级并添加到postfix列表中
            while (!ops_stack.is_empty()) && prec[ops_stack.peek().unwrap()] >= prec[token] {
                postfix.push(ops_stack.pop().unwrap());
            }
            ops_stack.push(token);
        }
    }
    // 将剩下的运算符出栈追加到后缀表达式列表
    while !ops_stack.is_empty() {
        postfix.push(ops_stack.pop().unwrap());
    }

    //拼装成字符串
    let mut postfix_str = String::new();
    for c in postfix {
        postfix_str += &c.to_string();
        postfix_str += &" ".to_string();
    }
    Some(postfix_str)
}

// 通过后缀表达式计算值
fn postfix_eval(postfix: &str) -> Option<i32> {
    // 后缀表达式最少需要5个字符，一个运算符两个操作数及两个空格
    if postfix.len() < 5 {
        return None;
    };
    let mut ops_stack = Stack::new();
    for token in postfix.split_ascii_whitespace() {
        // 先出栈的是第二个操作数，顺序对与-和/操作符很重要
        if token >= "0" && token <= "9" {
            ops_stack.push(token.parse::<i32>().unwrap());
        } else {
            let op_num2 = ops_stack.pop().unwrap();
            let op_num1 = ops_stack.pop().unwrap();
            let res = do_calculation(token, op_num1, op_num2);
            ops_stack.push(res);
        }
    }
    Some(ops_stack.pop().unwrap())
}

fn do_calculation(token: &str, op_num1: i32, op_num2: i32) -> i32 {
    match token {
        "+" => op_num1 + op_num2,
        "-" => op_num1 - op_num2,
        "*" => op_num1 * op_num2,
        "/" => {
            if op_num2 == 0 {
                panic!("除零操作，非法！");
            }
            op_num1 / op_num2
        }
        _ => {
            panic!("非法字符");
        }
    }
}

fn main() {
    // 表达式：( 1 + 5 ) * 3 + ( 15 - 7 ) / 2
    // 转换为后缀表达式
    let infix = "( 1 + 5 ) * 3 + ( 15 - 7 ) / 2";
    let postfix = infix_to_postfix(infix);
    let result = postfix_eval(postfix.clone().unwrap().as_str());
    println!(
        "infix:{infix} \r\npostfix:{:?}\r\nresult:{:?}",
        postfix.unwrap(),
        result.unwrap()
    );
}
