// 定义状态，不同的操作会影响状态，同时状态的改变又会影响对象和流程
trait State {
    fn handle(&self, context: &mut Context);
    fn get_temperature(&self) -> f32;
}

struct Context {
    state: Option<Box<dyn State>>,
}

impl Context {
    fn set_state(&mut self, state: Box<dyn State>) {
        self.state = Some(state);
    }
    fn request(&mut self) {
        self.state.take().as_mut().map(|s|s.handle(self));
    }
}

struct Spring {
    temperature: f32,
}
impl State for Spring {
    fn handle(&self, context: &mut Context) {
        println!(
            "春天到了,万物又到了繁殖的季节....当前温度:{}",
            self.get_temperature()
        );
        context.set_state(Box::new(Summer{temperature:33.2}));
    }
    fn get_temperature(&self) -> f32 {
        self.temperature
    }
}

struct Summer {
    temperature: f32,
}
impl State for Summer {
    fn handle(&self, context: &mut Context) {
        println!(
            "夏天到了,西瓜已经熟了....当前温度:{}",
            self.get_temperature()
        );
        context.set_state(Box::new(Autumn{temperature:23.1}));
    }
    fn get_temperature(&self) -> f32 {
        self.temperature
    }
}

struct Autumn {
    temperature: f32,
}
impl State for Autumn {
    fn handle(&self, context: &mut Context) {
        println!(
            "秋天到了,稻子已经黄了....当前温度:{}",
            self.get_temperature()
        );
        context.set_state(Box::new(Winter{temperature:-10.2}));
    }
    fn get_temperature(&self) -> f32 {
        self.temperature
    }
}

struct Winter {
    temperature: f32,
}
impl State for Winter {
    fn handle(&self, context: &mut Context) {
        println!("冬天来了,开始下雪....当前温度:{}", self.get_temperature());
        context.set_state(Box::new(Spring{temperature:15.2}));
    }
    fn get_temperature(&self) -> f32 {
        self.temperature
    }
}

fn main() {
    let mut season = Context {
        state: Some(Box::new(Winter { temperature: -2.5 })),
    };
    season.request();
    season.request();
    season.request();
    season.request();
    season.request();
    season.request();
    season.request();
    season.request();
    season.request();
    season.request();
    println!(".");
    println!(".");
    println!(".");
    println!(".");
    println!(".");
    println!(".");
    println!("一年四季，周而复始....");
}
