//! 熟悉Rust中的各种约定用法
//! 熟悉Rust中的基本概念

#[warn(unused_variables)]
#[warn(non_snake_case)]
fn main() {
    let result1: i32 = sum(10, 20);
    assert_eq!(30, result1);
    // 函数默认返回类型为单元类型
    let result2: () = test_function();
    // 使用println!宏来打印输出，使用{result1}作为占位符，用于输出result1;使用{:?}作为占位符，输出result2。
    // 因为单元类型没有实现Display Trait，所以需要使用{:?}方式来格式化
    println!("result1 = {result1} , result2 = {:?}", result2);
    println!("smile:{}", test_function1());
    println!("sum1:{}", sum1(10, 20));
    naming_example();
    // 等待不退出，便于运行可执行文件查看打印信息
    loop {}
}

/// 求两个数的和
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

/// 无参数  无显示的返回值  默认返回单元类型()
pub fn test_function() {
    println!("你好，世界");
}

fn test_function1() -> char {
    // 单行注释
    /* 单行注释 */
    /*
    多行注释
    可以换行
     */
    '\u{1F600}'
}

///计算两数之和
/// 用法：
/// ```
///  let result = sum1(10,20);
///  assert_eq!(result,30);
/// ```
///
fn sum1(a: i32, b: i32) -> i32 {
    a + b
}

/// 命名风格示例
#[warn(dead_code)]
fn naming_example() {
    // 枚举,泛型参数T和E
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // 特征Trait，泛型参数T，返回Self实例
    trait From<I> {
        fn from() -> Self;
    }

    // 结构体
    struct Student {
        age: u32,
        height: f32,
    }
    // 为结构体实现方法
    impl Student {
        // 构造函数，它是关联函数
        fn new(age: u32, height: f32) -> Self {
            Self { age, height }
        }
        // 定义其它方法
        fn get_age(&self) -> u32 {
            self.age
        }
    }

    // 静态变量
    static HODOOP_HOME: &str = "/data/softwares/hadoop-3.2.2";
    // 常量
    const VERSION: i32 = 33;

    // 定义宏
    macro_rules! add {
        ($a:expr,$b:expr) => {
            $a + $b
        };
    }

    // 使用宏
    let result = add!(2, 3);
    // 断言
    assert_eq!(result, 5);
}