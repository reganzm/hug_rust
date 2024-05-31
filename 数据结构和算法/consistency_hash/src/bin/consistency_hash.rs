//! 一致性hash算法

use std::{
    collections::{hash_map::DefaultHasher, BTreeMap},
    fmt::Debug,
    hash::{Hash, Hasher},
};

// 主机节点
#[derive(Debug, Clone)]
struct MachineNode {
    host: &'static str,
    ip: &'static str,
    port: u16,
}
// 为MachineNode添加字符串转换
impl ToString for MachineNode {
    fn to_string(&self) -> String {
        self.ip.to_string() + &self.port.to_string()
    }
}

// 环
#[derive(Debug)]
struct Ring<T: Clone + Debug + ToString> {
    // 每台主机的虚拟节点数
    machine_virtual_node_num: usize,
    // 用于保存环上的数据
    datas: BTreeMap<u64, T>,
}

// 默认主机虚拟节点数为10
const DEFAULT_MACHINE_VIRTUAL_NODE_NUM: usize = 10;
// hash函数
fn hash<T: Hash>(val: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    val.hash(&mut hasher);
    hasher.finish() % 197
}

impl<T: Debug + Clone + ToString> Ring<T> {
    fn new() -> Self {
        Self::with_virtual_node_num(DEFAULT_MACHINE_VIRTUAL_NODE_NUM)
    }
    fn with_virtual_node_num(num: usize) -> Self {
        Self {
            machine_virtual_node_num: num,
            datas: BTreeMap::new(),
        }
    }
    // 增加节点到环上
    fn add(&mut self, node: &T) {
        // 将主机虚拟出machine_virtual_node_num个数量
        for i in 0..self.machine_virtual_node_num {
            let key = hash(&(node.to_string() + &i.to_string()));
            self.datas.insert(key, node.clone());
        }
    }

    // 批量添加到环
    fn mult_add(&mut self, nodes: &[T]) {
        if !nodes.is_empty() {
            for node in nodes {
                self.add(node);
            }
        }
    }

    // 从环上移除节点
    fn remove(&mut self, node: &T) {
        assert!(!self.datas.is_empty());

        for i in 0..self.machine_virtual_node_num {
            let key = hash(&(node.to_string() + &i.to_string()));
            self.datas.remove(&key);
        }
    }

    // 批量删除
    fn mult_remove(&mut self, nodes: &[T]) {
        if !nodes.is_empty() {
            for node in nodes {
                self.remove(node);
            }
        }
    }

    // 获取节点
    fn get(&self, key: u64) -> Option<&T> {
        if self.datas.is_empty() {
            println!("-----");
            return None;
        }
        let mut keys = self.datas.keys();
        println!("key:{key}");
        keys.find(|&k| k >= &key)
            .and_then(|k| self.datas.get(k))
            .or(keys.nth(0).and_then(|x| self.datas.get(x)))
    }

    // 获取处理数据的机器
    fn dispatch(&self, data: i32) -> Option<&T> {
        let key = hash(&data);
        self.get(key)
    }
}

fn main() {
    let virtual_num = 2;
    let mut ring = Ring::with_virtual_node_num(virtual_num);
    let node1 = MachineNode {
        host: "bigdata1",
        ip: "192.168.0.29",
        port: 18088,
    };
    let node2 = MachineNode {
        host: "bigdata2",
        ip: "192.168.0.14",
        port: 28089,
    };
    let node3 = MachineNode {
        host: "bigdata3",
        ip: "192.168.0.22",
        port: 8088,
    };
    let node4 = MachineNode {
        host: "bigdata4",
        ip: "192.168.0.222",
        port: 8088,
    };

    ring.add(&node1);
    ring.add(&node2);
    ring.add(&node3);
    ring.add(&node4);

    println!("ring:{:#?}", ring);

    // 一个数据过来，通过get方法获取哪台机器处理
    let quest = 123;
    let result = ring.dispatch(quest);
    println!("处理机器：{:#?}", result);
    let quest = 1234;
    let result = ring.dispatch(quest);
    println!("处理机器：{:#?}", result);
    let quest = 12345;
    let result = ring.dispatch(quest);
    println!("处理机器：{:#?}", result);
}
