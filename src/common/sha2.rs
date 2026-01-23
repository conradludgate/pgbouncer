#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:54"]
pub mod _types_h {
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:54"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:54"]
pub mod _uint8_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint8_t = u8;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:54"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = u32;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint64_t.h:54"]
pub mod _uint64_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint64_t = u64;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/sha2_int.h:56"]
pub mod sha2_int_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:9"]
    pub struct pg_sha256_ctx {
        pub state: [uint32_t; 8],
        pub bitcount: uint64_t,
        pub buffer: [uint8_t; 64],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "61:9"]
    pub struct pg_sha512_ctx {
        pub state: [uint64_t; 8],
        pub bitcount: [uint64_t; 2],
        pub buffer: [uint8_t; 128],
    }
    #[c2rust::src_loc = "67:1"]
    pub type pg_sha224_ctx = pg_sha256_ctx;
    #[c2rust::src_loc = "68:1"]
    pub type pg_sha384_ctx = pg_sha512_ctx;
    use super::_uint32_t_h::uint32_t;
    use super::_uint64_t_h::uint64_t;
    use super::_uint8_t_h::uint8_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:54"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:54"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:54"]
pub mod sys__types_h {
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/sha2.h:56"]
pub mod sha2_h {
    #[c2rust::src_loc = "20:9"]
    pub const PG_SHA224_DIGEST_LENGTH: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
    #[c2rust::src_loc = "22:9"]
    pub const PG_SHA256_BLOCK_LENGTH: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
    #[c2rust::src_loc = "23:9"]
    pub const PG_SHA256_DIGEST_LENGTH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
    #[c2rust::src_loc = "25:9"]
    pub const PG_SHA384_BLOCK_LENGTH: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const PG_SHA384_DIGEST_LENGTH: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
    #[c2rust::src_loc = "28:9"]
    pub const PG_SHA512_BLOCK_LENGTH: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
    #[c2rust::src_loc = "29:9"]
    pub const PG_SHA512_DIGEST_LENGTH: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
}
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
use self::_string_h::{memcpy, memset};
pub use self::_types_h::__darwin_size_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::sha2_h::{
    PG_SHA224_DIGEST_LENGTH, PG_SHA256_BLOCK_LENGTH, PG_SHA256_DIGEST_LENGTH,
    PG_SHA384_BLOCK_LENGTH, PG_SHA384_DIGEST_LENGTH, PG_SHA512_BLOCK_LENGTH,
    PG_SHA512_DIGEST_LENGTH,
};
pub use self::sha2_int_h::{pg_sha224_ctx, pg_sha256_ctx, pg_sha384_ctx, pg_sha512_ctx};
pub use self::sys__types_h::__DARWIN_NULL;
#[c2rust::src_loc = "85:9"]
pub const PG_SHA256_SHORT_BLOCK_LENGTH: ::core::ffi::c_int =
    PG_SHA256_BLOCK_LENGTH - 8 as ::core::ffi::c_int;
#[c2rust::src_loc = "87:9"]
pub const PG_SHA512_SHORT_BLOCK_LENGTH: ::core::ffi::c_int =
    PG_SHA512_BLOCK_LENGTH - 16 as ::core::ffi::c_int;
#[c2rust::src_loc = "161:1"]
static mut K256: [uint32_t; 64] = [
    0x428a2f98 as ::core::ffi::c_ulong as uint32_t,
    0x71374491 as ::core::ffi::c_ulong as uint32_t,
    0xb5c0fbcf as ::core::ffi::c_ulong as uint32_t,
    0xe9b5dba5 as ::core::ffi::c_ulong as uint32_t,
    0x3956c25b as ::core::ffi::c_ulong as uint32_t,
    0x59f111f1 as ::core::ffi::c_ulong as uint32_t,
    0x923f82a4 as ::core::ffi::c_ulong as uint32_t,
    0xab1c5ed5 as ::core::ffi::c_ulong as uint32_t,
    0xd807aa98 as ::core::ffi::c_ulong as uint32_t,
    0x12835b01 as ::core::ffi::c_ulong as uint32_t,
    0x243185be as ::core::ffi::c_ulong as uint32_t,
    0x550c7dc3 as ::core::ffi::c_ulong as uint32_t,
    0x72be5d74 as ::core::ffi::c_ulong as uint32_t,
    0x80deb1fe as ::core::ffi::c_ulong as uint32_t,
    0x9bdc06a7 as ::core::ffi::c_ulong as uint32_t,
    0xc19bf174 as ::core::ffi::c_ulong as uint32_t,
    0xe49b69c1 as ::core::ffi::c_ulong as uint32_t,
    0xefbe4786 as ::core::ffi::c_ulong as uint32_t,
    0xfc19dc6 as ::core::ffi::c_ulong as uint32_t,
    0x240ca1cc as ::core::ffi::c_ulong as uint32_t,
    0x2de92c6f as ::core::ffi::c_ulong as uint32_t,
    0x4a7484aa as ::core::ffi::c_ulong as uint32_t,
    0x5cb0a9dc as ::core::ffi::c_ulong as uint32_t,
    0x76f988da as ::core::ffi::c_ulong as uint32_t,
    0x983e5152 as ::core::ffi::c_ulong as uint32_t,
    0xa831c66d as ::core::ffi::c_ulong as uint32_t,
    0xb00327c8 as ::core::ffi::c_ulong as uint32_t,
    0xbf597fc7 as ::core::ffi::c_ulong as uint32_t,
    0xc6e00bf3 as ::core::ffi::c_ulong as uint32_t,
    0xd5a79147 as ::core::ffi::c_ulong as uint32_t,
    0x6ca6351 as ::core::ffi::c_ulong as uint32_t,
    0x14292967 as ::core::ffi::c_ulong as uint32_t,
    0x27b70a85 as ::core::ffi::c_ulong as uint32_t,
    0x2e1b2138 as ::core::ffi::c_ulong as uint32_t,
    0x4d2c6dfc as ::core::ffi::c_ulong as uint32_t,
    0x53380d13 as ::core::ffi::c_ulong as uint32_t,
    0x650a7354 as ::core::ffi::c_ulong as uint32_t,
    0x766a0abb as ::core::ffi::c_ulong as uint32_t,
    0x81c2c92e as ::core::ffi::c_ulong as uint32_t,
    0x92722c85 as ::core::ffi::c_ulong as uint32_t,
    0xa2bfe8a1 as ::core::ffi::c_ulong as uint32_t,
    0xa81a664b as ::core::ffi::c_ulong as uint32_t,
    0xc24b8b70 as ::core::ffi::c_ulong as uint32_t,
    0xc76c51a3 as ::core::ffi::c_ulong as uint32_t,
    0xd192e819 as ::core::ffi::c_ulong as uint32_t,
    0xd6990624 as ::core::ffi::c_ulong as uint32_t,
    0xf40e3585 as ::core::ffi::c_ulong as uint32_t,
    0x106aa070 as ::core::ffi::c_ulong as uint32_t,
    0x19a4c116 as ::core::ffi::c_ulong as uint32_t,
    0x1e376c08 as ::core::ffi::c_ulong as uint32_t,
    0x2748774c as ::core::ffi::c_ulong as uint32_t,
    0x34b0bcb5 as ::core::ffi::c_ulong as uint32_t,
    0x391c0cb3 as ::core::ffi::c_ulong as uint32_t,
    0x4ed8aa4a as ::core::ffi::c_ulong as uint32_t,
    0x5b9cca4f as ::core::ffi::c_ulong as uint32_t,
    0x682e6ff3 as ::core::ffi::c_ulong as uint32_t,
    0x748f82ee as ::core::ffi::c_ulong as uint32_t,
    0x78a5636f as ::core::ffi::c_ulong as uint32_t,
    0x84c87814 as ::core::ffi::c_ulong as uint32_t,
    0x8cc70208 as ::core::ffi::c_ulong as uint32_t,
    0x90befffa as ::core::ffi::c_ulong as uint32_t,
    0xa4506ceb as ::core::ffi::c_ulong as uint32_t,
    0xbef9a3f7 as ::core::ffi::c_ulong as uint32_t,
    0xc67178f2 as ::core::ffi::c_ulong as uint32_t,
];
#[c2rust::src_loc = "181:1"]
static mut sha224_initial_hash_value: [uint32_t; 8] = [
    0xc1059ed8 as ::core::ffi::c_ulong as uint32_t,
    0x367cd507 as ::core::ffi::c_ulong as uint32_t,
    0x3070dd17 as ::core::ffi::c_ulong as uint32_t,
    0xf70e5939 as ::core::ffi::c_ulong as uint32_t,
    0xffc00b31 as ::core::ffi::c_ulong as uint32_t,
    0x68581511 as ::core::ffi::c_ulong as uint32_t,
    0x64f98fa7 as ::core::ffi::c_ulong as uint32_t,
    0xbefa4fa4 as ::core::ffi::c_ulong as uint32_t,
];
#[c2rust::src_loc = "193:1"]
static mut sha256_initial_hash_value: [uint32_t; 8] = [
    0x6a09e667 as ::core::ffi::c_ulong as uint32_t,
    0xbb67ae85 as ::core::ffi::c_ulong as uint32_t,
    0x3c6ef372 as ::core::ffi::c_ulong as uint32_t,
    0xa54ff53a as ::core::ffi::c_ulong as uint32_t,
    0x510e527f as ::core::ffi::c_ulong as uint32_t,
    0x9b05688c as ::core::ffi::c_ulong as uint32_t,
    0x1f83d9ab as ::core::ffi::c_ulong as uint32_t,
    0x5be0cd19 as ::core::ffi::c_ulong as uint32_t,
];
#[c2rust::src_loc = "205:1"]
static mut K512: [uint64_t; 80] = [
    0x428a2f98d728ae22 as ::core::ffi::c_ulonglong,
    0x7137449123ef65cd as ::core::ffi::c_ulonglong,
    0xb5c0fbcfec4d3b2f as ::core::ffi::c_ulonglong,
    0xe9b5dba58189dbbc as ::core::ffi::c_ulonglong,
    0x3956c25bf348b538 as ::core::ffi::c_ulonglong,
    0x59f111f1b605d019 as ::core::ffi::c_ulonglong,
    0x923f82a4af194f9b as ::core::ffi::c_ulonglong,
    0xab1c5ed5da6d8118 as ::core::ffi::c_ulonglong,
    0xd807aa98a3030242 as ::core::ffi::c_ulonglong,
    0x12835b0145706fbe as ::core::ffi::c_ulonglong,
    0x243185be4ee4b28c as ::core::ffi::c_ulonglong,
    0x550c7dc3d5ffb4e2 as ::core::ffi::c_ulonglong,
    0x72be5d74f27b896f as ::core::ffi::c_ulonglong,
    0x80deb1fe3b1696b1 as ::core::ffi::c_ulonglong,
    0x9bdc06a725c71235 as ::core::ffi::c_ulonglong,
    0xc19bf174cf692694 as ::core::ffi::c_ulonglong,
    0xe49b69c19ef14ad2 as ::core::ffi::c_ulonglong,
    0xefbe4786384f25e3 as ::core::ffi::c_ulonglong,
    0xfc19dc68b8cd5b5 as ::core::ffi::c_ulonglong,
    0x240ca1cc77ac9c65 as ::core::ffi::c_ulonglong,
    0x2de92c6f592b0275 as ::core::ffi::c_ulonglong,
    0x4a7484aa6ea6e483 as ::core::ffi::c_ulonglong,
    0x5cb0a9dcbd41fbd4 as ::core::ffi::c_ulonglong,
    0x76f988da831153b5 as ::core::ffi::c_ulonglong,
    0x983e5152ee66dfab as ::core::ffi::c_ulonglong,
    0xa831c66d2db43210 as ::core::ffi::c_ulonglong,
    0xb00327c898fb213f as ::core::ffi::c_ulonglong,
    0xbf597fc7beef0ee4 as ::core::ffi::c_ulonglong,
    0xc6e00bf33da88fc2 as ::core::ffi::c_ulonglong,
    0xd5a79147930aa725 as ::core::ffi::c_ulonglong,
    0x6ca6351e003826f as ::core::ffi::c_ulonglong,
    0x142929670a0e6e70 as ::core::ffi::c_ulonglong,
    0x27b70a8546d22ffc as ::core::ffi::c_ulonglong,
    0x2e1b21385c26c926 as ::core::ffi::c_ulonglong,
    0x4d2c6dfc5ac42aed as ::core::ffi::c_ulonglong,
    0x53380d139d95b3df as ::core::ffi::c_ulonglong,
    0x650a73548baf63de as ::core::ffi::c_ulonglong,
    0x766a0abb3c77b2a8 as ::core::ffi::c_ulonglong,
    0x81c2c92e47edaee6 as ::core::ffi::c_ulonglong,
    0x92722c851482353b as ::core::ffi::c_ulonglong,
    0xa2bfe8a14cf10364 as ::core::ffi::c_ulonglong,
    0xa81a664bbc423001 as ::core::ffi::c_ulonglong,
    0xc24b8b70d0f89791 as ::core::ffi::c_ulonglong,
    0xc76c51a30654be30 as ::core::ffi::c_ulonglong,
    0xd192e819d6ef5218 as ::core::ffi::c_ulonglong,
    0xd69906245565a910 as ::core::ffi::c_ulonglong,
    0xf40e35855771202a as ::core::ffi::c_ulonglong,
    0x106aa07032bbd1b8 as ::core::ffi::c_ulonglong,
    0x19a4c116b8d2d0c8 as ::core::ffi::c_ulonglong,
    0x1e376c085141ab53 as ::core::ffi::c_ulonglong,
    0x2748774cdf8eeb99 as ::core::ffi::c_ulonglong,
    0x34b0bcb5e19b48a8 as ::core::ffi::c_ulonglong,
    0x391c0cb3c5c95a63 as ::core::ffi::c_ulonglong,
    0x4ed8aa4ae3418acb as ::core::ffi::c_ulonglong,
    0x5b9cca4f7763e373 as ::core::ffi::c_ulonglong,
    0x682e6ff3d6b2b8a3 as ::core::ffi::c_ulonglong,
    0x748f82ee5defb2fc as ::core::ffi::c_ulonglong,
    0x78a5636f43172f60 as ::core::ffi::c_ulonglong,
    0x84c87814a1f0ab72 as ::core::ffi::c_ulonglong,
    0x8cc702081a6439ec as ::core::ffi::c_ulonglong,
    0x90befffa23631e28 as ::core::ffi::c_ulonglong,
    0xa4506cebde82bde9 as ::core::ffi::c_ulonglong,
    0xbef9a3f7b2c67915 as ::core::ffi::c_ulonglong,
    0xc67178f2e372532b as ::core::ffi::c_ulonglong,
    0xca273eceea26619c as ::core::ffi::c_ulonglong,
    0xd186b8c721c0c207 as ::core::ffi::c_ulonglong,
    0xeada7dd6cde0eb1e as ::core::ffi::c_ulonglong,
    0xf57d4f7fee6ed178 as ::core::ffi::c_ulonglong,
    0x6f067aa72176fba as ::core::ffi::c_ulonglong,
    0xa637dc5a2c898a6 as ::core::ffi::c_ulonglong,
    0x113f9804bef90dae as ::core::ffi::c_ulonglong,
    0x1b710b35131c471b as ::core::ffi::c_ulonglong,
    0x28db77f523047d84 as ::core::ffi::c_ulonglong,
    0x32caab7b40c72493 as ::core::ffi::c_ulonglong,
    0x3c9ebe0a15c9bebc as ::core::ffi::c_ulonglong,
    0x431d67c49c100d4c as ::core::ffi::c_ulonglong,
    0x4cc5d4becb3e42b6 as ::core::ffi::c_ulonglong,
    0x597f299cfc657e2a as ::core::ffi::c_ulonglong,
    0x5fcb6fab3ad6faec as ::core::ffi::c_ulonglong,
    0x6c44198c4a475817 as ::core::ffi::c_ulonglong,
];
#[c2rust::src_loc = "249:1"]
static mut sha384_initial_hash_value: [uint64_t; 8] = [
    0xcbbb9d5dc1059ed8 as ::core::ffi::c_ulonglong,
    0x629a292a367cd507 as ::core::ffi::c_ulonglong,
    0x9159015a3070dd17 as ::core::ffi::c_ulonglong,
    0x152fecd8f70e5939 as ::core::ffi::c_ulonglong,
    0x67332667ffc00b31 as ::core::ffi::c_ulonglong,
    0x8eb44a8768581511 as ::core::ffi::c_ulonglong,
    0xdb0c2e0d64f98fa7 as ::core::ffi::c_ulonglong,
    0x47b5481dbefa4fa4 as ::core::ffi::c_ulonglong,
];
#[c2rust::src_loc = "261:1"]
static mut sha512_initial_hash_value: [uint64_t; 8] = [
    0x6a09e667f3bcc908 as ::core::ffi::c_ulonglong,
    0xbb67ae8584caa73b as ::core::ffi::c_ulonglong,
    0x3c6ef372fe94f82b as ::core::ffi::c_ulonglong,
    0xa54ff53a5f1d36f1 as ::core::ffi::c_ulonglong,
    0x510e527fade682d1 as ::core::ffi::c_ulonglong,
    0x9b05688c2b3e6c1f as ::core::ffi::c_ulonglong,
    0x1f83d9abfb41bd6b as ::core::ffi::c_ulonglong,
    0x5be0cd19137e2179 as ::core::ffi::c_ulonglong,
];
#[no_mangle]
#[c2rust::src_loc = "274:1"]
pub unsafe extern "C" fn pg_sha256_init(mut context: *mut pg_sha256_ctx) {
    if context.is_null() {
        return;
    }
    memcpy(
        &raw mut (*context).state as *mut uint32_t as *mut ::core::ffi::c_void,
        &raw const sha256_initial_hash_value as *const uint32_t as *const ::core::ffi::c_void,
        PG_SHA256_DIGEST_LENGTH as size_t,
    );
    memset(
        &raw mut (*context).buffer as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        PG_SHA256_BLOCK_LENGTH as size_t,
    );
    (*context).bitcount = 0 as uint64_t;
}
#[c2rust::src_loc = "381:1"]
unsafe extern "C" fn SHA256_Transform(mut context: *mut pg_sha256_ctx, mut data: *const uint8_t) {
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut d: uint32_t = 0;
    let mut e: uint32_t = 0;
    let mut f: uint32_t = 0;
    let mut g: uint32_t = 0;
    let mut h: uint32_t = 0;
    let mut s0: uint32_t = 0;
    let mut s1: uint32_t = 0;
    let mut T1: uint32_t = 0;
    let mut T2: uint32_t = 0;
    let mut W256 = ::core::ptr::null_mut::<uint32_t>();
    let mut j: ::core::ffi::c_int = 0;
    W256 = &raw mut (*context).buffer as *mut uint8_t as *mut uint32_t;
    a = (*context).state[0 as ::core::ffi::c_int as usize];
    b = (*context).state[1 as ::core::ffi::c_int as usize];
    c = (*context).state[2 as ::core::ffi::c_int as usize];
    d = (*context).state[3 as ::core::ffi::c_int as usize];
    e = (*context).state[4 as ::core::ffi::c_int as usize];
    f = (*context).state[5 as ::core::ffi::c_int as usize];
    g = (*context).state[6 as ::core::ffi::c_int as usize];
    h = (*context).state[7 as ::core::ffi::c_int as usize];
    j = 0 as ::core::ffi::c_int;
    loop {
        *W256.offset(j as isize) = *data.offset(3 as ::core::ffi::c_int as isize) as uint32_t
            | (*data.offset(2 as ::core::ffi::c_int as isize) as uint32_t)
                << 8 as ::core::ffi::c_int
            | (*data.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
                << 16 as ::core::ffi::c_int
            | (*data.offset(0 as ::core::ffi::c_int as isize) as uint32_t)
                << 24 as ::core::ffi::c_int;
        data = data.offset(4 as ::core::ffi::c_int as isize);
        T1 = h
            .wrapping_add(
                (e >> 6 as ::core::ffi::c_int
                    | e << (32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int))
                    ^ (e >> 11 as ::core::ffi::c_int
                        | e << (32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int))
                    ^ (e >> 25 as ::core::ffi::c_int
                        | e << (32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)),
            )
            .wrapping_add(e & f ^ !e & g)
            .wrapping_add(K256[j as usize])
            .wrapping_add(*W256.offset(j as isize));
        T2 = ((a >> 2 as ::core::ffi::c_int
            | a << (32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int))
            ^ (a >> 13 as ::core::ffi::c_int
                | a << (32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int))
            ^ (a >> 22 as ::core::ffi::c_int
                | a << (32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int)))
            .wrapping_add(a & b ^ a & c ^ b & c);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(T1);
        d = c;
        c = b;
        b = a;
        a = T1.wrapping_add(T2);
        j += 1;
        if j >= 16 as ::core::ffi::c_int {
            break;
        }
    }
    loop {
        s0 = *W256.offset(((j + 1 as ::core::ffi::c_int) & 0xf as ::core::ffi::c_int) as isize);
        s0 = (s0 >> 7 as ::core::ffi::c_int
            | s0 << (32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int))
            ^ (s0 >> 18 as ::core::ffi::c_int
                | s0 << (32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int))
            ^ s0 >> 3 as ::core::ffi::c_int;
        s1 = *W256.offset(((j + 14 as ::core::ffi::c_int) & 0xf as ::core::ffi::c_int) as isize);
        s1 = (s1 >> 17 as ::core::ffi::c_int
            | s1 << (32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int))
            ^ (s1 >> 19 as ::core::ffi::c_int
                | s1 << (32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int))
            ^ s1 >> 10 as ::core::ffi::c_int;
        let fresh0 = &mut *W256.offset((j & 0xf as ::core::ffi::c_int) as isize);
        *fresh0 = (*fresh0).wrapping_add(
            s1.wrapping_add(
                *W256.offset(((j + 9 as ::core::ffi::c_int) & 0xf as ::core::ffi::c_int) as isize),
            )
            .wrapping_add(s0),
        );
        T1 = h
            .wrapping_add(
                (e >> 6 as ::core::ffi::c_int
                    | e << (32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int))
                    ^ (e >> 11 as ::core::ffi::c_int
                        | e << (32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int))
                    ^ (e >> 25 as ::core::ffi::c_int
                        | e << (32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)),
            )
            .wrapping_add(e & f ^ !e & g)
            .wrapping_add(K256[j as usize])
            .wrapping_add(*fresh0);
        T2 = ((a >> 2 as ::core::ffi::c_int
            | a << (32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int))
            ^ (a >> 13 as ::core::ffi::c_int
                | a << (32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int))
            ^ (a >> 22 as ::core::ffi::c_int
                | a << (32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int)))
            .wrapping_add(a & b ^ a & c ^ b & c);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(T1);
        d = c;
        c = b;
        b = a;
        a = T1.wrapping_add(T2);
        j += 1;
        if j >= 64 as ::core::ffi::c_int {
            break;
        }
    }
    (*context).state[0 as ::core::ffi::c_int as usize] =
        (*context).state[0 as ::core::ffi::c_int as usize].wrapping_add(a);
    (*context).state[1 as ::core::ffi::c_int as usize] =
        (*context).state[1 as ::core::ffi::c_int as usize].wrapping_add(b);
    (*context).state[2 as ::core::ffi::c_int as usize] =
        (*context).state[2 as ::core::ffi::c_int as usize].wrapping_add(c);
    (*context).state[3 as ::core::ffi::c_int as usize] =
        (*context).state[3 as ::core::ffi::c_int as usize].wrapping_add(d);
    (*context).state[4 as ::core::ffi::c_int as usize] =
        (*context).state[4 as ::core::ffi::c_int as usize].wrapping_add(e);
    (*context).state[5 as ::core::ffi::c_int as usize] =
        (*context).state[5 as ::core::ffi::c_int as usize].wrapping_add(f);
    (*context).state[6 as ::core::ffi::c_int as usize] =
        (*context).state[6 as ::core::ffi::c_int as usize].wrapping_add(g);
    (*context).state[7 as ::core::ffi::c_int as usize] =
        (*context).state[7 as ::core::ffi::c_int as usize].wrapping_add(h);
    T2 = 0 as uint32_t;
    T1 = T2;
    h = T1;
    g = h;
    f = g;
    e = f;
    d = e;
    c = d;
    b = c;
    a = b;
}
#[no_mangle]
#[c2rust::src_loc = "471:1"]
pub unsafe extern "C" fn pg_sha256_update(
    mut context: *mut pg_sha256_ctx,
    mut data: *const uint8_t,
    mut len: size_t,
) {
    let mut freespace: size_t = 0;
    let mut usedspace: size_t = 0;
    if len == 0 as size_t {
        return;
    }
    usedspace = ((*context).bitcount >> 3 as ::core::ffi::c_int)
        .wrapping_rem(PG_SHA256_BLOCK_LENGTH as uint64_t) as size_t;
    if usedspace > 0 as size_t {
        freespace = (PG_SHA256_BLOCK_LENGTH as size_t).wrapping_sub(usedspace);
        if len >= freespace {
            memcpy(
                (&raw mut (*context).buffer as *mut uint8_t).add(usedspace) as *mut uint8_t
                    as *mut ::core::ffi::c_void,
                data as *const ::core::ffi::c_void,
                freespace,
            );
            (*context).bitcount = (*context)
                .bitcount
                .wrapping_add((freespace << 3 as ::core::ffi::c_int) as uint64_t);
            len = len.wrapping_sub(freespace);
            data = data.add(freespace);
            SHA256_Transform(context, &raw mut (*context).buffer as *mut uint8_t);
        } else {
            memcpy(
                (&raw mut (*context).buffer as *mut uint8_t).add(usedspace) as *mut uint8_t
                    as *mut ::core::ffi::c_void,
                data as *const ::core::ffi::c_void,
                len,
            );
            (*context).bitcount = (*context)
                .bitcount
                .wrapping_add((len << 3 as ::core::ffi::c_int) as uint64_t);
            freespace = 0 as size_t;
            usedspace = freespace;
            return;
        }
    }
    while len >= PG_SHA256_BLOCK_LENGTH as size_t {
        SHA256_Transform(context, data);
        (*context).bitcount = (*context)
            .bitcount
            .wrapping_add((PG_SHA256_BLOCK_LENGTH << 3 as ::core::ffi::c_int) as uint64_t);
        len = len.wrapping_sub(PG_SHA256_BLOCK_LENGTH as size_t);
        data = data.offset(PG_SHA256_BLOCK_LENGTH as isize);
    }
    if len > 0 as size_t {
        memcpy(
            &raw mut (*context).buffer as *mut uint8_t as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            len,
        );
        (*context).bitcount = (*context)
            .bitcount
            .wrapping_add((len << 3 as ::core::ffi::c_int) as uint64_t);
    }
    freespace = 0 as size_t;
    usedspace = freespace;
}
#[c2rust::src_loc = "524:1"]
unsafe extern "C" fn SHA256_Last(mut context: *mut pg_sha256_ctx) {
    let mut usedspace: ::core::ffi::c_uint = 0;
    usedspace = ((*context).bitcount >> 3 as ::core::ffi::c_int)
        .wrapping_rem(PG_SHA256_BLOCK_LENGTH as uint64_t) as ::core::ffi::c_uint;
    let mut tmp: uint64_t = (*context).bitcount;
    tmp = tmp >> 32 as ::core::ffi::c_int | tmp << 32 as ::core::ffi::c_int;
    tmp = (tmp & 0xff00ff00ff00ff00 as uint64_t) >> 8 as ::core::ffi::c_int
        | (tmp & 0xff00ff00ff00ff as uint64_t) << 8 as ::core::ffi::c_int;
    (*context).bitcount = (tmp & 0xffff0000ffff0000 as uint64_t) >> 16 as ::core::ffi::c_int
        | (tmp & 0xffff0000ffff as uint64_t) << 16 as ::core::ffi::c_int;
    if usedspace > 0 as ::core::ffi::c_uint {
        let fresh1 = usedspace;
        usedspace = usedspace.wrapping_add(1);
        (*context).buffer[fresh1 as usize] = 0x80 as uint8_t;
        if usedspace <= PG_SHA256_SHORT_BLOCK_LENGTH as ::core::ffi::c_uint {
            memset(
                (&raw mut (*context).buffer as *mut uint8_t).offset(usedspace as isize)
                    as *mut uint8_t as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (PG_SHA256_SHORT_BLOCK_LENGTH as ::core::ffi::c_uint).wrapping_sub(usedspace)
                    as size_t,
            );
        } else {
            if usedspace < PG_SHA256_BLOCK_LENGTH as ::core::ffi::c_uint {
                memset(
                    (&raw mut (*context).buffer as *mut uint8_t).offset(usedspace as isize)
                        as *mut uint8_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (PG_SHA256_BLOCK_LENGTH as ::core::ffi::c_uint).wrapping_sub(usedspace)
                        as size_t,
                );
            }
            SHA256_Transform(context, &raw mut (*context).buffer as *mut uint8_t);
            memset(
                &raw mut (*context).buffer as *mut uint8_t as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                PG_SHA256_SHORT_BLOCK_LENGTH as size_t,
            );
        }
    } else {
        memset(
            &raw mut (*context).buffer as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            PG_SHA256_SHORT_BLOCK_LENGTH as size_t,
        );
        *(&raw mut (*context).buffer as *mut uint8_t) = 0x80 as uint8_t;
    }
    *((&raw mut (*context).buffer as *mut uint8_t).offset(PG_SHA256_SHORT_BLOCK_LENGTH as isize)
        as *mut uint8_t as *mut uint64_t) = (*context).bitcount;
    SHA256_Transform(context, &raw mut (*context).buffer as *mut uint8_t);
}
#[no_mangle]
#[c2rust::src_loc = "572:1"]
pub unsafe extern "C" fn pg_sha256_final(
    mut context: *mut pg_sha256_ctx,
    mut digest: *mut uint8_t,
) {
    if !digest.is_null() {
        SHA256_Last(context);
        let mut j: ::core::ffi::c_int = 0;
        j = 0 as ::core::ffi::c_int;
        while j < 8 as ::core::ffi::c_int {
            let mut tmp: uint32_t = (*context).state[j as usize];
            tmp = tmp >> 16 as ::core::ffi::c_int | tmp << 16 as ::core::ffi::c_int;
            (*context).state[j as usize] =
                ((tmp as ::core::ffi::c_ulong & 0xff00ff00 as ::core::ffi::c_ulong)
                    >> 8 as ::core::ffi::c_int
                    | (tmp as ::core::ffi::c_ulong & 0xff00ff as ::core::ffi::c_ulong)
                        << 8 as ::core::ffi::c_int) as uint32_t;
            j += 1;
        }
        memcpy(
            digest as *mut ::core::ffi::c_void,
            &raw mut (*context).state as *mut uint32_t as *const ::core::ffi::c_void,
            PG_SHA256_DIGEST_LENGTH as size_t,
        );
    }
    memset(
        context as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<pg_sha256_ctx>() as size_t,
    );
}
#[no_mangle]
#[c2rust::src_loc = "600:1"]
pub unsafe extern "C" fn pg_sha512_init(mut context: *mut pg_sha512_ctx) {
    if context.is_null() {
        return;
    }
    memcpy(
        &raw mut (*context).state as *mut uint64_t as *mut ::core::ffi::c_void,
        &raw const sha512_initial_hash_value as *const uint64_t as *const ::core::ffi::c_void,
        PG_SHA512_DIGEST_LENGTH as size_t,
    );
    memset(
        &raw mut (*context).buffer as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        PG_SHA512_BLOCK_LENGTH as size_t,
    );
    (*context).bitcount[1 as ::core::ffi::c_int as usize] = 0 as uint64_t;
    (*context).bitcount[0 as ::core::ffi::c_int as usize] =
        (*context).bitcount[1 as ::core::ffi::c_int as usize];
}
#[c2rust::src_loc = "707:1"]
unsafe extern "C" fn SHA512_Transform(mut context: *mut pg_sha512_ctx, mut data: *const uint8_t) {
    let mut a: uint64_t = 0;
    let mut b: uint64_t = 0;
    let mut c: uint64_t = 0;
    let mut d: uint64_t = 0;
    let mut e: uint64_t = 0;
    let mut f: uint64_t = 0;
    let mut g: uint64_t = 0;
    let mut h: uint64_t = 0;
    let mut s0: uint64_t = 0;
    let mut s1: uint64_t = 0;
    let mut T1: uint64_t = 0;
    let mut T2: uint64_t = 0;
    let mut W512 = &raw mut (*context).buffer as *mut uint8_t as *mut uint64_t;
    let mut j: ::core::ffi::c_int = 0;
    a = (*context).state[0 as ::core::ffi::c_int as usize];
    b = (*context).state[1 as ::core::ffi::c_int as usize];
    c = (*context).state[2 as ::core::ffi::c_int as usize];
    d = (*context).state[3 as ::core::ffi::c_int as usize];
    e = (*context).state[4 as ::core::ffi::c_int as usize];
    f = (*context).state[5 as ::core::ffi::c_int as usize];
    g = (*context).state[6 as ::core::ffi::c_int as usize];
    h = (*context).state[7 as ::core::ffi::c_int as usize];
    j = 0 as ::core::ffi::c_int;
    loop {
        *W512.offset(j as isize) = *data.offset(7 as ::core::ffi::c_int as isize) as uint64_t
            | (*data.offset(6 as ::core::ffi::c_int as isize) as uint64_t)
                << 8 as ::core::ffi::c_int
            | (*data.offset(5 as ::core::ffi::c_int as isize) as uint64_t)
                << 16 as ::core::ffi::c_int
            | (*data.offset(4 as ::core::ffi::c_int as isize) as uint64_t)
                << 24 as ::core::ffi::c_int
            | (*data.offset(3 as ::core::ffi::c_int as isize) as uint64_t)
                << 32 as ::core::ffi::c_int
            | (*data.offset(2 as ::core::ffi::c_int as isize) as uint64_t)
                << 40 as ::core::ffi::c_int
            | (*data.offset(1 as ::core::ffi::c_int as isize) as uint64_t)
                << 48 as ::core::ffi::c_int
            | (*data.offset(0 as ::core::ffi::c_int as isize) as uint64_t)
                << 56 as ::core::ffi::c_int;
        data = data.offset(8 as ::core::ffi::c_int as isize);
        T1 = h
            .wrapping_add(
                (e >> 14 as ::core::ffi::c_int
                    | e << (64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int))
                    ^ (e >> 18 as ::core::ffi::c_int
                        | e << (64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int))
                    ^ (e >> 41 as ::core::ffi::c_int
                        | e << (64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int)),
            )
            .wrapping_add(e & f ^ !e & g)
            .wrapping_add(K512[j as usize])
            .wrapping_add(*W512.offset(j as isize));
        T2 = ((a >> 28 as ::core::ffi::c_int
            | a << (64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int))
            ^ (a >> 34 as ::core::ffi::c_int
                | a << (64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int))
            ^ (a >> 39 as ::core::ffi::c_int
                | a << (64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int)))
            .wrapping_add(a & b ^ a & c ^ b & c);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(T1);
        d = c;
        c = b;
        b = a;
        a = T1.wrapping_add(T2);
        j += 1;
        if j >= 16 as ::core::ffi::c_int {
            break;
        }
    }
    loop {
        s0 = *W512.offset(((j + 1 as ::core::ffi::c_int) & 0xf as ::core::ffi::c_int) as isize);
        s0 = (s0 >> 1 as ::core::ffi::c_int
            | s0 << (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            ^ (s0 >> 8 as ::core::ffi::c_int
                | s0 << (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int))
            ^ s0 >> 7 as ::core::ffi::c_int;
        s1 = *W512.offset(((j + 14 as ::core::ffi::c_int) & 0xf as ::core::ffi::c_int) as isize);
        s1 = (s1 >> 19 as ::core::ffi::c_int
            | s1 << (64 as ::core::ffi::c_int - 19 as ::core::ffi::c_int))
            ^ (s1 >> 61 as ::core::ffi::c_int
                | s1 << (64 as ::core::ffi::c_int - 61 as ::core::ffi::c_int))
            ^ s1 >> 6 as ::core::ffi::c_int;
        let fresh2 = &mut *W512.offset((j & 0xf as ::core::ffi::c_int) as isize);
        *fresh2 = (*fresh2).wrapping_add(
            s1.wrapping_add(
                *W512.offset(((j + 9 as ::core::ffi::c_int) & 0xf as ::core::ffi::c_int) as isize),
            )
            .wrapping_add(s0),
        );
        T1 = h
            .wrapping_add(
                (e >> 14 as ::core::ffi::c_int
                    | e << (64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int))
                    ^ (e >> 18 as ::core::ffi::c_int
                        | e << (64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int))
                    ^ (e >> 41 as ::core::ffi::c_int
                        | e << (64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int)),
            )
            .wrapping_add(e & f ^ !e & g)
            .wrapping_add(K512[j as usize])
            .wrapping_add(*fresh2);
        T2 = ((a >> 28 as ::core::ffi::c_int
            | a << (64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int))
            ^ (a >> 34 as ::core::ffi::c_int
                | a << (64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int))
            ^ (a >> 39 as ::core::ffi::c_int
                | a << (64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int)))
            .wrapping_add(a & b ^ a & c ^ b & c);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(T1);
        d = c;
        c = b;
        b = a;
        a = T1.wrapping_add(T2);
        j += 1;
        if j >= 80 as ::core::ffi::c_int {
            break;
        }
    }
    (*context).state[0 as ::core::ffi::c_int as usize] =
        (*context).state[0 as ::core::ffi::c_int as usize].wrapping_add(a);
    (*context).state[1 as ::core::ffi::c_int as usize] =
        (*context).state[1 as ::core::ffi::c_int as usize].wrapping_add(b);
    (*context).state[2 as ::core::ffi::c_int as usize] =
        (*context).state[2 as ::core::ffi::c_int as usize].wrapping_add(c);
    (*context).state[3 as ::core::ffi::c_int as usize] =
        (*context).state[3 as ::core::ffi::c_int as usize].wrapping_add(d);
    (*context).state[4 as ::core::ffi::c_int as usize] =
        (*context).state[4 as ::core::ffi::c_int as usize].wrapping_add(e);
    (*context).state[5 as ::core::ffi::c_int as usize] =
        (*context).state[5 as ::core::ffi::c_int as usize].wrapping_add(f);
    (*context).state[6 as ::core::ffi::c_int as usize] =
        (*context).state[6 as ::core::ffi::c_int as usize].wrapping_add(g);
    (*context).state[7 as ::core::ffi::c_int as usize] =
        (*context).state[7 as ::core::ffi::c_int as usize].wrapping_add(h);
    T2 = 0 as uint64_t;
    T1 = T2;
    h = T1;
    g = h;
    f = g;
    e = f;
    d = e;
    c = d;
    b = c;
    a = b;
}
#[no_mangle]
#[c2rust::src_loc = "797:1"]
pub unsafe extern "C" fn pg_sha512_update(
    mut context: *mut pg_sha512_ctx,
    mut data: *const uint8_t,
    mut len: size_t,
) {
    let mut freespace: size_t = 0;
    let mut usedspace: size_t = 0;
    if len == 0 as size_t {
        return;
    }
    usedspace = ((*context).bitcount[0 as ::core::ffi::c_int as usize] >> 3 as ::core::ffi::c_int)
        .wrapping_rem(PG_SHA512_BLOCK_LENGTH as uint64_t) as size_t;
    if usedspace > 0 as size_t {
        freespace = (PG_SHA512_BLOCK_LENGTH as size_t).wrapping_sub(usedspace);
        if len >= freespace {
            memcpy(
                (&raw mut (*context).buffer as *mut uint8_t).add(usedspace) as *mut uint8_t
                    as *mut ::core::ffi::c_void,
                data as *const ::core::ffi::c_void,
                freespace,
            );
            (*context).bitcount[0 as ::core::ffi::c_int as usize] = (*context).bitcount
                [0 as ::core::ffi::c_int as usize]
                .wrapping_add((freespace << 3 as ::core::ffi::c_int) as uint64_t);
            if (*context).bitcount[0 as ::core::ffi::c_int as usize]
                < (freespace << 3 as ::core::ffi::c_int) as uint64_t
            {
                (*context).bitcount[1 as ::core::ffi::c_int as usize] =
                    (*context).bitcount[1 as ::core::ffi::c_int as usize].wrapping_add(1);
            }
            len = len.wrapping_sub(freespace);
            data = data.add(freespace);
            SHA512_Transform(context, &raw mut (*context).buffer as *mut uint8_t);
        } else {
            memcpy(
                (&raw mut (*context).buffer as *mut uint8_t).add(usedspace) as *mut uint8_t
                    as *mut ::core::ffi::c_void,
                data as *const ::core::ffi::c_void,
                len,
            );
            (*context).bitcount[0 as ::core::ffi::c_int as usize] = (*context).bitcount
                [0 as ::core::ffi::c_int as usize]
                .wrapping_add((len << 3 as ::core::ffi::c_int) as uint64_t);
            if (*context).bitcount[0 as ::core::ffi::c_int as usize]
                < (len << 3 as ::core::ffi::c_int) as uint64_t
            {
                (*context).bitcount[1 as ::core::ffi::c_int as usize] =
                    (*context).bitcount[1 as ::core::ffi::c_int as usize].wrapping_add(1);
            }
            freespace = 0 as size_t;
            usedspace = freespace;
            return;
        }
    }
    while len >= PG_SHA512_BLOCK_LENGTH as size_t {
        SHA512_Transform(context, data);
        (*context).bitcount[0 as ::core::ffi::c_int as usize] = (*context).bitcount
            [0 as ::core::ffi::c_int as usize]
            .wrapping_add(((128 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t);
        if (*context).bitcount[0 as ::core::ffi::c_int as usize]
            < ((128 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint64_t
        {
            (*context).bitcount[1 as ::core::ffi::c_int as usize] =
                (*context).bitcount[1 as ::core::ffi::c_int as usize].wrapping_add(1);
        }
        len = len.wrapping_sub(PG_SHA512_BLOCK_LENGTH as size_t);
        data = data.offset(PG_SHA512_BLOCK_LENGTH as isize);
    }
    if len > 0 as size_t {
        memcpy(
            &raw mut (*context).buffer as *mut uint8_t as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            len,
        );
        (*context).bitcount[0 as ::core::ffi::c_int as usize] = (*context).bitcount
            [0 as ::core::ffi::c_int as usize]
            .wrapping_add((len << 3 as ::core::ffi::c_int) as uint64_t);
        if (*context).bitcount[0 as ::core::ffi::c_int as usize]
            < (len << 3 as ::core::ffi::c_int) as uint64_t
        {
            (*context).bitcount[1 as ::core::ffi::c_int as usize] =
                (*context).bitcount[1 as ::core::ffi::c_int as usize].wrapping_add(1);
        }
    }
    freespace = 0 as size_t;
    usedspace = freespace;
}
#[c2rust::src_loc = "850:1"]
unsafe extern "C" fn SHA512_Last(mut context: *mut pg_sha512_ctx) {
    let mut usedspace: ::core::ffi::c_uint = 0;
    usedspace = ((*context).bitcount[0 as ::core::ffi::c_int as usize] >> 3 as ::core::ffi::c_int)
        .wrapping_rem(PG_SHA512_BLOCK_LENGTH as uint64_t) as ::core::ffi::c_uint;
    let mut tmp: uint64_t = (*context).bitcount[0 as ::core::ffi::c_int as usize];
    tmp = tmp >> 32 as ::core::ffi::c_int | tmp << 32 as ::core::ffi::c_int;
    tmp = (tmp & 0xff00ff00ff00ff00 as uint64_t) >> 8 as ::core::ffi::c_int
        | (tmp & 0xff00ff00ff00ff as uint64_t) << 8 as ::core::ffi::c_int;
    (*context).bitcount[0 as ::core::ffi::c_int as usize] = (tmp & 0xffff0000ffff0000 as uint64_t)
        >> 16 as ::core::ffi::c_int
        | (tmp & 0xffff0000ffff as uint64_t) << 16 as ::core::ffi::c_int;
    let mut tmp_0: uint64_t = (*context).bitcount[1 as ::core::ffi::c_int as usize];
    tmp_0 = tmp_0 >> 32 as ::core::ffi::c_int | tmp_0 << 32 as ::core::ffi::c_int;
    tmp_0 = (tmp_0 & 0xff00ff00ff00ff00 as uint64_t) >> 8 as ::core::ffi::c_int
        | (tmp_0 & 0xff00ff00ff00ff as uint64_t) << 8 as ::core::ffi::c_int;
    (*context).bitcount[1 as ::core::ffi::c_int as usize] =
        (tmp_0 & 0xffff0000ffff0000 as uint64_t) >> 16 as ::core::ffi::c_int
            | (tmp_0 & 0xffff0000ffff as uint64_t) << 16 as ::core::ffi::c_int;
    if usedspace > 0 as ::core::ffi::c_uint {
        let fresh3 = usedspace;
        usedspace = usedspace.wrapping_add(1);
        (*context).buffer[fresh3 as usize] = 0x80 as uint8_t;
        if usedspace <= PG_SHA512_SHORT_BLOCK_LENGTH as ::core::ffi::c_uint {
            memset(
                (&raw mut (*context).buffer as *mut uint8_t).offset(usedspace as isize)
                    as *mut uint8_t as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (PG_SHA512_SHORT_BLOCK_LENGTH as ::core::ffi::c_uint).wrapping_sub(usedspace)
                    as size_t,
            );
        } else {
            if usedspace < PG_SHA512_BLOCK_LENGTH as ::core::ffi::c_uint {
                memset(
                    (&raw mut (*context).buffer as *mut uint8_t).offset(usedspace as isize)
                        as *mut uint8_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (PG_SHA512_BLOCK_LENGTH as ::core::ffi::c_uint).wrapping_sub(usedspace)
                        as size_t,
                );
            }
            SHA512_Transform(context, &raw mut (*context).buffer as *mut uint8_t);
            memset(
                &raw mut (*context).buffer as *mut uint8_t as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (PG_SHA512_BLOCK_LENGTH - 2 as ::core::ffi::c_int) as size_t,
            );
        }
    } else {
        memset(
            &raw mut (*context).buffer as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            PG_SHA512_SHORT_BLOCK_LENGTH as size_t,
        );
        *(&raw mut (*context).buffer as *mut uint8_t) = 0x80 as uint8_t;
    }
    *((&raw mut (*context).buffer as *mut uint8_t).offset(PG_SHA512_SHORT_BLOCK_LENGTH as isize)
        as *mut uint8_t as *mut uint64_t) = (*context).bitcount[1 as ::core::ffi::c_int as usize];
    *((&raw mut (*context).buffer as *mut uint8_t)
        .offset((PG_SHA512_SHORT_BLOCK_LENGTH + 8 as ::core::ffi::c_int) as isize)
        as *mut uint8_t as *mut uint64_t) = (*context).bitcount[0 as ::core::ffi::c_int as usize];
    SHA512_Transform(context, &raw mut (*context).buffer as *mut uint8_t);
}
#[no_mangle]
#[c2rust::src_loc = "900:1"]
pub unsafe extern "C" fn pg_sha512_final(
    mut context: *mut pg_sha512_ctx,
    mut digest: *mut uint8_t,
) {
    if !digest.is_null() {
        SHA512_Last(context);
        let mut j: ::core::ffi::c_int = 0;
        j = 0 as ::core::ffi::c_int;
        while j < 8 as ::core::ffi::c_int {
            let mut tmp: uint64_t = (*context).state[j as usize];
            tmp = tmp >> 32 as ::core::ffi::c_int | tmp << 32 as ::core::ffi::c_int;
            tmp = (tmp & 0xff00ff00ff00ff00 as uint64_t) >> 8 as ::core::ffi::c_int
                | (tmp & 0xff00ff00ff00ff as uint64_t) << 8 as ::core::ffi::c_int;
            (*context).state[j as usize] = (tmp & 0xffff0000ffff0000 as uint64_t)
                >> 16 as ::core::ffi::c_int
                | (tmp & 0xffff0000ffff as uint64_t) << 16 as ::core::ffi::c_int;
            j += 1;
        }
        memcpy(
            digest as *mut ::core::ffi::c_void,
            &raw mut (*context).state as *mut uint64_t as *const ::core::ffi::c_void,
            PG_SHA512_DIGEST_LENGTH as size_t,
        );
    }
    memset(
        context as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<pg_sha512_ctx>() as size_t,
    );
}
#[no_mangle]
#[c2rust::src_loc = "929:1"]
pub unsafe extern "C" fn pg_sha384_init(mut context: *mut pg_sha384_ctx) {
    if context.is_null() {
        return;
    }
    memcpy(
        &raw mut (*context).state as *mut uint64_t as *mut ::core::ffi::c_void,
        &raw const sha384_initial_hash_value as *const uint64_t as *const ::core::ffi::c_void,
        PG_SHA512_DIGEST_LENGTH as size_t,
    );
    memset(
        &raw mut (*context).buffer as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        PG_SHA384_BLOCK_LENGTH as size_t,
    );
    (*context).bitcount[1 as ::core::ffi::c_int as usize] = 0 as uint64_t;
    (*context).bitcount[0 as ::core::ffi::c_int as usize] =
        (*context).bitcount[1 as ::core::ffi::c_int as usize];
}
#[no_mangle]
#[c2rust::src_loc = "939:1"]
pub unsafe extern "C" fn pg_sha384_update(
    mut context: *mut pg_sha384_ctx,
    mut data: *const uint8_t,
    mut len: size_t,
) {
    pg_sha512_update(context as *mut pg_sha512_ctx, data, len);
}
#[no_mangle]
#[c2rust::src_loc = "945:1"]
pub unsafe extern "C" fn pg_sha384_final(
    mut context: *mut pg_sha384_ctx,
    mut digest: *mut uint8_t,
) {
    if !digest.is_null() {
        SHA512_Last(context as *mut pg_sha512_ctx);
        let mut j: ::core::ffi::c_int = 0;
        j = 0 as ::core::ffi::c_int;
        while j < 6 as ::core::ffi::c_int {
            let mut tmp: uint64_t = (*context).state[j as usize];
            tmp = tmp >> 32 as ::core::ffi::c_int | tmp << 32 as ::core::ffi::c_int;
            tmp = (tmp & 0xff00ff00ff00ff00 as uint64_t) >> 8 as ::core::ffi::c_int
                | (tmp & 0xff00ff00ff00ff as uint64_t) << 8 as ::core::ffi::c_int;
            (*context).state[j as usize] = (tmp & 0xffff0000ffff0000 as uint64_t)
                >> 16 as ::core::ffi::c_int
                | (tmp & 0xffff0000ffff as uint64_t) << 16 as ::core::ffi::c_int;
            j += 1;
        }
        memcpy(
            digest as *mut ::core::ffi::c_void,
            &raw mut (*context).state as *mut uint64_t as *const ::core::ffi::c_void,
            PG_SHA384_DIGEST_LENGTH as size_t,
        );
    }
    memset(
        context as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<pg_sha384_ctx>() as size_t,
    );
}
#[no_mangle]
#[c2rust::src_loc = "973:1"]
pub unsafe extern "C" fn pg_sha224_init(mut context: *mut pg_sha224_ctx) {
    if context.is_null() {
        return;
    }
    memcpy(
        &raw mut (*context).state as *mut uint32_t as *mut ::core::ffi::c_void,
        &raw const sha224_initial_hash_value as *const uint32_t as *const ::core::ffi::c_void,
        PG_SHA256_DIGEST_LENGTH as size_t,
    );
    memset(
        &raw mut (*context).buffer as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        PG_SHA256_BLOCK_LENGTH as size_t,
    );
    (*context).bitcount = 0 as uint64_t;
}
#[no_mangle]
#[c2rust::src_loc = "983:1"]
pub unsafe extern "C" fn pg_sha224_update(
    mut context: *mut pg_sha224_ctx,
    mut data: *const uint8_t,
    mut len: size_t,
) {
    pg_sha256_update(context as *mut pg_sha256_ctx, data, len);
}
#[no_mangle]
#[c2rust::src_loc = "989:1"]
pub unsafe extern "C" fn pg_sha224_final(
    mut context: *mut pg_sha224_ctx,
    mut digest: *mut uint8_t,
) {
    if !digest.is_null() {
        SHA256_Last(context as *mut pg_sha256_ctx);
        let mut j: ::core::ffi::c_int = 0;
        j = 0 as ::core::ffi::c_int;
        while j < 8 as ::core::ffi::c_int {
            let mut tmp: uint32_t = (*context).state[j as usize];
            tmp = tmp >> 16 as ::core::ffi::c_int | tmp << 16 as ::core::ffi::c_int;
            (*context).state[j as usize] =
                ((tmp as ::core::ffi::c_ulong & 0xff00ff00 as ::core::ffi::c_ulong)
                    >> 8 as ::core::ffi::c_int
                    | (tmp as ::core::ffi::c_ulong & 0xff00ff as ::core::ffi::c_ulong)
                        << 8 as ::core::ffi::c_int) as uint32_t;
            j += 1;
        }
        memcpy(
            digest as *mut ::core::ffi::c_void,
            &raw mut (*context).state as *mut uint32_t as *const ::core::ffi::c_void,
            PG_SHA224_DIGEST_LENGTH as size_t,
        );
    }
    memset(
        context as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<pg_sha224_ctx>() as size_t,
    );
}
