trait Template {
    // 抽象方法
    fn step1(&self);
    // 默认方法
    fn step2(&self) {
        println!("step2:去掉皮")
    }
    // 抽象方法
    fn step3(&self);
    // 默认方法
    fn step4(&self) {
        println!("step4:煮耙后出锅")
    }
    // 模板方法
    fn action(&self) {
        self.step1();
        self.step2();
        self.step3();
        self.step4();
    }
}
// 具体类实现模板方法
struct PotatoImpl;
impl Template for PotatoImpl {
    fn step1(&self) {
        println!("step1:将土豆洗干净")
    }

    fn step3(&self) {
        println!("step3:切成1厘米见方的小块")
    }
}
struct BeafImpl;
impl Template for BeafImpl {
    fn step1(&self) {
        println!("step1:买一块上好的和牛,清洗干净")
    }
    fn step2(&self) {
        println!("step2:切成小块")
    }

    fn step3(&self) {
        println!("step3:配上料下锅煎至8成熟,起锅")
    }
    fn step4(&self) {
        println!("step4:摆盘，开整！")
    }
}

fn main() {
    println!("===========炖土豆===========");
    let potato_impl = PotatoImpl;
    potato_impl.action();
    println!("===========煎牛排===========");
    let beaf_impl = BeafImpl;
    beaf_impl.action();
}
