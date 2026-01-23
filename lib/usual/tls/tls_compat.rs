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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint64_t.h:19"]
pub mod _uint64_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint64_t = u64;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/tls/tls_internal.h:26"]
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
        #[c2rust::src_loc = "155:1"]
        pub fn tls_set_error(
            ctx: *mut tls,
            fmt: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "158:1"]
        pub fn tls_set_errorx(
            ctx: *mut tls,
            fmt: *const ::core::ffi::c_char,
            ...
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
    #[c2rust::src_loc = "63:1"]
    pub type ASN1_INTEGER = asn1_string_st;
    #[c2rust::src_loc = "64:1"]
    pub type ASN1_ENUMERATED = asn1_string_st;
    #[c2rust::src_loc = "65:1"]
    pub type ASN1_BIT_STRING = asn1_string_st;
    #[c2rust::src_loc = "66:1"]
    pub type ASN1_OCTET_STRING = asn1_string_st;
    #[c2rust::src_loc = "67:1"]
    pub type ASN1_PRINTABLESTRING = asn1_string_st;
    #[c2rust::src_loc = "68:1"]
    pub type ASN1_T61STRING = asn1_string_st;
    #[c2rust::src_loc = "69:1"]
    pub type ASN1_IA5STRING = asn1_string_st;
    #[c2rust::src_loc = "70:1"]
    pub type ASN1_GENERALSTRING = asn1_string_st;
    #[c2rust::src_loc = "71:1"]
    pub type ASN1_UNIVERSALSTRING = asn1_string_st;
    #[c2rust::src_loc = "72:1"]
    pub type ASN1_BMPSTRING = asn1_string_st;
    #[c2rust::src_loc = "73:1"]
    pub type ASN1_UTCTIME = asn1_string_st;
    #[c2rust::src_loc = "74:1"]
    pub type ASN1_TIME = asn1_string_st;
    #[c2rust::src_loc = "75:1"]
    pub type ASN1_GENERALIZEDTIME = asn1_string_st;
    #[c2rust::src_loc = "76:1"]
    pub type ASN1_VISIBLESTRING = asn1_string_st;
    #[c2rust::src_loc = "77:1"]
    pub type ASN1_UTF8STRING = asn1_string_st;
    #[c2rust::src_loc = "78:1"]
    pub type ASN1_STRING = asn1_string_st;
    #[c2rust::src_loc = "79:1"]
    pub type ASN1_BOOLEAN = ::core::ffi::c_int;
    #[c2rust::src_loc = "84:1"]
    pub type ASN1_OBJECT = asn1_object_st;
    #[c2rust::src_loc = "83:1"]
    pub type ASN1_TYPE = asn1_type_st;
    #[c2rust::src_loc = "95:1"]
    pub type BIO = bio_st;
    #[c2rust::src_loc = "110:1"]
    pub type EVP_CIPHER = evp_cipher_st;
    #[c2rust::src_loc = "116:1"]
    pub type EVP_PKEY = evp_pkey_st;
    #[c2rust::src_loc = "146:1"]
    pub type DH = dh_st;
    #[c2rust::src_loc = "170:1"]
    pub type X509_ALGOR = X509_algor_st;
    #[c2rust::src_loc = "161:1"]
    pub type EC_KEY = ec_key_st;
    #[c2rust::src_loc = "171:1"]
    pub type X509_CRL = X509_crl_st;
    #[c2rust::src_loc = "176:1"]
    pub type X509_STORE = x509_store_st;
    #[c2rust::src_loc = "232:1"]
    pub type OSSL_PARAM = ossl_param_st;
    #[c2rust::src_loc = "235:1"]
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
        #[c2rust::src_loc = "169:9"]
        pub type x509_st;
        #[c2rust::src_loc = "197:9"]
        pub type ssl_ctx_st;
        #[c2rust::src_loc = "196:9"]
        pub type ssl_st;
        #[c2rust::src_loc = "84:9"]
        pub type asn1_object_st;
        #[c2rust::src_loc = "95:9"]
        pub type bio_st;
        #[c2rust::src_loc = "110:9"]
        pub type evp_cipher_st;
        #[c2rust::src_loc = "116:9"]
        pub type evp_pkey_st;
        #[c2rust::src_loc = "146:9"]
        pub type dh_st;
        #[c2rust::src_loc = "161:9"]
        pub type ec_key_st;
        #[c2rust::src_loc = "171:9"]
        pub type X509_crl_st;
        #[c2rust::src_loc = "176:9"]
        pub type x509_store_st;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_time.h:19"]
pub mod _time_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "78:1"]
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
        #[c2rust::src_loc = "141:1"]
        pub fn timegm(_: *mut tm) -> time_t;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/stack.h:19"]
pub mod stack_h {
    #[c2rust::src_loc = "23:1"]
    pub type OPENSSL_STACK = stack_st;
    #[c2rust::src_loc = "26:1"]
    pub type OPENSSL_sk_freefunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    extern "C" {
        #[c2rust::src_loc = "23:9"]
        pub type stack_st;
        #[c2rust::src_loc = "30:1"]
        pub fn OPENSSL_sk_num(_: *const OPENSSL_STACK) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "31:1"]
        pub fn OPENSSL_sk_value(
            _: *const OPENSSL_STACK,
            _: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "41:1"]
        pub fn OPENSSL_sk_pop_free(st: *mut OPENSSL_STACK, func: OPENSSL_sk_freefunc);
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/asn1.h:19"]
pub mod asn1_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "186:1"]
    pub struct asn1_string_st {
        pub length: ::core::ffi::c_int,
        pub type_0: ::core::ffi::c_int,
        pub data: *mut ::core::ffi::c_uchar,
        pub flags: ::core::ffi::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "524:1"]
    pub struct asn1_type_st {
        pub type_0: ::core::ffi::c_int,
        pub value: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "526:5"]
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
    #[c2rust::src_loc = "279:1"]
    pub type ASN1_VALUE = ASN1_VALUE_st;
    #[c2rust::src_loc = "82:10"]
    pub const V_ASN1_UTCTIME: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
    #[c2rust::src_loc = "83:10"]
    pub const V_ASN1_GENERALIZEDTIME: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
    use super::types_h::{
        ASN1_BIT_STRING, ASN1_BMPSTRING, ASN1_BOOLEAN, ASN1_ENUMERATED, ASN1_GENERALIZEDTIME,
        ASN1_GENERALSTRING, ASN1_IA5STRING, ASN1_INTEGER, ASN1_OBJECT, ASN1_OCTET_STRING,
        ASN1_PRINTABLESTRING, ASN1_STRING, ASN1_T61STRING, ASN1_UNIVERSALSTRING, ASN1_UTCTIME,
        ASN1_UTF8STRING, ASN1_VISIBLESTRING,
    };
    extern "C" {
        #[c2rust::src_loc = "279:9"]
        pub type ASN1_VALUE_st;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/x509.h:19"]
pub mod x509_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "187:1"]
    pub struct X509_algor_st {
        pub algorithm: *mut ASN1_OBJECT,
        pub parameter: *mut ASN1_TYPE,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "375:9"]
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
    #[c2rust::src_loc = "375:1"]
    pub type X509_PKEY = private_key_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "390:9"]
    pub struct X509_info_st {
        pub x509: *mut X509,
        pub crl: *mut X509_CRL,
        pub x_pkey: *mut X509_PKEY,
        pub enc_cipher: EVP_CIPHER_INFO,
        pub enc_len: ::core::ffi::c_int,
        pub enc_data: *mut ::core::ffi::c_char,
    }
    #[c2rust::src_loc = "390:1"]
    pub type X509_INFO = X509_info_st;
    #[c2rust::src_loc = "398:1"]
    pub type sk_X509_INFO_freefunc = Option<unsafe extern "C" fn(*mut X509_INFO) -> ()>;
    #[inline]
    #[c2rust::src_loc = "398:1"]
    pub unsafe extern "C" fn ossl_check_const_X509_INFO_sk_type(
        mut sk: *const stack_st_X509_INFO,
    ) -> *const OPENSSL_STACK {
        return sk as *const OPENSSL_STACK;
    }
    #[inline]
    #[c2rust::src_loc = "398:1"]
    pub unsafe extern "C" fn ossl_check_X509_INFO_sk_type(
        mut sk: *mut stack_st_X509_INFO,
    ) -> *mut OPENSSL_STACK {
        return sk as *mut OPENSSL_STACK;
    }
    #[inline]
    #[c2rust::src_loc = "398:1"]
    pub unsafe extern "C" fn ossl_check_X509_INFO_freefunc_type(
        mut fr: sk_X509_INFO_freefunc,
    ) -> OPENSSL_sk_freefunc {
        return ::core::mem::transmute::<sk_X509_INFO_freefunc, OPENSSL_sk_freefunc>(fr);
    }
    use super::evp_h::EVP_CIPHER_INFO;
    use super::stack_h::{OPENSSL_sk_freefunc, OPENSSL_STACK};
    use super::types_h::{
        ASN1_OBJECT, ASN1_OCTET_STRING, ASN1_TYPE, EVP_PKEY, X509, X509_ALGOR, X509_CRL,
    };
    extern "C" {
        #[c2rust::src_loc = "398:1"]
        pub type stack_st_X509_INFO;
        #[c2rust::src_loc = "766:1"]
        pub fn X509_free(a: *mut X509);
        #[c2rust::src_loc = "818:1"]
        pub fn X509_INFO_free(a: *mut X509_INFO);
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/core.h:19"]
pub mod core_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "85:1"]
    pub struct ossl_param_st {
        pub key: *const ::core::ffi::c_char,
        pub data_type: ::core::ffi::c_uint,
        pub data: *mut ::core::ffi::c_void,
        pub data_size: size_t,
        pub return_size: size_t,
    }
    use super::_size_t_h::size_t;
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/evp.h:19"]
pub mod evp_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "504:9"]
    pub struct evp_cipher_info_st {
        pub cipher: *const EVP_CIPHER,
        pub iv: [::core::ffi::c_uchar; 16],
    }
    #[c2rust::src_loc = "504:1"]
    pub type EVP_CIPHER_INFO = evp_cipher_info_st;
    use super::types_h::{EVP_CIPHER, EVP_PKEY, OSSL_PARAM};
    extern "C" {
        #[c2rust::src_loc = "1377:1"]
        pub fn EVP_PKEY_get_bits(pkey: *const EVP_PKEY) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "2098:1"]
        pub fn EVP_PKEY_get_params(
            pkey: *const EVP_PKEY,
            params: *mut OSSL_PARAM,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:19"]
pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "77:1"]
        pub fn memcpy(
            __dst: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "83:1"]
        pub fn memset(
            __b: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __len: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "96:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/bio.h:19"]
pub mod bio_h {
    use super::types_h::BIO;
    extern "C" {
        #[c2rust::src_loc = "730:1"]
        pub fn BIO_free(a: *mut BIO) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "780:1"]
        pub fn BIO_new_mem_buf(
            buf: *const ::core::ffi::c_void,
            len: ::core::ffi::c_int,
        ) -> *mut BIO;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/params.h:19"]
pub mod params_h {
    use super::types_h::OSSL_PARAM;
    extern "C" {
        #[c2rust::src_loc = "75:1"]
        pub fn OSSL_PARAM_construct_int(
            key: *const ::core::ffi::c_char,
            buf: *mut ::core::ffi::c_int,
        ) -> OSSL_PARAM;
        #[c2rust::src_loc = "96:1"]
        pub fn OSSL_PARAM_construct_end() -> OSSL_PARAM;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/ec.h:19"]
pub mod ec_h {
    use super::types_h::EC_KEY;
    extern "C" {
        #[c2rust::src_loc = "1022:23"]
        pub fn EC_KEY_free(key: *mut EC_KEY);
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/dh.h:19"]
pub mod dh_h {
    use super::types_h::DH;
    extern "C" {
        #[c2rust::src_loc = "211:23"]
        pub fn DH_free(dh: *mut DH);
        #[c2rust::src_loc = "214:23"]
        pub fn DH_size(dh: *const DH) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/x509_vfy.h:19"]
pub mod x509_vfy_h {
    use super::types_h::{X509, X509_CRL, X509_STORE};
    extern "C" {
        #[c2rust::src_loc = "710:1"]
        pub fn X509_STORE_add_cert(xs: *mut X509_STORE, x: *mut X509) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "711:1"]
        pub fn X509_STORE_add_crl(xs: *mut X509_STORE, x: *mut X509_CRL) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/pem.h:19"]
pub mod pem_h {
    use super::types_h::{pem_password_cb, BIO, DH, X509};
    use super::x509_h::stack_st_X509_INFO;
    extern "C" {
        #[c2rust::src_loc = "405:1"]
        pub fn PEM_X509_INFO_read_bio(
            bp: *mut BIO,
            sk: *mut stack_st_X509_INFO,
            cb: Option<pem_password_cb>,
            u: *mut ::core::ffi::c_void,
        ) -> *mut stack_st_X509_INFO;
        #[c2rust::src_loc = "446:1"]
        pub fn PEM_read_bio_X509(
            out: *mut BIO,
            x: *mut *mut X509,
            cb: Option<pem_password_cb>,
            u: *mut ::core::ffi::c_void,
        ) -> *mut X509;
        #[c2rust::src_loc = "447:1"]
        pub fn PEM_read_bio_X509_AUX(
            out: *mut BIO,
            x: *mut *mut X509,
            cb: Option<pem_password_cb>,
            u: *mut ::core::ffi::c_void,
        ) -> *mut X509;
        #[c2rust::src_loc = "479:54"]
        pub fn PEM_read_bio_DHparams(
            out: *mut BIO,
            x: *mut *mut DH,
            cb: Option<pem_password_cb>,
            u: *mut ::core::ffi::c_void,
        ) -> *mut DH;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/ssl.h:19"]
pub mod ssl_h {
    #[c2rust::src_loc = "482:10"]
    pub const SSL_OP_SINGLE_ECDH_USE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "483:10"]
    pub const SSL_OP_SINGLE_DH_USE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "1280:10"]
    pub const SSL_CTRL_EXTRA_CHAIN_CERT: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
    #[c2rust::src_loc = "1344:10"]
    pub const SSL_CTRL_CLEAR_EXTRA_CHAIN_CERTS: ::core::ffi::c_int = 83 as ::core::ffi::c_int;
    use super::_uint64_t_h::uint64_t;
    use super::types_h::{evp_pkey_st, DH, SSL, SSL_CTX, X509, X509_STORE};
    extern "C" {
        #[c2rust::src_loc = "628:1"]
        pub fn SSL_CTX_set_options(ctx: *mut SSL_CTX, op: uint64_t) -> uint64_t;
        #[c2rust::src_loc = "1628:8"]
        pub fn SSL_CTX_get_cert_store(_: *const SSL_CTX) -> *mut X509_STORE;
        #[c2rust::src_loc = "1851:8"]
        pub fn SSL_CTX_use_certificate(ctx: *mut SSL_CTX, x: *mut X509) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "2020:1"]
        pub fn SSL_CTX_ctrl(
            ctx: *mut SSL_CTX,
            cmd: ::core::ffi::c_int,
            larg: ::core::ffi::c_long,
            parg: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_long;
        #[c2rust::src_loc = "2167:1"]
        pub fn SSL_get_privatekey(ssl: *const SSL) -> *mut evp_pkey_st;
        #[c2rust::src_loc = "2218:1"]
        pub fn SSL_get_ex_data(
            ssl: *const SSL,
            idx: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "2276:1"]
        pub fn SSL_CTX_set_tmp_dh_callback(
            ctx: *mut SSL_CTX,
            dh: Option<
                unsafe extern "C" fn(*mut SSL, ::core::ffi::c_int, ::core::ffi::c_int) -> *mut DH,
            >,
        );
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/err.h:19"]
pub mod err_h {
    #[c2rust::src_loc = "72:10"]
    pub const ERR_LIB_SYS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "79:10"]
    pub const ERR_LIB_PEM: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
    #[c2rust::src_loc = "218:10"]
    pub const ERR_SYSTEM_FLAG: ::core::ffi::c_uint =
        (INT_MAX as ::core::ffi::c_uint).wrapping_add(1 as ::core::ffi::c_uint);
    #[c2rust::src_loc = "219:10"]
    pub const ERR_SYSTEM_MASK: ::core::ffi::c_uint = INT_MAX as ::core::ffi::c_uint;
    #[c2rust::src_loc = "226:10"]
    pub const ERR_LIB_OFFSET: ::core::ffi::c_long = 23 as ::core::ffi::c_long;
    #[c2rust::src_loc = "227:10"]
    pub const ERR_LIB_MASK: ::core::ffi::c_int = 0xff as ::core::ffi::c_int;
    #[c2rust::src_loc = "230:10"]
    pub const ERR_REASON_MASK: ::core::ffi::c_int = 0x7fffff as ::core::ffi::c_int;
    #[inline]
    #[c2rust::src_loc = "241:1"]
    pub unsafe extern "C" fn ERR_GET_LIB(mut errcode: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
        if errcode & ERR_SYSTEM_FLAG as ::core::ffi::c_ulong != 0 as ::core::ffi::c_ulong {
            return ERR_LIB_SYS;
        }
        return (errcode >> ERR_LIB_OFFSET & ERR_LIB_MASK as ::core::ffi::c_ulong)
            as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "255:1"]
    pub unsafe extern "C" fn ERR_GET_REASON(
        mut errcode: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int {
        if errcode & ERR_SYSTEM_FLAG as ::core::ffi::c_ulong != 0 as ::core::ffi::c_ulong {
            return (errcode & ERR_SYSTEM_MASK as ::core::ffi::c_ulong) as ::core::ffi::c_int;
        }
        return (errcode & ERR_REASON_MASK as ::core::ffi::c_ulong) as ::core::ffi::c_int;
    }
    use super::limits_h::INT_MAX;
    extern "C" {
        #[c2rust::src_loc = "428:1"]
        pub fn ERR_peek_error() -> ::core::ffi::c_ulong;
        #[c2rust::src_loc = "440:1"]
        pub fn ERR_peek_last_error() -> ::core::ffi::c_ulong;
        #[c2rust::src_loc = "453:1"]
        pub fn ERR_clear_error();
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/limits.h:19"]
pub mod limits_h {
    #[c2rust::src_loc = "94:9"]
    pub const INT_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/opt/homebrew/Cellar/llvm/21.1.8/lib/clang/21/include/stdbool.h:19"]
pub mod stdbool_h {
    #[c2rust::src_loc = "25:9"]
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/obj_mac.h:19"]
pub mod obj_mac_h {
    #[c2rust::src_loc = "18:9"]
    pub const NID_undef: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/pemerr.h:19"]
pub mod pemerr_h {
    #[c2rust::src_loc = "46:10"]
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
#[c2rust::src_loc = "47:1"]
static mut file_dh2048: [::core::ffi::c_char; 425] = unsafe {
    ::core::mem::transmute::<
        [u8; 425],
        [::core::ffi::c_char; 425],
    >(
        *b"-----BEGIN DH PARAMETERS-----\nMIIBCAKCAQEA9kJXtwh/CBdyorrWqULzBej5UxE5T7bxbrlLOCDaAadWoxTpj0BV\n89AHxstDqZSt90xkhkn4DIO9ZekX1KHTUPj1WV/cdlJPPT2N286Z4VeSWc39uK50\nT8X8dryDxUcwYc58yWb/Ffm7/ZFexwGq01uejaClcjrUGvC/RgBYK+X0iP1YTknb\nzSC0neSRBzZrM2w4DUUdD3yIsxx8Wy2O9vPJI8BD8KVbGI2Ou1WMuF040zT9fBdX\nQ6MdGGzeMyEstSr/POGxKUAYEY18hKcKctaGxAMZyAcpesqVDNmWn6vQClCbAkbT\nCD1mpF1Bn5x8vYlLIhkmuquiXsNV6TILOwIBAg==\n-----END DH PARAMETERS-----\n\0",
    )
};
#[c2rust::src_loc = "57:1"]
static mut file_dh4096: [::core::ffi::c_char; 770] = unsafe {
    ::core::mem::transmute::<
        [u8; 770],
        [::core::ffi::c_char; 770],
    >(
        *b"-----BEGIN DH PARAMETERS-----\nMIICCAKCAgEA+hRyUsFN4VpJ1O8JLcCo/VWr19k3BCgJ4uk+d+KhehjdRqNDNyOQ\nl/MOyQNQfWXPeGKmOmIig6Ev/nm6Nf9Z2B1h3R4hExf+zTiHnvVPeRBhjdQi81rt\nXeoh6TNrSBIKIHfUJWBh3va0TxxjQIs6IZOLeVNRLMqzeylWqMf49HsIXqbcokUS\nVt1BkvLdW48j8PPv5DsKRN3tloTxqDJGo9tKvj1Fuk74A+Xda1kNhB7KFlqMyN98\nVETEJ6c7KpfOo30mnK30wqw3S8OtaIR/maYX72tGOno2ehFDkq3pnPtEbD2CScxc\nalJC+EL7RPk5c/tgeTvCngvc1KZn92Y//EI7G9tPZtylj2b56sHtMftIoYJ9+ODM\nsccD5Piz/rejE3Ome8EOOceUSCYAhXn8b3qvxVI1ddd1pED6FHRhFvLrZxFvBEM9\nERRMp5QqOaHJkM+Dxv8Cj6MqrCbfC4u+ZErxodzuusgDgvZiLF22uxMZbobFWyte\nOvOzKGtwcTqO/1wV5gKkzu1ZVswVUQd5Gg8lJicwqRWyyNRczDDoG9jVDxmogKTH\nAaqLulO7R8Ifa1SwF2DteSGVtgWEN8gDpN3RBmmPTDngyF2DHb5qmpnznwtFKdTL\nKWbuHn491xNO25CQWMtem80uKw+pTnisBRF/454n1Jnhub144YRBoN8CAQI=\n-----END DH PARAMETERS-----\n\0",
    )
};
#[c2rust::src_loc = "73:1"]
static mut dh4096: *mut DH = ::core::ptr::null::<DH>() as *mut DH;
#[c2rust::src_loc = "73:1"]
static mut dh2048: *mut DH = ::core::ptr::null::<DH>() as *mut DH;
#[c2rust::src_loc = "75:1"]
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
    return dh;
}
#[c2rust::src_loc = "92:1"]
unsafe extern "C" fn dh_auto_cb(
    mut s: *mut SSL,
    mut is_export: ::core::ffi::c_int,
    mut keylength: ::core::ffi::c_int,
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
    return load_dh_buffer(
        ctx,
        &raw mut dh2048,
        &raw const file_dh2048 as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
#[c2rust::src_loc = "108:1"]
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
    return 1 as ::core::ffi::c_long;
}
#[c2rust::src_loc = "128:1"]
static mut ecdh_cache: *mut EC_KEY = ::core::ptr::null::<EC_KEY>() as *mut EC_KEY;
#[no_mangle]
#[c2rust::src_loc = "135:1"]
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
    return false_0 != 0;
}
#[no_mangle]
#[c2rust::src_loc = "205:1"]
pub unsafe extern "C" fn SSL_CTX_set_ecdh_auto(
    mut ctx: *mut SSL_CTX,
    mut onoff: ::core::ffi::c_int,
) -> ::core::ffi::c_long {
    if onoff != 0 {
        SSL_CTX_set_options(ctx, SSL_OP_SINGLE_ECDH_USE as uint64_t);
    }
    return 1 as ::core::ffi::c_long;
}
#[no_mangle]
#[c2rust::src_loc = "218:1"]
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
#[c2rust::src_loc = "242:1"]
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
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "313:1"]
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
                if !(i < nstack) {
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
    return ret;
}
#[c2rust::src_loc = "358:1"]
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
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "372:1"]
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
    buf[len as usize] = '\0' as i32 as ::core::ffi::c_char;
    year = parse2num(
        &raw mut s,
        0 as ::core::ffi::c_int,
        99 as ::core::ffi::c_int,
    );
    if utctime != 0 {
        if year < 50 as ::core::ffi::c_int {
            year = 2000 as ::core::ffi::c_int + year;
        } else {
            year = 1900 as ::core::ffi::c_int + year;
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
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "432:1"]
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
    return 0 as ::core::ffi::c_int;
}
