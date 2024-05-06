#[derive(Clone, PartialEq, Eq, Debug)]
struct Shape {
    id: String,
    mtype: String,
}
impl Shape {
    fn set_id(&mut self, id: String) {
        self.id = id;
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Circle {
    shape: Shape,
}
impl Circle {
    fn new() -> Circle {
        Circle {
            shape: Shape {
                id: String::from("123"),
                mtype: String::from("圆"),
            },
        }
    }
}

fn main() {
    // 通过构造方法构造对象 s是原型
    let s = Circle::new();
    // 通过clone方法复制对象 s1是原型s的克隆
    let mut s1 = s.clone();
    // 克隆的两个对象的值应该是相等的
    println!("s:{:?}", s);
    println!("s1:{:?}", s1);
    // 修改s1
    s1.shape.set_id(String::from("234"));
    println!("s:{:?}", s);
    println!("s1:{:?}", s1);
}
