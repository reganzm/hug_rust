//! 排序算法之冒泡排序
//! 平均时间复杂度为O(n^2) 空间复杂度O(1) 稳定

pub fn bubbule_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    } else {
        for i in 1..nums.len() {
            for j in 0..nums.len() - i {
                // 如果j大于j+1元素，则交换
                if nums[j] > nums[j + 1] {
                    nums.swap(j, j + 1);
                }
            }
        }
    }
}

fn main() {
    let mut nums = [88, 99, 66, 33, 123, 45, 890, 10, 18];
    bubbule_sort(&mut nums);
    println!("排序结果:{:?}", nums);
}
