trait Engine {
    fn show(&self);
}
trait Wheel {
    fn show(&self);
}

trait Factory {
    type E: Engine;
    type W: Wheel;
    // 返回具体的关联类型
    fn create_engine(&self) -> Self::E;
    fn create_wheel(&self) -> Self::W;
}
trait DynFactory {
    // 返回特征对象，更具有通用性
    fn dyn_create_engine(&self) -> Box<dyn Engine>;
    fn dyn_create_wheel(&self) -> Box<dyn Wheel>;
}

struct Engine1;
impl Engine for Engine1 {
    fn show(&self) {
        println!("通用 LS");
    }
}
struct Engine2;
impl Engine for Engine2 {
    fn show(&self) {
        println!("福特 EcoBoost");
    }
}

struct Wheel1;
impl Wheel for Wheel1 {
    fn show(&self) {
        println!("米其林轮胎");
    }
}
struct Wheel2;
impl Wheel for Wheel2 {
    fn show(&self) {
        println!("朝阳轮胎");
    }
}

struct XMS7Factory;
// 通过关联类型返回
impl Factory for XMS7Factory {
    type E = Engine1;
    type W = Wheel1;

    fn create_engine(&self) -> Self::E {
        Engine1
    }

    fn create_wheel(&self) -> Self::W {
        Wheel1
    }
}
// 可以返回特征对象
impl DynFactory for XMS7Factory {
    fn dyn_create_engine(&self) -> Box<dyn Engine> {
        Box::new(Engine1)
    }

    fn dyn_create_wheel(&self) -> Box<dyn Wheel> {
        Box::new(Wheel1)
    }
}

fn main() {
    let xiaomi_su7_factory = XMS7Factory;
    let engine = xiaomi_su7_factory.create_engine();
    engine.show();
    let wheel = xiaomi_su7_factory.create_wheel();
    wheel.show();

    let engine = xiaomi_su7_factory.dyn_create_engine();
    engine.show();
    let wheel = xiaomi_su7_factory.dyn_create_wheel();
    wheel.show();
}
