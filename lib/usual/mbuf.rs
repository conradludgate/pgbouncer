//! Memory buffer utilities
//!
//! Refactored to use shared types module.

use crate::types::{false_0, realloc, size_t, true_0, uint8_t, MBuf};

#[no_mangle]

pub unsafe extern "C" fn mbuf_make_room(mut buf: *mut MBuf, mut len: ::core::ffi::c_uint) -> bool {
    let mut new_alloc = (*buf).alloc_len;
    let mut ptr = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if (*buf).reader as ::core::ffi::c_int != 0 || (*buf).fixed as ::core::ffi::c_int != 0 {
        return false_0 != 0;
    }
    if (*buf).write_pos.wrapping_add(len) <= (*buf).alloc_len {
        return true_0 != 0;
    }
    if new_alloc == 0 as ::core::ffi::c_uint {
        new_alloc = 128 as ::core::ffi::c_uint;
    }
    while new_alloc < (*buf).write_pos.wrapping_add(len) {
        new_alloc = new_alloc.wrapping_mul(2 as ::core::ffi::c_uint);
    }
    ptr = realloc((*buf).data as *mut ::core::ffi::c_void, new_alloc as size_t);
    if ptr.is_null() {
        return false_0 != 0;
    }
    (*buf).data = ptr as *mut uint8_t;
    (*buf).alloc_len = new_alloc;
    true_0 != 0
}
