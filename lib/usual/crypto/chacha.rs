#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:23"]
pub mod _types_h {
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:23"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:23"]
pub mod _uint8_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint8_t = u8;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:23"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = u32;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/crypto/chacha.h:23"]
pub mod chacha_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:1"]
    pub struct ChaCha {
        pub state: [uint32_t; 16],
        pub u: C2RustUnnamed,
        pub pos: ::core::ffi::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:2"]
    pub union C2RustUnnamed {
        pub output32: [uint32_t; 16],
        pub output8: [uint8_t; 64],
    }
    #[c2rust::src_loc = "31:9"]
    pub const CHACHA_BLOCK_SIZE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
    use super::_uint32_t_h::uint32_t;
    use super::_uint8_t_h::uint8_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:25"]
pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "77:1"]
        pub fn memcpy(
            __dst: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/endian.h:25"]
pub mod endian_h {
    #[inline]
    #[c2rust::src_loc = "274:1"]
    pub unsafe extern "C" fn usual_le32dec(mut p: *const ::core::ffi::c_void) -> uint32_t {
        let mut tmp: uint32_t = 0;
        memcpy(
            &raw mut tmp as *mut ::core::ffi::c_void,
            p,
            ::core::mem::size_of::<uint32_t>() as size_t,
        );
        return tmp;
    }
    use super::_size_t_h::size_t;
    use super::_string_h::memcpy;
    use super::_uint32_t_h::uint32_t;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/bits.h:26"]
pub mod bits_h {
    #[inline]
    #[c2rust::src_loc = "50:1"]
    pub unsafe extern "C" fn rol32(mut v: uint32_t, mut s: ::core::ffi::c_int) -> uint32_t {
        return v << s | v >> 32 as ::core::ffi::c_int - s;
    }
    use super::_uint32_t_h::uint32_t;
}
pub use self::_size_t_h::size_t;
use self::_string_h::memcpy;
pub use self::_types_h::__darwin_size_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::bits_h::rol32;
pub use self::chacha_h::{C2RustUnnamed, ChaCha, CHACHA_BLOCK_SIZE};
pub use self::endian_h::usual_le32dec;
#[c2rust::src_loc = "28:9"]
pub const CHACHA_ROUNDS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
#[c2rust::src_loc = "47:1"]
unsafe extern "C" fn chacha_mix(mut ctx: *mut ChaCha) {
    let mut input: *const uint32_t = &raw mut (*ctx).state as *mut uint32_t;
    let mut output = &raw mut (*ctx).u.output32 as *mut uint32_t;
    let mut i: ::core::ffi::c_int = 0;
    let mut x: [uint32_t; 16] = [0; 16];
    x[0 as ::core::ffi::c_int as usize] = (*input.offset(0 as ::core::ffi::c_int as isize))
        .wrapping_add(*input.offset(4 as ::core::ffi::c_int as isize));
    x[12 as ::core::ffi::c_int as usize] = rol32(
        *input.offset(12 as ::core::ffi::c_int as isize) ^ x[0 as ::core::ffi::c_int as usize],
        16 as ::core::ffi::c_int,
    );
    x[8 as ::core::ffi::c_int as usize] = (*input.offset(8 as ::core::ffi::c_int as isize))
        .wrapping_add(x[12 as ::core::ffi::c_int as usize]);
    x[4 as ::core::ffi::c_int as usize] = rol32(
        *input.offset(4 as ::core::ffi::c_int as isize) ^ x[8 as ::core::ffi::c_int as usize],
        12 as ::core::ffi::c_int,
    );
    x[0 as ::core::ffi::c_int as usize] =
        x[0 as ::core::ffi::c_int as usize].wrapping_add(x[4 as ::core::ffi::c_int as usize]);
    x[12 as ::core::ffi::c_int as usize] = rol32(
        x[12 as ::core::ffi::c_int as usize] ^ x[0 as ::core::ffi::c_int as usize],
        8 as ::core::ffi::c_int,
    );
    x[8 as ::core::ffi::c_int as usize] =
        x[8 as ::core::ffi::c_int as usize].wrapping_add(x[12 as ::core::ffi::c_int as usize]);
    x[4 as ::core::ffi::c_int as usize] = rol32(
        x[4 as ::core::ffi::c_int as usize] ^ x[8 as ::core::ffi::c_int as usize],
        7 as ::core::ffi::c_int,
    );
    x[1 as ::core::ffi::c_int as usize] = (*input.offset(1 as ::core::ffi::c_int as isize))
        .wrapping_add(*input.offset(5 as ::core::ffi::c_int as isize));
    x[13 as ::core::ffi::c_int as usize] = rol32(
        *input.offset(13 as ::core::ffi::c_int as isize) ^ x[1 as ::core::ffi::c_int as usize],
        16 as ::core::ffi::c_int,
    );
    x[9 as ::core::ffi::c_int as usize] = (*input.offset(9 as ::core::ffi::c_int as isize))
        .wrapping_add(x[13 as ::core::ffi::c_int as usize]);
    x[5 as ::core::ffi::c_int as usize] = rol32(
        *input.offset(5 as ::core::ffi::c_int as isize) ^ x[9 as ::core::ffi::c_int as usize],
        12 as ::core::ffi::c_int,
    );
    x[1 as ::core::ffi::c_int as usize] =
        x[1 as ::core::ffi::c_int as usize].wrapping_add(x[5 as ::core::ffi::c_int as usize]);
    x[13 as ::core::ffi::c_int as usize] = rol32(
        x[13 as ::core::ffi::c_int as usize] ^ x[1 as ::core::ffi::c_int as usize],
        8 as ::core::ffi::c_int,
    );
    x[9 as ::core::ffi::c_int as usize] =
        x[9 as ::core::ffi::c_int as usize].wrapping_add(x[13 as ::core::ffi::c_int as usize]);
    x[5 as ::core::ffi::c_int as usize] = rol32(
        x[5 as ::core::ffi::c_int as usize] ^ x[9 as ::core::ffi::c_int as usize],
        7 as ::core::ffi::c_int,
    );
    x[2 as ::core::ffi::c_int as usize] = (*input.offset(2 as ::core::ffi::c_int as isize))
        .wrapping_add(*input.offset(6 as ::core::ffi::c_int as isize));
    x[14 as ::core::ffi::c_int as usize] = rol32(
        *input.offset(14 as ::core::ffi::c_int as isize) ^ x[2 as ::core::ffi::c_int as usize],
        16 as ::core::ffi::c_int,
    );
    x[10 as ::core::ffi::c_int as usize] = (*input.offset(10 as ::core::ffi::c_int as isize))
        .wrapping_add(x[14 as ::core::ffi::c_int as usize]);
    x[6 as ::core::ffi::c_int as usize] = rol32(
        *input.offset(6 as ::core::ffi::c_int as isize) ^ x[10 as ::core::ffi::c_int as usize],
        12 as ::core::ffi::c_int,
    );
    x[2 as ::core::ffi::c_int as usize] =
        x[2 as ::core::ffi::c_int as usize].wrapping_add(x[6 as ::core::ffi::c_int as usize]);
    x[14 as ::core::ffi::c_int as usize] = rol32(
        x[14 as ::core::ffi::c_int as usize] ^ x[2 as ::core::ffi::c_int as usize],
        8 as ::core::ffi::c_int,
    );
    x[10 as ::core::ffi::c_int as usize] =
        x[10 as ::core::ffi::c_int as usize].wrapping_add(x[14 as ::core::ffi::c_int as usize]);
    x[6 as ::core::ffi::c_int as usize] = rol32(
        x[6 as ::core::ffi::c_int as usize] ^ x[10 as ::core::ffi::c_int as usize],
        7 as ::core::ffi::c_int,
    );
    x[3 as ::core::ffi::c_int as usize] = (*input.offset(3 as ::core::ffi::c_int as isize))
        .wrapping_add(*input.offset(7 as ::core::ffi::c_int as isize));
    x[15 as ::core::ffi::c_int as usize] = rol32(
        *input.offset(15 as ::core::ffi::c_int as isize) ^ x[3 as ::core::ffi::c_int as usize],
        16 as ::core::ffi::c_int,
    );
    x[11 as ::core::ffi::c_int as usize] = (*input.offset(11 as ::core::ffi::c_int as isize))
        .wrapping_add(x[15 as ::core::ffi::c_int as usize]);
    x[7 as ::core::ffi::c_int as usize] = rol32(
        *input.offset(7 as ::core::ffi::c_int as isize) ^ x[11 as ::core::ffi::c_int as usize],
        12 as ::core::ffi::c_int,
    );
    x[3 as ::core::ffi::c_int as usize] =
        x[3 as ::core::ffi::c_int as usize].wrapping_add(x[7 as ::core::ffi::c_int as usize]);
    x[15 as ::core::ffi::c_int as usize] = rol32(
        x[15 as ::core::ffi::c_int as usize] ^ x[3 as ::core::ffi::c_int as usize],
        8 as ::core::ffi::c_int,
    );
    x[11 as ::core::ffi::c_int as usize] =
        x[11 as ::core::ffi::c_int as usize].wrapping_add(x[15 as ::core::ffi::c_int as usize]);
    x[7 as ::core::ffi::c_int as usize] = rol32(
        x[7 as ::core::ffi::c_int as usize] ^ x[11 as ::core::ffi::c_int as usize],
        7 as ::core::ffi::c_int,
    );
    i = 0 as ::core::ffi::c_int;
    while i < CHACHA_ROUNDS / 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
        x[0 as ::core::ffi::c_int as usize] =
            x[0 as ::core::ffi::c_int as usize].wrapping_add(x[5 as ::core::ffi::c_int as usize]);
        x[15 as ::core::ffi::c_int as usize] = rol32(
            x[15 as ::core::ffi::c_int as usize] ^ x[0 as ::core::ffi::c_int as usize],
            16 as ::core::ffi::c_int,
        );
        x[10 as ::core::ffi::c_int as usize] =
            x[10 as ::core::ffi::c_int as usize].wrapping_add(x[15 as ::core::ffi::c_int as usize]);
        x[5 as ::core::ffi::c_int as usize] = rol32(
            x[5 as ::core::ffi::c_int as usize] ^ x[10 as ::core::ffi::c_int as usize],
            12 as ::core::ffi::c_int,
        );
        x[0 as ::core::ffi::c_int as usize] =
            x[0 as ::core::ffi::c_int as usize].wrapping_add(x[5 as ::core::ffi::c_int as usize]);
        x[15 as ::core::ffi::c_int as usize] = rol32(
            x[15 as ::core::ffi::c_int as usize] ^ x[0 as ::core::ffi::c_int as usize],
            8 as ::core::ffi::c_int,
        );
        x[10 as ::core::ffi::c_int as usize] =
            x[10 as ::core::ffi::c_int as usize].wrapping_add(x[15 as ::core::ffi::c_int as usize]);
        x[5 as ::core::ffi::c_int as usize] = rol32(
            x[5 as ::core::ffi::c_int as usize] ^ x[10 as ::core::ffi::c_int as usize],
            7 as ::core::ffi::c_int,
        );
        x[1 as ::core::ffi::c_int as usize] =
            x[1 as ::core::ffi::c_int as usize].wrapping_add(x[6 as ::core::ffi::c_int as usize]);
        x[12 as ::core::ffi::c_int as usize] = rol32(
            x[12 as ::core::ffi::c_int as usize] ^ x[1 as ::core::ffi::c_int as usize],
            16 as ::core::ffi::c_int,
        );
        x[11 as ::core::ffi::c_int as usize] =
            x[11 as ::core::ffi::c_int as usize].wrapping_add(x[12 as ::core::ffi::c_int as usize]);
        x[6 as ::core::ffi::c_int as usize] = rol32(
            x[6 as ::core::ffi::c_int as usize] ^ x[11 as ::core::ffi::c_int as usize],
            12 as ::core::ffi::c_int,
        );
        x[1 as ::core::ffi::c_int as usize] =
            x[1 as ::core::ffi::c_int as usize].wrapping_add(x[6 as ::core::ffi::c_int as usize]);
        x[12 as ::core::ffi::c_int as usize] = rol32(
            x[12 as ::core::ffi::c_int as usize] ^ x[1 as ::core::ffi::c_int as usize],
            8 as ::core::ffi::c_int,
        );
        x[11 as ::core::ffi::c_int as usize] =
            x[11 as ::core::ffi::c_int as usize].wrapping_add(x[12 as ::core::ffi::c_int as usize]);
        x[6 as ::core::ffi::c_int as usize] = rol32(
            x[6 as ::core::ffi::c_int as usize] ^ x[11 as ::core::ffi::c_int as usize],
            7 as ::core::ffi::c_int,
        );
        x[2 as ::core::ffi::c_int as usize] =
            x[2 as ::core::ffi::c_int as usize].wrapping_add(x[7 as ::core::ffi::c_int as usize]);
        x[13 as ::core::ffi::c_int as usize] = rol32(
            x[13 as ::core::ffi::c_int as usize] ^ x[2 as ::core::ffi::c_int as usize],
            16 as ::core::ffi::c_int,
        );
        x[8 as ::core::ffi::c_int as usize] =
            x[8 as ::core::ffi::c_int as usize].wrapping_add(x[13 as ::core::ffi::c_int as usize]);
        x[7 as ::core::ffi::c_int as usize] = rol32(
            x[7 as ::core::ffi::c_int as usize] ^ x[8 as ::core::ffi::c_int as usize],
            12 as ::core::ffi::c_int,
        );
        x[2 as ::core::ffi::c_int as usize] =
            x[2 as ::core::ffi::c_int as usize].wrapping_add(x[7 as ::core::ffi::c_int as usize]);
        x[13 as ::core::ffi::c_int as usize] = rol32(
            x[13 as ::core::ffi::c_int as usize] ^ x[2 as ::core::ffi::c_int as usize],
            8 as ::core::ffi::c_int,
        );
        x[8 as ::core::ffi::c_int as usize] =
            x[8 as ::core::ffi::c_int as usize].wrapping_add(x[13 as ::core::ffi::c_int as usize]);
        x[7 as ::core::ffi::c_int as usize] = rol32(
            x[7 as ::core::ffi::c_int as usize] ^ x[8 as ::core::ffi::c_int as usize],
            7 as ::core::ffi::c_int,
        );
        x[3 as ::core::ffi::c_int as usize] =
            x[3 as ::core::ffi::c_int as usize].wrapping_add(x[4 as ::core::ffi::c_int as usize]);
        x[14 as ::core::ffi::c_int as usize] = rol32(
            x[14 as ::core::ffi::c_int as usize] ^ x[3 as ::core::ffi::c_int as usize],
            16 as ::core::ffi::c_int,
        );
        x[9 as ::core::ffi::c_int as usize] =
            x[9 as ::core::ffi::c_int as usize].wrapping_add(x[14 as ::core::ffi::c_int as usize]);
        x[4 as ::core::ffi::c_int as usize] = rol32(
            x[4 as ::core::ffi::c_int as usize] ^ x[9 as ::core::ffi::c_int as usize],
            12 as ::core::ffi::c_int,
        );
        x[3 as ::core::ffi::c_int as usize] =
            x[3 as ::core::ffi::c_int as usize].wrapping_add(x[4 as ::core::ffi::c_int as usize]);
        x[14 as ::core::ffi::c_int as usize] = rol32(
            x[14 as ::core::ffi::c_int as usize] ^ x[3 as ::core::ffi::c_int as usize],
            8 as ::core::ffi::c_int,
        );
        x[9 as ::core::ffi::c_int as usize] =
            x[9 as ::core::ffi::c_int as usize].wrapping_add(x[14 as ::core::ffi::c_int as usize]);
        x[4 as ::core::ffi::c_int as usize] = rol32(
            x[4 as ::core::ffi::c_int as usize] ^ x[9 as ::core::ffi::c_int as usize],
            7 as ::core::ffi::c_int,
        );
        x[0 as ::core::ffi::c_int as usize] =
            x[0 as ::core::ffi::c_int as usize].wrapping_add(x[4 as ::core::ffi::c_int as usize]);
        x[12 as ::core::ffi::c_int as usize] = rol32(
            x[12 as ::core::ffi::c_int as usize] ^ x[0 as ::core::ffi::c_int as usize],
            16 as ::core::ffi::c_int,
        );
        x[8 as ::core::ffi::c_int as usize] =
            x[8 as ::core::ffi::c_int as usize].wrapping_add(x[12 as ::core::ffi::c_int as usize]);
        x[4 as ::core::ffi::c_int as usize] = rol32(
            x[4 as ::core::ffi::c_int as usize] ^ x[8 as ::core::ffi::c_int as usize],
            12 as ::core::ffi::c_int,
        );
        x[0 as ::core::ffi::c_int as usize] =
            x[0 as ::core::ffi::c_int as usize].wrapping_add(x[4 as ::core::ffi::c_int as usize]);
        x[12 as ::core::ffi::c_int as usize] = rol32(
            x[12 as ::core::ffi::c_int as usize] ^ x[0 as ::core::ffi::c_int as usize],
            8 as ::core::ffi::c_int,
        );
        x[8 as ::core::ffi::c_int as usize] =
            x[8 as ::core::ffi::c_int as usize].wrapping_add(x[12 as ::core::ffi::c_int as usize]);
        x[4 as ::core::ffi::c_int as usize] = rol32(
            x[4 as ::core::ffi::c_int as usize] ^ x[8 as ::core::ffi::c_int as usize],
            7 as ::core::ffi::c_int,
        );
        x[1 as ::core::ffi::c_int as usize] =
            x[1 as ::core::ffi::c_int as usize].wrapping_add(x[5 as ::core::ffi::c_int as usize]);
        x[13 as ::core::ffi::c_int as usize] = rol32(
            x[13 as ::core::ffi::c_int as usize] ^ x[1 as ::core::ffi::c_int as usize],
            16 as ::core::ffi::c_int,
        );
        x[9 as ::core::ffi::c_int as usize] =
            x[9 as ::core::ffi::c_int as usize].wrapping_add(x[13 as ::core::ffi::c_int as usize]);
        x[5 as ::core::ffi::c_int as usize] = rol32(
            x[5 as ::core::ffi::c_int as usize] ^ x[9 as ::core::ffi::c_int as usize],
            12 as ::core::ffi::c_int,
        );
        x[1 as ::core::ffi::c_int as usize] =
            x[1 as ::core::ffi::c_int as usize].wrapping_add(x[5 as ::core::ffi::c_int as usize]);
        x[13 as ::core::ffi::c_int as usize] = rol32(
            x[13 as ::core::ffi::c_int as usize] ^ x[1 as ::core::ffi::c_int as usize],
            8 as ::core::ffi::c_int,
        );
        x[9 as ::core::ffi::c_int as usize] =
            x[9 as ::core::ffi::c_int as usize].wrapping_add(x[13 as ::core::ffi::c_int as usize]);
        x[5 as ::core::ffi::c_int as usize] = rol32(
            x[5 as ::core::ffi::c_int as usize] ^ x[9 as ::core::ffi::c_int as usize],
            7 as ::core::ffi::c_int,
        );
        x[2 as ::core::ffi::c_int as usize] =
            x[2 as ::core::ffi::c_int as usize].wrapping_add(x[6 as ::core::ffi::c_int as usize]);
        x[14 as ::core::ffi::c_int as usize] = rol32(
            x[14 as ::core::ffi::c_int as usize] ^ x[2 as ::core::ffi::c_int as usize],
            16 as ::core::ffi::c_int,
        );
        x[10 as ::core::ffi::c_int as usize] =
            x[10 as ::core::ffi::c_int as usize].wrapping_add(x[14 as ::core::ffi::c_int as usize]);
        x[6 as ::core::ffi::c_int as usize] = rol32(
            x[6 as ::core::ffi::c_int as usize] ^ x[10 as ::core::ffi::c_int as usize],
            12 as ::core::ffi::c_int,
        );
        x[2 as ::core::ffi::c_int as usize] =
            x[2 as ::core::ffi::c_int as usize].wrapping_add(x[6 as ::core::ffi::c_int as usize]);
        x[14 as ::core::ffi::c_int as usize] = rol32(
            x[14 as ::core::ffi::c_int as usize] ^ x[2 as ::core::ffi::c_int as usize],
            8 as ::core::ffi::c_int,
        );
        x[10 as ::core::ffi::c_int as usize] =
            x[10 as ::core::ffi::c_int as usize].wrapping_add(x[14 as ::core::ffi::c_int as usize]);
        x[6 as ::core::ffi::c_int as usize] = rol32(
            x[6 as ::core::ffi::c_int as usize] ^ x[10 as ::core::ffi::c_int as usize],
            7 as ::core::ffi::c_int,
        );
        x[3 as ::core::ffi::c_int as usize] =
            x[3 as ::core::ffi::c_int as usize].wrapping_add(x[7 as ::core::ffi::c_int as usize]);
        x[15 as ::core::ffi::c_int as usize] = rol32(
            x[15 as ::core::ffi::c_int as usize] ^ x[3 as ::core::ffi::c_int as usize],
            16 as ::core::ffi::c_int,
        );
        x[11 as ::core::ffi::c_int as usize] =
            x[11 as ::core::ffi::c_int as usize].wrapping_add(x[15 as ::core::ffi::c_int as usize]);
        x[7 as ::core::ffi::c_int as usize] = rol32(
            x[7 as ::core::ffi::c_int as usize] ^ x[11 as ::core::ffi::c_int as usize],
            12 as ::core::ffi::c_int,
        );
        x[3 as ::core::ffi::c_int as usize] =
            x[3 as ::core::ffi::c_int as usize].wrapping_add(x[7 as ::core::ffi::c_int as usize]);
        x[15 as ::core::ffi::c_int as usize] = rol32(
            x[15 as ::core::ffi::c_int as usize] ^ x[3 as ::core::ffi::c_int as usize],
            8 as ::core::ffi::c_int,
        );
        x[11 as ::core::ffi::c_int as usize] =
            x[11 as ::core::ffi::c_int as usize].wrapping_add(x[15 as ::core::ffi::c_int as usize]);
        x[7 as ::core::ffi::c_int as usize] = rol32(
            x[7 as ::core::ffi::c_int as usize] ^ x[11 as ::core::ffi::c_int as usize],
            7 as ::core::ffi::c_int,
        );
        i += 1;
    }
    x[0 as ::core::ffi::c_int as usize] =
        x[0 as ::core::ffi::c_int as usize].wrapping_add(x[5 as ::core::ffi::c_int as usize]);
    x[15 as ::core::ffi::c_int as usize] = rol32(
        x[15 as ::core::ffi::c_int as usize] ^ x[0 as ::core::ffi::c_int as usize],
        16 as ::core::ffi::c_int,
    );
    x[10 as ::core::ffi::c_int as usize] =
        x[10 as ::core::ffi::c_int as usize].wrapping_add(x[15 as ::core::ffi::c_int as usize]);
    x[5 as ::core::ffi::c_int as usize] = rol32(
        x[5 as ::core::ffi::c_int as usize] ^ x[10 as ::core::ffi::c_int as usize],
        12 as ::core::ffi::c_int,
    );
    x[0 as ::core::ffi::c_int as usize] =
        x[0 as ::core::ffi::c_int as usize].wrapping_add(x[5 as ::core::ffi::c_int as usize]);
    x[15 as ::core::ffi::c_int as usize] = rol32(
        x[15 as ::core::ffi::c_int as usize] ^ x[0 as ::core::ffi::c_int as usize],
        8 as ::core::ffi::c_int,
    );
    x[10 as ::core::ffi::c_int as usize] =
        x[10 as ::core::ffi::c_int as usize].wrapping_add(x[15 as ::core::ffi::c_int as usize]);
    x[5 as ::core::ffi::c_int as usize] = rol32(
        x[5 as ::core::ffi::c_int as usize] ^ x[10 as ::core::ffi::c_int as usize],
        7 as ::core::ffi::c_int,
    );
    *output.offset(0 as ::core::ffi::c_int as isize) = x[0 as ::core::ffi::c_int as usize]
        .wrapping_add(*input.offset(0 as ::core::ffi::c_int as isize));
    *output.offset(5 as ::core::ffi::c_int as isize) = x[5 as ::core::ffi::c_int as usize]
        .wrapping_add(*input.offset(5 as ::core::ffi::c_int as isize));
    *output.offset(10 as ::core::ffi::c_int as isize) = x[10 as ::core::ffi::c_int as usize]
        .wrapping_add(*input.offset(10 as ::core::ffi::c_int as isize));
    *output.offset(15 as ::core::ffi::c_int as isize) = x[15 as ::core::ffi::c_int as usize]
        .wrapping_add(*input.offset(15 as ::core::ffi::c_int as isize));
    x[1 as ::core::ffi::c_int as usize] =
        x[1 as ::core::ffi::c_int as usize].wrapping_add(x[6 as ::core::ffi::c_int as usize]);
    x[12 as ::core::ffi::c_int as usize] = rol32(
        x[12 as ::core::ffi::c_int as usize] ^ x[1 as ::core::ffi::c_int as usize],
        16 as ::core::ffi::c_int,
    );
    x[11 as ::core::ffi::c_int as usize] =
        x[11 as ::core::ffi::c_int as usize].wrapping_add(x[12 as ::core::ffi::c_int as usize]);
    x[6 as ::core::ffi::c_int as usize] = rol32(
        x[6 as ::core::ffi::c_int as usize] ^ x[11 as ::core::ffi::c_int as usize],
        12 as ::core::ffi::c_int,
    );
    x[1 as ::core::ffi::c_int as usize] =
        x[1 as ::core::ffi::c_int as usize].wrapping_add(x[6 as ::core::ffi::c_int as usize]);
    x[12 as ::core::ffi::c_int as usize] = rol32(
        x[12 as ::core::ffi::c_int as usize] ^ x[1 as ::core::ffi::c_int as usize],
        8 as ::core::ffi::c_int,
    );
    x[11 as ::core::ffi::c_int as usize] =
        x[11 as ::core::ffi::c_int as usize].wrapping_add(x[12 as ::core::ffi::c_int as usize]);
    x[6 as ::core::ffi::c_int as usize] = rol32(
        x[6 as ::core::ffi::c_int as usize] ^ x[11 as ::core::ffi::c_int as usize],
        7 as ::core::ffi::c_int,
    );
    *output.offset(1 as ::core::ffi::c_int as isize) = x[1 as ::core::ffi::c_int as usize]
        .wrapping_add(*input.offset(1 as ::core::ffi::c_int as isize));
    *output.offset(6 as ::core::ffi::c_int as isize) = x[6 as ::core::ffi::c_int as usize]
        .wrapping_add(*input.offset(6 as ::core::ffi::c_int as isize));
    *output.offset(11 as ::core::ffi::c_int as isize) = x[11 as ::core::ffi::c_int as usize]
        .wrapping_add(*input.offset(11 as ::core::ffi::c_int as isize));
    *output.offset(12 as ::core::ffi::c_int as isize) = x[12 as ::core::ffi::c_int as usize]
        .wrapping_add(*input.offset(12 as ::core::ffi::c_int as isize));
    x[2 as ::core::ffi::c_int as usize] =
        x[2 as ::core::ffi::c_int as usize].wrapping_add(x[7 as ::core::ffi::c_int as usize]);
    x[13 as ::core::ffi::c_int as usize] = rol32(
        x[13 as ::core::ffi::c_int as usize] ^ x[2 as ::core::ffi::c_int as usize],
        16 as ::core::ffi::c_int,
    );
    x[8 as ::core::ffi::c_int as usize] =
        x[8 as ::core::ffi::c_int as usize].wrapping_add(x[13 as ::core::ffi::c_int as usize]);
    x[7 as ::core::ffi::c_int as usize] = rol32(
        x[7 as ::core::ffi::c_int as usize] ^ x[8 as ::core::ffi::c_int as usize],
        12 as ::core::ffi::c_int,
    );
    x[2 as ::core::ffi::c_int as usize] =
        x[2 as ::core::ffi::c_int as usize].wrapping_add(x[7 as ::core::ffi::c_int as usize]);
    x[13 as ::core::ffi::c_int as usize] = rol32(
        x[13 as ::core::ffi::c_int as usize] ^ x[2 as ::core::ffi::c_int as usize],
        8 as ::core::ffi::c_int,
    );
    x[8 as ::core::ffi::c_int as usize] =
        x[8 as ::core::ffi::c_int as usize].wrapping_add(x[13 as ::core::ffi::c_int as usize]);
    x[7 as ::core::ffi::c_int as usize] = rol32(
        x[7 as ::core::ffi::c_int as usize] ^ x[8 as ::core::ffi::c_int as usize],
        7 as ::core::ffi::c_int,
    );
    *output.offset(2 as ::core::ffi::c_int as isize) = x[2 as ::core::ffi::c_int as usize]
        .wrapping_add(*input.offset(2 as ::core::ffi::c_int as isize));
    *output.offset(7 as ::core::ffi::c_int as isize) = x[7 as ::core::ffi::c_int as usize]
        .wrapping_add(*input.offset(7 as ::core::ffi::c_int as isize));
    *output.offset(8 as ::core::ffi::c_int as isize) = x[8 as ::core::ffi::c_int as usize]
        .wrapping_add(*input.offset(8 as ::core::ffi::c_int as isize));
    *output.offset(13 as ::core::ffi::c_int as isize) = x[13 as ::core::ffi::c_int as usize]
        .wrapping_add(*input.offset(13 as ::core::ffi::c_int as isize));
    x[3 as ::core::ffi::c_int as usize] =
        x[3 as ::core::ffi::c_int as usize].wrapping_add(x[4 as ::core::ffi::c_int as usize]);
    x[14 as ::core::ffi::c_int as usize] = rol32(
        x[14 as ::core::ffi::c_int as usize] ^ x[3 as ::core::ffi::c_int as usize],
        16 as ::core::ffi::c_int,
    );
    x[9 as ::core::ffi::c_int as usize] =
        x[9 as ::core::ffi::c_int as usize].wrapping_add(x[14 as ::core::ffi::c_int as usize]);
    x[4 as ::core::ffi::c_int as usize] = rol32(
        x[4 as ::core::ffi::c_int as usize] ^ x[9 as ::core::ffi::c_int as usize],
        12 as ::core::ffi::c_int,
    );
    x[3 as ::core::ffi::c_int as usize] =
        x[3 as ::core::ffi::c_int as usize].wrapping_add(x[4 as ::core::ffi::c_int as usize]);
    x[14 as ::core::ffi::c_int as usize] = rol32(
        x[14 as ::core::ffi::c_int as usize] ^ x[3 as ::core::ffi::c_int as usize],
        8 as ::core::ffi::c_int,
    );
    x[9 as ::core::ffi::c_int as usize] =
        x[9 as ::core::ffi::c_int as usize].wrapping_add(x[14 as ::core::ffi::c_int as usize]);
    x[4 as ::core::ffi::c_int as usize] = rol32(
        x[4 as ::core::ffi::c_int as usize] ^ x[9 as ::core::ffi::c_int as usize],
        7 as ::core::ffi::c_int,
    );
    *output.offset(3 as ::core::ffi::c_int as isize) = x[3 as ::core::ffi::c_int as usize]
        .wrapping_add(*input.offset(3 as ::core::ffi::c_int as isize));
    *output.offset(4 as ::core::ffi::c_int as isize) = x[4 as ::core::ffi::c_int as usize]
        .wrapping_add(*input.offset(4 as ::core::ffi::c_int as isize));
    *output.offset(9 as ::core::ffi::c_int as isize) = x[9 as ::core::ffi::c_int as usize]
        .wrapping_add(*input.offset(9 as ::core::ffi::c_int as isize));
    *output.offset(14 as ::core::ffi::c_int as isize) = x[14 as ::core::ffi::c_int as usize]
        .wrapping_add(*input.offset(14 as ::core::ffi::c_int as isize));
    (*ctx).pos = 0 as ::core::ffi::c_uint;
    (*ctx).state[12 as ::core::ffi::c_int as usize] =
        (*ctx).state[12 as ::core::ffi::c_int as usize].wrapping_add(1);
    if (*ctx).state[12 as ::core::ffi::c_int as usize] == 0 {
        (*ctx).state[13 as ::core::ffi::c_int as usize] =
            (*ctx).state[13 as ::core::ffi::c_int as usize].wrapping_add(1);
    }
}
#[no_mangle]
#[c2rust::src_loc = "87:1"]
pub unsafe extern "C" fn chacha_set_key_256(
    mut ctx: *mut ChaCha,
    mut key: *const ::core::ffi::c_void,
) {
    let mut i: ::core::ffi::c_uint = 0;
    memcpy(
        (&raw mut (*ctx).state as *mut uint32_t).offset(0 as ::core::ffi::c_int as isize)
            as *mut uint32_t as *mut ::core::ffi::c_void,
        b"expand 32-byte k\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        16 as size_t,
    );
    memcpy(
        (&raw mut (*ctx).state as *mut uint32_t).offset(4 as ::core::ffi::c_int as isize)
            as *mut uint32_t as *mut ::core::ffi::c_void,
        key,
        32 as size_t,
    );
    i = 0 as ::core::ffi::c_uint;
    while i < 12 as ::core::ffi::c_uint {
        (*ctx).state[i as usize] = (*ctx).state[i as usize];
        i = i.wrapping_add(1);
    }
    (*ctx).pos = CHACHA_BLOCK_SIZE as ::core::ffi::c_uint;
}
#[no_mangle]
#[c2rust::src_loc = "99:1"]
pub unsafe extern "C" fn chacha_set_key_128(
    mut ctx: *mut ChaCha,
    mut key: *const ::core::ffi::c_void,
) {
    let mut i: ::core::ffi::c_uint = 0;
    memcpy(
        (&raw mut (*ctx).state as *mut uint32_t).offset(0 as ::core::ffi::c_int as isize)
            as *mut uint32_t as *mut ::core::ffi::c_void,
        b"expand 16-byte k\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        16 as size_t,
    );
    memcpy(
        (&raw mut (*ctx).state as *mut uint32_t).offset(4 as ::core::ffi::c_int as isize)
            as *mut uint32_t as *mut ::core::ffi::c_void,
        key,
        16 as size_t,
    );
    memcpy(
        (&raw mut (*ctx).state as *mut uint32_t).offset(8 as ::core::ffi::c_int as isize)
            as *mut uint32_t as *mut ::core::ffi::c_void,
        key,
        16 as size_t,
    );
    i = 0 as ::core::ffi::c_uint;
    while i < 12 as ::core::ffi::c_uint {
        (*ctx).state[i as usize] = (*ctx).state[i as usize];
        i = i.wrapping_add(1);
    }
    (*ctx).pos = CHACHA_BLOCK_SIZE as ::core::ffi::c_uint;
}
#[no_mangle]
#[c2rust::src_loc = "112:1"]
pub unsafe extern "C" fn chacha_set_nonce(
    mut ctx: *mut ChaCha,
    mut counter_low: uint32_t,
    mut counter_high: uint32_t,
    mut iv: *const ::core::ffi::c_void,
) {
    let mut _iv = iv as *const uint8_t;
    (*ctx).state[12 as ::core::ffi::c_int as usize] = counter_low;
    (*ctx).state[13 as ::core::ffi::c_int as usize] = counter_high;
    if !_iv.is_null() {
        (*ctx).state[14 as ::core::ffi::c_int as usize] =
            usual_le32dec(_iv as *const ::core::ffi::c_void);
        (*ctx).state[15 as ::core::ffi::c_int as usize] = usual_le32dec(
            _iv.offset(4 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        );
    }
    (*ctx).pos = CHACHA_BLOCK_SIZE as ::core::ffi::c_uint;
}
#[no_mangle]
#[c2rust::src_loc = "126:1"]
pub unsafe extern "C" fn chacha_keystream(
    mut ctx: *mut ChaCha,
    mut stream: *mut ::core::ffi::c_void,
    mut bytes: size_t,
) {
    let mut n: ::core::ffi::c_uint = 0;
    let mut avail: ::core::ffi::c_uint = 0;
    let mut ks: *const uint8_t = &raw mut (*ctx).u.output8 as *mut uint8_t;
    let mut dst = stream as *mut uint8_t;
    while bytes > 0 as size_t {
        if (*ctx).pos >= CHACHA_BLOCK_SIZE as ::core::ffi::c_uint {
            chacha_mix(ctx);
        }
        avail = (CHACHA_BLOCK_SIZE as ::core::ffi::c_uint).wrapping_sub((*ctx).pos);
        n = (if bytes > avail as size_t {
            avail as size_t
        } else {
            bytes
        }) as ::core::ffi::c_uint;
        memcpy(
            dst as *mut ::core::ffi::c_void,
            ks.offset((*ctx).pos as isize) as *const ::core::ffi::c_void,
            n as size_t,
        );
        bytes = bytes.wrapping_sub(n as size_t);
        dst = dst.offset(n as isize);
        (*ctx).pos = (*ctx).pos.wrapping_add(n);
    }
}
#[no_mangle]
#[c2rust::src_loc = "147:1"]
pub unsafe extern "C" fn chacha_keystream_xor(
    mut ctx: *mut ChaCha,
    mut plain: *const ::core::ffi::c_void,
    mut encrypted: *mut ::core::ffi::c_void,
    mut bytes: size_t,
) {
    let mut i: ::core::ffi::c_uint = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut avail: ::core::ffi::c_uint = 0;
    let mut ks: *const uint8_t = &raw mut (*ctx).u.output8 as *mut uint8_t;
    let mut src = plain as *const uint8_t;
    let mut dst = encrypted as *mut uint8_t;
    while bytes > 0 as size_t {
        if (*ctx).pos >= CHACHA_BLOCK_SIZE as ::core::ffi::c_uint {
            chacha_mix(ctx);
        }
        avail = (CHACHA_BLOCK_SIZE as ::core::ffi::c_uint).wrapping_sub((*ctx).pos);
        n = (if bytes > avail as size_t {
            avail as size_t
        } else {
            bytes
        }) as ::core::ffi::c_uint;
        i = 0 as ::core::ffi::c_uint;
        while i < n {
            *dst.offset(i as isize) = (*src.offset(i as isize) as ::core::ffi::c_int
                ^ *ks.offset(i as isize) as ::core::ffi::c_int)
                as uint8_t;
            i = i.wrapping_add(1);
        }
        bytes = bytes.wrapping_sub(n as size_t);
        dst = dst.offset(n as isize);
        src = src.offset(n as isize);
        (*ctx).pos = (*ctx).pos.wrapping_add(n);
    }
}
