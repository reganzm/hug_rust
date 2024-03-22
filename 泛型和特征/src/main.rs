use std::{
    cmp::max,
    fmt::{Debug, Display},
    ops::{Add, Mul},
};

fn main() {
    // 编译阶段T被替换为i32类型
    let s: i32 = sum(120, 20);
    println!("s:{s}");
    // 编译阶段T被替换为f32类型
    let s: f32 = sum(120_f32, 20_f32);
    println!("s:{s}");

    let arr = [1, 2, 3, 4, 5, 6];
    let lar = largest(&arr);
    println!("最大值:{lar}");

    let arr1 = ["a", "ff", "yf", "f"];
    let lar = largest(&arr1);
    println!("最大值:{lar}");

    // 结构体中的泛型
    let rectangle = Rectangle {
        length: 39,
        width: 89,
    };
    println!("周长:{},面积:{}", rectangle.perimeter(), rectangle.area());
    let rectangle = Rectangle {
        length: 3.14,
        width: 89.77,
    };
    println!("周长:{},面积:{}", rectangle.perimeter(), rectangle.area());

    // 枚举中使用泛型,T由具体的类型进行替换
    let some_data: Option<i32> = Some(123);
    // 使用if let进行模式匹配，将Some同的值绑定到data上
    // 如果匹配成功，将打印出data数值
    if let Some(data) = some_data {
        println!("data:{data}");
    }
    // 使用_占位，并由Ok中的123推断出_的类型为i32
    // 对于Result中的E类型，需要显示指定类型，否则编译不通过
    let result: Result<_, &str> = Ok(123);
    if let Ok(data) = result {
        println!("ok data:{data}");
    }

    // 特殊的泛型
    let a = [1, 3, 4, 5];
    let b = [3, 4, 5];
    // 调用接收数组切片的函数，打印
    let c = ["a", "c", "m"];
    print1(&a);
    print1(&b);
    print1(&c);

    print2(a);
    print2(b);
    print2(c);
}

// 接收数组切片的函数
// 泛型参数T的泛型限定为Debug，Rust内置的打印特征
// 表示T是可以被打印输出的
fn print1<T: Debug>(a: &[T]) {
    println!("a={:?}", a);
}
// 另外一种打印数组的泛型方法
// T泛型限定为Debug，可打印
// N应该为一个数量,使用usize进行限定是不行的
// 因为usize是一个数据类型，需要加上const
// 表示这是一个const类型的泛型且是一个数值类型
fn print2<T: Debug, const N: usize>(a: [T; N]) {
    println!("a={:?}", a);
}

// 定义泛型函数，泛型需要再尖括号中声明才能使用
fn sum<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

// 定义泛型函数，求最大值
// 函数参数为T类型的切片
// 返回值为T
// 要比较切片中的值，需要为T增加一个约束，让T可以比较
// 使用Rust内置的PartialOrd特征约束,实现该特征的类型具有可比较性
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 结构体中使用泛型

struct Rectangle<U> {
    length: U,
    width: U,
}
impl<U: Mul<U, Output = U> + Add<U, Output = U> + Copy + PartialOrd> Rectangle<U> {
    /// 求周长，该方法返回值是泛型
    fn perimeter(&self) -> U {
        self.length + self.width + self.length + self.width
    }
    /// 求面积，该方法返回值是泛型
    fn area(&self) -> U {
        self.length * self.width
    }
    /// maxrectangle构造并返回最大的矩形
    fn max_rectangle(&self, other: Self) -> Rectangle<U> {
        let mut max_length = self.length;
        if other.length > self.length {
            max_length = other.length;
        }
        let mut max_width = self.width;
        if other.width > self.width {
            max_width = other.width;
        }
        Self {
            length: max_length,
            width: max_width,
        }
    }
}
