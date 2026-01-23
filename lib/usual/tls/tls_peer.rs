#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:19"]
pub mod _types_h {
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
    #[c2rust::src_loc = "119:1"]
    pub type __darwin_time_t = ::core::ffi::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:19"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_time_t.h:19"]
pub mod _time_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type time_t = __darwin_time_t;
    use super::_types_h::__darwin_time_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:19"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = u32;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/tls/tls_internal.h:27"]
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
    use super::_size_t_h::size_t;
    use super::_time_t_h::time_t;
    use super::_uint32_t_h::uint32_t;
    use super::types_h::{SSL, SSL_CTX, X509};
    extern "C" {
        #[c2rust::src_loc = "94:1"]
        pub type tls_ocsp_query;
        #[c2rust::src_loc = "133:1"]
        pub fn tls_check_name(
            ctx: *mut tls,
            cert: *mut X509,
            servername: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/types.h:19"]
pub mod types_h {
    #[c2rust::src_loc = "169:1"]
    pub type X509 = x509_st;
    #[c2rust::src_loc = "197:1"]
    pub type SSL_CTX = ssl_ctx_st;
    #[c2rust::src_loc = "196:1"]
    pub type SSL = ssl_st;
    extern "C" {
        #[c2rust::src_loc = "169:9"]
        pub type x509_st;
        #[c2rust::src_loc = "197:9"]
        pub type ssl_ctx_st;
        #[c2rust::src_loc = "196:9"]
        pub type ssl_st;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:19"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:19"]
pub mod sys__types_h {
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
pub use self::_time_t_h::time_t;
pub use self::_types_h::{__darwin_size_t, __darwin_time_t};
pub use self::_uint32_t_h::uint32_t;
pub use self::sys__types_h::__DARWIN_NULL;
pub use self::tls_internal_h::{
    tls, tls_check_name, tls_config, tls_conninfo, tls_error, tls_keypair, tls_ocsp_info,
    tls_ocsp_query,
};
pub use self::types_h::{ssl_ctx_st, ssl_st, x509_st, SSL, SSL_CTX, X509};
#[no_mangle]
#[c2rust::src_loc = "29:1"]
pub unsafe extern "C" fn tls_peer_cert_hash(mut ctx: *mut tls) -> *const ::core::ffi::c_char {
    if !(*ctx).conninfo.is_null() {
        return (*(*ctx).conninfo).hash;
    }
    ::core::ptr::null::<::core::ffi::c_char>()
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn tls_peer_cert_issuer(mut ctx: *mut tls) -> *const ::core::ffi::c_char {
    if !(*ctx).conninfo.is_null() {
        return (*(*ctx).conninfo).issuer;
    }
    ::core::ptr::null::<::core::ffi::c_char>()
}
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn tls_peer_cert_subject(mut ctx: *mut tls) -> *const ::core::ffi::c_char {
    if !(*ctx).conninfo.is_null() {
        return (*(*ctx).conninfo).subject;
    }
    ::core::ptr::null::<::core::ffi::c_char>()
}
#[no_mangle]
#[c2rust::src_loc = "49:1"]
pub unsafe extern "C" fn tls_peer_cert_provided(mut ctx: *mut tls) -> ::core::ffi::c_int {
    ((*ctx).ssl_peer_cert != NULL as *mut X509) as ::core::ffi::c_int
}
#[no_mangle]
#[c2rust::src_loc = "54:1"]
pub unsafe extern "C" fn tls_peer_cert_contains_name(
    mut ctx: *mut tls,
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if (*ctx).ssl_peer_cert.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (tls_check_name(ctx, (*ctx).ssl_peer_cert, name) == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int
}
#[no_mangle]
#[c2rust::src_loc = "62:1"]
pub unsafe extern "C" fn tls_peer_cert_notbefore(mut ctx: *mut tls) -> time_t {
    if (*ctx).ssl_peer_cert.is_null() {
        return -(1 as ::core::ffi::c_int) as time_t;
    }
    if (*ctx).conninfo.is_null() {
        return -(1 as ::core::ffi::c_int) as time_t;
    }
    (*(*ctx).conninfo).notbefore
}
#[no_mangle]
#[c2rust::src_loc = "71:1"]
pub unsafe extern "C" fn tls_peer_cert_notafter(mut ctx: *mut tls) -> time_t {
    if (*ctx).ssl_peer_cert.is_null() {
        return -(1 as ::core::ffi::c_int) as time_t;
    }
    if (*ctx).conninfo.is_null() {
        return -(1 as ::core::ffi::c_int) as time_t;
    }
    (*(*ctx).conninfo).notafter
}
