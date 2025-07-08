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

    // 数组，在栈上创建，大小不可变
    let mut array1 = [1, 2, 3, 4, 5];
    println!("array1 size:{}", array1.len());
    // 使用vec!宏创建堆上可变大小的数组
    let mut array2 = vec![2, 3, 4, 5, 5, 6, 7];
    println!("array2 :{:?}", array2);
    array2.push(9999);
    println!("array2 :{:?}", array2);
    // 使用Vec的关联函数
    let mut array3 = Vec::new();
    array3.push(99);
    array3.push(88);
    array3.push(77);
    println!("array3 :{:?}", array3);

    // 结构体
    // 通过过程宏实现Debug特征，便于使用println！打印输出
    #[derive(Debug)]
    struct Student {
        age: i32,
        score: u32,
    }
    // impl块实现关联函数和方法
    impl Student {
        // 关联函数,返回Self表示Student实例
        fn new(age: i32, score: u32) -> Self {
            Student {
                age: age,
                score: score,
            }
        }
        // 第一个参数为self，注意和关联函数返回值Self的区别，小写self表示类型，大写Self表示实例
        fn get_age(&self) -> i32 {
            self.age
        }
    }
    // 结构体可以实现多个impl块
    impl Student {
        fn get_score(&self) -> u32 {
            self.score
        }
    }

    let student = Student::new(25, 140);
    // 实现了Debug特征，可以:?的方式输出
    println!(
        "age:{} , score:{}, student:{:?}",
        student.age, student.score, student
    );

    // 为Student实现Sing特征
    impl Sing for Student {
        fn sing(&self) {
            println!("唱歌:dou ri mi fa sou la xi do.....");
        }
    }
    // 调用sing方法
    student.sing();

    // 枚举
    let action = Action::JUMP { height: 2.12_f64 };
    do_action(&action);
    println!("action:{:?}", &action);
    let action = Action::QUIT;
    println!("action:{:?}", &action);
    do_action(&action);

    action.say_hello("Rust");
    action.sing();

    // 切片
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..11];
    println!("{hello} -- {world}");
    // 索引从0开始的两种写法
    let s = String::from("rust");
    let slice = &s[0..2];
    println!("{slice}");
    let slice = &s[..2];
    println!("{slice}");
    // 索引到最后一个字符的写法
    let s = String::from("rust");
    let len = s.len();
    let slice = &s[2..len];
    println!("{slice}");
    let slice = &s[2..];
    println!("{slice}");
    // 索引必须落在UTF-8编码字符的边界，否则会报错退出
    let s = "中国人";
    //let a = &s[0..2];
    //println!("{a}");

    // 在数组中使用切片
    let array = [2, 4, 6, 8, 10, 3, 6, 9];
    println!("array slice:{:?}", &array[2..]);
    println!("array slice:{:?}", &array[..5]);

    // 复合类型数据转换
    from_and_into_trait();
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

trait Sing {
    fn sing(&self) {
        println!("唱歌...")
    }
}

#[derive(Debug)]
enum Action {
    QUIT,
    RUN { speed: i32, direction: u8 },
    JUMP { height: f64 },
}
// 为枚举实现方法
impl Action {
    fn say_hello(&self, name: &str) {
        println!("say hello,{name}");
    }
}
// 为枚举实现特征
impl Sing for Action {
    fn sing(&self) {
        println!("i'am action,i can sing,啦啦啦啦啦.....");
    }
}

// 传入枚举进行模式匹配
fn do_action(action: &Action) {
    if let Action::QUIT = action {
        println!("退出....");
        return;
    }
    // 使用match进行模式匹配
    match action {
        Action::JUMP { height } => {
            println!("跳跃{}米", height)
        }
        Action::RUN { speed, direction } => {
            println!("跑，速度:{}m/s , 方向:{}", speed, direction)
        }
        Action::QUIT => {
            println!("退出....")
        }
    }
}

// From和Into两个Trait的演示用例
fn from_and_into_trait() {
    // 动物
    #[derive(Debug)]
    struct Animal {
        age: u32,
    }
    // 龙
    #[derive(Debug)]
    struct Loong {
        age: u32,
    }

    impl From<Animal> for Loong {
        fn from(value: Animal) -> Self {
            Loong { age: value.age }
        }
    }
    impl Into<Animal> for Loong {
        fn into(self) -> Animal {
            Animal { age: self.age }
        }
    }

    let loong = Loong { age: 10000 };
    println!("中国龙:{:?}", loong);
    let loong_is_a_animal: Animal = loong.into();
    println!("animal:{:?}", loong_is_a_animal);
    let animal = Animal { age: 9999 };
    let loong = Loong::from(animal);
    println!("loong:{:?}", loong);
}
