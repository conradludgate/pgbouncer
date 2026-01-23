#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:18"]
pub mod _types_h {
    #[c2rust::src_loc = "32:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "34:1"]
    pub type __uint16_t = u16;
    #[c2rust::src_loc = "36:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
    #[c2rust::src_loc = "119:1"]
    pub type __darwin_time_t = ::core::ffi::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_in_addr_t.h:18"]
pub mod _in_addr_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type in_addr_t = __uint32_t;
    use super::_types_h::__uint32_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:18"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
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
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/tls/tls_internal.h:24"]
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:1"]
    pub union tls_addr {
        pub ip4: in_addr,
        pub ip6: in6_addr,
    }
    use super::_size_t_h::size_t;
    use super::_time_t_h::time_t;
    use super::_uint32_t_h::uint32_t;
    use super::in6_h::in6_addr;
    use super::in_h::in_addr;
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
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/types.h:18"]
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
    #[c2rust::src_loc = "174:1"]
    pub type X509_NAME = X509_name_st;
    use super::asn1_h::{asn1_string_st, asn1_type_st};
    extern "C" {
        #[c2rust::src_loc = "169:9"]
        pub type x509_st;
        #[c2rust::src_loc = "197:9"]
        pub type ssl_ctx_st;
        #[c2rust::src_loc = "196:9"]
        pub type ssl_st;
        #[c2rust::src_loc = "84:9"]
        pub type asn1_object_st;
        #[c2rust::src_loc = "174:9"]
        pub type X509_name_st;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet/in.h:18"]
pub mod in_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "301:1"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    use super::_in_addr_t_h::in_addr_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet6/in6.h:18"]
pub mod in6_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "152:9"]
    pub struct in6_addr {
        pub __u6_addr: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "153:2"]
    pub union C2RustUnnamed {
        pub __u6_addr8: [__uint8_t; 16],
        pub __u6_addr16: [__uint16_t; 8],
        pub __u6_addr32: [__uint32_t; 4],
    }
    use super::_types_h::{__uint16_t, __uint32_t, __uint8_t};
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/stack.h:18"]
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
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/asn1.h:18"]
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
        pub value: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "526:5"]
    pub union C2RustUnnamed_0 {
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
    #[c2rust::src_loc = "81:10"]
    pub const V_ASN1_IA5STRING: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
    use super::types_h::{
        ASN1_BIT_STRING, ASN1_BMPSTRING, ASN1_BOOLEAN, ASN1_ENUMERATED, ASN1_GENERALIZEDTIME,
        ASN1_GENERALSTRING, ASN1_IA5STRING, ASN1_INTEGER, ASN1_OBJECT, ASN1_OCTET_STRING,
        ASN1_PRINTABLESTRING, ASN1_STRING, ASN1_T61STRING, ASN1_UNIVERSALSTRING, ASN1_UTCTIME,
        ASN1_UTF8STRING, ASN1_VISIBLESTRING,
    };
    extern "C" {
        #[c2rust::src_loc = "279:9"]
        pub type ASN1_VALUE_st;
        #[c2rust::src_loc = "678:1"]
        pub fn ASN1_STRING_length(x: *const ASN1_STRING) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "682:1"]
        pub fn ASN1_STRING_type(x: *const ASN1_STRING) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "686:1"]
        pub fn ASN1_STRING_get0_data(x: *const ASN1_STRING) -> *const ::core::ffi::c_uchar;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/x509v3.h:22"]
pub mod x509v3_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "162:9"]
    pub struct otherName_st {
        pub type_id: *mut ASN1_OBJECT,
        pub value: *mut ASN1_TYPE,
    }
    #[c2rust::src_loc = "162:1"]
    pub type OTHERNAME = otherName_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "167:9"]
    pub struct EDIPartyName_st {
        pub nameAssigner: *mut ASN1_STRING,
        pub partyName: *mut ASN1_STRING,
    }
    #[c2rust::src_loc = "167:1"]
    pub type EDIPARTYNAME = EDIPartyName_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "172:9"]
    pub struct GENERAL_NAME_st {
        pub type_0: ::core::ffi::c_int,
        pub d: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "183:5"]
    pub union C2RustUnnamed_1 {
        pub ptr: *mut ::core::ffi::c_char,
        pub otherName: *mut OTHERNAME,
        pub rfc822Name: *mut ASN1_IA5STRING,
        pub dNSName: *mut ASN1_IA5STRING,
        pub x400Address: *mut ASN1_STRING,
        pub directoryName: *mut X509_NAME,
        pub ediPartyName: *mut EDIPARTYNAME,
        pub uniformResourceIdentifier: *mut ASN1_IA5STRING,
        pub iPAddress: *mut ASN1_OCTET_STRING,
        pub registeredID: *mut ASN1_OBJECT,
        pub ip: *mut ASN1_OCTET_STRING,
        pub dirn: *mut X509_NAME,
        pub ia5: *mut ASN1_IA5STRING,
        pub rid: *mut ASN1_OBJECT,
        pub other: *mut ASN1_TYPE,
    }
    #[c2rust::src_loc = "172:1"]
    pub type GENERAL_NAME = GENERAL_NAME_st;
    #[c2rust::src_loc = "237:1"]
    pub type sk_GENERAL_NAME_freefunc = Option<unsafe extern "C" fn(*mut GENERAL_NAME) -> ()>;
    #[c2rust::src_loc = "175:10"]
    pub const GEN_DNS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "180:10"]
    pub const GEN_IPADD: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
    #[inline]
    #[c2rust::src_loc = "237:1"]
    pub unsafe extern "C" fn ossl_check_const_GENERAL_NAME_sk_type(
        mut sk: *const stack_st_GENERAL_NAME,
    ) -> *const OPENSSL_STACK {
        sk as *const OPENSSL_STACK
    }
    #[inline]
    #[c2rust::src_loc = "237:1"]
    pub unsafe extern "C" fn ossl_check_GENERAL_NAME_sk_type(
        mut sk: *mut stack_st_GENERAL_NAME,
    ) -> *mut OPENSSL_STACK {
        sk as *mut OPENSSL_STACK
    }
    #[inline]
    #[c2rust::src_loc = "237:1"]
    pub unsafe extern "C" fn ossl_check_GENERAL_NAME_freefunc_type(
        mut fr: sk_GENERAL_NAME_freefunc,
    ) -> OPENSSL_sk_freefunc {
        ::core::mem::transmute::<sk_GENERAL_NAME_freefunc, OPENSSL_sk_freefunc>(fr)
    }
    use super::stack_h::{OPENSSL_sk_freefunc, OPENSSL_STACK};
    use super::types_h::{
        ASN1_IA5STRING, ASN1_OBJECT, ASN1_OCTET_STRING, ASN1_STRING, ASN1_TYPE, X509_NAME,
    };
    extern "C" {
        #[c2rust::src_loc = "237:1"]
        pub type stack_st_GENERAL_NAME;
        #[c2rust::src_loc = "804:1"]
        pub fn GENERAL_NAME_free(a: *mut GENERAL_NAME);
    }
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
        #[c2rust::src_loc = "88:1"]
        pub fn strchr(
            __s: *const ::core::ffi::c_char,
            __c: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "89:1"]
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "96:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/x509.h:18"]
pub mod x509_h {
    use super::types_h::{X509, X509_NAME};
    extern "C" {
        #[c2rust::src_loc = "860:1"]
        pub fn X509_get_subject_name(a: *const X509) -> *mut X509_NAME;
        #[c2rust::src_loc = "1044:1"]
        pub fn X509_NAME_get_text_by_NID(
            name: *const X509_NAME,
            nid: ::core::ffi::c_int,
            buf: *mut ::core::ffi::c_char,
            len: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1112:1"]
        pub fn X509_get_ext_d2i(
            x: *const X509,
            nid: ::core::ffi::c_int,
            crit: *mut ::core::ffi::c_int,
            idx: *mut ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:18"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:18"]
pub mod sys__types_h {
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/socket.h:18"]
pub mod socket_h {
    #[c2rust::src_loc = "365:9"]
    pub const AF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "396:9"]
    pub const AF_INET6: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arpa/inet.h:18"]
pub mod inet_h {
    extern "C" {
        #[c2rust::src_loc = "81:1"]
        pub fn inet_pton(
            _: ::core::ffi::c_int,
            _: *const ::core::ffi::c_char,
            _: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/obj_mac.h:18"]
pub mod obj_mac_h {
    #[c2rust::src_loc = "2403:9"]
    pub const NID_commonName: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
    #[c2rust::src_loc = "2695:9"]
    pub const NID_subject_alt_name: ::core::ffi::c_int = 85 as ::core::ffi::c_int;
}
pub use self::_in_addr_t_h::in_addr_t;
use self::_malloc_h::{calloc, free};
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
use self::_string_h::{memcmp, strchr, strcmp, strlen};
use self::_strings_h::strcasecmp;
pub use self::_time_t_h::time_t;
pub use self::_types_h::{__darwin_size_t, __darwin_time_t, __uint16_t, __uint32_t, __uint8_t};
pub use self::_uint32_t_h::uint32_t;
pub use self::asn1_h::{
    asn1_string_st, asn1_type_st, ASN1_STRING_get0_data, ASN1_STRING_length, ASN1_STRING_type,
    ASN1_VALUE_st, C2RustUnnamed_0, ASN1_VALUE, V_ASN1_IA5STRING,
};
pub use self::in6_h::{in6_addr, C2RustUnnamed};
pub use self::in_h::in_addr;
use self::inet_h::inet_pton;
pub use self::obj_mac_h::{NID_commonName, NID_subject_alt_name};
pub use self::socket_h::{AF_INET, AF_INET6};
pub use self::stack_h::{
    stack_st, OPENSSL_sk_freefunc, OPENSSL_sk_num, OPENSSL_sk_pop_free, OPENSSL_sk_value,
    OPENSSL_STACK,
};
pub use self::sys__types_h::__DARWIN_NULL;
pub use self::tls_internal_h::{
    tls, tls_addr, tls_config, tls_conninfo, tls_error, tls_keypair, tls_ocsp_info, tls_ocsp_query,
    tls_set_error, tls_set_errorx,
};
pub use self::types_h::{
    asn1_object_st, ssl_ctx_st, ssl_st, x509_st, X509_name_st, ASN1_BIT_STRING, ASN1_BMPSTRING,
    ASN1_BOOLEAN, ASN1_ENUMERATED, ASN1_GENERALIZEDTIME, ASN1_GENERALSTRING, ASN1_IA5STRING,
    ASN1_INTEGER, ASN1_OBJECT, ASN1_OCTET_STRING, ASN1_PRINTABLESTRING, ASN1_STRING,
    ASN1_T61STRING, ASN1_TYPE, ASN1_UNIVERSALSTRING, ASN1_UTCTIME, ASN1_UTF8STRING,
    ASN1_VISIBLESTRING, SSL, SSL_CTX, X509, X509_NAME,
};
use self::x509_h::{X509_NAME_get_text_by_NID, X509_get_ext_d2i, X509_get_subject_name};
pub use self::x509v3_h::{
    ossl_check_GENERAL_NAME_freefunc_type, ossl_check_GENERAL_NAME_sk_type,
    ossl_check_const_GENERAL_NAME_sk_type, otherName_st, sk_GENERAL_NAME_freefunc,
    stack_st_GENERAL_NAME, C2RustUnnamed_1, EDIPartyName_st, GENERAL_NAME_free, GENERAL_NAME_st,
    EDIPARTYNAME, GENERAL_NAME, GEN_DNS, GEN_IPADD, OTHERNAME,
};
#[c2rust::src_loc = "31:1"]
unsafe extern "C" fn tls_match_name(
    mut cert_name: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut cert_domain = ::core::ptr::null::<::core::ffi::c_char>();
    let mut domain = ::core::ptr::null::<::core::ffi::c_char>();
    let mut next_dot = ::core::ptr::null::<::core::ffi::c_char>();
    if strcasecmp(cert_name, name) == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if *cert_name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '*' as i32 {
        cert_domain =
            cert_name.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char;
        if *cert_domain.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\0' as i32
        {
            return -(1 as ::core::ffi::c_int);
        }
        if *cert_domain.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '.' as i32
        {
            return -(1 as ::core::ffi::c_int);
        }
        if *cert_domain.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '.' as i32
        {
            return -(1 as ::core::ffi::c_int);
        }
        next_dot = strchr(
            cert_domain.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
            '.' as i32,
        );
        if next_dot.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        if *next_dot.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '.' as i32 {
            return -(1 as ::core::ffi::c_int);
        }
        domain = strchr(name, '.' as i32);
        if *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '.' as i32 {
            return -(1 as ::core::ffi::c_int);
        }
        if domain.is_null() || strlen(domain) == 1 as size_t {
            return -(1 as ::core::ffi::c_int);
        }
        if strcasecmp(cert_domain, domain) == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
    }
    -(1 as ::core::ffi::c_int)
}
#[c2rust::src_loc = "83:1"]
unsafe extern "C" fn tls_check_subject_altname(
    mut ctx: *mut tls,
    mut cert: *mut X509,
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut altname_stack = ::core::ptr::null_mut::<stack_st_GENERAL_NAME>();
    let mut addrbuf = tls_addr {
        ip4: in_addr { s_addr: 0 },
    };
    let mut addrlen: ::core::ffi::c_int = 0;
    let mut type_0: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut rv = -(1 as ::core::ffi::c_int);
    altname_stack = X509_get_ext_d2i(
        cert,
        NID_subject_alt_name,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    ) as *mut stack_st_GENERAL_NAME;
    if altname_stack.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if inet_pton(AF_INET, name, &raw mut addrbuf as *mut ::core::ffi::c_void)
        == 1 as ::core::ffi::c_int
    {
        type_0 = GEN_IPADD;
        addrlen = 4 as ::core::ffi::c_int;
    } else if inet_pton(AF_INET6, name, &raw mut addrbuf as *mut ::core::ffi::c_void)
        == 1 as ::core::ffi::c_int
    {
        type_0 = GEN_IPADD;
        addrlen = 16 as ::core::ffi::c_int;
    } else {
        type_0 = GEN_DNS;
        addrlen = 0 as ::core::ffi::c_int;
    }
    count = OPENSSL_sk_num(ossl_check_const_GENERAL_NAME_sk_type(altname_stack));
    i = 0 as ::core::ffi::c_int;
    while i < count {
        let mut altname = ::core::ptr::null_mut::<GENERAL_NAME>();
        altname = OPENSSL_sk_value(ossl_check_const_GENERAL_NAME_sk_type(altname_stack), i)
            as *mut GENERAL_NAME;
        if (*altname).type_0 == type_0 {
            if type_0 == GEN_DNS {
                let mut data = ::core::ptr::null::<::core::ffi::c_void>();
                let mut format: ::core::ffi::c_int = 0;
                let mut len: ::core::ffi::c_int = 0;
                format = ASN1_STRING_type((*altname).d.dNSName);
                if format == V_ASN1_IA5STRING {
                    data =
                        ASN1_STRING_get0_data((*altname).d.dNSName) as *const ::core::ffi::c_void;
                    len = ASN1_STRING_length((*altname).d.dNSName);
                    if len < 0 as ::core::ffi::c_int
                        || len != strlen(data as *const ::core::ffi::c_char) as ::core::ffi::c_int
                    {
                        tls_set_errorx(
                            ctx,
                            b"error verifying name '%s': NUL byte in subjectAltName, probably a malicious certificate\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            name,
                        );
                        rv = -(2 as ::core::ffi::c_int);
                        break;
                    } else if strcmp(
                        data as *const ::core::ffi::c_char,
                        b" \0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        tls_set_error(
                            ctx,
                            b"error verifying name '%s': a dNSName of \" \" must not be used\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            name,
                        );
                        rv = -(2 as ::core::ffi::c_int);
                        break;
                    } else if tls_match_name(data as *const ::core::ffi::c_char, name)
                        == 0 as ::core::ffi::c_int
                    {
                        rv = 0 as ::core::ffi::c_int;
                        break;
                    }
                }
            } else if type_0 == GEN_IPADD {
                let mut data_0 = ::core::ptr::null::<::core::ffi::c_uchar>();
                let mut datalen: ::core::ffi::c_int = 0;
                datalen = ASN1_STRING_length((*altname).d.iPAddress);
                data_0 = ASN1_STRING_get0_data((*altname).d.iPAddress);
                if datalen < 0 as ::core::ffi::c_int {
                    tls_set_errorx(
                        ctx,
                        b"Unexpected negative length for an IP address: %d\0" as *const u8
                            as *const ::core::ffi::c_char,
                        datalen,
                    );
                    rv = -(2 as ::core::ffi::c_int);
                    break;
                } else if datalen == addrlen
                    && memcmp(
                        data_0 as *const ::core::ffi::c_void,
                        &raw mut addrbuf as *const ::core::ffi::c_void,
                        addrlen as size_t,
                    ) == 0 as ::core::ffi::c_int
                {
                    rv = 0 as ::core::ffi::c_int;
                    break;
                }
            }
        }
        i += 1;
    }
    OPENSSL_sk_pop_free(
        ossl_check_GENERAL_NAME_sk_type(altname_stack),
        ossl_check_GENERAL_NAME_freefunc_type(Some(
            GENERAL_NAME_free as unsafe extern "C" fn(*mut GENERAL_NAME) -> (),
        )),
    );
    rv
}
#[c2rust::src_loc = "191:1"]
unsafe extern "C" fn tls_check_common_name(
    mut ctx: *mut tls,
    mut cert: *mut X509,
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut subject_name = ::core::ptr::null_mut::<X509_NAME>();
    let mut common_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut addrbuf = tls_addr {
        ip4: in_addr { s_addr: 0 },
    };
    let mut common_name_len: ::core::ffi::c_int = 0;
    let mut rv = -(1 as ::core::ffi::c_int);
    subject_name = X509_get_subject_name(cert);
    if !subject_name.is_null() {
        common_name_len = X509_NAME_get_text_by_NID(
            subject_name,
            NID_commonName,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
        );
        if common_name_len >= 0 as ::core::ffi::c_int {
            common_name = calloc(
                (common_name_len + 1 as ::core::ffi::c_int) as size_t,
                1 as size_t,
            ) as *mut ::core::ffi::c_char;
            if !common_name.is_null() {
                X509_NAME_get_text_by_NID(
                    subject_name,
                    NID_commonName,
                    common_name,
                    common_name_len + 1 as ::core::ffi::c_int,
                );
                if common_name_len != strlen(common_name) as ::core::ffi::c_int {
                    tls_set_errorx(
                        ctx,
                        b"error verifying name '%s': NUL byte in Common Name field, probably a malicious certificate\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        name,
                    );
                    rv = -(2 as ::core::ffi::c_int);
                } else if inet_pton(AF_INET, name, &raw mut addrbuf as *mut ::core::ffi::c_void)
                    == 1 as ::core::ffi::c_int
                    || inet_pton(AF_INET6, name, &raw mut addrbuf as *mut ::core::ffi::c_void)
                        == 1 as ::core::ffi::c_int
                {
                    if strcmp(common_name, name) == 0 as ::core::ffi::c_int {
                        rv = 0 as ::core::ffi::c_int;
                    } else {
                        rv = -(1 as ::core::ffi::c_int);
                    }
                } else if tls_match_name(common_name, name) == 0 as ::core::ffi::c_int {
                    rv = 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    free(common_name as *mut ::core::ffi::c_void);
    rv
}
#[no_mangle]
#[c2rust::src_loc = "244:1"]
pub unsafe extern "C" fn tls_check_name(
    mut ctx: *mut tls,
    mut cert: *mut X509,
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    rv = tls_check_subject_altname(ctx, cert, name);
    if rv == 0 as ::core::ffi::c_int || rv == -(2 as ::core::ffi::c_int) {
        return rv;
    }
    tls_check_common_name(ctx, cert, name)
}
