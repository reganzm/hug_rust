//! 顺序查找
//! 时间复杂度O(n)
fn sequential_search<T: PartialEq>(nums: &[T], num: T) -> bool {
    let mut found = false;
    let mut index = 0;
    while index < nums.len() {
        if num == nums[index] {
            found = true;
            break;
        }
        index += 1;
    }
    found
}
fn main() {
    let num = 4;
    let nums = [2, 3, 4, 6, 7, 8, 9, 10, 4];
    let search_result = sequential_search(&nums, num);
    println!("search result:{search_result}");
}
