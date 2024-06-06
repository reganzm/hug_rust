//! cpu眼里的self

pub fn get(pointer: *const A) -> *const A {
    pointer
}

pub struct A;
impl A {
    pub fn get(&self) -> *const A {
        self as *const A
    }
}
pub fn main() {
    let b = A.get();
    let d = get(b);
    println!("b:{:p} d:{:p}", b, d);
}
