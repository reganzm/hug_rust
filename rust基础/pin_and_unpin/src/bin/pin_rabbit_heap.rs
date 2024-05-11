use std::marker::PhantomPinned;
use std::pin::Pin;
/// 将值Pin在栈上
// 创建一个自引用类型的兔子结构体
#[derive(Debug)]
struct Rabbit {
    name: String,
    p: *const String,
    _marker: PhantomPinned,
}

impl Rabbit {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let rabbit = Rabbit {
            name: String::from(txt),
            p: std::ptr::null(),
            // 这个标记可以让Rabbit自动实现!Unpin
            _marker: PhantomPinned,
        };
        let mut boxed = Box::pin(rabbit);
        let self_ptr: *const String = &boxed.as_ref().name;
        unsafe { boxed.as_mut().get_unchecked_mut().p = self_ptr };
        boxed
    }
    fn name(self: Pin<&Self>) -> &str {
        &self.get_ref().name
    }
    fn p_value(self: Pin<&Self>) -> &String {
        assert!(
            !self.p.is_null(),
            "Rabbit::p_value called without Rabbit::init being called first"
        );
        unsafe { &*(self.p) }
    }
}

fn main() {
    let mut rabbit1 = Rabbit::new("小白");
    let mut rabbit2 = Rabbit::new("小黑");

    println!("rabbit1 name: {}, rabbit1 p_value: {} rabbit1 name addr:{:p} rabbit1 p addr:{:p} rabbit1 addr:{:p}",
     Rabbit::name(rabbit1.as_ref()), Rabbit::p_value(rabbit1.as_ref()),&rabbit1.name,rabbit1.p,&rabbit1);
    println!("rabbit2 name: {}, rabbit2 p_value: {} rabbit2 name addr:{:p} rabbit2 p addr:{:p} rabbit2 addr:{:p}",
     Rabbit::name(rabbit2.as_ref()), Rabbit::p_value(rabbit2.as_ref()),&rabbit2.name,rabbit2.p,&rabbit2);
}
