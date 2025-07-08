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

// example::get::h64f063fea484ae4e:
//         mov     rax, rdi
//         ret

// example::A::get::h6be26c2822996cdb:
//         mov     rax, rdi
//         ret

// example::main::h238d10f39e0a48fc:
//         push    rax
//         lea     rdi, [rip + .L__unnamed_1]
//         call    qword ptr [rip + example::A::get::h6be26c2822996cdb@GOTPCREL]
//         mov     rdi, rax
//         call    qword ptr [rip + example::get::h64f063fea484ae4e@GOTPCREL]
//         pop     rax
//         ret

// .L__unnamed_1:
