/// Copy of `pow()` from `std`.
#[inline(never)]
pub fn pow_std(mut base: u32, mut exp: u32) -> u32 {
    let mut acc = 1;

    while exp > 1 {
        if (exp & 1) == 1 {
            acc *= base;
        }
        exp /= 2;
        base = base * base;
    }

    // Deal with the final bit of the exponent separately, since
    // squaring the base afterwards is not necessary and may cause a
    // needless overflow.
    if exp == 1 {
        acc *= base;
    }

    acc
}

/// `pow()` from `std` with power-of-2 optimization.
#[inline(never)]
pub fn pow_std_2opt(mut base: u32, mut exp: u32) -> u32 {
    if base & base.wrapping_sub(1) == 0 {
        if base == 0 {
            if exp == 0 {
                return 1;
            }
            return 0;
        }
        let base_pow = base.trailing_zeros();
        return 1 << (base_pow * exp);
    }

    let mut acc = 1;
    while exp > 1 {
        if (exp & 1) == 1 {
            acc *= base;
        }
        exp /= 2;
        base = base * base;
    }

    // Deal with the final bit of the exponent separately, since
    // squaring the base afterwards is not necessary and may cause a
    // needless overflow.
    if exp == 1 {
        acc *= base;
    }

    acc
}

/// Alternate `pow()` uninlined.
#[inline(never)]
pub fn pow_alt(mut base: u32, mut exp: u32) -> u32 {
    let mut acc = 1;

    loop {
        if (exp & 1) == 1 {
            acc *= base;
        }
        exp /= 2;
        if exp == 0 {
            return acc;
        }
        base = base * base;
    }
}

/// Alternate `pow()` uninlined with 01-opt.
#[inline(never)]
pub fn pow_alt_01opt(mut base: u32, mut exp: u32) -> u32 {
    if base <= 1 {
        if exp == 0 {
            return 1;
        }
        return base;
    }
    let mut acc = 1;
    loop {
        if (exp & 1) == 1 {
            acc *= base;
        }
        exp /= 2;
        if exp == 0 {
            return acc;
        }
        base = base * base;
    }
}

/// Alternate `pow()` uninlined with 01-opt and 2-opt.
#[inline(never)]
pub fn pow_alt_012opt(mut base: u32, mut exp: u32) -> u32 {
    if base & base.wrapping_sub(1) == 0 {
        if base <= 1 {
            if exp == 0 {
                return 1;
            }
            return base;
        }
        let base_pow = base.trailing_zeros();
        return 1 << (base_pow * exp);
    }

    let mut acc = 1;
    loop {
        if (exp & 1) == 1 {
            acc *= base;
        }
        exp /= 2;
        if exp == 0 {
            return acc;
        }
        base = base * base;
    }
}

/// Alternate `pow()` uninlined with 0-opt and 2-opt.
#[inline(never)]
pub fn pow_alt_02opt(mut base: u32, mut exp: u32) -> u32 {
    if base & base.wrapping_sub(1) == 0 {
        if base == 0 {
            if exp == 0 {
                return 1;
            }
            return 0;
        }
        let base_pow = base.trailing_zeros();
        return 1 << (base_pow * exp);
    }

    let mut acc = 1;
    loop {
        if (exp & 1) == 1 {
            acc *= base;
        }
        exp /= 2;
        if exp == 0 {
            return acc;
        }
        base = base * base;
    }
}

/// Alternate `pow()` always inlined.
#[inline(always)]
pub fn pow_alt_inline(mut base: u32, mut exp: u32) -> u32 {
    let mut acc = 1;

    loop {
        if (exp & 1) == 1 {
            acc *= base;
        }
        exp /= 2;
        if exp == 0 {
            return acc;
        }
        base = base * base;
    }
}

/// Alternate `pow()` uninlined with 2-opt.
#[inline(never)]
pub fn pow_alt_2opt(mut base: u32, mut exp: u32) -> u32 {
    if base & base.wrapping_sub(1) == 0 {
        if base == 0 {
            if exp == 0 {
                return 1;
            }
            return 0;
        }
        let base_pow = base.trailing_zeros();
        return 1 << (base_pow * exp);
    }

    let mut acc = 1;
    loop {
        if (exp & 1) == 1 {
            acc *= base;
        }
        exp /= 2;
        if exp == 0 {
            return acc;
        }
        base = base * base;
    }
}

/// Alternate `pow()` always inlined with 2-opt.
#[inline(always)]
pub fn pow_alt_2opt_inline(mut base: u32, mut exp: u32) -> u32 {
    if base & base.wrapping_sub(1) == 0 {
        if base == 0 {
            if exp == 0 {
                return 1;
            }
            return 0;
        }
        let base_pow = base.trailing_zeros();
        return 1 << (base_pow * exp);
    }

    let mut acc = 1;
    loop {
        if (exp & 1) == 1 {
            acc *= base;
        }
        exp /= 2;
        if exp == 0 {
            return acc;
        }
        base = base * base;
    }
}
