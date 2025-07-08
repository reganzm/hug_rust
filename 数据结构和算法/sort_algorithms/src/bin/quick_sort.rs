//! 快速排序
//! 平均时间复杂度O(nlog2N) 空间复杂度O(nlog2N) 不稳定

pub fn quick_sort(nums: &mut [i32], low: usize, high: usize) {
    if low < high {
        // 选出分裂点
        let split_index = partition(nums, low, high);
        if split_index > 1 {
            // 对分裂点左边排序
            quick_sort(nums, low, split_index - 1);
        }
        // 对分裂点右边排序
        quick_sort(nums, split_index + 1, high);
    }
}

fn partition(nums: &mut [i32], low: usize, high: usize) -> usize {
    let mut left = low; // 左标记
    let mut right = high; // 右标记
                          // nums[low]作为参考值
    loop {
        // 左标记右移
        while left <= right && nums[left] <= nums[low] {
            left += 1;
        }
        // 右标记左移
        while left <= right && nums[right] >= nums[low] {
            right -= 1
        }
        // 当左标记越过右标记时退出
        if left > right {
            break;
        } else {
            // 交换左右标记的值
            nums.swap(left, right);
        }
    }
    // 交换右标记和参考值
    nums.swap(low, right);
    right
}
fn main() {
    let mut nums = [88, 99, 66, 33, 123, 45, 890, 10, 18];
    println!("原始值：{:?}", &nums);
    let high = nums.len() - 1;
    quick_sort(&mut nums, 0, high);
    println!("排序结果:{:?}", nums);
}
