trait Strategy {
    fn do_operation(&self, num1: i32, num2: i32) -> i32;
}

struct StrategyAdd;
impl Strategy for StrategyAdd {
    fn do_operation(&self, num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
}
struct StrategySub;
impl Strategy for StrategySub {
    fn do_operation(&self, num1: i32, num2: i32) -> i32 {
        num1 - num2
    }
}

struct StrategyMul;
impl Strategy for StrategyMul {
    fn do_operation(&self, num1: i32, num2: i32) -> i32 {
        num1 * num2
    }
}
struct Context {
    strategy: Box<dyn Strategy>,
}
impl Context {
    fn execute(&self, num1: i32, num2: i32) -> i32 {
        self.strategy.do_operation(num1, num2)
    }
}

fn main() {
    let mut context = Context {
        strategy: Box::new(StrategyAdd),
    };
    let num1 = 100;
    let num2 = 200;
    println!("100 + 200 = {}", context.execute(num1, num2));
    // 更改策略
    context.strategy = Box::new(StrategyMul);
    println!("100 - 200 = {}", context.execute(num1, num2));
    context.strategy = Box::new(StrategyMul);
    println!("100 * 200 = {}", context.execute(num1, num2));
}
