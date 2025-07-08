#[path = "../bloom_filter.rs"]
mod bloom_filter;

use bloom_filter::BloomFilter;

fn main() {
    let mut b = BloomFilter::new(1024, 3);
    println!("bitset len:{}", b.bits.len());
    b.add(&999);
    b.add(&10);
    b.add("你好");
    b.add(&345);

    assert!(b.test(&999));
    assert!(b.test(&10));
    assert!(b.test("你好"));
    assert!(b.test(&345));
    println!("bitset len:{}", b.bits.len());
}
