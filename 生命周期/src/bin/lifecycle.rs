fn main() {
    // 悬垂指针
    // let age;
    // {
    //     let age1 = 30;
    //     age = &age1;
    // }
    // println!("age:{age}");

    // 生命周期标注语法
    // 显式周期的引用 &'a u32
    // 显式周期的可变引用 &'a mut u32

    // 函数传参，显式标明生命周期
    // 类似于泛型，要在尖括号中先声明才能使用
    // 第一个参数是生命周期为'a的不可变引用
    // 第二个参数是生命周期为'a的可变引用
    fn bigest_age<'a>(age1:&'a i32,age2:&'a mut i32)->&'a i32{
        println!("age1:{age1} age2:{age2}");
        if age1 > age2{
            age1
        }else{
            age2
        }
    }
    let age1 = 123;
    let mut age2 = 73;
    bigest_age(&age1, &mut age2);


}
