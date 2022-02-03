# Faster Rust `u32::pow()` implementation
Bart Massey

This is a little demo of Rust benchmarking using the most
excellent `criterion-rs` package that shows a putative
speedup for Rust's `u32::pow()` implementation.

This also shows experiments with optimizing for bases 0, 1,
and power-of-2. It turns out to be a big win for the
optimized cases. The optimized function also performs a
little faster than `std` in the general case.

The benchmarks should be runnable on stable Rust on most any
platform using `cargo bench`. They will report wall-clock
time, which is pretty fuzzy for these kinds of measurements.

On Linux-compatible platforms, instruction cycle counting
using hardware performance counters can be enabled via the
`count-cycles` feature. This will require nightly Rust
(because `asm` isn't stabilized quite yet) and running as
root (to get access to the performance counters). You can
run much shorter per benchmark, since the cycle counter is
pretty accurate. (It would be nice to run less than a
second, so I will work on
[this issue](https://github.com/bheisler/criterion.rs/issues/551).)
Run it all like this:

```
rustup override set nightly-2022-01-19
sudo cargo bench --features=count-cycles --bench=bench -- \
  --measurement-time=1 --warm-up-time=1
```

Many thanks to the authors of the `criterion`,
`criterion-perf-events` and `perfcnt` crates.

Thanks to this
[blog post](https://www.reddit.com/r/rust/comments/sh8u72/why_my_rust_benchmarks_were_wrong_or_how_to/)
for pointing out where I was going wrong with `criterion::black_box()`.
