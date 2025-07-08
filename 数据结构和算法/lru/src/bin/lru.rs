//! LRU 最近最少使用缓存淘汰算法
//! 基于LRUList数据结构：双向链表且头节点指向尾节点

use std::{collections::HashMap, mem::swap};

// 可变裸指针，方便通过裸指针修改
type LRUHandle<T> = *mut LRUNode<T>;

#[derive(Debug)]
struct LRUNode<T> {
    next: Option<Box<LRUNode<T>>>,
    prev: Option<LRUHandle<T>>,
    data: Option<T>,
}

#[derive(Debug)]
struct LRUList<T> {
    head: LRUNode<T>,
    count: usize,
}

impl<T> LRUList<T> {
    /// 初始化LRUList
    fn new() -> Self {
        LRUList {
            head: LRUNode {
                next: None,
                prev: None,
                data: None,
            },
            count: 0,
        }
    }

    /// 在首部插入值
    fn insert(&mut self, data: T) -> LRUHandle<T> {
        // 长度+1
        self.count += 1;
        // 判断插入的位置：尾部和首部
        let mut node = Box::new(LRUNode {
            next: None,
            prev: Some(&mut self.head),
            data: Some(data),
        });
        let newp = node.as_mut() as *mut LRUNode<T>;
        // 没有节点
        if self.head.next.is_none() {
            // head的前驱节点指向最后加入的节点
            self.head.prev = Some(newp);
        }
        // 在头部加入
        else {
            self.head.next.as_mut().unwrap().prev = Some(newp);
            node.next = self.head.next.take();
        }
        self.head.next = Some(node);
        newp
    }

    /// 从双向链表中删除数据
    fn remove(&mut self, handle: LRUHandle<T>) -> T {
        unsafe {
            let d = (*handle).data.take().unwrap();
            let mut current = (*(*handle).prev.unwrap()).next.take().unwrap();
            let pre = current.prev.unwrap();
            // 如果不是尾部节点
            if current.next.is_some() {
                current.next.as_mut().unwrap().prev = current.prev.take();
            }
            (*pre).next = current.next.take();
            // 长度减1
            self.count -= 1;
            d
        }
    }

    /// 删除最后一个数据
    fn remove_last(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        let last = unsafe {
            // 找到最后节点的前一个节点，拿走next的所有权
            (*(*self.head.prev.unwrap()).prev.unwrap()).next.take()
        };

        if let Some(mut last_node) = last {
            // 修改head的prev指向last_node的前一个节点
            self.head.prev = last_node.prev;
            self.count -= 1;
            // 拿走last_node数据所有权
            last_node.data.take()
        } else {
            None
        }
    }

    /// 重新在首部插入数据
    fn reinsert_front(&mut self, handler: LRUHandle<T>) {
        unsafe {
            // 当前节点的前向节点指针
            let prevp = (*handler).prev.unwrap();

            // 修改指针指向
            // 1.当前节点处于中间
            if let Some(next) = (*handler).next.as_mut() {
                next.prev = Some(prevp);
            }
            // 2.当前节点处于尾部
            else {
                self.head.prev = Some(prevp);
            }

            // 3.交换指向
            swap(&mut (*prevp).next, &mut (*handler).next); // prepv-handler.next;handler.next=self
            swap(&mut (*handler).next, &mut self.head.next); //head.next = handler;handler.next =  old head.next

            if let Some(ref mut next) = (*handler).next {
                (*handler).prev = next.prev;
                next.prev = Some(handler);
            } else {
                self.head.prev = Some(handler);
            }
        }
    }

    fn count(&self) -> usize {
        self.count
    }
}

/// 基于LRUList构建Cache
/// cache基于cachekey快速查找
type CacheKey = [u8; 16];
// 记录value和key在链表中的指针
type CacheEntry<T> = (T, LRUHandle<CacheKey>);

#[derive(Debug)]
struct Cache<T> {
    // 记录cache key的关系
    list: LRUList<CacheKey>,
    // 记录key和value的关系
    map: HashMap<CacheKey, CacheEntry<T>>,
    // cache容量
    cap: usize,
}

impl<T> Cache<T> {
    fn new(cap: usize) -> Self {
        Cache {
            list: LRUList::new(),
            map: HashMap::with_capacity(cap * 2),
            cap,
        }
    }

    fn insert(&mut self, key: &CacheKey, data: T) {
        // 先移除最近最少使用数据再插入
        if self.list.count >= self.cap {
            println!("缓存已满，删除最近最少使用数据...");
            if let Some(remove_key) = self.list.remove_last() {
                self.map.remove(&remove_key).unwrap();
            } else {
                println!("删除数据失败");
            }
        }

        let handle = self.list.insert(*key);
        self.map.insert(*key, (data, handle));
    }

    fn get(&mut self, key: &CacheKey) -> Option<&T> {
        match self.map.get(key) {
            None => None,
            Some((data, handle)) => {
                self.list.reinsert_front(*handle);
                Some(data)
            }
        }
    }

    fn remove(&mut self, key: &CacheKey) -> Option<T> {
        match self.map.remove(key) {
            None => None,
            Some((data, handle)) => {
                self.list.remove(handle);
                Some(data)
            }
        }
    }

    fn count(&self) -> usize {
        self.list.count()
    }
}

fn gen_key(a: u8, b: u8, c: u8) -> CacheKey {
    [a, b, c, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
}

fn main() {
    let mut cache: Cache<i32> = Cache::new(3);

    let key1 = gen_key(1, 2, 3);
    let key2 = gen_key(1, 2, 5);
    let key3 = gen_key(1, 2, 7);
    let key4 = gen_key(1, 2, 9);
    let key5 = gen_key(1, 2, 11);
    let key6 = gen_key(1, 2, 12);

    cache.insert(&key1, 123);
    cache.insert(&key2, 125);
    cache.insert(&key3, 127);
    cache.insert(&key4, 129);
    cache.insert(&key5, 1211);
    cache.insert(&key6, 1212);

    println!("cache 数据条数:{}", cache.count());
    println!("删除key5:{:?}", cache.remove(&key5));
    println!("cache 数据条数:{}", cache.count());

    cache.get(&key6);
    println!(
        "最近访问元素:{:?}",
        &cache.list.head.next.as_ref().unwrap().data
    );
    cache.get(&key4);
    println!(
        "最近访问元素:{:?}",
        &cache.list.head.next.as_ref().unwrap().data
    );
}
