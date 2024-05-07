//! 责任链模式
//!

// 定义Requst结构体抽象为Patient病人
struct Patient {
    name: String,
}

// 处理者Handler抽象为Department部门
trait Department {
    // 处理者接口
    fn handle(&mut self, patient: &mut Patient);
    // 责任链的下一级
    fn next(&mut self) -> &mut Option<Box<dyn Department>>;

    fn execute(&mut self, patient: &mut Patient) {
        self.handle(patient);
        self.next().as_mut().map(|d| {
            d.execute(patient);
        });
    }
}

// 医生
struct Doctor {
    next: Option<Box<dyn Department>>,
}

impl Doctor {
    fn new(next: impl Department + 'static) -> Self {
        Self {
            next: Some(Box::new(next)),
        }
    }
}
impl Department for Doctor {
    fn handle(&mut self, patient: &mut Patient) {
        println!("医生正在给病人:'{}' 诊断...", patient.name);
    }
    fn next(&mut self) -> &mut Option<Box<dyn Department>> {
        &mut self.next
    }
}

// 药师
struct Medical {
    next: Option<Box<dyn Department>>,
}
impl Medical {
    fn new(next: impl Department + 'static) -> Self {
        Self {
            next: Some(Box::new(next)),
        }
    }
}
impl Department for Medical {
    fn handle(&mut self, patient: &mut Patient) {
        println!("药师正在给病人:'{}' 开药...", patient.name)
    }
    fn next(&mut self) -> &mut Option<Box<dyn Department>> {
        &mut self.next
    }
}

// 收银
#[derive(Default)]
struct Cashier {
    next: Option<Box<dyn Department>>,
}

impl Department for Cashier {
    fn handle(&mut self, patient: &mut Patient) {
        println!("收银员正在给病人:'{}' 收银...", patient.name);
    }
    fn next(&mut self) -> &mut Option<Box<dyn Department>> {
        &mut self.next
    }
}

// 接待
struct Reception {
    next: Option<Box<dyn Department>>,
}
impl Reception {
    fn new(next: impl Department + 'static) -> Self {
        Self {
            next: Some(Box::new(next)),
        }
    }
}
impl Department for Reception {
    fn handle(&mut self, patient: &mut Patient) {
        println!("接待员正在接待病人:'{}'...", patient.name)
    }
    fn next(&mut self) -> &mut Option<Box<dyn Department>> {
        &mut self.next
    }
}

fn main() {
    // 构造责任链
    let cashier = Cashier::default();
    let medical = Medical::new(cashier);
    let doctor = Doctor::new(medical);
    let mut reception = Reception::new(doctor);

    // 病人
    let mut patient = Patient {
        name: "张三".to_string(),
    };

    // 执行
    reception.execute(&mut patient);
}
