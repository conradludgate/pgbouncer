#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:6"]
pub mod _types_h {
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:6"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:6"]
pub mod _uint8_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint8_t = u8;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/mbuf.h:6"]
pub mod mbuf_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "14:1"]
    pub struct MBuf {
        pub data: *mut uint8_t,
        pub read_pos: ::core::ffi::c_uint,
        pub write_pos: ::core::ffi::c_uint,
        pub alloc_len: ::core::ffi::c_uint,
        pub reader: bool,
        pub fixed: bool,
    }
    use super::_uint8_t_h::uint8_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:6"]
pub mod _malloc_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "57:1"]
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t)
            -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/llvm/21.1.8/lib/clang/21/include/stdbool.h:6"]
pub mod stdbool_h {
    #[c2rust::src_loc = "25:9"]
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
use self::_malloc_h::realloc;
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::mbuf_h::MBuf;
pub use self::stdbool_h::{false_0, true_0};
#[no_mangle]
#[c2rust::src_loc = "8:1"]
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
