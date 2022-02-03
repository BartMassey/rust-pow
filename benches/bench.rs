use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    BenchmarkId,
    Criterion,
    measurement::Measurement,
};

use pow::*;

macro_rules! bench_pow {
    ($c: expr, $name:expr, $func:tt, $args:expr) => {{
        for &(base, max_exp) in $args {
            let mut group = $c.benchmark_group($name);
            for exp in 0..=max_exp {
                group.bench_function(
                    BenchmarkId::new(base.to_string(), exp),
                    |b| b.iter(|| {
                        black_box($func(black_box(base), black_box(exp)))
                    }),
                );
            }
            group.finish();
        }
    }};
}

pub fn criterion_benchmark<P: Measurement>(c: &mut Criterion<P>) {
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

#[cfg(feature = "count-cycles")]
mod cg {
    use super::*;
    use criterion_perf_events::Perf;
    use perfcnt::linux::HardwareEventType as Hardware;
    use perfcnt::linux::PerfCounterBuilderLinux as Builder;
    criterion_group!(
        name = benches;
        config = Criterion::default().with_measurement(
            Perf::new(Builder::from_hardware_event(Hardware::Instructions))
        );
        targets = criterion_benchmark
    );
}

#[cfg(not(feature = "count-cycles"))]
mod cg {
    use super::*;
    criterion_group!(benches, criterion_benchmark);
}

criterion_main!(cg::benches);
