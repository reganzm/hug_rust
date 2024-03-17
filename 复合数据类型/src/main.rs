use std::any::Any;

fn main() {
    // 元组
    // 定义一个元组
    let tuple1 = ();
    println!("tuple1:{:?}", tuple1);
    // 定义一个拥有相同数据类型的元组
    let tuple2 = (2, 4, 6, 8, 10);
    println!("tuple2:{:?}", tuple2);
    // 定义一个拥有不同数据类型的元组
    let tuple3 = (1, 3.14, 9877_u32, 87658.9878_f64, true, '😄');
    println!("tuple3:{:?}", tuple3);

    // 通过下标获取元组值
    println!("tuple3 第六个元素{:?}", tuple3.5);
    // 通过解构方式获取元组值
    let (a, b, c, d, e, f) = tuple3;
    println!("第一个元素：{:?} 第二个元素:{:?} 第三个元素:{:?} 第四个元素:{:?} 第五个元素:{:?} 第六个元素:{:?}",a,b,c,d,e,f);
    let result: () = no_return_function();
    println!("result:{:?}", result);
    let result1: i32 = have_return_function();
    println!("result1:{}", result1);
}

// 隐式返回()
fn no_return_function() {
    println!("没有显式返回值的函数，隐式返回()");
}
// 表达式返回
fn have_return_function() -> i32 {
    // 可以使用return关键字返回，如:return 8+9
    // 推荐使用表达式返回
    8 + 9
}
