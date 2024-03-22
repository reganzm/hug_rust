use std::ops::{Add, Mul};

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
    let rectangle = Rectangle{length:39,width:89};
    println!("周长:{},面积:{}",rectangle.perimeter(),rectangle.area());
    let rectangle = Rectangle{length:3.14,width:89.77};
    println!("周长:{},面积:{}",rectangle.perimeter(),rectangle.area());



    // 枚举中使用泛型

    Option

    Result
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
impl<U: Mul<U, Output = U> + Add<U, Output = U> + Copy> Rectangle<U> {
    /// 求周长，该方法返回值是泛型
    fn perimeter(&self) -> U {
       self.length + self.width + self.length + self.width
    }
    // 求面积，该方法返回值是泛型
    fn area(&self) -> U {
        self.length * self.width
    }
}
