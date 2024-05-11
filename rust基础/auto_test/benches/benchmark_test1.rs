use criterion::{black_box, criterion_group, criterion_main, Criterion};
fn pow_5(num:i32)->i32{
    num * num * num * num * num
}
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("pow_5", |b| b.iter(|| pow_5(black_box(100))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
