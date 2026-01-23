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
        #[c2rust::src_loc = "57:1"]
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t)
            -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:19"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/bits.h:20"]
pub mod bits_h {
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
    use super::_size_t_h::size_t;
    use super::stdbool_h::{false_0, true_0};
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:19"]
pub mod sys__types_h {
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/errno.h:19"]
pub mod errno_h {
    #[c2rust::src_loc = "100:9"]
    pub const ENOMEM: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
    extern "C" {
        #[c2rust::src_loc = "80:1"]
        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/llvm/21.1.8/lib/clang/21/include/stdbool.h:19"]
pub mod stdbool_h {
    #[c2rust::src_loc = "25:9"]
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
use self::_malloc_h::realloc;
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
pub use self::bits_h::safe_mul_size;
pub use self::errno_h::{__error, ENOMEM};
pub use self::stdbool_h::{false_0, true_0};
pub use self::sys__types_h::__DARWIN_NULL;
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
