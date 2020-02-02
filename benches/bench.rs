use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pow::*;

macro_rules! bench_pow {
    ($c: expr, $name:expr, $func:tt, $args:expr) => {{
        let (base, exp) = $args;
        $c.bench_function($name, |b| b.iter(|| {
            let (base, exp) = black_box((base, exp));
            $func(base, exp)
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
        bench_pow!(group, "pow_std_2opt", pow_std_2opt, arg);
        bench_pow!(group, "pow_alt", pow_alt, arg);
        bench_pow!(group, "pow_alt_2opt", pow_std_2opt, arg);
        bench_pow!(group, "u32::pow", (u32::pow), arg);
        bench_pow!(group, "pow_alt_inline", pow_alt_inline, arg);
        bench_pow!(group, "pow_alt_2opt_inline", pow_alt_inline, arg);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
