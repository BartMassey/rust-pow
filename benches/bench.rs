use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pow::*;

macro_rules! bench_pow {
    ($c: expr, $name:expr, $func:tt, $args:expr) => {{
        let (b, e) = $args;
        $c.bench_function(&format!("{}({},{})", $name, b, e), |b| b.iter(|| {
            let (b, e) = black_box((2, 31));
            $func(b, e)
        }));
    }};
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let args = vec![
        (2,31),
        (7,11),
        (3,18),
    ];
    for arg in args {
        let mut group = c.benchmark_group(&format!("pow({}, {})", arg.0, arg.1));
        bench_pow!(group, "pow_std", pow_std, arg);
        bench_pow!(group, "pow_alt", pow_alt, arg);
        bench_pow!(group, "u32::pow", (u32::pow), arg);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
