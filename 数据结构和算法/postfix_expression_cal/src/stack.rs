//! 实现栈数据结构

#[derive(Debug)]
pub struct Stack<T> {
    size: usize,  // 栈大小
    data: Vec<T>, // 栈数据，泛型T
}

impl<T> Stack<T> {
    // 初始化空栈
    pub fn new() -> Self {
        Self {
            size: 0,
            data: vec![],
        }
    }

    // 判空
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    // 栈元素个数
    pub fn len(&self) -> usize {
        self.size
    }

    // 清空栈
    pub fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    // 将数据压入栈顶
    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    // 将数据从栈顶弹出
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        } else {
            self.size -= 1;
            self.data.pop()
        }
    }

    // “偷窥”栈顶元素，返回栈顶元素的引用
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        } else {
            self.data.get(self.len() - 1)
        }
    }

    // 返回栈顶元素的可变引用
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None;
        } else {
            self.data.get_mut(self.size - 1)
        }
    }

    // 为栈实现迭代器
    // 将栈转换为迭代器
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // 获取不可变引用迭代器
    pub fn iter(&self) -> Iter<T> {
        let mut iter = Iter(Stack::new());
        for i in self.data.iter() {
            iter.0.push(i);
        }
        iter
    }

    // 获取可变引用迭代器
    pub fn iter_mut(&mut self) -> IterMut<T> {
        let mut iter_mut = IterMut(vec![]);
        for i in self.data.iter_mut() {
            iter_mut.0.push(i);
        }
        iter_mut
    }
}

struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            self.0.size -= 1;
            self.0.pop()
        }
    }
}

struct Iter<'a, T: 'a>(Stack<&'a T>);
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct IterMut<'a, T: 'a>(Vec<&'a mut T>);
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
