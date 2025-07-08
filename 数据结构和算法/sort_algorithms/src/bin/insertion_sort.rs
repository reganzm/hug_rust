//! 插入排序算法
//! 平均时间复杂度O(n^2) 空间复杂度O(1) 稳定

pub fn insert_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }
    for i in 1..nums.len() {
        let mut pos = i;
        let current = nums[i];
        while pos > 0 && current < nums[pos - 1] {
            nums[pos] = nums[pos - 1];
            pos -= 1;
        }
        nums[pos] = current;
    }
}

fn main() {
    let mut nums = [88, 99, 66, 33, 123, 45, 890, 10, 18];
    println!("原始值：{:?}", &nums);
    insert_sort(&mut nums);
    println!("排序结果:{:?}", nums);
}
