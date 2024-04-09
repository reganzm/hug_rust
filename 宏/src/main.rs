macro_rules! sum_all {
    ($($arg:expr),*) => {
        {
            let mut sum = 0;
            $(sum +=$arg;)*
            sum
        }
    };
}
fn main() {
    let result = sum_all!(1, 2, 3, 4, 5);
    println!("The sum is: {}", result); // 输出: The sum is: 15

    let result = sum_all!(1, 2, 3, 4, 5, 6, 7, 8, 9);
    println!("The sum is: {}", result); // 输出: The sum is: 45


    let result = {
        let mut num = 0;
        num += 1;
        num += 2;
        num += 3;
        num += 4;
        num += 5;
        num
    };
    println!("result:{result}");
}
