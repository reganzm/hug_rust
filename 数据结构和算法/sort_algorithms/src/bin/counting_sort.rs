//! 计数排序
//! 平均时间复杂度O(n+k) 空间复杂度O(n+k) 稳定 n为待排数组长度，k为待排数据个数

fn counting_sort(nums: &mut [usize]) {
    if nums.len() <= 1 {
        return;
    }

    // 桶的数量等于最大值-最小值+1
    let max = nums.iter().max().unwrap().clone();
    let min = nums.iter().min().unwrap().clone();

    let bkt_nums = max - min + 1;

    // 将数据入桶
    let mut counter = vec![0; bkt_nums];

    for i in nums.iter() {
        let index = (i - min) as usize;
        counter[index] += 1;
    }

    // 将数据写回
    let mut j = 0;
    for i in 0..bkt_nums {
        while counter[i] > 0 {
            nums[j] = i + min;
            counter[i] -= 1;
            j += 1;
        }
    }
}

fn main() {
    let mut nums = [88999, 99, 66, 33, 123, 45, 890, 10, 18, 897543];
    println!("原始值：{:?}", &nums);
    counting_sort(&mut nums);
    println!("排序结果:{:?}", nums);
}
