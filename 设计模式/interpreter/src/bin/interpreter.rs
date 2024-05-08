//! 解释器模式
trait Expression {
    fn interpret(&self) -> i32;
}

// 数字表达式 属于TerminalExpression
struct NumberExpression {
    value: i32,
}
impl NumberExpression {
    fn new(value: i32) -> Self {
        NumberExpression { value }
    }
}
impl Expression for NumberExpression {
    fn interpret(&self) -> i32 {
        self.value
    }
}
// 加法表达式 属于NonterminalExpression
struct AddExpression {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}
impl AddExpression {
    fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        Self { left, right }
    }
}
impl Expression for AddExpression {
    fn interpret(&self) -> i32 {
        self.left.interpret() + self.right.interpret()
    }
}

fn main() {
    let first = NumberExpression::new(10);
    let second = NumberExpression::new(99);
    let add = AddExpression::new(Box::new(first), Box::new(second));
    let result = add.interpret();
    println!("result:{result}");
}
