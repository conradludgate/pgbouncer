#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:18"]
pub mod _types_h {
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:18"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:18"]
pub mod _stdio_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "435:1"]
        pub fn snprintf(
            __str: *mut ::core::ffi::c_char,
            __size: size_t,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:18"]
pub mod _malloc_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "54:1"]
        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:18"]
pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "89:1"]
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "96:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:18"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:18"]
pub mod sys__types_h {
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdlib.h:18"]
pub mod _stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "186:1"]
        pub fn strtol(
            __str: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
            __base: ::core::ffi::c_int,
        ) -> ::core::ffi::c_long;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/errno.h:18"]
pub mod errno_h {
    #[c2rust::src_loc = "126:9"]
    pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
    extern "C" {
        #[c2rust::src_loc = "80:1"]
        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/llvm/21.1.8/lib/clang/21/include/stdbool.h:18"]
pub mod stdbool_h {
    #[c2rust::src_loc = "25:9"]
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/postgres_compat.h:18"]
pub mod postgres_compat_h {
    #[c2rust::src_loc = "32:9"]
    pub const HIGHBIT: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
}
use self::_malloc_h::malloc;
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
use self::_stdio_h::snprintf;
use self::_stdlib_h::strtol;
use self::_string_h::{strcmp, strlen};
pub use self::_types_h::__darwin_size_t;
pub use self::errno_h::{__error, ERANGE};
pub use self::postgres_compat_h::HIGHBIT;
pub use self::stdbool_h::{false_0, true_0};
pub use self::sys__types_h::__DARWIN_NULL;
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
    str = str.offset(slen.wrapping_sub(elen) as isize);
    return strcmp(str, end) == 0 as ::core::ffi::c_int;
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
    return val as ::core::ffi::c_int;
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
                dst.offset(i as isize) as *mut ::core::ffi::c_char,
                dstlen.wrapping_sub(i),
                b"\\x%02x\0" as *const u8 as *const ::core::ffi::c_char,
                *p as ::core::ffi::c_uchar as ::core::ffi::c_int,
            );
            i = i.wrapping_add(4 as size_t);
        } else {
            *dst.offset(i as isize) = *p;
            i = i.wrapping_add(1);
        }
        p = p.offset(1);
    }
    *dst.offset(i as isize) = '\0' as i32 as ::core::ffi::c_char;
    return dst;
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
    return true_0 != 0;
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
    return len;
}
