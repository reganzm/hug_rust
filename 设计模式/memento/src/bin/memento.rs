//! 备忘录模式

// 备忘录
#[derive(Clone)]
struct Memento {
    state: String,
}
impl Memento {
    fn get_state(&self) -> &str {
        &self.state
    }
    fn set_state(&mut self, state: String) {
        self.state = state
    }
}

// 发起人
struct Originator {
    state: String,
}
impl Originator {
    fn get_state(&self) -> &str {
        &self.state
    }
    fn set_state(&mut self, state: String) {
        self.state = state
    }
    fn create_memento(&self) -> Memento {
        Memento {
            state: self.state.clone(),
        }
    }
    fn restore_from_memento(&mut self, memento: Memento) {
        self.state = memento.state
    }
}

// 存档
struct CareTacker {
    memento_list: Vec<Memento>,
}
impl CareTacker {
    fn add(&mut self, memento: Memento) {
        self.memento_list.push(memento);
    }
    fn get(&self, index: usize) -> Memento {
        self.memento_list.get(index).unwrap().clone()
    }
}

fn main() {
    // 发起人
    let mut originator = Originator {
        state: "状态1:红灯".to_string(),
    };
    println!("init state:{}", originator.get_state());
    // 存档
    let mut care_tacker = CareTacker {
        memento_list: vec![],
    };
    // 将状态保存到备忘录并存档
    care_tacker.add(originator.create_memento());
    originator.set_state("状态2:黄灯".to_string());
    println!("update state:{}", originator.get_state());
    care_tacker.add(originator.create_memento());
    originator.set_state("状态3:绿灯".to_string());
    println!("update state:{}", originator.get_state());
    care_tacker.add(originator.create_memento());
    // 从存档恢复状态
    originator.restore_from_memento(care_tacker.get(0));
    println!("restore state:{}", originator.get_state());
}
