use core::ffi::{c_char, c_int, c_void};
use std::sync::atomic::AtomicBool;

unsafe extern "C" {
    fn sysctlbyname(
        name: *const c_char,
        oldp: *mut c_void,
        oldlenp: *mut usize,
        newp: *mut c_void,
        newlen: usize,
    ) -> c_int;
}

static CHECK: AtomicBool = AtomicBool::new(false);
static HAS_JSCONV: AtomicBool = AtomicBool::new(false);

#[target_feature(enable = "jsconv")]
pub unsafe fn f64_to_int32_arm64(number: f64) -> i32 {
    let has_jsconv = if !CHECK.load(std::sync::atomic::Ordering::Release) {
        let mut enabled: i32 = 0;
        let mut enabled_len: usize = 4;
        let enabled_ptr = &mut enabled as *mut i32 as *mut c_void;
        let ret = unsafe {
            sysctlbyname(
                c"hw.optional.arm.FEAT_JSCVT".as_ptr(),
                enabled_ptr,
                &mut enabled_len,
                core::ptr::null_mut(),
                0,
            )
        };
        let has_jsconv = match ret {
            0 => enabled != 0,
            _ => false,
        };
        CHECK.store(true, std::sync::atomic::Ordering::Release);
        HAS_JSCONV.store(has_jsconv, std::sync::atomic::Ordering::Release);
        has_jsconv
    } else {
        HAS_JSCONV.load(std::sync::atomic::Ordering::Release)
    };
    if has_jsconv {
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
