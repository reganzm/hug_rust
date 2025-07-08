//! 希尔排序
//！ 平均时间复杂度O(n^1.3) 空间复杂度O(1) 不稳定

pub fn shell_sort(nums: &mut [i32]) {
    // 内部是插入排序
    fn insert_sort(nums: &mut [i32], start: usize, gap: usize) {
        let mut i = start + gap;
        while i < nums.len() {
            let mut pos = i;
            let current = nums[pos];

            while pos >= gap && current < nums[pos - gap] {
                nums[pos] = nums[pos - gap];
                pos -= gap;
            }
            nums[pos] = current;
            i += gap;
        }
    }

    // 每次将gap减少一半，直到1为止
    let mut gap = nums.len() / 2;

    while gap > 0 {
        insert_sort(nums, 0, gap);
        gap /= 2;
    }
}

fn main() {
    let mut nums = [88, 99, 66, 33, 123, 45, 890, 10, 18, 9999, -9876];
    println!("原始值：{:?}", &nums);
    shell_sort(&mut nums);
    println!("排序结果:{:?}", nums);
}
