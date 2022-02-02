use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
    BenchmarkId,
};
use pow::*;

macro_rules! bench_pow {
    ($c: expr, $name:expr, $func:tt, $args:expr) => {{
        let (base, exp) = $args;
        $c.bench_function($name, |b| b.iter(|| {
            black_box($func(black_box(base), black_box(exp)))
        }));
    }};
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let args = vec![
        (0,31),
        (1,31),
        (2,31),
        (3,18),
        (7,11),
        (3,7),
    ];
    for arg in args {
        let mut group = c.benchmark_group(&format!("pow({}, {})", arg.0, arg.1));
        bench_pow!(group, "pow_std", pow_std, arg);
        bench_pow!(group, "pow_std_2opt", pow_std_2opt, arg);
        bench_pow!(group, "pow_alt", pow_alt, arg);
        bench_pow!(group, "pow_alt_2opt", pow_alt_2opt, arg);
        bench_pow!(group, "pow_alt_01opt", pow_alt_01opt, arg);
        bench_pow!(group, "pow_alt_012opt", pow_alt_012opt, arg);
        bench_pow!(group, "u32::pow", (u32::pow), arg);
        bench_pow!(group, "pow_alt_inline", pow_alt_inline, arg);
        bench_pow!(group, "pow_alt_2opt_inline", pow_alt_2opt_inline, arg);
        group.finish();
    }
    let mut group = c.benchmark_group("pow(3, 0..18)");
    for exp in 0..=18 {
        group.bench_with_input(
            BenchmarkId::new("pow_std", exp),
            &exp,
            |b, exp| b.iter(|| black_box(pow_std(black_box(3), black_box(*exp)))),
        );
        group.bench_with_input(
            BenchmarkId::new("pow_alt_012opt", exp),
            &exp,
            |b, exp| b.iter(|| black_box(pow_alt_012opt(black_box(3), black_box(*exp)))),
        );
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
