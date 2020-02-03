use pow::*;

#[test]
fn test_ranges() {
    let args = vec![
        (0,31),
        (1,31),
        (2,31),
        (3,18),
        (7,11),
        (3,7),
    ];
    let funcs: Vec<(&str, fn(u32,u32)->u32)> = vec![
        ("pow_std", pow_std),
        ("pow_std_2opt", pow_std_2opt),
        ("pow_alt", pow_alt),
        ("pow_alt_2opt", pow_alt_2opt),
        ("pow_alt_01opt", pow_alt_01opt),
        ("pow_alt_012opt", pow_alt_012opt),
        ("pow_alt_inline", pow_alt_inline),
        ("pow_alt_2opt_inline", pow_alt_2opt_inline),
    ];
    for (base, max_exp) in args {
        for exp in 0..=max_exp {
            let answer = u32::pow(base, exp);
            for (name, func) in &funcs {
                println!("{}({},{})", name, base, exp);
                assert_eq!(answer, (*func)(base, exp));
            }
        }
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
