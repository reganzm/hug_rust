use std::{sync::Once, thread, time::Duration};

fn main() {
    static mut VERSION: &str = "1.2.3";
    static INIT: Once = Once::new();

    let handle1 = thread::spawn(move || {
        //thread::sleep(Duration::from_millis(10));
        INIT.call_once(|| unsafe {
            VERSION = "1.2.5";
        });
    });
    let handle2 = thread::spawn(move || {
        //thread::sleep(Duration::from_millis(10));
        INIT.call_once(|| unsafe {
            VERSION = "1.2.6";
        });
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("VERSION:{}", unsafe { VERSION });
}
