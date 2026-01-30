//! Base utilities for usual library
//!
//! Refactored to use shared types module.

use crate::types::{__error, false_0, realloc, size_t, true_0, ENOMEM, __DARWIN_NULL};

pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;

#[inline]
#[c2rust::src_loc = "228:1"]
pub unsafe extern "C" fn safe_mul_size(
    mut res_p: *mut size_t,
    mut a: size_t,
    mut b: size_t,
) -> bool {
    let mut unsafe_0: size_t = (1 as ::core::ffi::c_int as size_t)
        << ::core::mem::size_of::<size_t>()
            .wrapping_mul(8_usize)
            .wrapping_div(2_usize);
    if !(a < unsafe_0 && b < unsafe_0)
        && !(a == 0 || b == 0)
        && ((18446744073709551615 as size_t).wrapping_div(a) < b)
    {
        return false_0 != 0;
    }
    *res_p = a.wrapping_mul(b);
    true_0 != 0
}

#[no_mangle]
#[c2rust::src_loc = "57:1"]
pub unsafe extern "C" fn usual_reallocarray(
    mut p: *mut ::core::ffi::c_void,
    mut count: size_t,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    let mut total: size_t = 0;
    if !safe_mul_size(&raw mut total, count, size) {
        *__error() = ENOMEM;
        return NULL;
    }
    realloc(p, total)
}
