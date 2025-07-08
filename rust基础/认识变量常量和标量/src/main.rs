// 包级别静态变量，不允许遮盖
static mut AGE: i32 = 30;
// 遮盖报错
//static mut AGE:i32 = 30;
fn main() {
    // 不会被使用到的变量，使用_开头来定义
    let _age: i32 = 33;
    // 编译报错，不可变变量不能改变值
    //age = 31;
    let mut height: i32 = 175;
    // 可变变量可以重新绑定值，重新绑定后位于栈中的175将被自动释放掉
    height -= 5;
    println!("height:{height}");
    // 可以使用let覆盖之前定义过的同名变量
    let mut height: i32 = 185;
    height += 5;
    println!("一个美男子,身高:{height}");

    unsafe {
        println!("静态变量age:{AGE}");
        AGE += 3;
        println!("静态变量age:{AGE}");

        static mut AGE: i32 = 35;

        // 方法级别静态变量，不允许遮盖,不允许修改
        static SCORES: i32 = 88;
        // 遮盖报错
        //static SCORES:i32 = 99;
        println!("Scores:{SCORES}");
        // 允许修改，不允许遮盖
        static mut MINUTES: i32 = 50;
        MINUTES += 10;
        println!("MINUTES:{MINUTES}");
    }

    // 由于不能修改，定义时不能使用mut关键字修饰
    const HADOOP_HOME: &str = "/data/softwares/hadoop-3.2.2";
    // 不可修改
    //HADOOP_HOME = "/data/softwares/hadoop-3.2.1";
    // 不能遮盖
    //const HADOOP_HOME:&str = "/data/softwares/hadoop-3.3.3";
    println!("HADOOP HOME:{HADOOP_HOME}");

    // 整数的具体类型
    let a: i32 = 123_456_789;
    let b = 123_789_u32;
    let c = 9879768723usize;
    println!("a:{a} b:{b} c:{c}");

    // 浮点数

    let weight = 73.5_f64;
    let height = 172.5_f32;
    println!("weight:{weight},height:{height}");

    // 布尔值
    let _pass: bool = true;

    // 字符
    let smile: char = '😄';
    let dog_head: char = '🐶';

    println!("smile : {smile} , dog_head : {dog_head}");

    // 类型转换

    let bool_to_u32 = _pass as u32;
    let char_to_u32 = smile as u32;
    let f64_to_u32 = weight as u32;
    let i32_to_f64 = c as f64;
    println!("bool to u32:{bool_to_u32} \n char to u32 :{char_to_u32} \n f64 to u32 :{f64_to_u32} \n i32 to f64:{i32_to_f64}\n");
}
