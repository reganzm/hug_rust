//! 基数排序
//! 平均时间复杂度O(O*K) 空间复杂度O(n+k) 稳定

fn radix_sort(nums: &mut [usize]) {
    if nums.len() <= 1 {
        return;
    }
    // 最大值
    let max_num = nums.iter().max().unwrap().clone();

    // 寻找最小的x，使得2^x>= len
    let radix = nums.len().next_power_of_two();

    let mut digit = 1;
    while digit <= max_num {
        let index_of = |x| x / digit % radix;
        let mut counter = vec![0; radix];
        for &x in nums.iter() {
            counter[index_of(x)] += 1;
        }
        for i in 1..radix {
            counter[i] += counter[i - 1];
        }
        for &x in nums.to_owned().iter().rev() {
            counter[index_of(x)] -= 1;
            nums[counter[index_of(x)]] = x;
        }
        digit *= radix;
    }
}

fn main() {
    let mut nums = [88999, 99, 66, 33, 123, 45, 890, 10, 18];
    println!("原始值：{:?}", &nums);
    // 需要确保数值越大hash越大，不能用取余符号
    radix_sort(&mut nums);
    println!("排序结果:{:?}", nums);
}
