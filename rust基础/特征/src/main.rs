use std::{
    fmt::{Debug, Display},
    ops::Add,
};

fn main() {
    // 定义特征
    // 为类型实现特征
    let tiger = Tiger::new(3, "OldSix");
    tiger.behavior();
    // 结构体方法传入特征
    tiger.show(&tiger);
    // 结构体方法返回特征
    let action = tiger.do_action();
    action.eat();

    // 打破孤儿规则
    struct MyString(String);
    impl Display for MyString {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let display_str = String::from("---->") + &self.0 + &String::from("<----");
            f.write_str(display_str.as_str())
        }
    }

    let my_string = MyString(String::from("我是newtype"));
    println!("{my_string}");

    // 函数传递特征参数
    fn show_my_string(my_string: &impl Display) {
        println!("{my_string}");
    }
    show_my_string(&my_string);

    // 泛型约束，定义say_hello泛型方法
    // 使用Action+Debug+Clone+PartialOrd+PartialEq多重特征约束
    fn say_hello<T: Action + Debug + Clone + PartialOrd + PartialEq>(item: &T) {
        println!("----开始展示----");
        item.eat();
        item.wearing();
        item.living();
        item.walk();
        println!("{:?}", item);
    }
    // 使用where字句改写
    fn say_hello1<T>(item: &T)
    where
        T: Action + Debug + Clone + PartialOrd + PartialEq,
    {
        println!("----开始展示----");
        item.eat();
        item.wearing();
        item.living();
        item.walk();
        println!("{:?}", item);
    }
    say_hello(&tiger);
    say_hello1(&tiger);

    // 函数返回特征
    fn do_action() -> impl Action + Debug + Clone + PartialEq + PartialOrd {
        Tiger::new(20, "OldSixSix")
    }
    let action = do_action();
    action.eat();

    // 定义函数，接收一个参数
    // 函数体根据参数，决定返回哪个实现了Action的实例
    // 下面代码编译报错
    // fn do_action_with_arg(label: u8) -> impl Action {
    //     if label == 1 {
    //         Tiger::new(3, "OldSix")
    //     } else {
    //         Whale::new(60, "Tom")
    //     }
    // }
    // 使用特征对象改进
    fn do_action_with_arg(label: u8) -> Box<dyn Action> {
        if label == 1 {
            Box::new(Tiger::new(3, "OldSix"))
        } else {
            Box::new(Whale::new(60, "Tom"))
        }
    }

    let action1 = do_action_with_arg(1);
    let action2 = do_action_with_arg(2);
    action1.eat();
    action2.eat();

    // 特征中的关联类型
    let mut list = List {
        datas: vec![1, 4, 6, 8, 9],
    };

    let next_value = list.next();
    println!("next_value:{:?} list:{:?}", next_value, list);

    // 特征中使用泛型
    let msg = Message {
        msg: String::from("学习rust,2024拿下rust"),
    };
    msg.printf(String::from(" 拥抱未来语言Rust "));

    // 特征中为泛型指定默认类型
    let point_a = Point { x: 34, y: 56 };
    let point_b = Point { x: 98, y: 108 };
    let point_c = point_a.sum(point_b);
    // 下面打印报错，具体涉及到所有权知识点，后面章节详细介绍
    //println!("point_a:{:?} + point_b:{:?} = point_c:{:?}",point_a, point_b, point_c);
    println!("point_c:{:?}", point_c);

    // 特征约束
    let human = Human::new(175.12, 75.01);
    human.behavior();
}

/// 定义Action特征
trait Action {
    // 吃
    fn eat(&self);
    // 穿
    // 可以提供默认实现
    fn wearing(&self) {
        println!("我穿皮毛....");
    }
    // 住
    fn living(&self);
    // 行
    fn walk(&self);
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct Tiger {
    age: u32,
    name: String,
}
impl Action for Tiger {
    fn eat(&self) {
        println!("我吃肉...特别喜欢鹿肉哦.")
    }

    fn living(&self) {
        println!("我住山洞，冬暖夏凉.")
    }

    fn walk(&self) {
        println!("我的四条火腿跑得贼快.")
    }
}

struct Whale {
    age: u32,
    name: String,
}
impl Action for Whale {
    fn eat(&self) {
        println!("我吃磷虾.");
    }

    fn living(&self) {
        println!("我住南太平洋.");
    }

    fn walk(&self) {
        println!("我用尾巴当螺旋桨");
    }
}

impl Whale {
    fn new(age: u32, name: &str) -> Self {
        Self {
            age: age,
            name: String::from(name),
        }
    }
}

impl Tiger {
    fn new(age: u32, name: &str) -> Self {
        Self {
            age: age,
            name: String::from(name),
        }
    }
    fn behavior(&self) {
        println!("大家好,我今年{}岁,我叫:{}", self.age, self.name);
        self.eat();
        self.wearing();
        self.living();
        self.walk();
    }
    // 方法中传递特征
    fn show(&self, action: &impl Action) {
        action.eat();
    }
    // 方法返回特征
    fn do_action(&self) -> impl Action + Debug + Clone + PartialEq + PartialOrd {
        Self {
            age: self.age,
            name: String::from(self.name.as_str()),
        }
    }
}

/// 在特征中使用特征约束
/// 实现HumanAction必须先实现Action特征
trait HumanAction: Action {
    fn typing(&self) {
        println!("我会码字...")
    }
}

struct Human {
    height: f32,
    weight: f32,
}

impl Action for Human {
    fn eat(&self) {
        println!("我吃得健康，必须荤素搭配.");
    }

    fn living(&self) {
        println!("我住钢筋水泥..");
    }

    fn walk(&self) {
        println!("我出行开发，坐飞机，坐火车都行..");
    }

    fn wearing(&self) {
        println!("我穿棉衣");
    }
}
impl HumanAction for Human {
    fn typing(&self) {
        println!("我靠码字养活自己.");
    }
}
impl Human {
    fn new(height: f32, weight: f32) -> Self {
        Self { height, weight }
    }
    fn behavior(&self) {
        println!("大家好,我身高{}cm,我体重:{}公斤", self.height, self.weight);
        self.eat();
        self.wearing();
        self.living();
        self.walk();
    }
}

/// 关联类型
trait Iterator {
    // 使用关键字type定义的关联类型
    // 是一个占位符，此时还未有具体的类型
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

#[derive(Debug)]
struct List {
    datas: Vec<i32>,
}
impl Iterator for List {
    type Item = i32;
    // 倒序遍历
    fn next(&mut self) -> Option<Self::Item> {
        self.datas.pop()
    }
}

// 特征中使用泛型并带默认泛型类型
// 默认泛型类型使用=号指定，这里指Self类型
// 即实现了Sum特征的类型
trait Sum<T = Self> {
    // 定义关联类型，是一个占位符
    // 该占位符由具体的结构体指定
    // 用于指定方法返回结果类型
    type Output;
    fn sum(&self, other: T) -> Self::Output;
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Sum for Point {
    // 关联类型指定为Point类型
    // 即sum函数的返回类型为Point
    type Output = Point;
    // 由于默认泛化类型是Self
    // 在Point结构体中就指Point类型
    // 因此在sum函数中T使用Point
    fn sum(&self, other: Point) -> Point {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// 在特征中使用泛型
trait Print<T: Debug> {
    fn printf(&self, data: T);
}
struct Message {
    msg: String,
}
impl<T: Debug> Print<T> for Message {
    fn printf(&self, data: T) {
        println!("self message:{} , data:{:?}", self.msg, data);
    }
}
impl Message {
    fn new(msg: String) -> Self {
        Self { msg: msg }
    }
}
