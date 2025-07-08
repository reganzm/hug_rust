//! 哈希查找
//! hash函数：一种值到存储地址的映射函数，通过它直接建立值和地址的关系，达到快速查找的目的
//! hash 冲突的解决：拉链法，在冲突的槽位建立链表(或一颗红黑树)，此时时间复杂度为O(log2N)

use std::{
    collections::hash_map::DefaultHasher,
    fmt::Debug,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct HashMap<K, V> {
    // 分配的容量
    cap: usize,
    // 存储key
    slot: Vec<K>,
    // 存储value
    data: Vec<V>,
}

impl<K, V> HashMap<K, V>
where
    K: Clone + Default + PartialEq + PartialOrd + Debug + Hash,
    V: Clone + Default + PartialEq + PartialOrd + Debug + Hash,
{
    fn new(cap: usize) -> Self {
        //初始化slot和data
        let mut slot = Vec::with_capacity(cap);
        let mut data = Vec::with_capacity(cap);

        for _ in 0..cap {
            slot.push(Default::default());
            data.push(Default::default());
        }

        HashMap { cap, slot, data }
    }

    fn len(&self) -> usize {
        let mut len = 0;
        for i in self.slot.iter() {
            // 不等于0表示有数据，长度+1
            if *i != Default::default() {
                len += 1;
            }
        }
        len
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn clear(&mut self) {
        let mut slot = Vec::with_capacity(self.cap);
        let mut data = Vec::with_capacity(self.cap);
        for _ in 0..self.cap {
            slot.push(Default::default());
            data.push(Default::default());
        }
        self.slot = slot;
        self.data = data;
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let result = hasher.finish() as usize;
        result % self.cap
    }

    fn rehash(&self, pos: usize) -> usize {
        (pos + 1) % self.cap
    }

    fn insert(&mut self, key: K, value: V) {
        if key == Default::default() {
            panic!("{:?}不能作为key", key);
        }

        let pos = self.hash(&key);

        // 槽位没有数据，直接插入
        if self.slot[pos] == Default::default() {
            self.slot[pos] = key;
            self.data[pos] = value;
        } else {
            // 要插入的槽位有数据，线性探测下一个可以插入的位置
            let mut next = self.rehash(pos);
            while self.slot[next] != Default::default() && key != self.slot[next] {
                next = self.rehash(next);
                // 一直寻找知道没有槽位为止
                if next == pos {
                    println!("WARN:槽位已用完");
                    return;
                }
            }
            // 在找到的槽位中插入数据
            if self.slot[next] == Default::default() {
                self.slot[next] = key;
                self.data[next] = value;
            } else {
                self.data[next] = value;
            }
        }
    }

    fn remove(&mut self, key: K) -> Option<V> {
        if key == Default::default() {
            panic!("key:{:?}不能是默认值！", key);
        }
        let pos = self.hash(&key);
        if self.slot[pos] == Default::default() {
            None
        } else if key == self.slot[pos] {
            self.slot[pos] = Default::default();
            let data = Some(self.data[pos].clone());
            self.data[pos] = Default::default();
            data
        } else {
            let mut data: Option<V> = None;
            let mut stop = false;
            let mut found = false;
            let mut current = pos;
            while self.slot[current] != Default::default() && !found && !stop {
                if key == self.slot[current] {
                    found = true;
                    self.slot[current] = Default::default();
                    data = Some(self.data[current].clone());
                    self.data[current] = Default::default();
                } else {
                    current = self.rehash(current);
                    if current == pos {
                        stop = true;
                    }
                }
            }
            data
        }
    }

    fn get_pos(&self, key: K) -> usize {
        if key == Default::default() {
            panic!("key:{:?}不能是默认值！", key);
        }

        let pos = self.hash(&key);
        let mut stop = false;
        let mut found = false;
        let mut current = pos;

        while self.slot[current] != Default::default() && !found && !stop {
            if key == self.slot[current] {
                found = true;
            } else {
                current = self.rehash(current);
                if current == pos {
                    stop = true;
                }
            }
        }
        current
    }

    fn get(&self, key: K) -> Option<&V> {
        let pos = self.get_pos(key);
        self.data.get(pos)
    }

    fn get_mut(&mut self, key: K) -> Option<&mut V> {
        let pos = self.get_pos(key);
        self.data.get_mut(pos)
    }

    fn contains(&self, key: K) -> bool {
        if key == Default::default() {
            panic!("key:{:?}不能是默认值！", key);
        }
        self.slot.contains(&key)
    }

    fn iter(&self) -> Iter<V> {
        let mut iter = Iter { datas: Vec::new() };
        for item in self.data.iter() {
            iter.datas.push(item);
        }
        iter
    }

    fn iter_mut(&mut self) -> IterMut<V> {
        let mut iter = IterMut { datas: Vec::new() };
        for item in self.data.iter_mut() {
            iter.datas.push(item);
        }
        iter
    }
}

struct Iter<'a, V: 'a> {
    datas: Vec<&'a V>,
}
impl<'a, V> Iterator for Iter<'a, V> {
    type Item = &'a V;
    fn next(&mut self) -> Option<Self::Item> {
        self.datas.pop()
    }
}

struct IterMut<'a, V: 'a> {
    datas: Vec<&'a mut V>,
}
impl<'a, V> Iterator for IterMut<'a, V> {
    type Item = &'a mut V;
    fn next(&mut self) -> Option<Self::Item> {
        self.datas.pop()
    }
}

fn main() {
    let mut map = HashMap::new(11);
    map.insert("小黑", 10);
    map.insert("小白", 20);
    map.insert("小宏", 30);
    map.insert("小红", 40);
    map.insert("小洪", 50);
    map.insert("小李", 60);
    map.insert("小样", 70);
    map.insert("小阳", 80);
    map.insert("小王", 100);
    println!("有小王吗:{}", map.contains("小王"));
    println!("有小王吧:{}", map.contains("小王吧"));
    println!("小王成绩:{}", map.get("小王").unwrap());
    // 修改小王成绩
    *map.get_mut("小王").unwrap() = 99;
    println!("修改后小王成绩:{}", map.get("小王").unwrap());

    println!(
        "map 长度:{} 容量:{} 负载因子:{}",
        map.len(),
        map.cap,
        map.len() as f32 / map.cap as f32
    );

    // 删除小黑
    map.remove("小黑");

    // 迭代
    let iter = map.iter();
    for i in iter {
        println!("score:{i}");
    }


    println!("数据个数:{}",map.len());
}
