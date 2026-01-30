
pub mod _types_h {
    
    pub type __darwin_size_t = usize;
}

pub mod _size_t_h {
    
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}

pub mod _uint8_t_h {
    
    pub type uint8_t = u8;
}

pub mod _uint32_t_h {
    
    pub type uint32_t = u32;
}

pub mod _uint64_t_h {
    
    pub type uint64_t = u64;
}

pub mod keccak_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct KeccakContext {
        pub u: C2RustUnnamed,
        pub pos: uint32_t,
        pub rbytes: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union C2RustUnnamed {
        pub state64: [uint64_t; 25],
        pub state32: [uint32_t; 50],
    }
    use super::_size_t_h::size_t;
    use super::_uint32_t_h::uint32_t;
    use super::_uint64_t_h::uint64_t;
    use super::_uint8_t_h::uint8_t;
    extern "C" {
        
        pub fn keccak_init(
            ctx: *mut KeccakContext,
            capacity: ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int;
        
        pub fn keccak_absorb(
            ctx: *mut KeccakContext,
            data: *const ::core::ffi::c_void,
            len: size_t,
        );
        
        pub fn keccak_squeeze(ctx: *mut KeccakContext, dst: *mut uint8_t, len: size_t);
        
        pub fn keccak_pad(ctx: *mut KeccakContext, data: *const ::core::ffi::c_void, len: size_t);
        
        pub fn keccak_rewind(ctx: *mut KeccakContext);
    }
}

pub mod keccak_prng_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct KeccakPRNG {
        pub ctx: KeccakContext,
        pub extracting: bool,
        pub have_data: bool,
    }
    use super::keccak_h::KeccakContext;
}

pub mod stdbool_h {
    
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::keccak_h::{
    keccak_absorb, keccak_init, keccak_pad, keccak_rewind, keccak_squeeze, C2RustUnnamed,
    KeccakContext,
};
pub use self::keccak_prng_h::KeccakPRNG;
pub use self::stdbool_h::{false_0, true_0};
#[no_mangle]

pub unsafe extern "C" fn keccak_prng_init(
    mut prng: *mut KeccakPRNG,
    mut capacity: ::core::ffi::c_int,
) -> bool {
    if keccak_init(&raw mut (*prng).ctx, capacity as ::core::ffi::c_uint) == 0 {
        return false_0 != 0;
    }
    (*prng).extracting = false_0 != 0;
    (*prng).have_data = false_0 != 0;
    true_0 != 0
}
#[no_mangle]

pub unsafe extern "C" fn keccak_prng_add_data(
    mut prng: *mut KeccakPRNG,
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    if (*prng).extracting {
        keccak_rewind(&raw mut (*prng).ctx);
        (*prng).extracting = false_0 != 0;
    }
    keccak_absorb(&raw mut (*prng).ctx, data, len);
    if !(*prng).have_data && len > 0 as size_t {
        (*prng).have_data = true_0 != 0;
    }
}
#[no_mangle]

pub unsafe extern "C" fn keccak_prng_extract(
    mut prng: *mut KeccakPRNG,
    mut data: *mut ::core::ffi::c_void,
    mut len: size_t,
) -> bool {
    if !(*prng).have_data {
        return false_0 != 0;
    }
    if !(*prng).extracting {
        keccak_pad(
            &raw mut (*prng).ctx,
            b"\x01\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            1 as size_t,
        );
        (*prng).extracting = true_0 != 0;
    }
    keccak_squeeze(&raw mut (*prng).ctx, data as *mut uint8_t, len);
    true_0 != 0
}
