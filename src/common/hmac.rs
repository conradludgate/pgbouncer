#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:17"]
pub mod _types_h {
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:17"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:17"]
pub mod _uint8_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint8_t = u8;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/cryptohash.h:23"]
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
    use super::_size_t_h::size_t;
    use super::_uint8_t_h::uint8_t;
    extern "C" {
        #[c2rust::src_loc = "28:9"]
        pub type pg_cryptohash_ctx;
        #[c2rust::src_loc = "30:1"]
        pub fn pg_cryptohash_create(type_0: pg_cryptohash_type) -> *mut pg_cryptohash_ctx;
        #[c2rust::src_loc = "31:1"]
        pub fn pg_cryptohash_init(ctx: *mut pg_cryptohash_ctx) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "32:1"]
        pub fn pg_cryptohash_update(
            ctx: *mut pg_cryptohash_ctx,
            data: *const uint8_t,
            len: size_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "33:1"]
        pub fn pg_cryptohash_final(
            ctx: *mut pg_cryptohash_ctx,
            dest: *mut uint8_t,
            len: size_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "34:1"]
        pub fn pg_cryptohash_free(ctx: *mut pg_cryptohash_ctx);
        #[c2rust::src_loc = "35:1"]
        pub fn pg_cryptohash_error(ctx: *mut pg_cryptohash_ctx) -> *const ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:17"]
pub mod _malloc_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "54:1"]
        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "56:1"]
        pub fn free(_: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:17"]
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
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/string.h:17"]
pub mod string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "153:1"]
        pub fn usual_explicit_bzero(buf: *mut ::core::ffi::c_void, len: size_t);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:17"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:17"]
pub mod sys__types_h {
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/sha2.h:25"]
pub mod sha2_h {
    #[c2rust::src_loc = "19:9"]
    pub const PG_SHA224_BLOCK_LENGTH: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
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
use self::_malloc_h::{free, malloc};
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
use self::_string_h::memset;
pub use self::_types_h::__darwin_size_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::cryptohash_h::{
    pg_cryptohash_create, pg_cryptohash_ctx, pg_cryptohash_error, pg_cryptohash_final,
    pg_cryptohash_free, pg_cryptohash_init, pg_cryptohash_type, pg_cryptohash_update, PG_SHA224,
    PG_SHA256, PG_SHA384, PG_SHA512,
};
pub use self::sha2_h::{
    PG_SHA224_BLOCK_LENGTH, PG_SHA224_DIGEST_LENGTH, PG_SHA256_BLOCK_LENGTH,
    PG_SHA256_DIGEST_LENGTH, PG_SHA384_BLOCK_LENGTH, PG_SHA384_DIGEST_LENGTH,
    PG_SHA512_BLOCK_LENGTH, PG_SHA512_DIGEST_LENGTH,
};
use self::string_h::usual_explicit_bzero;
pub use self::sys__types_h::__DARWIN_NULL;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "48:1"]
pub struct pg_hmac_ctx {
    pub hash: *mut pg_cryptohash_ctx,
    pub type_0: pg_cryptohash_type,
    pub error: pg_hmac_errno,
    pub errreason: *const ::core::ffi::c_char,
    pub block_size: ::core::ffi::c_int,
    pub digest_size: ::core::ffi::c_int,
    pub k_ipad: [uint8_t; 128],
    pub k_opad: [uint8_t; 128],
}
#[c2rust::src_loc = "40:9"]
pub type pg_hmac_errno = ::core::ffi::c_uint;
#[c2rust::src_loc = "44:2"]
pub const PG_HMAC_ERROR_INTERNAL: pg_hmac_errno = 2;
#[c2rust::src_loc = "43:2"]
pub const PG_HMAC_ERROR_OOM: pg_hmac_errno = 1;
#[c2rust::src_loc = "42:2"]
pub const PG_HMAC_ERROR_NONE: pg_hmac_errno = 0;
#[c2rust::src_loc = "65:9"]
pub const HMAC_IPAD: ::core::ffi::c_int = 0x36 as ::core::ffi::c_int;
#[c2rust::src_loc = "66:9"]
pub const HMAC_OPAD: ::core::ffi::c_int = 0x5c as ::core::ffi::c_int;
#[no_mangle]
#[c2rust::src_loc = "74:1"]
pub unsafe extern "C" fn pg_hmac_create(mut type_0: pg_cryptohash_type) -> *mut pg_hmac_ctx {
    let mut ctx = ::core::ptr::null_mut::<pg_hmac_ctx>();
    ctx = malloc(::core::mem::size_of::<pg_hmac_ctx>() as size_t) as *mut pg_hmac_ctx;
    if ctx.is_null() {
        return ::core::ptr::null_mut::<pg_hmac_ctx>();
    }
    memset(
        ctx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<pg_hmac_ctx>() as size_t,
    );
    (*ctx).type_0 = type_0;
    (*ctx).error = PG_HMAC_ERROR_NONE;
    (*ctx).errreason = ::core::ptr::null::<::core::ffi::c_char>();
    match type_0 as ::core::ffi::c_uint {
        0 => {
            (*ctx).digest_size = PG_SHA224_DIGEST_LENGTH;
            (*ctx).block_size = PG_SHA224_BLOCK_LENGTH;
        }
        1 => {
            (*ctx).digest_size = PG_SHA256_DIGEST_LENGTH;
            (*ctx).block_size = PG_SHA256_BLOCK_LENGTH;
        }
        2 => {
            (*ctx).digest_size = PG_SHA384_DIGEST_LENGTH;
            (*ctx).block_size = PG_SHA384_BLOCK_LENGTH;
        }
        3 => {
            (*ctx).digest_size = PG_SHA512_DIGEST_LENGTH;
            (*ctx).block_size = PG_SHA512_BLOCK_LENGTH;
        }
        _ => {}
    }
    (*ctx).hash = pg_cryptohash_create(type_0);
    if (*ctx).hash.is_null() {
        usual_explicit_bzero(
            ctx as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<pg_hmac_ctx>() as size_t,
        );
        free(ctx as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<pg_hmac_ctx>();
    }
    return ctx;
}
#[no_mangle]
#[c2rust::src_loc = "127:1"]
pub unsafe extern "C" fn pg_hmac_init(
    mut ctx: *mut pg_hmac_ctx,
    mut key: *const uint8_t,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut digest_size: ::core::ffi::c_int = 0;
    let mut block_size: ::core::ffi::c_int = 0;
    let mut shrinkbuf = ::core::ptr::null_mut::<uint8_t>();
    if ctx.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    digest_size = (*ctx).digest_size;
    block_size = (*ctx).block_size;
    memset(
        &raw mut (*ctx).k_opad as *mut uint8_t as *mut ::core::ffi::c_void,
        HMAC_OPAD,
        (*ctx).block_size as size_t,
    );
    memset(
        &raw mut (*ctx).k_ipad as *mut uint8_t as *mut ::core::ffi::c_void,
        HMAC_IPAD,
        (*ctx).block_size as size_t,
    );
    if len > block_size as size_t {
        let mut hash_ctx = ::core::ptr::null_mut::<pg_cryptohash_ctx>();
        shrinkbuf = malloc(digest_size as size_t) as *mut uint8_t;
        if shrinkbuf.is_null() {
            (*ctx).error = PG_HMAC_ERROR_OOM;
            return -(1 as ::core::ffi::c_int);
        }
        memset(
            shrinkbuf as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            digest_size as size_t,
        );
        hash_ctx = pg_cryptohash_create((*ctx).type_0);
        if hash_ctx.is_null() {
            (*ctx).error = PG_HMAC_ERROR_OOM;
            free(shrinkbuf as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
        if pg_cryptohash_init(hash_ctx) < 0 as ::core::ffi::c_int
            || pg_cryptohash_update(hash_ctx, key, len) < 0 as ::core::ffi::c_int
            || pg_cryptohash_final(hash_ctx, shrinkbuf, digest_size as size_t)
                < 0 as ::core::ffi::c_int
        {
            (*ctx).error = PG_HMAC_ERROR_INTERNAL;
            (*ctx).errreason = pg_cryptohash_error(hash_ctx);
            pg_cryptohash_free(hash_ctx);
            free(shrinkbuf as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
        key = shrinkbuf;
        len = digest_size as size_t;
        pg_cryptohash_free(hash_ctx);
    }
    i = 0 as ::core::ffi::c_int;
    while (i as size_t) < len {
        (*ctx).k_ipad[i as usize] = ((*ctx).k_ipad[i as usize] as ::core::ffi::c_int
            ^ *key.offset(i as isize) as ::core::ffi::c_int)
            as uint8_t;
        (*ctx).k_opad[i as usize] = ((*ctx).k_opad[i as usize] as ::core::ffi::c_int
            ^ *key.offset(i as isize) as ::core::ffi::c_int)
            as uint8_t;
        i += 1;
    }
    if pg_cryptohash_init((*ctx).hash) < 0 as ::core::ffi::c_int
        || pg_cryptohash_update(
            (*ctx).hash,
            &raw mut (*ctx).k_ipad as *mut uint8_t,
            (*ctx).block_size as size_t,
        ) < 0 as ::core::ffi::c_int
    {
        (*ctx).error = PG_HMAC_ERROR_INTERNAL;
        (*ctx).errreason = pg_cryptohash_error((*ctx).hash);
        if !shrinkbuf.is_null() {
            free(shrinkbuf as *mut ::core::ffi::c_void);
        }
        return -(1 as ::core::ffi::c_int);
    }
    if !shrinkbuf.is_null() {
        free(shrinkbuf as *mut ::core::ffi::c_void);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "212:1"]
pub unsafe extern "C" fn pg_hmac_update(
    mut ctx: *mut pg_hmac_ctx,
    mut data: *const uint8_t,
    mut len: size_t,
) -> ::core::ffi::c_int {
    if ctx.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if pg_cryptohash_update((*ctx).hash, data, len) < 0 as ::core::ffi::c_int {
        (*ctx).error = PG_HMAC_ERROR_INTERNAL;
        (*ctx).errreason = pg_cryptohash_error((*ctx).hash);
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "233:1"]
pub unsafe extern "C" fn pg_hmac_final(
    mut ctx: *mut pg_hmac_ctx,
    mut dest: *mut uint8_t,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut h = ::core::ptr::null_mut::<uint8_t>();
    if ctx.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    h = malloc((*ctx).digest_size as size_t) as *mut uint8_t;
    if h.is_null() {
        (*ctx).error = PG_HMAC_ERROR_OOM;
        return -(1 as ::core::ffi::c_int);
    }
    memset(
        h as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (*ctx).digest_size as size_t,
    );
    if pg_cryptohash_final((*ctx).hash, h, (*ctx).digest_size as size_t) < 0 as ::core::ffi::c_int {
        (*ctx).error = PG_HMAC_ERROR_INTERNAL;
        (*ctx).errreason = pg_cryptohash_error((*ctx).hash);
        free(h as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    if pg_cryptohash_init((*ctx).hash) < 0 as ::core::ffi::c_int
        || pg_cryptohash_update(
            (*ctx).hash,
            &raw mut (*ctx).k_opad as *mut uint8_t,
            (*ctx).block_size as size_t,
        ) < 0 as ::core::ffi::c_int
        || pg_cryptohash_update((*ctx).hash, h, (*ctx).digest_size as size_t)
            < 0 as ::core::ffi::c_int
        || pg_cryptohash_final((*ctx).hash, dest, len) < 0 as ::core::ffi::c_int
    {
        (*ctx).error = PG_HMAC_ERROR_INTERNAL;
        (*ctx).errreason = pg_cryptohash_error((*ctx).hash);
        free(h as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    free(h as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "278:1"]
pub unsafe extern "C" fn pg_hmac_free(mut ctx: *mut pg_hmac_ctx) {
    if ctx.is_null() {
        return;
    }
    pg_cryptohash_free((*ctx).hash);
    usual_explicit_bzero(
        ctx as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<pg_hmac_ctx>() as size_t,
    );
    free(ctx as *mut ::core::ffi::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "295:1"]
pub unsafe extern "C" fn pg_hmac_error(mut ctx: *mut pg_hmac_ctx) -> *const ::core::ffi::c_char {
    if ctx.is_null() {
        return b"out of memory\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if !(*ctx).errreason.is_null() {
        return (*ctx).errreason;
    }
    match (*ctx).error as ::core::ffi::c_uint {
        0 => return b"success\0" as *const u8 as *const ::core::ffi::c_char,
        2 => return b"internal error\0" as *const u8 as *const ::core::ffi::c_char,
        1 => return b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
        _ => {}
    }
    return b"success\0" as *const u8 as *const ::core::ffi::c_char;
}
