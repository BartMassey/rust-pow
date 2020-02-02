# Faster Rust `u32::pow()` implementation
Bart Massey

This is a little demo of Rust benchmarking using the most
excellent `criterion-rs` package that shows a putative
speedup of 10-20% for Rust's `u32::pow()` implementation.
Interestingly, inlining this faster function seems to make
it perform worse in the benchmarks.

See the source for details.
