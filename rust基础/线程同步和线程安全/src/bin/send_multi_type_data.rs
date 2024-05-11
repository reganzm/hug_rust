use std::{sync::mpsc, thread};
fn main() {
    // 创建枚举
    #[derive(Debug)]
    enum Animal {
        Dog(String, u8),
        Cat(String, u8),
        Fish(String, u8, f32),
    }
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let dog = Animal::Dog(String::from("憨憨"), 3);
        let cat = Animal::Cat("胖丁儿".to_string(), 3);
        let fish = Animal::Fish("鱼儿".to_string(), 1, 3.5);
        sender.send(dog).unwrap();
        sender.send(cat).unwrap();
        sender.send(fish).unwrap();
    });
    // 在主线程中接收消息
    for msg in receiver {
        println!("{:?}", msg);
    }
}
