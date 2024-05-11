// pub关键字将模块暴露给外面使用
pub fn core1_test() {
    println!("lib-core core1 core1_test")
}
// 这个方法在该模块外无法访问到
fn core1_test1() {
    println!("lib-core core1 core1_test1")
}
