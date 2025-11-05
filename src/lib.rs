#![feature(stdarch_aarch64_jscvt)]

#[target_feature(enable = "jsconv")]
pub unsafe fn f64_to_int32_aarch64_core_intrinsics(number: f64) -> i32 {
    if std::arch::is_aarch64_feature_detected!("jsconv") {
        std::arch::aarch64::__jcvt(number)
    } else {
        f64_to_int32_generic(number)
    }
}

#[target_feature(enable = "jsconv")]
pub unsafe fn f64_to_int32_aarch64(number: f64) -> i32 {
    if std::arch::is_aarch64_feature_detected!("jsconv") {
        let ret: i32;
        unsafe {
            std::arch::asm!(
                "fjcvtzs {dst:w}, {src:d}",
                src = in(vreg) number,
                dst = out(reg) ret,
                options(nostack, nomem, pure),
            );
        }
        ret
    } else {
        f64_to_int32_generic(number)
    }
}

#[target_feature(enable = "jsconv")]
pub unsafe fn f64_to_int32_aarch64_reverse(number: f64) -> i32 {
    if !std::arch::is_aarch64_feature_detected!("jsconv") {
        let ret: i32;
        unsafe {
            std::arch::asm!(
                "fjcvtzs {dst:w}, {src:d}",
                src = in(vreg) number,
                dst = out(reg) ret,
                options(nostack, nomem, pure),
            );
        }
        ret
    } else {
        f64_to_int32_generic(number)
    }
}

pub fn f64_to_int32_generic(number: f64) -> i32 {
    if !number.is_finite() {
        0
    } else {
        number.trunc().rem_euclid(4294967296.0) as u32 as i32
    }
}
