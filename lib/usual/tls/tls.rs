#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = *mut ::core::ffi::c_char;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:18"]
pub mod _types_h {
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
    #[c2rust::src_loc = "95:1"]
    pub type __darwin_va_list = __builtin_va_list;
    #[c2rust::src_loc = "118:1"]
    pub type __darwin_ssize_t = isize;
    #[c2rust::src_loc = "119:1"]
    pub type __darwin_time_t = ::core::ffi::c_long;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:18"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ssize_t.h:18"]
pub mod _ssize_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_time_t.h:18"]
pub mod _time_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type time_t = __darwin_time_t;
    use super::_types_h::__darwin_time_t;
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_va_list.h:18"]
pub mod _va_list_h {
    #[c2rust::src_loc = "44:1"]
    pub type va_list = __darwin_va_list;
    use super::_types_h::__darwin_va_list;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/tls/tls_internal.h:31"]
pub mod tls_internal_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "97:1"]
    pub struct tls {
        pub config: *mut tls_config,
        pub error: tls_error,
        pub flags: uint32_t,
        pub state: uint32_t,
        pub servername: *mut ::core::ffi::c_char,
        pub socket: ::core::ffi::c_int,
        pub ssl_conn: *mut SSL,
        pub ssl_ctx: *mut SSL_CTX,
        pub ssl_peer_cert: *mut X509,
        pub conninfo: *mut tls_conninfo,
        pub used_dh_bits: ::core::ffi::c_int,
        pub used_ecdh_nid: ::core::ffi::c_int,
        pub ocsp_result: *const ::core::ffi::c_char,
        pub ocsp_info: *mut tls_ocsp_info,
        pub ocsp_query: *mut tls_ocsp_query,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "121:1"]
    pub struct tls_ocsp_info {
        pub response_status: ::core::ffi::c_int,
        pub cert_status: ::core::ffi::c_int,
        pub crl_reason: ::core::ffi::c_int,
        pub this_update: time_t,
        pub next_update: time_t,
        pub revocation_time: time_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "73:1"]
    pub struct tls_conninfo {
        pub issuer: *mut ::core::ffi::c_char,
        pub subject: *mut ::core::ffi::c_char,
        pub hash: *mut ::core::ffi::c_char,
        pub serial: *mut ::core::ffi::c_char,
        pub fingerprint: *mut ::core::ffi::c_char,
        pub version: *mut ::core::ffi::c_char,
        pub cipher: *mut ::core::ffi::c_char,
        pub notbefore: time_t,
        pub notafter: time_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:1"]
    pub struct tls_error {
        pub msg: *mut ::core::ffi::c_char,
        pub num: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:1"]
    pub struct tls_config {
        pub error: tls_error,
        pub ca_file: *const ::core::ffi::c_char,
        pub ca_path: *const ::core::ffi::c_char,
        pub ca_mem: *mut ::core::ffi::c_char,
        pub ca_len: size_t,
        pub ciphers: *const ::core::ffi::c_char,
        pub cipher_suites: *const ::core::ffi::c_char,
        pub ciphers_server: ::core::ffi::c_int,
        pub dheparams: ::core::ffi::c_int,
        pub ecdhecurve: ::core::ffi::c_int,
        pub keypair: *mut tls_keypair,
        pub ocsp_file: *const ::core::ffi::c_char,
        pub ocsp_mem: *mut ::core::ffi::c_char,
        pub ocsp_len: size_t,
        pub protocols: uint32_t,
        pub verify_cert: ::core::ffi::c_int,
        pub verify_client: ::core::ffi::c_int,
        pub verify_depth: ::core::ffi::c_int,
        pub verify_name: ::core::ffi::c_int,
        pub verify_time: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:1"]
    pub struct tls_keypair {
        pub next: *mut tls_keypair,
        pub cert_file: *const ::core::ffi::c_char,
        pub cert_mem: *mut ::core::ffi::c_char,
        pub cert_len: size_t,
        pub key_file: *const ::core::ffi::c_char,
        pub key_mem: *mut ::core::ffi::c_char,
        pub key_len: size_t,
    }
    #[c2rust::src_loc = "85:9"]
    pub const TLS_CLIENT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "86:9"]
    pub const TLS_SERVER: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "87:9"]
    pub const TLS_SERVER_CONN: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "88:9"]
    pub const TLS_OCSP_CLIENT: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "90:9"]
    pub const TLS_EOF_NO_CLOSE_NOTIFY: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "91:9"]
    pub const TLS_HANDSHAKE_COMPLETE: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "92:9"]
    pub const TLS_DO_ABORT: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    use super::_size_t_h::size_t;
    use super::_time_t_h::time_t;
    use super::_uint32_t_h::uint32_t;
    use super::types_h::{SSL, SSL_CTX, X509};
    extern "C" {
        #[c2rust::src_loc = "94:1"]
        pub type tls_ocsp_query;
        #[c2rust::src_loc = "136:1"]
        pub fn tls_configure_server(ctx: *mut tls) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "139:1"]
        pub fn tls_handshake_client(ctx: *mut tls) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "140:1"]
        pub fn tls_handshake_server(ctx: *mut tls) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "168:1"]
        pub fn tls_get_conninfo(ctx: *mut tls) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "169:1"]
        pub fn tls_free_conninfo(conninfo: *mut tls_conninfo);
        #[c2rust::src_loc = "173:1"]
        pub fn tls_ocsp_client_free(ctx: *mut tls);
        #[c2rust::src_loc = "174:1"]
        pub fn tls_ocsp_info_free(info: *mut tls_ocsp_info);
    }
}
// OpenSSL types - use shared openssl_types module
pub mod types_h {
    pub use super::super::openssl_types::{
        bio_st, evp_pkey_st, pem_password_cb, ssl_ctx_st, ssl_st, x509_st, x509_store_ctx_st,
        X509_VERIFY_PARAM_st, BIO, EVP_PKEY, SSL, SSL_CTX, X509, X509_STORE_CTX, X509_VERIFY_PARAM,
    };
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/ssl.h:18"]
// OpenSSL SSL functions - use shared openssl_types module
pub mod ssl_h {
    use super::super::openssl_types as ossl;
    use super::_uint64_t_h::uint64_t;

    // Re-export types
    pub use ossl::{SSL_verify_cb, X509_FILETYPE_PEM as SSL_FILETYPE_PEM};
    pub use ossl::{
        SSL_CB_HANDSHAKE_START, SSL_CTRL_MODE, SSL_CTRL_SET_MAX_PROTO_VERSION,
        SSL_CTRL_SET_MIN_PROTO_VERSION, SSL_ERROR_NONE, SSL_ERROR_SSL, SSL_ERROR_SYSCALL,
        SSL_ERROR_WANT_ACCEPT, SSL_ERROR_WANT_CONNECT, SSL_ERROR_WANT_READ, SSL_ERROR_WANT_WRITE,
        SSL_ERROR_WANT_X509_LOOKUP, SSL_ERROR_ZERO_RETURN,
    };
    pub use ossl::{SSL_OP_NO_SSLv2, SSL_OP_NO_SSLv3, SSL_OP_NO_TLSv1};
    pub use ossl::{SSL_OP_NO_TLSv1_1, SSL_OP_NO_TLSv1_2, SSL_OP_NO_TLSv1_3};

    // Re-export functions
    pub use ossl::SSL_CTX_check_private_key;
    pub use ossl::SSL_CTX_clear_options;
    pub use ossl::SSL_CTX_ctrl;
    pub use ossl::SSL_CTX_free;
    pub use ossl::SSL_CTX_get0_param;
    pub use ossl::SSL_CTX_load_verify_locations;
    pub use ossl::SSL_CTX_set_cipher_list;
    pub use ossl::SSL_CTX_set_ciphersuites;
    pub use ossl::SSL_CTX_set_info_callback;
    pub use ossl::SSL_CTX_set_options;
    pub use ossl::SSL_CTX_set_verify;
    pub use ossl::SSL_CTX_set_verify_depth;
    pub use ossl::SSL_CTX_use_PrivateKey;
    pub use ossl::SSL_CTX_use_PrivateKey_file;
    pub use ossl::SSL_CTX_use_certificate_chain_file;
    pub use ossl::SSL_free;
    pub use ossl::SSL_get1_peer_certificate;
    pub use ossl::SSL_get_error;
    pub use ossl::SSL_get_ex_data;
    pub use ossl::SSL_read;
    pub use ossl::SSL_shutdown;
    pub use ossl::SSL_version;
    pub use ossl::SSL_write;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:18"]
pub mod _malloc_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "55:1"]
        pub fn calloc(__count: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "56:1"]
        pub fn free(_: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:18"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/tls/tls.h:18"]
pub mod tls_h {
    #[c2rust::src_loc = "29:9"]
    pub const TLS_PROTOCOL_TLSv1_0: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "30:9"]
    pub const TLS_PROTOCOL_TLSv1_1: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "31:9"]
    pub const TLS_PROTOCOL_TLSv1_2: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "32:9"]
    pub const TLS_PROTOCOL_TLSv1_3: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
    #[c2rust::src_loc = "39:9"]
    pub const TLS_WANT_POLLIN: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
    #[c2rust::src_loc = "40:9"]
    pub const TLS_WANT_POLLOUT: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
    use super::tls_internal_h::tls_config;
    extern "C" {
        #[c2rust::src_loc = "77:1"]
        pub fn tls_config_new() -> *mut tls_config;
        #[c2rust::src_loc = "78:1"]
        pub fn tls_config_free(_config: *mut tls_config);
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/x509.h:18"]
// OpenSSL X509 functions - use shared openssl_types module
pub mod x509_h {
    pub use super::super::openssl_types::{X509_FILETYPE_PEM, X509_free};
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:18"]
pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "75:1"]
        pub fn memcmp(
            __s1: *const ::core::ffi::c_void,
            __s2: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "95:1"]
        pub fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "96:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
// OpenSSL BIO functions - use shared openssl_types module
pub mod bio_h {
    pub use super::super::openssl_types::{BIO_free, BIO_new_mem_buf};
}
// OpenSSL EVP functions - use shared openssl_types module
pub mod evp_h {
    pub use super::super::openssl_types::EVP_PKEY_free;
}
// OpenSSL X509 verification - use shared openssl_types module
pub mod x509_vfy_h {
    pub const X509_V_FLAG_NO_CHECK_TIME: ::core::ffi::c_int = 0x200000;
    pub use super::super::openssl_types::X509_VERIFY_PARAM_set_flags;
}
// OpenSSL PEM functions - use shared openssl_types module
pub mod pem_h {
    pub use super::super::openssl_types::PEM_read_bio_PrivateKey;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/tls/tls_compat.h:18"]
pub mod tls_compat_h {
    use super::types_h::SSL_CTX;
    extern "C" {
        #[c2rust::src_loc = "111:1"]
        pub fn SSL_CTX_use_certificate_chain_mem(
            ctx: *mut SSL_CTX,
            buf: *mut ::core::ffi::c_void,
            len: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "115:1"]
        pub fn SSL_CTX_load_verify_mem(
            ctx: *mut SSL_CTX,
            buf: *mut ::core::ffi::c_void,
            len: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "130:1"]
        pub fn tls_compat_cleanup();
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:18"]
pub mod sys__types_h {
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/limits.h:18"]
pub mod limits_h {
    #[c2rust::src_loc = "94:9"]
    pub const INT_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:18"]
pub mod _stdio_h {

    extern "C" {
        #[c2rust::src_loc = "471:1"]
        pub fn asprintf(
            _: *mut *mut ::core::ffi::c_char,
            _: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "478:1"]
        pub fn vasprintf(
            _: *mut *mut ::core::ffi::c_char,
            _: *const ::core::ffi::c_char,
            _: ::core::ffi::VaList,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/errno.h:18"]
pub mod errno_h {
    #[c2rust::src_loc = "169:9"]
    pub const ECONNRESET: ::core::ffi::c_int = 54 as ::core::ffi::c_int;
    #[c2rust::src_loc = "172:9"]
    pub const ENOTCONN: ::core::ffi::c_int = 57 as ::core::ffi::c_int;
    extern "C" {
        #[c2rust::src_loc = "80:1"]
        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/unistd.h:18"]
pub mod unistd_h {
    extern "C" {
        #[c2rust::src_loc = "441:1"]
        pub fn close(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/llvm/21.1.8/lib/clang/21/include/stdbool.h:18"]
pub mod stdbool_h {
    #[c2rust::src_loc = "25:9"]
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_strings.h:18"]
pub mod _strings_h {
    extern "C" {
        #[c2rust::src_loc = "81:1"]
        pub fn strcasecmp(
            _: *const ::core::ffi::c_char,
            _: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/string.h:18"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "217:1"]
        pub fn strcmpeq(
            str_left: *const ::core::ffi::c_char,
            str_right: *const ::core::ffi::c_char,
        ) -> bool;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/socket.h:18"]
pub mod socket_h {
    #[c2rust::src_loc = "690:9"]
    pub const SHUT_RDWR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    extern "C" {
        #[c2rust::src_loc = "727:1"]
        pub fn shutdown(_: ::core::ffi::c_int, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    }
}
// OpenSSL crypto functions - use shared openssl_types module
pub mod crypto_h {
    pub use super::super::openssl_types::OPENSSL_cleanup;
}
// OpenSSL TLS version constants - use shared openssl_types module
pub mod prov_ssl_h {
    pub use super::super::openssl_types::{
        TLS1_1_VERSION, TLS1_2_VERSION, TLS1_3_VERSION, TLS1_VERSION,
    };
}
// OpenSSL error functions - use shared openssl_types module
pub mod err_h {
    pub use super::super::openssl_types::{
        ERR_clear_error, ERR_error_string, ERR_peek_error, ERR_reason_error_string,
    };
}
use self::_malloc_h::{calloc, free};
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
pub use self::_ssize_t_h::ssize_t;
use self::_stdio_h::{asprintf, vasprintf};
use self::_string_h::{memcmp, strerror, strlen};
use self::_strings_h::strcasecmp;
pub use self::_time_t_h::time_t;
pub use self::_types_h::{__darwin_size_t, __darwin_ssize_t, __darwin_time_t, __darwin_va_list};
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_va_list_h::va_list;
use self::bio_h::{BIO_free, BIO_new_mem_buf};
use self::crypto_h::OPENSSL_cleanup;
use self::err_h::{ERR_clear_error, ERR_error_string, ERR_peek_error, ERR_reason_error_string};
pub use self::errno_h::{__error, ECONNRESET, ENOTCONN};
use self::evp_h::EVP_PKEY_free;
pub use self::internal::__builtin_va_list;
pub use self::limits_h::INT_MAX;
use self::pem_h::PEM_read_bio_PrivateKey;
pub use self::prov_ssl_h::{TLS1_1_VERSION, TLS1_2_VERSION, TLS1_3_VERSION, TLS1_VERSION};
pub use self::socket_h::{shutdown, SHUT_RDWR};
pub use self::ssl_h::{
    SSL_CTX_check_private_key, SSL_CTX_clear_options, SSL_CTX_ctrl, SSL_CTX_free,
    SSL_CTX_get0_param, SSL_CTX_load_verify_locations, SSL_CTX_set_cipher_list,
    SSL_CTX_set_ciphersuites, SSL_CTX_set_info_callback, SSL_CTX_set_options, SSL_CTX_set_verify,
    SSL_CTX_set_verify_depth, SSL_CTX_use_PrivateKey, SSL_CTX_use_PrivateKey_file,
    SSL_CTX_use_certificate_chain_file, SSL_OP_NO_SSLv2, SSL_OP_NO_SSLv3, SSL_OP_NO_TLSv1,
    SSL_OP_NO_TLSv1_1, SSL_OP_NO_TLSv1_2, SSL_OP_NO_TLSv1_3, SSL_free, SSL_get1_peer_certificate,
    SSL_get_error, SSL_get_ex_data, SSL_read, SSL_shutdown, SSL_verify_cb, SSL_version, SSL_write,
    SSL_CB_HANDSHAKE_START, SSL_CTRL_MODE, SSL_CTRL_SET_MAX_PROTO_VERSION,
    SSL_CTRL_SET_MIN_PROTO_VERSION, SSL_ERROR_NONE, SSL_ERROR_SSL, SSL_ERROR_SYSCALL,
    SSL_ERROR_WANT_ACCEPT, SSL_ERROR_WANT_CONNECT, SSL_ERROR_WANT_READ, SSL_ERROR_WANT_WRITE,
    SSL_ERROR_WANT_X509_LOOKUP, SSL_ERROR_ZERO_RETURN, SSL_FILETYPE_PEM,
};
pub use self::stdbool_h::{false_0, true_0};
use self::string_h::strcmpeq;
pub use self::sys__types_h::__DARWIN_NULL;
use self::tls_compat_h::{
    tls_compat_cleanup, SSL_CTX_load_verify_mem, SSL_CTX_use_certificate_chain_mem,
};
pub use self::tls_h::{
    tls_config_free, tls_config_new, TLS_PROTOCOL_TLSv1_0, TLS_PROTOCOL_TLSv1_1,
    TLS_PROTOCOL_TLSv1_2, TLS_PROTOCOL_TLSv1_3, TLS_WANT_POLLIN, TLS_WANT_POLLOUT,
};
pub use self::tls_internal_h::{
    tls, tls_config, tls_configure_server, tls_conninfo, tls_error, tls_free_conninfo,
    tls_get_conninfo, tls_handshake_client, tls_handshake_server, tls_keypair,
    tls_ocsp_client_free, tls_ocsp_info, tls_ocsp_info_free, tls_ocsp_query, TLS_CLIENT,
    TLS_DO_ABORT, TLS_EOF_NO_CLOSE_NOTIFY, TLS_HANDSHAKE_COMPLETE, TLS_OCSP_CLIENT, TLS_SERVER,
    TLS_SERVER_CONN,
};
pub use self::types_h::{
    bio_st, evp_pkey_st, pem_password_cb, ssl_ctx_st, ssl_st, x509_st, x509_store_ctx_st,
    X509_VERIFY_PARAM_st, BIO, EVP_PKEY, SSL, SSL_CTX, X509, X509_STORE_CTX, X509_VERIFY_PARAM,
};
use self::unistd_h::close;
pub use self::x509_h::{X509_free, X509_FILETYPE_PEM};
pub use self::x509_vfy_h::{X509_VERIFY_PARAM_set_flags, X509_V_FLAG_NO_CHECK_TIME};
#[c2rust::src_loc = "33:1"]
static mut tls_config_default: *mut tls_config =
    ::core::ptr::null::<tls_config>() as *mut tls_config;
#[c2rust::src_loc = "35:1"]
static mut tls_initialised: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn tls_init() -> ::core::ffi::c_int {
    if tls_initialised != 0 {
        return 0 as ::core::ffi::c_int;
    }
    tls_config_default = tls_config_new();
    if tls_config_default.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    tls_initialised = 1 as ::core::ffi::c_int;
    0 as ::core::ffi::c_int
}
#[no_mangle]
#[c2rust::src_loc = "58:1"]
pub unsafe extern "C" fn tls_deinit() {
    if tls_initialised != 0 {
        tls_compat_cleanup();
        tls_config_free(tls_config_default);
        tls_config_default = ::core::ptr::null_mut::<tls_config>();
        OPENSSL_cleanup();
        tls_initialised = 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
#[c2rust::src_loc = "81:1"]
pub unsafe extern "C" fn tls_error(mut ctx: *mut tls) -> *const ::core::ffi::c_char {
    (*ctx).error.msg
}
#[c2rust::src_loc = "86:1"]
unsafe extern "C" fn tls_error_vset(
    mut error: *mut tls_error,
    mut errnum: ::core::ffi::c_int,
    mut fmt: *const ::core::ffi::c_char,
    mut ap: ::core::ffi::VaList,
) -> ::core::ffi::c_int {
    let mut errmsg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rv = -(1 as ::core::ffi::c_int);
    free((*error).msg as *mut ::core::ffi::c_void);
    (*error).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*error).num = errnum;
    if vasprintf(&raw mut errmsg, fmt, ap.as_va_list()) == -(1 as ::core::ffi::c_int) {
        errmsg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        if errnum == -(1 as ::core::ffi::c_int) {
            (*error).msg = errmsg;
            return 0 as ::core::ffi::c_int;
        }
        if asprintf(
            &raw mut (*error).msg,
            b"%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
            errmsg,
            strerror(errnum),
        ) == -(1 as ::core::ffi::c_int)
        {
            (*error).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            rv = 0 as ::core::ffi::c_int;
        }
    }
    free(errmsg as *mut ::core::ffi::c_void);
    rv
}
#[no_mangle]
#[c2rust::src_loc = "118:1"]
pub unsafe extern "C" fn tls_error_set(
    mut error: *mut tls_error,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ...
) -> ::core::ffi::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    let mut errnum: ::core::ffi::c_int = 0;
    let mut rv: ::core::ffi::c_int = 0;
    errnum = *__error();
    ap = args.clone();
    rv = tls_error_vset(error, errnum, fmt, ap.as_va_list());
    rv
}
#[no_mangle]
#[c2rust::src_loc = "132:1"]
pub unsafe extern "C" fn tls_error_setx(
    mut error: *mut tls_error,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ...
) -> ::core::ffi::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    let mut rv: ::core::ffi::c_int = 0;
    ap = args.clone();
    rv = tls_error_vset(error, -(1 as ::core::ffi::c_int), fmt, ap.as_va_list());
    rv
}
#[no_mangle]
#[c2rust::src_loc = "144:1"]
pub unsafe extern "C" fn tls_config_set_error(
    mut config: *mut tls_config,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ...
) -> ::core::ffi::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    let mut errnum: ::core::ffi::c_int = 0;
    let mut rv: ::core::ffi::c_int = 0;
    errnum = *__error();
    ap = args.clone();
    rv = tls_error_vset(&raw mut (*config).error, errnum, fmt, ap.as_va_list());
    rv
}
#[no_mangle]
#[c2rust::src_loc = "158:1"]
pub unsafe extern "C" fn tls_config_set_errorx(
    mut config: *mut tls_config,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ...
) -> ::core::ffi::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    let mut rv: ::core::ffi::c_int = 0;
    ap = args.clone();
    rv = tls_error_vset(
        &raw mut (*config).error,
        -(1 as ::core::ffi::c_int),
        fmt,
        ap.as_va_list(),
    );
    rv
}
#[no_mangle]
#[c2rust::src_loc = "170:1"]
pub unsafe extern "C" fn tls_set_error(
    mut ctx: *mut tls,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ...
) -> ::core::ffi::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    let mut errnum: ::core::ffi::c_int = 0;
    let mut rv: ::core::ffi::c_int = 0;
    errnum = *__error();
    ap = args.clone();
    rv = tls_error_vset(&raw mut (*ctx).error, errnum, fmt, ap.as_va_list());
    rv
}
#[no_mangle]
#[c2rust::src_loc = "184:1"]
pub unsafe extern "C" fn tls_set_errorx(
    mut ctx: *mut tls,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ...
) -> ::core::ffi::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    let mut rv: ::core::ffi::c_int = 0;
    ap = args.clone();
    rv = tls_error_vset(
        &raw mut (*ctx).error,
        -(1 as ::core::ffi::c_int),
        fmt,
        ap.as_va_list(),
    );
    rv
}
#[no_mangle]
#[c2rust::src_loc = "196:1"]
pub unsafe extern "C" fn tls_set_error_libssl(
    mut ctx: *mut tls,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ...
) -> ::core::ffi::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    let mut rv: ::core::ffi::c_int = 0;
    let mut msg = ::core::ptr::null::<::core::ffi::c_char>();
    let mut old = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut err: ::core::ffi::c_int = 0;
    err = ERR_peek_error() as ::core::ffi::c_int;
    if err != 0 as ::core::ffi::c_int {
        msg = ERR_reason_error_string(err as ::core::ffi::c_ulong);
    }
    ap = args.clone();
    rv = tls_error_vset(
        &raw mut (*ctx).error,
        -(1 as ::core::ffi::c_int),
        fmt,
        ap.as_va_list(),
    );
    if rv != 0 as ::core::ffi::c_int || msg.is_null() {
        return rv;
    }
    old = (*ctx).error.msg;
    (*ctx).error.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if asprintf(
        &raw mut (*ctx).error.msg,
        b"%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
        old,
        msg,
    ) == -(1 as ::core::ffi::c_int)
    {
        (*ctx).error.msg = old;
    } else {
        free(old as *mut ::core::ffi::c_void);
    }
    0 as ::core::ffi::c_int
}
#[no_mangle]
#[c2rust::src_loc = "224:1"]
pub unsafe extern "C" fn tls_new() -> *mut tls {
    let mut ctx = ::core::ptr::null_mut::<tls>();
    ctx = calloc(1 as size_t, ::core::mem::size_of::<tls>() as size_t) as *mut tls;
    if ctx.is_null() {
        return ::core::ptr::null_mut::<tls>();
    }
    (*ctx).config = tls_config_default;
    tls_reset(ctx);
    ctx
}
#[no_mangle]
#[c2rust::src_loc = "238:1"]
pub unsafe extern "C" fn tls_configure(
    mut ctx: *mut tls,
    mut config: *mut tls_config,
) -> ::core::ffi::c_int {
    if config.is_null() {
        config = tls_config_default;
    }
    (*ctx).config = config;
    if (*ctx).flags & TLS_SERVER as uint32_t != 0 as uint32_t {
        return tls_configure_server(ctx);
    }
    0 as ::core::ffi::c_int
}
#[no_mangle]
#[c2rust::src_loc = "251:1"]
pub unsafe extern "C" fn tls_configure_keypair(
    mut ctx: *mut tls,
    mut ssl_ctx: *mut SSL_CTX,
    mut keypair: *mut tls_keypair,
    mut required: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut pkey = ::core::ptr::null_mut::<EVP_PKEY>();
    let mut cert = ::core::ptr::null_mut::<X509>();
    let mut bio = ::core::ptr::null_mut::<BIO>();
    if required == 0
        && (*keypair).cert_mem.is_null()
        && (*keypair).key_mem.is_null()
        && (*keypair).cert_file.is_null()
        && (*keypair).key_file.is_null()
    {
        return 0 as ::core::ffi::c_int;
    }
    if !(*keypair).cert_mem.is_null() {
        if (*keypair).cert_len > INT_MAX as size_t {
            tls_set_errorx(
                ctx,
                b"certificate too long\0" as *const u8 as *const ::core::ffi::c_char,
            );
            current_block = 8709731486888603138;
        } else if SSL_CTX_use_certificate_chain_mem(
            ssl_ctx,
            (*keypair).cert_mem as *mut ::core::ffi::c_void,
            (*keypair).cert_len as ::core::ffi::c_int,
        ) != 1 as ::core::ffi::c_int
        {
            tls_set_errorx(
                ctx,
                b"failed to load certificate\0" as *const u8 as *const ::core::ffi::c_char,
            );
            current_block = 8709731486888603138;
        } else {
            cert = ::core::ptr::null_mut::<X509>();
            current_block = 17216689946888361452;
        }
    } else {
        current_block = 17216689946888361452;
    }
    if current_block == 17216689946888361452 {
        if !(*keypair).key_mem.is_null() {
            if (*keypair).key_len > INT_MAX as size_t {
                tls_set_errorx(
                    ctx,
                    b"key too long\0" as *const u8 as *const ::core::ffi::c_char,
                );
                current_block = 8709731486888603138;
            } else {
                bio = BIO_new_mem_buf(
                    (*keypair).key_mem as *const ::core::ffi::c_void,
                    (*keypair).key_len as ::core::ffi::c_int,
                );
                if bio.is_null() {
                    tls_set_errorx(
                        ctx,
                        b"failed to create buffer\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    current_block = 8709731486888603138;
                } else {
                    pkey = PEM_read_bio_PrivateKey(
                        bio,
                        ::core::ptr::null_mut::<*mut EVP_PKEY>(),
                        None,
                        NULL,
                    );
                    if pkey.is_null() {
                        tls_set_errorx(
                            ctx,
                            b"failed to read private key\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        current_block = 8709731486888603138;
                    } else if SSL_CTX_use_PrivateKey(ssl_ctx, pkey) != 1 as ::core::ffi::c_int {
                        tls_set_errorx(
                            ctx,
                            b"failed to load private key\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        current_block = 8709731486888603138;
                    } else {
                        BIO_free(bio);
                        bio = ::core::ptr::null_mut::<BIO>();
                        EVP_PKEY_free(pkey);
                        pkey = ::core::ptr::null_mut::<EVP_PKEY>();
                        current_block = 4808432441040389987;
                    }
                }
            }
        } else {
            current_block = 4808432441040389987;
        }
        match current_block {
            8709731486888603138 => {}
            _ => {
                if !(*keypair).cert_file.is_null() {
                    if SSL_CTX_use_certificate_chain_file(ssl_ctx, (*keypair).cert_file)
                        != 1 as ::core::ffi::c_int
                    {
                        let mut errstr =
                            b"unknown error\0" as *const u8 as *const ::core::ffi::c_char;
                        let mut err: ::core::ffi::c_ulong = 0;
                        err = ERR_peek_error();
                        if err != 0 as ::core::ffi::c_ulong {
                            errstr = ERR_reason_error_string(err);
                        }
                        tls_set_errorx(
                            ctx,
                            b"failed to load certificate file \"%s\": %s\0" as *const u8
                                as *const ::core::ffi::c_char,
                            (*keypair).cert_file,
                            errstr,
                        );
                        current_block = 8709731486888603138;
                    } else {
                        current_block = 6669252993407410313;
                    }
                } else {
                    current_block = 6669252993407410313;
                }
                match current_block {
                    8709731486888603138 => {}
                    _ => {
                        if !(*keypair).key_file.is_null() {
                            if SSL_CTX_use_PrivateKey_file(
                                ssl_ctx,
                                (*keypair).key_file,
                                SSL_FILETYPE_PEM,
                            ) != 1 as ::core::ffi::c_int
                            {
                                let mut errstr_0 =
                                    b"unknown error\0" as *const u8 as *const ::core::ffi::c_char;
                                let mut err_0: ::core::ffi::c_ulong = 0;
                                err_0 = ERR_peek_error();
                                if err_0 != 0 as ::core::ffi::c_ulong {
                                    errstr_0 = ERR_reason_error_string(err_0);
                                }
                                tls_set_errorx(
                                    ctx,
                                    b"failed to load private key file \"%s\": %s\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    (*keypair).key_file,
                                    errstr_0,
                                );
                                current_block = 8709731486888603138;
                            } else {
                                current_block = 5494826135382683477;
                            }
                        } else {
                            current_block = 5494826135382683477;
                        }
                        match current_block {
                            8709731486888603138 => {}
                            _ => {
                                if SSL_CTX_check_private_key(ssl_ctx) != 1 as ::core::ffi::c_int {
                                    tls_set_errorx(
                                        ctx,
                                        b"private/public key mismatch\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                } else {
                                    return 0 as ::core::ffi::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    EVP_PKEY_free(pkey);
    X509_free(cert);
    BIO_free(bio);
    1 as ::core::ffi::c_int
}
#[c2rust::src_loc = "346:1"]
unsafe extern "C" fn tls_info_callback(
    mut ssl: *const SSL,
    mut where_0: ::core::ffi::c_int,
    mut _rc: ::core::ffi::c_int,
) {
    let mut ctx = SSL_get_ex_data(ssl, 0 as ::core::ffi::c_int) as *mut tls;
    if SSL_version(ssl) < TLS1_3_VERSION
        && where_0 & SSL_CB_HANDSHAKE_START != 0
        && (*ctx).state & TLS_HANDSHAKE_COMPLETE as uint32_t != 0
    {
        (*ctx).state |= TLS_DO_ABORT as uint32_t;
    }
}
#[c2rust::src_loc = "375:1"]
unsafe extern "C" fn tls_do_abort(mut ctx: *mut tls) -> ::core::ffi::c_int {
    let mut ssl_ret: ::core::ffi::c_int = 0;
    let mut rv: ::core::ffi::c_int = 0;
    ssl_ret = SSL_shutdown((*ctx).ssl_conn);
    if ssl_ret < 0 as ::core::ffi::c_int {
        rv = tls_ssl_error(
            ctx,
            (*ctx).ssl_conn,
            ssl_ret,
            b"shutdown\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if rv == TLS_WANT_POLLIN || rv == TLS_WANT_POLLOUT {
            return rv;
        }
    }
    tls_set_errorx(
        ctx,
        b"unexpected handshake, closing connection\0" as *const u8 as *const ::core::ffi::c_char,
    );
    -(1 as ::core::ffi::c_int)
}
#[c2rust::src_loc = "391:1"]
unsafe extern "C" fn get_min_ssl_version(mut protocols: uint32_t) -> ::core::ffi::c_int {
    if protocols & TLS_PROTOCOL_TLSv1_0 as uint32_t != 0 {
        return TLS1_VERSION;
    }
    if protocols & TLS_PROTOCOL_TLSv1_1 as uint32_t != 0 {
        return TLS1_1_VERSION;
    }
    if protocols & TLS_PROTOCOL_TLSv1_2 as uint32_t != 0 {
        return TLS1_2_VERSION;
    }
    if protocols & TLS_PROTOCOL_TLSv1_3 as uint32_t != 0 {
        return TLS1_3_VERSION;
    }
    TLS1_VERSION
}
#[c2rust::src_loc = "404:1"]
unsafe extern "C" fn get_max_ssl_version(mut protocols: uint32_t) -> ::core::ffi::c_int {
    if protocols & TLS_PROTOCOL_TLSv1_3 as uint32_t != 0 {
        return TLS1_3_VERSION;
    }
    if protocols & TLS_PROTOCOL_TLSv1_2 as uint32_t != 0 {
        return TLS1_2_VERSION;
    }
    if protocols & TLS_PROTOCOL_TLSv1_1 as uint32_t != 0 {
        return TLS1_1_VERSION;
    }
    if protocols & TLS_PROTOCOL_TLSv1_0 as uint32_t != 0 {
        return TLS1_VERSION;
    }
    TLS1_3_VERSION
}
#[no_mangle]
#[c2rust::src_loc = "418:1"]
pub unsafe extern "C" fn tls_configure_ssl(mut ctx: *mut tls) -> ::core::ffi::c_int {
    let mut current_block: u64;
    SSL_CTX_ctrl(
        (*ctx).ssl_ctx,
        SSL_CTRL_MODE,
        0x1 as ::core::ffi::c_long,
        NULL,
    );
    SSL_CTX_ctrl(
        (*ctx).ssl_ctx,
        SSL_CTRL_MODE,
        0x2 as ::core::ffi::c_long,
        NULL,
    );
    SSL_CTX_ctrl(
        (*ctx).ssl_ctx,
        SSL_CTRL_MODE,
        0x10 as ::core::ffi::c_long,
        NULL,
    );
    SSL_CTX_set_options((*ctx).ssl_ctx, SSL_OP_NO_SSLv2 as uint64_t);
    SSL_CTX_set_options((*ctx).ssl_ctx, SSL_OP_NO_SSLv3);
    SSL_CTX_clear_options((*ctx).ssl_ctx, SSL_OP_NO_TLSv1);
    SSL_CTX_clear_options((*ctx).ssl_ctx, SSL_OP_NO_TLSv1_1);
    SSL_CTX_clear_options((*ctx).ssl_ctx, SSL_OP_NO_TLSv1_2);
    SSL_CTX_clear_options((*ctx).ssl_ctx, SSL_OP_NO_TLSv1_3);
    SSL_CTX_ctrl(
        (*ctx).ssl_ctx,
        SSL_CTRL_SET_MIN_PROTO_VERSION,
        get_min_ssl_version((*(*ctx).config).protocols) as ::core::ffi::c_long,
        NULL,
    );
    SSL_CTX_ctrl(
        (*ctx).ssl_ctx,
        SSL_CTRL_SET_MAX_PROTO_VERSION,
        get_max_ssl_version((*(*ctx).config).protocols) as ::core::ffi::c_long,
        NULL,
    );
    if (*(*ctx).config).protocols & TLS_PROTOCOL_TLSv1_0 as uint32_t == 0 as uint32_t {
        SSL_CTX_set_options((*ctx).ssl_ctx, SSL_OP_NO_TLSv1);
    }
    if (*(*ctx).config).protocols & TLS_PROTOCOL_TLSv1_1 as uint32_t == 0 as uint32_t {
        SSL_CTX_set_options((*ctx).ssl_ctx, SSL_OP_NO_TLSv1_1);
    }
    if (*(*ctx).config).protocols & TLS_PROTOCOL_TLSv1_2 as uint32_t == 0 as uint32_t {
        SSL_CTX_set_options((*ctx).ssl_ctx, SSL_OP_NO_TLSv1_2);
    }
    if (*(*ctx).config).protocols & TLS_PROTOCOL_TLSv1_3 as uint32_t == 0 as uint32_t {
        SSL_CTX_set_options((*ctx).ssl_ctx, SSL_OP_NO_TLSv1_3);
    }
    if !(*(*ctx).config).ciphers.is_null() {
        if strcasecmp(
            (*(*ctx).config).ciphers,
            b"default\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
            && strcasecmp(
                (*(*ctx).config).ciphers,
                b"secure\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            && strcasecmp(
                (*(*ctx).config).ciphers,
                b"normal\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            && strcasecmp(
                (*(*ctx).config).ciphers,
                b"fast\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            if SSL_CTX_set_cipher_list((*ctx).ssl_ctx, (*(*ctx).config).ciphers)
                != 1 as ::core::ffi::c_int
            {
                tls_set_errorx(
                    ctx,
                    b"failed to set ciphers\0" as *const u8 as *const ::core::ffi::c_char,
                );
                current_block = 8237318671472554270;
            } else {
                current_block = 17833034027772472439;
            }
        } else {
            current_block = 17833034027772472439;
        }
    } else {
        current_block = 17833034027772472439;
    }
    if current_block == 17833034027772472439 {
        if (*(*ctx).config).protocols & TLS_PROTOCOL_TLSv1_3 as uint32_t != 0
            && !(*(*ctx).config).cipher_suites.is_null()
            && strlen((*(*ctx).config).cipher_suites) > 0 as size_t
        {
            if SSL_CTX_set_ciphersuites((*ctx).ssl_ctx, (*(*ctx).config).cipher_suites)
                != 1 as ::core::ffi::c_int
            {
                tls_set_errorx(
                    ctx,
                    b"failed to set the TLSv1.3 cipher suites\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                current_block = 8237318671472554270;
            } else {
                current_block = 224731115979188411;
            }
        } else {
            current_block = 224731115979188411;
        }
        match current_block {
            8237318671472554270 => {}
            _ => {
                SSL_CTX_set_info_callback(
                    (*ctx).ssl_ctx,
                    Some(
                        tls_info_callback
                            as unsafe extern "C" fn(
                                *const SSL,
                                ::core::ffi::c_int,
                                ::core::ffi::c_int,
                            ) -> (),
                    ),
                );
                if (*(*ctx).config).verify_time == 0 as ::core::ffi::c_int {
                    let mut vfp = SSL_CTX_get0_param((*ctx).ssl_ctx);
                    X509_VERIFY_PARAM_set_flags(
                        vfp,
                        X509_V_FLAG_NO_CHECK_TIME as ::core::ffi::c_ulong,
                    );
                }
                return 0 as ::core::ffi::c_int;
            }
        }
    }
    -(1 as ::core::ffi::c_int)
}
#[no_mangle]
#[c2rust::src_loc = "493:1"]
pub unsafe extern "C" fn tls_configure_ssl_verify(
    mut ctx: *mut tls,
    mut verify: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    SSL_CTX_set_verify((*ctx).ssl_ctx, verify, None);
    if !(*(*ctx).config).ca_mem.is_null() {
        if (*(*ctx).config).ca_len > INT_MAX as size_t {
            tls_set_errorx(
                ctx,
                b"ca too long\0" as *const u8 as *const ::core::ffi::c_char,
            );
            current_block = 16323917377200177085;
        } else if SSL_CTX_load_verify_mem(
            (*ctx).ssl_ctx,
            (*(*ctx).config).ca_mem as *mut ::core::ffi::c_void,
            (*(*ctx).config).ca_len as ::core::ffi::c_int,
        ) != 1 as ::core::ffi::c_int
        {
            tls_set_errorx(
                ctx,
                b"ssl verify memory setup failure\0" as *const u8 as *const ::core::ffi::c_char,
            );
            current_block = 16323917377200177085;
        } else {
            current_block = 13109137661213826276;
        }
    } else if SSL_CTX_load_verify_locations(
        (*ctx).ssl_ctx,
        (*(*ctx).config).ca_file,
        (*(*ctx).config).ca_path,
    ) != 1 as ::core::ffi::c_int
    {
        let mut errstr = b"unknown error\0" as *const u8 as *const ::core::ffi::c_char;
        let mut err: ::core::ffi::c_ulong = 0;
        err = ERR_peek_error();
        if err != 0 as ::core::ffi::c_ulong {
            errstr = ERR_reason_error_string(err);
        }
        tls_set_errorx(
            ctx,
            b"failed to load CA: %s\0" as *const u8 as *const ::core::ffi::c_char,
            errstr,
        );
        current_block = 16323917377200177085;
    } else {
        current_block = 13109137661213826276;
    }
    match current_block {
        16323917377200177085 => -(1 as ::core::ffi::c_int),
        _ => {
            if (*(*ctx).config).verify_depth >= 0 as ::core::ffi::c_int {
                SSL_CTX_set_verify_depth((*ctx).ssl_ctx, (*(*ctx).config).verify_depth);
            }
            0 as ::core::ffi::c_int
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "529:1"]
pub unsafe extern "C" fn usual_tls_free(mut ctx: *mut tls) {
    if ctx.is_null() {
        return;
    }
    tls_reset(ctx);
    free(ctx as *mut ::core::ffi::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "537:1"]
pub unsafe extern "C" fn tls_reset(mut ctx: *mut tls) {
    SSL_CTX_free((*ctx).ssl_ctx);
    SSL_free((*ctx).ssl_conn);
    X509_free((*ctx).ssl_peer_cert);
    (*ctx).ssl_conn = ::core::ptr::null_mut::<SSL>();
    (*ctx).ssl_ctx = ::core::ptr::null_mut::<SSL_CTX>();
    (*ctx).ssl_peer_cert = ::core::ptr::null_mut::<X509>();
    (*ctx).socket = -(1 as ::core::ffi::c_int);
    (*ctx).state = 0 as uint32_t;
    free((*ctx).servername as *mut ::core::ffi::c_void);
    (*ctx).servername = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*ctx).error.msg as *mut ::core::ffi::c_void);
    (*ctx).error.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*ctx).error.num = -(1 as ::core::ffi::c_int);
    tls_free_conninfo((*ctx).conninfo);
    free((*ctx).conninfo as *mut ::core::ffi::c_void);
    (*ctx).conninfo = ::core::ptr::null_mut::<tls_conninfo>();
    (*ctx).used_dh_bits = 0 as ::core::ffi::c_int;
    (*ctx).used_ecdh_nid = 0 as ::core::ffi::c_int;
    tls_ocsp_info_free((*ctx).ocsp_info);
    (*ctx).ocsp_info = ::core::ptr::null_mut::<tls_ocsp_info>();
    (*ctx).ocsp_result = ::core::ptr::null::<::core::ffi::c_char>();
    if (*ctx).flags & TLS_OCSP_CLIENT as uint32_t != 0 {
        tls_ocsp_client_free(ctx);
    }
}
#[no_mangle]
#[c2rust::src_loc = "572:1"]
pub unsafe extern "C" fn tls_ssl_error(
    mut ctx: *mut tls,
    mut ssl_conn: *mut SSL,
    mut ssl_ret: ::core::ffi::c_int,
    mut prefix: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut errstr = b"unknown error\0" as *const u8 as *const ::core::ffi::c_char;
    let mut err: ::core::ffi::c_ulong = 0;
    let mut ssl_err: ::core::ffi::c_int = 0;
    ssl_err = SSL_get_error(ssl_conn, ssl_ret);
    match ssl_err {
        SSL_ERROR_NONE | SSL_ERROR_ZERO_RETURN => 0 as ::core::ffi::c_int,
        SSL_ERROR_WANT_READ => -(2 as ::core::ffi::c_int),
        SSL_ERROR_WANT_WRITE => -(3 as ::core::ffi::c_int),
        SSL_ERROR_SYSCALL => {
            err = ERR_peek_error();
            if err != 0 as ::core::ffi::c_ulong {
                errstr = ERR_error_string(err, ::core::ptr::null_mut::<::core::ffi::c_char>());
            } else if ssl_ret == 0 as ::core::ffi::c_int {
                if (*ctx).state & TLS_HANDSHAKE_COMPLETE as uint32_t != 0 as uint32_t {
                    (*ctx).state |= TLS_EOF_NO_CLOSE_NOTIFY as uint32_t;
                    return 0 as ::core::ffi::c_int;
                }
                errstr = b"unexpected EOF\0" as *const u8 as *const ::core::ffi::c_char;
            } else if ssl_ret == -(1 as ::core::ffi::c_int) {
                errstr = strerror(*__error());
            }
            tls_set_errorx(
                ctx,
                b"%s failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
                prefix,
                errstr,
            );
            -(1 as ::core::ffi::c_int)
        }
        SSL_ERROR_SSL => {
            err = ERR_peek_error();
            if err != 0 as ::core::ffi::c_ulong {
                errstr = ERR_error_string(err, ::core::ptr::null_mut::<::core::ffi::c_char>());
            }
            tls_set_errorx(
                ctx,
                b"%s failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
                prefix,
                errstr,
            );
            -(1 as ::core::ffi::c_int)
        }
        SSL_ERROR_WANT_CONNECT | SSL_ERROR_WANT_ACCEPT | SSL_ERROR_WANT_X509_LOOKUP | _ => {
            tls_set_errorx(
                ctx,
                b"%s failed (%i)\0" as *const u8 as *const ::core::ffi::c_char,
                prefix,
                ssl_err,
            );
            -(1 as ::core::ffi::c_int)
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "621:1"]
pub unsafe extern "C" fn tls_handshake(mut ctx: *mut tls) -> ::core::ffi::c_int {
    let mut rv = -(1 as ::core::ffi::c_int);
    if (*ctx).flags & (TLS_CLIENT | TLS_SERVER_CONN) as uint32_t == 0 as uint32_t {
        tls_set_errorx(
            ctx,
            b"invalid operation for context\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if !((*ctx).conninfo.is_null() && {
        (*ctx).conninfo = calloc(
            1 as size_t,
            ::core::mem::size_of::<tls_conninfo>() as size_t,
        ) as *mut tls_conninfo;
        (*ctx).conninfo.is_null()
    }) {
        if (*ctx).flags & TLS_CLIENT as uint32_t != 0 as uint32_t {
            rv = tls_handshake_client(ctx);
        } else if (*ctx).flags & TLS_SERVER_CONN as uint32_t != 0 as uint32_t {
            rv = tls_handshake_server(ctx);
        }
        if rv == 0 as ::core::ffi::c_int {
            (*ctx).ssl_peer_cert = SSL_get1_peer_certificate((*ctx).ssl_conn);
            if tls_get_conninfo(ctx) == -(1 as ::core::ffi::c_int) {
                rv = -(1 as ::core::ffi::c_int);
            }
        }
    }
    *__error() = 0 as ::core::ffi::c_int;
    rv
}
#[no_mangle]
#[c2rust::src_loc = "650:1"]
pub unsafe extern "C" fn tls_read(
    mut ctx: *mut tls,
    mut buf: *mut ::core::ffi::c_void,
    mut buflen: size_t,
) -> ssize_t {
    let mut current_block: u64;
    let mut rv: ssize_t = -(1 as ::core::ffi::c_int) as ssize_t;
    let mut ssl_ret: ::core::ffi::c_int = 0;
    if (*ctx).state & TLS_DO_ABORT as uint32_t != 0 {
        rv = tls_do_abort(ctx) as ssize_t;
    } else {
        if (*ctx).state & TLS_HANDSHAKE_COMPLETE as uint32_t == 0 as uint32_t {
            rv = tls_handshake(ctx) as ssize_t;
            if rv != 0 as ssize_t {
                current_block = 17367576438093281244;
            } else {
                current_block = 820271813250567934;
            }
        } else {
            current_block = 820271813250567934;
        }
        match current_block {
            17367576438093281244 => {}
            _ => {
                if buflen > INT_MAX as size_t {
                    tls_set_errorx(
                        ctx,
                        b"buflen too long\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else {
                    ERR_clear_error();
                    ssl_ret = SSL_read((*ctx).ssl_conn, buf, buflen as ::core::ffi::c_int);
                    if ssl_ret > 0 as ::core::ffi::c_int {
                        rv = ssl_ret as ssize_t;
                    } else {
                        rv = tls_ssl_error(
                            ctx,
                            (*ctx).ssl_conn,
                            ssl_ret,
                            b"read\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as ssize_t;
                    }
                }
            }
        }
    }
    *__error() = 0 as ::core::ffi::c_int;
    rv
}
#[no_mangle]
#[c2rust::src_loc = "683:1"]
pub unsafe extern "C" fn tls_write(
    mut ctx: *mut tls,
    mut buf: *const ::core::ffi::c_void,
    mut buflen: size_t,
) -> ssize_t {
    let mut current_block: u64;
    let mut rv: ssize_t = -(1 as ::core::ffi::c_int) as ssize_t;
    let mut ssl_ret: ::core::ffi::c_int = 0;
    if (*ctx).state & TLS_DO_ABORT as uint32_t != 0 {
        rv = tls_do_abort(ctx) as ssize_t;
    } else {
        if (*ctx).state & TLS_HANDSHAKE_COMPLETE as uint32_t == 0 as uint32_t {
            rv = tls_handshake(ctx) as ssize_t;
            if rv != 0 as ssize_t {
                current_block = 5132567788168539391;
            } else {
                current_block = 820271813250567934;
            }
        } else {
            current_block = 820271813250567934;
        }
        match current_block {
            5132567788168539391 => {}
            _ => {
                if buflen > INT_MAX as size_t {
                    tls_set_errorx(
                        ctx,
                        b"buflen too long\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else {
                    ERR_clear_error();
                    ssl_ret = SSL_write((*ctx).ssl_conn, buf, buflen as ::core::ffi::c_int);
                    if ssl_ret > 0 as ::core::ffi::c_int {
                        rv = ssl_ret as ssize_t;
                    } else {
                        rv = tls_ssl_error(
                            ctx,
                            (*ctx).ssl_conn,
                            ssl_ret,
                            b"write\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as ssize_t;
                    }
                }
            }
        }
    }
    *__error() = 0 as ::core::ffi::c_int;
    rv
}
#[no_mangle]
#[c2rust::src_loc = "716:1"]
pub unsafe extern "C" fn tls_close(mut ctx: *mut tls) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ssl_ret: ::core::ffi::c_int = 0;
    let mut rv = 0 as ::core::ffi::c_int;
    if (*ctx).flags & (TLS_CLIENT | TLS_SERVER_CONN) as uint32_t == 0 as uint32_t {
        tls_set_errorx(
            ctx,
            b"invalid operation for context\0" as *const u8 as *const ::core::ffi::c_char,
        );
        rv = -(1 as ::core::ffi::c_int);
    } else {
        if !(*ctx).ssl_conn.is_null() {
            ERR_clear_error();
            ssl_ret = SSL_shutdown((*ctx).ssl_conn);
            if ssl_ret < 0 as ::core::ffi::c_int {
                rv = tls_ssl_error(
                    ctx,
                    (*ctx).ssl_conn,
                    ssl_ret,
                    b"shutdown\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if rv == TLS_WANT_POLLIN || rv == TLS_WANT_POLLOUT {
                    current_block = 12672191506162062702;
                } else {
                    current_block = 13513818773234778473;
                }
            } else {
                current_block = 13513818773234778473;
            }
        } else {
            current_block = 13513818773234778473;
        }
        match current_block {
            12672191506162062702 => {}
            _ => {
                if (*ctx).socket != -(1 as ::core::ffi::c_int) {
                    if shutdown((*ctx).socket, SHUT_RDWR) != 0 as ::core::ffi::c_int
                        && rv == 0 as ::core::ffi::c_int
                        && *__error() != ENOTCONN
                        && *__error() != ECONNRESET
                    {
                        tls_set_error(
                            ctx,
                            b"shutdown\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        rv = -(1 as ::core::ffi::c_int);
                    }
                    if close((*ctx).socket) != 0 as ::core::ffi::c_int
                        && rv == 0 as ::core::ffi::c_int
                    {
                        tls_set_error(ctx, b"close\0" as *const u8 as *const ::core::ffi::c_char);
                        rv = -(1 as ::core::ffi::c_int);
                    }
                    (*ctx).socket = -(1 as ::core::ffi::c_int);
                }
                if (*ctx).state & TLS_EOF_NO_CLOSE_NOTIFY as uint32_t != 0 as uint32_t {
                    tls_set_errorx(
                        ctx,
                        b"EOF without close notify\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    rv = -(1 as ::core::ffi::c_int);
                }
            }
        }
    }
    *__error() = 0 as ::core::ffi::c_int;
    rv
}
#[c2rust::src_loc = "766:1"]
unsafe extern "C" fn tls_mem_equal(
    mut mem1: *mut ::core::ffi::c_char,
    mut mem2: *mut ::core::ffi::c_char,
    mut len1: size_t,
    mut len2: size_t,
) -> bool {
    if len1 != len2 {
        return false_0 != 0;
    }
    if !mem1.is_null()
        && !mem2.is_null()
        && memcmp(
            mem1 as *const ::core::ffi::c_void,
            mem2 as *const ::core::ffi::c_void,
            len1,
        ) != 0 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    true_0 != 0
}
#[c2rust::src_loc = "775:1"]
unsafe extern "C" fn tls_keypair_equal(
    mut tkp1: *mut tls_keypair,
    mut tkp2: *mut tls_keypair,
) -> bool {
    if !strcmpeq((*tkp1).cert_file, (*tkp2).cert_file) {
        return false_0 != 0;
    }
    if !tls_mem_equal(
        (*tkp1).cert_mem,
        (*tkp2).cert_mem,
        (*tkp1).cert_len,
        (*tkp2).cert_len,
    ) {
        return false_0 != 0;
    }
    if !strcmpeq((*tkp1).key_file, (*tkp2).key_file) {
        return false_0 != 0;
    }
    if !tls_mem_equal(
        (*tkp1).key_mem,
        (*tkp2).key_mem,
        (*tkp1).key_len,
        (*tkp2).key_len,
    ) {
        return false_0 != 0;
    }
    true_0 != 0
}
#[no_mangle]
#[c2rust::src_loc = "788:1"]
pub unsafe extern "C" fn tls_keypair_list_equal(
    mut tkp1: *mut tls_keypair,
    mut tkp2: *mut tls_keypair,
) -> bool {
    while !tkp1.is_null() && !tkp2.is_null() {
        if !tls_keypair_equal(tkp1, tkp2) {
            return false_0 != 0;
        }
        tkp1 = (*tkp1).next;
        tkp2 = (*tkp2).next;
    }
    tkp1.is_null() && tkp2.is_null()
}
#[no_mangle]
#[c2rust::src_loc = "798:1"]
pub unsafe extern "C" fn tls_config_equal(
    mut tc1: *mut tls_config,
    mut tc2: *mut tls_config,
) -> bool {
    if tc1.is_null() && tc2.is_null() {
        return true_0 != 0;
    }
    if tc1.is_null() && !tc2.is_null() {
        return false_0 != 0;
    }
    if !tc1.is_null() && tc2.is_null() {
        return false_0 != 0;
    }
    if !strcmpeq((*tc1).ca_file, (*tc2).ca_file) {
        return false_0 != 0;
    }
    if !strcmpeq((*tc1).ca_path, (*tc2).ca_path) {
        return false_0 != 0;
    }
    if !tls_mem_equal((*tc1).ca_mem, (*tc2).ca_mem, (*tc1).ca_len, (*tc2).ca_len) {
        return false_0 != 0;
    }
    if !strcmpeq((*tc1).ciphers, (*tc2).ciphers) {
        return false_0 != 0;
    }
    if (*tc1).ciphers_server != (*tc2).ciphers_server {
        return false_0 != 0;
    }
    if (*tc1).dheparams != (*tc2).dheparams {
        return false_0 != 0;
    }
    if (*tc1).ecdhecurve != (*tc2).ecdhecurve {
        return false_0 != 0;
    }
    if !tls_keypair_list_equal((*tc1).keypair, (*tc2).keypair) {
        return false_0 != 0;
    }
    if !strcmpeq((*tc1).ocsp_file, (*tc2).ocsp_file) {
        return false_0 != 0;
    }
    if !tls_mem_equal(
        (*tc1).ocsp_mem,
        (*tc2).ocsp_mem,
        (*tc1).ocsp_len,
        (*tc2).ocsp_len,
    ) {
        return false_0 != 0;
    }
    if (*tc1).protocols != (*tc2).protocols {
        return false_0 != 0;
    }
    if (*tc1).verify_cert != (*tc2).verify_cert {
        return false_0 != 0;
    }
    if (*tc1).verify_client != (*tc2).verify_client {
        return false_0 != 0;
    }
    if (*tc1).verify_depth != (*tc2).verify_depth {
        return false_0 != 0;
    }
    if (*tc1).verify_name != (*tc2).verify_name {
        return false_0 != 0;
    }
    if (*tc1).verify_time != (*tc2).verify_time {
        return false_0 != 0;
    }
    true_0 != 0
}
