//!基于链表实现的栈数据结构

mod linklist;
use std::fmt::Debug;
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
impl<T> Node<T> {
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

impl<T> LStack<T> {
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
}

fn main() {}
