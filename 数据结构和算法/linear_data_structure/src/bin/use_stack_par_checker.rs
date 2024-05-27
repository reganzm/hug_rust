//! 使用栈进行"括号"匹配
//! 这里的"括号"包括[]、{}、()

use std::vec;

use crate::stack::Stack;

mod stack;
/// 判断是否匹配
fn matcher(left: char, right: char) -> bool {
    let lefts = "([{";
    let rights = ")]}";
    lefts.find(left) == rights.find(right)
}
// 使用栈判断括号是否匹配
fn par_checher(input: &str) -> bool {
    if input.is_empty() {
        return false;
    } else {
        let mut chars = vec![];
        for c in input.chars() {
            chars.push(c);
        }

        // 记录chars的下标
        let mut index = 0;
        // 标记是否匹配
        let mut balance = true;
        let mut stack = Stack::new();
        // 借助栈匹配字符，直至匹配完毕或者出现不匹配
        while index < chars.len() && balance {
            let i = chars[index];
            // 如果i='['或i='{'或i='('，则入栈
            if i == '(' || i == '[' || i == '{' {
                stack.push(i);
            }

            // 如果i=']' 或i='}'或i=')',判断是否平衡
            if i == ')' || i == '}' || i == ']' {
                if stack.is_empty() {
                    balance = false;
                } else {
                    let top = stack.pop().unwrap();
                    if !matcher(top, i) {
                        balance = false;
                    }
                }
            }
            index += 1;
        }
        balance && stack.is_empty()
    }
}

fn main() {
    let a = "(3+2)-5*{5*7}+[{()}]";
    println!("{a} is balance:{}", par_checher(a));
    let a = "(3+2)-5*{5*7}+[{(}}]";
    println!("{a} is balance:{}", par_checher(a));
    let a = "(([{([1+1-2+232*({[]})])}]))";
    println!("{a} is balance:{}", par_checher(a));
}
