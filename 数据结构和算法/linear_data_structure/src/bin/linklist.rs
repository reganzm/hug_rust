//! 链表

use std::fmt::Debug;
// 节点引用类型
type NodeRef<T> = Option<Box<Node<T>>>;
#[derive(Debug)]
struct List<T: Debug> {
    // 链表大小
    size: usize,
    // 头结点
    head: NodeRef<T>,
}
// 链表节点
#[derive(Debug)]
struct Node<T> {
    // 数据
    elem: T,
    // 下一个节点
    next: NodeRef<T>,
}

impl<T: Debug> List<T> {
    fn new() -> Self {
        Self {
            size: 0,
            head: None,
        }
    }

    // 判断链表是否为空
    fn is_empty(&self) -> bool {
        self.size == 0
    }
    // 链表长度
    fn len(&self) -> usize {
        self.size
    }

    // 清空链表
    fn clear(&mut self) {
        self.size = 0;
        self.head = None;
    }

    // 添加新节点
    fn push(&mut self, ele: T) {
        let node = Box::new(Node {
            elem: ele,
            next: self.head.take(),
        });
        self.head = Some(node);
        self.size += 1;
    }

    // 取出头节点数据
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.elem
        })
    }

    // 查看引用值
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    // 转换为迭代器
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        Iter(self.head.as_deref())
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut(self.head.as_deref_mut())
    }

    fn display(&self) {
        for i in self.iter() {
            println!("value:{:#?}", i);
        }
    }
}

// 释放链表
impl<T: Debug> Drop for List<T> {
    fn drop(&mut self) {
        let mut node_ref = self.head.take();
        while let Some(mut node) = node_ref {
            // 获取所有权并释放数据
            node_ref = node.next.take();
        }
    }
}

struct IterMut<'a, T: 'a>(Option<&'a mut Node<T>>);
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

struct Iter<'a, T: 'a>(Option<&'a Node<T>>);
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(|node| {
            self.0 = node.next.as_deref();
            &node.elem
        })
    }
}

#[derive(Debug)]
struct IntoIter<T: Debug>(List<T>);
impl<T: Debug> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

fn main() {
    let mut list = List::new();
    list.push(10);
    list.push(20);
    list.push(30);
    println!("原始值....");
    list.display();
    let iter_mut = list.iter_mut();
    for i in iter_mut {
        // 元素乘以100
        *i *= 100
    }
    println!("改变之后...");
    list.display();
}
