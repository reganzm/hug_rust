fn main() {
    // 便量绑定
    let str_a = String::from("拿下rust");
    // 打印str_a内存地址
    println!("str_a memory address :{:p}", &(*str_a));
    // move所有权,str_b变量现在拥有值的所有权
    // str_a不再拥有值的所有权，不能再访问
    let str_b = str_a;
    // 打印str_b内存地址
    // 可以发现str_b和str_a值对应的内存地址是一样的
    // 只是拥有这个值的变量变化了
    println!("str_b memory address :{:p}", &(*str_b));
    println!("str_b:{str_b}");
    // 下面打印编译错误，原因是已不再拥有值的所有权
    // println!("str_a:{str_a}");

    // 基本类型默认实现了Copy语义，不会发生所有权的移动
    let num_a = 520_1314_0_u32;
    let num_b = num_a;
    println!(
        "num_a:{num_a},num_b:{num_b} num_a address:{:p} num_b address:{:p}",
        &num_a, &num_b
    );

    // 函数传值导致所有权转移
    let msg = String::from("2024学好Rust");
    do_somthing(msg);
    // 下面打印报错，因为"2024学好Rust"值的所有权已经被move到函数里面
    // 本作用域不能再使用该值
    //println!("msg:{msg}");

    // 传递基本类型，不会发生所有权转移
    let age = 18;
    do_somthing2(age);
    println!("age:{age}");
} // <------ str_b离开作用域，自动调用drop函数释放"拿下rust"值对应的内存

fn do_somthing(msg: String) {
    println!("msg:{msg}");
}

fn do_somthing2(age: u32) {
    println!("age:{age}");
}

struct Animal {
    name: String,
    age: i32,
}

impl Animal {
    fn new(name: String, age: i32) -> Self {
        Self { name, age }
    }
    // 下面方法报编译错误，self.name是一个共享引用
    // 不能move所有权
    //fn get_name(&self)->String{
    //    self.name
    //}
    // self.age实现了Copy特征，直接拷贝栈上的值返回
    fn get_age(&self) -> i32 {
        self.age
    }
}
