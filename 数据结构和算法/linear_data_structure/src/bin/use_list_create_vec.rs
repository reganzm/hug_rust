//!使用链表创建vec

use std::fmt::Debug;

type NodeRef<T> = Option<Box<Node<T>>>;
#[derive(Debug)]
struct Node<T> {
    ele: T,
    next: NodeRef<T>,
}

impl<T> Node<T> {
    fn new(ele: T) -> Self {
        Self {
            ele: ele,
            next: None,
        }
    }
}

// 基于链表实现的Vec
#[derive(Debug)]
struct LVec<T> {
    size: usize,
    head: NodeRef<T>,
}

impl<T: Debug + Copy> LVec<T> {
    fn new() -> Self {
        Self {
            size: 0,
            head: None,
        }
    }

    fn is_empty(&self) -> bool {
        0 == self.size
    }

    fn len(&self) -> usize {
        self.size
    }
    fn clear(&mut self) {
        self.size = 0;
        self.head = None
    }

    fn push(&mut self, elem: T) {
        let node = Node::new(elem);
        // 判断是否为空，如果为空则作为头结点
        if self.is_empty() {
            self.head = Some(Box::new(node));
        } else {
            // 找到链表最后一个节点并插入
            let mut current = self.head.as_deref_mut().unwrap();
            for _i in 0..self.size - 1 {
                current = current.next.as_deref_mut().unwrap();
            }
            // 插入数据
            current.next = Some(Box::new(node));
        }
        self.size += 1;
    }

    // 在当前LVec后面添加其它的LVec
    fn extend(&mut self, other: &mut Self) {
        while let Some(node) = other.head.as_deref_mut().take() {
            self.push(node.ele);
            other.head = node.next.take();
        }
        other.clear();
    }

    // 指定位置插入数据
    fn insert(&mut self, mut index: usize, elem: T) {
        // 如果指定的index大于LVec的大小，则重置为self.size
        if index > self.size {
            index = self.size
        }

        let mut node = Node::new(elem);

        // 为空，作为头节点
        if self.is_empty() {
            self.head = Some(Box::new(node));
        }
        // 在头部位置插入数据
        else if index == 0 {
            node.next = self.head.take();
            self.head = Some(Box::new(node));
        }
        // 在中间和尾部插入数据
        else {
            let mut current = self.head.as_deref_mut().unwrap();
            for _i in 0..index - 1 {
                // 找到插入位置
                current = current.next.as_deref_mut().unwrap();
            }
            node.next = current.next.take();
            current.next = Some(Box::new(node));
        }
    }

    // 从尾部弹出数据
    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.remove(self.size - 1)
        }
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        } else {
            let mut node;
            if index == 0 {
                node = self.head.take().unwrap();
                self.head = node.next.take();
            } else {
                let mut current = self.head.as_deref_mut().unwrap();
                for _i in 0..index - 1 {
                    current = current.next.as_deref_mut().unwrap();
                }
                node = current.next.take().unwrap();
                current.next = node.next.take();
            }
            self.size -= 1;
            Some(node.ele)
        }
    }

    // 实现迭代器功能

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        Iter(self.head.as_deref())
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut(self.head.as_deref_mut())
    }

    // 打印输出
    fn display(&self){
        println!("--------------------");
        if self.is_empty(){
            println!("Empty");
        }else{
            for i in self.iter(){
                println!("ele:{:?}",i);
            }
        }
        println!("--------------------");
    }
}
struct IterMut<'a, T: 'a>(Option<&'a mut Node<T>>);
impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = node.next.as_deref_mut();
            &mut node.ele
        })
    }
}

struct Iter<'a, T: 'a>(Option<&'a Node<T>>);
impl<'a, T: Debug + Copy> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(|node| {
            self.0 = node.next.as_deref();
            &node.ele
        })
    }
}

struct IntoIter<T>(LVec<T>);
impl<T: Debug + Copy> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

fn main() {

    let mut l1 = LVec::new();
    l1.push(30);
    l1.push(40);
    l1.push(50);
    l1.display();

    let mut l2 = LVec::new();
    l2.push(999);
    l2.push(888);

    l1.extend(&mut l2);

    l1.display();
    for item in l1.iter_mut(){
        *item *= 100;
    }

    l1.display();



}
