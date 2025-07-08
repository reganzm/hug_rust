//! 使用hash函数+BitSet实现 布隆过滤器

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[path = "bitset.rs"]
mod bitset;

use bitset::BitSet;

pub struct BloomFilter {
    pub k: usize, // 哈希函数的个数
    pub bits: BitSet,
}

// 根据key，产生5个哈希值
fn hash<T: Hash + ?Sized>(key: &T) -> [u64; 5] {
    let mut hasher = DefaultHasher::new();

    key.hash(&mut hasher);

    let v0 = hasher.finish();

    (v0 & 0xF).hash(&mut hasher);
    let v1 = hasher.finish();

    (v0 & 0xFF).hash(&mut hasher);
    let v2 = hasher.finish();

    (v0 & 0xFFF).hash(&mut hasher);
    let v3 = hasher.finish();

    (v0 & 0xFFFF).hash(&mut hasher);
    let v4 = hasher.finish();
    [v0, v1, v2, v3, v4]
}

impl BloomFilter {
    /// 创建一个能容纳capacity个bit的过滤器，使用k个哈希函数
    pub fn new(capacity: usize, k: usize) -> Self {
        Self {
            k: k,
            bits: BitSet::new(capacity),
        }
    }

    /// 将key添加到bitset
    pub fn add<T: Hash + ?Sized>(&mut self, key: &T) {
        let hashs = hash(key);
        for i in 0..self.k {
            let pos = self.location(&hashs, i);
            self.bits.set(pos);
        }
    }

    /// 计算添加的位置
    fn location(&self, hashs: &[u64; 5], i: usize) -> usize {
        // 按照某种规则(可自定义)从hashs中找到第一个hash值
        let p1 = hashs[((i + i % 2) % 4) / 2 + 3];
        // 找到第二个hash值和第一个hash值相加
        println!("p1:{p1}");
        let (p2, _) = hashs[i % 2].overflowing_add(p1 / (i + 1) as u64);
        // 防止超过bitset表示范围
        p2 as usize % self.bits.len()
    }

    /// 验证key是否存在于布隆过滤器中，如果存在返回true，否则返回false
    pub fn test<T: Hash + ?Sized>(&self, key: &T) -> bool {
        let hashs = hash(key);
        for i in 0..self.k {
            let pos = self.location(&hashs, i);

            if !self.bits.test(pos) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::BloomFilter;

    #[test]
    fn test_bloom_filter() {
        let mut b = BloomFilter::new(1024, 3);
        println!("bitset len:{}", b.bits.len());
        b.add(&999);
        b.add(&10);
        b.add("你好");
        b.add(&345);

        assert!(b.test(&999));
        assert!(b.test(&10));
        assert!(b.test("你好"));
        assert!(b.test(&345));
        println!("bitset len:{}", b.bits.len());
    }
}
