# Faster Rust `u32::pow()` implementation
Bart Massey

This is a little demo of Rust benchmarking using the most
excellent `criterion-rs` package that shows a putative
speedup for Rust's `u32::pow()` implementation.

This also shows experiments with optimizing for bases 0, 1,
and power-of-2. It turns out to be a big win for the
optimized cases, performing roughly twice as fast. The
optimized function also performs 10-15% faster than `std` in
the general case.

* Here is the
[`criterion-rs` report](https://BartMassey.github.io/rust-pow/criterion-report-ryzen9/report/index.html)
from my latest box — an AMD Ryzen 9 3900X 3.8GHz with 32GB RAM
running Debian Linux, compiled with the default options.

* Here is the
[`criterion-rs` report](https://BartMassey.github.io/rust-pow/criterion-report-haswell/report/index.html)
from my older box — an Intel Haswell i7-4770K 3.9GHz with 32GB RAM
running Debian Linux, compiled with the default options.

* Here is the
[`criterion-rs` report](https://BartMassey.github.io/rust-pow/criterion-report-i586/report/index.html)
from my older box, but compiled using target
`i586-unknown-linux-musl` to get a sense of how different
the results might be on a hypothetical 32-bit machine with
no vector instructions.

The benchmarks were *not* taken as
root on a quiescent system, but there wasn't much load:
pretty sure they had a core or two to themselves.

See the source for details of the work.
