use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    BenchmarkId,
    Criterion,
};
use pow::*;

macro_rules! bench_pow {
    ($c: expr, $name:expr, $func:tt, $args:expr) => {{
        let mut group = $c.benchmark_group($name);
        for (base, max_exp) in $args {
            let id = base.to_string();
            for exp in 0..=*max_exp {
                group.bench_with_input(
                    BenchmarkId::new(&id, exp),
                    &exp,
                    |b, exp| b.iter(|| {
                        black_box($func(black_box(*base), black_box(*exp)))
                    }),
                );
            }
        }
        group.finish();
    }};
}

pub fn criterion_benchmark(c: &mut Criterion) {
    #[rustfmt::ignore]
    let args = &[
        (0, 63),
        (1, 63),
        (2, 31),
        (3, 20),
        (4, 15),
        (5, 14),
        (6, 12),
        (7, 11),
        (8, 10),
    ];
    bench_pow!(c, "pow_std", pow_std, args);
    bench_pow!(c, "pow_std_2opt", pow_std_2opt, args);
    bench_pow!(c, "pow_alt", pow_alt, args);
    bench_pow!(c, "pow_alt_2opt", pow_alt_2opt, args);
    bench_pow!(c, "pow_alt_01opt", pow_alt_01opt, args);
    bench_pow!(c, "pow_alt_012opt", pow_alt_012opt, args);
    bench_pow!(c, "u32::pow", (u32::pow), args);
    bench_pow!(c, "pow_alt_inline", pow_alt_inline, args);
    bench_pow!(c, "pow_alt_2opt_inline", pow_alt_2opt_inline, args);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
