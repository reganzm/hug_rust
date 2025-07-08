//! 迷你区块链

use chrono::Utc;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{thread, time::Duration};

/// 区块头
#[derive(Serialize, Debug, PartialEq, Eq)]
struct BlockHeader {
    pre_hash: String,
    txs_hash: String,
    time: i64,
}

/// 区块体
#[derive(Debug)]
struct Block {
    // 区块头
    header: BlockHeader,
    // 区块包含的所有交易数据
    tranxs: String,
    // 区块的hash
    hash: String,
}

impl Block {
    fn new(txs: String, pre_hash: String) -> Self {
        // 挖矿
        println!("开始挖矿....");
        thread::sleep(Duration::from_secs(5));

        // 准备区块数据
        let time = Utc::now().timestamp();
        let txs_serialize = serialize(&txs);
        let txs_hash = cal_hash(&txs_serialize);
        let mut block = Block {
            header: BlockHeader {
                time: time,
                txs_hash: txs_hash,
                pre_hash: pre_hash,
            },
            tranxs: txs,
            hash: "".to_string(),
        };
        block.set_block_hash();
        println!("产生出一个区块");
        block
    }
    // 计算区块的hash
    fn set_block_hash(&mut self) {
        let header = serialize(&self.header);
        self.hash = cal_hash(&header);
    }
}

struct BlockChain {
    blocks: Vec<Block>,
}

impl BlockChain {
    fn new() -> Self {
        BlockChain {
            blocks: vec![Self::genesis_block()],
        }
    }

    // 添加区块
    fn add_block(&mut self, txs: String) {
        // 前一个区块的hash
        let last_index = self.blocks.len() - 1;
        let pre_block = &self.blocks[last_index];
        let pre_hash = pre_block.hash.clone();
        // 创建新块并加入区块链
        let new_block = Block::new(txs, pre_hash);
        self.blocks.push(new_block);
    }

    // 区块两信息
    fn display(&self) {
        for i in self.blocks.iter() {
            println!("{:#?}", i);
        }
    }

    /// 创世块
    fn genesis_block() -> Block {
        Block::new("创世块".to_string(), "2024".to_string())
    }
}

/// T是实现了Serialize并且可变大小的泛型，返回二进制u8格式
fn serialize<T: ?Sized + Serialize>(value: &T) -> Vec<u8> {
    bincode::serialize(value).unwrap()
}

/// 计算hash值
fn cal_hash(value: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value);
    let hash = hasher.finalize();
    format!("{:x}", hash)
}

fn main() {
    let mut block_chain = BlockChain::new();
    let txs = r#"
    账户A向账户B转账5元;
    账户B向账户C转账4.6元;
    账户E向账户A转账100元;
    "#
    .to_string();
    block_chain.add_block(txs);

    let txs = r#"
    账户E向账户B转账5元;
    账户B向账户C转账4.6元;
    账户C向账户A转账100元;
    "#
    .to_string();
    block_chain.add_block(txs);

    block_chain.display();
}
