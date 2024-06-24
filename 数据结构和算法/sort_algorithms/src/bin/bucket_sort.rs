//! 桶排序
//! 平均时间复杂度O(n+k)，其中k是桶个数 空间复杂度O(n+k) 稳定

use std::fmt::Debug;

struct Bucket<H, T> {
    hash: H,
    values: Vec<T>,
}

impl<H, T> Bucket<H, T> {
    fn new(hash: H, value: T) -> Self {
        Bucket {
            hash: hash,
            values: vec![value],
        }
    }
}

pub fn bucket_sort<H, T, F>(nums: &mut [T], hasher: F)
where
    H: Ord,
    T: Ord + Clone + Debug,
    F: Fn(&T) -> H,
{
    // 桶列表
    let mut buckets: Vec<Bucket<H, T>> = Vec::new();
    // 迭代数据，将数据放入桶中
    for value in nums.iter() {
        //
        let hash = hasher(&value);
        // 对桶中数据进行二分查找并排序
        match buckets.binary_search_by(|buckt| buckt.hash.cmp(&hash)) {
            Ok(index) => buckets[index].values.push(value.clone()),
            Err(index) => buckets.insert(index, Bucket::new(hash, value.clone())),
        }
    }
    // 合并桶中的数据
    let ret = buckets
        .into_iter()
        .flat_map(|mut bucket| {
            bucket.values.sort();
            bucket.values
        })
        .collect::<Vec<T>>();

    nums.clone_from_slice(&ret);
}

fn main() {
    let mut nums = [88999, -99, 66, 33, 123, 45, -890, 10, 18];
    println!("原始值：{:?}", &nums);
    // 需要确保数值越大hash越大，不能用取余符号
    bucket_sort(&mut nums, |v| v / 11);
    println!("排序结果:{:?}", nums);
}
