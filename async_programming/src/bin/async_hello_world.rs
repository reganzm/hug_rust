use futures::executor::block_on;

/// 异步编程的Hello world
fn main() {
    // 使用async fn创建异步函数
    async fn async_hi() {
        println!("hello world from async");
    }
    // 调用async_hi异步函数
    let result = async_hi();
    block_on(result);

    async fn async_hello() {
        println!("hello");
        async_world().await;
    }
    async fn async_world() {
        println!("world")
    }
    block_on(async_hello());
}
