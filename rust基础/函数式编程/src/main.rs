use core::num;

fn main() {
    // 定义闭包
    // 判断是否为奇数
    let is_odd_number = |num: u32| num % 2 != 0;

    let a = 10;
    let b = 11;
    // 像普通函数一样使用
    println!(
        "a is odd number:{} b is odd number:{}",
        is_odd_number(a),
        is_odd_number(b)
    );

    // 闭包体捕获外部变量
    let base = 11;
    let add = |x: i32| x + base;
    let result = add(9);
    println!("add result:{result} ");

    let base = 12;
    // 使用move关键字，强制移动外部变量到闭包体中
    let add = move |x: i32| {
        // 打印此内存地址
        println!("base addr:{:p}", &base);
        x + base
    };
    let result = add(18);
    // 下面不会报错，因为基础变量实现了Copy语义，栈上复制
    // 从打印出的内存地址上可以验证这一点
    println!("add result:{result} base:{base} base addr:{:p}", &base);

    // 闭包作为函数参数
    let closure_1 = |a: i32, b: i32| -> i32 { a + b };
    fn test_closure_1(a: i32, b: i32, closure: impl Fn(i32, i32) -> i32) -> i32 {
        closure(a, b)
    }
    let a = 123;
    let b = 234;
    let c = test_closure_1(a, b, closure_1);
    println!("123+234={c}");

    // 闭包作为函数返回值
    fn return_closure(a: i32) -> impl Fn(i32) -> i32 {
        move |x: i32| a * x
    }
    let a = 10;
    let closure = return_closure(a);
    let result = closure(100);
    println!("10 * 100 ={result}");

    // 闭包中的生命周期
    // 编译错误
    //let closure = |x:&i32|->&i32{x};
    // 使用Fn
    fn fn_closure<T, F: Fn(&T) -> &T>(f: F) -> F {
        f
    }
    let closure = fn_closure(|x: &i32| -> &i32 { x });
    println!("fn result:{}", closure(&32));

    // 迭代器
    let mut nums = vec![1, 2, 3, 4, 3, 2, 1, 3, 4, 5, 99];
    // iter不改变值且可重入
    for num in nums.iter() {
        println!("num:{num}");
    }
    // iter_mut可改变值且可重入
    for num in nums.iter_mut() {
        *num += 10;
        // 可以再次使用
        println!("num:{num}");
    }
    // into_iter不可改变值且不可重入
    for num in nums.into_iter() {
        // 编译报错,不可修改
        //*num += 1;
        println!("num:{num}");
    }
    // 编译报错，nums不可在使用，nums被into_iter消费掉了
    //println!("{:?}",nums);

    // 函数式编程
    let nums = vec![1,4,5,13,15,45,90,99];
    let result = nums.iter().filter(|&num|num%3==0 && num%5==0).sum::<i32>();
    println!("result:{result}");

}
