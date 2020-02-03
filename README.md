# Faster Rust `u32::pow()` implementation
Bart Massey

This is a little demo of Rust benchmarking using the most
excellent `criterion-rs` package that shows a putative
speedup of 10-20% for Rust's `u32::pow()` implementation.

This also shows experiments with optimizing for bases 0, 1,
and power-of-2. It turns out to be a big win for the
optimized function, performing many times faster, while also
performing faster in the general case.

Here is the
[`criterion-rs` report](https://BartMassey.github.io/rust-pow/criterion-report/report/index.html)
from my box — an Intel Haswell i7-4770K 3.9GHz with 32GB RAM
running Debian Linux. The benchmarks were *not* taken as
root on a quiescent system, but there wasn't much load:
pretty sure they had a core or two to themselves.

See the source for details of the work.
