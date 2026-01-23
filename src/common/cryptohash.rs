#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:18"]
pub mod _types_h {
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:18"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:18"]
pub mod _uint8_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint8_t = u8;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:18"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = u32;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint64_t.h:18"]
pub mod _uint64_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint64_t = u64;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/cryptohash.h:22"]
pub mod cryptohash_h {
    #[c2rust::src_loc = "19:9"]
    pub type pg_cryptohash_type = ::core::ffi::c_uint;
    #[c2rust::src_loc = "24:2"]
    pub const PG_SHA512: pg_cryptohash_type = 3;
    #[c2rust::src_loc = "23:2"]
    pub const PG_SHA384: pg_cryptohash_type = 2;
    #[c2rust::src_loc = "22:2"]
    pub const PG_SHA256: pg_cryptohash_type = 1;
    #[c2rust::src_loc = "21:2"]
    pub const PG_SHA224: pg_cryptohash_type = 0;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/sha2_int.h:23"]
pub mod sha2_int_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "61:9"]
    pub struct pg_sha512_ctx {
        pub state: [uint64_t; 8],
        pub bitcount: [uint64_t; 2],
        pub buffer: [uint8_t; 128],
    }
    #[c2rust::src_loc = "68:1"]
    pub type pg_sha384_ctx = pg_sha512_ctx;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:9"]
    pub struct pg_sha256_ctx {
        pub state: [uint32_t; 8],
        pub bitcount: uint64_t,
        pub buffer: [uint8_t; 64],
    }
    #[c2rust::src_loc = "67:1"]
    pub type pg_sha224_ctx = pg_sha256_ctx;
    use super::_size_t_h::size_t;
    use super::_uint32_t_h::uint32_t;
    use super::_uint64_t_h::uint64_t;
    use super::_uint8_t_h::uint8_t;
    extern "C" {
        #[c2rust::src_loc = "71:1"]
        pub fn pg_sha224_init(ctx: *mut pg_sha224_ctx);
        #[c2rust::src_loc = "72:1"]
        pub fn pg_sha224_update(ctx: *mut pg_sha224_ctx, input0: *const uint8_t, len: size_t);
        #[c2rust::src_loc = "74:1"]
        pub fn pg_sha224_final(ctx: *mut pg_sha224_ctx, dest: *mut uint8_t);
        #[c2rust::src_loc = "76:1"]
        pub fn pg_sha256_init(ctx: *mut pg_sha256_ctx);
        #[c2rust::src_loc = "77:1"]
        pub fn pg_sha256_update(ctx: *mut pg_sha256_ctx, input0: *const uint8_t, len: size_t);
        #[c2rust::src_loc = "79:1"]
        pub fn pg_sha256_final(ctx: *mut pg_sha256_ctx, dest: *mut uint8_t);
        #[c2rust::src_loc = "81:1"]
        pub fn pg_sha384_init(ctx: *mut pg_sha384_ctx);
        #[c2rust::src_loc = "82:1"]
        pub fn pg_sha384_update(ctx: *mut pg_sha384_ctx, _: *const uint8_t, len: size_t);
        #[c2rust::src_loc = "84:1"]
        pub fn pg_sha384_final(ctx: *mut pg_sha384_ctx, dest: *mut uint8_t);
        #[c2rust::src_loc = "86:1"]
        pub fn pg_sha512_init(ctx: *mut pg_sha512_ctx);
        #[c2rust::src_loc = "87:1"]
        pub fn pg_sha512_update(ctx: *mut pg_sha512_ctx, input0: *const uint8_t, len: size_t);
        #[c2rust::src_loc = "89:1"]
        pub fn pg_sha512_final(ctx: *mut pg_sha512_ctx, dest: *mut uint8_t);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:18"]
pub mod _malloc_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "54:1"]
        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "56:1"]
        pub fn free(_: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:18"]
pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "83:1"]
        pub fn memset(
            __b: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __len: size_t,
        ) -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/string.h:18"]
pub mod string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "153:1"]
        pub fn usual_explicit_bzero(buf: *mut ::core::ffi::c_void, len: size_t);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:18"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:18"]
pub mod sys__types_h {
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/sha2.h:23"]
pub mod sha2_h {
    #[c2rust::src_loc = "20:9"]
    pub const PG_SHA224_DIGEST_LENGTH: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
    #[c2rust::src_loc = "23:9"]
    pub const PG_SHA256_DIGEST_LENGTH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const PG_SHA384_DIGEST_LENGTH: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
    #[c2rust::src_loc = "29:9"]
    pub const PG_SHA512_DIGEST_LENGTH: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
}
use self::_malloc_h::{free, malloc};
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
use self::_string_h::memset;
pub use self::_types_h::__darwin_size_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::cryptohash_h::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};
pub use self::sha2_h::{
    PG_SHA224_DIGEST_LENGTH, PG_SHA256_DIGEST_LENGTH, PG_SHA384_DIGEST_LENGTH,
    PG_SHA512_DIGEST_LENGTH,
};
pub use self::sha2_int_h::{
    pg_sha224_ctx, pg_sha224_final, pg_sha224_init, pg_sha224_update, pg_sha256_ctx,
    pg_sha256_final, pg_sha256_init, pg_sha256_update, pg_sha384_ctx, pg_sha384_final,
    pg_sha384_init, pg_sha384_update, pg_sha512_ctx, pg_sha512_final, pg_sha512_init,
    pg_sha512_update,
};
use self::string_h::usual_explicit_bzero;
pub use self::sys__types_h::__DARWIN_NULL;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "45:1"]
pub struct pg_cryptohash_ctx {
    pub type_0: pg_cryptohash_type,
    pub error: pg_cryptohash_errno,
    pub data: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "50:2"]
pub union C2RustUnnamed {
    pub sha224: pg_sha224_ctx,
    pub sha256: pg_sha256_ctx,
    pub sha384: pg_sha384_ctx,
    pub sha512: pg_sha512_ctx,
}
#[c2rust::src_loc = "38:9"]
pub type pg_cryptohash_errno = ::core::ffi::c_uint;
#[c2rust::src_loc = "41:2"]
pub const PG_CRYPTOHASH_ERROR_DEST_LEN: pg_cryptohash_errno = 1;
#[c2rust::src_loc = "40:2"]
pub const PG_CRYPTOHASH_ERROR_NONE: pg_cryptohash_errno = 0;
#[no_mangle]
#[c2rust::src_loc = "65:1"]
pub unsafe extern "C" fn pg_cryptohash_create(
    mut type_0: pg_cryptohash_type,
) -> *mut pg_cryptohash_ctx {
    let mut ctx = ::core::ptr::null_mut::<pg_cryptohash_ctx>();
    ctx = malloc(::core::mem::size_of::<pg_cryptohash_ctx>() as size_t) as *mut pg_cryptohash_ctx;
    if ctx.is_null() {
        return ::core::ptr::null_mut::<pg_cryptohash_ctx>();
    }
    memset(
        ctx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<pg_cryptohash_ctx>() as size_t,
    );
    (*ctx).type_0 = type_0;
    (*ctx).error = PG_CRYPTOHASH_ERROR_NONE;
    return ctx;
}
#[no_mangle]
#[c2rust::src_loc = "91:1"]
pub unsafe extern "C" fn pg_cryptohash_init(mut ctx: *mut pg_cryptohash_ctx) -> ::core::ffi::c_int {
    if ctx.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    match (*ctx).type_0 as ::core::ffi::c_uint {
        0 => {
            pg_sha224_init(&raw mut (*ctx).data.sha224);
        }
        1 => {
            pg_sha256_init(&raw mut (*ctx).data.sha256);
        }
        2 => {
            pg_sha384_init(&raw mut (*ctx).data.sha384);
        }
        3 => {
            pg_sha512_init(&raw mut (*ctx).data.sha512);
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "121:1"]
pub unsafe extern "C" fn pg_cryptohash_update(
    mut ctx: *mut pg_cryptohash_ctx,
    mut data: *const uint8_t,
    mut len: size_t,
) -> ::core::ffi::c_int {
    if ctx.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    match (*ctx).type_0 as ::core::ffi::c_uint {
        0 => {
            pg_sha224_update(&raw mut (*ctx).data.sha224, data, len);
        }
        1 => {
            pg_sha256_update(&raw mut (*ctx).data.sha256, data, len);
        }
        2 => {
            pg_sha384_update(&raw mut (*ctx).data.sha384, data, len);
        }
        3 => {
            pg_sha512_update(&raw mut (*ctx).data.sha512, data, len);
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "151:1"]
pub unsafe extern "C" fn pg_cryptohash_final(
    mut ctx: *mut pg_cryptohash_ctx,
    mut dest: *mut uint8_t,
    mut len: size_t,
) -> ::core::ffi::c_int {
    if ctx.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    match (*ctx).type_0 as ::core::ffi::c_uint {
        0 => {
            if len < PG_SHA224_DIGEST_LENGTH as size_t {
                (*ctx).error = PG_CRYPTOHASH_ERROR_DEST_LEN;
                return -(1 as ::core::ffi::c_int);
            }
            pg_sha224_final(&raw mut (*ctx).data.sha224, dest);
        }
        1 => {
            if len < PG_SHA256_DIGEST_LENGTH as size_t {
                (*ctx).error = PG_CRYPTOHASH_ERROR_DEST_LEN;
                return -(1 as ::core::ffi::c_int);
            }
            pg_sha256_final(&raw mut (*ctx).data.sha256, dest);
        }
        2 => {
            if len < PG_SHA384_DIGEST_LENGTH as size_t {
                (*ctx).error = PG_CRYPTOHASH_ERROR_DEST_LEN;
                return -(1 as ::core::ffi::c_int);
            }
            pg_sha384_final(&raw mut (*ctx).data.sha384, dest);
        }
        3 => {
            if len < PG_SHA512_DIGEST_LENGTH as size_t {
                (*ctx).error = PG_CRYPTOHASH_ERROR_DEST_LEN;
                return -(1 as ::core::ffi::c_int);
            }
            pg_sha512_final(&raw mut (*ctx).data.sha512, dest);
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "201:1"]
pub unsafe extern "C" fn pg_cryptohash_free(mut ctx: *mut pg_cryptohash_ctx) {
    if ctx.is_null() {
        return;
    }
    usual_explicit_bzero(
        ctx as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<pg_cryptohash_ctx>() as size_t,
    );
    free(ctx as *mut ::core::ffi::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "217:1"]
pub unsafe extern "C" fn pg_cryptohash_error(
    mut ctx: *mut pg_cryptohash_ctx,
) -> *const ::core::ffi::c_char {
    if ctx.is_null() {
        return b"out of memory\0" as *const u8 as *const ::core::ffi::c_char;
    }
    match (*ctx).error as ::core::ffi::c_uint {
        0 => return b"success\0" as *const u8 as *const ::core::ffi::c_char,
        1 => {
            return b"destination buffer too small\0" as *const u8 as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return b"success\0" as *const u8 as *const ::core::ffi::c_char;
}
