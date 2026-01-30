
pub mod _types_h {
    
    pub type __darwin_size_t = usize;
    
    pub type __darwin_time_t = ::core::ffi::c_long;
}

pub mod _size_t_h {
    
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}

pub mod _time_t_h {
    
    pub type time_t = __darwin_time_t;
    use super::_types_h::__darwin_time_t;
}

pub mod _uint32_t_h {
    
    pub type uint32_t = u32;
}

pub mod tls_internal_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
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
    
    pub struct tls_error {
        pub msg: *mut ::core::ffi::c_char,
        pub num: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
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
    use super::_time_h::tm;
    use super::_time_t_h::time_t;
    use super::_uint32_t_h::uint32_t;
    use super::types_h::{SSL, SSL_CTX, X509};
    extern "C" {
        
        pub type tls_ocsp_query;
        
        pub fn tls_set_errorx(
            ctx: *mut tls,
            fmt: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        
        pub fn asn1_time_parse(
            _: *const ::core::ffi::c_char,
            _: size_t,
            _: *mut tm,
            _: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}

pub mod types_h {
    
    pub type X509 = x509_st;
    
    pub type SSL_CTX = ssl_ctx_st;
    
    pub type SSL = ssl_st;
    
    pub type ASN1_TIME = asn1_string_st;
    
    pub type EVP_MD = evp_md_st;
    
    pub type X509_NAME = X509_name_st;
    use super::asn1_h::asn1_string_st;
    extern "C" {
        
        pub type x509_st;
        
        pub type ssl_ctx_st;
        
        pub type ssl_st;
        
        pub type evp_md_st;
        
        pub type X509_name_st;
    }
}

pub mod _time_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct tm {
        pub tm_sec: ::core::ffi::c_int,
        pub tm_min: ::core::ffi::c_int,
        pub tm_hour: ::core::ffi::c_int,
        pub tm_mday: ::core::ffi::c_int,
        pub tm_mon: ::core::ffi::c_int,
        pub tm_year: ::core::ffi::c_int,
        pub tm_wday: ::core::ffi::c_int,
        pub tm_yday: ::core::ffi::c_int,
        pub tm_isdst: ::core::ffi::c_int,
        pub tm_gmtoff: ::core::ffi::c_long,
        pub tm_zone: *mut ::core::ffi::c_char,
    }
    use super::_time_t_h::time_t;
    extern "C" {
        
        pub fn timegm(_: *mut tm) -> time_t;
    }
}

pub mod asn1_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct asn1_string_st {
        pub length: ::core::ffi::c_int,
        pub type_0: ::core::ffi::c_int,
        pub data: *mut ::core::ffi::c_uchar,
        pub flags: ::core::ffi::c_long,
    }
}

pub mod ssl_h {
    
    pub type SSL_CIPHER = ssl_cipher_st;
    use super::types_h::SSL;
    extern "C" {
        
        pub type ssl_cipher_st;
        
        pub fn SSL_get_current_cipher(s: *const SSL) -> *const SSL_CIPHER;
        
        pub fn SSL_CIPHER_get_name(c: *const SSL_CIPHER) -> *const ::core::ffi::c_char;
        
        pub fn SSL_get_version(s: *const SSL) -> *const ::core::ffi::c_char;
    }
}

pub mod base_h {
    use super::_size_t_h::size_t;
    extern "C" {
        
        pub fn usual_reallocarray(
            p: *mut ::core::ffi::c_void,
            count: size_t,
            size: size_t,
        ) -> *mut ::core::ffi::c_void;
    }
}

pub mod _null_h {
    
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}

pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        
        pub fn memset(
            __b: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __len: size_t,
        ) -> *mut ::core::ffi::c_void;
        
        pub fn strdup(__s1: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    }
}

pub mod evp_h {
    use super::types_h::EVP_MD;
    extern "C" {
        
        pub fn EVP_sha256() -> *const EVP_MD;
    }
}

pub mod x509_h {
    use super::types_h::{ASN1_TIME, EVP_MD, X509, X509_NAME};
    extern "C" {
        
        pub fn X509_digest(
            data: *const X509,
            type_0: *const EVP_MD,
            md: *mut ::core::ffi::c_uchar,
            len: *mut ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int;
        
        pub fn X509_NAME_oneline(
            a: *const X509_NAME,
            buf: *mut ::core::ffi::c_char,
            size: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_char;
        
        pub fn X509_get_issuer_name(a: *const X509) -> *mut X509_NAME;
        
        pub fn X509_get_subject_name(a: *const X509) -> *mut X509_NAME;
        
        pub fn X509_getm_notBefore(x: *const X509) -> *mut ASN1_TIME;
        
        pub fn X509_getm_notAfter(x: *const X509) -> *mut ASN1_TIME;
    }
}

pub mod sys__types_h {
    
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}

pub mod stdint_h {
    
    pub const UINTPTR_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
    
    pub const SIZE_MAX: ::core::ffi::c_ulong = UINTPTR_MAX;
}

pub mod _stdio_h {
    extern "C" {
        
        pub fn asprintf(
            _: *mut *mut ::core::ffi::c_char,
            _: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}

pub mod _malloc_h {
    extern "C" {
        
        pub fn free(_: *mut ::core::ffi::c_void);
    }
}

pub mod crypto_h {
    extern "C" {
        
        pub fn CRYPTO_free(
            ptr: *mut ::core::ffi::c_void,
            file: *const ::core::ffi::c_char,
            line: ::core::ffi::c_int,
        );
    }
}
use self::_malloc_h::free;
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
use self::_stdio_h::asprintf;
use self::_string_h::{memset, strdup};
pub use self::_time_h::{timegm, tm};
pub use self::_time_t_h::time_t;
pub use self::_types_h::{__darwin_size_t, __darwin_time_t};
pub use self::_uint32_t_h::uint32_t;
pub use self::asn1_h::asn1_string_st;
use self::base_h::usual_reallocarray;
use self::crypto_h::CRYPTO_free;
use self::evp_h::EVP_sha256;
pub use self::ssl_h::{
    ssl_cipher_st, SSL_CIPHER_get_name, SSL_get_current_cipher, SSL_get_version, SSL_CIPHER,
};
pub use self::stdint_h::{SIZE_MAX, UINTPTR_MAX};
pub use self::sys__types_h::__DARWIN_NULL;
pub use self::tls_internal_h::{
    asn1_time_parse, tls, tls_config, tls_conninfo, tls_error, tls_keypair, tls_ocsp_info,
    tls_ocsp_query, tls_set_errorx,
};
pub use self::types_h::{
    evp_md_st, ssl_ctx_st, ssl_st, x509_st, X509_name_st, ASN1_TIME, EVP_MD, SSL, SSL_CTX, X509,
    X509_NAME,
};
use self::x509_h::{
    X509_NAME_oneline, X509_digest, X509_get_issuer_name, X509_get_subject_name,
    X509_getm_notAfter, X509_getm_notBefore,
};

unsafe extern "C" fn tls_hex_string(
    mut in_0: *const ::core::ffi::c_uchar,
    mut inlen: size_t,
    mut out: *mut *mut ::core::ffi::c_char,
    mut outlen: *mut size_t,
) -> ::core::ffi::c_int {
    static mut hex: [::core::ffi::c_char; 17] = unsafe {
        ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"0123456789abcdef\0")
    };
    let mut i: size_t = 0;
    let mut len: size_t = 0;
    let mut p = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !outlen.is_null() {
        *outlen = 0 as size_t;
    }
    if inlen >= SIZE_MAX as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    *out = usual_reallocarray(
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        inlen.wrapping_add(1 as size_t),
        2 as size_t,
    ) as *mut ::core::ffi::c_char;
    if (*out).is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    p = *out;
    len = 0 as size_t;
    i = 0 as size_t;
    while i < inlen {
        let fresh0 = len;
        len = len.wrapping_add(1);
        *p.add(fresh0) = hex[(*in_0.add(i) as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int) as usize];
        let fresh1 = len;
        len = len.wrapping_add(1);
        *p.add(fresh1) =
            hex[(*in_0.add(i) as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as usize];
        i = i.wrapping_add(1);
    }
    let fresh2 = len;
    len = len.wrapping_add(1);
    *p.add(fresh2) = 0 as ::core::ffi::c_char;
    if !outlen.is_null() {
        *outlen = len;
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn tls_get_peer_cert_hash(
    mut ctx: *mut tls,
    mut hash: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut d: [::core::ffi::c_uchar; 64] = [0; 64];
    let mut dhex = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut dlen: ::core::ffi::c_uint = 0;
    let mut rv = -(1 as ::core::ffi::c_int);
    *hash = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*ctx).ssl_peer_cert.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if X509_digest(
        (*ctx).ssl_peer_cert,
        EVP_sha256(),
        &raw mut d as *mut ::core::ffi::c_uchar,
        &raw mut dlen,
    ) != 1 as ::core::ffi::c_int
    {
        tls_set_errorx(
            ctx,
            b"digest failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if tls_hex_string(
        &raw mut d as *mut ::core::ffi::c_uchar,
        dlen as size_t,
        &raw mut dhex,
        ::core::ptr::null_mut::<size_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        tls_set_errorx(
            ctx,
            b"digest hex string failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if asprintf(
        hash,
        b"SHA256:%s\0" as *const u8 as *const ::core::ffi::c_char,
        dhex,
    ) == -(1 as ::core::ffi::c_int)
    {
        tls_set_errorx(
            ctx,
            b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
        );
        *hash = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        rv = 0 as ::core::ffi::c_int;
    }
    free(dhex as *mut ::core::ffi::c_void);
    rv
}

unsafe extern "C" fn tls_get_peer_cert_issuer(
    mut ctx: *mut tls,
    mut issuer: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut name = ::core::ptr::null_mut::<X509_NAME>();
    *issuer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*ctx).ssl_peer_cert.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    name = X509_get_issuer_name((*ctx).ssl_peer_cert);
    if name.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    *issuer = X509_NAME_oneline(
        name,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
    );
    if (*issuer).is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn tls_get_peer_cert_subject(
    mut ctx: *mut tls,
    mut subject: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut name = ::core::ptr::null_mut::<X509_NAME>();
    *subject = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*ctx).ssl_peer_cert.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    name = X509_get_subject_name((*ctx).ssl_peer_cert);
    if name.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    *subject = X509_NAME_oneline(
        name,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
    );
    if (*subject).is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn tls_get_peer_cert_times(
    mut ctx: *mut tls,
    mut notbefore: *mut time_t,
    mut notafter: *mut time_t,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut before_tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut after_tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut before = ::core::ptr::null_mut::<ASN1_TIME>();
    let mut after = ::core::ptr::null_mut::<ASN1_TIME>();
    let mut rv = -(1 as ::core::ffi::c_int);
    memset(
        &raw mut before_tm as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<tm>() as size_t,
    );
    memset(
        &raw mut after_tm as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<tm>() as size_t,
    );
    if !(*ctx).ssl_peer_cert.is_null() {
        before = X509_getm_notBefore((*ctx).ssl_peer_cert);
        if before.is_null() {
            current_block = 14571905179014639509;
        } else {
            after = X509_getm_notAfter((*ctx).ssl_peer_cert);
            if after.is_null() {
                current_block = 14571905179014639509;
            } else if asn1_time_parse(
                (*before).data as *mut ::core::ffi::c_char,
                (*before).length as size_t,
                &raw mut before_tm,
                0 as ::core::ffi::c_int,
            ) == -(1 as ::core::ffi::c_int)
            {
                current_block = 14571905179014639509;
            } else if asn1_time_parse(
                (*after).data as *mut ::core::ffi::c_char,
                (*after).length as size_t,
                &raw mut after_tm,
                0 as ::core::ffi::c_int,
            ) == -(1 as ::core::ffi::c_int)
            {
                current_block = 14571905179014639509;
            } else {
                *notbefore = timegm(&raw mut before_tm);
                if *notbefore == -(1 as ::core::ffi::c_int) as time_t {
                    current_block = 14571905179014639509;
                } else {
                    *notafter = timegm(&raw mut after_tm);
                    if *notafter == -(1 as ::core::ffi::c_int) as time_t {
                        current_block = 14571905179014639509;
                    } else {
                        current_block = 1394248824506584008;
                    }
                }
            }
        }
    } else {
        current_block = 1394248824506584008;
    }
    if current_block == 1394248824506584008 {
        rv = 0 as ::core::ffi::c_int;
    }
    rv
}
#[no_mangle]

pub unsafe extern "C" fn tls_get_conninfo(mut ctx: *mut tls) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut tmp = ::core::ptr::null::<::core::ffi::c_char>();
    tls_free_conninfo((*ctx).conninfo);
    if !(*ctx).ssl_peer_cert.is_null() {
        if tls_get_peer_cert_hash(ctx, &raw mut (*(*ctx).conninfo).hash)
            == -(1 as ::core::ffi::c_int)
        {
            current_block = 2499676429396879715;
        } else if tls_get_peer_cert_subject(ctx, &raw mut (*(*ctx).conninfo).subject)
            == -(1 as ::core::ffi::c_int)
        {
            current_block = 2499676429396879715;
        } else if tls_get_peer_cert_issuer(ctx, &raw mut (*(*ctx).conninfo).issuer)
            == -(1 as ::core::ffi::c_int)
        {
            current_block = 2499676429396879715;
        } else if tls_get_peer_cert_times(
            ctx,
            &raw mut (*(*ctx).conninfo).notbefore,
            &raw mut (*(*ctx).conninfo).notafter,
        ) == -(1 as ::core::ffi::c_int)
        {
            current_block = 2499676429396879715;
        } else {
            current_block = 15619007995458559411;
        }
    } else {
        current_block = 15619007995458559411;
    }
    if current_block == 15619007995458559411 {
        tmp = SSL_get_version((*ctx).ssl_conn);
        if !tmp.is_null() {
            (*(*ctx).conninfo).version = strdup(tmp);
            if !(*(*ctx).conninfo).version.is_null() {
                tmp = SSL_CIPHER_get_name(SSL_get_current_cipher((*ctx).ssl_conn));
                if !tmp.is_null() {
                    (*(*ctx).conninfo).cipher = strdup(tmp);
                    if !(*(*ctx).conninfo).cipher.is_null() {
                        return 0 as ::core::ffi::c_int;
                    }
                }
            }
        }
    }
    tls_free_conninfo((*ctx).conninfo);
    -(1 as ::core::ffi::c_int)
}
#[no_mangle]

pub unsafe extern "C" fn tls_free_conninfo(mut conninfo: *mut tls_conninfo) {
    if !conninfo.is_null() {
        free((*conninfo).hash as *mut ::core::ffi::c_void);
        (*conninfo).hash = ::core::ptr::null_mut::<::core::ffi::c_char>();
        CRYPTO_free(
            (*conninfo).subject as *mut ::core::ffi::c_void,
            b"lib/usual/tls/tls_conninfo.c\0" as *const u8 as *const ::core::ffi::c_char,
            188 as ::core::ffi::c_int,
        );
        (*conninfo).subject = ::core::ptr::null_mut::<::core::ffi::c_char>();
        CRYPTO_free(
            (*conninfo).issuer as *mut ::core::ffi::c_void,
            b"lib/usual/tls/tls_conninfo.c\0" as *const u8 as *const ::core::ffi::c_char,
            190 as ::core::ffi::c_int,
        );
        (*conninfo).issuer = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*conninfo).version as *mut ::core::ffi::c_void);
        (*conninfo).version = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free((*conninfo).cipher as *mut ::core::ffi::c_void);
        (*conninfo).cipher = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
}
#[no_mangle]

pub unsafe extern "C" fn tls_conn_cipher(mut ctx: *mut tls) -> *const ::core::ffi::c_char {
    if (*ctx).conninfo.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    (*(*ctx).conninfo).cipher
}
#[no_mangle]

pub unsafe extern "C" fn tls_conn_version(mut ctx: *mut tls) -> *const ::core::ffi::c_char {
    if (*ctx).conninfo.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    (*(*ctx).conninfo).version
}
