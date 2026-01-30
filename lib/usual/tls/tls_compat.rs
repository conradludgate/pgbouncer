
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

pub mod _uint64_t_h {
    
    pub type uint64_t = u64;
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
    use super::_time_t_h::time_t;
    use super::_uint32_t_h::uint32_t;
    use super::types_h::{SSL, SSL_CTX, X509};
    extern "C" {
        
        pub type tls_ocsp_query;
        
        pub fn tls_set_error(
            ctx: *mut tls,
            fmt: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        
        pub fn tls_set_errorx(
            ctx: *mut tls,
            fmt: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}

pub mod types_h {
    
    pub type X509 = x509_st;
    
    pub type SSL_CTX = ssl_ctx_st;
    
    pub type SSL = ssl_st;
    
    pub type ASN1_INTEGER = asn1_string_st;
    
    pub type ASN1_ENUMERATED = asn1_string_st;
    
    pub type ASN1_BIT_STRING = asn1_string_st;
    
    pub type ASN1_OCTET_STRING = asn1_string_st;
    
    pub type ASN1_PRINTABLESTRING = asn1_string_st;
    
    pub type ASN1_T61STRING = asn1_string_st;
    
    pub type ASN1_IA5STRING = asn1_string_st;
    
    pub type ASN1_GENERALSTRING = asn1_string_st;
    
    pub type ASN1_UNIVERSALSTRING = asn1_string_st;
    
    pub type ASN1_BMPSTRING = asn1_string_st;
    
    pub type ASN1_UTCTIME = asn1_string_st;
    
    pub type ASN1_TIME = asn1_string_st;
    
    pub type ASN1_GENERALIZEDTIME = asn1_string_st;
    
    pub type ASN1_VISIBLESTRING = asn1_string_st;
    
    pub type ASN1_UTF8STRING = asn1_string_st;
    
    pub type ASN1_STRING = asn1_string_st;
    
    pub type ASN1_BOOLEAN = ::core::ffi::c_int;
    
    pub type ASN1_OBJECT = asn1_object_st;
    
    pub type ASN1_TYPE = asn1_type_st;
    
    pub type BIO = bio_st;
    
    pub type EVP_CIPHER = evp_cipher_st;
    
    pub type EVP_PKEY = evp_pkey_st;
    
    pub type DH = dh_st;
    
    pub type X509_ALGOR = X509_algor_st;
    
    pub type EC_KEY = ec_key_st;
    
    pub type X509_CRL = X509_crl_st;
    
    pub type X509_STORE = x509_store_st;
    
    pub type OSSL_PARAM = ossl_param_st;
    
    pub type pem_password_cb = unsafe extern "C" fn(
        *mut ::core::ffi::c_char,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    use super::asn1_h::{asn1_string_st, asn1_type_st};
    use super::core_h::ossl_param_st;
    use super::x509_h::X509_algor_st;
    extern "C" {
        
        pub type x509_st;
        
        pub type ssl_ctx_st;
        
        pub type ssl_st;
        
        pub type asn1_object_st;
        
        pub type bio_st;
        
        pub type evp_cipher_st;
        
        pub type evp_pkey_st;
        
        pub type dh_st;
        
        pub type ec_key_st;
        
        pub type X509_crl_st;
        
        pub type x509_store_st;
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

pub mod stack_h {
    
    pub type OPENSSL_STACK = stack_st;
    
    pub type OPENSSL_sk_freefunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    extern "C" {
        
        pub type stack_st;
        
        pub fn OPENSSL_sk_num(_: *const OPENSSL_STACK) -> ::core::ffi::c_int;
        
        pub fn OPENSSL_sk_value(
            _: *const OPENSSL_STACK,
            _: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void;
        
        pub fn OPENSSL_sk_pop_free(st: *mut OPENSSL_STACK, func: OPENSSL_sk_freefunc);
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct asn1_type_st {
        pub type_0: ::core::ffi::c_int,
        pub value: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union C2RustUnnamed {
        pub ptr: *mut ::core::ffi::c_char,
        pub boolean: ASN1_BOOLEAN,
        pub asn1_string: *mut ASN1_STRING,
        pub object: *mut ASN1_OBJECT,
        pub integer: *mut ASN1_INTEGER,
        pub enumerated: *mut ASN1_ENUMERATED,
        pub bit_string: *mut ASN1_BIT_STRING,
        pub octet_string: *mut ASN1_OCTET_STRING,
        pub printablestring: *mut ASN1_PRINTABLESTRING,
        pub t61string: *mut ASN1_T61STRING,
        pub ia5string: *mut ASN1_IA5STRING,
        pub generalstring: *mut ASN1_GENERALSTRING,
        pub bmpstring: *mut ASN1_BMPSTRING,
        pub universalstring: *mut ASN1_UNIVERSALSTRING,
        pub utctime: *mut ASN1_UTCTIME,
        pub generalizedtime: *mut ASN1_GENERALIZEDTIME,
        pub visiblestring: *mut ASN1_VISIBLESTRING,
        pub utf8string: *mut ASN1_UTF8STRING,
        pub set: *mut ASN1_STRING,
        pub sequence: *mut ASN1_STRING,
        pub asn1_value: *mut ASN1_VALUE,
    }
    
    pub type ASN1_VALUE = ASN1_VALUE_st;
    
    pub const V_ASN1_UTCTIME: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
    
    pub const V_ASN1_GENERALIZEDTIME: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
    use super::types_h::{
        ASN1_BIT_STRING, ASN1_BMPSTRING, ASN1_BOOLEAN, ASN1_ENUMERATED, ASN1_GENERALIZEDTIME,
        ASN1_GENERALSTRING, ASN1_IA5STRING, ASN1_INTEGER, ASN1_OBJECT, ASN1_OCTET_STRING,
        ASN1_PRINTABLESTRING, ASN1_STRING, ASN1_T61STRING, ASN1_UNIVERSALSTRING, ASN1_UTCTIME,
        ASN1_UTF8STRING, ASN1_VISIBLESTRING,
    };
    extern "C" {
        
        pub type ASN1_VALUE_st;
    }
}

pub mod x509_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct X509_algor_st {
        pub algorithm: *mut ASN1_OBJECT,
        pub parameter: *mut ASN1_TYPE,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct private_key_st {
        pub version: ::core::ffi::c_int,
        pub enc_algor: *mut X509_ALGOR,
        pub enc_pkey: *mut ASN1_OCTET_STRING,
        pub dec_pkey: *mut EVP_PKEY,
        pub key_length: ::core::ffi::c_int,
        pub key_data: *mut ::core::ffi::c_char,
        pub key_free: ::core::ffi::c_int,
        pub cipher: EVP_CIPHER_INFO,
    }
    
    pub type X509_PKEY = private_key_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct X509_info_st {
        pub x509: *mut X509,
        pub crl: *mut X509_CRL,
        pub x_pkey: *mut X509_PKEY,
        pub enc_cipher: EVP_CIPHER_INFO,
        pub enc_len: ::core::ffi::c_int,
        pub enc_data: *mut ::core::ffi::c_char,
    }
    
    pub type X509_INFO = X509_info_st;
    
    pub type sk_X509_INFO_freefunc = Option<unsafe extern "C" fn(*mut X509_INFO) -> ()>;
    #[inline]
    
    pub unsafe extern "C" fn ossl_check_const_X509_INFO_sk_type(
        mut sk: *const stack_st_X509_INFO,
    ) -> *const OPENSSL_STACK {
        sk as *const OPENSSL_STACK
    }
    #[inline]
    
    pub unsafe extern "C" fn ossl_check_X509_INFO_sk_type(
        mut sk: *mut stack_st_X509_INFO,
    ) -> *mut OPENSSL_STACK {
        sk as *mut OPENSSL_STACK
    }
    #[inline]
    
    pub unsafe extern "C" fn ossl_check_X509_INFO_freefunc_type(
        mut fr: sk_X509_INFO_freefunc,
    ) -> OPENSSL_sk_freefunc {
        ::core::mem::transmute::<sk_X509_INFO_freefunc, OPENSSL_sk_freefunc>(fr)
    }
    use super::evp_h::EVP_CIPHER_INFO;
    use super::stack_h::{OPENSSL_sk_freefunc, OPENSSL_STACK};
    use super::types_h::{
        ASN1_OBJECT, ASN1_OCTET_STRING, ASN1_TYPE, EVP_PKEY, X509, X509_ALGOR, X509_CRL,
    };
    extern "C" {
        
        pub type stack_st_X509_INFO;
        
        pub fn X509_free(a: *mut X509);
        
        pub fn X509_INFO_free(a: *mut X509_INFO);
    }
}

pub mod core_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct ossl_param_st {
        pub key: *const ::core::ffi::c_char,
        pub data_type: ::core::ffi::c_uint,
        pub data: *mut ::core::ffi::c_void,
        pub data_size: size_t,
        pub return_size: size_t,
    }
    use super::_size_t_h::size_t;
}

pub mod evp_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct evp_cipher_info_st {
        pub cipher: *const EVP_CIPHER,
        pub iv: [::core::ffi::c_uchar; 16],
    }
    
    pub type EVP_CIPHER_INFO = evp_cipher_info_st;
    use super::types_h::{EVP_CIPHER, EVP_PKEY, OSSL_PARAM};
    extern "C" {
        
        pub fn EVP_PKEY_get_bits(pkey: *const EVP_PKEY) -> ::core::ffi::c_int;
        
        pub fn EVP_PKEY_get_params(
            pkey: *const EVP_PKEY,
            params: *mut OSSL_PARAM,
        ) -> ::core::ffi::c_int;
    }
}

pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        
        pub fn memcpy(
            __dst: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        
        pub fn memset(
            __b: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __len: size_t,
        ) -> *mut ::core::ffi::c_void;
        
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}

pub mod bio_h {
    use super::types_h::BIO;
    extern "C" {
        
        pub fn BIO_free(a: *mut BIO) -> ::core::ffi::c_int;
        
        pub fn BIO_new_mem_buf(
            buf: *const ::core::ffi::c_void,
            len: ::core::ffi::c_int,
        ) -> *mut BIO;
    }
}

pub mod params_h {
    use super::types_h::OSSL_PARAM;
    extern "C" {
        
        pub fn OSSL_PARAM_construct_int(
            key: *const ::core::ffi::c_char,
            buf: *mut ::core::ffi::c_int,
        ) -> OSSL_PARAM;
        
        pub fn OSSL_PARAM_construct_end() -> OSSL_PARAM;
    }
}

pub mod ec_h {
    use super::types_h::EC_KEY;
    extern "C" {
        
        pub fn EC_KEY_free(key: *mut EC_KEY);
    }
}

pub mod dh_h {
    use super::types_h::DH;
    extern "C" {
        
        pub fn DH_free(dh: *mut DH);
        
        pub fn DH_size(dh: *const DH) -> ::core::ffi::c_int;
    }
}

pub mod x509_vfy_h {
    use super::types_h::{X509, X509_CRL, X509_STORE};
    extern "C" {
        
        pub fn X509_STORE_add_cert(xs: *mut X509_STORE, x: *mut X509) -> ::core::ffi::c_int;
        
        pub fn X509_STORE_add_crl(xs: *mut X509_STORE, x: *mut X509_CRL) -> ::core::ffi::c_int;
    }
}

pub mod pem_h {
    use super::types_h::{pem_password_cb, BIO, DH, X509};
    use super::x509_h::stack_st_X509_INFO;
    extern "C" {
        
        pub fn PEM_X509_INFO_read_bio(
            bp: *mut BIO,
            sk: *mut stack_st_X509_INFO,
            cb: Option<pem_password_cb>,
            u: *mut ::core::ffi::c_void,
        ) -> *mut stack_st_X509_INFO;
        
        pub fn PEM_read_bio_X509(
            out: *mut BIO,
            x: *mut *mut X509,
            cb: Option<pem_password_cb>,
            u: *mut ::core::ffi::c_void,
        ) -> *mut X509;
        
        pub fn PEM_read_bio_X509_AUX(
            out: *mut BIO,
            x: *mut *mut X509,
            cb: Option<pem_password_cb>,
            u: *mut ::core::ffi::c_void,
        ) -> *mut X509;
        
        pub fn PEM_read_bio_DHparams(
            out: *mut BIO,
            x: *mut *mut DH,
            cb: Option<pem_password_cb>,
            u: *mut ::core::ffi::c_void,
        ) -> *mut DH;
    }
}

pub mod ssl_h {
    
    pub const SSL_OP_SINGLE_ECDH_USE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    pub const SSL_OP_SINGLE_DH_USE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    pub const SSL_CTRL_EXTRA_CHAIN_CERT: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
    
    pub const SSL_CTRL_CLEAR_EXTRA_CHAIN_CERTS: ::core::ffi::c_int = 83 as ::core::ffi::c_int;
    use super::_uint64_t_h::uint64_t;
    use super::types_h::{evp_pkey_st, DH, SSL, SSL_CTX, X509, X509_STORE};
    extern "C" {
        
        pub fn SSL_CTX_set_options(ctx: *mut SSL_CTX, op: uint64_t) -> uint64_t;
        
        pub fn SSL_CTX_get_cert_store(_: *const SSL_CTX) -> *mut X509_STORE;
        
        pub fn SSL_CTX_use_certificate(ctx: *mut SSL_CTX, x: *mut X509) -> ::core::ffi::c_int;
        
        pub fn SSL_CTX_ctrl(
            ctx: *mut SSL_CTX,
            cmd: ::core::ffi::c_int,
            larg: ::core::ffi::c_long,
            parg: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_long;
        
        pub fn SSL_get_privatekey(ssl: *const SSL) -> *mut evp_pkey_st;
        
        pub fn SSL_get_ex_data(
            ssl: *const SSL,
            idx: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void;
        
        pub fn SSL_CTX_set_tmp_dh_callback(
            ctx: *mut SSL_CTX,
            dh: Option<
                unsafe extern "C" fn(*mut SSL, ::core::ffi::c_int, ::core::ffi::c_int) -> *mut DH,
            >,
        );
    }
}

pub mod err_h {
    
    pub const ERR_LIB_SYS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    pub const ERR_LIB_PEM: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
    
    pub const ERR_SYSTEM_FLAG: ::core::ffi::c_uint =
        (INT_MAX as ::core::ffi::c_uint).wrapping_add(1 as ::core::ffi::c_uint);
    
    pub const ERR_SYSTEM_MASK: ::core::ffi::c_uint = INT_MAX as ::core::ffi::c_uint;
    
    pub const ERR_LIB_OFFSET: ::core::ffi::c_long = 23 as ::core::ffi::c_long;
    
    pub const ERR_LIB_MASK: ::core::ffi::c_int = 0xff as ::core::ffi::c_int;
    
    pub const ERR_REASON_MASK: ::core::ffi::c_int = 0x7fffff as ::core::ffi::c_int;
    #[inline]
    
    pub unsafe extern "C" fn ERR_GET_LIB(mut errcode: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
        if errcode & ERR_SYSTEM_FLAG as ::core::ffi::c_ulong != 0 as ::core::ffi::c_ulong {
            return ERR_LIB_SYS;
        }
        (errcode >> ERR_LIB_OFFSET & ERR_LIB_MASK as ::core::ffi::c_ulong) as ::core::ffi::c_int
    }
    #[inline]
    
    pub unsafe extern "C" fn ERR_GET_REASON(
        mut errcode: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int {
        if errcode & ERR_SYSTEM_FLAG as ::core::ffi::c_ulong != 0 as ::core::ffi::c_ulong {
            return (errcode & ERR_SYSTEM_MASK as ::core::ffi::c_ulong) as ::core::ffi::c_int;
        }
        (errcode & ERR_REASON_MASK as ::core::ffi::c_ulong) as ::core::ffi::c_int
    }
    use super::limits_h::INT_MAX;
    extern "C" {
        
        pub fn ERR_peek_error() -> ::core::ffi::c_ulong;
        
        pub fn ERR_peek_last_error() -> ::core::ffi::c_ulong;
        
        pub fn ERR_clear_error();
    }
}

pub mod _null_h {
    
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}

pub mod sys__types_h {
    
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}

pub mod limits_h {
    
    pub const INT_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
}

pub mod stdbool_h {
    
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}

pub mod obj_mac_h {
    
    pub const NID_undef: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}

pub mod pemerr_h {
    
    pub const PEM_R_NO_START_LINE: ::core::ffi::c_int = 108 as ::core::ffi::c_int;
}
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
use self::_string_h::{memcpy, memset, strlen};
pub use self::_time_h::{timegm, tm};
pub use self::_time_t_h::time_t;
pub use self::_types_h::{__darwin_size_t, __darwin_time_t};
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::asn1_h::{
    asn1_string_st, asn1_type_st, ASN1_VALUE_st, C2RustUnnamed, ASN1_VALUE, V_ASN1_GENERALIZEDTIME,
    V_ASN1_UTCTIME,
};
use self::bio_h::{BIO_free, BIO_new_mem_buf};
pub use self::core_h::ossl_param_st;
use self::dh_h::{DH_free, DH_size};
use self::ec_h::EC_KEY_free;
pub use self::err_h::{
    ERR_clear_error, ERR_peek_error, ERR_peek_last_error, ERR_GET_LIB, ERR_GET_REASON,
    ERR_LIB_MASK, ERR_LIB_OFFSET, ERR_LIB_PEM, ERR_LIB_SYS, ERR_REASON_MASK, ERR_SYSTEM_FLAG,
    ERR_SYSTEM_MASK,
};
pub use self::evp_h::{
    evp_cipher_info_st, EVP_PKEY_get_bits, EVP_PKEY_get_params, EVP_CIPHER_INFO,
};
pub use self::limits_h::INT_MAX;
pub use self::obj_mac_h::NID_undef;
use self::params_h::{OSSL_PARAM_construct_end, OSSL_PARAM_construct_int};
use self::pem_h::{
    PEM_X509_INFO_read_bio, PEM_read_bio_DHparams, PEM_read_bio_X509, PEM_read_bio_X509_AUX,
};
pub use self::pemerr_h::PEM_R_NO_START_LINE;
pub use self::ssl_h::{
    SSL_CTX_ctrl, SSL_CTX_get_cert_store, SSL_CTX_set_options, SSL_CTX_set_tmp_dh_callback,
    SSL_CTX_use_certificate, SSL_get_ex_data, SSL_get_privatekey, SSL_CTRL_CLEAR_EXTRA_CHAIN_CERTS,
    SSL_CTRL_EXTRA_CHAIN_CERT, SSL_OP_SINGLE_DH_USE, SSL_OP_SINGLE_ECDH_USE,
};
pub use self::stack_h::{
    stack_st, OPENSSL_sk_freefunc, OPENSSL_sk_num, OPENSSL_sk_pop_free, OPENSSL_sk_value,
    OPENSSL_STACK,
};
pub use self::stdbool_h::{false_0, true_0};
pub use self::sys__types_h::__DARWIN_NULL;
pub use self::tls_internal_h::{
    tls, tls_config, tls_conninfo, tls_error, tls_keypair, tls_ocsp_info, tls_ocsp_query,
    tls_set_error, tls_set_errorx,
};
pub use self::types_h::{
    asn1_object_st, bio_st, dh_st, ec_key_st, evp_cipher_st, evp_pkey_st, pem_password_cb,
    ssl_ctx_st, ssl_st, x509_st, x509_store_st, X509_crl_st, ASN1_BIT_STRING, ASN1_BMPSTRING,
    ASN1_BOOLEAN, ASN1_ENUMERATED, ASN1_GENERALIZEDTIME, ASN1_GENERALSTRING, ASN1_IA5STRING,
    ASN1_INTEGER, ASN1_OBJECT, ASN1_OCTET_STRING, ASN1_PRINTABLESTRING, ASN1_STRING,
    ASN1_T61STRING, ASN1_TIME, ASN1_TYPE, ASN1_UNIVERSALSTRING, ASN1_UTCTIME, ASN1_UTF8STRING,
    ASN1_VISIBLESTRING, BIO, DH, EC_KEY, EVP_CIPHER, EVP_PKEY, OSSL_PARAM, SSL, SSL_CTX, X509,
    X509_ALGOR, X509_CRL, X509_STORE,
};
pub use self::x509_h::{
    ossl_check_X509_INFO_freefunc_type, ossl_check_X509_INFO_sk_type,
    ossl_check_const_X509_INFO_sk_type, private_key_st, sk_X509_INFO_freefunc, stack_st_X509_INFO,
    X509_INFO_free, X509_algor_st, X509_free, X509_info_st, X509_INFO, X509_PKEY,
};
use self::x509_vfy_h::{X509_STORE_add_cert, X509_STORE_add_crl};

static mut file_dh2048: [::core::ffi::c_char; 425] = unsafe {
    ::core::mem::transmute::<
        [u8; 425],
        [::core::ffi::c_char; 425],
    >(
        *b"-----BEGIN DH PARAMETERS-----\nMIIBCAKCAQEA9kJXtwh/CBdyorrWqULzBej5UxE5T7bxbrlLOCDaAadWoxTpj0BV\n89AHxstDqZSt90xkhkn4DIO9ZekX1KHTUPj1WV/cdlJPPT2N286Z4VeSWc39uK50\nT8X8dryDxUcwYc58yWb/Ffm7/ZFexwGq01uejaClcjrUGvC/RgBYK+X0iP1YTknb\nzSC0neSRBzZrM2w4DUUdD3yIsxx8Wy2O9vPJI8BD8KVbGI2Ou1WMuF040zT9fBdX\nQ6MdGGzeMyEstSr/POGxKUAYEY18hKcKctaGxAMZyAcpesqVDNmWn6vQClCbAkbT\nCD1mpF1Bn5x8vYlLIhkmuquiXsNV6TILOwIBAg==\n-----END DH PARAMETERS-----\n\0",
    )
};

static mut file_dh4096: [::core::ffi::c_char; 770] = unsafe {
    ::core::mem::transmute::<
        [u8; 770],
        [::core::ffi::c_char; 770],
    >(
        *b"-----BEGIN DH PARAMETERS-----\nMIICCAKCAgEA+hRyUsFN4VpJ1O8JLcCo/VWr19k3BCgJ4uk+d+KhehjdRqNDNyOQ\nl/MOyQNQfWXPeGKmOmIig6Ev/nm6Nf9Z2B1h3R4hExf+zTiHnvVPeRBhjdQi81rt\nXeoh6TNrSBIKIHfUJWBh3va0TxxjQIs6IZOLeVNRLMqzeylWqMf49HsIXqbcokUS\nVt1BkvLdW48j8PPv5DsKRN3tloTxqDJGo9tKvj1Fuk74A+Xda1kNhB7KFlqMyN98\nVETEJ6c7KpfOo30mnK30wqw3S8OtaIR/maYX72tGOno2ehFDkq3pnPtEbD2CScxc\nalJC+EL7RPk5c/tgeTvCngvc1KZn92Y//EI7G9tPZtylj2b56sHtMftIoYJ9+ODM\nsccD5Piz/rejE3Ome8EOOceUSCYAhXn8b3qvxVI1ddd1pED6FHRhFvLrZxFvBEM9\nERRMp5QqOaHJkM+Dxv8Cj6MqrCbfC4u+ZErxodzuusgDgvZiLF22uxMZbobFWyte\nOvOzKGtwcTqO/1wV5gKkzu1ZVswVUQd5Gg8lJicwqRWyyNRczDDoG9jVDxmogKTH\nAaqLulO7R8Ifa1SwF2DteSGVtgWEN8gDpN3RBmmPTDngyF2DHb5qmpnznwtFKdTL\nKWbuHn491xNO25CQWMtem80uKw+pTnisBRF/454n1Jnhub144YRBoN8CAQI=\n-----END DH PARAMETERS-----\n\0",
    )
};

static mut dh4096: *mut DH = ::core::ptr::null::<DH>() as *mut DH;

static mut dh2048: *mut DH = ::core::ptr::null::<DH>() as *mut DH;

unsafe extern "C" fn load_dh_buffer(
    mut ctx: *mut tls,
    mut dhp: *mut *mut DH,
    mut buf: *const ::core::ffi::c_char,
) -> *mut DH {
    let mut bio = ::core::ptr::null_mut::<BIO>();
    let mut dh = *dhp;
    if dh.is_null() {
        bio = BIO_new_mem_buf(
            buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            strlen(buf) as ::core::ffi::c_int,
        );
        if !bio.is_null() {
            dh = PEM_read_bio_DHparams(bio, ::core::ptr::null_mut::<*mut DH>(), None, NULL);
            BIO_free(bio);
        }
        *dhp = dh;
    }
    if !ctx.is_null() {
        (*ctx).used_dh_bits = DH_size(dh) * 8 as ::core::ffi::c_int;
    }
    dh
}

unsafe extern "C" fn dh_auto_cb(
    mut s: *mut SSL,
    mut _is_export: ::core::ffi::c_int,
    mut _keylength: ::core::ffi::c_int,
) -> *mut DH {
    let mut pk = ::core::ptr::null_mut::<EVP_PKEY>();
    let mut bits: ::core::ffi::c_int = 0;
    let mut ctx = SSL_get_ex_data(s, 0 as ::core::ffi::c_int) as *mut tls;
    pk = SSL_get_privatekey(s) as *mut EVP_PKEY;
    if pk.is_null() {
        return load_dh_buffer(
            ctx,
            &raw mut dh2048,
            &raw const file_dh2048 as *const ::core::ffi::c_char,
        );
    }
    bits = EVP_PKEY_get_bits(pk);
    if bits >= 3072 as ::core::ffi::c_int {
        return load_dh_buffer(
            ctx,
            &raw mut dh4096,
            &raw const file_dh4096 as *const ::core::ffi::c_char,
        );
    }
    load_dh_buffer(
        ctx,
        &raw mut dh2048,
        &raw const file_dh2048 as *const ::core::ffi::c_char,
    )
}
#[no_mangle]

pub unsafe extern "C" fn SSL_CTX_set_dh_auto(
    mut ctx: *mut SSL_CTX,
    mut onoff: ::core::ffi::c_int,
) -> ::core::ffi::c_long {
    if onoff == 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_long;
    } else {
        SSL_CTX_set_tmp_dh_callback(
            ctx,
            Some(
                dh_auto_cb
                    as unsafe extern "C" fn(
                        *mut SSL,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> *mut DH,
            ),
        );
    }
    SSL_CTX_set_options(ctx, SSL_OP_SINGLE_DH_USE as uint64_t);
    1 as ::core::ffi::c_long
}

static mut ecdh_cache: *mut EC_KEY = ::core::ptr::null::<EC_KEY>() as *mut EC_KEY;
#[no_mangle]

pub unsafe extern "C" fn get_ecdh_curve_nid(
    mut pk: *mut EVP_PKEY,
    mut nid: *mut ::core::ffi::c_int,
) -> bool {
    let mut params: [OSSL_PARAM; 2] = [ossl_param_st {
        key: ::core::ptr::null::<::core::ffi::c_char>(),
        data_type: 0,
        data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        data_size: 0,
        return_size: 0,
    }; 2];
    let mut curve_nid = NID_undef;
    params[0 as ::core::ffi::c_int as usize] = OSSL_PARAM_construct_int(
        b"curve\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut curve_nid,
    );
    params[1 as ::core::ffi::c_int as usize] = OSSL_PARAM_construct_end();
    if EVP_PKEY_get_params(pk, &raw mut params as *mut OSSL_PARAM) == 1 as ::core::ffi::c_int
        && curve_nid != NID_undef
    {
        *nid = curve_nid;
        return true_0 != 0;
    }
    false_0 != 0
}
#[no_mangle]

pub unsafe extern "C" fn SSL_CTX_set_ecdh_auto(
    mut ctx: *mut SSL_CTX,
    mut onoff: ::core::ffi::c_int,
) -> ::core::ffi::c_long {
    if onoff != 0 {
        SSL_CTX_set_options(ctx, SSL_OP_SINGLE_ECDH_USE as uint64_t);
    }
    1 as ::core::ffi::c_long
}
#[no_mangle]

pub unsafe extern "C" fn tls_compat_cleanup() {
    if !dh2048.is_null() {
        DH_free(dh2048);
        dh2048 = ::core::ptr::null_mut::<DH>();
    }
    if !dh4096.is_null() {
        DH_free(dh4096);
        dh4096 = ::core::ptr::null_mut::<DH>();
    }
    if !ecdh_cache.is_null() {
        EC_KEY_free(ecdh_cache);
        ecdh_cache = ::core::ptr::null_mut::<EC_KEY>();
    }
}
#[no_mangle]

pub unsafe extern "C" fn SSL_CTX_use_certificate_chain_mem(
    mut ctx: *mut SSL_CTX,
    mut data: *mut ::core::ffi::c_void,
    mut data_len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut psw_fn: Option<pem_password_cb> = None;
    let mut psw_arg = NULL;
    let mut cert = ::core::ptr::null_mut::<X509>();
    let mut bio = ::core::ptr::null_mut::<BIO>();
    let mut ok: ::core::ffi::c_int = 0;
    ERR_clear_error();
    bio = BIO_new_mem_buf(data, data_len);
    if !bio.is_null() {
        cert = PEM_read_bio_X509_AUX(bio, ::core::ptr::null_mut::<*mut X509>(), psw_fn, psw_arg);
        if !cert.is_null() {
            ok = SSL_CTX_use_certificate(ctx, cert);
            X509_free(cert);
            if !(ok == 0 || ERR_peek_error() != 0) {
                ok = SSL_CTX_ctrl(
                    ctx,
                    SSL_CTRL_CLEAR_EXTRA_CHAIN_CERTS,
                    0 as ::core::ffi::c_long,
                    NULL,
                ) as ::core::ffi::c_int;
                while ok != 0 {
                    cert = PEM_read_bio_X509(
                        bio,
                        ::core::ptr::null_mut::<*mut X509>(),
                        psw_fn,
                        psw_arg,
                    );
                    if cert.is_null() {
                        let mut err = ERR_peek_last_error();
                        if ERR_GET_LIB(err) != ERR_LIB_PEM {
                            break;
                        }
                        if ERR_GET_REASON(err) != PEM_R_NO_START_LINE {
                            break;
                        }
                        BIO_free(bio);
                        ERR_clear_error();
                        return 1 as ::core::ffi::c_int;
                    } else {
                        ok = SSL_CTX_ctrl(
                            ctx,
                            SSL_CTRL_EXTRA_CHAIN_CERT,
                            0 as ::core::ffi::c_long,
                            cert as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                        ) as ::core::ffi::c_int;
                        if ok == 0 {
                            X509_free(cert);
                        }
                    }
                }
            }
        }
    }
    if !bio.is_null() {
        BIO_free(bio);
    }
    0 as ::core::ffi::c_int
}
#[no_mangle]

pub unsafe extern "C" fn SSL_CTX_load_verify_mem(
    mut ctx: *mut SSL_CTX,
    mut data: *mut ::core::ffi::c_void,
    mut data_len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut stack = ::core::ptr::null_mut::<stack_st_X509_INFO>();
    let mut store = ::core::ptr::null_mut::<X509_STORE>();
    let mut info = ::core::ptr::null_mut::<X509_INFO>();
    let mut nstack: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ret = 0 as ::core::ffi::c_int;
    let mut got = 0 as ::core::ffi::c_int;
    let mut bio = ::core::ptr::null_mut::<BIO>();
    bio = BIO_new_mem_buf(data, data_len);
    if !bio.is_null() {
        stack = PEM_X509_INFO_read_bio(
            bio,
            ::core::ptr::null_mut::<stack_st_X509_INFO>(),
            None,
            NULL,
        );
        if !stack.is_null() {
            store = SSL_CTX_get_cert_store(ctx);
            nstack = OPENSSL_sk_num(ossl_check_const_X509_INFO_sk_type(stack));
            i = 0 as ::core::ffi::c_int;
            loop {
                if i >= nstack {
                    current_block = 11050875288958768710;
                    break;
                }
                info = OPENSSL_sk_value(ossl_check_const_X509_INFO_sk_type(stack), i)
                    as *mut X509_INFO;
                if !(*info).x509.is_null() && X509_STORE_add_cert(store, (*info).x509) == 0 {
                    current_block = 9239305583937002435;
                    break;
                }
                if !(*info).crl.is_null() && X509_STORE_add_crl(store, (*info).crl) == 0 {
                    current_block = 9239305583937002435;
                    break;
                }
                if !(*info).x509.is_null() || !(*info).crl.is_null() {
                    got = 1 as ::core::ffi::c_int;
                }
                i += 1;
            }
            match current_block {
                9239305583937002435 => {}
                _ => {
                    ret = got;
                }
            }
        }
    }
    if !bio.is_null() {
        BIO_free(bio);
    }
    if !stack.is_null() {
        OPENSSL_sk_pop_free(
            ossl_check_X509_INFO_sk_type(stack),
            ossl_check_X509_INFO_freefunc_type(Some(
                X509_INFO_free as unsafe extern "C" fn(*mut X509_INFO) -> (),
            )),
        );
    }
    ret == 0;
    ret
}

unsafe extern "C" fn parse2num(
    mut str_p: *mut *const ::core::ffi::c_char,
    mut min: ::core::ffi::c_int,
    mut max: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s = *str_p;
    if !s.is_null()
        && *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= '0' as i32
        && *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= '9' as i32
        && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= '0' as i32
        && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= '9' as i32
    {
        let mut val = (*s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - '0' as i32)
            * 10 as ::core::ffi::c_int
            + (*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int - '0' as i32);
        if val >= min && val <= max {
            *str_p = (*str_p).offset(2 as ::core::ffi::c_int as isize);
            return val;
        }
    }
    *str_p = ::core::ptr::null::<::core::ffi::c_char>();
    0 as ::core::ffi::c_int
}
#[no_mangle]

pub unsafe extern "C" fn asn1_time_parse(
    mut src: *const ::core::ffi::c_char,
    mut len: size_t,
    mut tm: *mut tm,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut buf: [::core::ffi::c_char; 16] = [0; 16];
    let mut s: *const ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
    let mut utctime: ::core::ffi::c_int = 0;
    let mut year: ::core::ffi::c_int = 0;
    memset(
        tm as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<tm>() as size_t,
    );
    if mode != 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if len == 15 as size_t {
        utctime = 0 as ::core::ffi::c_int;
    } else if len > 8 as size_t && len < 15 as size_t {
        utctime = 1 as ::core::ffi::c_int;
    } else {
        return -(1 as ::core::ffi::c_int);
    }
    memcpy(
        &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        len,
    );
    buf[len] = '\0' as i32 as ::core::ffi::c_char;
    year = parse2num(
        &raw mut s,
        0 as ::core::ffi::c_int,
        99 as ::core::ffi::c_int,
    );
    if utctime != 0 {
        if year < 50 as ::core::ffi::c_int {
            year += 2000 as ::core::ffi::c_int;
        } else {
            year += 1900 as ::core::ffi::c_int;
        }
    } else {
        year = year * 100 as ::core::ffi::c_int
            + parse2num(
                &raw mut s,
                0 as ::core::ffi::c_int,
                99 as ::core::ffi::c_int,
            );
    }
    (*tm).tm_year = year - 1900 as ::core::ffi::c_int;
    (*tm).tm_mon = parse2num(
        &raw mut s,
        1 as ::core::ffi::c_int,
        12 as ::core::ffi::c_int,
    ) - 1 as ::core::ffi::c_int;
    (*tm).tm_mday = parse2num(
        &raw mut s,
        1 as ::core::ffi::c_int,
        31 as ::core::ffi::c_int,
    );
    (*tm).tm_hour = parse2num(
        &raw mut s,
        0 as ::core::ffi::c_int,
        23 as ::core::ffi::c_int,
    );
    (*tm).tm_min = parse2num(
        &raw mut s,
        0 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int,
    );
    if utctime != 0 {
        if !s.is_null()
            && *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 'Z' as i32
            && *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
        {
            (*tm).tm_sec = parse2num(
                &raw mut s,
                0 as ::core::ffi::c_int,
                61 as ::core::ffi::c_int,
            );
        }
    } else {
        (*tm).tm_sec = parse2num(
            &raw mut s,
            0 as ::core::ffi::c_int,
            61 as ::core::ffi::c_int,
        );
    }
    if !s.is_null() {
        if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 {
            current_block = 14513103962030590156;
        } else if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'Z' as i32
            && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32
        {
            current_block = 14513103962030590156;
        } else {
            current_block = 1608152415753874203;
        }
        match current_block {
            1608152415753874203 => {}
            _ => {
                return if utctime != 0 {
                    V_ASN1_UTCTIME
                } else {
                    V_ASN1_GENERALIZEDTIME
                };
            }
        }
    }
    -(1 as ::core::ffi::c_int)
}
#[no_mangle]

pub unsafe extern "C" fn tls_asn1_parse_time(
    mut ctx: *mut tls,
    mut asn1time: *const ASN1_TIME,
    mut dst: *mut time_t,
) -> ::core::ffi::c_int {
    let mut tm = tm {
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
    let mut res: ::core::ffi::c_int = 0;
    let mut tval: time_t = 0;
    *dst = 0 as time_t;
    if asn1time.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*asn1time).type_0 != V_ASN1_GENERALIZEDTIME && (*asn1time).type_0 != V_ASN1_UTCTIME {
        tls_set_errorx(
            ctx,
            b"Invalid time object type: %d\0" as *const u8 as *const ::core::ffi::c_char,
            (*asn1time).type_0,
        );
        return -(1 as ::core::ffi::c_int);
    }
    res = asn1_time_parse(
        (*asn1time).data as *mut ::core::ffi::c_char,
        (*asn1time).length as size_t,
        &raw mut tm,
        0 as ::core::ffi::c_int,
    );
    if res == -(1 as ::core::ffi::c_int) {
        tls_set_errorx(
            ctx,
            b"Invalid asn1 time\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    tval = timegm(&raw mut tm);
    if tval == -(1 as ::core::ffi::c_int) as time_t {
        tls_set_error(
            ctx,
            b"Cannot convert asn1 time\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    *dst = tval;
    0 as ::core::ffi::c_int
}
