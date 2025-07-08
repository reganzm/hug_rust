trait Target {
    fn request(&self) -> String;
}

pub struct Adaptee {
    value: i32,
}
impl Adaptee {
    fn specific_request(&self) -> String {
        format!("Adaptee value is {}", self.value)
    }
}

struct Adapter {
    adaptee: Adaptee,
}

impl Adapter {
    fn new(adaptee: Adaptee) -> Self {
        Adapter { adaptee }
    }
}
impl Target for Adapter {
    fn request(&self) -> String {
        self.adaptee.specific_request()
    }
}

fn main() {
    let adaptee = Adaptee { value: 100 };
    let adapter = Adapter::new(adaptee);
    println!("{}", adapter.request());
}
