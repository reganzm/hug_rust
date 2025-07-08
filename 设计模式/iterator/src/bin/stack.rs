//! 实现栈数据结构并为栈实现三种不同类型的迭代器

#[derive(Debug)]
struct Stack<T> {
    size: usize,
    data: Vec<T>,
}
impl<T> Stack<T> {
    // 初始化栈
    fn new() -> Self {
        Self {
            size: 0,
            data: vec![],
        }
    }
    // 判断栈是否为空
    fn is_empty(&self) -> bool {
        self.size == 0
    }
    // 栈的长度
    fn len(&self) -> usize {
        self.size
    }
    // 清空栈
    fn clear(&mut self) {
        self.data.clear();
        self.size = 0;
    }
    // push元素
    fn push(&mut self, value: T) {
        self.data.push(value);
        self.size += 1;
    }
    // pop弹出元素
    fn pop(&mut self) -> Option<T> {
        self.size -= 1;
        self.data.pop()
    }
    // 返回栈顶数据的不可变引用
    fn peek(&self) -> Option<&T> {
        self.data.get(self.size - 1)
    }
    // 返回栈顶数据的可变引用
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.get_mut(self.size - 1)
    }
    // 将集合转换为迭代器
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // 将集合转换为不可变引用迭代器
    fn iter(&self) -> Iter<T> {
        let mut iter = Iter(Vec::new());
        for item in self.data.iter() {
            iter.0.push(item);
        }
        iter
    }

    // 将集合转换为可变引用迭代器
    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iter = IterMut(Vec::new());
        for item in self.data.iter_mut() {
            iter.0.push(item);
        }
        iter
    }
}

// 将Stack变成三种不同的迭代器

// 消费集合数据变成一个迭代器
struct IntoIter<T>(Stack<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}

// 不可变引用迭代器
struct Iter<'a, T: 'a>(Vec<&'a T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// 可变引用迭代器
struct IterMut<'a, T: 'a>(Vec<&'a mut T>);
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

fn main() {
    let mut s = Stack::new();
    s.push(100);
    s.push(200);
    s.push(99);
    s.push(888);

    // 不可变引用迭代器
    let sum = s.iter().sum::<i32>();
    println!("sum:{sum}"); // 输出1287

    // 可变引用迭代器
    // 输出ele:8880 ele:990 ele:2000 ele:1000
    s.iter_mut()
        .map(|item| *item * 10)
        .for_each(|ele| print!("ele:{ele} "));
    // 通过可变引用修改值
    for i in s.iter_mut() {
        *i += 10;
    }
    println!("stack:{:?}", s); // 输出stack:Stack { size: 4, data: [110, 210, 109, 898] }

    // 将消费stack生成迭代器
    let sum = s.into_iter().sum::<i32>();
    println!("sum:{sum}"); // 输出1327
    // stack已被消费，所有权已经转移，下面代码会报编译错误
    // println!("stack:{:?}",s);
}
