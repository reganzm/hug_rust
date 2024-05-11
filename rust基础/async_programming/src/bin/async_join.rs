use futures::{executor::block_on, FutureExt};

/// join并发的执行多个Future
fn main() {
    // 使用异步的方式计算(1+1)*(2+2)
    async fn one_add_one() -> i32 {
        1 + 1
    }
    async fn two_add_two() -> i32 {
        2 + 2
    }
    async fn part_mult_part() -> i32 {
        let f1 = one_add_one();
        let f2 = two_add_two();
        // 使用join!并发执行多个Future
        let res = futures::join!(f1, f2);
        res.0 * res.1
    }
    let result = block_on(part_mult_part());
    println!("(1+1)*(2+2)={result}");
}
