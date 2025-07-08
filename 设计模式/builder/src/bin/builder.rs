//! 实现构建者模式

// 使用构建者模式模拟食堂制备早餐

// 早餐包含的食物，抽象为Item

trait Item {
    fn name(&self) -> String;
    fn price(&self) -> f32;
    fn packing(&self) -> String;
}
// 鸡肉汉堡
struct ChickenHamburg;
impl Item for ChickenHamburg {
    fn name(&self) -> String {
        String::from("鸡肉汉堡")
    }

    fn price(&self) -> f32 {
        23.2
    }

    fn packing(&self) -> String {
        String::from("盒子")
    }
}
// 牛肉汉堡
struct BeafHamburg;
impl Item for BeafHamburg {
    fn name(&self) -> String {
        "牛肉汉堡".to_string()
    }

    fn price(&self) -> f32 {
        35.8
    }

    fn packing(&self) -> String {
        "包装纸".to_string()
    }
}
// 百世可乐
struct Pepsi;
impl Item for Pepsi {
    fn name(&self) -> String {
        "百事可乐".to_string()
    }

    fn price(&self) -> f32 {
        5.5
    }

    fn packing(&self) -> String {
        "罐装".to_string()
    }
}
// 苏打水
struct Soda;
impl Item for Soda {
    fn name(&self) -> String {
        "苏打水".to_string()
    }

    fn price(&self) -> f32 {
        4.5
    }

    fn packing(&self) -> String {
        "罐装".to_string()
    }
}

// 食物
struct Meal {
    items: Vec<Box<dyn Item>>,
}
impl Meal {
    fn add_item(&mut self, item: Box<dyn Item>) {
        self.items.push(item);
    }
    fn get_cost(&self) -> f32 {
        self.items.iter().fold(0.0, |a, b| a + b.price())
    }
    fn show_items(&self) {
        for i in self.items.iter() {
            println!(
                "name:{} price:{} packing:{}",
                i.name(),
                i.price(),
                i.packing()
            )
        }
    }
}

// 构建者
trait Builder {
    fn prepare_beaf_humberg() -> Meal;
    fn prepare_chicken_humberg() -> Meal;
}

// 天津的小作坊
struct TiJinBuilder;
impl Builder for TiJinBuilder {
    fn prepare_beaf_humberg() -> Meal {
        let mut meal = Meal { items: vec![] };
        meal.add_item(Box::new(BeafHamburg));
        meal.add_item(Box::new(Pepsi));
        meal
    }

    fn prepare_chicken_humberg() -> Meal {
        let mut meal = Meal { items: vec![] };
        meal.add_item(Box::new(ChickenHamburg));
        meal.add_item(Box::new(Soda));
        meal
    }
}

// 成都的小作坊
struct CdBuilder;
impl Builder for CdBuilder {
    fn prepare_beaf_humberg() -> Meal {
        let mut meal = Meal { items: vec![] };
        meal.add_item(Box::new(BeafHamburg));
        meal.add_item(Box::new(Soda));
        meal
    }

    fn prepare_chicken_humberg() -> Meal {
        let mut meal = Meal { items: vec![] };
        meal.add_item(Box::new(ChickenHamburg));
        meal.add_item(Box::new(Soda));
        meal
    }
}

fn main() {
    // 成都小作坊制作的牛肉汉堡
    let cd = CdBuilder::prepare_beaf_humberg();
    cd.show_items();
    cd.get_cost();

    // 天津小作坊制作的牛肉汉堡
    let tj = TiJinBuilder::prepare_beaf_humberg();
    tj.show_items();
    tj.get_cost();
}
