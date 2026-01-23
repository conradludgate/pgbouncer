#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:19"]
pub mod _types_h {
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:19"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:19"]
pub mod _malloc_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "54:1"]
        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "57:1"]
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t)
            -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/err.h:19"]
pub mod err_h {
    extern "C" {
        #[c2rust::src_loc = "76:1"]
        pub fn err(_: ::core::ffi::c_int, _: *const ::core::ffi::c_char, ...) -> !;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:21"]
pub mod _string_h {
    extern "C" {
        #[c2rust::src_loc = "141:1"]
        pub fn strdup(__s1: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    }
}
use self::_malloc_h::{malloc, realloc};
pub use self::_size_t_h::size_t;
use self::_string_h::strdup;
pub use self::_types_h::__darwin_size_t;
use self::err_h::err;
#[no_mangle]
#[c2rust::src_loc = "106:1"]
pub unsafe extern "C" fn xmalloc(mut len: size_t) -> *mut ::core::ffi::c_void {
    let mut p = malloc(len);
    if p.is_null() {
        err(
            1 as ::core::ffi::c_int,
            b"no mem\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "114:1"]
pub unsafe extern "C" fn xrealloc(
    mut p: *mut ::core::ffi::c_void,
    mut len: size_t,
) -> *mut ::core::ffi::c_void {
    let mut p2 = realloc(p, len);
    if p2.is_null() {
        err(
            1 as ::core::ffi::c_int,
            b"no mem\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return p2;
}
#[no_mangle]
#[c2rust::src_loc = "122:1"]
pub unsafe extern "C" fn xstrdup(mut s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut s2 = strdup(s) as *mut ::core::ffi::c_void;
    if s2.is_null() {
        err(
            1 as ::core::ffi::c_int,
            b"no mem\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return s2 as *mut ::core::ffi::c_char;
}
