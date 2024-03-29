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
    fn bigest_age<'a>(age1: &'a i32, age2: &'a mut i32) -> &'a i32 {
        println!("age1:{age1} age2:{age2}");
        if age1 > age2 {
            age1
        } else {
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
    fn get_message() -> String {
        String::from("hello AI")
    }

    // 结构体中的生命周期
    #[derive(Debug)]
    struct Message<'a> {
        msg: &'a str,
    }

    impl<'a> Message<'a> {
        fn new(msg: &'a str) -> Self {
            Self { msg }
        }
    }

    let say_hi = String::from("hi,how are you");
    let message = Message {
        msg: say_hi.as_str(),
    };
    println!("{:?}", message);

    // 下面方式声明会报错
    // let message;
    // {
    //     let say_hi = String::from("hi,how are you");
    //     message = Message{msg:say_hi.as_str()};
    // }
    // println!("{:?}", message);

    // 生命周期消除规则
    /*
    // 第一个例子
    fn fun(msg:&str)->&str{msg};
    // 编译器运用第一条规则，默认加上生命周期
    fn fun<'a>(msg:&'a str)->&str{msg};
    // 再运用第二条规则,确定返回值的声明周期
    fn fun<'a>(msg:&'a str)->&'a str{msg};

    // 第二个例子
    fn fun(msg:&str,name:&str)->&str{msg};
    // 编译器运用第一个规则，给输入引用默认加上独立生命周期
    fn fun<'a,'b>(msg:&'a str,name:&'b str)->&str{msg};
    // 此时第二个规则无法使用，地撒个规则也无法使用
    // 这时需要手动标注生命周期才能解决了，手动标注rux
    fn fun<'a,'b>(msg:&'a str,name:&'b str)->&'a str{msg};
    */
    // 第三条规则，结构体方法中第一个参数是&self或&mut self
    struct Animal<'a> {
        name: &'a str,
    }
    impl<'a> Animal<'a> {
        fn moo(&self, voice: &str) -> &str {
            println!("the cows moo and moo:{voice}");
            self.name
        }
        // moo等价于
        fn moo1<'b>(&'a self, voice: &'b str) -> &'a str {
            println!("the cows moo and moo:{voice}");
            self.name
        }
        // 做个小的改动，返回voice
        fn moo2<'b:'a>(&'a self, voice: &'b str) -> &'b str {
            println!("the cows moo and moo:{voice}");
            voice
        }
        // 和实现泛型约束一样，也可以使用where语法
        fn moo3<'b>(&'a self, voice: &'b str) -> &'b str where 'b:'a {
            println!("the cows moo and moo:{voice}");
            voice
        }
    }
}
