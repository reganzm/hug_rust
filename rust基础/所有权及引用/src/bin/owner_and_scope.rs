#[warn(unused_variables)]
fn main() {
    let a: f64 = 3.14;
    // 所有权和作用域
    owner_and_scope();
} // <-------------------------------main函数作用域中的a离开作用域，自动被回收

// 所有权和作用域
#[warn(unused_variables)]
fn owner_and_scope() {
    let a = 1314; // <---------- a出现在owner_and_scope中
    {
        //------------------------------临时作用域
        let a = 3.15; // <-------a出现在临时作用域
        println!("a at inner scope:{a}");
        let b = 567; // <-------b出现在临时作用域
        println!("b at inner scope:{b}");
    } // <------------------------------临时作用域中的a和b离开临时作用域，会自动调用drop清理占用的内存
    let a = 520; // <-------------a1314被遮盖
    println!("a at outer scope:{a}");
} // <-----------------------------------外面的a离开作用域，自动回收
