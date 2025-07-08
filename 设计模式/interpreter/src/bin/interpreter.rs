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
// 减法表达式
struct SubExpression {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}
impl SubExpression {
    fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        Self { left, right }
    }
}
impl Expression for SubExpression {
    fn interpret(&self) -> i32 {
        self.left.interpret() - self.right.interpret()
    }
}
// 乘法表达式
struct MulExpression {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}
impl MulExpression {
    fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        Self { left, right }
    }
}
impl Expression for MulExpression {
    fn interpret(&self) -> i32 {
        self.left.interpret() * self.right.interpret()
    }
}
// 除法表达式
struct DiviExpression {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}
impl DiviExpression {
    fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        Self { left, right }
    }
}
impl Expression for DiviExpression {
    fn interpret(&self) -> i32 {
        let tmp = self.right.interpret();
        // 除数不能为0
        assert_ne!(0, tmp);
        self.left.interpret() / self.right.interpret()
    }
}

fn main() {
    // 计算4 * 5 + 6 / 3 - 1
    let a = NumberExpression::new(4);
    let b = NumberExpression::new(5);
    let c = NumberExpression::new(6);
    let d = NumberExpression::new(3);
    let e = NumberExpression::new(1);

    let part1 = MulExpression::new(Box::new(a), Box::new(b));
    let part2 = DiviExpression::new(Box::new(c), Box::new(d));
    let part3 = AddExpression::new(Box::new(part1), Box::new(part2));
    let part4 = SubExpression::new(Box::new(part3), Box::new(e));

    let result = part4.interpret();
    println!(" 4 * 5 + 6 / 3 - 1 = {result} ");
}
