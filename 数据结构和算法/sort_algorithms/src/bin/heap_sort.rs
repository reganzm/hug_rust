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

pub fn heap_sort(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }

    let len = nums.len() - 1;
    let parent = parent(len);
    // 非叶子节点：[0..len/2 -1]
    // 构建完成后，堆顶是最大元素
    for p in (1..=parent).rev() {
        // 构建大顶堆 (下标从1开始是为了方便孩子节点关系：左孩子left_c=2*p 右孩子right_c=2*p+1)
        move_down(nums, p);
    }

    for end in (1..nums.len()).rev() {
        // 堆顶元素和最后一个元素交换
        nums.swap(1, end);
        // 重建堆
        move_down(&mut nums[..end], 1);
    }
}

fn move_down(nums: &mut [i32], mut parent: usize) {
    // nums最后索引
    let last = nums.len() - 1;
    loop {
        // 左孩子
        let left = left_child(parent);
        // 右孩子
        let right = right_child(parent);
        // 退出条件
        if left > last {
            break;
        }
        // 找到大的孩子
        let child = if right <= last && nums[left] < nums[right] {
            right
        } else {
            left
        };
        // 孩子子节点大于父节点，交换数据
        if nums[child] > nums[parent] {
            nums.swap(parent, child);
        }
        // 一直下推，直到退出条件
        parent = child;
    }
}

fn main() {
    // 第一项用0填充，是无效项
    let mut nums = [0, 88, 99, 66, 33, 123, 45, 890, 10, 18, 9999, -9876];
    println!("原始值：{:?}", &nums);
    heap_sort(&mut nums);
    println!("排序结果:{:?}", nums);
}
