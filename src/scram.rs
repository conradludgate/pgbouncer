pub mod bouncer_h {
    pub use crate::types::*;
    pub use c2rust_bitfields::BitfieldStruct;
    extern "C" {}
}

pub mod sbuf_h {

    pub use crate::types::*;

    pub use crate::types::*;

    pub use crate::types::*;






}

pub mod iobuf_h {

    pub use crate::types::*;

    pub use crate::types::*;

    pub use crate::types::*;






}

pub mod pktbuf_h {

    pub use crate::types::*;

    pub use crate::types::*;

    pub use crate::types::*;






}

pub mod dnslookup_h {

    pub use crate::types::*;

    pub use crate::types::*;

    pub use crate::types::*;
    extern "C" {
    }






}

pub mod scram_h {

    pub use crate::types::*;
    extern "C" {

        pub static mut cf_scram_iterations: ::core::ffi::c_int;
    }


}

pub mod hmac_h {
    use crate::types::pg_cryptohash_type;
    use crate::types::size_t;
    use crate::types::uint8_t;
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
}

pub mod saslprep_h {

    pub const SASLPREP_SUCCESS: pg_saslprep_rc = 0;

    pub type pg_saslprep_rc = ::core::ffi::c_int;

    pub const SASLPREP_PROHIBITED: pg_saslprep_rc = -3;

    pub const SASLPREP_INVALID_UTF8: pg_saslprep_rc = -2;

    pub const SASLPREP_OOM: pg_saslprep_rc = -1;
    extern "C" {

        pub fn pg_saslprep(
            input: *const ::core::ffi::c_char,
            output: *mut *mut ::core::ffi::c_char,
        ) -> pg_saslprep_rc;
    }
}

pub mod util_h {

    pub const MD5_PASSWD_LEN: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
    use crate::types::uint8_t;
    extern "C" {

        pub fn get_random_bytes(dest: *mut uint8_t, len: ::core::ffi::c_int);
    }
}

pub mod scram_common_h {

    pub const SCRAM_SHA_256_KEY_LEN: ::core::ffi::c_int = PG_SHA256_DIGEST_LENGTH;

    pub const SCRAM_RAW_NONCE_LEN: ::core::ffi::c_int = 18 as ::core::ffi::c_int;

    pub const SCRAM_DEFAULT_SALT_LEN: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    use super::sha2_h::PG_SHA256_DIGEST_LENGTH;
    use crate::types::pg_cryptohash_type;
    use crate::types::uint8_t;
    extern "C" {

        pub fn scram_SaltedPassword(
            password: *const ::core::ffi::c_char,
            hash_type: pg_cryptohash_type,
            key_length: ::core::ffi::c_int,
            salt: *const uint8_t,
            saltlen: ::core::ffi::c_int,
            iterations: ::core::ffi::c_int,
            result: *mut uint8_t,
            errstr: *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;

        pub fn scram_H(
            input: *const uint8_t,
            hash_type: pg_cryptohash_type,
            key_length: ::core::ffi::c_int,
            result: *mut uint8_t,
            errstr: *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;

        pub fn scram_ClientKey(
            salted_password: *const uint8_t,
            hash_type: pg_cryptohash_type,
            key_length: ::core::ffi::c_int,
            result: *mut uint8_t,
            errstr: *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;

        pub fn scram_ServerKey(
            salted_password: *const uint8_t,
            hash_type: pg_cryptohash_type,
            key_length: ::core::ffi::c_int,
            result: *mut uint8_t,
            errstr: *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}

pub mod base64_h {
    use crate::types::uint8_t;
    extern "C" {

        pub fn pg_b64_encode(
            src: *const uint8_t,
            len: ::core::ffi::c_int,
            dst: *mut ::core::ffi::c_char,
            dstlen: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;

        pub fn pg_b64_decode(
            src: *const ::core::ffi::c_char,
            len: ::core::ffi::c_int,
            dst: *mut uint8_t,
            dstlen: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;

        pub fn pg_b64_enc_len(srclen: ::core::ffi::c_int) -> ::core::ffi::c_int;

        pub fn pg_b64_dec_len(srclen: ::core::ffi::c_int) -> ::core::ffi::c_int;
    }
}

pub mod limits_h {

    pub use crate::types::*;


}

pub mod sha2_h {

    pub const PG_SHA256_DIGEST_LENGTH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
}
use crate::types::{free, malloc};
use crate::types::snprintf;
use crate::types::strtol;
use crate::types::{
    memcmp, memcpy, memset, strcmp, strdup, strlcat, strlen, strncmp, strspn, strtok,
};
use self::base64_h::{pg_b64_dec_len, pg_b64_decode, pg_b64_enc_len, pg_b64_encode};
pub use self::bouncer_h::{
    sockaddr_ucreds, C2RustUnnamed_9, CallbackState, LoadBalanceHosts, PacketCallbackFlag, PgAddr,
    PgCredentials, PgDatabase, PgGlobalUser, PgPool, PgSocket, PgStats, ReplicationType,
    ScramState, SocketState, CB_HANDLE_COMPLETE_PACKET, CB_NONE, CB_WANT_COMPLETE_PACKET,
    CL_ACTIVE, CL_ACTIVE_CANCEL, CL_FREE, CL_JUSTFREE, CL_LOGIN, CL_WAITING, CL_WAITING_CANCEL,
    CL_WAITING_LOGIN, LOAD_BALANCE_HOSTS_DISABLE, LOAD_BALANCE_HOSTS_ROUND_ROBIN,
    REPLICATION_LOGICAL, REPLICATION_NONE, REPLICATION_PHYSICAL, SV_ACTIVE, SV_ACTIVE_CANCEL,
    SV_BEING_CANCELED, SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN, SV_TESTED, SV_USED,
};
pub use crate::types::in_addr_t;
pub use crate::types::in_port_t;
pub use crate::types::pid_t;
pub use crate::types::ptrdiff_t;
pub use crate::types::sa_family_t;
pub use crate::types::size_t;
pub use crate::types::ssize_t;
pub use crate::types::timeval;
pub use crate::types::uid_t;
pub use crate::types::uint16_t;
pub use crate::types::uint32_t;
pub use crate::types::uint64_t;
pub use crate::types::uint8_t;
pub use crate::types::uintptr_t;
pub use crate::types::NULL;
pub use crate::types::{
    __darwin_ptrdiff_t, __darwin_size_t, __darwin_ssize_t, __darwin_time_t, __int32_t, __uint16_t,
    __uint32_t, __uint8_t,
};
pub use crate::types::{aatree_cmp_f, aatree_walker_f, AANode, AATree};
pub use crate::types::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};

use crate::types::__error;

pub use crate::types::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
use self::hmac_h::{
    pg_hmac_create, pg_hmac_ctx, pg_hmac_error, pg_hmac_final, pg_hmac_free, pg_hmac_init,
    pg_hmac_update,
};
pub use crate::types::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use crate::types::{in_addr, sockaddr_in};
pub use self::iobuf_h::{iobuf, IOBuf};
pub use self::limits_h::INT_MAX;
pub use crate::types::{
    log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE, LG_STATS, LG_WARNING,
};
pub use self::pktbuf_h::PktBuf;
pub use self::saslprep_h::{
    pg_saslprep, pg_saslprep_rc, SASLPREP_INVALID_UTF8, SASLPREP_OOM, SASLPREP_PROHIBITED,
    SASLPREP_SUCCESS,
};
pub use self::sbuf_h::{
    sbuf_cb_t, SBuf, SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK, SBUF_EV_FLUSH,
    SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY,
};
pub use self::scram_common_h::{
    scram_ClientKey, scram_H, scram_SaltedPassword, scram_ServerKey, SCRAM_DEFAULT_SALT_LEN,
    SCRAM_RAW_NONCE_LEN, SCRAM_SHA_256_KEY_LEN,
};
pub use self::scram_h::{
    cf_scram_iterations, PasswordType, PASSWORD_TYPE_MD5, PASSWORD_TYPE_PLAINTEXT,
    PASSWORD_TYPE_SCRAM_SHA_256,
};
pub use self::sha2_h::PG_SHA256_DIGEST_LENGTH;
pub use crate::types::sockaddr;
pub use crate::types::{__darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t, __DARWIN_NULL};
pub use crate::types::usec_t;
pub use crate::types::List;
pub use crate::types::MBuf;
pub use crate::types::PktHdr;
pub use crate::types::StatList;
pub use crate::types::{false_0, true_0};
pub use crate::types::{PStr, StrPool};
pub use crate::types::{PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement};

pub use self::util_h::{get_random_bytes, MD5_PASSWD_LEN};
pub use crate::types::VarCache;
pub use crate::types::{UT_hash_bucket, UT_hash_handle, UT_hash_table};
#[no_mangle]

pub unsafe extern "C" fn free_scram_state(mut state: *mut ScramState) {
    free((*state).client_nonce as *mut ::core::ffi::c_void);
    free((*state).client_first_message_bare as *mut ::core::ffi::c_void);
    free((*state).client_final_message_without_proof as *mut ::core::ffi::c_void);
    free((*state).server_nonce as *mut ::core::ffi::c_void);
    free((*state).server_first_message as *mut ::core::ffi::c_void);
    free((*state).SaltedPassword as *mut ::core::ffi::c_void);
    free((*state).salt as *mut ::core::ffi::c_void);
    free((*state).encoded_salt as *mut ::core::ffi::c_void);
    memset(
        state as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<ScramState>() as size_t,
    );
}

unsafe extern "C" fn is_scram_printable(mut p: *mut ::core::ffi::c_char) -> bool {
    while *p != 0 {
        if (*p as ::core::ffi::c_int) < 0x21 as ::core::ffi::c_int
            || *p as ::core::ffi::c_int > 0x7e as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == 0x2c as ::core::ffi::c_int
        {
            return false;
        }
        p = p.offset(1);
    }
    true
}

unsafe extern "C" fn sanitize_char(mut c: ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    static mut buf: [::core::ffi::c_char; 5] = [0; 5];
    if c as ::core::ffi::c_int >= 0x21 as ::core::ffi::c_int
        && c as ::core::ffi::c_int <= 0x7e as ::core::ffi::c_int
    {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t,
            c"'%c'".as_ptr(),
            c as ::core::ffi::c_int,
        );
    } else {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t,
            c"0x%02x".as_ptr(),
            c as ::core::ffi::c_uchar as ::core::ffi::c_int,
        );
    }
    &raw mut buf as *mut ::core::ffi::c_char
}

unsafe extern "C" fn read_attr_value(
    mut sk: *mut PgSocket,
    mut input: *mut *mut ::core::ffi::c_char,
    mut attr: ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut begin = *input;
    let mut end = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if *begin as ::core::ffi::c_int != attr as ::core::ffi::c_int {
        log_generic(
            LG_ERROR,
            sk as *mut ::core::ffi::c_void,
            b"malformed SCRAM message (attribute \"%c\" expected)\0" as *const u8
                as *const ::core::ffi::c_char,
            attr as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    begin = begin.offset(1);
    if *begin as ::core::ffi::c_int != '=' as i32 {
        log_generic(
            LG_ERROR,
            sk as *mut ::core::ffi::c_void,
            b"malformed SCRAM message (expected \"=\" after attribute \"%c\")\0" as *const u8
                as *const ::core::ffi::c_char,
            attr as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    begin = begin.offset(1);
    end = begin;
    while *end as ::core::ffi::c_int != 0 && *end as ::core::ffi::c_int != ',' as i32 {
        end = end.offset(1);
    }
    if *end != 0 {
        *end = '\0' as i32 as ::core::ffi::c_char;
        *input = end.offset(1 as ::core::ffi::c_int as isize);
    } else {
        *input = end;
    }
    begin
}

unsafe extern "C" fn read_any_attr(
    mut sk: *mut PgSocket,
    mut input: *mut *mut ::core::ffi::c_char,
    mut attr_p: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut begin = *input;
    let mut end = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut attr = *begin;
    if !(attr as ::core::ffi::c_int >= 'A' as i32 && attr as ::core::ffi::c_int <= 'Z' as i32
        || attr as ::core::ffi::c_int >= 'a' as i32 && attr as ::core::ffi::c_int <= 'z' as i32)
    {
        log_generic(
            LG_ERROR,
            sk as *mut ::core::ffi::c_void,
            b"malformed SCRAM message (attribute expected, but found invalid character \"%s\")\0"
                as *const u8 as *const ::core::ffi::c_char,
            sanitize_char(attr),
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !attr_p.is_null() {
        *attr_p = attr;
    }
    begin = begin.offset(1);
    if *begin as ::core::ffi::c_int != '=' as i32 {
        log_generic(
            LG_ERROR,
            sk as *mut ::core::ffi::c_void,
            b"malformed SCRAM message (expected character \"=\" after attribute \"%c\")\0"
                as *const u8 as *const ::core::ffi::c_char,
            attr as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    begin = begin.offset(1);
    end = begin;
    while *end as ::core::ffi::c_int != 0 && *end as ::core::ffi::c_int != ',' as i32 {
        end = end.offset(1);
    }
    if *end != 0 {
        *end = '\0' as i32 as ::core::ffi::c_char;
        *input = end.offset(1 as ::core::ffi::c_int as isize);
    } else {
        *input = end;
    }
    begin
}

unsafe extern "C" fn parse_scram_secret(
    mut secret: *const ::core::ffi::c_char,
    mut iterations: *mut ::core::ffi::c_int,
    mut salt: *mut *mut ::core::ffi::c_char,
    mut stored_key: *mut uint8_t,
    mut server_key: *mut uint8_t,
) -> bool {
    let mut s = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut scheme_str = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut salt_str = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut iterations_str = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut storedkey_str = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut serverkey_str = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut decoded_len: ::core::ffi::c_int = 0;
    let mut decoded_salt_buf = ::core::ptr::null_mut::<uint8_t>();
    let mut decoded_stored_buf = ::core::ptr::null_mut::<uint8_t>();
    let mut decoded_server_buf = ::core::ptr::null_mut::<uint8_t>();
    s = strdup(secret);
    if !s.is_null() {
        scheme_str = strtok(s, c"$".as_ptr());
        if !scheme_str.is_null() {
            iterations_str = strtok(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                c":".as_ptr(),
            );
            if !iterations_str.is_null() {
                salt_str = strtok(
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    c"$".as_ptr(),
                );
                if !salt_str.is_null() {
                    storedkey_str = strtok(
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        c":".as_ptr(),
                    );
                    if !storedkey_str.is_null() {
                        serverkey_str =
                            strtok(::core::ptr::null_mut::<::core::ffi::c_char>(), c"".as_ptr());
                        if !serverkey_str.is_null()
                            && (strcmp(scheme_str, c"SCRAM-SHA-256".as_ptr())
                                == 0 as ::core::ffi::c_int)
                        {
                            *__error() = 0 as ::core::ffi::c_int;
                            *iterations =
                                strtol(iterations_str, &raw mut p, 10 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int;
                            if !(*p as ::core::ffi::c_int != 0
                                || *__error() != 0 as ::core::ffi::c_int)
                            {
                                decoded_len =
                                    pg_b64_dec_len(strlen(salt_str) as ::core::ffi::c_int);
                                decoded_salt_buf = malloc(decoded_len as size_t) as *mut uint8_t;
                                if !decoded_salt_buf.is_null() {
                                    decoded_len = pg_b64_decode(
                                        salt_str,
                                        strlen(salt_str) as ::core::ffi::c_int,
                                        decoded_salt_buf,
                                        decoded_len,
                                    );
                                    free(decoded_salt_buf as *mut ::core::ffi::c_void);
                                    if decoded_len >= 0 as ::core::ffi::c_int {
                                        *salt = strdup(salt_str);
                                        if !(*salt).is_null() {
                                            decoded_len = pg_b64_dec_len(
                                                strlen(storedkey_str) as ::core::ffi::c_int
                                            );
                                            decoded_stored_buf =
                                                malloc(decoded_len as size_t) as *mut uint8_t;
                                            if !decoded_stored_buf.is_null() {
                                                decoded_len = pg_b64_decode(
                                                    storedkey_str,
                                                    strlen(storedkey_str) as ::core::ffi::c_int,
                                                    decoded_stored_buf,
                                                    decoded_len,
                                                );
                                                if decoded_len == SCRAM_SHA_256_KEY_LEN {
                                                    memcpy(
                                                        stored_key as *mut ::core::ffi::c_void,
                                                        decoded_stored_buf
                                                            as *const ::core::ffi::c_void,
                                                        SCRAM_SHA_256_KEY_LEN as size_t,
                                                    );
                                                    decoded_len =
                                                        pg_b64_dec_len(strlen(serverkey_str)
                                                            as ::core::ffi::c_int);
                                                    decoded_server_buf =
                                                        malloc(decoded_len as size_t)
                                                            as *mut uint8_t;
                                                    if !decoded_server_buf.is_null() {
                                                        decoded_len = pg_b64_decode(
                                                            serverkey_str,
                                                            strlen(serverkey_str)
                                                                as ::core::ffi::c_int,
                                                            decoded_server_buf,
                                                            decoded_len,
                                                        );
                                                        if decoded_len == SCRAM_SHA_256_KEY_LEN {
                                                            memcpy(
                                                                server_key
                                                                    as *mut ::core::ffi::c_void,
                                                                decoded_server_buf
                                                                    as *const ::core::ffi::c_void,
                                                                SCRAM_SHA_256_KEY_LEN as size_t,
                                                            );
                                                            free(
                                                                decoded_stored_buf
                                                                    as *mut ::core::ffi::c_void,
                                                            );
                                                            free(
                                                                decoded_server_buf
                                                                    as *mut ::core::ffi::c_void,
                                                            );
                                                            free(s as *mut ::core::ffi::c_void);
                                                            return true;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free(decoded_stored_buf as *mut ::core::ffi::c_void);
    free(decoded_server_buf as *mut ::core::ffi::c_void);
    free(s as *mut ::core::ffi::c_void);
    free(*salt as *mut ::core::ffi::c_void);
    *salt = ::core::ptr::null_mut::<::core::ffi::c_char>();
    false
}

pub const MD5_PASSWD_CHARSET: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"0123456789abcdef\0")
};
#[no_mangle]

pub unsafe extern "C" fn get_password_type(
    mut shadow_pass: *const ::core::ffi::c_char,
) -> PasswordType {
    let mut encoded_salt = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut iterations: ::core::ffi::c_int = 0;
    let mut stored_key: [uint8_t; 32] = [0; 32];
    let mut server_key: [uint8_t; 32] = [0; 32];
    if strncmp(shadow_pass, c"md5".as_ptr(), 3 as size_t) == 0 as ::core::ffi::c_int
        && strlen(shadow_pass) == MD5_PASSWD_LEN as size_t
        && strspn(
            shadow_pass.offset(3 as ::core::ffi::c_int as isize),
            MD5_PASSWD_CHARSET.as_ptr(),
        ) == (MD5_PASSWD_LEN - 3 as ::core::ffi::c_int) as ::core::ffi::c_ulong
    {
        return PASSWORD_TYPE_MD5;
    }
    if parse_scram_secret(
        shadow_pass,
        &raw mut iterations,
        &raw mut encoded_salt,
        &raw mut stored_key as *mut uint8_t,
        &raw mut server_key as *mut uint8_t,
    ) {
        free(encoded_salt as *mut ::core::ffi::c_void);
        return PASSWORD_TYPE_SCRAM_SHA_256;
    }
    free(encoded_salt as *mut ::core::ffi::c_void);
    PASSWORD_TYPE_PLAINTEXT
}
#[no_mangle]

pub unsafe extern "C" fn build_client_first_message(
    mut state: *mut ScramState,
) -> *mut ::core::ffi::c_char {
    let mut raw_nonce: [uint8_t; 19] = [0; 19];
    let mut encoded_len: ::core::ffi::c_int = 0;
    let mut len: size_t = 0;
    let mut result = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*state).hash_type = PG_SHA256;
    (*state).key_length = SCRAM_SHA_256_KEY_LEN;
    get_random_bytes(&raw mut raw_nonce as *mut uint8_t, SCRAM_RAW_NONCE_LEN);
    encoded_len = pg_b64_enc_len(SCRAM_RAW_NONCE_LEN);
    (*state).client_nonce =
        malloc((encoded_len + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
    if !(*state).client_nonce.is_null() {
        encoded_len = pg_b64_encode(
            &raw mut raw_nonce as *mut uint8_t,
            SCRAM_RAW_NONCE_LEN,
            (*state).client_nonce,
            encoded_len,
        );
        if encoded_len >= 0 as ::core::ffi::c_int {
            *(*state).client_nonce.offset(encoded_len as isize) =
                '\0' as i32 as ::core::ffi::c_char;
            len = (8 as size_t)
                .wrapping_add(strlen((*state).client_nonce))
                .wrapping_add(1 as size_t);
            result = malloc(len) as *mut ::core::ffi::c_char;
            if !result.is_null() {
                snprintf(result, len, c"n,,n=,r=%s".as_ptr(), (*state).client_nonce);
                (*state).client_first_message_bare =
                    strdup(result.offset(3 as ::core::ffi::c_int as isize));
                if !(*state).client_first_message_bare.is_null() {
                    return result;
                }
            }
        }
    }
    free(result as *mut ::core::ffi::c_void);
    free((*state).client_nonce as *mut ::core::ffi::c_void);
    free((*state).client_first_message_bare as *mut ::core::ffi::c_void);
    ::core::ptr::null_mut::<::core::ffi::c_char>()
}
#[no_mangle]

pub unsafe extern "C" fn build_client_final_message(
    mut server: *mut PgSocket,
    mut credentials: *const PgCredentials,
) -> *mut ::core::ffi::c_char {
    let mut state: *mut ScramState = &raw mut (*server).scram_state;
    let mut buf: [::core::ffi::c_char; 512] = [0; 512];
    let mut len: size_t = 0;
    let mut client_proof: [uint8_t; 32] = [0; 32];
    let mut enclen: ::core::ffi::c_int = 0;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
        c"c=biws,r=%s".as_ptr(),
        (*state).server_nonce,
    );
    (*state).client_final_message_without_proof = strdup(&raw mut buf as *mut ::core::ffi::c_char);
    if !(*state).client_final_message_without_proof.is_null()
        && calculate_client_proof(
            server,
            credentials,
            &raw mut buf as *mut ::core::ffi::c_char,
            &raw mut client_proof as *mut uint8_t,
        )
    {
        len = strlcat(
            &raw mut buf as *mut ::core::ffi::c_char,
            c",p=".as_ptr(),
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
        ) as size_t;
        enclen = pg_b64_enc_len(::core::mem::size_of::<[uint8_t; 32]>() as ::core::ffi::c_int);
        enclen = pg_b64_encode(
            &raw mut client_proof as *mut uint8_t,
            SCRAM_SHA_256_KEY_LEN,
            (&raw mut buf as *mut ::core::ffi::c_char).add(len),
            enclen,
        );
        if enclen >= 0 as ::core::ffi::c_int {
            len = len.wrapping_add(enclen as size_t);
            buf[len as usize] = '\0' as i32 as ::core::ffi::c_char;
            return strdup(&raw mut buf as *mut ::core::ffi::c_char);
        }
    }
    ::core::ptr::null_mut::<::core::ffi::c_char>()
}
#[no_mangle]

pub unsafe extern "C" fn read_server_first_message(
    mut server: *mut PgSocket,
    mut input: *mut ::core::ffi::c_char,
) -> bool {
    let mut state: *mut ScramState = &raw mut (*server).scram_state;
    let mut server_nonce = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut encoded_salt = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut decoded_salt_len: ::core::ffi::c_int = 0;
    let mut iterations_str = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut endptr = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut iterations: ::core::ffi::c_int = 0;
    (*state).server_first_message = strdup(input);
    if (*state).server_first_message.is_null() {
        return false;
    }
    server_nonce = read_attr_value(server, &raw mut input, 'r' as i32 as ::core::ffi::c_char);
    if server_nonce.is_null() {
        return false;
    }
    if strlen(server_nonce) < strlen((*state).client_nonce)
        || memcmp(
            server_nonce as *const ::core::ffi::c_void,
            (*state).client_nonce as *const ::core::ffi::c_void,
            strlen((*state).client_nonce),
        ) != 0 as ::core::ffi::c_int
    {
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            c"invalid SCRAM response (nonce mismatch)".as_ptr(),
        );
        return false;
    }
    (*state).server_nonce = strdup(server_nonce);
    if (*state).server_nonce.is_null() {
        return false;
    }
    encoded_salt = read_attr_value(server, &raw mut input, 's' as i32 as ::core::ffi::c_char);
    if encoded_salt.is_null() {
        return false;
    }
    decoded_salt_len = pg_b64_dec_len(strlen(encoded_salt) as ::core::ffi::c_int);
    (*state).salt = malloc(decoded_salt_len as size_t) as *mut uint8_t;
    if (*state).salt.is_null() {
        return false;
    }
    (*state).saltlen = pg_b64_decode(
        encoded_salt,
        strlen(encoded_salt) as ::core::ffi::c_int,
        (*state).salt,
        decoded_salt_len,
    );
    if (*state).saltlen < 0 as ::core::ffi::c_int {
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            c"malformed SCRAM message (invalid salt)".as_ptr(),
        );
        return false;
    }
    iterations_str = read_attr_value(server, &raw mut input, 'i' as i32 as ::core::ffi::c_char);
    if iterations_str.is_null() {
        return false;
    }
    iterations =
        strtol(iterations_str, &raw mut endptr, 10 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if *endptr as ::core::ffi::c_int != '\0' as i32 || iterations < 1 as ::core::ffi::c_int {
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            c"malformed SCRAM message (invalid iteration count)".as_ptr(),
        );
        return false;
    }
    (*state).iterations = iterations;
    if *input as ::core::ffi::c_int != '\0' as i32 {
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            c"malformed SCRAM message (garbage at end of server-first-message)".as_ptr(),
        );
        return false;
    }
    true
}
#[no_mangle]

pub unsafe extern "C" fn read_server_final_message(
    mut server: *mut PgSocket,
    mut input: *mut ::core::ffi::c_char,
    mut ServerSignature: *mut ::core::ffi::c_char,
) -> bool {
    let mut encoded_server_signature = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut decoded_server_signature = ::core::ptr::null_mut::<uint8_t>();
    let mut server_signature_len: ::core::ffi::c_int = 0;
    if *input as ::core::ffi::c_int == 'e' as i32 {
        let mut errmsg = read_attr_value(server, &raw mut input, 'e' as i32 as ::core::ffi::c_char);
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            c"error received from server in SCRAM exchange: %s".as_ptr(),
            errmsg,
        );
    } else {
        encoded_server_signature =
            read_attr_value(server, &raw mut input, 'v' as i32 as ::core::ffi::c_char);
        if !encoded_server_signature.is_null() {
            if *input as ::core::ffi::c_int != '\0' as i32 {
                log_generic(
                    LG_ERROR,
                    server as *mut ::core::ffi::c_void,
                    c"malformed SCRAM message (garbage at end of server-final-message)".as_ptr(),
                );
            }
            server_signature_len =
                pg_b64_dec_len(strlen(encoded_server_signature) as ::core::ffi::c_int);
            decoded_server_signature = malloc(server_signature_len as size_t) as *mut uint8_t;
            if !decoded_server_signature.is_null() {
                server_signature_len = pg_b64_decode(
                    encoded_server_signature,
                    strlen(encoded_server_signature) as ::core::ffi::c_int,
                    decoded_server_signature,
                    server_signature_len,
                );
                if server_signature_len != SCRAM_SHA_256_KEY_LEN {
                    log_generic(
                        LG_ERROR,
                        server as *mut ::core::ffi::c_void,
                        c"malformed SCRAM message (malformed server signature)".as_ptr(),
                    );
                } else {
                    memcpy(
                        ServerSignature as *mut ::core::ffi::c_void,
                        decoded_server_signature as *const ::core::ffi::c_void,
                        SCRAM_SHA_256_KEY_LEN as size_t,
                    );
                    free(decoded_server_signature as *mut ::core::ffi::c_void);
                    return true;
                }
            }
        }
    }
    free(decoded_server_signature as *mut ::core::ffi::c_void);
    false
}

unsafe extern "C" fn calculate_client_proof(
    mut server: *mut PgSocket,
    mut credentials: *const PgCredentials,
    mut client_final_message_without_proof: *const ::core::ffi::c_char,
    mut result: *mut uint8_t,
) -> bool {
    let mut current_block: u64;
    let mut state: *mut ScramState = &raw mut (*server).scram_state;
    let mut rc = SASLPREP_SUCCESS;
    let mut prep_password = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut StoredKey: [uint8_t; 32] = [0; 32];
    let mut ClientKey: [uint8_t; 32] = [0; 32];
    let mut ClientSignature: [uint8_t; 32] = [0; 32];
    let mut i: ::core::ffi::c_int = 0;
    let mut ctx = ::core::ptr::null_mut::<pg_hmac_ctx>();
    let mut errstr = ::core::ptr::null::<::core::ffi::c_char>();
    ctx = pg_hmac_create((*state).hash_type);
    if ctx.is_null() {
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            c"HMAC context creation failed: %s".as_ptr(),
            pg_hmac_error(::core::ptr::null_mut::<pg_hmac_ctx>()),
        );
    } else {
        if (*credentials).use_scram_keys {
            memcpy(
                &raw mut ClientKey as *mut uint8_t as *mut ::core::ffi::c_void,
                &raw const (*credentials).scram_ClientKey as *const uint8_t
                    as *const ::core::ffi::c_void,
                SCRAM_SHA_256_KEY_LEN as size_t,
            );
            current_block = 4956146061682418353;
        } else {
            rc = pg_saslprep(
                &raw const (*credentials).passwd as *const ::core::ffi::c_char,
                &raw mut prep_password,
            );
            if rc as ::core::ffi::c_int == SASLPREP_OOM as ::core::ffi::c_int {
                current_block = 13714201677023511950;
            } else {
                if rc as ::core::ffi::c_int != SASLPREP_SUCCESS as ::core::ffi::c_int {
                    prep_password =
                        strdup(&raw const (*credentials).passwd as *const ::core::ffi::c_char);
                    if prep_password.is_null() {
                        current_block = 13714201677023511950;
                    } else {
                        current_block = 2979737022853876585;
                    }
                } else {
                    current_block = 2979737022853876585;
                }
                match current_block {
                    13714201677023511950 => {}
                    _ => {
                        (*state).SaltedPassword =
                            malloc(SCRAM_SHA_256_KEY_LEN as size_t) as *mut uint8_t;
                        if (*state).SaltedPassword.is_null() {
                            current_block = 13714201677023511950;
                        } else if scram_SaltedPassword(
                            prep_password,
                            (*state).hash_type,
                            (*state).key_length,
                            (*state).salt,
                            (*state).saltlen,
                            (*state).iterations,
                            (*state).SaltedPassword,
                            &raw mut errstr,
                        ) < 0 as ::core::ffi::c_int
                            || scram_ClientKey(
                                (*state).SaltedPassword,
                                (*state).hash_type,
                                (*state).key_length,
                                &raw mut ClientKey as *mut uint8_t,
                                &raw mut errstr,
                            ) < 0 as ::core::ffi::c_int
                        {
                            log_generic(
                                LG_ERROR,
                                server as *mut ::core::ffi::c_void,
                                c"SCRAM key derivation failed: %s".as_ptr(),
                                errstr,
                            );
                            current_block = 13714201677023511950;
                        } else {
                            current_block = 4956146061682418353;
                        }
                    }
                }
            }
        }
        match current_block {
            13714201677023511950 => {}
            _ => {
                if scram_H(
                    &raw mut ClientKey as *mut uint8_t,
                    (*state).hash_type,
                    (*state).key_length,
                    &raw mut StoredKey as *mut uint8_t,
                    &raw mut errstr,
                ) < 0 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_ERROR,
                        server as *mut ::core::ffi::c_void,
                        c"SCRAM hash computation failed: %s".as_ptr(),
                        errstr,
                    );
                } else if pg_hmac_init(
                    ctx,
                    &raw mut StoredKey as *mut uint8_t,
                    (*state).key_length as size_t,
                ) < 0 as ::core::ffi::c_int
                    || pg_hmac_update(
                        ctx,
                        (*state).client_first_message_bare as *mut uint8_t,
                        strlen((*state).client_first_message_bare),
                    ) < 0 as ::core::ffi::c_int
                    || pg_hmac_update(ctx, c",".as_ptr() as *mut uint8_t, 1 as size_t)
                        < 0 as ::core::ffi::c_int
                    || pg_hmac_update(
                        ctx,
                        (*state).server_first_message as *mut uint8_t,
                        strlen((*state).server_first_message),
                    ) < 0 as ::core::ffi::c_int
                    || pg_hmac_update(ctx, c",".as_ptr() as *mut uint8_t, 1 as size_t)
                        < 0 as ::core::ffi::c_int
                    || pg_hmac_update(
                        ctx,
                        client_final_message_without_proof as *mut uint8_t,
                        strlen(client_final_message_without_proof),
                    ) < 0 as ::core::ffi::c_int
                    || pg_hmac_final(
                        ctx,
                        &raw mut ClientSignature as *mut uint8_t,
                        (*state).key_length as size_t,
                    ) < 0 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_ERROR,
                        server as *mut ::core::ffi::c_void,
                        c"HMAC computation failed: %s".as_ptr(),
                        pg_hmac_error(ctx),
                    );
                } else {
                    i = 0 as ::core::ffi::c_int;
                    while i < (*state).key_length {
                        *result.offset(i as isize) = (ClientKey[i as usize] as ::core::ffi::c_int
                            ^ ClientSignature[i as usize] as ::core::ffi::c_int)
                            as uint8_t;
                        i += 1;
                    }
                    free(prep_password as *mut ::core::ffi::c_void);
                    pg_hmac_free(ctx);
                    return true;
                }
            }
        }
    }
    free(prep_password as *mut ::core::ffi::c_void);
    pg_hmac_free(ctx);
    false
}
#[no_mangle]

pub unsafe extern "C" fn verify_server_signature(
    mut server: *mut PgSocket,
    mut credentials: *const PgCredentials,
    mut ServerSignature: *const ::core::ffi::c_char,
    mut match_0: *mut bool,
) -> bool {
    let mut state: *mut ScramState = &raw mut (*server).scram_state;
    let mut expected_ServerSignature: [uint8_t; 32] = [0; 32];
    let mut ServerKey: [uint8_t; 32] = [0; 32];
    let mut ctx = ::core::ptr::null_mut::<pg_hmac_ctx>();
    let mut errstr = ::core::ptr::null::<::core::ffi::c_char>();
    ctx = pg_hmac_create((*state).hash_type);
    if ctx.is_null() {
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            c"HMAC context creation failed: %s".as_ptr(),
            pg_hmac_error(::core::ptr::null_mut::<pg_hmac_ctx>()),
        );
        return false;
    }
    if (*credentials).use_scram_keys {
        memcpy(
            &raw mut ServerKey as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw const (*credentials).scram_ServerKey as *const uint8_t
                as *const ::core::ffi::c_void,
            SCRAM_SHA_256_KEY_LEN as size_t,
        );
    } else if scram_ServerKey(
        (*state).SaltedPassword,
        (*state).hash_type,
        (*state).key_length,
        &raw mut ServerKey as *mut uint8_t,
        &raw mut errstr,
    ) < 0 as ::core::ffi::c_int
    {
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            c"SCRAM server key derivation failed: %s".as_ptr(),
            errstr,
        );
        pg_hmac_free(ctx);
        return false;
    }
    if pg_hmac_init(
        ctx,
        &raw mut ServerKey as *mut uint8_t,
        (*state).key_length as size_t,
    ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            (*state).client_first_message_bare as *mut uint8_t,
            strlen((*state).client_first_message_bare),
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(ctx, c",".as_ptr() as *mut uint8_t, 1 as size_t) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            (*state).server_first_message as *mut uint8_t,
            strlen((*state).server_first_message),
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(ctx, c",".as_ptr() as *mut uint8_t, 1 as size_t) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            (*state).client_final_message_without_proof as *mut uint8_t,
            strlen((*state).client_final_message_without_proof),
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_final(
            ctx,
            &raw mut expected_ServerSignature as *mut uint8_t,
            (*state).key_length as size_t,
        ) < 0 as ::core::ffi::c_int
    {
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            c"HMAC server signature computation failed: %s".as_ptr(),
            pg_hmac_error(ctx),
        );
        pg_hmac_free(ctx);
        return false;
    }
    pg_hmac_free(ctx);
    if memcmp(
        &raw mut expected_ServerSignature as *mut uint8_t as *const ::core::ffi::c_void,
        ServerSignature as *const ::core::ffi::c_void,
        (*state).key_length as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        *match_0 = false;
    } else {
        *match_0 = true;
    }
    true
}
#[no_mangle]

pub unsafe extern "C" fn read_client_first_message(
    mut client: *mut PgSocket,
    mut input: *mut ::core::ffi::c_char,
) -> bool {
    let mut current_block: u64;
    let mut state: *mut ScramState = &raw mut (*client).scram_state;
    let mut client_first_message_bare = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut client_nonce = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut client_nonce_copy = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*state).cbind_flag = *input;
    match *input as ::core::ffi::c_int {
        110 => {
            input = input.offset(1);
            current_block = 8515828400728868193;
        }
        121 => {
            input = input.offset(1);
            current_block = 8515828400728868193;
        }
        112 => {
            log_generic(
                LG_ERROR,
                client as *mut ::core::ffi::c_void,
                c"client requires SCRAM channel binding, but it is not supported".as_ptr(),
            );
            current_block = 5151340100945259836;
        }
        _ => {
            log_generic(
                LG_ERROR,
                client as *mut ::core::ffi::c_void,
                b"malformed SCRAM message (unexpected channel-binding flag \"%s\")\0" as *const u8
                    as *const ::core::ffi::c_char,
                sanitize_char(*input),
            );
            current_block = 5151340100945259836;
        }
    }
    if current_block == 8515828400728868193 {
        if *input as ::core::ffi::c_int != ',' as i32 {
            log_generic(
                LG_ERROR,
                client as *mut ::core::ffi::c_void,
                b"malformed SCRAM message (comma expected, but found character \"%s\")\0"
                    as *const u8 as *const ::core::ffi::c_char,
                sanitize_char(*input),
            );
        } else {
            input = input.offset(1);
            if *input as ::core::ffi::c_int == 'a' as i32 {
                log_generic(
                    LG_ERROR,
                    client as *mut ::core::ffi::c_void,
                    c"client uses authorization identity, but it is not supported".as_ptr(),
                );
            } else if *input as ::core::ffi::c_int != ',' as i32 {
                log_generic(
                    LG_ERROR,
                    client as *mut ::core::ffi::c_void,
                    b"malformed SCRAM message (unexpected attribute \"%s\" in client-first-message)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    sanitize_char(*input),
                );
            } else {
                input = input.offset(1);
                client_first_message_bare = strdup(input);
                if !client_first_message_bare.is_null() {
                    if *input as ::core::ffi::c_int == 'm' as i32 {
                        log_generic(
                            LG_ERROR,
                            client as *mut ::core::ffi::c_void,
                            c"client requires an unsupported SCRAM extension".as_ptr(),
                        );
                    } else {
                        read_attr_value(client, &raw mut input, 'n' as i32 as ::core::ffi::c_char);
                        client_nonce = read_attr_value(
                            client,
                            &raw mut input,
                            'r' as i32 as ::core::ffi::c_char,
                        );
                        if !client_nonce.is_null() {
                            if !is_scram_printable(client_nonce) {
                                log_generic(
                                    LG_ERROR,
                                    client as *mut ::core::ffi::c_void,
                                    c"non-printable characters in SCRAM nonce".as_ptr(),
                                );
                            } else {
                                client_nonce_copy = strdup(client_nonce);
                                if !client_nonce_copy.is_null() {
                                    loop {
                                        if *input as ::core::ffi::c_int == '\0' as i32 {
                                            current_block = 13550086250199790493;
                                            break;
                                        }
                                        if read_any_attr(
                                            client,
                                            &raw mut input,
                                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                        )
                                        .is_null()
                                        {
                                            current_block = 5151340100945259836;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        5151340100945259836 => {}
                                        _ => {
                                            (*state).client_first_message_bare =
                                                client_first_message_bare;
                                            (*state).client_nonce = client_nonce_copy;
                                            return true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free(client_first_message_bare as *mut ::core::ffi::c_void);
    free(client_nonce_copy as *mut ::core::ffi::c_void);
    false
}
#[no_mangle]

pub unsafe extern "C" fn read_client_final_message(
    mut client: *mut PgSocket,
    mut raw_input: *const uint8_t,
    mut input: *mut ::core::ffi::c_char,
    mut client_final_nonce_p: *mut *const ::core::ffi::c_char,
    mut proof_p: *mut *mut ::core::ffi::c_char,
) -> bool {
    let mut state: *mut ScramState = &raw mut (*client).scram_state;
    let mut input_start: *const ::core::ffi::c_char = input;
    let mut attr: ::core::ffi::c_char = 0;
    let mut channel_binding = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut client_final_nonce = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut proof_start = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut value = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut encoded_proof = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut proof = ::core::ptr::null_mut::<uint8_t>();
    let mut prooflen: ::core::ffi::c_int = 0;
    channel_binding = read_attr_value(client, &raw mut input, 'c' as i32 as ::core::ffi::c_char);
    if !channel_binding.is_null() {
        if !(strcmp(channel_binding, c"biws".as_ptr()) == 0 as ::core::ffi::c_int
            && (*state).cbind_flag as ::core::ffi::c_int == 'n' as i32)
            && !(strcmp(channel_binding, c"eSws".as_ptr()) == 0 as ::core::ffi::c_int
                && (*state).cbind_flag as ::core::ffi::c_int == 'y' as i32)
        {
            log_generic(
                LG_ERROR,
                client as *mut ::core::ffi::c_void,
                c"unexpected SCRAM channel-binding attribute in client-final-message".as_ptr(),
            );
        } else {
            client_final_nonce =
                read_attr_value(client, &raw mut input, 'r' as i32 as ::core::ffi::c_char);
            loop {
                proof_start = input.offset(-(1 as ::core::ffi::c_int as isize));
                value = read_any_attr(client, &raw mut input, &raw mut attr);
                if !(!value.is_null() && attr as ::core::ffi::c_int != 'p' as i32) {
                    break;
                }
            }
            if value.is_null() {
                log_generic(
                    LG_ERROR,
                    client as *mut ::core::ffi::c_void,
                    c"could not read proof".as_ptr(),
                );
            } else {
                encoded_proof = value;
                prooflen = pg_b64_dec_len(strlen(encoded_proof) as ::core::ffi::c_int);
                proof = malloc(prooflen as size_t) as *mut uint8_t;
                if proof.is_null() {
                    log_generic(
                        LG_ERROR,
                        client as *mut ::core::ffi::c_void,
                        c"could not decode proof".as_ptr(),
                    );
                } else {
                    prooflen = pg_b64_decode(
                        encoded_proof,
                        strlen(encoded_proof) as ::core::ffi::c_int,
                        proof,
                        prooflen,
                    );
                    if prooflen != SCRAM_SHA_256_KEY_LEN {
                        log_generic(
                            LG_ERROR,
                            client as *mut ::core::ffi::c_void,
                            c"malformed SCRAM message (malformed proof in client-final-message)"
                                .as_ptr(),
                        );
                    } else if *input as ::core::ffi::c_int != '\0' as i32 {
                        log_generic(
                            LG_ERROR,
                            client as *mut ::core::ffi::c_void,
                            c"malformed SCRAM message (garbage at the end of client-final-message)"
                                .as_ptr(),
                        );
                    } else {
                        (*state).client_final_message_without_proof = malloc(
                            (proof_start.offset_from(input_start) as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as size_t,
                        )
                            as *mut ::core::ffi::c_char;
                        if !(*state).client_final_message_without_proof.is_null() {
                            memcpy(
                                (*state).client_final_message_without_proof
                                    as *mut ::core::ffi::c_void,
                                raw_input as *const ::core::ffi::c_void,
                                proof_start.offset_from(input_start) as ::core::ffi::c_long
                                    as size_t,
                            );
                            *(*state).client_final_message_without_proof.offset(
                                proof_start.offset_from(input_start) as ::core::ffi::c_long
                                    as isize,
                            ) = '\0' as i32 as ::core::ffi::c_char;
                            *client_final_nonce_p = client_final_nonce;
                            *proof_p = proof as *mut ::core::ffi::c_char;
                            return true;
                        }
                    }
                }
            }
        }
    }
    free(proof as *mut ::core::ffi::c_void);
    false
}

unsafe extern "C" fn build_adhoc_scram_secret(
    mut plain_password: *const ::core::ffi::c_char,
    mut state: *mut ScramState,
) -> bool {
    let mut password = ::core::ptr::null::<::core::ffi::c_char>();
    let mut prep_password = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rc = SASLPREP_SUCCESS;
    let mut saltbuf: [uint8_t; 16] = [0; 16];
    let mut encoded_len: ::core::ffi::c_int = 0;
    let mut salted_password: [uint8_t; 32] = [0; 32];
    let mut errstr = ::core::ptr::null::<::core::ffi::c_char>();
    rc = pg_saslprep(plain_password, &raw mut prep_password);
    if rc as ::core::ffi::c_int != SASLPREP_OOM as ::core::ffi::c_int {
        if rc as ::core::ffi::c_int == SASLPREP_SUCCESS as ::core::ffi::c_int {
            password = prep_password;
        } else {
            password = plain_password;
        }
        get_random_bytes(
            &raw mut saltbuf as *mut uint8_t,
            ::core::mem::size_of::<[uint8_t; 16]>() as ::core::ffi::c_int,
        );
        (*state).adhoc = true;
        (*state).iterations = cf_scram_iterations;
        encoded_len = pg_b64_enc_len(::core::mem::size_of::<[uint8_t; 16]>() as ::core::ffi::c_int);
        (*state).encoded_salt =
            malloc((encoded_len + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
        if !(*state).encoded_salt.is_null() {
            encoded_len = pg_b64_encode(
                &raw mut saltbuf as *mut uint8_t,
                ::core::mem::size_of::<[uint8_t; 16]>() as ::core::ffi::c_int,
                (*state).encoded_salt,
                encoded_len,
            );
            if encoded_len >= 0 as ::core::ffi::c_int {
                *(*state).encoded_salt.offset(encoded_len as isize) =
                    '\0' as i32 as ::core::ffi::c_char;
                scram_SaltedPassword(
                    password,
                    (*state).hash_type,
                    (*state).key_length,
                    &raw mut saltbuf as *mut uint8_t,
                    ::core::mem::size_of::<[uint8_t; 16]>() as ::core::ffi::c_int,
                    (*state).iterations,
                    &raw mut salted_password as *mut uint8_t,
                    &raw mut errstr,
                );
                scram_ClientKey(
                    &raw mut salted_password as *mut uint8_t,
                    (*state).hash_type,
                    (*state).key_length,
                    &raw mut (*state).StoredKey as *mut uint8_t,
                    &raw mut errstr,
                );
                scram_H(
                    &raw mut (*state).StoredKey as *mut uint8_t,
                    (*state).hash_type,
                    (*state).key_length,
                    &raw mut (*state).StoredKey as *mut uint8_t,
                    &raw mut errstr,
                );
                scram_ServerKey(
                    &raw mut salted_password as *mut uint8_t,
                    (*state).hash_type,
                    (*state).key_length,
                    &raw mut (*state).ServerKey as *mut uint8_t,
                    &raw mut errstr,
                );
                free(prep_password as *mut ::core::ffi::c_void);
                return true;
            }
        }
    }
    free(prep_password as *mut ::core::ffi::c_void);
    false
}

unsafe extern "C" fn scram_mock_salt(
    mut username: *const ::core::ffi::c_char,
    mut saltbuf: *mut uint8_t,
) -> bool {
    static mut mock_auth_nonce: [uint8_t; 32] = [0; 32];
    static mut mock_auth_nonce_initialized: bool = false;
    let mut ctx = ::core::ptr::null_mut::<pg_cryptohash_ctx>();
    let mut sha_digest: [uint8_t; 32] = [0; 32];
    if !mock_auth_nonce_initialized {
        get_random_bytes(
            &raw mut mock_auth_nonce as *mut uint8_t,
            ::core::mem::size_of::<[uint8_t; 32]>() as ::core::ffi::c_int,
        );
        mock_auth_nonce_initialized = true;
    }
    ctx = pg_cryptohash_create(PG_SHA256);
    if ctx.is_null() {
        let mut _log_ctx = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx,
            c"could not create cryptohash context".as_ptr(),
        );
        return false;
    }
    if pg_cryptohash_init(ctx) < 0 as ::core::ffi::c_int
        || pg_cryptohash_update(ctx, username as *mut uint8_t, strlen(username))
            < 0 as ::core::ffi::c_int
        || pg_cryptohash_update(
            ctx,
            &raw mut mock_auth_nonce as *mut uint8_t,
            ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
        ) < 0 as ::core::ffi::c_int
        || pg_cryptohash_final(
            ctx,
            &raw mut sha_digest as *mut uint8_t,
            ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
        ) < 0 as ::core::ffi::c_int
    {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx_0,
            c"could not generate mock salt: %s".as_ptr(),
            pg_cryptohash_error(ctx),
        );
        pg_cryptohash_free(ctx);
        return false;
    }
    pg_cryptohash_free(ctx);
    memcpy(
        saltbuf as *mut ::core::ffi::c_void,
        &raw mut sha_digest as *mut uint8_t as *const ::core::ffi::c_void,
        SCRAM_DEFAULT_SALT_LEN as size_t,
    );
    true
}

unsafe extern "C" fn build_mock_scram_secret(
    mut username: *const ::core::ffi::c_char,
    mut state: *mut ScramState,
) -> bool {
    let mut saltbuf: [uint8_t; 16] = [0; 16];
    let mut encoded_len: ::core::ffi::c_int = 0;
    (*state).iterations = cf_scram_iterations;
    if scram_mock_salt(username, &raw mut saltbuf as *mut uint8_t) {
        encoded_len = pg_b64_enc_len(::core::mem::size_of::<[uint8_t; 16]>() as ::core::ffi::c_int);
        (*state).encoded_salt =
            malloc((encoded_len + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
        if !(*state).encoded_salt.is_null() {
            encoded_len = pg_b64_encode(
                &raw mut saltbuf as *mut uint8_t,
                ::core::mem::size_of::<[uint8_t; 16]>() as ::core::ffi::c_int,
                (*state).encoded_salt,
                encoded_len,
            );
            if encoded_len >= 0 as ::core::ffi::c_int {
                *(*state).encoded_salt.offset(encoded_len as isize) =
                    '\0' as i32 as ::core::ffi::c_char;
                return true;
            }
        }
    }
    false
}
#[no_mangle]

pub unsafe extern "C" fn build_server_first_message(
    mut state: *mut ScramState,
    mut user: *mut PgCredentials,
    mut stored_secret: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut current_block: u64;
    let mut raw_nonce: [uint8_t; 19] = [0; 19];
    let mut encoded_len: ::core::ffi::c_int = 0;
    let mut len: size_t = 0;
    let mut result = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*state).hash_type = PG_SHA256;
    (*state).key_length = SCRAM_SHA_256_KEY_LEN;
    if stored_secret.is_null() {
        if !build_mock_scram_secret(&raw mut (*user).name as *mut ::core::ffi::c_char, state) {
            current_block = 4658626102049014135;
        } else {
            current_block = 5601891728916014340;
        }
    } else if (*user).adhoc_scram_secrets_cached {
        (*state).adhoc = true;
        (*state).iterations = (*user).scram_Iiterations;
        (*state).encoded_salt = strdup((*user).scram_SaltKey);
        memcpy(
            &raw mut (*state).StoredKey as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw mut (*user).scram_StoredKey as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
        );
        memcpy(
            &raw mut (*state).ServerKey as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw mut (*user).scram_ServerKey as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
        );
        current_block = 5601891728916014340;
    } else {
        match get_password_type(stored_secret) as ::core::ffi::c_uint {
            2 => {
                current_block = 1894949217534896604;
                match current_block {
                    2508738315967109217 => {
                        if !build_adhoc_scram_secret(stored_secret, state) {
                            current_block = 4658626102049014135;
                        } else {
                            current_block = 8236137900636309791;
                        }
                    }
                    _ => {
                        if !parse_scram_secret(
                            stored_secret,
                            &raw mut (*state).iterations,
                            &raw mut (*state).encoded_salt,
                            &raw mut (*state).StoredKey as *mut uint8_t,
                            &raw mut (*state).ServerKey as *mut uint8_t,
                        ) {
                            current_block = 4658626102049014135;
                        } else {
                            current_block = 8236137900636309791;
                        }
                    }
                }
                match current_block {
                    4658626102049014135 => {}
                    _ => {
                        if !(*user).dynamic_passwd {
                            (*user).scram_Iiterations = (*state).iterations;
                            (*user).scram_SaltKey = strdup((*state).encoded_salt);
                            memcpy(
                                &raw mut (*user).scram_StoredKey as *mut uint8_t
                                    as *mut ::core::ffi::c_void,
                                &raw mut (*state).StoredKey as *mut uint8_t
                                    as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
                            );
                            memcpy(
                                &raw mut (*user).scram_ServerKey as *mut uint8_t
                                    as *mut ::core::ffi::c_void,
                                &raw mut (*state).ServerKey as *mut uint8_t
                                    as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
                            );
                            (*user).adhoc_scram_secrets_cached = true;
                        }
                        current_block = 5601891728916014340;
                    }
                }
            }
            0 => {
                current_block = 2508738315967109217;
                match current_block {
                    2508738315967109217 => {
                        if !build_adhoc_scram_secret(stored_secret, state) {
                            current_block = 4658626102049014135;
                        } else {
                            current_block = 8236137900636309791;
                        }
                    }
                    _ => {
                        if !parse_scram_secret(
                            stored_secret,
                            &raw mut (*state).iterations,
                            &raw mut (*state).encoded_salt,
                            &raw mut (*state).StoredKey as *mut uint8_t,
                            &raw mut (*state).ServerKey as *mut uint8_t,
                        ) {
                            current_block = 4658626102049014135;
                        } else {
                            current_block = 8236137900636309791;
                        }
                    }
                }
                match current_block {
                    4658626102049014135 => {}
                    _ => {
                        if !(*user).dynamic_passwd {
                            (*user).scram_Iiterations = (*state).iterations;
                            (*user).scram_SaltKey = strdup((*state).encoded_salt);
                            memcpy(
                                &raw mut (*user).scram_StoredKey as *mut uint8_t
                                    as *mut ::core::ffi::c_void,
                                &raw mut (*state).StoredKey as *mut uint8_t
                                    as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
                            );
                            memcpy(
                                &raw mut (*user).scram_ServerKey as *mut uint8_t
                                    as *mut ::core::ffi::c_void,
                                &raw mut (*state).ServerKey as *mut uint8_t
                                    as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
                            );
                            (*user).adhoc_scram_secrets_cached = true;
                        }
                        current_block = 5601891728916014340;
                    }
                }
            }
            _ => {
                current_block = 4658626102049014135;
            }
        }
    }
    if current_block == 5601891728916014340 {
        get_random_bytes(&raw mut raw_nonce as *mut uint8_t, SCRAM_RAW_NONCE_LEN);
        encoded_len = pg_b64_enc_len(SCRAM_RAW_NONCE_LEN);
        (*state).server_nonce =
            malloc((encoded_len + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
        if !(*state).server_nonce.is_null() {
            encoded_len = pg_b64_encode(
                &raw mut raw_nonce as *mut uint8_t,
                SCRAM_RAW_NONCE_LEN,
                (*state).server_nonce,
                encoded_len,
            );
            if encoded_len >= 0 as ::core::ffi::c_int {
                *(*state).server_nonce.offset(encoded_len as isize) =
                    '\0' as i32 as ::core::ffi::c_char;
                len = (2 as size_t)
                    .wrapping_add(strlen((*state).client_nonce))
                    .wrapping_add(strlen((*state).server_nonce))
                    .wrapping_add(3 as size_t)
                    .wrapping_add(strlen((*state).encoded_salt))
                    .wrapping_add(3 as size_t)
                    .wrapping_add(10 as size_t)
                    .wrapping_add(1 as size_t);
                result = malloc(len) as *mut ::core::ffi::c_char;
                if !result.is_null() {
                    snprintf(
                        result,
                        len,
                        c"r=%s%s,s=%s,i=%u".as_ptr(),
                        (*state).client_nonce,
                        (*state).server_nonce,
                        (*state).encoded_salt,
                        (*state).iterations,
                    );
                    (*state).server_first_message = result;
                    return result;
                }
            }
        }
    }
    free((*state).server_nonce as *mut ::core::ffi::c_void);
    free((*state).server_first_message as *mut ::core::ffi::c_void);
    ::core::ptr::null_mut::<::core::ffi::c_char>()
}

unsafe extern "C" fn compute_server_signature(
    mut client: *mut PgSocket,
    mut state: *mut ScramState,
) -> *mut ::core::ffi::c_char {
    let mut ServerSignature: [uint8_t; 32] = [0; 32];
    let mut server_signature_base64 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut siglen: ::core::ffi::c_int = 0;
    let mut ctx = ::core::ptr::null_mut::<pg_hmac_ctx>();
    ctx = pg_hmac_create((*state).hash_type);
    if ctx.is_null() {
        log_generic(
            LG_ERROR,
            client as *mut ::core::ffi::c_void,
            c"HMAC context creation failed: %s".as_ptr(),
            pg_hmac_error(::core::ptr::null_mut::<pg_hmac_ctx>()),
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if pg_hmac_init(
        ctx,
        &raw mut (*state).ServerKey as *mut uint8_t,
        (*state).key_length as size_t,
    ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            (*state).client_first_message_bare as *mut uint8_t,
            strlen((*state).client_first_message_bare),
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(ctx, c",".as_ptr() as *mut uint8_t, 1 as size_t) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            (*state).server_first_message as *mut uint8_t,
            strlen((*state).server_first_message),
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(ctx, c",".as_ptr() as *mut uint8_t, 1 as size_t) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            (*state).client_final_message_without_proof as *mut uint8_t,
            strlen((*state).client_final_message_without_proof),
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_final(
            ctx,
            &raw mut ServerSignature as *mut uint8_t,
            (*state).key_length as size_t,
        ) < 0 as ::core::ffi::c_int
    {
        log_generic(
            LG_ERROR,
            client as *mut ::core::ffi::c_void,
            c"HMAC operation failed: %s".as_ptr(),
            pg_hmac_error(ctx),
        );
        pg_hmac_free(ctx);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    siglen = pg_b64_enc_len(SCRAM_SHA_256_KEY_LEN);
    server_signature_base64 =
        malloc((siglen + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
    if server_signature_base64.is_null() {
        pg_hmac_free(ctx);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    siglen = pg_b64_encode(
        &raw mut ServerSignature as *mut uint8_t,
        SCRAM_SHA_256_KEY_LEN,
        server_signature_base64,
        siglen,
    );
    if siglen < 0 as ::core::ffi::c_int {
        free(server_signature_base64 as *mut ::core::ffi::c_void);
        pg_hmac_free(ctx);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    *server_signature_base64.offset(siglen as isize) = '\0' as i32 as ::core::ffi::c_char;
    pg_hmac_free(ctx);
    server_signature_base64
}
#[no_mangle]

pub unsafe extern "C" fn build_server_final_message(
    mut client: *mut PgSocket,
) -> *mut ::core::ffi::c_char {
    let mut state: *mut ScramState = &raw mut (*client).scram_state;
    let mut server_signature = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: size_t = 0;
    let mut result = ::core::ptr::null_mut::<::core::ffi::c_char>();
    server_signature = compute_server_signature(client, state);
    if !server_signature.is_null() {
        len = (2 as size_t)
            .wrapping_add(strlen(server_signature))
            .wrapping_add(1 as size_t);
        if len < INT_MAX as size_t {
            result = malloc(len) as *mut ::core::ffi::c_char;
            if !result.is_null() {
                snprintf(result, len, c"v=%s".as_ptr(), server_signature);
                free(server_signature as *mut ::core::ffi::c_void);
                return result;
            }
        }
    }
    free(server_signature as *mut ::core::ffi::c_void);
    ::core::ptr::null_mut::<::core::ffi::c_char>()
}
#[no_mangle]

pub unsafe extern "C" fn verify_final_nonce(
    mut state: *const ScramState,
    mut client_final_nonce: *const ::core::ffi::c_char,
) -> bool {
    let mut client_nonce_len = strlen((*state).client_nonce);
    let mut server_nonce_len = strlen((*state).server_nonce);
    let mut final_nonce_len = strlen(client_final_nonce);
    if final_nonce_len != client_nonce_len.wrapping_add(server_nonce_len) {
        return false;
    }
    if memcmp(
        client_final_nonce as *const ::core::ffi::c_void,
        (*state).client_nonce as *const ::core::ffi::c_void,
        client_nonce_len,
    ) != 0 as ::core::ffi::c_int
    {
        return false;
    }
    if memcmp(
        client_final_nonce.add(client_nonce_len) as *const ::core::ffi::c_void,
        (*state).server_nonce as *const ::core::ffi::c_void,
        server_nonce_len,
    ) != 0 as ::core::ffi::c_int
    {
        return false;
    }
    true
}
#[no_mangle]

pub unsafe extern "C" fn verify_client_proof(
    mut client: *mut PgSocket,
    mut ClientProof: *const ::core::ffi::c_char,
) -> bool {
    let mut state: *mut ScramState = &raw mut (*client).scram_state;
    let mut ClientSignature: [uint8_t; 32] = [0; 32];
    let mut client_StoredKey: [uint8_t; 32] = [0; 32];
    let mut ctx = ::core::ptr::null_mut::<pg_hmac_ctx>();
    let mut i: ::core::ffi::c_int = 0;
    let mut errstr = ::core::ptr::null::<::core::ffi::c_char>();
    ctx = pg_hmac_create((*state).hash_type);
    if ctx.is_null() {
        log_generic(
            LG_ERROR,
            client as *mut ::core::ffi::c_void,
            c"HMAC context creation failed: %s".as_ptr(),
            pg_hmac_error(::core::ptr::null_mut::<pg_hmac_ctx>()),
        );
        return false;
    }
    if pg_hmac_init(
        ctx,
        &raw mut (*state).StoredKey as *mut uint8_t,
        (*state).key_length as size_t,
    ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            (*state).client_first_message_bare as *mut uint8_t,
            strlen((*state).client_first_message_bare),
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(ctx, c",".as_ptr() as *mut uint8_t, 1 as size_t) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            (*state).server_first_message as *mut uint8_t,
            strlen((*state).server_first_message),
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(ctx, c",".as_ptr() as *mut uint8_t, 1 as size_t) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            (*state).client_final_message_without_proof as *mut uint8_t,
            strlen((*state).client_final_message_without_proof),
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_final(
            ctx,
            &raw mut ClientSignature as *mut uint8_t,
            (*state).key_length as size_t,
        ) < 0 as ::core::ffi::c_int
    {
        log_generic(
            LG_ERROR,
            client as *mut ::core::ffi::c_void,
            c"HMAC operation failed: %s".as_ptr(),
            pg_hmac_error(ctx),
        );
        pg_hmac_free(ctx);
        return false;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*state).key_length {
        (*state).ClientKey[i as usize] = (*ClientProof.offset(i as isize) as ::core::ffi::c_int
            ^ ClientSignature[i as usize] as ::core::ffi::c_int)
            as uint8_t;
        i += 1;
    }
    if scram_H(
        &raw mut (*state).ClientKey as *mut uint8_t,
        (*state).hash_type,
        (*state).key_length,
        &raw mut client_StoredKey as *mut uint8_t,
        &raw mut errstr,
    ) < 0 as ::core::ffi::c_int
    {
        pg_hmac_free(ctx);
        return false;
    }
    if memcmp(
        &raw mut client_StoredKey as *mut uint8_t as *const ::core::ffi::c_void,
        &raw mut (*state).StoredKey as *mut uint8_t as *const ::core::ffi::c_void,
        (*state).key_length as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        pg_hmac_free(ctx);
        return false;
    }
    pg_hmac_free(ctx);
    true
}
#[no_mangle]

pub unsafe extern "C" fn scram_verify_plain_password(
    mut client: *mut PgSocket,
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut secret: *const ::core::ffi::c_char,
) -> bool {
    let mut encoded_salt = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut salt = ::core::ptr::null_mut::<uint8_t>();
    let mut saltlen: ::core::ffi::c_int = 0;
    let mut iterations: ::core::ffi::c_int = 0;
    let mut salted_password: [uint8_t; 32] = [0; 32];
    let mut stored_key: [uint8_t; 32] = [0; 32];
    let mut server_key: [uint8_t; 32] = [0; 32];
    let mut computed_key: [uint8_t; 32] = [0; 32];
    let mut prep_password = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rc = SASLPREP_SUCCESS;
    let mut result = false;
    let mut errstr = ::core::ptr::null::<::core::ffi::c_char>();
    if !parse_scram_secret(
        secret,
        &raw mut iterations,
        &raw mut encoded_salt,
        &raw mut stored_key as *mut uint8_t,
        &raw mut server_key as *mut uint8_t,
    ) {
        log_generic(
            LG_WARNING,
            client as *mut ::core::ffi::c_void,
            b"invalid SCRAM secret for user \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            username,
        );
    } else {
        saltlen = pg_b64_dec_len(strlen(encoded_salt) as ::core::ffi::c_int);
        salt = malloc(saltlen as size_t) as *mut uint8_t;
        if !salt.is_null() {
            saltlen = pg_b64_decode(
                encoded_salt,
                strlen(encoded_salt) as ::core::ffi::c_int,
                salt,
                saltlen,
            );
            if saltlen < 0 as ::core::ffi::c_int {
                log_generic(
                    LG_WARNING,
                    client as *mut ::core::ffi::c_void,
                    b"invalid SCRAM secret for user \"%s\"\0" as *const u8
                        as *const ::core::ffi::c_char,
                    username,
                );
            } else {
                rc = pg_saslprep(password, &raw mut prep_password);
                if rc as ::core::ffi::c_int == SASLPREP_SUCCESS as ::core::ffi::c_int {
                    password = prep_password;
                }
                scram_SaltedPassword(
                    password,
                    PG_SHA256,
                    SCRAM_SHA_256_KEY_LEN,
                    salt,
                    saltlen,
                    iterations,
                    &raw mut salted_password as *mut uint8_t,
                    &raw mut errstr,
                );
                scram_ServerKey(
                    &raw mut salted_password as *mut uint8_t,
                    PG_SHA256,
                    SCRAM_SHA_256_KEY_LEN,
                    &raw mut computed_key as *mut uint8_t,
                    &raw mut errstr,
                );
                result = memcmp(
                    &raw mut computed_key as *mut uint8_t as *const ::core::ffi::c_void,
                    &raw mut server_key as *mut uint8_t as *const ::core::ffi::c_void,
                    SCRAM_SHA_256_KEY_LEN as size_t,
                ) == 0 as ::core::ffi::c_int;
            }
        }
    }
    free(encoded_salt as *mut ::core::ffi::c_void);
    free(salt as *mut ::core::ffi::c_void);
    free(prep_password as *mut ::core::ffi::c_void);
    result
}

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
