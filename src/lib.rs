/// `pow()` from `std`.
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
    if base & (base - 1) == 0 {
        if base == 0 {
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

/// Alternate `pow()` uninlined.
#[inline(never)]
pub fn pow_alt_2opt(mut base: u32, mut exp: u32) -> u32 {
    if base & (base - 1) == 0 {
        if base == 0 {
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
pub fn pow_alt_2opt_inline(mut base: u32, mut exp: u32) -> u32 {
    if base & (base - 1) == 0 {
        if base == 0 {
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

#[test]
fn test_pow2() {
    for i in 0..32 {
        assert_eq!(u32::pow(2, i), pow_alt(2, i));
    }
}

#[test]
fn test_pow2_2opt() {
    for i in 0..32 {
        assert_eq!(u32::pow(2, i), pow_std_2opt(2, i));
    }
}

#[test]
fn test_pow4_2opt() {
    for i in 0..8 {
        assert_eq!(u32::pow(4, i), pow_std_2opt(4, i));
    }
}

#[test]
fn test_pow_large() {
    for i in 0..2 {
        assert_eq!(u32::pow(0xffff, i), pow_alt(0xffff, i));
    }
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn test_pow_std_too_large() {
    let _ = pow_std(0x10000, 2);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn test_pow_alt_too_large() {
    let _ = pow_alt(0x10000, 2);
}

#[cfg(debug_assertions)]
#[test]
fn test_pow_std_just_right() {
    let _ = pow_std(0x10000, 1);
}

#[cfg(debug_assertions)]
#[test]
fn test_pow_alt_just_right() {
    let _ = pow_alt(0x10000, 1);
}
