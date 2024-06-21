//! 归并排序 递归+合并
//! 平均时间复杂度O(nlog(2n)) 空间复杂度O(n) 稳定

fn merge_sort(nums: &mut [i32]) {
    if nums.len() > 1 {
        let mid = nums.len() >> 1;
        println!("nums:{:?},mid:{mid}", nums);
        // 排序前半部分数据
        merge_sort(&mut nums[..mid]);
        // 排序后半部分数据
        merge_sort(&mut nums[mid..]);

        merge(nums, mid);
    }
}

fn merge(nums: &mut [i32], mid: usize) {
    let mut l = 0;
    let mut r = mid;
    let mut tmp = Vec::new();
    for _ in 0..nums.len() {
        if r == nums.len() || l == mid {
            break;
        }

        // 将数据放到临时集合中
        if nums[l] < nums[r] {
            tmp.push(nums[l]);
            l += 1;
        } else {
            tmp.push(nums[r]);
            r += 1
        }
    }

    // 合并的两部分数据长度很大可能不一样，需要将集合中未处理的数据全部加入
    if l < mid && r == nums.len() {
        for i in l..mid {
            tmp.push(nums[i]);
        }
    } else if l == mid && r < nums.len() {
        for i in r..nums.len() {
            tmp.push(nums[i]);
        }
    }

    // 将tmp中的数据放回nums
    for i in 0..nums.len() {
        nums[i] = tmp[i];
    }
}

fn main() {
    let mut nums = [88, 99, 66, 33, 123, 45, 890, 10, 18, 9999, -9876];
    println!("原始值：{:?}", &nums);
    merge_sort(&mut nums);
    println!("排序结果:{:?}", nums);
}
