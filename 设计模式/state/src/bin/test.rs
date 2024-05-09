use std::{borrow::BorrowMut, cell::RefCell, sync::Arc};

// 定义状态，不同的操作会影响状态，同时状态的改变又会影响对象和流程
trait State {
    fn handle(&mut self, context: &mut Context);
    fn get_temperature(&self) -> f32;
}

struct Context {
    state: Box<dyn State>,
}

impl Context {
    fn set_state(&mut self, state: Box<dyn State>) {
        self.state = state;
    }
    fn request(&mut self) {
        //let a = Arc::new(RefCell::new(self));
        self.state.handle(self.borrow_mut());
    }

}

struct Spring {
    temperature: f32,
}
impl State for Spring {
    fn handle(&mut self, context: &mut Context) {
        println!(
            "春天到了,万物又到了繁殖的季节....当前温度:{}",
            self.get_temperature()
        );
        context.set_state(Box::new(Spring{temperature:22.3}));
    }
    fn get_temperature(&self) -> f32 {
        self.temperature
    }
}

fn main() {
    let mut season = Context{state:Box::new(Spring { temperature: 15.1 })};
    season.request();
   
}
