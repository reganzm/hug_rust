//! 过滤器模式

// 需要过滤的结构体
#[derive(PartialEq, Clone)]
struct Person {
    name: String,
    gender: String,
    marital_status: String,
}
impl Person {
    fn get_gender(&self) -> &str {
        self.gender.as_str()
    }
    fn get_marital_status(&self) -> &str {
        self.marital_status.as_str()
    }
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

// 过滤器特征(又名标准特征)
trait Criteria {
    fn fit_criteria(&self, persons: Vec<Person>) -> Vec<Person>;
}
// 男性过滤标准
struct MaleCriteria;
impl Criteria for MaleCriteria {
    fn fit_criteria(&self, persons: Vec<Person>) -> Vec<Person> {
        persons
            .into_iter()
            .filter(|p| p.get_gender().eq("男"))
            .collect()
    }
}
// 女性过滤标准
struct FemaleCriteria;
impl Criteria for FemaleCriteria {
    fn fit_criteria(&self, persons: Vec<Person>) -> Vec<Person> {
        persons
            .into_iter()
            .filter(|p| p.get_gender().eq("女"))
            .collect()
    }
}
// 单身标准
struct SingleCriteria;
impl Criteria for SingleCriteria {
    fn fit_criteria(&self, persons: Vec<Person>) -> Vec<Person> {
        persons
            .into_iter()
            .filter(|p| p.get_marital_status().eq("未婚"))
            .collect()
    }
}
// 两个标准的交集
struct AndCriteria {
    one_criteria: Box<dyn Criteria>,
    two_criteria: Box<dyn Criteria>,
}
impl Criteria for AndCriteria {
    fn fit_criteria(&self, persons: Vec<Person>) -> Vec<Person> {
        self.one_criteria
            .fit_criteria(self.two_criteria.fit_criteria(persons))
    }
}
// 两个标注的取Or
struct OrCriteria {
    one_criteria: Box<dyn Criteria>,
    two_criteria: Box<dyn Criteria>,
}
impl Criteria for OrCriteria {
    fn fit_criteria(&self, persons: Vec<Person>) -> Vec<Person> {
        let mut one_criteria_items = self.one_criteria.fit_criteria(persons.clone());
        let two_criteria_items = self.two_criteria.fit_criteria(persons);
        for p in two_criteria_items {
            if !one_criteria_items.contains(&p) {
                one_criteria_items.push(p);
            }
        }
        one_criteria_items
    }
}
fn print_persons(persons: Vec<Person>) {
    persons.into_iter().for_each(|p| {
        println!(
            "Person name:{} gender:{} marital_status:{}",
            p.get_name(),
            p.get_gender(),
            p.get_marital_status()
        )
    })
}

fn main() {
    // 构造Person集合
    let mut persons = Vec::new();
    persons.push(Person {
        name: "小张".to_string(),
        gender: "男".to_string(),
        marital_status: "未婚".to_string(),
    });
    persons.push(Person {
        name: "小红".to_string(),
        gender: "女".to_string(),
        marital_status: "未婚".to_string(),
    });
    persons.push(Person {
        name: "小杨".to_string(),
        gender: "女".to_string(),
        marital_status: "已婚".to_string(),
    });
    persons.push(Person {
        name: "小林".to_string(),
        gender: "男".to_string(),
        marital_status: "未婚".to_string(),
    });
    persons.push(Person {
        name: "小强".to_string(),
        gender: "男".to_string(),
        marital_status: "未婚".to_string(),
    });
    persons.push(Person {
        name: "小李".to_string(),
        gender: "女".to_string(),
        marital_status: "已婚".to_string(),
    });

    // 过滤出男士
    let males = MaleCriteria.fit_criteria(persons.clone());
    println!("-----男性用户------");
    print_persons(males);
    // 过滤出女士
    println!("-----女性用户------");
    let females = FemaleCriteria.fit_criteria(persons.clone());
    print_persons(females);
    // 过滤女性已婚
    println!("-----女性单身用户------");
    let single_females = AndCriteria {
        one_criteria: Box::new(FemaleCriteria),
        two_criteria: Box::new(SingleCriteria),
    }
    .fit_criteria(persons.clone());
    print_persons(single_females);
    // 过滤男性或单身用户
    println!("-----男性或单身用户------");
    let single_or_males = OrCriteria {
        one_criteria: Box::new(MaleCriteria),
        two_criteria: Box::new(SingleCriteria),
    }
    .fit_criteria(persons.clone());
    print_persons(single_or_males);
}
