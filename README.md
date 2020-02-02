# Faster Rust `u32::pow()` implementation
Bart Massey

This is a little demo of Rust benchmarking using the most
excellent `criterion-rs` package that shows a putative
speedup of 10-20% for Rust's `u32::pow()` implementation.
Interestingly, inlining this faster function seems to make
it perform slightly worse in some benchmarks.

This also shows experiments with optimizing for power-of-2
bases. It turns out to be a big win for the optimized
function, performing many times faster.

See the source for details.
