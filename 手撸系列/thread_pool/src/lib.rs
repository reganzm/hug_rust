use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::SystemTime,
};

type Job = Box<dyn FnOnce() + Send>;
pub struct Threadpool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

struct Worker {
    id: usize,
    handle: Option<JoinHandle<()>>,
}

// 为Threadpool实现方法
impl Threadpool {
    pub fn new(size: usize) -> Self {
        // 线程池的数量需要大于0
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel::<Job>();
        // receiver需要在多个线程间共享所有权，使用Arc
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 1..=size {
            let receiver = receiver.clone();
            workers.push(Worker::new(id, receiver));
        }
        Threadpool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for Threadpool {
    fn drop(&mut self) {
        // 需要等待所有线程都执行完成
        // 先释放channel
        drop(self.sender.take());
        // 等待线程结束
        for worker in &mut self.workers {
            let time = SystemTime::now();
            if let Some(thread) = worker.handle.take() {
                thread.join().unwrap();
            }
            println!("{:?} shutdown worker:{}", time, worker.id);
        }
        // println
        let time = SystemTime::now();
        println!("{:?} shutdown thread pool", time);
    }
}

// 为Worker实现方法
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let handle = thread::spawn(move || {
            // 从channel拉取job执行
            // 线程需要一直运行，用loop循环
            loop {
                let job = receiver.lock().unwrap().recv();
                match job {
                    Ok(job) => {
                        let time = SystemTime::now();
                        println!("{:?} Worker {id} is working...", time);
                        job();
                    }
                    Err(_) => {
                        let time = SystemTime::now();
                        println!("{:?} Worker {id} disconnected ; exit", time);
                        break;
                    }
                }
            }
        });
        Worker {
            id,
            handle: Some(handle),
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::{Instant, SystemTime};

    use crate::Threadpool;
    #[test]
    fn test_thread_pool() {
        let pool = Threadpool::new(2);

        let f1 = || {
            let time = SystemTime::now();
            let result = 1 + 1;
            println!("{:?} result:{result}", time);
        };

        
        let f2 = || {
            let time = SystemTime::now();
            let result = 99*88*77*76;
            println!("{:?} result:{result}", time);
        };
        
        let f3 = || {
            let time = SystemTime::now();
            let result = 1000*9899;
            println!("{:?} result:{result}", time);
        };
        pool.execute(f1);
        pool.execute(f2);
        pool.execute(f3);
    }
}
