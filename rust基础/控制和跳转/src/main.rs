use std::{collections::HashMap, path::PrefixComponent};

fn main() {
    // 表达式和语句
    // 函数func是语句
    fn func() -> i32 {
        // 123是表达式，表达式返回
        123
    }

    // 整句是语句，name = "regan".to_string()是表达式
    let name = "regan".to_string();
    // age=123是表达式
    let age = 123;
    // 整句是语句，args=[1,2,3]是表达式
    let args = [1, 2, 3, 4, 5];

    // if else
    let num = 50;
    if num >= 60 {
        println!("num大于等于60");
    } else if num <= 30 {
        println!("num小于30");
    } else {
        println!("num大于30且小于60");
    }

    // let if ; if let  ; match
    // let if，将满足条件的返回值绑定到result变量
    let result = if num >= 50 { true } else { false };
    println!("let if:{result}");

    // if let模式匹配:pattern = expr
    // 匹配到之后将运行{}中的代码，否则运行else中的代码
    if let 50 = num {
        println!("num==50")
    } else {
        println!("num!=50");
    }

    // match模式匹配
    match num {
        20 => println!("num=20"),
        30 => println!("num=30"),
        50 => println!("num=50"),
        // 使用_表示其它的选项
        _ => println!("num not in 20 30 50"),
    }
    // 单分支多模式匹配
    match num {
        // pat|pat 单分支多模式匹配
        20 | 30 => println!("num=20 or num=30"),
        // expr..=expr进行范围匹配
        40..=50 => println!("num>=40 and num<=50"),
        // 匹配守卫，提供额外的条件
        x if x >= 100 => println!("x>=100"),
        1 | 2 | 3 | 4 | 500 if num == 500 => print!("num = 500"),
        // 使用_表示其它的选项
        _ => println!("num not in 20 30 50"),
    }

    // if let和match结合枚举进行模式匹配
    enum Fruit {
        Apple,
        Orange,
        Watermelon,
    }

    let my_favorite_fruit = Fruit::Watermelon;
    // if let结合枚举进行模式匹配
    if let Fruit::Watermelon = my_favorite_fruit {
        println!("是的，我最喜欢的水果是西瓜");
    } else {
        println!("猜错了");
    }

    // match结合枚举进行模式匹配
    match my_favorite_fruit {
        Fruit::Apple => println!("我讨厌吃苹果"),
        Fruit::Orange => println!("橘子一般般"),
        Fruit::Watermelon => println!("西瓜，我的最爱"),
    }

    // 循环
    // loop循环体，使用break退出循环
    let mut num = 0;
    let result = loop {
        num += 1;
        if num == 10 {
            break num * 10; // 结束循环并返回num的值乘以10
        }
    };
    println!("result:{:?}", result);

    // for循环
    // 遍历切片
    for i in 0..=100 {
        println!("i:{i}");
    }
    // 遍历数组
    for i in [1, 2, 3] {
        println!("i = {i}");
    }
    // 遍历动态数组
    for i in vec![2, 3, 4] {
        println!("i=={i}");
    }
    // 遍历HashMap
    let mut map = HashMap::new();
    map.insert("regan", 100);
    map.insert("pip", 66);
    for i in map {
        println!("{:?}", i);
    }
    // 遍历字符串
    let s = "加油学习Rust".to_string();

    for i in s.chars() {
        println!("{:?}", i);
    }
    // break退出循环 continue结束本次循环
    let mut sum = 0;
    for i in 1..=100 {
        if i == 88 {
            // 跳过本次循环
            println!("跳过本次循环");
            continue;
        }
        if i == 99 {
            // 结束循环
            println!("结束循环");
            break;
        }
        sum += i;
    }
    println!("sum={sum}");

    // while循环
    let mut i = 0;
    while i <= 10 {
        println!("i=:{i}");
        i += 1;
    }

    // while let
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top_num) = stack.pop() {
        println!("top num = {top_num}");
    }
}
