//! 选择排序
//! 平均时间复杂度O(n^2) 空间复杂度 O(1) 不稳定

pub fn select_sort(nums: &mut [i32]) {
    let mut left = nums.len() - 1;
    while left > 0 {
        let mut pos_max = 0;
        for i in 1..=left {
            if nums[i] > nums[pos_max] {
                pos_max = i;
            }
        }
        // 交换数据，完成一轮数据的排序，并将待排序数据个数减1
        nums.swap(left, pos_max);
        left -= 1;
    }
}

fn main() {
    let mut nums = [88, 99, 66, 33, 123, 45, 890, 10, 18, 9999, -9876];
    println!("原始值：{:?}", &nums);
    select_sort(&mut nums);
    println!("排序结果:{:?}", nums);
}
