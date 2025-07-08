use std::{collections::HashMap, rc::Rc};

// 电视 Receiver
struct Tv;
impl Tv {
    fn on(&self) {
        println!("打开电视....")
    }
    fn off(&self) {
        println!("关闭电视....")
    }
}

// 命令接口
trait Command {
    fn execute(&self);
}

// 具体命令：打开电视
struct OnTvCommand {
    tv: Rc<Tv>,
}
impl OnTvCommand {
    fn new(tv: Rc<Tv>) -> Self {
        Self { tv }
    }
}
impl Command for OnTvCommand {
    fn execute(&self) {
        self.tv.on();
    }
}

// 具体命令：关闭电视
struct OffTvCommand {
    tv: Rc<Tv>,
}
impl OffTvCommand {
    fn new(tv: Rc<Tv>) -> Self {
        Self { tv }
    }
}
impl Command for OffTvCommand {
    fn execute(&self) {
        self.tv.off();
    }
}
// 遥控器：Invoker
struct RemoteControl {
    commands: HashMap<String, Rc<dyn Command>>,
}
impl RemoteControl {
    fn new(tv: Rc<Tv>) -> Self {
        let mut commands: HashMap<String, Rc<dyn Command>> = HashMap::new();
        commands.insert("on".to_string(), Rc::new(OnTvCommand::new(tv.clone())));
        commands.insert("off".to_string(), Rc::new(OffTvCommand::new(tv.clone())));
        Self { commands }
    }
    // 打开电视
    fn on_tv(&self) {
        self.commands.get("on").map(|command| command.execute());
    }
    // 关闭电视
    fn off_tv(&self) {
        self.commands.get("off").map(|command| command.execute());
    }
}

fn main() {
    let remote_control = RemoteControl::new(Rc::new(Tv));
    remote_control.on_tv();
    remote_control.off_tv();
}
