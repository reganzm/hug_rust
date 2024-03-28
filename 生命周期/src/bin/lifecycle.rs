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
    let max_age = bigest_age(&age1, &mut age2);
    println!("max_age:{max_age}");

    // 函数有两个一样的生命周期的参数，返回值的生命周期等于较小作用域的哪个值的生命周期
    // let age1 = 123;
    // let max_age;
    // {
    //     let mut age2 = 73;
    //     max_age = bigest_age(&age1, &mut age2);
    // }

    // println!("max_age:{max_age}");

    
    // 悬垂引用 编译不通过
    // fn get_message<'a>()->&'a str{
    //     let msg = String::from("hello AI");
    //     msg.as_str()
    // }

    // 上述代码编译不通过，做如下修改，返回内部所有权
    fn get_message()->String{
        String::from("hello AI")
    }
    

    // 结构体中的生命周期
    #[derive(Debug)]
    struct Message<'a>{
        msg:&'a str
    }

    impl<'a> Message<'a>{
        fn new(msg:&'a str)->Self{
            Self{msg}
        }
    }

    let say_hi = String::from("hi,how are you");
    let message = Message{msg:say_hi.as_str()};
    println!("{:?}", message);

    // 下面方式声明会报错
    // let message;
    // {
    //     let say_hi = String::from("hi,how are you");
    //     message = Message{msg:say_hi.as_str()};
    // }
    // println!("{:?}", message);
    

}
