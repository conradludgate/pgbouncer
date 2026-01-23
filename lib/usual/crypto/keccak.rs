#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:39"]
pub mod _types_h {
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:39"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:39"]
pub mod _uint8_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint8_t = u8;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:39"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = u32;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint64_t.h:39"]
pub mod _uint64_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint64_t = u64;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/crypto/keccak.h:39"]
pub mod keccak_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:1"]
    pub struct KeccakContext {
        pub u: C2RustUnnamed,
        pub pos: uint32_t,
        pub rbytes: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:2"]
    pub union C2RustUnnamed {
        pub state64: [uint64_t; 25],
        pub state32: [uint32_t; 50],
    }
    use super::_uint32_t_h::uint32_t;
    use super::_uint64_t_h::uint64_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:40"]
pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "77:1"]
        pub fn memcpy(
            __dst: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "83:1"]
        pub fn memset(
            __b: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __len: size_t,
        ) -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/bits.h:40"]
pub mod bits_h {
    #[inline]
    #[c2rust::src_loc = "55:1"]
    pub unsafe extern "C" fn rol64(mut v: uint64_t, mut s: ::core::ffi::c_int) -> uint64_t {
        v << s | v >> (64 as ::core::ffi::c_int - s)
    }
    use super::_uint64_t_h::uint64_t;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/endian.h:41"]
pub mod endian_h {
    #[inline]
    #[c2rust::src_loc = "282:1"]
    pub unsafe extern "C" fn usual_le64dec(mut p: *const ::core::ffi::c_void) -> uint64_t {
        let mut tmp: uint64_t = 0;
        memcpy(
            &raw mut tmp as *mut ::core::ffi::c_void,
            p,
            ::core::mem::size_of::<uint64_t>() as size_t,
        );
        tmp
    }
    #[inline]
    #[c2rust::src_loc = "357:1"]
    pub unsafe extern "C" fn usual_le64enc(mut p: *mut ::core::ffi::c_void, mut x: uint64_t) {
        let mut tmp: uint64_t = x;
        memcpy(
            p,
            &raw mut tmp as *const ::core::ffi::c_void,
            ::core::mem::size_of::<uint64_t>() as size_t,
        );
    }
    use super::_size_t_h::size_t;
    use super::_string_h::memcpy;
    use super::_uint64_t_h::uint64_t;
}
pub use self::_size_t_h::size_t;
use self::_string_h::{memcpy, memset};
pub use self::_types_h::__darwin_size_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::bits_h::rol64;
pub use self::endian_h::{usual_le64dec, usual_le64enc};
pub use self::keccak_h::{C2RustUnnamed, KeccakContext};
#[c2rust::src_loc = "47:9"]
pub const KECCAK_ROUNDS: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
#[c2rust::src_loc = "84:1"]
static mut RoundConstants64: [uint64_t; 24] = [
    0x1 as ::core::ffi::c_ulonglong,
    0x8082 as ::core::ffi::c_ulonglong,
    0x800000000000808a as ::core::ffi::c_ulonglong,
    0x8000000080008000 as ::core::ffi::c_ulonglong,
    0x808b as ::core::ffi::c_ulonglong,
    0x80000001 as ::core::ffi::c_ulonglong,
    0x8000000080008081 as ::core::ffi::c_ulonglong,
    0x8000000000008009 as ::core::ffi::c_ulonglong,
    0x8a as ::core::ffi::c_ulonglong,
    0x88 as ::core::ffi::c_ulonglong,
    0x80008009 as ::core::ffi::c_ulonglong,
    0x8000000a as ::core::ffi::c_ulonglong,
    0x8000808b as ::core::ffi::c_ulonglong,
    0x800000000000008b as ::core::ffi::c_ulonglong,
    0x8000000000008089 as ::core::ffi::c_ulonglong,
    0x8000000000008003 as ::core::ffi::c_ulonglong,
    0x8000000000008002 as ::core::ffi::c_ulonglong,
    0x8000000000000080 as ::core::ffi::c_ulonglong,
    0x800a as ::core::ffi::c_ulonglong,
    0x800000008000000a as ::core::ffi::c_ulonglong,
    0x8000000080008081 as ::core::ffi::c_ulonglong,
    0x8000000000008080 as ::core::ffi::c_ulonglong,
    0x80000001 as ::core::ffi::c_ulonglong,
    0x8000000080008008 as ::core::ffi::c_ulonglong,
];
#[c2rust::src_loc = "162:1"]
unsafe extern "C" fn keccak_f(mut ctx: *mut KeccakContext) {
    let mut state = &raw mut (*ctx).u.state64 as *mut uint64_t;
    let mut Ba: uint64_t = 0;
    let mut Be: uint64_t = 0;
    let mut Bi: uint64_t = 0;
    let mut Bo: uint64_t = 0;
    let mut Bu: uint64_t = 0;
    let mut Ca: uint64_t = 0;
    let mut Ce: uint64_t = 0;
    let mut Ci: uint64_t = 0;
    let mut Co: uint64_t = 0;
    let mut Cu: uint64_t = 0;
    let mut Da: uint64_t = 0;
    let mut De: uint64_t = 0;
    let mut Di: uint64_t = 0;
    let mut Do: uint64_t = 0;
    let mut Du: uint64_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < KECCAK_ROUNDS {
        Ca = *state.offset(0 as ::core::ffi::c_int as isize)
            ^ *state.offset(5 as ::core::ffi::c_int as isize)
            ^ *state.offset(10 as ::core::ffi::c_int as isize)
            ^ *state.offset(15 as ::core::ffi::c_int as isize)
            ^ *state.offset(20 as ::core::ffi::c_int as isize);
        Ce = *state.offset(1 as ::core::ffi::c_int as isize)
            ^ *state.offset(6 as ::core::ffi::c_int as isize)
            ^ *state.offset(11 as ::core::ffi::c_int as isize)
            ^ *state.offset(16 as ::core::ffi::c_int as isize)
            ^ *state.offset(21 as ::core::ffi::c_int as isize);
        Ci = *state.offset(2 as ::core::ffi::c_int as isize)
            ^ *state.offset(7 as ::core::ffi::c_int as isize)
            ^ *state.offset(12 as ::core::ffi::c_int as isize)
            ^ *state.offset(17 as ::core::ffi::c_int as isize)
            ^ *state.offset(22 as ::core::ffi::c_int as isize);
        Co = *state.offset(3 as ::core::ffi::c_int as isize)
            ^ *state.offset(8 as ::core::ffi::c_int as isize)
            ^ *state.offset(13 as ::core::ffi::c_int as isize)
            ^ *state.offset(18 as ::core::ffi::c_int as isize)
            ^ *state.offset(23 as ::core::ffi::c_int as isize);
        Cu = *state.offset(4 as ::core::ffi::c_int as isize)
            ^ *state.offset(9 as ::core::ffi::c_int as isize)
            ^ *state.offset(14 as ::core::ffi::c_int as isize)
            ^ *state.offset(19 as ::core::ffi::c_int as isize)
            ^ *state.offset(24 as ::core::ffi::c_int as isize);
        Da = Cu ^ rol64(Ce, 1 as ::core::ffi::c_int);
        De = Ca ^ rol64(Ci, 1 as ::core::ffi::c_int);
        Di = Ce ^ rol64(Co, 1 as ::core::ffi::c_int);
        Do = Ci ^ rol64(Cu, 1 as ::core::ffi::c_int);
        Du = Co ^ rol64(Ca, 1 as ::core::ffi::c_int);
        Ba = *state.offset(0 as ::core::ffi::c_int as isize) ^ Da;
        Be = rol64(
            *state.offset(6 as ::core::ffi::c_int as isize) ^ De,
            44 as ::core::ffi::c_int,
        );
        Bi = rol64(
            *state.offset(12 as ::core::ffi::c_int as isize) ^ Di,
            43 as ::core::ffi::c_int,
        );
        Bo = rol64(
            *state.offset(18 as ::core::ffi::c_int as isize) ^ Do,
            21 as ::core::ffi::c_int,
        );
        Bu = rol64(
            *state.offset(24 as ::core::ffi::c_int as isize) ^ Du,
            14 as ::core::ffi::c_int,
        );
        *state.offset(0 as ::core::ffi::c_int as isize) = Ba ^ !Be & Bi;
        *state.offset(0 as ::core::ffi::c_int as isize) ^=
            RoundConstants64[(i + 0 as ::core::ffi::c_int) as usize];
        *state.offset(6 as ::core::ffi::c_int as isize) = Be ^ !Bi & Bo;
        *state.offset(12 as ::core::ffi::c_int as isize) = Bi ^ !Bo & Bu;
        *state.offset(18 as ::core::ffi::c_int as isize) = Bo ^ !Bu & Ba;
        *state.offset(24 as ::core::ffi::c_int as isize) = Bu ^ !Ba & Be;
        Bi = rol64(
            *state.offset(10 as ::core::ffi::c_int as isize) ^ Da,
            3 as ::core::ffi::c_int,
        );
        Bo = rol64(
            *state.offset(16 as ::core::ffi::c_int as isize) ^ De,
            45 as ::core::ffi::c_int,
        );
        Bu = rol64(
            *state.offset(22 as ::core::ffi::c_int as isize) ^ Di,
            61 as ::core::ffi::c_int,
        );
        Ba = rol64(
            *state.offset(3 as ::core::ffi::c_int as isize) ^ Do,
            28 as ::core::ffi::c_int,
        );
        Be = rol64(
            *state.offset(9 as ::core::ffi::c_int as isize) ^ Du,
            20 as ::core::ffi::c_int,
        );
        *state.offset(10 as ::core::ffi::c_int as isize) = Ba ^ !Be & Bi;
        *state.offset(16 as ::core::ffi::c_int as isize) = Be ^ !Bi & Bo;
        *state.offset(22 as ::core::ffi::c_int as isize) = Bi ^ !Bo & Bu;
        *state.offset(3 as ::core::ffi::c_int as isize) = Bo ^ !Bu & Ba;
        *state.offset(9 as ::core::ffi::c_int as isize) = Bu ^ !Ba & Be;
        Bu = rol64(
            *state.offset(20 as ::core::ffi::c_int as isize) ^ Da,
            18 as ::core::ffi::c_int,
        );
        Ba = rol64(
            *state.offset(1 as ::core::ffi::c_int as isize) ^ De,
            1 as ::core::ffi::c_int,
        );
        Be = rol64(
            *state.offset(7 as ::core::ffi::c_int as isize) ^ Di,
            6 as ::core::ffi::c_int,
        );
        Bi = rol64(
            *state.offset(13 as ::core::ffi::c_int as isize) ^ Do,
            25 as ::core::ffi::c_int,
        );
        Bo = rol64(
            *state.offset(19 as ::core::ffi::c_int as isize) ^ Du,
            8 as ::core::ffi::c_int,
        );
        *state.offset(20 as ::core::ffi::c_int as isize) = Ba ^ !Be & Bi;
        *state.offset(1 as ::core::ffi::c_int as isize) = Be ^ !Bi & Bo;
        *state.offset(7 as ::core::ffi::c_int as isize) = Bi ^ !Bo & Bu;
        *state.offset(13 as ::core::ffi::c_int as isize) = Bo ^ !Bu & Ba;
        *state.offset(19 as ::core::ffi::c_int as isize) = Bu ^ !Ba & Be;
        Be = rol64(
            *state.offset(5 as ::core::ffi::c_int as isize) ^ Da,
            36 as ::core::ffi::c_int,
        );
        Bi = rol64(
            *state.offset(11 as ::core::ffi::c_int as isize) ^ De,
            10 as ::core::ffi::c_int,
        );
        Bo = rol64(
            *state.offset(17 as ::core::ffi::c_int as isize) ^ Di,
            15 as ::core::ffi::c_int,
        );
        Bu = rol64(
            *state.offset(23 as ::core::ffi::c_int as isize) ^ Do,
            56 as ::core::ffi::c_int,
        );
        Ba = rol64(
            *state.offset(4 as ::core::ffi::c_int as isize) ^ Du,
            27 as ::core::ffi::c_int,
        );
        *state.offset(5 as ::core::ffi::c_int as isize) = Ba ^ !Be & Bi;
        *state.offset(11 as ::core::ffi::c_int as isize) = Be ^ !Bi & Bo;
        *state.offset(17 as ::core::ffi::c_int as isize) = Bi ^ !Bo & Bu;
        *state.offset(23 as ::core::ffi::c_int as isize) = Bo ^ !Bu & Ba;
        *state.offset(4 as ::core::ffi::c_int as isize) = Bu ^ !Ba & Be;
        Bo = rol64(
            *state.offset(15 as ::core::ffi::c_int as isize) ^ Da,
            41 as ::core::ffi::c_int,
        );
        Bu = rol64(
            *state.offset(21 as ::core::ffi::c_int as isize) ^ De,
            2 as ::core::ffi::c_int,
        );
        Ba = rol64(
            *state.offset(2 as ::core::ffi::c_int as isize) ^ Di,
            62 as ::core::ffi::c_int,
        );
        Be = rol64(
            *state.offset(8 as ::core::ffi::c_int as isize) ^ Do,
            55 as ::core::ffi::c_int,
        );
        Bi = rol64(
            *state.offset(14 as ::core::ffi::c_int as isize) ^ Du,
            39 as ::core::ffi::c_int,
        );
        *state.offset(15 as ::core::ffi::c_int as isize) = Ba ^ !Be & Bi;
        *state.offset(21 as ::core::ffi::c_int as isize) = Be ^ !Bi & Bo;
        *state.offset(2 as ::core::ffi::c_int as isize) = Bi ^ !Bo & Bu;
        *state.offset(8 as ::core::ffi::c_int as isize) = Bo ^ !Bu & Ba;
        *state.offset(14 as ::core::ffi::c_int as isize) = Bu ^ !Ba & Be;
        Ca = *state.offset(0 as ::core::ffi::c_int as isize)
            ^ *state.offset(10 as ::core::ffi::c_int as isize)
            ^ *state.offset(20 as ::core::ffi::c_int as isize)
            ^ *state.offset(5 as ::core::ffi::c_int as isize)
            ^ *state.offset(15 as ::core::ffi::c_int as isize);
        Ce = *state.offset(6 as ::core::ffi::c_int as isize)
            ^ *state.offset(16 as ::core::ffi::c_int as isize)
            ^ *state.offset(1 as ::core::ffi::c_int as isize)
            ^ *state.offset(11 as ::core::ffi::c_int as isize)
            ^ *state.offset(21 as ::core::ffi::c_int as isize);
        Ci = *state.offset(12 as ::core::ffi::c_int as isize)
            ^ *state.offset(22 as ::core::ffi::c_int as isize)
            ^ *state.offset(7 as ::core::ffi::c_int as isize)
            ^ *state.offset(17 as ::core::ffi::c_int as isize)
            ^ *state.offset(2 as ::core::ffi::c_int as isize);
        Co = *state.offset(18 as ::core::ffi::c_int as isize)
            ^ *state.offset(3 as ::core::ffi::c_int as isize)
            ^ *state.offset(13 as ::core::ffi::c_int as isize)
            ^ *state.offset(23 as ::core::ffi::c_int as isize)
            ^ *state.offset(8 as ::core::ffi::c_int as isize);
        Cu = *state.offset(24 as ::core::ffi::c_int as isize)
            ^ *state.offset(9 as ::core::ffi::c_int as isize)
            ^ *state.offset(19 as ::core::ffi::c_int as isize)
            ^ *state.offset(4 as ::core::ffi::c_int as isize)
            ^ *state.offset(14 as ::core::ffi::c_int as isize);
        Da = Cu ^ rol64(Ce, 1 as ::core::ffi::c_int);
        De = Ca ^ rol64(Ci, 1 as ::core::ffi::c_int);
        Di = Ce ^ rol64(Co, 1 as ::core::ffi::c_int);
        Do = Ci ^ rol64(Cu, 1 as ::core::ffi::c_int);
        Du = Co ^ rol64(Ca, 1 as ::core::ffi::c_int);
        Ba = *state.offset(0 as ::core::ffi::c_int as isize) ^ Da;
        Be = rol64(
            *state.offset(16 as ::core::ffi::c_int as isize) ^ De,
            44 as ::core::ffi::c_int,
        );
        Bi = rol64(
            *state.offset(7 as ::core::ffi::c_int as isize) ^ Di,
            43 as ::core::ffi::c_int,
        );
        Bo = rol64(
            *state.offset(23 as ::core::ffi::c_int as isize) ^ Do,
            21 as ::core::ffi::c_int,
        );
        Bu = rol64(
            *state.offset(14 as ::core::ffi::c_int as isize) ^ Du,
            14 as ::core::ffi::c_int,
        );
        *state.offset(0 as ::core::ffi::c_int as isize) = Ba ^ !Be & Bi;
        *state.offset(0 as ::core::ffi::c_int as isize) ^=
            RoundConstants64[(i + 1 as ::core::ffi::c_int) as usize];
        *state.offset(16 as ::core::ffi::c_int as isize) = Be ^ !Bi & Bo;
        *state.offset(7 as ::core::ffi::c_int as isize) = Bi ^ !Bo & Bu;
        *state.offset(23 as ::core::ffi::c_int as isize) = Bo ^ !Bu & Ba;
        *state.offset(14 as ::core::ffi::c_int as isize) = Bu ^ !Ba & Be;
        Bi = rol64(
            *state.offset(20 as ::core::ffi::c_int as isize) ^ Da,
            3 as ::core::ffi::c_int,
        );
        Bo = rol64(
            *state.offset(11 as ::core::ffi::c_int as isize) ^ De,
            45 as ::core::ffi::c_int,
        );
        Bu = rol64(
            *state.offset(2 as ::core::ffi::c_int as isize) ^ Di,
            61 as ::core::ffi::c_int,
        );
        Ba = rol64(
            *state.offset(18 as ::core::ffi::c_int as isize) ^ Do,
            28 as ::core::ffi::c_int,
        );
        Be = rol64(
            *state.offset(9 as ::core::ffi::c_int as isize) ^ Du,
            20 as ::core::ffi::c_int,
        );
        *state.offset(20 as ::core::ffi::c_int as isize) = Ba ^ !Be & Bi;
        *state.offset(11 as ::core::ffi::c_int as isize) = Be ^ !Bi & Bo;
        *state.offset(2 as ::core::ffi::c_int as isize) = Bi ^ !Bo & Bu;
        *state.offset(18 as ::core::ffi::c_int as isize) = Bo ^ !Bu & Ba;
        *state.offset(9 as ::core::ffi::c_int as isize) = Bu ^ !Ba & Be;
        Bu = rol64(
            *state.offset(15 as ::core::ffi::c_int as isize) ^ Da,
            18 as ::core::ffi::c_int,
        );
        Ba = rol64(
            *state.offset(6 as ::core::ffi::c_int as isize) ^ De,
            1 as ::core::ffi::c_int,
        );
        Be = rol64(
            *state.offset(22 as ::core::ffi::c_int as isize) ^ Di,
            6 as ::core::ffi::c_int,
        );
        Bi = rol64(
            *state.offset(13 as ::core::ffi::c_int as isize) ^ Do,
            25 as ::core::ffi::c_int,
        );
        Bo = rol64(
            *state.offset(4 as ::core::ffi::c_int as isize) ^ Du,
            8 as ::core::ffi::c_int,
        );
        *state.offset(15 as ::core::ffi::c_int as isize) = Ba ^ !Be & Bi;
        *state.offset(6 as ::core::ffi::c_int as isize) = Be ^ !Bi & Bo;
        *state.offset(22 as ::core::ffi::c_int as isize) = Bi ^ !Bo & Bu;
        *state.offset(13 as ::core::ffi::c_int as isize) = Bo ^ !Bu & Ba;
        *state.offset(4 as ::core::ffi::c_int as isize) = Bu ^ !Ba & Be;
        Be = rol64(
            *state.offset(10 as ::core::ffi::c_int as isize) ^ Da,
            36 as ::core::ffi::c_int,
        );
        Bi = rol64(
            *state.offset(1 as ::core::ffi::c_int as isize) ^ De,
            10 as ::core::ffi::c_int,
        );
        Bo = rol64(
            *state.offset(17 as ::core::ffi::c_int as isize) ^ Di,
            15 as ::core::ffi::c_int,
        );
        Bu = rol64(
            *state.offset(8 as ::core::ffi::c_int as isize) ^ Do,
            56 as ::core::ffi::c_int,
        );
        Ba = rol64(
            *state.offset(24 as ::core::ffi::c_int as isize) ^ Du,
            27 as ::core::ffi::c_int,
        );
        *state.offset(10 as ::core::ffi::c_int as isize) = Ba ^ !Be & Bi;
        *state.offset(1 as ::core::ffi::c_int as isize) = Be ^ !Bi & Bo;
        *state.offset(17 as ::core::ffi::c_int as isize) = Bi ^ !Bo & Bu;
        *state.offset(8 as ::core::ffi::c_int as isize) = Bo ^ !Bu & Ba;
        *state.offset(24 as ::core::ffi::c_int as isize) = Bu ^ !Ba & Be;
        Bo = rol64(
            *state.offset(5 as ::core::ffi::c_int as isize) ^ Da,
            41 as ::core::ffi::c_int,
        );
        Bu = rol64(
            *state.offset(21 as ::core::ffi::c_int as isize) ^ De,
            2 as ::core::ffi::c_int,
        );
        Ba = rol64(
            *state.offset(12 as ::core::ffi::c_int as isize) ^ Di,
            62 as ::core::ffi::c_int,
        );
        Be = rol64(
            *state.offset(3 as ::core::ffi::c_int as isize) ^ Do,
            55 as ::core::ffi::c_int,
        );
        Bi = rol64(
            *state.offset(19 as ::core::ffi::c_int as isize) ^ Du,
            39 as ::core::ffi::c_int,
        );
        *state.offset(5 as ::core::ffi::c_int as isize) = Ba ^ !Be & Bi;
        *state.offset(21 as ::core::ffi::c_int as isize) = Be ^ !Bi & Bo;
        *state.offset(12 as ::core::ffi::c_int as isize) = Bi ^ !Bo & Bu;
        *state.offset(3 as ::core::ffi::c_int as isize) = Bo ^ !Bu & Ba;
        *state.offset(19 as ::core::ffi::c_int as isize) = Bu ^ !Ba & Be;
        Ca = *state.offset(0 as ::core::ffi::c_int as isize)
            ^ *state.offset(20 as ::core::ffi::c_int as isize)
            ^ *state.offset(15 as ::core::ffi::c_int as isize)
            ^ *state.offset(10 as ::core::ffi::c_int as isize)
            ^ *state.offset(5 as ::core::ffi::c_int as isize);
        Ce = *state.offset(16 as ::core::ffi::c_int as isize)
            ^ *state.offset(11 as ::core::ffi::c_int as isize)
            ^ *state.offset(6 as ::core::ffi::c_int as isize)
            ^ *state.offset(1 as ::core::ffi::c_int as isize)
            ^ *state.offset(21 as ::core::ffi::c_int as isize);
        Ci = *state.offset(7 as ::core::ffi::c_int as isize)
            ^ *state.offset(2 as ::core::ffi::c_int as isize)
            ^ *state.offset(22 as ::core::ffi::c_int as isize)
            ^ *state.offset(17 as ::core::ffi::c_int as isize)
            ^ *state.offset(12 as ::core::ffi::c_int as isize);
        Co = *state.offset(23 as ::core::ffi::c_int as isize)
            ^ *state.offset(18 as ::core::ffi::c_int as isize)
            ^ *state.offset(13 as ::core::ffi::c_int as isize)
            ^ *state.offset(8 as ::core::ffi::c_int as isize)
            ^ *state.offset(3 as ::core::ffi::c_int as isize);
        Cu = *state.offset(14 as ::core::ffi::c_int as isize)
            ^ *state.offset(9 as ::core::ffi::c_int as isize)
            ^ *state.offset(4 as ::core::ffi::c_int as isize)
            ^ *state.offset(24 as ::core::ffi::c_int as isize)
            ^ *state.offset(19 as ::core::ffi::c_int as isize);
        Da = Cu ^ rol64(Ce, 1 as ::core::ffi::c_int);
        De = Ca ^ rol64(Ci, 1 as ::core::ffi::c_int);
        Di = Ce ^ rol64(Co, 1 as ::core::ffi::c_int);
        Do = Ci ^ rol64(Cu, 1 as ::core::ffi::c_int);
        Du = Co ^ rol64(Ca, 1 as ::core::ffi::c_int);
        Ba = *state.offset(0 as ::core::ffi::c_int as isize) ^ Da;
        Be = rol64(
            *state.offset(11 as ::core::ffi::c_int as isize) ^ De,
            44 as ::core::ffi::c_int,
        );
        Bi = rol64(
            *state.offset(22 as ::core::ffi::c_int as isize) ^ Di,
            43 as ::core::ffi::c_int,
        );
        Bo = rol64(
            *state.offset(8 as ::core::ffi::c_int as isize) ^ Do,
            21 as ::core::ffi::c_int,
        );
        Bu = rol64(
            *state.offset(19 as ::core::ffi::c_int as isize) ^ Du,
            14 as ::core::ffi::c_int,
        );
        *state.offset(0 as ::core::ffi::c_int as isize) = Ba ^ !Be & Bi;
        *state.offset(0 as ::core::ffi::c_int as isize) ^=
            RoundConstants64[(i + 2 as ::core::ffi::c_int) as usize];
        *state.offset(11 as ::core::ffi::c_int as isize) = Be ^ !Bi & Bo;
        *state.offset(22 as ::core::ffi::c_int as isize) = Bi ^ !Bo & Bu;
        *state.offset(8 as ::core::ffi::c_int as isize) = Bo ^ !Bu & Ba;
        *state.offset(19 as ::core::ffi::c_int as isize) = Bu ^ !Ba & Be;
        Bi = rol64(
            *state.offset(15 as ::core::ffi::c_int as isize) ^ Da,
            3 as ::core::ffi::c_int,
        );
        Bo = rol64(
            *state.offset(1 as ::core::ffi::c_int as isize) ^ De,
            45 as ::core::ffi::c_int,
        );
        Bu = rol64(
            *state.offset(12 as ::core::ffi::c_int as isize) ^ Di,
            61 as ::core::ffi::c_int,
        );
        Ba = rol64(
            *state.offset(23 as ::core::ffi::c_int as isize) ^ Do,
            28 as ::core::ffi::c_int,
        );
        Be = rol64(
            *state.offset(9 as ::core::ffi::c_int as isize) ^ Du,
            20 as ::core::ffi::c_int,
        );
        *state.offset(15 as ::core::ffi::c_int as isize) = Ba ^ !Be & Bi;
        *state.offset(1 as ::core::ffi::c_int as isize) = Be ^ !Bi & Bo;
        *state.offset(12 as ::core::ffi::c_int as isize) = Bi ^ !Bo & Bu;
        *state.offset(23 as ::core::ffi::c_int as isize) = Bo ^ !Bu & Ba;
        *state.offset(9 as ::core::ffi::c_int as isize) = Bu ^ !Ba & Be;
        Bu = rol64(
            *state.offset(5 as ::core::ffi::c_int as isize) ^ Da,
            18 as ::core::ffi::c_int,
        );
        Ba = rol64(
            *state.offset(16 as ::core::ffi::c_int as isize) ^ De,
            1 as ::core::ffi::c_int,
        );
        Be = rol64(
            *state.offset(2 as ::core::ffi::c_int as isize) ^ Di,
            6 as ::core::ffi::c_int,
        );
        Bi = rol64(
            *state.offset(13 as ::core::ffi::c_int as isize) ^ Do,
            25 as ::core::ffi::c_int,
        );
        Bo = rol64(
            *state.offset(24 as ::core::ffi::c_int as isize) ^ Du,
            8 as ::core::ffi::c_int,
        );
        *state.offset(5 as ::core::ffi::c_int as isize) = Ba ^ !Be & Bi;
        *state.offset(16 as ::core::ffi::c_int as isize) = Be ^ !Bi & Bo;
        *state.offset(2 as ::core::ffi::c_int as isize) = Bi ^ !Bo & Bu;
        *state.offset(13 as ::core::ffi::c_int as isize) = Bo ^ !Bu & Ba;
        *state.offset(24 as ::core::ffi::c_int as isize) = Bu ^ !Ba & Be;
        Be = rol64(
            *state.offset(20 as ::core::ffi::c_int as isize) ^ Da,
            36 as ::core::ffi::c_int,
        );
        Bi = rol64(
            *state.offset(6 as ::core::ffi::c_int as isize) ^ De,
            10 as ::core::ffi::c_int,
        );
        Bo = rol64(
            *state.offset(17 as ::core::ffi::c_int as isize) ^ Di,
            15 as ::core::ffi::c_int,
        );
        Bu = rol64(
            *state.offset(3 as ::core::ffi::c_int as isize) ^ Do,
            56 as ::core::ffi::c_int,
        );
        Ba = rol64(
            *state.offset(14 as ::core::ffi::c_int as isize) ^ Du,
            27 as ::core::ffi::c_int,
        );
        *state.offset(20 as ::core::ffi::c_int as isize) = Ba ^ !Be & Bi;
        *state.offset(6 as ::core::ffi::c_int as isize) = Be ^ !Bi & Bo;
        *state.offset(17 as ::core::ffi::c_int as isize) = Bi ^ !Bo & Bu;
        *state.offset(3 as ::core::ffi::c_int as isize) = Bo ^ !Bu & Ba;
        *state.offset(14 as ::core::ffi::c_int as isize) = Bu ^ !Ba & Be;
        Bo = rol64(
            *state.offset(10 as ::core::ffi::c_int as isize) ^ Da,
            41 as ::core::ffi::c_int,
        );
        Bu = rol64(
            *state.offset(21 as ::core::ffi::c_int as isize) ^ De,
            2 as ::core::ffi::c_int,
        );
        Ba = rol64(
            *state.offset(7 as ::core::ffi::c_int as isize) ^ Di,
            62 as ::core::ffi::c_int,
        );
        Be = rol64(
            *state.offset(18 as ::core::ffi::c_int as isize) ^ Do,
            55 as ::core::ffi::c_int,
        );
        Bi = rol64(
            *state.offset(4 as ::core::ffi::c_int as isize) ^ Du,
            39 as ::core::ffi::c_int,
        );
        *state.offset(10 as ::core::ffi::c_int as isize) = Ba ^ !Be & Bi;
        *state.offset(21 as ::core::ffi::c_int as isize) = Be ^ !Bi & Bo;
        *state.offset(7 as ::core::ffi::c_int as isize) = Bi ^ !Bo & Bu;
        *state.offset(18 as ::core::ffi::c_int as isize) = Bo ^ !Bu & Ba;
        *state.offset(4 as ::core::ffi::c_int as isize) = Bu ^ !Ba & Be;
        Ca = *state.offset(0 as ::core::ffi::c_int as isize)
            ^ *state.offset(15 as ::core::ffi::c_int as isize)
            ^ *state.offset(5 as ::core::ffi::c_int as isize)
            ^ *state.offset(20 as ::core::ffi::c_int as isize)
            ^ *state.offset(10 as ::core::ffi::c_int as isize);
        Ce = *state.offset(11 as ::core::ffi::c_int as isize)
            ^ *state.offset(1 as ::core::ffi::c_int as isize)
            ^ *state.offset(16 as ::core::ffi::c_int as isize)
            ^ *state.offset(6 as ::core::ffi::c_int as isize)
            ^ *state.offset(21 as ::core::ffi::c_int as isize);
        Ci = *state.offset(22 as ::core::ffi::c_int as isize)
            ^ *state.offset(12 as ::core::ffi::c_int as isize)
            ^ *state.offset(2 as ::core::ffi::c_int as isize)
            ^ *state.offset(17 as ::core::ffi::c_int as isize)
            ^ *state.offset(7 as ::core::ffi::c_int as isize);
        Co = *state.offset(8 as ::core::ffi::c_int as isize)
            ^ *state.offset(23 as ::core::ffi::c_int as isize)
            ^ *state.offset(13 as ::core::ffi::c_int as isize)
            ^ *state.offset(3 as ::core::ffi::c_int as isize)
            ^ *state.offset(18 as ::core::ffi::c_int as isize);
        Cu = *state.offset(19 as ::core::ffi::c_int as isize)
            ^ *state.offset(9 as ::core::ffi::c_int as isize)
            ^ *state.offset(24 as ::core::ffi::c_int as isize)
            ^ *state.offset(14 as ::core::ffi::c_int as isize)
            ^ *state.offset(4 as ::core::ffi::c_int as isize);
        Da = Cu ^ rol64(Ce, 1 as ::core::ffi::c_int);
        De = Ca ^ rol64(Ci, 1 as ::core::ffi::c_int);
        Di = Ce ^ rol64(Co, 1 as ::core::ffi::c_int);
        Do = Ci ^ rol64(Cu, 1 as ::core::ffi::c_int);
        Du = Co ^ rol64(Ca, 1 as ::core::ffi::c_int);
        Ba = *state.offset(0 as ::core::ffi::c_int as isize) ^ Da;
        Be = rol64(
            *state.offset(1 as ::core::ffi::c_int as isize) ^ De,
            44 as ::core::ffi::c_int,
        );
        Bi = rol64(
            *state.offset(2 as ::core::ffi::c_int as isize) ^ Di,
            43 as ::core::ffi::c_int,
        );
        Bo = rol64(
            *state.offset(3 as ::core::ffi::c_int as isize) ^ Do,
            21 as ::core::ffi::c_int,
        );
        Bu = rol64(
            *state.offset(4 as ::core::ffi::c_int as isize) ^ Du,
            14 as ::core::ffi::c_int,
        );
        *state.offset(0 as ::core::ffi::c_int as isize) = Ba ^ !Be & Bi;
        *state.offset(0 as ::core::ffi::c_int as isize) ^=
            RoundConstants64[(i + 3 as ::core::ffi::c_int) as usize];
        *state.offset(1 as ::core::ffi::c_int as isize) = Be ^ !Bi & Bo;
        *state.offset(2 as ::core::ffi::c_int as isize) = Bi ^ !Bo & Bu;
        *state.offset(3 as ::core::ffi::c_int as isize) = Bo ^ !Bu & Ba;
        *state.offset(4 as ::core::ffi::c_int as isize) = Bu ^ !Ba & Be;
        Bi = rol64(
            *state.offset(5 as ::core::ffi::c_int as isize) ^ Da,
            3 as ::core::ffi::c_int,
        );
        Bo = rol64(
            *state.offset(6 as ::core::ffi::c_int as isize) ^ De,
            45 as ::core::ffi::c_int,
        );
        Bu = rol64(
            *state.offset(7 as ::core::ffi::c_int as isize) ^ Di,
            61 as ::core::ffi::c_int,
        );
        Ba = rol64(
            *state.offset(8 as ::core::ffi::c_int as isize) ^ Do,
            28 as ::core::ffi::c_int,
        );
        Be = rol64(
            *state.offset(9 as ::core::ffi::c_int as isize) ^ Du,
            20 as ::core::ffi::c_int,
        );
        *state.offset(5 as ::core::ffi::c_int as isize) = Ba ^ !Be & Bi;
        *state.offset(6 as ::core::ffi::c_int as isize) = Be ^ !Bi & Bo;
        *state.offset(7 as ::core::ffi::c_int as isize) = Bi ^ !Bo & Bu;
        *state.offset(8 as ::core::ffi::c_int as isize) = Bo ^ !Bu & Ba;
        *state.offset(9 as ::core::ffi::c_int as isize) = Bu ^ !Ba & Be;
        Bu = rol64(
            *state.offset(10 as ::core::ffi::c_int as isize) ^ Da,
            18 as ::core::ffi::c_int,
        );
        Ba = rol64(
            *state.offset(11 as ::core::ffi::c_int as isize) ^ De,
            1 as ::core::ffi::c_int,
        );
        Be = rol64(
            *state.offset(12 as ::core::ffi::c_int as isize) ^ Di,
            6 as ::core::ffi::c_int,
        );
        Bi = rol64(
            *state.offset(13 as ::core::ffi::c_int as isize) ^ Do,
            25 as ::core::ffi::c_int,
        );
        Bo = rol64(
            *state.offset(14 as ::core::ffi::c_int as isize) ^ Du,
            8 as ::core::ffi::c_int,
        );
        *state.offset(10 as ::core::ffi::c_int as isize) = Ba ^ !Be & Bi;
        *state.offset(11 as ::core::ffi::c_int as isize) = Be ^ !Bi & Bo;
        *state.offset(12 as ::core::ffi::c_int as isize) = Bi ^ !Bo & Bu;
        *state.offset(13 as ::core::ffi::c_int as isize) = Bo ^ !Bu & Ba;
        *state.offset(14 as ::core::ffi::c_int as isize) = Bu ^ !Ba & Be;
        Be = rol64(
            *state.offset(15 as ::core::ffi::c_int as isize) ^ Da,
            36 as ::core::ffi::c_int,
        );
        Bi = rol64(
            *state.offset(16 as ::core::ffi::c_int as isize) ^ De,
            10 as ::core::ffi::c_int,
        );
        Bo = rol64(
            *state.offset(17 as ::core::ffi::c_int as isize) ^ Di,
            15 as ::core::ffi::c_int,
        );
        Bu = rol64(
            *state.offset(18 as ::core::ffi::c_int as isize) ^ Do,
            56 as ::core::ffi::c_int,
        );
        Ba = rol64(
            *state.offset(19 as ::core::ffi::c_int as isize) ^ Du,
            27 as ::core::ffi::c_int,
        );
        *state.offset(15 as ::core::ffi::c_int as isize) = Ba ^ !Be & Bi;
        *state.offset(16 as ::core::ffi::c_int as isize) = Be ^ !Bi & Bo;
        *state.offset(17 as ::core::ffi::c_int as isize) = Bi ^ !Bo & Bu;
        *state.offset(18 as ::core::ffi::c_int as isize) = Bo ^ !Bu & Ba;
        *state.offset(19 as ::core::ffi::c_int as isize) = Bu ^ !Ba & Be;
        Bo = rol64(
            *state.offset(20 as ::core::ffi::c_int as isize) ^ Da,
            41 as ::core::ffi::c_int,
        );
        Bu = rol64(
            *state.offset(21 as ::core::ffi::c_int as isize) ^ De,
            2 as ::core::ffi::c_int,
        );
        Ba = rol64(
            *state.offset(22 as ::core::ffi::c_int as isize) ^ Di,
            62 as ::core::ffi::c_int,
        );
        Be = rol64(
            *state.offset(23 as ::core::ffi::c_int as isize) ^ Do,
            55 as ::core::ffi::c_int,
        );
        Bi = rol64(
            *state.offset(24 as ::core::ffi::c_int as isize) ^ Du,
            39 as ::core::ffi::c_int,
        );
        *state.offset(20 as ::core::ffi::c_int as isize) = Ba ^ !Be & Bi;
        *state.offset(21 as ::core::ffi::c_int as isize) = Be ^ !Bi & Bo;
        *state.offset(22 as ::core::ffi::c_int as isize) = Bi ^ !Bo & Bu;
        *state.offset(23 as ::core::ffi::c_int as isize) = Bo ^ !Bu & Ba;
        *state.offset(24 as ::core::ffi::c_int as isize) = Bu ^ !Ba & Be;
        i += 4 as ::core::ffi::c_int;
    }
}
#[inline]
#[c2rust::src_loc = "470:1"]
unsafe extern "C" fn xor_lane(
    mut ctx: *mut KeccakContext,
    mut lane: ::core::ffi::c_int,
    mut val: uint64_t,
) {
    (*ctx).u.state64[lane as usize] ^= val;
}
#[c2rust::src_loc = "475:1"]
unsafe extern "C" fn extract(
    mut dst: *mut uint8_t,
    mut ctx: *const KeccakContext,
    mut startLane: ::core::ffi::c_int,
    mut laneCount: ::core::ffi::c_int,
) {
    let mut src = (&raw const (*ctx).u.state64 as *const uint64_t).offset(startLane as isize);
    loop {
        let fresh6 = laneCount;
        laneCount -= 1;
        if fresh6 == 0 {
            break;
        }
        let fresh7 = src;
        src = src.offset(1);
        usual_le64enc(dst as *mut ::core::ffi::c_void, *fresh7);
        dst = dst.offset(8 as ::core::ffi::c_int as isize);
    }
}
#[c2rust::src_loc = "1190:1"]
unsafe extern "C" fn xor_byte(
    mut ctx: *mut KeccakContext,
    mut nbyte: ::core::ffi::c_int,
    mut val: uint8_t,
) {
    let mut o = nbyte / 8 as ::core::ffi::c_int;
    let mut s = nbyte % 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int;
    xor_lane(ctx, o, (val as uint64_t) << s);
}
#[c2rust::src_loc = "1198:1"]
unsafe extern "C" fn add_bytes(
    mut ctx: *mut KeccakContext,
    mut p: *const uint8_t,
    mut ofs: ::core::ffi::c_uint,
    mut len: ::core::ffi::c_uint,
) {
    let mut w: uint64_t = 0;
    let mut m = ofs.wrapping_rem(8 as ::core::ffi::c_uint);
    if m != 0 {
        m = (8 as ::core::ffi::c_uint).wrapping_sub(m);
        if m > len {
            m = len;
        }
        loop {
            let fresh0 = m;
            m = m.wrapping_sub(1);
            if fresh0 == 0 {
                break;
            }
            let fresh1 = ofs;
            ofs = ofs.wrapping_add(1);
            let fresh2 = p;
            p = p.offset(1);
            xor_byte(ctx, fresh1 as ::core::ffi::c_int, *fresh2);
            len = len.wrapping_sub(1);
        }
    }
    while len >= 8 as ::core::ffi::c_uint {
        w = usual_le64dec(p as *const ::core::ffi::c_void);
        xor_lane(
            ctx,
            ofs.wrapping_div(8 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            w,
        );
        ofs = ofs.wrapping_add(8 as ::core::ffi::c_uint);
        p = p.offset(8 as ::core::ffi::c_int as isize);
        len = len.wrapping_sub(8 as ::core::ffi::c_uint);
    }
    loop {
        let fresh3 = len;
        len = len.wrapping_sub(1);
        if fresh3 == 0 {
            break;
        }
        let fresh4 = ofs;
        ofs = ofs.wrapping_add(1);
        let fresh5 = p;
        p = p.offset(1);
        xor_byte(ctx, fresh4 as ::core::ffi::c_int, *fresh5);
    }
}
#[c2rust::src_loc = "1228:1"]
unsafe extern "C" fn extract_bytes(
    mut ctx: *mut KeccakContext,
    mut dst: *mut uint8_t,
    mut ofs: ::core::ffi::c_uint,
    mut count: ::core::ffi::c_uint,
) {
    let mut lanebuf: [uint8_t; 8] = [0; 8];
    let mut n: ::core::ffi::c_uint = 0;
    let mut avail: ::core::ffi::c_uint = 0;
    if ofs.wrapping_rem(8 as ::core::ffi::c_uint) != 0 as ::core::ffi::c_uint
        || count < 8 as ::core::ffi::c_uint
    {
        avail = (8 as ::core::ffi::c_uint).wrapping_sub(ofs.wrapping_rem(8 as ::core::ffi::c_uint));
        n = if avail > count { count } else { avail };
        extract(
            &raw mut lanebuf as *mut uint8_t,
            ctx,
            ofs.wrapping_div(8 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        memcpy(
            dst as *mut ::core::ffi::c_void,
            (&raw mut lanebuf as *mut uint8_t)
                .offset(ofs.wrapping_rem(8 as ::core::ffi::c_uint) as isize)
                as *const ::core::ffi::c_void,
            n as size_t,
        );
        dst = dst.offset(n as isize);
        ofs = ofs.wrapping_add(n);
        count = count.wrapping_sub(n);
    }
    if count > 8 as ::core::ffi::c_uint {
        n = count.wrapping_div(8 as ::core::ffi::c_uint);
        extract(
            dst,
            ctx,
            ofs.wrapping_div(8 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            n as ::core::ffi::c_int,
        );
        dst = dst.offset(n.wrapping_mul(8 as ::core::ffi::c_uint) as isize);
        ofs = ofs.wrapping_add(n.wrapping_mul(8 as ::core::ffi::c_uint));
        count = count.wrapping_sub(n.wrapping_mul(8 as ::core::ffi::c_uint));
    }
    if count > 0 as ::core::ffi::c_uint {
        extract(
            &raw mut lanebuf as *mut uint8_t,
            ctx,
            ofs.wrapping_div(8 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        memcpy(
            dst as *mut ::core::ffi::c_void,
            &raw mut lanebuf as *mut uint8_t as *const ::core::ffi::c_void,
            count as size_t,
        );
    }
    memset(
        &raw mut lanebuf as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 8]>() as size_t,
    );
}
#[inline]
#[c2rust::src_loc = "1259:1"]
unsafe extern "C" fn permute_if_needed(mut ctx: *mut KeccakContext) {
    if (*ctx).pos == (*ctx).rbytes {
        keccak_f(ctx);
        (*ctx).pos = 0 as uint32_t;
    }
}
#[no_mangle]
#[c2rust::src_loc = "1271:1"]
pub unsafe extern "C" fn keccak_init(
    mut ctx: *mut KeccakContext,
    mut capacity: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if capacity.wrapping_rem(8 as ::core::ffi::c_uint) != 0 as ::core::ffi::c_uint
        || capacity < 8 as ::core::ffi::c_uint
        || capacity > (1600 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    memset(
        ctx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<KeccakContext>() as size_t,
    );
    (*ctx).rbytes = (1600 as ::core::ffi::c_uint)
        .wrapping_sub(capacity)
        .wrapping_div(8 as ::core::ffi::c_uint) as uint32_t;
    1 as ::core::ffi::c_int
}
#[no_mangle]
#[c2rust::src_loc = "1280:1"]
pub unsafe extern "C" fn keccak_absorb(
    mut ctx: *mut KeccakContext,
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut n: ::core::ffi::c_uint = 0;
    let mut avail: ::core::ffi::c_uint = 0;
    let mut src = data as *const uint8_t;
    while len > 0 as size_t {
        avail = (*ctx).rbytes.wrapping_sub((*ctx).pos) as ::core::ffi::c_uint;
        n = (if len > avail as size_t {
            avail as size_t
        } else {
            len
        }) as ::core::ffi::c_uint;
        add_bytes(ctx, src, (*ctx).pos as ::core::ffi::c_uint, n);
        src = src.offset(n as isize);
        len = len.wrapping_sub(n as size_t);
        (*ctx).pos = ((*ctx).pos as ::core::ffi::c_uint).wrapping_add(n) as uint32_t as uint32_t;
        permute_if_needed(ctx);
    }
}
#[no_mangle]
#[c2rust::src_loc = "1299:1"]
pub unsafe extern "C" fn keccak_squeeze(
    mut ctx: *mut KeccakContext,
    mut dst: *mut uint8_t,
    mut len: size_t,
) {
    let mut avail: ::core::ffi::c_uint = 0;
    let mut n: ::core::ffi::c_uint = 0;
    while len > 0 as size_t {
        avail = (*ctx).rbytes.wrapping_sub((*ctx).pos) as ::core::ffi::c_uint;
        n = (if len > avail as size_t {
            avail as size_t
        } else {
            len
        }) as ::core::ffi::c_uint;
        extract_bytes(ctx, dst, (*ctx).pos as ::core::ffi::c_uint, n);
        (*ctx).pos = ((*ctx).pos as ::core::ffi::c_uint).wrapping_add(n) as uint32_t as uint32_t;
        dst = dst.offset(n as isize);
        len = len.wrapping_sub(n as size_t);
        permute_if_needed(ctx);
    }
}
#[no_mangle]
#[c2rust::src_loc = "1317:1"]
pub unsafe extern "C" fn keccak_squeeze_xor(
    mut ctx: *mut KeccakContext,
    mut dst: *mut uint8_t,
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut src = data as *const uint8_t;
    let mut n: ::core::ffi::c_uint = 0;
    let mut avail: ::core::ffi::c_uint = 0;
    let mut i: ::core::ffi::c_uint = 0;
    while len > 0 as size_t {
        avail = (*ctx).rbytes.wrapping_sub((*ctx).pos) as ::core::ffi::c_uint;
        n = (if len > avail as size_t {
            avail as size_t
        } else {
            len
        }) as ::core::ffi::c_uint;
        extract_bytes(ctx, dst, (*ctx).pos as ::core::ffi::c_uint, n);
        i = 0 as ::core::ffi::c_uint;
        while i < n {
            let fresh8 = &mut *dst.offset(i as isize);
            *fresh8 = (*fresh8 as ::core::ffi::c_int
                ^ *src.offset(i as isize) as ::core::ffi::c_int) as uint8_t;
            i = i.wrapping_add(1);
        }
        (*ctx).pos = ((*ctx).pos as ::core::ffi::c_uint).wrapping_add(n) as uint32_t as uint32_t;
        src = src.offset(n as isize);
        dst = dst.offset(n as isize);
        len = len.wrapping_sub(n as size_t);
        permute_if_needed(ctx);
    }
}
#[no_mangle]
#[c2rust::src_loc = "1339:1"]
pub unsafe extern "C" fn keccak_encrypt(
    mut ctx: *mut KeccakContext,
    mut dst: *mut uint8_t,
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut src = data as *const uint8_t;
    let mut n: ::core::ffi::c_uint = 0;
    let mut avail: ::core::ffi::c_uint = 0;
    while len > 0 as size_t {
        avail = (*ctx).rbytes.wrapping_sub((*ctx).pos) as ::core::ffi::c_uint;
        n = (if len > avail as size_t {
            avail as size_t
        } else {
            len
        }) as ::core::ffi::c_uint;
        add_bytes(ctx, src, (*ctx).pos as ::core::ffi::c_uint, n);
        extract_bytes(ctx, dst, (*ctx).pos as ::core::ffi::c_uint, n);
        (*ctx).pos = ((*ctx).pos as ::core::ffi::c_uint).wrapping_add(n) as uint32_t as uint32_t;
        src = src.offset(n as isize);
        dst = dst.offset(n as isize);
        len = len.wrapping_sub(n as size_t);
        permute_if_needed(ctx);
    }
}
#[no_mangle]
#[c2rust::src_loc = "1360:1"]
pub unsafe extern "C" fn keccak_decrypt(
    mut ctx: *mut KeccakContext,
    mut dst: *mut uint8_t,
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut src = data as *const uint8_t;
    let mut n: ::core::ffi::c_uint = 0;
    let mut avail: ::core::ffi::c_uint = 0;
    let mut i: ::core::ffi::c_uint = 0;
    while len > 0 as size_t {
        avail = (*ctx).rbytes.wrapping_sub((*ctx).pos) as ::core::ffi::c_uint;
        n = (if len > avail as size_t {
            avail as size_t
        } else {
            len
        }) as ::core::ffi::c_uint;
        extract_bytes(ctx, dst, (*ctx).pos as ::core::ffi::c_uint, n);
        i = 0 as ::core::ffi::c_uint;
        while i < n {
            let fresh9 = &mut *dst.offset(i as isize);
            *fresh9 = (*fresh9 as ::core::ffi::c_int
                ^ *src.offset(i as isize) as ::core::ffi::c_int) as uint8_t;
            i = i.wrapping_add(1);
        }
        add_bytes(ctx, dst, (*ctx).pos as ::core::ffi::c_uint, n);
        (*ctx).pos = ((*ctx).pos as ::core::ffi::c_uint).wrapping_add(n) as uint32_t as uint32_t;
        src = src.offset(n as isize);
        dst = dst.offset(n as isize);
        len = len.wrapping_sub(n as size_t);
        permute_if_needed(ctx);
    }
}
#[no_mangle]
#[c2rust::src_loc = "1383:1"]
pub unsafe extern "C" fn keccak_pad(
    mut ctx: *mut KeccakContext,
    mut pad: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut src = pad as *const uint8_t;
    if len > 0 as size_t {
        if len > 1 as size_t {
            keccak_absorb(
                ctx,
                src as *const ::core::ffi::c_void,
                len.wrapping_sub(1 as size_t),
            );
            src = src.add(len.wrapping_sub(1 as size_t));
        }
        xor_byte(
            ctx,
            (*ctx).pos as ::core::ffi::c_int,
            *src.offset(0 as ::core::ffi::c_int as isize),
        );
        xor_byte(
            ctx,
            (*ctx).rbytes.wrapping_sub(1 as uint32_t) as ::core::ffi::c_int,
            0x80 as uint8_t,
        );
    }
    keccak_f(ctx);
    (*ctx).pos = 0 as uint32_t;
}
#[no_mangle]
#[c2rust::src_loc = "1399:1"]
pub unsafe extern "C" fn keccak_rewind(mut ctx: *mut KeccakContext) {
    (*ctx).pos = 0 as uint32_t;
}
#[no_mangle]
#[c2rust::src_loc = "1404:1"]
pub unsafe extern "C" fn keccak_forget(mut ctx: *mut KeccakContext) {
    let mut rem = ((*ctx).rbytes as ::core::ffi::c_uint).wrapping_rem(8 as ::core::ffi::c_uint);
    let mut buf: [uint8_t; 8] = [0; 8];
    memset(
        &raw mut (*ctx).u.state32 as *mut uint32_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (*ctx).rbytes.wrapping_sub(rem as uint32_t) as size_t,
    );
    if rem != 0 {
        extract_bytes(
            ctx,
            &raw mut buf as *mut uint8_t,
            ((*ctx).rbytes as ::core::ffi::c_uint).wrapping_sub(rem),
            rem,
        );
        add_bytes(
            ctx,
            &raw mut buf as *mut uint8_t,
            ((*ctx).rbytes as ::core::ffi::c_uint).wrapping_sub(rem),
            rem,
        );
        memset(
            &raw mut buf as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[uint8_t; 8]>() as size_t,
        );
    }
    (*ctx).pos = 0 as uint32_t;
}
