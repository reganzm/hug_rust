// 创建一个自引用类型的兔子结构体
#[derive(Debug)]
struct Rabbit {
    name: String,
    p: *const String,
}

impl Rabbit {
    fn new(txt: &str) -> Self {
        Rabbit {
            name: String::from(txt),
            p: std::ptr::null(),
        }
    }
    fn init(&mut self) {
        let self_ref: *const String = &self.name;
        self.p = self_ref;
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn p_value(&self) -> &String {
        assert!(
            !self.p.is_null(),
            "Rabbit::p_value called without Rabbit::init being called first"
        );
        unsafe { &*(self.p) }
    }
}

fn main() {
    let mut rabbit1 = Rabbit::new("小白");
    rabbit1.init();
    let mut rabbit2: Rabbit = Rabbit::new("小黑");
    rabbit2.init();

    println!("rabbit1 name: {}, rabbit1 p_value: {} rabbit1 name addr:{:p} rabbit1 p addr:{:p} rabbit1 addr:{:p}",
     rabbit1.name(), rabbit1.p_value(),&rabbit1.name,rabbit1.p,&rabbit1);
    println!("rabbit2 name: {}, rabbit2 p_value: {} rabbit2 name addr:{:p} rabbit2 p addr:{:p} rabbit2 addr:{:p}",
     rabbit2.name(), rabbit2.p_value(),&rabbit2.name,rabbit2.p,&rabbit2);
    println!("--------------------------------------------------------------------------------------------");
    // std::mem::swap函数用于交换两个内存地址处的值
    std::mem::swap(&mut rabbit1, &mut rabbit2);
    println!("rabbit1 name: {}, rabbit1 p_value: {} rabbit1 name addr:{:p} rabbit1 p addr:{:p} rabbit1 addr:{:p}", 
    rabbit1.name(), rabbit1.p_value(),&rabbit1.name,rabbit1.p,&rabbit1);
    println!("rabbit2 name: {}, rabbit2 p_value: {} rabbit2 name addr:{:p} rabbit2 p addr:{:p} rabbit2 addr:{:p}",
     rabbit2.name(), rabbit2.p_value(),&rabbit2.name,rabbit2.p,&rabbit2);
}
