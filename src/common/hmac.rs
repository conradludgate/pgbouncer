//! HMAC implementation for PostgreSQL
//!
//! Refactored to use shared types module.

use super::types::{
    free, malloc, memset, pg_cryptohash_type, size_t, uint8_t, usual_explicit_bzero,
    PG_SHA224_BLOCK_LENGTH, PG_SHA224_DIGEST_LENGTH, PG_SHA256_BLOCK_LENGTH,
    PG_SHA256_DIGEST_LENGTH, PG_SHA384_BLOCK_LENGTH, PG_SHA384_DIGEST_LENGTH,
    PG_SHA512_BLOCK_LENGTH, PG_SHA512_DIGEST_LENGTH,
};

extern "C" {
    pub type pg_cryptohash_ctx;
    pub fn pg_cryptohash_create(type_0: pg_cryptohash_type) -> *mut pg_cryptohash_ctx;
    pub fn pg_cryptohash_init(ctx: *mut pg_cryptohash_ctx) -> ::core::ffi::c_int;
    pub fn pg_cryptohash_update(
        ctx: *mut pg_cryptohash_ctx,
        data: *const uint8_t,
        len: size_t,
    ) -> ::core::ffi::c_int;
    pub fn pg_cryptohash_final(
        ctx: *mut pg_cryptohash_ctx,
        dest: *mut uint8_t,
        len: size_t,
    ) -> ::core::ffi::c_int;
    pub fn pg_cryptohash_free(ctx: *mut pg_cryptohash_ctx);
    pub fn pg_cryptohash_error(ctx: *mut pg_cryptohash_ctx) -> *const ::core::ffi::c_char;
}

#[derive(Copy, Clone)]
#[repr(C)]
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

pub type pg_hmac_errno = ::core::ffi::c_uint;
pub const PG_HMAC_ERROR_INTERNAL: pg_hmac_errno = 2;
pub const PG_HMAC_ERROR_OOM: pg_hmac_errno = 1;
pub const PG_HMAC_ERROR_NONE: pg_hmac_errno = 0;

pub const HMAC_IPAD: ::core::ffi::c_int = 0x36;
pub const HMAC_OPAD: ::core::ffi::c_int = 0x5c;

#[no_mangle]
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
    ctx
}

#[no_mangle]
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
    0 as ::core::ffi::c_int
}

#[no_mangle]
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
    0 as ::core::ffi::c_int
}

#[no_mangle]
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
    0 as ::core::ffi::c_int
}

#[no_mangle]
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
pub unsafe extern "C" fn pg_hmac_error(mut ctx: *mut pg_hmac_ctx) -> *const ::core::ffi::c_char {
    if ctx.is_null() {
        return c"out of memory".as_ptr();
    }
    if !(*ctx).errreason.is_null() {
        return (*ctx).errreason;
    }
    match (*ctx).error as ::core::ffi::c_uint {
        0 => return c"success".as_ptr(),
        2 => return c"internal error".as_ptr(),
        1 => return c"out of memory".as_ptr(),
        _ => {}
    }
    c"success".as_ptr()
}
