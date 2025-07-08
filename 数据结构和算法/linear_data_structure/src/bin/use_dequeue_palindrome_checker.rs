//!使用双端队列进行回文检测

use dequeue::Dequeue;
mod dequeue;

// 检测字符串是否是回文字符串
fn check_palindrome(input: &str) -> bool {
    let mut dq = Dequeue::new(input.len());
    for c in input.chars() {
        let _ = dq.add_rear(c);
    }

    let mut is_match = true;
    while dq.len() > 1 && is_match {
        let head = dq.remove_front();
        let tail = dq.remove_rear();

        if head != tail {
            is_match = false;
        }
    }
    is_match
}

fn main() {
    let a = "i love you";
    println!("input:'{a}' is palindrome:{}", check_palindrome(a));
    let a = "i love i";
    println!("input:'{a}' is palindrome:{}", check_palindrome(a));
    let a = "1 2 3 4 5 6 7 8 9 0 0 9 8 7 6 5 4 3 2 1";
    println!("input:'{a}' is palindrome:{}", check_palindrome(a));
}
