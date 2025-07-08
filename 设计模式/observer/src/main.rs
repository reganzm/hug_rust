use std::collections::HashMap;

trait Observer {
    fn update(&mut self, temperature: f32, pressure: f32, humidity: f32);
    fn display(&self);
    fn get_id(&self) -> i32;
}

struct Baidu {
    id: i32,
    temperature: f32,
    pressure: f32,
    humidity: f32,
}
impl Observer for Baidu {
    fn update(&mut self, temperature: f32, pressure: f32, humidity: f32) {
        self.temperature = temperature;
        self.pressure = pressure;
        self.humidity = humidity;
        self.display();
    }
    fn display(&self) {
        println!("---------百度天气预报-------------");
        println!(
            "温度:{}  气压:{}   湿度:{}",
            self.temperature, self.pressure, self.humidity
        );
        println!("---------百度天气预报-------------");
    }
    fn get_id(&self) -> i32 {
        self.id
    }
}
struct Sina {
    id: i32,
    temperature: f32,
    pressure: f32,
    humidity: f32,
}
impl Observer for Sina {
    fn update(&mut self, temperature: f32, pressure: f32, humidity: f32) {
        self.temperature = temperature;
        self.pressure = pressure;
        self.humidity = humidity;
        self.display();
    }
    fn display(&self) {
        println!("---------新浪天气预报-------------");
        println!(
            "温度:{}  气压:{}   湿度:{}",
            self.temperature, self.pressure, self.humidity
        );
        println!("---------新浪天气预报-------------");
    }
    fn get_id(&self) -> i32 {
        self.id
    }
}

trait Subject {
    fn attach(&mut self, observer: Box<dyn Observer>);
    fn dettach(&mut self, observer: Box<dyn Observer>);
    fn notifyall(&mut self);
}
struct CNWeather {
    temperature: f32,
    pressure: f32,
    humidity: f32,
    observers: HashMap<i32, Box<dyn Observer>>,
}
impl CNWeather {
    fn new() -> Self {
        CNWeather {
            temperature: 0_f32,
            pressure: 0_f32,
            humidity: 0_f32,
            observers: HashMap::new(),
        }
    }
    fn update(&mut self, temperature: f32, pressure: f32, humidity: f32) {
        self.temperature = temperature;
        self.pressure = pressure;
        self.humidity = humidity;
        self.notifyall();
    }
}

impl Subject for CNWeather {
    fn attach(&mut self, observer: Box<dyn Observer>) {
        self.observers.entry(observer.get_id()).or_insert(observer);
    }

    fn dettach(&mut self, observer: Box<dyn Observer>) {
        self.observers.remove(&observer.get_id());
    }

    fn notifyall(&mut self) {
        for observer in self.observers.iter_mut() {
            println!("\r\n");
            observer
                .1
                .update(self.temperature, self.pressure, self.humidity);
            println!("\r\n\r\n")
        }
    }
}

fn main() {
    let mut cn_weather = CNWeather::new();
    let baidu = Baidu {
        id: 10001,
        temperature: 0_f32,
        pressure: 0_f32,
        humidity: 0_f32,
    };
    let sina = Sina {
        id: 10002,
        temperature: 0_f32,
        pressure: 0_f32,
        humidity: 0_f32,
    };
    cn_weather.attach(Box::new(baidu));
    cn_weather.attach(Box::new(sina));

    // 更新
    cn_weather.update(23.1, 101.23, 89.1);
}
