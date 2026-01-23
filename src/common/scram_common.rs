#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:16"]
pub mod _types_h {
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:16"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:16"]
pub mod _uint8_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint8_t = u8;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:16"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = u32;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/cryptohash.h:19"]
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
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/hmac.h:19"]
pub mod hmac_h {
    use super::_size_t_h::size_t;
    use super::_uint8_t_h::uint8_t;
    use super::cryptohash_h::pg_cryptohash_type;
    extern "C" {
        #[c2rust::src_loc = "21:9"]
        pub type pg_hmac_ctx;
        #[c2rust::src_loc = "23:1"]
        pub fn pg_hmac_create(type_0: pg_cryptohash_type) -> *mut pg_hmac_ctx;
        #[c2rust::src_loc = "24:1"]
        pub fn pg_hmac_init(
            ctx: *mut pg_hmac_ctx,
            key: *const uint8_t,
            len: size_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "25:1"]
        pub fn pg_hmac_update(
            ctx: *mut pg_hmac_ctx,
            data: *const uint8_t,
            len: size_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "26:1"]
        pub fn pg_hmac_final(
            ctx: *mut pg_hmac_ctx,
            dest: *mut uint8_t,
            len: size_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "27:1"]
        pub fn pg_hmac_free(ctx: *mut pg_hmac_ctx);
        #[c2rust::src_loc = "28:1"]
        pub fn pg_hmac_error(ctx: *mut pg_hmac_ctx) -> *const ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:16"]
pub mod _malloc_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "54:1"]
        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "56:1"]
        pub fn free(_: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:16"]
pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "77:1"]
        pub fn memcpy(
            __dst: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "96:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/endian.h:16"]
pub mod endian_h {
    #[inline]
    #[c2rust::src_loc = "144:1"]
    pub unsafe extern "C" fn usual_bswap32(mut x: uint32_t) -> uint32_t {
        return x.swap_bytes();
    }
    use super::_uint32_t_h::uint32_t;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/base64.h:18"]
pub mod base64_h {
    use super::_uint8_t_h::uint8_t;
    extern "C" {
        #[c2rust::src_loc = "14:1"]
        pub fn pg_b64_encode(
            src: *const uint8_t,
            len: ::core::ffi::c_int,
            dst: *mut ::core::ffi::c_char,
            dstlen: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "16:1"]
        pub fn pg_b64_enc_len(srclen: ::core::ffi::c_int) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:20"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:16"]
pub mod sys__types_h {
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:16"]
pub mod _stdio_h {
    extern "C" {
        #[c2rust::src_loc = "275:1"]
        pub fn sprintf(
            _: *mut ::core::ffi::c_char,
            _: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}
use self::_malloc_h::{free, malloc};
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
use self::_stdio_h::sprintf;
use self::_string_h::{memcpy, strlen};
pub use self::_types_h::__darwin_size_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint8_t_h::uint8_t;
use self::base64_h::{pg_b64_enc_len, pg_b64_encode};
pub use self::cryptohash_h::{
    pg_cryptohash_create, pg_cryptohash_ctx, pg_cryptohash_error, pg_cryptohash_final,
    pg_cryptohash_free, pg_cryptohash_init, pg_cryptohash_type, pg_cryptohash_update, PG_SHA224,
    PG_SHA256, PG_SHA384, PG_SHA512,
};
pub use self::endian_h::usual_bswap32;
use self::hmac_h::{
    pg_hmac_create, pg_hmac_ctx, pg_hmac_error, pg_hmac_final, pg_hmac_free, pg_hmac_init,
    pg_hmac_update,
};
pub use self::sys__types_h::__DARWIN_NULL;
#[no_mangle]
#[c2rust::src_loc = "29:1"]
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
    let mut password_len = strlen(password) as ::core::ffi::c_int;
    let mut one = usual_bswap32(1 as uint32_t);
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut Ui: [uint8_t; 32] = [0; 32];
    let mut Ui_prev: [uint8_t; 32] = [0; 32];
    let mut hmac_ctx = pg_hmac_create(hash_type);
    if hmac_ctx.is_null() {
        *errstr = pg_hmac_error(::core::ptr::null_mut::<pg_hmac_ctx>());
        return -(1 as ::core::ffi::c_int);
    }
    if pg_hmac_init(hmac_ctx, password as *mut uint8_t, password_len as size_t)
        < 0 as ::core::ffi::c_int
        || pg_hmac_update(hmac_ctx, salt, saltlen as size_t) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            hmac_ctx,
            &raw mut one as *mut uint8_t,
            ::core::mem::size_of::<uint32_t>() as size_t,
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_final(
            hmac_ctx,
            &raw mut Ui_prev as *mut uint8_t,
            key_length as size_t,
        ) < 0 as ::core::ffi::c_int
    {
        *errstr = pg_hmac_error(hmac_ctx);
        pg_hmac_free(hmac_ctx);
        return -(1 as ::core::ffi::c_int);
    }
    memcpy(
        result as *mut ::core::ffi::c_void,
        &raw mut Ui_prev as *mut uint8_t as *const ::core::ffi::c_void,
        key_length as size_t,
    );
    i = 1 as ::core::ffi::c_int;
    while i < iterations {
        if pg_hmac_init(hmac_ctx, password as *mut uint8_t, password_len as size_t)
            < 0 as ::core::ffi::c_int
            || pg_hmac_update(
                hmac_ctx,
                &raw mut Ui_prev as *mut uint8_t,
                key_length as size_t,
            ) < 0 as ::core::ffi::c_int
            || pg_hmac_final(hmac_ctx, &raw mut Ui as *mut uint8_t, key_length as size_t)
                < 0 as ::core::ffi::c_int
        {
            *errstr = pg_hmac_error(hmac_ctx);
            pg_hmac_free(hmac_ctx);
            return -(1 as ::core::ffi::c_int);
        }
        j = 0 as ::core::ffi::c_int;
        while j < key_length {
            let ref mut fresh0 = *result.offset(j as isize);
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
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "103:1"]
pub unsafe extern "C" fn scram_H(
    mut input: *const uint8_t,
    mut hash_type: pg_cryptohash_type,
    mut key_length: ::core::ffi::c_int,
    mut result: *mut uint8_t,
    mut errstr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ctx = ::core::ptr::null_mut::<pg_cryptohash_ctx>();
    ctx = pg_cryptohash_create(hash_type);
    if ctx.is_null() {
        *errstr = pg_cryptohash_error(::core::ptr::null_mut::<pg_cryptohash_ctx>());
        return -(1 as ::core::ffi::c_int);
    }
    if pg_cryptohash_init(ctx) < 0 as ::core::ffi::c_int
        || pg_cryptohash_update(ctx, input, key_length as size_t) < 0 as ::core::ffi::c_int
        || pg_cryptohash_final(ctx, result, key_length as size_t) < 0 as ::core::ffi::c_int
    {
        *errstr = pg_cryptohash_error(ctx);
        pg_cryptohash_free(ctx);
        return -(1 as ::core::ffi::c_int);
    }
    pg_cryptohash_free(ctx);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "133:1"]
pub unsafe extern "C" fn scram_ClientKey(
    mut salted_password: *const uint8_t,
    mut hash_type: pg_cryptohash_type,
    mut key_length: ::core::ffi::c_int,
    mut result: *mut uint8_t,
    mut errstr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ctx = pg_hmac_create(hash_type);
    if ctx.is_null() {
        *errstr = pg_hmac_error(::core::ptr::null_mut::<pg_hmac_ctx>());
        return -(1 as ::core::ffi::c_int);
    }
    if pg_hmac_init(ctx, salted_password, key_length as size_t) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            b"Client Key\0" as *const u8 as *const ::core::ffi::c_char as *mut uint8_t,
            strlen(b"Client Key\0" as *const u8 as *const ::core::ffi::c_char),
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_final(ctx, result, key_length as size_t) < 0 as ::core::ffi::c_int
    {
        *errstr = pg_hmac_error(ctx);
        pg_hmac_free(ctx);
        return -(1 as ::core::ffi::c_int);
    }
    pg_hmac_free(ctx);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "163:1"]
pub unsafe extern "C" fn scram_ServerKey(
    mut salted_password: *const uint8_t,
    mut hash_type: pg_cryptohash_type,
    mut key_length: ::core::ffi::c_int,
    mut result: *mut uint8_t,
    mut errstr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ctx = pg_hmac_create(hash_type);
    if ctx.is_null() {
        *errstr = pg_hmac_error(::core::ptr::null_mut::<pg_hmac_ctx>());
        return -(1 as ::core::ffi::c_int);
    }
    if pg_hmac_init(ctx, salted_password, key_length as size_t) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            b"Server Key\0" as *const u8 as *const ::core::ffi::c_char as *mut uint8_t,
            strlen(b"Server Key\0" as *const u8 as *const ::core::ffi::c_char),
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_final(ctx, result, key_length as size_t) < 0 as ::core::ffi::c_int
    {
        *errstr = pg_hmac_error(ctx);
        pg_hmac_free(ctx);
        return -(1 as ::core::ffi::c_int);
    }
    pg_hmac_free(ctx);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "200:1"]
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
    let mut result = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut maxlen: ::core::ffi::c_int = 0;
    let mut encoded_salt_len: ::core::ffi::c_int = 0;
    let mut encoded_stored_len: ::core::ffi::c_int = 0;
    let mut encoded_server_len: ::core::ffi::c_int = 0;
    let mut encoded_result: ::core::ffi::c_int = 0;
    if scram_SaltedPassword(
        password,
        hash_type,
        key_length,
        salt,
        saltlen,
        iterations,
        &raw mut salted_password as *mut uint8_t,
        errstr,
    ) < 0 as ::core::ffi::c_int
        || scram_ClientKey(
            &raw mut salted_password as *mut uint8_t,
            hash_type,
            key_length,
            &raw mut stored_key as *mut uint8_t,
            errstr,
        ) < 0 as ::core::ffi::c_int
        || scram_H(
            &raw mut stored_key as *mut uint8_t,
            hash_type,
            key_length,
            &raw mut stored_key as *mut uint8_t,
            errstr,
        ) < 0 as ::core::ffi::c_int
        || scram_ServerKey(
            &raw mut salted_password as *mut uint8_t,
            hash_type,
            key_length,
            &raw mut server_key as *mut uint8_t,
            errstr,
        ) < 0 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    encoded_salt_len = pg_b64_enc_len(saltlen);
    encoded_stored_len = pg_b64_enc_len(key_length);
    encoded_server_len = pg_b64_enc_len(key_length);
    maxlen = strlen(b"SCRAM-SHA-256\0" as *const u8 as *const ::core::ffi::c_char)
        .wrapping_add(1 as size_t)
        .wrapping_add(10 as size_t)
        .wrapping_add(1 as size_t)
        .wrapping_add(encoded_salt_len as size_t)
        .wrapping_add(1 as size_t)
        .wrapping_add(encoded_stored_len as size_t)
        .wrapping_add(1 as size_t)
        .wrapping_add(encoded_server_len as size_t)
        .wrapping_add(1 as size_t) as ::core::ffi::c_int;
    result = malloc(maxlen as size_t) as *mut ::core::ffi::c_char;
    if result.is_null() {
        *errstr = b"out of memory\0" as *const u8 as *const ::core::ffi::c_char;
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    p = result.offset(sprintf(
        result,
        b"SCRAM-SHA-256$%d:\0" as *const u8 as *const ::core::ffi::c_char,
        iterations,
    ) as isize);
    encoded_result = pg_b64_encode(salt, saltlen, p, encoded_salt_len);
    if encoded_result < 0 as ::core::ffi::c_int {
        *errstr = b"could not encode salt\0" as *const u8 as *const ::core::ffi::c_char;
        free(result as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    p = p.offset(encoded_result as isize);
    let fresh1 = p;
    p = p.offset(1);
    *fresh1 = '$' as i32 as ::core::ffi::c_char;
    encoded_result = pg_b64_encode(
        &raw mut stored_key as *mut uint8_t,
        key_length,
        p,
        encoded_stored_len,
    );
    if encoded_result < 0 as ::core::ffi::c_int {
        *errstr = b"could not encode stored key\0" as *const u8 as *const ::core::ffi::c_char;
        free(result as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    p = p.offset(encoded_result as isize);
    let fresh2 = p;
    p = p.offset(1);
    *fresh2 = ':' as i32 as ::core::ffi::c_char;
    encoded_result = pg_b64_encode(
        &raw mut server_key as *mut uint8_t,
        key_length,
        p,
        encoded_server_len,
    );
    if encoded_result < 0 as ::core::ffi::c_int {
        *errstr = b"could not encode server key\0" as *const u8 as *const ::core::ffi::c_char;
        free(result as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    p = p.offset(encoded_result as isize);
    let fresh3 = p;
    p = p.offset(1);
    *fresh3 = '\0' as i32 as ::core::ffi::c_char;
    return result;
}
