trait Product {
    fn weight(&self) -> f64;
}

#[derive(Debug)]
struct Pen;
#[derive(Debug)]
struct Car;

impl Product for Pen {
    fn weight(&self) -> f64 {
        println!("这只笔重量为150g");
        150.0
    }
}
impl Product for Car {
    fn weight(&self) -> f64 {
        println!("这辆汽车重1.5吨");
        1500000.0
    }
}

struct SimpleFactory;
impl SimpleFactory {
    fn get_product(product: &str) -> Option<Box<dyn Product>> {
        if product == "car" {
            Some(Box::new(Car))
        } else if product == "pen" {
            Some(Box::new(Pen))
        } else {
            None
        }
    }
}
fn main() {
    SimpleFactory::get_product("pen").unwrap().weight();
    SimpleFactory::get_product("car").unwrap().weight();
    let product = SimpleFactory::get_product("other");
    if product.is_some() {
        product.unwrap().weight();
    } else {
        println!("没有该产品")
    }
}
