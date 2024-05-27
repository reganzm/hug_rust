//!基于链表实现的栈数据结构

mod linklist;
use std::fmt::{Debug, Display};
// 节点引用类型
type NodeRef<T> = Option<Box<Node<T>>>;

// 链表节点
#[derive(Debug)]
struct Node<T> {
    // 数据
    elem: T,
    // 下一个节点
    next: NodeRef<T>,
}
impl<T: Debug + Display> Node<T> {
    fn new(data: T) -> Self {
        Self {
            elem: data,
            next: None,
        }
    }
}
struct LStack<T> {
    size: usize,
    top: NodeRef<T>,
}

impl<T: Debug + Display> LStack<T> {
    fn new() -> Self {
        Self { size: 0, top: None }
    }

    fn is_empty(&self) -> bool {
        0 == self.size
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.size = 0;
        self.top = None;
    }

    fn push(&mut self, val: T) {
        let mut node = Node::new(val);
        node.next = self.top.take();
        self.top = Some(Box::new(node));
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            let node = *node;
            self.top = node.next;
            self.size -= 1;
            node.elem
        })
    }

    fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| &node.elem)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.top.as_deref_mut().map(|node| &mut node.elem)
    }

    // 迭代器
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        Iter(self.top.as_deref())
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut(self.top.as_deref_mut())
    }

    fn display(&self) {
        for i in self.iter() {
            println!("ele:{i}");
        }
    }
}

struct IterMut<'a, T>(Option<&'a mut Node<T>>);
impl<'a, T: 'a + Debug + Display> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

struct Iter<'a, T: 'a>(Option<&'a Node<T>>);
impl<'a, T: 'a + Debug + Display> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(|node| {
            self.0 = node.next.as_deref();
            &node.elem
        })
    }
}

struct IntoIter<T>(LStack<T>);

impl<T: Debug + Display> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

fn main() {
    let mut s = LStack::new();
    s.push(11);
    s.push(22);
    s.display();

    for item in s.iter_mut() {
        *item += 2024;
    }

    println!("-----------");
    s.display();
}
