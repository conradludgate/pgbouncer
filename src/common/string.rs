//! String utilities for PostgreSQL
//!
//! Refactored to use shared types module.

use super::types::{
    __error, false_0, malloc, size_t, snprintf, strcmp, strlen, strtol, true_0, __DARWIN_NULL,
};

pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const ERANGE: ::core::ffi::c_int = 34;
pub const HIGHBIT: ::core::ffi::c_int = 0x80;

#[no_mangle]
#[c2rust::src_loc = "26:1"]
pub unsafe extern "C" fn pg_str_endswith(
    mut str: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> bool {
    let mut slen = strlen(str);
    let mut elen = strlen(end);
    if elen > slen {
        return false_0 != 0;
    }
    str = str.add(slen.wrapping_sub(elen));
    strcmp(str, end) == 0 as ::core::ffi::c_int
}

#[no_mangle]
#[c2rust::src_loc = "45:1"]
pub unsafe extern "C" fn strtoint(
    mut str: *const ::core::ffi::c_char,
    mut endptr: *mut *mut ::core::ffi::c_char,
    mut base: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut val: ::core::ffi::c_long = 0;
    val = strtol(str, endptr, base);
    if val != val as ::core::ffi::c_int as ::core::ffi::c_long {
        *__error() = ERANGE;
    }
    val as ::core::ffi::c_int
}

#[no_mangle]
#[c2rust::src_loc = "80:1"]
pub unsafe extern "C" fn pg_clean_ascii(
    mut str: *const ::core::ffi::c_char,
    mut _alloc_flags: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut dstlen: size_t = 0;
    let mut dst = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p = ::core::ptr::null::<::core::ffi::c_char>();
    let mut i: size_t = 0 as size_t;
    dstlen = strlen(str)
        .wrapping_mul(4 as size_t)
        .wrapping_add(1 as size_t);
    dst = malloc(dstlen) as *mut ::core::ffi::c_char;
    if dst.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    p = str;
    while *p as ::core::ffi::c_int != '\0' as i32 {
        if (*p as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
            || *p as ::core::ffi::c_int > 126 as ::core::ffi::c_int
        {
            snprintf(
                dst.add(i) as *mut ::core::ffi::c_char,
                dstlen.wrapping_sub(i),
                b"\\x%02x\0" as *const u8 as *const ::core::ffi::c_char,
                *p as ::core::ffi::c_uchar as ::core::ffi::c_int,
            );
            i = i.wrapping_add(4 as size_t);
        } else {
            *dst.add(i) = *p;
            i = i.wrapping_add(1);
        }
        p = p.offset(1);
    }
    *dst.add(i) = '\0' as i32 as ::core::ffi::c_char;
    dst
}

#[no_mangle]
#[c2rust::src_loc = "127:1"]
pub unsafe extern "C" fn pg_is_ascii(mut str: *const ::core::ffi::c_char) -> bool {
    while *str != 0 {
        if *str as ::core::ffi::c_uchar as ::core::ffi::c_int & HIGHBIT != 0 {
            return false_0 != 0;
        }
        str = str.offset(1);
    }
    true_0 != 0
}

#[no_mangle]
#[c2rust::src_loc = "149:1"]
pub unsafe extern "C" fn pg_strip_crlf(mut str: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut len = strlen(str) as ::core::ffi::c_int;
    while len > 0 as ::core::ffi::c_int
        && (*str.offset((len - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            == '\n' as i32
            || *str.offset((len - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == '\r' as i32)
    {
        len -= 1;
        *str.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    len
}
