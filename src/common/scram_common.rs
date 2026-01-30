//! SCRAM authentication common functions
//!
//! Refactored to use shared types module.

use super::types::{
    free, malloc, memcpy, pg_cryptohash_type, size_t, sprintf, strlen, uint32_t, uint8_t,
};

// External cryptohash functions
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

// External HMAC functions
extern "C" {
    pub type pg_hmac_ctx;
    pub fn pg_hmac_create(type_0: pg_cryptohash_type) -> *mut pg_hmac_ctx;
    pub fn pg_hmac_init(
        ctx: *mut pg_hmac_ctx,
        key: *const uint8_t,
        len: size_t,
    ) -> ::core::ffi::c_int;
    pub fn pg_hmac_update(
        ctx: *mut pg_hmac_ctx,
        data: *const uint8_t,
        len: size_t,
    ) -> ::core::ffi::c_int;
    pub fn pg_hmac_final(
        ctx: *mut pg_hmac_ctx,
        dest: *mut uint8_t,
        len: size_t,
    ) -> ::core::ffi::c_int;
    pub fn pg_hmac_free(ctx: *mut pg_hmac_ctx);
    pub fn pg_hmac_error(ctx: *mut pg_hmac_ctx) -> *const ::core::ffi::c_char;
}

// External base64 functions
extern "C" {
    pub fn pg_b64_encode(
        src: *const uint8_t,
        len: ::core::ffi::c_int,
        dst: *mut ::core::ffi::c_char,
        dstlen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn pg_b64_enc_len(srclen: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe extern "C" fn usual_bswap32(x: uint32_t) -> uint32_t {
    x.swap_bytes()
}

#[no_mangle]
pub unsafe extern "C" fn scram_SaltedPassword(
    mut password: *const ::core::ffi::c_char,
    mut hash_type: pg_cryptohash_type,
    mut key_length: ::core::ffi::c_int,
    mut salt: *const uint8_t,
    mut saltlen: ::core::ffi::c_int,
    mut iterations: ::core::ffi::c_int,
    mut result: *mut uint8_t,
    mut errstr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let password_len = strlen(password) as ::core::ffi::c_int;
    let mut one = usual_bswap32(1);
    let mut i: ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int;
    let mut Ui: [uint8_t; 32] = [0; 32];
    let mut Ui_prev: [uint8_t; 32] = [0; 32];
    let hmac_ctx = pg_hmac_create(hash_type);
    if hmac_ctx.is_null() {
        *errstr = pg_hmac_error(::core::ptr::null_mut::<pg_hmac_ctx>());
        return -1;
    }
    if pg_hmac_init(hmac_ctx, password as *mut uint8_t, password_len as size_t) < 0
        || pg_hmac_update(hmac_ctx, salt, saltlen as size_t) < 0
        || pg_hmac_update(
            hmac_ctx,
            &raw mut one as *mut uint8_t,
            ::core::mem::size_of::<uint32_t>() as size_t,
        ) < 0
        || pg_hmac_final(
            hmac_ctx,
            &raw mut Ui_prev as *mut uint8_t,
            key_length as size_t,
        ) < 0
    {
        *errstr = pg_hmac_error(hmac_ctx);
        pg_hmac_free(hmac_ctx);
        return -1;
    }
    memcpy(
        result as *mut ::core::ffi::c_void,
        &raw mut Ui_prev as *mut uint8_t as *const ::core::ffi::c_void,
        key_length as size_t,
    );
    i = 1;
    while i < iterations {
        if pg_hmac_init(hmac_ctx, password as *mut uint8_t, password_len as size_t) < 0
            || pg_hmac_update(
                hmac_ctx,
                &raw mut Ui_prev as *mut uint8_t,
                key_length as size_t,
            ) < 0
            || pg_hmac_final(hmac_ctx, &raw mut Ui as *mut uint8_t, key_length as size_t) < 0
        {
            *errstr = pg_hmac_error(hmac_ctx);
            pg_hmac_free(hmac_ctx);
            return -1;
        }
        j = 0;
        while j < key_length {
            let fresh0 = &mut *result.offset(j as isize);
            *fresh0 =
                (*fresh0 as ::core::ffi::c_int ^ Ui[j as usize] as ::core::ffi::c_int) as uint8_t;
            j += 1;
        }
        memcpy(
            &raw mut Ui_prev as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw mut Ui as *mut uint8_t as *const ::core::ffi::c_void,
            key_length as size_t,
        );
        i += 1;
    }
    pg_hmac_free(hmac_ctx);
    0
}

#[no_mangle]
pub unsafe extern "C" fn scram_H(
    mut input: *const uint8_t,
    mut hash_type: pg_cryptohash_type,
    mut key_length: ::core::ffi::c_int,
    mut result: *mut uint8_t,
    mut errstr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let ctx = pg_cryptohash_create(hash_type);
    if ctx.is_null() {
        *errstr = pg_cryptohash_error(::core::ptr::null_mut::<pg_cryptohash_ctx>());
        return -1;
    }
    if pg_cryptohash_init(ctx) < 0
        || pg_cryptohash_update(ctx, input, key_length as size_t) < 0
        || pg_cryptohash_final(ctx, result, key_length as size_t) < 0
    {
        *errstr = pg_cryptohash_error(ctx);
        pg_cryptohash_free(ctx);
        return -1;
    }
    pg_cryptohash_free(ctx);
    0
}

#[no_mangle]
pub unsafe extern "C" fn scram_ClientKey(
    mut salted_password: *const uint8_t,
    mut hash_type: pg_cryptohash_type,
    mut key_length: ::core::ffi::c_int,
    mut result: *mut uint8_t,
    mut errstr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let ctx = pg_hmac_create(hash_type);
    if ctx.is_null() {
        *errstr = pg_hmac_error(::core::ptr::null_mut::<pg_hmac_ctx>());
        return -1;
    }
    if pg_hmac_init(ctx, salted_password, key_length as size_t) < 0
        || pg_hmac_update(
            ctx,
            b"Client Key\0" as *const u8 as *mut uint8_t,
            strlen(c"Client Key".as_ptr()),
        ) < 0
        || pg_hmac_final(ctx, result, key_length as size_t) < 0
    {
        *errstr = pg_hmac_error(ctx);
        pg_hmac_free(ctx);
        return -1;
    }
    pg_hmac_free(ctx);
    0
}

#[no_mangle]
pub unsafe extern "C" fn scram_ServerKey(
    mut salted_password: *const uint8_t,
    mut hash_type: pg_cryptohash_type,
    mut key_length: ::core::ffi::c_int,
    mut result: *mut uint8_t,
    mut errstr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let ctx = pg_hmac_create(hash_type);
    if ctx.is_null() {
        *errstr = pg_hmac_error(::core::ptr::null_mut::<pg_hmac_ctx>());
        return -1;
    }
    if pg_hmac_init(ctx, salted_password, key_length as size_t) < 0
        || pg_hmac_update(
            ctx,
            b"Server Key\0" as *const u8 as *mut uint8_t,
            strlen(c"Server Key".as_ptr()),
        ) < 0
        || pg_hmac_final(ctx, result, key_length as size_t) < 0
    {
        *errstr = pg_hmac_error(ctx);
        pg_hmac_free(ctx);
        return -1;
    }
    pg_hmac_free(ctx);
    0
}

#[no_mangle]
pub unsafe extern "C" fn scram_build_secret(
    mut hash_type: pg_cryptohash_type,
    mut key_length: ::core::ffi::c_int,
    mut salt: *const uint8_t,
    mut saltlen: ::core::ffi::c_int,
    mut iterations: ::core::ffi::c_int,
    mut password: *const ::core::ffi::c_char,
    mut errstr: *mut *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut salted_password: [uint8_t; 32] = [0; 32];
    let mut stored_key: [uint8_t; 32] = [0; 32];
    let mut server_key: [uint8_t; 32] = [0; 32];
    let mut result: *mut ::core::ffi::c_char;
    let mut p: *mut ::core::ffi::c_char;
    let maxlen: ::core::ffi::c_int;
    let encoded_salt_len: ::core::ffi::c_int;
    let encoded_stored_len: ::core::ffi::c_int;
    let encoded_server_len: ::core::ffi::c_int;
    let mut encoded_result: ::core::ffi::c_int;

    if scram_SaltedPassword(
        password,
        hash_type,
        key_length,
        salt,
        saltlen,
        iterations,
        &raw mut salted_password as *mut uint8_t,
        errstr,
    ) < 0
        || scram_ClientKey(
            &raw mut salted_password as *mut uint8_t,
            hash_type,
            key_length,
            &raw mut stored_key as *mut uint8_t,
            errstr,
        ) < 0
        || scram_H(
            &raw mut stored_key as *mut uint8_t,
            hash_type,
            key_length,
            &raw mut stored_key as *mut uint8_t,
            errstr,
        ) < 0
        || scram_ServerKey(
            &raw mut salted_password as *mut uint8_t,
            hash_type,
            key_length,
            &raw mut server_key as *mut uint8_t,
            errstr,
        ) < 0
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }

    encoded_salt_len = pg_b64_enc_len(saltlen);
    encoded_stored_len = pg_b64_enc_len(key_length);
    encoded_server_len = pg_b64_enc_len(key_length);
    maxlen = strlen(c"SCRAM-SHA-256".as_ptr())
        .wrapping_add(1)
        .wrapping_add(10)
        .wrapping_add(1)
        .wrapping_add(encoded_salt_len as size_t)
        .wrapping_add(1)
        .wrapping_add(encoded_stored_len as size_t)
        .wrapping_add(1)
        .wrapping_add(encoded_server_len as size_t)
        .wrapping_add(1) as ::core::ffi::c_int;

    result = malloc(maxlen as size_t) as *mut ::core::ffi::c_char;
    if result.is_null() {
        *errstr = c"out of memory".as_ptr();
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }

    p = result.offset(sprintf(result, c"SCRAM-SHA-256$%d:".as_ptr(), iterations) as isize);

    encoded_result = pg_b64_encode(salt, saltlen, p, encoded_salt_len);
    if encoded_result < 0 {
        *errstr = c"could not encode salt".as_ptr();
        free(result as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    p = p.offset(encoded_result as isize);
    let fresh1 = p;
    p = p.offset(1);
    *fresh1 = '$' as ::core::ffi::c_char;

    encoded_result = pg_b64_encode(
        &raw mut stored_key as *mut uint8_t,
        key_length,
        p,
        encoded_stored_len,
    );
    if encoded_result < 0 {
        *errstr = c"could not encode stored key".as_ptr();
        free(result as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    p = p.offset(encoded_result as isize);
    let fresh2 = p;
    p = p.offset(1);
    *fresh2 = ':' as ::core::ffi::c_char;

    encoded_result = pg_b64_encode(
        &raw mut server_key as *mut uint8_t,
        key_length,
        p,
        encoded_server_len,
    );
    if encoded_result < 0 {
        *errstr = c"could not encode server key".as_ptr();
        free(result as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    p = p.offset(encoded_result as isize);
    *p = '\0' as ::core::ffi::c_char;

    result
}
