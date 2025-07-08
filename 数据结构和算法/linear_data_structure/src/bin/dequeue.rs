//! 双端队列

#[derive(Debug)]
pub struct Dequeue<T> {
    cap: usize,
    datas: Vec<T>,
}

impl<T> Dequeue<T> {
    pub fn new(cap: usize) -> Self {
        Self {
            cap: cap,
            datas: Vec::with_capacity(cap),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.datas.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.cap
    }

    pub fn len(&self) -> usize {
        self.datas.len()
    }

    pub fn clear(&mut self) {
        self.datas = Vec::with_capacity(self.cap)
    }

    // 在队首添加元素(vec的末尾为队首)
    pub fn add_front(&mut self, val: T) -> Result<(), String> {
        if self.is_full() {
            return Err("没有空间".to_string());
        }
        self.datas.push(val);
        Ok(())
    }

    // 从队首移除元素
    pub fn remove_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.datas.pop()
        }
    }

    // 在队尾插入元素(vec的开始为队尾)
    pub fn add_rear(&mut self, val: T) -> Result<(), String> {
        if self.is_full() {
            return Err("没有空间".to_string());
        }
        self.datas.insert(0, val);
        Ok(())
    }
    // 从队尾移除元素
    pub fn remove_rear(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.datas.remove(0))
        }
    }

    // 实现迭代功能
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    fn iter(&self) -> Iter<T> {
        let mut iter = Iter(vec![]);
        for i in self.datas.iter() {
            iter.0.push(i);
        }
        iter
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        let mut iter_mut = IterMut(vec![]);

        for i in self.datas.iter_mut() {
            iter_mut.0.push(i);
        }
        iter_mut
    }
}

struct IterMut<'a, T>(Vec<&'a mut T>);
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

struct Iter<'a, T>(Vec<&'a T>);
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.len() == 0 {
            None
        } else {
            Some(self.0.remove(0))
        }
    }
}

struct IntoIter<T>(Dequeue<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            self.0.remove_front()
        }
    }
}

fn main() {
    fn test1() {
        let mut d = Dequeue::new(2);
        let _r3 = d.add_rear(3);
        let _r4 = d.add_rear(4);
        if let Err(error) = d.add_front(5) {
            println!("add_front error: {error}");
        }
        println!("{:?}", d);
        match d.remove_rear() {
            Some(data) => println!("remove rear data {data}"),
            None => println!("empty deque"),
        }
        match d.remove_front() {
            Some(data) => println!("remove front data {data}"),
            None => println!("empty deque"),
        }
        println!("empty: {}, len: {}", d.is_empty(), d.len());
        println!("full: {}, {:?}", d.is_full(), d);
        d.clear();
        println!("{:?}", d);
    }
    fn test2() {
        let mut d = Dequeue::new(2);
        let _r1 = d.add_front(1);
        let _r4 = d.add_rear(4);
        let sum1 = d.iter().sum::<i32>();
        let mut sum = 0;
        for item in d.iter_mut() {
            *item += 1;
            sum += 1;
        }
        let sum2 = d.iter().sum::<i32>();
        println!("{sum1} + {sum} = {sum2}");
        assert_eq!(7, d.into_iter().sum::<i32>());
    }
    test1();
    test2();
}
