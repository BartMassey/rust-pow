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
[`criterion-rs` report](https://BartMassey.github.io/rust-pow/criterion-report/report/index.html) from my
box.

See the source for details of the work.
