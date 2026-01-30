//! Cryptographic hash implementation for PostgreSQL
//!
//! Refactored to use shared types module.

use super::types::{
    free, malloc, memset, pg_cryptohash_type, pg_sha224_ctx, pg_sha256_ctx, pg_sha384_ctx,
    pg_sha512_ctx, size_t, uint8_t, usual_explicit_bzero, PG_SHA224_DIGEST_LENGTH,
    PG_SHA256_DIGEST_LENGTH, PG_SHA384_DIGEST_LENGTH, PG_SHA512_DIGEST_LENGTH,
};

extern "C" {
    pub fn pg_sha224_init(ctx: *mut pg_sha224_ctx);
    pub fn pg_sha224_update(ctx: *mut pg_sha224_ctx, input0: *const uint8_t, len: size_t);
    pub fn pg_sha224_final(ctx: *mut pg_sha224_ctx, dest: *mut uint8_t);
    pub fn pg_sha256_init(ctx: *mut pg_sha256_ctx);
    pub fn pg_sha256_update(ctx: *mut pg_sha256_ctx, input0: *const uint8_t, len: size_t);
    pub fn pg_sha256_final(ctx: *mut pg_sha256_ctx, dest: *mut uint8_t);
    pub fn pg_sha384_init(ctx: *mut pg_sha384_ctx);
    pub fn pg_sha384_update(ctx: *mut pg_sha384_ctx, _: *const uint8_t, len: size_t);
    pub fn pg_sha384_final(ctx: *mut pg_sha384_ctx, dest: *mut uint8_t);
    pub fn pg_sha512_init(ctx: *mut pg_sha512_ctx);
    pub fn pg_sha512_update(ctx: *mut pg_sha512_ctx, input0: *const uint8_t, len: size_t);
    pub fn pg_sha512_final(ctx: *mut pg_sha512_ctx, dest: *mut uint8_t);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct pg_cryptohash_ctx {
    pub type_0: pg_cryptohash_type,
    pub error: pg_cryptohash_errno,
    pub data: CryptoHashData,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union CryptoHashData {
    pub sha224: pg_sha224_ctx,
    pub sha256: pg_sha256_ctx,
    pub sha384: pg_sha384_ctx,
    pub sha512: pg_sha512_ctx,
}

pub type pg_cryptohash_errno = ::core::ffi::c_uint;
pub const PG_CRYPTOHASH_ERROR_DEST_LEN: pg_cryptohash_errno = 1;
pub const PG_CRYPTOHASH_ERROR_NONE: pg_cryptohash_errno = 0;

#[no_mangle]
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
    ctx
}

#[no_mangle]
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
    0 as ::core::ffi::c_int
}

#[no_mangle]
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
    0 as ::core::ffi::c_int
}

#[no_mangle]
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
    0 as ::core::ffi::c_int
}

#[no_mangle]
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
pub unsafe extern "C" fn pg_cryptohash_error(
    mut ctx: *mut pg_cryptohash_ctx,
) -> *const ::core::ffi::c_char {
    if ctx.is_null() {
        return c"out of memory".as_ptr();
    }
    match (*ctx).error as ::core::ffi::c_uint {
        0 => return c"success".as_ptr(),
        1 => {
            return c"destination buffer too small".as_ptr();
        }
        _ => {}
    }
    c"success".as_ptr()
}
