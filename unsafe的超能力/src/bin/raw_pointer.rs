use std::{slice::from_raw_parts, str::from_utf8_unchecked};

fn main() {
    create_raw_point();
    //random_mem_addr_as_raw_pointer();

    let hi = "Hi 2024";
    // 获取“Hi 2024”的内存地址
    let hi_addr = hi.as_ptr() as usize;
    // 字符串长度
    let len = hi.len();
    // 从指定内存地址处读取指定长度的数据
    let data = unsafe { from_utf8_unchecked(from_raw_parts(hi_addr as *const u8, len)) };
    println!("data :{data}");

    // unsafe代码
    unsafe fn dangerous() {}
    unsafe { dangerous() };

    fn safe() {
        unsafe { dangerous() };
    }
    safe();

    // 使用extern声明这是一个C库中的方法
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    // 对于外部库中的方法，必须使用unsafe代码块包裹
    println!("Absolute value of -99999 according to C: {}", unsafe {
        abs(-99999)
    });

    static mut VERSION: u8 = 12_u8;
    unsafe { VERSION = 13 };
}

/// 创建裸指针
fn create_raw_point() {
    let mut age = 30;
    // 基于值的引用创建不可变和可变引用
    let raw_point1 = &age as *const i32;
    let raw_point2 = &mut age as *mut i32;
    println!("raw point1 value:{}", unsafe { *raw_point1 });
    println!("raw point2 value:{}", unsafe { *raw_point2 });
}

/// 凭空捏造一个内存地址作为裸指针
fn random_mem_addr_as_raw_pointer() {
    let addr = 100000;
    let raw_pointer = addr as *mut i32;
    // 修改内存地址100001地址处的值
    unsafe {
        *raw_pointer = 123;
    }
}
