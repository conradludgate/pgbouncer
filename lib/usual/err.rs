
pub mod _types_h {
    
    pub type __darwin_size_t = usize;
}

pub mod _size_t_h {
    
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}

pub mod _malloc_h {
    use super::_size_t_h::size_t;
    extern "C" {
        
        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
        
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t)
            -> *mut ::core::ffi::c_void;
    }
}

pub mod err_h {
    extern "C" {
        
        pub fn err(_: ::core::ffi::c_int, _: *const ::core::ffi::c_char, ...) -> !;
    }
}

pub mod _string_h {
    extern "C" {
        
        pub fn strdup(__s1: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    }
}
use self::_malloc_h::{malloc, realloc};
pub use self::_size_t_h::size_t;
use self::_string_h::strdup;
pub use self::_types_h::__darwin_size_t;
use self::err_h::err;
#[no_mangle]

pub unsafe extern "C" fn xmalloc(mut len: size_t) -> *mut ::core::ffi::c_void {
    let mut p = malloc(len);
    if p.is_null() {
        err(
            1 as ::core::ffi::c_int,
            b"no mem\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    p
}
#[no_mangle]

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
    p2
}
#[no_mangle]

pub unsafe extern "C" fn xstrdup(mut s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut s2 = strdup(s) as *mut ::core::ffi::c_void;
    if s2.is_null() {
        err(
            1 as ::core::ffi::c_int,
            b"no mem\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    s2 as *mut ::core::ffi::c_char
}
