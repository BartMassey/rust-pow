use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    BenchmarkId,
    Criterion,
};
use pow::*;

macro_rules! bench_pow {
    ($c: expr, $name:expr, $func:tt, $base:expr, $max_exp:expr) => {{
        let id = format!("{}/{}", $name, $base);
        let mut group = $c.benchmark_group(&id);
        for exp in 0..=$max_exp {
            group.bench_with_input(
                BenchmarkId::new(&id, exp),
                &exp,
                |b, exp| b.iter(|| {
                    black_box($func(black_box($base), black_box(*exp)))
                }),
            );
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
    for &(base, max_exp) in args {
        bench_pow!(c, "pow_std", pow_std, base, max_exp);
        bench_pow!(c, "pow_std_2opt", pow_std_2opt, base, max_exp);
        bench_pow!(c, "pow_alt", pow_alt, base, max_exp);
        bench_pow!(c, "pow_alt_2opt", pow_alt_2opt, base, max_exp);
        bench_pow!(c, "pow_alt_01opt", pow_alt_01opt, base, max_exp);
        bench_pow!(c, "pow_alt_012opt", pow_alt_012opt, base, max_exp);
        bench_pow!(c, "u32::pow", (u32::pow), base, max_exp);
        bench_pow!(c, "pow_alt_inline", pow_alt_inline, base, max_exp);
        bench_pow!(c, "pow_alt_2opt_inline", pow_alt_2opt_inline, base, max_exp);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
