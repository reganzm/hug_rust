

use std::future;
/// 判断一个数是否为奇数
fn odd_number(a: usize) -> bool {
    let mut result = true;
    if a % 2 == 0 {
        result = false
    };
    result
}
/// 判断一个数是否是偶数
fn even_number(a: usize) -> bool {
    !odd_number(a)
}

/// 判断一个数是否能被5整除
/// `````
/// let a = 125_usize;
/// let r = auto_test::devide_by_5(a);
/// assert_eq!(r,true);
/// `````
pub fn devide_by_5(a: usize) -> bool {
    if a % 5 == 0 {
        true
    } else {
        false
    }
}

/// 通过右移操作除以2
pub fn devide_2(a:isize)->isize{
    a >> 1
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_odd_number() {
        let num = 125;
        assert!(odd_number(num));
    }

    #[test]
    fn test_even_number() {
        let num = 124;
        assert!(even_number(num));
    }

    // 通过#[ignore忽略该测试]
    #[test]
    #[ignore]
    fn test_even_number2() {
        let num = 125;
        assert!(even_number(num), "你输入的数:{}不是偶数", num);
    }

    // 使用pretty_assertion中的assert_eq断言
    #[test]
    fn prettry_assertion(){
        use pretty_assertions::assert_eq;
        assert_eq!(1+1,3);
    }



}
