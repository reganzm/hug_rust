use std::{mem::size_of_val, ops::Deref};
fn main() {
    let a: &str = "Rust";
    // 分别打印出&str的大小、切片变量地址、切片对应值的地址
    // 和切片对应值的大小，它和切片中的len属性大小一致
    println!(
        "size of a:{} , address of a:{:p}, value address of a:{:p} , size of data:{}",
        a.len(),
        &a,
        &(*a),
        size_of_val(&(*a))
    );

    // 指针(引用)
    let a: u8 = 8;
    let b: &u8 = &a;
    let c: *const u32;

    let a: i32 = 8;
    let b: &i32 = &a;
    // 裸指针
    let a: i32 = 8;
    let b: *const i32 = &a;
    unsafe {
        println!("a:{a} b:{}", *b);
    }

    // 指针的指针(引用的引用)
    let a = 123_i32;
    let b = &a;
    let c = &b;
    let d = &c;
    println!(
        "d address:{:p} , c address:{:p} , b address:{:p} , a address:{:p} , a value:{a}",
        &d, d, c, b
    );

    // 为结构体实现Deref和Drop特征
    let sandbox = SandBox("Ruster");
    // sandbox离开作用域调用drop自动释放内存
    println!("sandbox:{:?}", sandbox);
    // 使用*解引用运算符获取结构体中的值(如果是指针就是获取指针指向的内存中的值)
    // 如果没有实现Deref接口，*sandbox是会报错的
    println!("sandbox value:{:?}", *sandbox);

    let sandbox_1 = SandBox(33);
    println!("sandbox_1:{:?}", sandbox_1);
    // 使用解引用运算符*相当于调用了deref方法
    println!("sandbox_1 value:{:?} {:?}", *sandbox_1, sandbox_1.deref());

    // Box指针，数据存储在对上，调用指针自动解引用
    let age_stack = 10;
    let age_heap = Box::new(age_stack);
    println!(
        "age stack:{age_stack} 
        age_heap:{age_heap} 
        age_stack_addr:{:p}  
        age_heap_addr:{:p}",
        &age_stack,
        &(*age_heap)
    );
    // 使用*解引用堆上的数据
    println!("{} {} ", age_stack == *age_heap, age_heap.deref());
    // 使用特征对象存储不同的类型到集合中
    get_animals();
    // Box::leak，获取全局生效的静态字符串
    println!("{}", leak_a_static_str());
}

// 定义元组结构体
#[derive(Debug)]
struct SandBox<T>(T);
impl<T> SandBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}
impl<T> Deref for SandBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        println!("解引用...");
        &self.0
    }
}
impl<T> Drop for SandBox<T> {
    fn drop(&mut self) {
        println!("沙箱调用drop方法释放内存");
    }
}

trait Eat {
    fn eat(&self);
}
#[derive(Debug)]
struct Dog {
    age: i32,
}
#[derive(Debug)]
struct Pig {
    age: i32,
}

impl Eat for Dog {
    fn eat(&self) {
        println!("我吃肉");
    }
}
impl Eat for Pig {
    fn eat(&self) {
        println!("我吃草");
    }
}

fn get_animals() {
    // 能成为特征对象，必须保证类型安全：需满足如下条件
    // 结构体中不包含泛型
    // 所有方法中都不是返回Self
    // 方法不是异步的
    let animals: Vec<Box<dyn Eat>> = vec![Box::new(Pig { age: 2 }), Box::new(Dog { age: 3 })];
    for animal in animals {
        animal.eat();
    }
}

// Box::leak关联函数
fn leak_a_static_str() -> &'static str {
    let s = String::from("拿下Rust,成为一个Ruster");
    Box::leak(s.into_boxed_str())
}

// Box所有权转移
fn move_ownership_on_box() {
    let pig = Box::new(Pig { age: 10 });
    println!("pig:{:?}", pig);
    // pig的所有权转移到pig1，pig不能继续访问
    let pig1 = pig;
    // 下面注释掉的代码会报所有权转移的编译错误
    //println!("pig1:{:?} pig:{:?}",pig1,pig);
    println!("pig1:{:?} ", pig1);
}
