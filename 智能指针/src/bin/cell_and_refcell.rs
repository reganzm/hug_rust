use std::{
    borrow::Borrow, cell::{Cell, RefCell}, collections::HashMap, ops::{Add, Shl}, rc::Rc
};
fn main() {
    // Cell
    let regan = Student {
        age: 30,
        id: Cell::new(10000000888),
    };
    println!("regan:{:?}", regan);
    // 编译报错，age不可变
    //regan.age = 28;
    // regan.id = Cell::new(String::from("513822199xxxxx"));
    regan.id.set(10000000999);
    println!("student id:{:?} regan:{:?}", regan.id.get(), regan);

    // RefCell
    let dog = Animal {
        age: 3,
        weight: RefCell::new(10.5),
        name:RefCell::new(String::from("小憨包"))
    };
    println!("dog:{:?}", dog);
    // 修改weight
    *dog.weight.borrow_mut() += 10.1;
    // 修改name
    *dog.name.borrow_mut() = String::from("憨憨");
    println!("dog weight:{:?}", dog);

    // Rc和RefCell组建共享可被修改的数据结构
    let shared_map: Rc<RefCell<HashMap<&str,i32>>> = Rc::new(RefCell::new(HashMap::new()));
    println!("strong count :{}",Rc::strong_count(&shared_map));
    
    {
        let shared_map2 = shared_map.clone();
        println!("strong count :{}",Rc::strong_count(&shared_map));
        let mut map = shared_map.borrow_mut();
        map.insert("lucky", 1);
        map.insert("regan", 2);
        map.insert("lily", 3);
        map.insert("pop",4);
    }
    println!("strong count :{}",Rc::strong_count(&shared_map));

}

#[derive(Debug)]
struct Student {
    age: u8,
    id: Cell<i64>,
}

#[derive(Debug)]
struct Animal {
    age: u8,
    weight: RefCell<f32>,
    name:RefCell<String>
}
