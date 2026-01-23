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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:19"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = u32;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdlib.h:19"]
pub mod _stdlib_h {
    use super::_size_t_h::size_t;
    use super::_uint32_t_h::uint32_t;
    extern "C" {
        #[c2rust::src_loc = "282:1"]
        pub fn arc4random() -> uint32_t;
        #[c2rust::src_loc = "288:1"]
        pub fn arc4random_buf(__buf: *mut ::core::ffi::c_void, __nbytes: size_t);
        #[c2rust::src_loc = "290:1"]
        pub fn arc4random_uniform(__upper_bound: uint32_t) -> uint32_t;
    }
}
pub use self::_size_t_h::size_t;
use self::_stdlib_h::{arc4random, arc4random_buf, arc4random_uniform};
pub use self::_types_h::__darwin_size_t;
pub use self::_uint32_t_h::uint32_t;
#[no_mangle]
#[c2rust::src_loc = "33:1"]
pub unsafe extern "C" fn csrandom() -> uint32_t {
    arc4random()
}
#[no_mangle]
#[c2rust::src_loc = "38:1"]
pub unsafe extern "C" fn csrandom_bytes(mut buf: *mut ::core::ffi::c_void, mut nbytes: size_t) {
    arc4random_buf(buf, nbytes);
}
#[no_mangle]
#[c2rust::src_loc = "43:1"]
pub unsafe extern "C" fn csrandom_range(mut upper_bound: uint32_t) -> uint32_t {
    arc4random_uniform(upper_bound)
}
