//! Boolean parsing utilities for PostgreSQL
//!
//! Refactored to use shared types module.

use super::types::{false_0, size_t, strlen, true_0};

// pg_strncasecmp is defined in pgstrcasecmp.rs within this module
extern "C" {
    #[c2rust::src_loc = "18:1"]
    pub fn pg_strncasecmp(
        s1: *const ::core::ffi::c_char,
        s2: *const ::core::ffi::c_char,
        n: size_t,
    ) -> ::core::ffi::c_int;
}

#[no_mangle]
#[c2rust::src_loc = "26:1"]
pub unsafe extern "C" fn parse_bool(
    mut value: *const ::core::ffi::c_char,
    mut result: *mut bool,
) -> bool {
    parse_bool_with_len(value, strlen(value), result)
}

#[no_mangle]
#[c2rust::src_loc = "32:1"]
pub unsafe extern "C" fn parse_bool_with_len(
    mut value: *const ::core::ffi::c_char,
    mut len: size_t,
    mut result: *mut bool,
) -> bool {
    match *value as ::core::ffi::c_int {
        116 | 84 => {
            if pg_strncasecmp(
                value,
                b"true\0" as *const u8 as *const ::core::ffi::c_char,
                len,
            ) == 0 as ::core::ffi::c_int
            {
                if !result.is_null() {
                    *result = true_0 != 0;
                }
                return true_0 != 0;
            }
        }
        102 | 70 => {
            if pg_strncasecmp(
                value,
                b"false\0" as *const u8 as *const ::core::ffi::c_char,
                len,
            ) == 0 as ::core::ffi::c_int
            {
                if !result.is_null() {
                    *result = false_0 != 0;
                }
                return true_0 != 0;
            }
        }
        121 | 89 => {
            if pg_strncasecmp(
                value,
                b"yes\0" as *const u8 as *const ::core::ffi::c_char,
                len,
            ) == 0 as ::core::ffi::c_int
            {
                if !result.is_null() {
                    *result = true_0 != 0;
                }
                return true_0 != 0;
            }
        }
        110 | 78 => {
            if pg_strncasecmp(
                value,
                b"no\0" as *const u8 as *const ::core::ffi::c_char,
                len,
            ) == 0 as ::core::ffi::c_int
            {
                if !result.is_null() {
                    *result = false_0 != 0;
                }
                return true_0 != 0;
            }
        }
        111 | 79 => {
            if pg_strncasecmp(
                value,
                b"on\0" as *const u8 as *const ::core::ffi::c_char,
                if len > 2 as size_t { len } else { 2 as size_t },
            ) == 0 as ::core::ffi::c_int
            {
                if !result.is_null() {
                    *result = true_0 != 0;
                }
                return true_0 != 0;
            } else if pg_strncasecmp(
                value,
                b"off\0" as *const u8 as *const ::core::ffi::c_char,
                if len > 2 as size_t { len } else { 2 as size_t },
            ) == 0 as ::core::ffi::c_int
            {
                if !result.is_null() {
                    *result = false_0 != 0;
                }
                return true_0 != 0;
            }
        }
        49 => {
            if len == 1 as size_t {
                if !result.is_null() {
                    *result = true_0 != 0;
                }
                return true_0 != 0;
            }
        }
        48 => {
            if len == 1 as size_t {
                if !result.is_null() {
                    *result = false_0 != 0;
                }
                return true_0 != 0;
            }
        }
        _ => {}
    }
    if !result.is_null() {
        *result = false_0 != 0;
    }
    false_0 != 0
}
