use derive_more::{
    Add, AddAssign, Constructor, Deref, DerefMut, Display, From, FromStr, Index, IndexMut, Into,
    IsVariant, Mul, MulAssign, Not, TryInto,
};

/// Some docs
#[derive(
    Add, AddAssign, Constructor, Display, From, FromStr, Into, Mul, MulAssign, Not, Clone, Copy,
)]
pub struct MyInt(i32);

/// Some docs
#[derive(Deref, DerefMut)]
pub struct MyBoxedInt(Box<i32>);

/// Some docs
#[derive(Index, IndexMut)]
pub struct MyVec(Vec<i32>);

/// Some docs
#[allow(dead_code)]
#[derive(Clone, Copy, TryInto, IsVariant)]
enum MixedInts {
    SmallInt(i32),
    NamedBigInt { int: i64 },
}

fn main() {
    let a = MyInt(1000);
    let b = MyInt(20);
    let c = a + b;
    println!("{a} + {b} = {c}");
}
