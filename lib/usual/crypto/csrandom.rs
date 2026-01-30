
pub mod _types_h {
    
    pub type __darwin_size_t = usize;
}

pub mod _size_t_h {
    
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}

pub mod _uint32_t_h {
    
    pub type uint32_t = u32;
}

pub mod _stdlib_h {
    use super::_size_t_h::size_t;
    use super::_uint32_t_h::uint32_t;
    extern "C" {
        
        pub fn arc4random() -> uint32_t;
        
        pub fn arc4random_buf(__buf: *mut ::core::ffi::c_void, __nbytes: size_t);
        
        pub fn arc4random_uniform(__upper_bound: uint32_t) -> uint32_t;
    }
}
pub use self::_size_t_h::size_t;
use self::_stdlib_h::{arc4random, arc4random_buf, arc4random_uniform};
pub use self::_types_h::__darwin_size_t;
pub use self::_uint32_t_h::uint32_t;
#[no_mangle]

pub unsafe extern "C" fn csrandom() -> uint32_t {
    arc4random()
}
#[no_mangle]

pub unsafe extern "C" fn csrandom_bytes(mut buf: *mut ::core::ffi::c_void, mut nbytes: size_t) {
    arc4random_buf(buf, nbytes);
}
#[no_mangle]

pub unsafe extern "C" fn csrandom_range(mut upper_bound: uint32_t) -> uint32_t {
    arc4random_uniform(upper_bound)
}
