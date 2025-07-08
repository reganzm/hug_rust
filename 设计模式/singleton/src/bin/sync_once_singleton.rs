use std::cell::RefCell;
use std::sync::{Arc, Once};
use std::{thread, vec};
type V = Option<Arc<RefCell<Vec<i32>>>>;
static mut VALS: V = None;
static INIT: Once = Once::new();
fn main() {
    let handle1 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VALS = Some(Arc::new(RefCell::new(vec![1, 2, 3, 4, 5])));
            println!("thread1 INIT VALS:{:?}", VALS);
        });
    });
    let handle2 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VALS = Some(Arc::new(RefCell::new(vec![1, 2, 3, 4, 5, 6])));
            println!("thread1 INIT VALS:{:?}", VALS);
        });
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("{:?}", unsafe { VALS.take() });
}
