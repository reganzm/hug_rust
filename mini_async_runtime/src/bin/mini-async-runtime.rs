use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::Future,
        pin::Pin,
        sync::{
            mpsc::{self, Receiver, Sender},
            Arc, Mutex,
        },
        task::{Context, Poll, Waker},
        thread,
        time::{Duration, SystemTime},
    },
};

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    async fn hello() {
        thread::sleep(Duration::from_secs(1));
        let current_time = SystemTime::now();
        let msg = format!("{:?} from async hello , do something .....", current_time);
    }
    spawner.spawn(
        async {
            hello().await;
        },
        "任务1",
    );
    spawner.spawn(
        async {
            hello().await;
            hello().await;
        },
        "任务2",
    );
    spawner.spawn(
        async {
            hello().await;
            TimerFuture::new(Duration::new(1, 0)).await;
            hello().await;
        },
        "任务3",
    );
    spawner.spawn(
        async {
            hello().await;
        },
        "任务4",
    );
    // 关闭task_sender端，结束executor中的while let循环
    drop(spawner);

    executor.run();
}

/// Task，封装了future且拥有发送端，等待执行器去`poll`
struct Task {
    // 进行中的Future，在未来的某个时间点会被完成
    // BoxFuture<'a, T> = Pin<alloc::boxed::Box<dyn Future<Output = T> + Send + 'a>>;
    future: Mutex<BoxFuture<'static, ()>>,
    // 可以将该任务自身放回到任务通道中，等待执行器的poll
    task_sender: Sender<Arc<Task>>,
    task_name: String,
}

/// 为Task实现ArcWake trait
/// ArcWake特征需要Task能在thread之间安全的共享
/// 要么加锁，要么使用Mutex
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("发送任务失败");
    }
}

/// 任务执行器，负责从通道中接收任务然后执行
struct Executor {
    task_receiver: Receiver<Arc<Task>>,
}

/// `Spawner`负责创建新的`Future`然后将它发送到任务通道中
struct Spawner {
    task_sender: Sender<Arc<Task>>,
}

// 为Spawner实现spawn方法
impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send, task_name: &str) {
        // boxed方法需要引入futures::future::FutureExt
        // Pin住future
        let future: std::pin::Pin<Box<dyn Future<Output = ()> + Send>> = future.boxed();
        // 使用task封装future
        let task = Arc::new(Task {
            future: Mutex::new(future),
            task_sender: self.task_sender.clone(),
            task_name: String::from(task_name),
        });
        // 发送task到channel
        self.task_sender.send(task).expect("发送消息失败");
    }
}

/// 为Executor实现run方法
impl Executor {
    fn run(&self) {
        // 使用while let循环从channel中拿task
        while let Ok(task) = self.task_receiver.recv() {
            // 使用waker_ref方法生成WakerRef，需要task是Arc<_>类型
            // 需要task实现ArcWake trait
            let waker = waker_ref(&task);
            // 构建context
            let context = &mut Context::from_waker(&*waker);
            let mut future_slot = task.future.lock().unwrap();
            let status = future_slot.as_mut().poll(context);
            let current_time = SystemTime::now();
            println!(
                "{:?} 任务状态:{:?} 任务名称:{:?}",
                current_time, status, &task.task_name
            )
        }
    }
}

/// 辅助方法，创建spawner和executor
fn new_executor_and_spawner() -> (Executor, Spawner) {
    let (task_sender, task_receiver) = mpsc::channel::<Arc<Task>>();
    (Executor { task_receiver }, Spawner { task_sender })
}

// 实现一个Future，调用wake方法通知executor执行
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// 在Future和等待的线程间共享状态
struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            let current_time = SystemTime::now();
            //println!("{:?} task ready", current_time);
            Poll::Ready(())
        } else {
            // poll方法将waker传入Future中
            shared_state.waker = Some(cx.waker().clone());
            let current_time = SystemTime::now();
            //println!("{:?} task pending", current_time);
            Poll::Pending
        }
    }
}

impl TimerFuture {
    /// 创建一个新的`TimerFuture`，在指定的时间结束后，该`Future`可以完成
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // 创建新线程
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            let current_time = SystemTime::now();
            println!("{:?} 任务睡眠中 ...", current_time);
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                let current_time = SystemTime::now();
                println!("{:?} 任务结束通知executor ...", current_time);
                waker.wake_by_ref()
            }
        });

        TimerFuture { shared_state }
    }
}
