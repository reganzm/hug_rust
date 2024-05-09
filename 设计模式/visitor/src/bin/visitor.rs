trait Visitor {
    fn visit_element_a(&self, element: &ElementA);
    fn visit_element_b(&self, element: &ElementB);
}
trait Element {
    fn accept(&self, viritor: &dyn Visitor);
}
#[derive(Debug)]
struct ElementA {
    name: String,
    age: i32,
}
impl Element for ElementA {
    fn accept(&self, viritor: &dyn Visitor) {
        viritor.visit_element_a(self)
    }
}
#[derive(Debug)]
struct ElementB {
    weight: f32,
    height: f32,
}
impl Element for ElementB {
    fn accept(&self, viritor: &dyn Visitor) {
        viritor.visit_element_b(self)
    }
}

// 具体的访问者
struct ConcreteVisitor;
impl Visitor for ConcreteVisitor {
    fn visit_element_a(&self, element: &ElementA) {
        println!("elementa:{:?}", element);
    }

    fn visit_element_b(&self, element: &ElementB) {
        println!("elementb:{:?}", element);
    }
}
struct ObjectStructure {
    elements: Vec<Box<dyn Element>>,
}
impl ObjectStructure {
    fn new() -> Self {
        Self { elements: vec![] }
    }
    fn add_element(&mut self, element: Box<dyn Element>) {
        self.elements.push(element);
    }
    fn accept(&self, visitor: &dyn Visitor) {
        for element in &self.elements {
            element.accept(visitor);
        }
    }
}

fn main() {
    let mut object_structure = ObjectStructure::new();
    //添加元素
    object_structure.add_element(Box::new(ElementA {
        name: "小东西".to_string(),
        age: 3,
    }));
    object_structure.add_element(Box::new(ElementB {
        height: 60.1,
        weight: 10.3,
    }));
    // 访问者
    let visitor = ConcreteVisitor;
    // 接收访问者
    object_structure.accept(&visitor);
}
