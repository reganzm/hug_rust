use std::{collections::HashMap, sync::Arc};

#[warn(dead_code)]
struct BigObject {
    data: [i32; 10000],
}
impl Default for BigObject {
    fn default() -> Self {
        Self { data: [0; 10000] }
    }
}

struct FlywightFactory {
    // BigObject需要共享，使用Arc只能指针
    objects: HashMap<String, Arc<BigObject>>,
}
impl Default for FlywightFactory {
    fn default() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }
}
impl FlywightFactory {
    fn get(&self, name: &str) -> Option<Arc<BigObject>> {
        self.objects.get(name).cloned()
    }

    fn create<T: FnOnce() -> BigObject>(
        &mut self,
        name: String,
        create_object: T,
    ) -> Arc<BigObject> {
        let res = Arc::new(create_object());
        self.objects.insert(name, res.clone());
        res
    }
}

fn main() {
    let mut flyweight_factory = FlywightFactory::default();
    // 创建一个超大对象
    flyweight_factory.create("bigobject".to_string(), BigObject::default);
    // 获取超大对象，不存在再创建
    if let None = flyweight_factory.get("bigobject") {
        flyweight_factory.create("bigobject".to_string(), BigObject::default);
    }
    println!(
        "flyweight factory objects keys:{:?}",
        flyweight_factory.objects.keys()
    );
}
