use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

use jsconv_aarch64::f64_to_int32_aarch64;
use jsconv_aarch64::f64_to_int32_aarch64_core_intrinsics;
use jsconv_aarch64::f64_to_int32_aarch64_reverse;
use jsconv_aarch64::f64_to_int32_generic;

fn bench_jsconv(c: &mut Criterion) {
    let fixtures = [
        0.0,
        -0.0,
        1.0,
        -1.0,
        42.7,
        -42.7,
        f64::from(i32::MAX),
        f64::from(i32::MIN),
        f64::from(i32::MAX) + 1.0,
        f64::from(i32::MIN) - 1.0,
        9_007_199_254_740_992.0,  // 2^53
        -9_007_199_254_740_992.0, // -2^53
        f64::NAN,
        -f64::NAN,
    ];

    for (i, &num) in fixtures.iter().enumerate() {
        assert_eq!(
            unsafe { f64_to_int32_aarch64(num) },
            f64_to_int32_generic(num),
            "fixtures[{i}]: {num}",
        );
    }

    c.bench_function("f64_to_int32_generic", |b| {
        b.iter(|| {
            for fixture in fixtures {
                black_box(f64_to_int32_generic(fixture));
            }
        });
    });

    c.bench_function("f64_to_int32_aarch64_core_intrinsics", |b| {
        b.iter(|| {
            for fixture in fixtures {
                black_box(unsafe { f64_to_int32_aarch64_core_intrinsics(fixture) });
            }
        });
    });

    c.bench_function("f64_to_int32_aarch64", |b| {
        b.iter(|| {
            for fixture in fixtures {
                black_box(unsafe { f64_to_int32_aarch64(fixture) });
            }
        });
    });

    c.bench_function("f64_to_int32_aarch64_reverse", |b| {
        b.iter(|| {
            for fixture in fixtures {
                black_box(unsafe { f64_to_int32_aarch64_reverse(fixture) });
            }
        });
    });
}

criterion_group!(benches, bench_jsconv);
criterion_main!(benches);
