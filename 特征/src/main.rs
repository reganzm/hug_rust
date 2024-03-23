use std::fmt::Display;

fn main() {
    // 定义特征
    // 为类型实现特征
    let tiger = Tiger::new(3, "OldSix");
    tiger.behavior();


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
}
