//! 内插查找
//! 它是二分查找的变体，思路是使用线性内插法 (y-y0)/(x-x0) = (y1-y0)/(x1-x0) => x = {(y-y0)(x1-x0)}/(y1-y0) + x0
//! 时间平均查找复杂度为O(loglog(n))
//! 前提条件：数组已经排好序

fn insert_search(nums: &[i32], target: i32) -> bool {
    if nums.is_empty() {
        return false;
    }

    let mut low = 0;
    let mut high = nums.len() - 1;
    // 一直循环找到上界
    loop {
        let low_val = nums[low];
        let high_val = nums[high];

        if high <= low || target < low_val || target > high_val {
            break;
        }

        // 插值位置
        let offset = (target - low_val) * (high - low) as i32 / (high_val - low_val);
        let insert_index = low + offset as usize;

        // 更新上/下界
        if nums[insert_index as usize] > target {
            high = insert_index - 1;
        } else if nums[insert_index] < target {
            low = insert_index + 1;
        } else {
            high = insert_index;
            break;
        }
    }
    target == nums[high]
}

fn main() {
    let num = 11;
    let nums = [2, 3, 4, 6, 7, 8, 9, 10];
    let search_result = insert_search(&nums, num);
    println!("search result:{search_result}");

    let num = 2;
    let nums = [2, 3, 6, 7, 8, 9, 10];
    let search_result = insert_search(&nums, num);
    println!("search result:{search_result}");

    let num = 10;
    let nums = [2, 3, 6, 7, 8, 9, 10];
    let search_result = insert_search(&nums, num);
    println!("search result:{search_result}");
}
