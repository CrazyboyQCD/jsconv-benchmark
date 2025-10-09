#[target_feature(enable = "jsconv")]
pub unsafe fn f64_to_int32_arm64(number: f64) -> i32 {
    if !std::arch::is_aarch64_feature_detected!("jsconv") {
        let ret: i32;
        // SAFETY: Number is not nan so no floating-point exception should throw.
        unsafe {
            std::arch::asm!(
                "fjcvtzs {dst:w}, {src:d}",
                src = in(vreg) number,
                dst = out(reg) ret,
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
