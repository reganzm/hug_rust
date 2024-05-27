//!实现队列
#[derive(Debug)]
pub struct Queue<T> {
    cap: usize,
    datas: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new(size: usize) -> Self {
        Self {
            cap: size,
            datas: Vec::with_capacity(size),
        }
    }
    // 判断队列元素个数
    pub fn len(&self) -> usize {
        self.datas.len()
    }
    // 判断队列是否为空
    pub fn is_empty(&self) -> bool {
        0 == self.len()
    }
    // 判断队列是否满
    pub fn is_full(&self) -> bool {
        self.len() == self.cap
    }

    // 清空队列
    pub fn clear(&mut self) {
        self.datas.clear();
        self.datas = Vec::with_capacity(self.cap);
    }

    // 入队
    pub fn enqueue(&mut self, val: T) -> Result<(), String> {
        if self.is_full() {
            return Err("没有空间".to_string());
        } else {
            self.datas.insert(0, val);
        }
        Ok(())
    }

    // 出队
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.datas.pop()
        }
    }

    // 将队列转换为迭代器(消耗队列)
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // 将队列转换为引用迭代器
    pub fn iter(&self) -> Iter<T> {
        let mut iter = Iter(Vec::new());
        for i in self.datas.iter() {
            iter.0.push(i);
        }
        iter
    }

    // 将队列转换为可变引用迭代器
    pub fn iter_mut(&mut self) -> IterMut<T> {
        let mut iter_mut = IterMut(Vec::new());
        for i in self.datas.iter_mut() {
            iter_mut.0.push(i);
        }
        iter_mut
    }
}

struct IterMut<'a, T: 'a>(Vec<&'a mut T>);
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.len() == 0 {
            None
        } else {
            Some(self.0.remove(0))
        }
    }
}

struct IntoIter<T>(Queue<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.dequeue()
    }
}

struct Iter<'a, T: 'a>(Vec<&'a T>);
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if 0 != self.0.len() {
            Some(self.0.remove(0))
        } else {
            None
        }
    }
}

fn main() {
    basic();
    iter();
    fn basic() {
        let mut q = Queue::new(2);
        let _r1 = q.enqueue(1);
        let _r3 = q.enqueue(3);
        if let Err(error) = q.enqueue(5) {
            println!("Enqueue error: {error}");
        }
        if let Some(data) = q.dequeue() {
            println!("dequeue data: {data}");
        } else {
            println!("empty queue");
        }
        println!("empty: {}, len: {}", q.is_empty(), q.len());
        println!("full: {}", q.is_full());
        println!("q: {:?}", q);
        q.clear();
        println!("{:?}", q);
    }
    fn iter() {
        let mut q = Queue::new(3);
        let _r1 = q.enqueue(1);
        let _r3 = q.enqueue(3);
        let sum1 = q.iter().sum::<i32>();
        let mut sum = 0;
        for item in q.iter_mut() {
            *item += 1;
            sum += 1;
        }
        let sum2 = q.iter().sum::<i32>();
        println!("{sum1} + {sum} = {sum2}");
        println!("sum = {}", q.into_iter().sum::<i32>());
    }
}
