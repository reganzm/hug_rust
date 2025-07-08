//! 约瑟夫环(最后剩下谁，烫手的山芋游戏）
mod queue;
use queue::Queue;

fn josephus_circle(names: Vec<&str>, num: usize) -> &str {
    // 将参与游戏的名字入队列
    let mut q = Queue::new(names.len());
    for name in names {
        let _i = q.enqueue(name);
    }
    while q.len() > 1 {
        // 数num-1次，相当于做num-1次出队入队操作,第num次的人将被移除
        for _i in 0..num {
            let name = q.dequeue().unwrap();
            let _i = q.enqueue(name);
        }
        let killed_name = q.dequeue().unwrap();
        println!("{killed_name} is out!!");
    }
    // 返回剩下的最后一个人
    q.dequeue().unwrap()
}

fn main() {
    let names = vec![
        "小花", "小黑", "小白", "小黄", "小含", "小满", "小就", "小来",
    ];
    let survivor = josephus_circle(names, 7);
    println!("最后的幸存者:{survivor}");
}
