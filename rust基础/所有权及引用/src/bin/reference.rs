fn main() {
    let name = String::from("三角兽");
    let nick_name = String::from("兽兽");
    println!("name:{name} nick_name:{nick_name}");
    // 通过&获取对象的引用
    // 不会转移所有权
    let ref_name = &name;
    let ref_nick_name = &name;

    println!("ref_name:{ref_name} ref_nick_name:{ref_nick_name}");
    // 打印出引用的地址和值的地址
    println!("ref_name_p:{:p} name_p:{:p}", &ref_name, &(*ref_name));

    // 传递引用
    // 函数传值导致所有权转移
    let msg = String::from("2024学好Rust");
    do_somthing(&msg);
    // 下面使用msg变量能够正常编译了
    println!("msg:{msg}");

    // 结构体方法中返回引用
    let ruster = Ruster {
        name: String::from("兽兽"),
    };
    println!("ruster name:{}", ruster.get_name());


    // 可变引用
    let mut age = 123;
    let age1 = &mut age;
    // 使用*解引用，并对其值进行加法操作
    *age1 += 10;
    println!("age1:{age1}");
}

// 函数接收引用参数引用
fn do_somthing(msg: &String) {
    println!("msg:{msg}");
}

struct Ruster {
    name: String,
}
impl Ruster {
    fn new(name: String) -> Self {
        Self { name }
    }
    // 结构体方法返回引用
    fn get_name(&self) -> &String {
        &self.name
    }
}
