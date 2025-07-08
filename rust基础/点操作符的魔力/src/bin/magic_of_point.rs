use std::rc::Rc;

fn main() {
    let array: Rc<Box<[i32; 10]>> = Rc::new(Box::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 0]));
    let first_element = array.get(0);
    println!("first element:{:?}", first_element);

    let mut rc = Rc::new(10);

    let rc1 = &rc;
    let mut rc1 = &mut rc;

    let array1 = array.clone();
    //let sandbox = *array1;

    let mut Box = Box::new(1);

    let a = [1, 2, 3, 4];
    let result = a.get(0);
}
