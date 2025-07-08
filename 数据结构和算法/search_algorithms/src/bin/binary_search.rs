//! 二分查找
//! 时间复杂度O(log2n)
//! 前提条件：需要集合有序

/// 二分查找有序数组
fn binary_search<T: PartialEq + PartialOrd>(nums: &[T], num: T) -> bool {
    let mut found = false;
    let mut low = 0;
    let mut high = nums.len() - 1;

    while low <= high && !found {
        // 右移操作，mid为len的一半
        let mid = (low + high) >> 1;
        if num == nums[mid] {
            found = true;
            break;
        } else if num < nums[mid] {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    found
}

// 递归方式实现二分查找
fn binary_search_recursive<T: PartialEq + PartialOrd>(nums: &[T], num: T) -> bool {
    if nums.len() == 0 {
        return false;
    }
    let mid = nums.len() >> 1;
    if num == nums[mid] {
        return true;
    } else if num < nums[mid] {
        return binary_search_recursive(&nums[..mid], num);
    } else {
        return binary_search_recursive(&nums[mid..], num);
    }
}

fn main() {
    let num = 40;
    let nums = [2, 3, 4, 6, 7, 8, 9, 10];
    let search_result = binary_search(&nums, num);
    println!("search result:{search_result}");

    let num = 4;
    let nums = [2, 3, 4, 6, 7, 8, 9, 10];
    let search_result = binary_search_recursive(&nums, num);
    println!("search result:{search_result}");
}
