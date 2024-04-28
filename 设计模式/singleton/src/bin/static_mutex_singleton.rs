use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[derive(Debug)]
struct Singleton(Vec<i32>);
static mut INSTANCE: Option<Arc<Mutex<Singleton>>> = None; //静态初始化，只运行一次
static LOCK: Mutex<i32> = Mutex::new(0);
impl Singleton {
    //关联方法， 获取单例实例的方法
    fn get_instance(sec: u64) -> Arc<Mutex<Singleton>> {
        // 使用懒加载创建单例实例
        // 这里使用了 Arc 和 Mutex 来实现线程安全的单例
        // 只有第一次调用 get_instance 时会创建实例，之后都会返回已创建的实例
        unsafe {
            if INSTANCE.is_none() {
                let lock = LOCK.lock().unwrap();
                // get_or_insert_with ,如果是 None ，则将从data计算的值插入选项中，然后返回对包含值的可变引用。
                INSTANCE
                    .get_or_insert_with(|| {
                        thread::sleep(Duration::from_secs(sec));
                        Arc::new(Mutex::new(Singleton(Vec::new())))
                    })
                    .clone()
            } else {
                INSTANCE.clone().unwrap()
            }
        }
    }
    fn show_message(&self) {
        println!("singleton vec addr:{:p}", &self.0);
    }
}

// 下面是错误的单例实现方式
struct Singleton1(Vec<i32>);
impl Singleton1 {
    //关联方法， 获取单例实例的方法
    fn get_instance(sec: u64) -> Arc<Mutex<Singleton1>> {
        // 使用懒加载创建单例实例
        // 这里使用了 Arc 和 Mutex 来实现线程安全的单例
        // 只有第一次调用 get_instance 时会创建实例，之后都会返回已创建的实例
        static mut INSTANCE: Option<Arc<Mutex<Singleton1>>> = None; //静态初始化，只运行一次
        unsafe {
            // get_or_insert_with ,如果是 None ，则将从data计算的值插入选项中，然后返回对包含值的可变引用。
            INSTANCE
                .get_or_insert_with(|| {
                    thread::sleep(Duration::from_secs(sec));
                    Arc::new(Mutex::new(Singleton1(Vec::new())))
                })
                .clone()
        }
    }
    fn show_message(&self) {
        println!("singleton1 vec addr:{:p}", &self.0);
    }
}

fn main() {
    // 获取单例实例,自定义
    let handle1 = thread::spawn(|| {
        let instance = Singleton::get_instance(1);
        instance.lock().unwrap().show_message();
    });

    let handle2 = thread::spawn(|| {
        let instance = Singleton::get_instance(0);
        instance.lock().unwrap().show_message();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    // 下面是错误的单例实例化方式
    let handle1 = thread::spawn(|| {
        let instance = Singleton1::get_instance(2);
        instance.lock().unwrap().show_message();
    });

    let handle2 = thread::spawn(|| {
        let instance = Singleton1::get_instance(2);
        instance.lock().unwrap().show_message();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
