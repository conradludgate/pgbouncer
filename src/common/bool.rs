#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:16"]
pub mod _types_h {
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:16"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:16"]
pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "96:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/builtins.h:18"]
pub mod builtins_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "18:1"]
        pub fn pg_strncasecmp(
            s1: *const ::core::ffi::c_char,
            s2: *const ::core::ffi::c_char,
            n: size_t,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/llvm/21.1.8/lib/clang/21/include/stdbool.h:16"]
pub mod stdbool_h {
    #[c2rust::src_loc = "25:9"]
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::_size_t_h::size_t;
use self::_string_h::strlen;
pub use self::_types_h::__darwin_size_t;
use self::builtins_h::pg_strncasecmp;
pub use self::stdbool_h::{false_0, true_0};
#[no_mangle]
#[c2rust::src_loc = "26:1"]
pub unsafe extern "C" fn parse_bool(
    mut value: *const ::core::ffi::c_char,
    mut result: *mut bool,
) -> bool {
    return parse_bool_with_len(value, strlen(value), result);
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
                (if len > 2 as size_t { len } else { 2 as size_t }),
            ) == 0 as ::core::ffi::c_int
            {
                if !result.is_null() {
                    *result = true_0 != 0;
                }
                return true_0 != 0;
            } else if pg_strncasecmp(
                value,
                b"off\0" as *const u8 as *const ::core::ffi::c_char,
                (if len > 2 as size_t { len } else { 2 as size_t }),
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
    return false_0 != 0;
}
