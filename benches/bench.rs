use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pow::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("pow_std(2,31)", |b| b.iter(|| {
        let (b, e) = black_box((2, 31));
        pow_std(b, e)
    }));
    c.bench_function("pow_alt(2,31)", |b| b.iter(|| {
        let (b, e) = black_box((2, 31));
        pow_alt(b, e)
    }));
    c.bench_function("pow_std(7,11)", |b| b.iter(|| {
        let (b, e) = black_box((7, 11));
        pow_std(b, e)
    }));
    c.bench_function("pow_alt(7,11)", |b| b.iter(|| {
        let (b, e) = black_box((7, 11));
        pow_alt(b, e)
    }));
    c.bench_function("pow_std(3,16)", |b| b.iter(|| {
        let (b, e) = black_box((3, 16));
        pow_std(b, e)
    }));
    c.bench_function("pow_alt(3,16)", |b| b.iter(|| {
        let (b, e) = black_box((3, 16));
        pow_alt(b, e)
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
