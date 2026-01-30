//! Boolean parsing utilities for PostgreSQL
//!
//! Refactored to use shared types module.

use super::types::{false_0, size_t, strlen, true_0};

// pg_strncasecmp is defined in pgstrcasecmp.rs within this module
extern "C" {

    pub fn pg_strncasecmp(
        s1: *const ::core::ffi::c_char,
        s2: *const ::core::ffi::c_char,
        n: size_t,
    ) -> ::core::ffi::c_int;
}

#[no_mangle]

pub unsafe extern "C" fn parse_bool(
    mut value: *const ::core::ffi::c_char,
    mut result: *mut bool,
) -> bool {
    parse_bool_with_len(value, strlen(value), result)
}

#[no_mangle]

pub unsafe extern "C" fn parse_bool_with_len(
    mut value: *const ::core::ffi::c_char,
    mut len: size_t,
    mut result: *mut bool,
) -> bool {
    match *value as ::core::ffi::c_int {
        116 | 84 => {
            if pg_strncasecmp(value, c"true".as_ptr(), len) == 0 as ::core::ffi::c_int {
                if !result.is_null() {
                    *result = true;
                }
                return true;
            }
        }
        102 | 70 => {
            if pg_strncasecmp(value, c"false".as_ptr(), len) == 0 as ::core::ffi::c_int {
                if !result.is_null() {
                    *result = false;
                }
                return true;
            }
        }
        121 | 89 => {
            if pg_strncasecmp(value, c"yes".as_ptr(), len) == 0 as ::core::ffi::c_int {
                if !result.is_null() {
                    *result = true;
                }
                return true;
            }
        }
        110 | 78 => {
            if pg_strncasecmp(value, c"no".as_ptr(), len) == 0 as ::core::ffi::c_int {
                if !result.is_null() {
                    *result = false;
                }
                return true;
            }
        }
        111 | 79 => {
            if pg_strncasecmp(
                value,
                c"on".as_ptr(),
                if len > 2 as size_t { len } else { 2 as size_t },
            ) == 0 as ::core::ffi::c_int
            {
                if !result.is_null() {
                    *result = true;
                }
                return true;
            } else if pg_strncasecmp(
                value,
                c"off".as_ptr(),
                if len > 2 as size_t { len } else { 2 as size_t },
            ) == 0 as ::core::ffi::c_int
            {
                if !result.is_null() {
                    *result = false;
                }
                return true;
            }
        }
        49 => {
            if len == 1 as size_t {
                if !result.is_null() {
                    *result = true;
                }
                return true;
            }
        }
        48 => {
            if len == 1 as size_t {
                if !result.is_null() {
                    *result = false;
                }
                return true;
            }
        }
        _ => {}
    }
    if !result.is_null() {
        *result = false;
    }
    false
}
