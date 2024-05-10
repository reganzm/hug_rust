use std::{collections::HashMap, mem, option};

trait Mediator{
    fn register_colleague(&mut self,member:Box<dyn Colleague>);
    fn notify(&self,member_name:&str,msg:String);
}
struct CenteralMediator{
    // 注册的同事名字-同事对象的对应关系
    members:HashMap<String,Box<dyn Colleague>>
}
impl Mediator for CenteralMediator{
    fn register_colleague(&mut self,member:Box<dyn Colleague>) {
        self.members.insert(member.name(),member);
    }

    fn notify(&self,member_name:&str,msg:String) {
        self.members.get(member_name).map(|m|m.action(msg.as_str()));
    }

}


trait Colleague{
    fn name(&self)->String;
    fn action(&self,msg:&str) {
        println!("msg:{msg}")
    }
    fn send(&self,mediator:&dyn Mediator,msg:&str){
        mediator.notify(self.name().as_str(), msg.to_string());
    }
}

// 闹钟
struct Alarm;
impl Colleague for Alarm{
    fn name(&self)->String {
        "Alarm".to_string()
    }
    fn send(&self,mediator:&dyn Mediator,msg:&str) {
        mediator.notify(self.name().as_str(), msg.to_string());
    }

}
// 窗帘
struct Curtain;
impl Colleague for Curtain{
    fn name(&self)->String {
        "Curtain".to_string()
    }
    fn send(&self,mediator:&dyn Mediator,msg:&str) {
        mediator.notify(self.name().as_str(), msg.to_string());
    }

}

// 灯
struct Lamp;
impl Colleague for Lamp{
    fn name(&self)->String {
        "Lamp".to_string()
    }
    fn send(&self,mediator:&dyn Mediator,msg:&str) {
        mediator.notify(self.name().as_str(), msg.to_string());
    }

}

fn main(){

    // 具体中介者对象
    let mut mediator = CenteralMediator{members:HashMap::new()};

    // 同事对象
    let alarm = Alarm;
    let curtain = Curtain;
    let lamp = Lamp;

    // 注册到中介者
    mediator.register_colleague(Box::new(alarm));
    mediator.register_colleague(Box::new(curtain));
    mediator.register_colleague(Box::new(lamp));

    mediator.members.get("Alarm").map(|m|m.send(&mediator, "闹钟开始响铃"));
    mediator.members.get("Curtain").map(|m|m.send(&mediator, "自动打开窗帘"));
    mediator.members.get("Lamp").map(|m|m.send(&mediator, "打开灯光"));
    mediator.members.get("Alarm").map(|m|m.send(&mediator, "关闭闹钟"));

}