//! 堆排序
//! 平均复杂度O(nlog2n) 空间复杂度O(1) 不稳定

/// 计算父节点下标
fn parent(child_index: usize) -> usize {
    child_index >> 1
}
/// 计算左子节点的下标
fn left_child(parent_index: usize) -> usize {
    parent_index << 1
}
/// 计算右子节点的下标
fn right_child(parent_index: usize) -> usize {
    (parent_index << 1) + 1
}

fn heap_sort(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }

    let len = nums.len() - 1;
    let parent = parent(len);
    for end in (1..=parent).rev() {
        // 构建小顶堆，下标从1开始
        move_down(nums, end);
    }

    for end in (1..nums.len()).rev() {
        // 堆顶元素和最后一个元素交换
        nums.swap(1, end);
        move_down(&mut nums[..end], 1); // 重建堆
    }
}

fn move_down(nums: &mut [i32], mut parent: usize) {
    let last = nums.len() - 1;
    loop {
        let left = left_child(parent);
        let right = right_child(parent);
        if left > last {
            break;
        }
        let child = if right <= last && nums[left] < nums[right] {
            right
        } else {
            left
        };
        // 子节点大于父节点，交换数据
        if nums[child] > nums[parent] {
            nums.swap(parent, child);
        }
        parent = child;
    }
    println!("{:?}",nums);
}

fn main() {
    // 第一项用0填充，是无效项
    let mut nums = [0, 88, 99, 66, 33, 123, 45, 890, 10, 18, 9999, -9876];
    println!("原始值：{:?}", &nums);
    heap_sort(&mut nums);
    println!("排序结果:{:?}", nums);
}
