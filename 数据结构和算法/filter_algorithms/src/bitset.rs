//! 位图

#[derive(Debug)]
pub struct BitSet {
    length: usize,
    bits: Vec<u64>,
}

// 对齐
const ALIGN: usize = 6;
// 字大小(2^6 = 64) 一个u64存储64位
const WORD_SIZE: usize = 64;

// vec的大小
fn vec_size(n: usize) -> usize {
    let mut size = 0;
    if n & (WORD_SIZE - 1) == 0 {
        size = n >> ALIGN;
    }
    size = (n >> ALIGN) + 1;
    size
}

// 找到Vec中u64 64位的下标
fn value_index(n: usize) -> usize {
    let index = n & (WORD_SIZE - 1);
    index
}

impl BitSet {
    pub fn new(cap: usize) -> Self {
        let size = vec_size(cap);
        Self {
            length: size * WORD_SIZE,
            bits: vec![0; size],
        }
    }

    /// 当前位图大小
    pub fn len(&self) -> usize {
        self.length
    }

    /// 重置位图
    pub fn reset(&mut self) {
        self.bits.fill(0);
    }

    /// 扩展位图 只能由小扩到大
    fn extend(&mut self, length: usize) {
        assert!(length > self.length);
        // 扩展了几个
        let new_add_size = vec_size(length - self.length);
        let new_bits = vec![0; new_add_size];
        self.bits.extend(new_bits.iter());
        // 位图长度加新字长度
        self.length += new_add_size * WORD_SIZE;
    }

    /// 将pos位设置为1(索引从0开始)
    pub fn set(&mut self, pos: usize) {
        if pos >= self.len() {
            // 扩容
            self.extend(pos + 1);
        }
        // 右移找到对应的字和字对应位的值做或运算
        let index = pos >> ALIGN;
        self.bits[index] = self.bits[index] | 1 << value_index(pos);
    }

    /// test 检测第pos位是否为1
    pub fn test(&self, pos: usize) -> bool {
        if pos >= self.len() {
            return false;
        }
        self.bits[pos >> ALIGN] & 1 << value_index(pos) != 0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_alignof6() {
        let result = vec_size(99999);
        println!("result:{result}");
    }

    #[test]
    fn test_bitset() {
        let mut bs = BitSet::new(10000);
        println!("start bs length:{}", bs.len());
        bs.set(65);
        assert!(bs.test(65));
        println!("end bs length:{}", bs.len());

        println!("values:{:#?}", bs);
    }
}
