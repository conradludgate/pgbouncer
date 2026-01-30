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

pub mod md5_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct md5_ctx {
        pub nbytes: uint64_t,
        pub a: uint32_t,
        pub b: uint32_t,
        pub c: uint32_t,
        pub d: uint32_t,
        pub buf: [uint32_t; 16],
    }

    pub const MD5_BLOCK_LENGTH: ::core::ffi::c_int = 64 as ::core::ffi::c_int;

    pub const MD5_DIGEST_LENGTH: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    use super::_uint32_t_h::uint32_t;
    use super::_uint64_t_h::uint64_t;
}

pub mod digest_h {

    pub type DigestInitFunc = unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ();

    pub type DigestUpdateFunc = unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_void,
        ::core::ffi::c_uint,
    ) -> ();

    pub type DigestFinalFunc = unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> ();
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct DigestInfo {
        pub init: Option<DigestInitFunc>,
        pub update: Option<DigestUpdateFunc>,
        pub final_0: Option<DigestFinalFunc>,
        pub state_len: ::core::ffi::c_short,
        pub result_len: ::core::ffi::c_short,
        pub block_len: ::core::ffi::c_short,
    }
    use super::_uint8_t_h::uint8_t;
}

pub mod bits_h {
    #[inline]

    pub unsafe extern "C" fn rol32(mut v: uint32_t, mut s: ::core::ffi::c_int) -> uint32_t {
        v << s | v >> (32 as ::core::ffi::c_int - s)
    }
    use super::_uint32_t_h::uint32_t;
}

pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {

        pub fn memcpy(
            __dst: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
    }
}

pub mod endian_h {
    #[inline]

    pub unsafe extern "C" fn usual_le32enc(mut p: *mut ::core::ffi::c_void, mut x: uint32_t) {
        let mut tmp: uint32_t = x;
        memcpy(
            p,
            &raw mut tmp as *const ::core::ffi::c_void,
            ::core::mem::size_of::<uint32_t>() as size_t,
        );
    }
    use super::_size_t_h::size_t;
    use super::_string_h::memcpy;
    use super::_uint32_t_h::uint32_t;
}
pub use self::_size_t_h::size_t;
use self::_string_h::memcpy;
pub use self::_types_h::__darwin_size_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::bits_h::rol32;
pub use self::digest_h::{DigestFinalFunc, DigestInfo, DigestInitFunc, DigestUpdateFunc};
pub use self::endian_h::usual_le32enc;
pub use self::md5_h::{md5_ctx, MD5_BLOCK_LENGTH, MD5_DIGEST_LENGTH};
#[inline]

unsafe extern "C" fn swap_words(mut _w: *mut uint32_t, mut _n: ::core::ffi::c_int) {}

unsafe extern "C" fn md5_mix(mut ctx: *mut md5_ctx, mut X: *const uint32_t) {
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut d: uint32_t = 0;
    a = (*ctx).a;
    b = (*ctx).b;
    c = (*ctx).c;
    d = (*ctx).d;
    a = b.wrapping_add(rol32(
        a.wrapping_add(b & c | !b & d)
            .wrapping_add(*X.offset(0 as ::core::ffi::c_int as isize))
            .wrapping_add(0xd76aa478 as uint32_t),
        7 as ::core::ffi::c_int,
    ));
    d = a.wrapping_add(rol32(
        d.wrapping_add(a & b | !a & c)
            .wrapping_add(*X.offset(1 as ::core::ffi::c_int as isize))
            .wrapping_add(0xe8c7b756 as uint32_t),
        12 as ::core::ffi::c_int,
    ));
    c = d.wrapping_add(rol32(
        c.wrapping_add(d & a | !d & b)
            .wrapping_add(*X.offset(2 as ::core::ffi::c_int as isize))
            .wrapping_add(0x242070db as uint32_t),
        17 as ::core::ffi::c_int,
    ));
    b = c.wrapping_add(rol32(
        b.wrapping_add(c & d | !c & a)
            .wrapping_add(*X.offset(3 as ::core::ffi::c_int as isize))
            .wrapping_add(0xc1bdceee as uint32_t),
        22 as ::core::ffi::c_int,
    ));
    a = b.wrapping_add(rol32(
        a.wrapping_add(b & c | !b & d)
            .wrapping_add(*X.offset(4 as ::core::ffi::c_int as isize))
            .wrapping_add(0xf57c0faf as uint32_t),
        7 as ::core::ffi::c_int,
    ));
    d = a.wrapping_add(rol32(
        d.wrapping_add(a & b | !a & c)
            .wrapping_add(*X.offset(5 as ::core::ffi::c_int as isize))
            .wrapping_add(0x4787c62a as uint32_t),
        12 as ::core::ffi::c_int,
    ));
    c = d.wrapping_add(rol32(
        c.wrapping_add(d & a | !d & b)
            .wrapping_add(*X.offset(6 as ::core::ffi::c_int as isize))
            .wrapping_add(0xa8304613 as uint32_t),
        17 as ::core::ffi::c_int,
    ));
    b = c.wrapping_add(rol32(
        b.wrapping_add(c & d | !c & a)
            .wrapping_add(*X.offset(7 as ::core::ffi::c_int as isize))
            .wrapping_add(0xfd469501 as uint32_t),
        22 as ::core::ffi::c_int,
    ));
    a = b.wrapping_add(rol32(
        a.wrapping_add(b & c | !b & d)
            .wrapping_add(*X.offset(8 as ::core::ffi::c_int as isize))
            .wrapping_add(0x698098d8 as uint32_t),
        7 as ::core::ffi::c_int,
    ));
    d = a.wrapping_add(rol32(
        d.wrapping_add(a & b | !a & c)
            .wrapping_add(*X.offset(9 as ::core::ffi::c_int as isize))
            .wrapping_add(0x8b44f7af as uint32_t),
        12 as ::core::ffi::c_int,
    ));
    c = d.wrapping_add(rol32(
        c.wrapping_add(d & a | !d & b)
            .wrapping_add(*X.offset(10 as ::core::ffi::c_int as isize))
            .wrapping_add(0xffff5bb1 as uint32_t),
        17 as ::core::ffi::c_int,
    ));
    b = c.wrapping_add(rol32(
        b.wrapping_add(c & d | !c & a)
            .wrapping_add(*X.offset(11 as ::core::ffi::c_int as isize))
            .wrapping_add(0x895cd7be as uint32_t),
        22 as ::core::ffi::c_int,
    ));
    a = b.wrapping_add(rol32(
        a.wrapping_add(b & c | !b & d)
            .wrapping_add(*X.offset(12 as ::core::ffi::c_int as isize))
            .wrapping_add(0x6b901122 as uint32_t),
        7 as ::core::ffi::c_int,
    ));
    d = a.wrapping_add(rol32(
        d.wrapping_add(a & b | !a & c)
            .wrapping_add(*X.offset(13 as ::core::ffi::c_int as isize))
            .wrapping_add(0xfd987193 as uint32_t),
        12 as ::core::ffi::c_int,
    ));
    c = d.wrapping_add(rol32(
        c.wrapping_add(d & a | !d & b)
            .wrapping_add(*X.offset(14 as ::core::ffi::c_int as isize))
            .wrapping_add(0xa679438e as uint32_t),
        17 as ::core::ffi::c_int,
    ));
    b = c.wrapping_add(rol32(
        b.wrapping_add(c & d | !c & a)
            .wrapping_add(*X.offset(15 as ::core::ffi::c_int as isize))
            .wrapping_add(0x49b40821 as uint32_t),
        22 as ::core::ffi::c_int,
    ));
    a = b.wrapping_add(rol32(
        a.wrapping_add(b & d | c & !d)
            .wrapping_add(*X.offset(1 as ::core::ffi::c_int as isize))
            .wrapping_add(0xf61e2562 as uint32_t),
        5 as ::core::ffi::c_int,
    ));
    d = a.wrapping_add(rol32(
        d.wrapping_add(a & c | b & !c)
            .wrapping_add(*X.offset(6 as ::core::ffi::c_int as isize))
            .wrapping_add(0xc040b340 as uint32_t),
        9 as ::core::ffi::c_int,
    ));
    c = d.wrapping_add(rol32(
        c.wrapping_add(d & b | a & !b)
            .wrapping_add(*X.offset(11 as ::core::ffi::c_int as isize))
            .wrapping_add(0x265e5a51 as uint32_t),
        14 as ::core::ffi::c_int,
    ));
    b = c.wrapping_add(rol32(
        b.wrapping_add(c & a | d & !a)
            .wrapping_add(*X.offset(0 as ::core::ffi::c_int as isize))
            .wrapping_add(0xe9b6c7aa as uint32_t),
        20 as ::core::ffi::c_int,
    ));
    a = b.wrapping_add(rol32(
        a.wrapping_add(b & d | c & !d)
            .wrapping_add(*X.offset(5 as ::core::ffi::c_int as isize))
            .wrapping_add(0xd62f105d as uint32_t),
        5 as ::core::ffi::c_int,
    ));
    d = a.wrapping_add(rol32(
        d.wrapping_add(a & c | b & !c)
            .wrapping_add(*X.offset(10 as ::core::ffi::c_int as isize))
            .wrapping_add(0x2441453 as uint32_t),
        9 as ::core::ffi::c_int,
    ));
    c = d.wrapping_add(rol32(
        c.wrapping_add(d & b | a & !b)
            .wrapping_add(*X.offset(15 as ::core::ffi::c_int as isize))
            .wrapping_add(0xd8a1e681 as uint32_t),
        14 as ::core::ffi::c_int,
    ));
    b = c.wrapping_add(rol32(
        b.wrapping_add(c & a | d & !a)
            .wrapping_add(*X.offset(4 as ::core::ffi::c_int as isize))
            .wrapping_add(0xe7d3fbc8 as uint32_t),
        20 as ::core::ffi::c_int,
    ));
    a = b.wrapping_add(rol32(
        a.wrapping_add(b & d | c & !d)
            .wrapping_add(*X.offset(9 as ::core::ffi::c_int as isize))
            .wrapping_add(0x21e1cde6 as uint32_t),
        5 as ::core::ffi::c_int,
    ));
    d = a.wrapping_add(rol32(
        d.wrapping_add(a & c | b & !c)
            .wrapping_add(*X.offset(14 as ::core::ffi::c_int as isize))
            .wrapping_add(0xc33707d6 as uint32_t),
        9 as ::core::ffi::c_int,
    ));
    c = d.wrapping_add(rol32(
        c.wrapping_add(d & b | a & !b)
            .wrapping_add(*X.offset(3 as ::core::ffi::c_int as isize))
            .wrapping_add(0xf4d50d87 as uint32_t),
        14 as ::core::ffi::c_int,
    ));
    b = c.wrapping_add(rol32(
        b.wrapping_add(c & a | d & !a)
            .wrapping_add(*X.offset(8 as ::core::ffi::c_int as isize))
            .wrapping_add(0x455a14ed as uint32_t),
        20 as ::core::ffi::c_int,
    ));
    a = b.wrapping_add(rol32(
        a.wrapping_add(b & d | c & !d)
            .wrapping_add(*X.offset(13 as ::core::ffi::c_int as isize))
            .wrapping_add(0xa9e3e905 as uint32_t),
        5 as ::core::ffi::c_int,
    ));
    d = a.wrapping_add(rol32(
        d.wrapping_add(a & c | b & !c)
            .wrapping_add(*X.offset(2 as ::core::ffi::c_int as isize))
            .wrapping_add(0xfcefa3f8 as uint32_t),
        9 as ::core::ffi::c_int,
    ));
    c = d.wrapping_add(rol32(
        c.wrapping_add(d & b | a & !b)
            .wrapping_add(*X.offset(7 as ::core::ffi::c_int as isize))
            .wrapping_add(0x676f02d9 as uint32_t),
        14 as ::core::ffi::c_int,
    ));
    b = c.wrapping_add(rol32(
        b.wrapping_add(c & a | d & !a)
            .wrapping_add(*X.offset(12 as ::core::ffi::c_int as isize))
            .wrapping_add(0x8d2a4c8a as uint32_t),
        20 as ::core::ffi::c_int,
    ));
    a = b.wrapping_add(rol32(
        a.wrapping_add(b ^ c ^ d)
            .wrapping_add(*X.offset(5 as ::core::ffi::c_int as isize))
            .wrapping_add(0xfffa3942 as uint32_t),
        4 as ::core::ffi::c_int,
    ));
    d = a.wrapping_add(rol32(
        d.wrapping_add(a ^ b ^ c)
            .wrapping_add(*X.offset(8 as ::core::ffi::c_int as isize))
            .wrapping_add(0x8771f681 as uint32_t),
        11 as ::core::ffi::c_int,
    ));
    c = d.wrapping_add(rol32(
        c.wrapping_add(d ^ a ^ b)
            .wrapping_add(*X.offset(11 as ::core::ffi::c_int as isize))
            .wrapping_add(0x6d9d6122 as uint32_t),
        16 as ::core::ffi::c_int,
    ));
    b = c.wrapping_add(rol32(
        b.wrapping_add(c ^ d ^ a)
            .wrapping_add(*X.offset(14 as ::core::ffi::c_int as isize))
            .wrapping_add(0xfde5380c as uint32_t),
        23 as ::core::ffi::c_int,
    ));
    a = b.wrapping_add(rol32(
        a.wrapping_add(b ^ c ^ d)
            .wrapping_add(*X.offset(1 as ::core::ffi::c_int as isize))
            .wrapping_add(0xa4beea44 as uint32_t),
        4 as ::core::ffi::c_int,
    ));
    d = a.wrapping_add(rol32(
        d.wrapping_add(a ^ b ^ c)
            .wrapping_add(*X.offset(4 as ::core::ffi::c_int as isize))
            .wrapping_add(0x4bdecfa9 as uint32_t),
        11 as ::core::ffi::c_int,
    ));
    c = d.wrapping_add(rol32(
        c.wrapping_add(d ^ a ^ b)
            .wrapping_add(*X.offset(7 as ::core::ffi::c_int as isize))
            .wrapping_add(0xf6bb4b60 as uint32_t),
        16 as ::core::ffi::c_int,
    ));
    b = c.wrapping_add(rol32(
        b.wrapping_add(c ^ d ^ a)
            .wrapping_add(*X.offset(10 as ::core::ffi::c_int as isize))
            .wrapping_add(0xbebfbc70 as uint32_t),
        23 as ::core::ffi::c_int,
    ));
    a = b.wrapping_add(rol32(
        a.wrapping_add(b ^ c ^ d)
            .wrapping_add(*X.offset(13 as ::core::ffi::c_int as isize))
            .wrapping_add(0x289b7ec6 as uint32_t),
        4 as ::core::ffi::c_int,
    ));
    d = a.wrapping_add(rol32(
        d.wrapping_add(a ^ b ^ c)
            .wrapping_add(*X.offset(0 as ::core::ffi::c_int as isize))
            .wrapping_add(0xeaa127fa as uint32_t),
        11 as ::core::ffi::c_int,
    ));
    c = d.wrapping_add(rol32(
        c.wrapping_add(d ^ a ^ b)
            .wrapping_add(*X.offset(3 as ::core::ffi::c_int as isize))
            .wrapping_add(0xd4ef3085 as uint32_t),
        16 as ::core::ffi::c_int,
    ));
    b = c.wrapping_add(rol32(
        b.wrapping_add(c ^ d ^ a)
            .wrapping_add(*X.offset(6 as ::core::ffi::c_int as isize))
            .wrapping_add(0x4881d05 as uint32_t),
        23 as ::core::ffi::c_int,
    ));
    a = b.wrapping_add(rol32(
        a.wrapping_add(b ^ c ^ d)
            .wrapping_add(*X.offset(9 as ::core::ffi::c_int as isize))
            .wrapping_add(0xd9d4d039 as uint32_t),
        4 as ::core::ffi::c_int,
    ));
    d = a.wrapping_add(rol32(
        d.wrapping_add(a ^ b ^ c)
            .wrapping_add(*X.offset(12 as ::core::ffi::c_int as isize))
            .wrapping_add(0xe6db99e5 as uint32_t),
        11 as ::core::ffi::c_int,
    ));
    c = d.wrapping_add(rol32(
        c.wrapping_add(d ^ a ^ b)
            .wrapping_add(*X.offset(15 as ::core::ffi::c_int as isize))
            .wrapping_add(0x1fa27cf8 as uint32_t),
        16 as ::core::ffi::c_int,
    ));
    b = c.wrapping_add(rol32(
        b.wrapping_add(c ^ d ^ a)
            .wrapping_add(*X.offset(2 as ::core::ffi::c_int as isize))
            .wrapping_add(0xc4ac5665 as uint32_t),
        23 as ::core::ffi::c_int,
    ));
    a = b.wrapping_add(rol32(
        a.wrapping_add(c ^ (b | !d))
            .wrapping_add(*X.offset(0 as ::core::ffi::c_int as isize))
            .wrapping_add(0xf4292244 as uint32_t),
        6 as ::core::ffi::c_int,
    ));
    d = a.wrapping_add(rol32(
        d.wrapping_add(b ^ (a | !c))
            .wrapping_add(*X.offset(7 as ::core::ffi::c_int as isize))
            .wrapping_add(0x432aff97 as uint32_t),
        10 as ::core::ffi::c_int,
    ));
    c = d.wrapping_add(rol32(
        c.wrapping_add(a ^ (d | !b))
            .wrapping_add(*X.offset(14 as ::core::ffi::c_int as isize))
            .wrapping_add(0xab9423a7 as uint32_t),
        15 as ::core::ffi::c_int,
    ));
    b = c.wrapping_add(rol32(
        b.wrapping_add(d ^ (c | !a))
            .wrapping_add(*X.offset(5 as ::core::ffi::c_int as isize))
            .wrapping_add(0xfc93a039 as uint32_t),
        21 as ::core::ffi::c_int,
    ));
    a = b.wrapping_add(rol32(
        a.wrapping_add(c ^ (b | !d))
            .wrapping_add(*X.offset(12 as ::core::ffi::c_int as isize))
            .wrapping_add(0x655b59c3 as uint32_t),
        6 as ::core::ffi::c_int,
    ));
    d = a.wrapping_add(rol32(
        d.wrapping_add(b ^ (a | !c))
            .wrapping_add(*X.offset(3 as ::core::ffi::c_int as isize))
            .wrapping_add(0x8f0ccc92 as uint32_t),
        10 as ::core::ffi::c_int,
    ));
    c = d.wrapping_add(rol32(
        c.wrapping_add(a ^ (d | !b))
            .wrapping_add(*X.offset(10 as ::core::ffi::c_int as isize))
            .wrapping_add(0xffeff47d as uint32_t),
        15 as ::core::ffi::c_int,
    ));
    b = c.wrapping_add(rol32(
        b.wrapping_add(d ^ (c | !a))
            .wrapping_add(*X.offset(1 as ::core::ffi::c_int as isize))
            .wrapping_add(0x85845dd1 as uint32_t),
        21 as ::core::ffi::c_int,
    ));
    a = b.wrapping_add(rol32(
        a.wrapping_add(c ^ (b | !d))
            .wrapping_add(*X.offset(8 as ::core::ffi::c_int as isize))
            .wrapping_add(0x6fa87e4f as uint32_t),
        6 as ::core::ffi::c_int,
    ));
    d = a.wrapping_add(rol32(
        d.wrapping_add(b ^ (a | !c))
            .wrapping_add(*X.offset(15 as ::core::ffi::c_int as isize))
            .wrapping_add(0xfe2ce6e0 as uint32_t),
        10 as ::core::ffi::c_int,
    ));
    c = d.wrapping_add(rol32(
        c.wrapping_add(a ^ (d | !b))
            .wrapping_add(*X.offset(6 as ::core::ffi::c_int as isize))
            .wrapping_add(0xa3014314 as uint32_t),
        15 as ::core::ffi::c_int,
    ));
    b = c.wrapping_add(rol32(
        b.wrapping_add(d ^ (c | !a))
            .wrapping_add(*X.offset(13 as ::core::ffi::c_int as isize))
            .wrapping_add(0x4e0811a1 as uint32_t),
        21 as ::core::ffi::c_int,
    ));
    a = b.wrapping_add(rol32(
        a.wrapping_add(c ^ (b | !d))
            .wrapping_add(*X.offset(4 as ::core::ffi::c_int as isize))
            .wrapping_add(0xf7537e82 as uint32_t),
        6 as ::core::ffi::c_int,
    ));
    d = a.wrapping_add(rol32(
        d.wrapping_add(b ^ (a | !c))
            .wrapping_add(*X.offset(11 as ::core::ffi::c_int as isize))
            .wrapping_add(0xbd3af235 as uint32_t),
        10 as ::core::ffi::c_int,
    ));
    c = d.wrapping_add(rol32(
        c.wrapping_add(a ^ (d | !b))
            .wrapping_add(*X.offset(2 as ::core::ffi::c_int as isize))
            .wrapping_add(0x2ad7d2bb as uint32_t),
        15 as ::core::ffi::c_int,
    ));
    b = c.wrapping_add(rol32(
        b.wrapping_add(d ^ (c | !a))
            .wrapping_add(*X.offset(9 as ::core::ffi::c_int as isize))
            .wrapping_add(0xeb86d391 as uint32_t),
        21 as ::core::ffi::c_int,
    ));
    (*ctx).a = (*ctx).a.wrapping_add(a);
    (*ctx).b = (*ctx).b.wrapping_add(b);
    (*ctx).c = (*ctx).c.wrapping_add(c);
    (*ctx).d = (*ctx).d.wrapping_add(d);
}
#[no_mangle]

pub unsafe extern "C" fn md5_reset(mut ctx: *mut md5_ctx) {
    (*ctx).nbytes = 0 as uint64_t;
    (*ctx).a = 0x67452301 as uint32_t;
    (*ctx).b = 0xefcdab89 as ::core::ffi::c_uint as uint32_t;
    (*ctx).c = 0x98badcfe as ::core::ffi::c_uint as uint32_t;
    (*ctx).d = 0x10325476 as uint32_t;
}
#[no_mangle]

pub unsafe extern "C" fn md5_update(
    mut ctx: *mut md5_ctx,
    mut data: *const ::core::ffi::c_void,
    mut len: ::core::ffi::c_uint,
) {
    let mut n: ::core::ffi::c_uint = 0;
    let mut ptr = data as *const uint8_t;
    let mut buf = &raw mut (*ctx).buf as *mut uint32_t as *mut uint8_t;
    while len > 0 as ::core::ffi::c_uint {
        n = (MD5_BLOCK_LENGTH as uint64_t)
            .wrapping_sub((*ctx).nbytes & (MD5_BLOCK_LENGTH - 1 as ::core::ffi::c_int) as uint64_t)
            as ::core::ffi::c_uint;
        if n > len {
            n = len;
        }
        memcpy(
            buf.offset(
                ((*ctx).nbytes & (MD5_BLOCK_LENGTH - 1 as ::core::ffi::c_int) as uint64_t) as isize,
            ) as *mut ::core::ffi::c_void,
            ptr as *const ::core::ffi::c_void,
            n as size_t,
        );
        ptr = ptr.offset(n as isize);
        len = len.wrapping_sub(n);
        (*ctx).nbytes = (*ctx).nbytes.wrapping_add(n as uint64_t);
        if (*ctx).nbytes & (MD5_BLOCK_LENGTH - 1 as ::core::ffi::c_int) as uint64_t == 0 as uint64_t
        {
            swap_words(
                &raw mut (*ctx).buf as *mut uint32_t,
                16 as ::core::ffi::c_int,
            );
            md5_mix(ctx, &raw mut (*ctx).buf as *mut uint32_t);
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn md5_final(mut ctx: *mut md5_ctx, mut dst: *mut uint8_t) {
    static mut padding: [uint8_t; 64] = [
        0x80 as ::core::ffi::c_int as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut final_len: uint64_t = (*ctx).nbytes.wrapping_mul(8 as uint64_t);
    let mut pad_len: ::core::ffi::c_int = 0;
    let mut pos = ((*ctx).nbytes & (MD5_BLOCK_LENGTH - 1 as ::core::ffi::c_int) as uint64_t)
        as ::core::ffi::c_int;
    pad_len = MD5_BLOCK_LENGTH - 8 as ::core::ffi::c_int - pos;
    if pad_len <= 0 as ::core::ffi::c_int {
        pad_len += MD5_BLOCK_LENGTH;
    }
    md5_update(
        ctx,
        &raw const padding as *const uint8_t as *const ::core::ffi::c_void,
        pad_len as ::core::ffi::c_uint,
    );
    swap_words(
        &raw mut (*ctx).buf as *mut uint32_t,
        14 as ::core::ffi::c_int,
    );
    (*ctx).buf[14 as ::core::ffi::c_int as usize] = final_len as uint32_t;
    (*ctx).buf[15 as ::core::ffi::c_int as usize] =
        (final_len >> 32 as ::core::ffi::c_int) as uint32_t;
    md5_mix(ctx, &raw mut (*ctx).buf as *mut uint32_t);
    usual_le32enc(
        dst.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        (*ctx).a,
    );
    usual_le32enc(
        dst.offset(4 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        (*ctx).b,
    );
    usual_le32enc(
        dst.offset(8 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        (*ctx).c,
    );
    usual_le32enc(
        dst.offset(12 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        (*ctx).d,
    );
}

static mut md5: DigestInfo = unsafe {
    DigestInfo {
        init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut md5_ctx) -> ()>,
            Option<DigestInitFunc>,
        >(Some(md5_reset as unsafe extern "C" fn(*mut md5_ctx) -> ())),
        update: ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut md5_ctx,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_uint,
                ) -> (),
            >,
            Option<DigestUpdateFunc>,
        >(Some(
            md5_update
                as unsafe extern "C" fn(
                    *mut md5_ctx,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_uint,
                ) -> (),
        )),
        final_0: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut md5_ctx, *mut uint8_t) -> ()>,
            Option<DigestFinalFunc>,
        >(Some(
            md5_final as unsafe extern "C" fn(*mut md5_ctx, *mut uint8_t) -> (),
        )),
        state_len: ::core::mem::size_of::<md5_ctx>() as ::core::ffi::c_short,
        result_len: MD5_DIGEST_LENGTH as ::core::ffi::c_short,
        block_len: MD5_BLOCK_LENGTH as ::core::ffi::c_short,
    }
};
#[no_mangle]

pub unsafe extern "C" fn digest_MD5() -> *const DigestInfo {
    &raw const md5
}
