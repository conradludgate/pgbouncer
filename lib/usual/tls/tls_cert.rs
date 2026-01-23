#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:18"]
pub mod _types_h {
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
    #[c2rust::src_loc = "119:1"]
    pub type __darwin_time_t = ::core::ffi::c_long;
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
    use super::types_h::{ASN1_TIME, SSL, SSL_CTX, X509};
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
        #[c2rust::src_loc = "161:1"]
        pub fn tls_set_error_libssl(
            ctx: *mut tls,
            fmt: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "176:1"]
        pub fn tls_asn1_parse_time(
            ctx: *mut tls,
            asn1time: *const ASN1_TIME,
            dst: *mut time_t,
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
    #[c2rust::src_loc = "96:1"]
    pub type BIGNUM = bignum_st;
    #[c2rust::src_loc = "112:1"]
    pub type EVP_MD = evp_md_st;
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
        #[c2rust::src_loc = "96:9"]
        pub type bignum_st;
        #[c2rust::src_loc = "112:9"]
        pub type evp_md_st;
        #[c2rust::src_loc = "174:9"]
        pub type X509_name_st;
    }
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
    #[c2rust::src_loc = "635:1"]
    pub type sk_ASN1_OBJECT_freefunc = Option<unsafe extern "C" fn(*mut ASN1_OBJECT) -> ()>;
    #[c2rust::src_loc = "73:10"]
    pub const V_ASN1_UTF8STRING: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
    #[c2rust::src_loc = "76:10"]
    pub const V_ASN1_NUMERICSTRING: ::core::ffi::c_int = 18;
    #[c2rust::src_loc = "77:10"]
    pub const V_ASN1_PRINTABLESTRING: ::core::ffi::c_int = 19;
    #[c2rust::src_loc = "78:10"]
    pub const V_ASN1_T61STRING: ::core::ffi::c_int = 20;
    #[c2rust::src_loc = "81:10"]
    pub const V_ASN1_IA5STRING: ::core::ffi::c_int = 22;
    #[c2rust::src_loc = "86:10"]
    pub const V_ASN1_VISIBLESTRING: ::core::ffi::c_int = 26;
    #[c2rust::src_loc = "88:10"]
    pub const V_ASN1_UNIVERSALSTRING: ::core::ffi::c_int = 28;
    #[c2rust::src_loc = "89:10"]
    pub const V_ASN1_BMPSTRING: ::core::ffi::c_int = 30;
    #[c2rust::src_loc = "117:10"]
    pub const B_ASN1_UTF8STRING: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
    #[c2rust::src_loc = "122:10"]
    pub const MBSTRING_FLAG: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
    #[c2rust::src_loc = "123:10"]
    pub const MBSTRING_UTF8: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
    #[c2rust::src_loc = "124:10"]
    pub const MBSTRING_ASC: ::core::ffi::c_int = MBSTRING_FLAG | 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "125:10"]
    pub const MBSTRING_BMP: ::core::ffi::c_int = MBSTRING_FLAG | 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "126:10"]
    pub const MBSTRING_UNIV: ::core::ffi::c_int = MBSTRING_FLAG | 4 as ::core::ffi::c_int;
    #[inline]
    #[c2rust::src_loc = "635:1"]
    pub unsafe extern "C" fn ossl_check_ASN1_OBJECT_sk_type(
        mut sk: *mut stack_st_ASN1_OBJECT,
    ) -> *mut OPENSSL_STACK {
        return sk as *mut OPENSSL_STACK;
    }
    #[inline]
    #[c2rust::src_loc = "635:1"]
    pub unsafe extern "C" fn ossl_check_ASN1_OBJECT_freefunc_type(
        mut fr: sk_ASN1_OBJECT_freefunc,
    ) -> OPENSSL_sk_freefunc {
        return ::core::mem::transmute::<sk_ASN1_OBJECT_freefunc, OPENSSL_sk_freefunc>(fr);
    }
    use super::stack_h::{OPENSSL_sk_freefunc, OPENSSL_STACK};
    use super::types_h::{
        ASN1_BIT_STRING, ASN1_BMPSTRING, ASN1_BOOLEAN, ASN1_ENUMERATED, ASN1_GENERALIZEDTIME,
        ASN1_GENERALSTRING, ASN1_IA5STRING, ASN1_INTEGER, ASN1_OBJECT, ASN1_OCTET_STRING,
        ASN1_PRINTABLESTRING, ASN1_STRING, ASN1_T61STRING, ASN1_UNIVERSALSTRING, ASN1_UTCTIME,
        ASN1_UTF8STRING, ASN1_VISIBLESTRING, BIGNUM,
    };
    extern "C" {
        #[c2rust::src_loc = "279:9"]
        pub type ASN1_VALUE_st;
        #[c2rust::src_loc = "635:1"]
        pub type stack_st_ASN1_OBJECT;
        #[c2rust::src_loc = "663:1"]
        pub fn ASN1_OBJECT_free(a: *mut ASN1_OBJECT);
        #[c2rust::src_loc = "666:1"]
        pub fn ASN1_STRING_free(a: *mut ASN1_STRING);
        #[c2rust::src_loc = "678:1"]
        pub fn ASN1_STRING_length(x: *const ASN1_STRING) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "682:1"]
        pub fn ASN1_STRING_type(x: *const ASN1_STRING) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "686:1"]
        pub fn ASN1_STRING_get0_data(x: *const ASN1_STRING) -> *const ::core::ffi::c_uchar;
        #[c2rust::src_loc = "688:1"]
        pub fn ASN1_BIT_STRING_free(a: *mut ASN1_BIT_STRING);
        #[c2rust::src_loc = "878:1"]
        pub fn ASN1_INTEGER_get(a: *const ASN1_INTEGER) -> ::core::ffi::c_long;
        #[c2rust::src_loc = "880:1"]
        pub fn ASN1_INTEGER_to_BN(ai: *const ASN1_INTEGER, bn: *mut BIGNUM) -> *mut BIGNUM;
        #[c2rust::src_loc = "1014:1"]
        pub fn ASN1_mbstring_ncopy(
            out: *mut *mut ASN1_STRING,
            in_0: *const ::core::ffi::c_uchar,
            len: ::core::ffi::c_int,
            inform: ::core::ffi::c_int,
            mask: ::core::ffi::c_ulong,
            minsize: ::core::ffi::c_long,
            maxsize: ::core::ffi::c_long,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/x509v3.h:24"]
pub mod x509v3_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:9"]
    pub struct BASIC_CONSTRAINTS_st {
        pub ca: ::core::ffi::c_int,
        pub pathlen: *mut ASN1_INTEGER,
    }
    #[c2rust::src_loc = "147:1"]
    pub type BASIC_CONSTRAINTS = BASIC_CONSTRAINTS_st;
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
        pub d: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "183:5"]
    pub union C2RustUnnamed_0 {
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
    #[c2rust::src_loc = "266:1"]
    pub type EXTENDED_KEY_USAGE = stack_st_ASN1_OBJECT;
    #[c2rust::src_loc = "174:10"]
    pub const GEN_EMAIL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "175:10"]
    pub const GEN_DNS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "179:10"]
    pub const GEN_URI: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
    #[c2rust::src_loc = "180:10"]
    pub const GEN_IPADD: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
    #[inline]
    #[c2rust::src_loc = "237:1"]
    pub unsafe extern "C" fn ossl_check_const_GENERAL_NAME_sk_type(
        mut sk: *const stack_st_GENERAL_NAME,
    ) -> *const OPENSSL_STACK {
        return sk as *const OPENSSL_STACK;
    }
    #[inline]
    #[c2rust::src_loc = "237:1"]
    pub unsafe extern "C" fn ossl_check_GENERAL_NAME_freefunc_type(
        mut fr: sk_GENERAL_NAME_freefunc,
    ) -> OPENSSL_sk_freefunc {
        return ::core::mem::transmute::<sk_GENERAL_NAME_freefunc, OPENSSL_sk_freefunc>(fr);
    }
    #[inline]
    #[c2rust::src_loc = "237:1"]
    pub unsafe extern "C" fn ossl_check_GENERAL_NAME_sk_type(
        mut sk: *mut stack_st_GENERAL_NAME,
    ) -> *mut OPENSSL_STACK {
        return sk as *mut OPENSSL_STACK;
    }
    #[c2rust::src_loc = "674:10"]
    pub const KU_DIGITAL_SIGNATURE: ::core::ffi::c_int = X509v3_KU_DIGITAL_SIGNATURE;
    #[c2rust::src_loc = "675:10"]
    pub const KU_NON_REPUDIATION: ::core::ffi::c_int = X509v3_KU_NON_REPUDIATION;
    #[c2rust::src_loc = "676:10"]
    pub const KU_KEY_ENCIPHERMENT: ::core::ffi::c_int = X509v3_KU_KEY_ENCIPHERMENT;
    #[c2rust::src_loc = "677:10"]
    pub const KU_DATA_ENCIPHERMENT: ::core::ffi::c_int = X509v3_KU_DATA_ENCIPHERMENT;
    #[c2rust::src_loc = "678:10"]
    pub const KU_KEY_AGREEMENT: ::core::ffi::c_int = X509v3_KU_KEY_AGREEMENT;
    #[c2rust::src_loc = "679:10"]
    pub const KU_KEY_CERT_SIGN: ::core::ffi::c_int = X509v3_KU_KEY_CERT_SIGN;
    #[c2rust::src_loc = "680:10"]
    pub const KU_CRL_SIGN: ::core::ffi::c_int = X509v3_KU_CRL_SIGN;
    #[c2rust::src_loc = "681:10"]
    pub const KU_ENCIPHER_ONLY: ::core::ffi::c_int = X509v3_KU_ENCIPHER_ONLY;
    #[c2rust::src_loc = "682:10"]
    pub const KU_DECIPHER_ONLY: ::core::ffi::c_int = X509v3_KU_DECIPHER_ONLY;
    #[c2rust::src_loc = "693:10"]
    pub const XKU_SSL_SERVER: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "694:10"]
    pub const XKU_SSL_CLIENT: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "695:10"]
    pub const XKU_SMIME: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    #[c2rust::src_loc = "696:10"]
    pub const XKU_CODE_SIGN: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    #[c2rust::src_loc = "697:10"]
    pub const XKU_SGC: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    #[c2rust::src_loc = "698:10"]
    pub const XKU_OCSP_SIGN: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    #[c2rust::src_loc = "699:10"]
    pub const XKU_TIMESTAMP: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    #[c2rust::src_loc = "700:10"]
    pub const XKU_DVCS: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
    use super::_uint32_t_h::uint32_t;
    use super::asn1_h::stack_st_ASN1_OBJECT;
    use super::stack_h::{OPENSSL_sk_freefunc, OPENSSL_STACK};
    use super::types_h::{
        ASN1_IA5STRING, ASN1_INTEGER, ASN1_OBJECT, ASN1_OCTET_STRING, ASN1_STRING, ASN1_TYPE, X509,
        X509_NAME,
    };
    use super::x509_h::{
        X509v3_KU_CRL_SIGN, X509v3_KU_DATA_ENCIPHERMENT, X509v3_KU_DECIPHER_ONLY,
        X509v3_KU_DIGITAL_SIGNATURE, X509v3_KU_ENCIPHER_ONLY, X509v3_KU_KEY_AGREEMENT,
        X509v3_KU_KEY_CERT_SIGN, X509v3_KU_KEY_ENCIPHERMENT, X509v3_KU_NON_REPUDIATION,
    };
    extern "C" {
        #[c2rust::src_loc = "237:1"]
        pub type stack_st_GENERAL_NAME;
        #[c2rust::src_loc = "782:1"]
        pub fn BASIC_CONSTRAINTS_free(a: *mut BASIC_CONSTRAINTS);
        #[c2rust::src_loc = "804:1"]
        pub fn GENERAL_NAME_free(a: *mut GENERAL_NAME);
        #[c2rust::src_loc = "990:1"]
        pub fn X509_check_ca(x: *mut X509) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1000:1"]
        pub fn X509_get_key_usage(x: *mut X509) -> uint32_t;
        #[c2rust::src_loc = "1001:1"]
        pub fn X509_get_extended_key_usage(x: *mut X509) -> uint32_t;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/x509.h:18"]
pub mod x509_h {
    #[c2rust::src_loc = "201:1"]
    pub type X509_NAME_ENTRY = X509_name_entry_st;
    #[c2rust::src_loc = "174:10"]
    pub const X509v3_KU_DIGITAL_SIGNATURE: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
    #[c2rust::src_loc = "175:10"]
    pub const X509v3_KU_NON_REPUDIATION: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    #[c2rust::src_loc = "176:10"]
    pub const X509v3_KU_KEY_ENCIPHERMENT: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    #[c2rust::src_loc = "177:10"]
    pub const X509v3_KU_DATA_ENCIPHERMENT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    #[c2rust::src_loc = "178:10"]
    pub const X509v3_KU_KEY_AGREEMENT: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    #[c2rust::src_loc = "179:10"]
    pub const X509v3_KU_KEY_CERT_SIGN: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    #[c2rust::src_loc = "180:10"]
    pub const X509v3_KU_CRL_SIGN: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "181:10"]
    pub const X509v3_KU_ENCIPHER_ONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "182:10"]
    pub const X509v3_KU_DECIPHER_ONLY: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
    use super::types_h::{ASN1_INTEGER, ASN1_STRING, ASN1_TIME, EVP_MD, X509, X509_NAME};
    extern "C" {
        #[c2rust::src_loc = "201:9"]
        pub type X509_name_entry_st;
        #[c2rust::src_loc = "557:1"]
        pub fn X509_digest(
            data: *const X509,
            type_0: *const EVP_MD,
            md: *mut ::core::ffi::c_uchar,
            len: *mut ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "852:1"]
        pub fn X509_get_version(x: *const X509) -> ::core::ffi::c_long;
        #[c2rust::src_loc = "855:1"]
        pub fn X509_get_serialNumber(x: *mut X509) -> *mut ASN1_INTEGER;
        #[c2rust::src_loc = "858:1"]
        pub fn X509_get_issuer_name(a: *const X509) -> *mut X509_NAME;
        #[c2rust::src_loc = "860:1"]
        pub fn X509_get_subject_name(a: *const X509) -> *mut X509_NAME;
        #[c2rust::src_loc = "862:1"]
        pub fn X509_getm_notBefore(x: *const X509) -> *mut ASN1_TIME;
        #[c2rust::src_loc = "865:1"]
        pub fn X509_getm_notAfter(x: *const X509) -> *mut ASN1_TIME;
        #[c2rust::src_loc = "1053:1"]
        pub fn X509_NAME_get_index_by_NID(
            name: *const X509_NAME,
            nid: ::core::ffi::c_int,
            lastpos: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1056:1"]
        pub fn X509_NAME_get_entry(
            name: *const X509_NAME,
            loc: ::core::ffi::c_int,
        ) -> *mut X509_NAME_ENTRY;
        #[c2rust::src_loc = "1085:1"]
        pub fn X509_NAME_ENTRY_get_data(ne: *const X509_NAME_ENTRY) -> *mut ASN1_STRING;
        #[c2rust::src_loc = "1112:1"]
        pub fn X509_get_ext_d2i(
            x: *const X509,
            nid: ::core::ffi::c_int,
            crit: *mut ::core::ffi::c_int,
            idx: *mut ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/tls/tls_cert.h:29"]
pub mod tls_cert_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:1"]
    pub struct tls_cert_general_name {
        pub name_value: *const ::core::ffi::c_void,
        pub name_type: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:1"]
    pub struct tls_cert_dname {
        pub common_name: *const ::core::ffi::c_char,
        pub country_name: *const ::core::ffi::c_char,
        pub state_or_province_name: *const ::core::ffi::c_char,
        pub locality_name: *const ::core::ffi::c_char,
        pub street_address: *const ::core::ffi::c_char,
        pub organization_name: *const ::core::ffi::c_char,
        pub organizational_unit_name: *const ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:1"]
    pub struct tls_cert {
        pub version: ::core::ffi::c_int,
        pub successful_verify: ::core::ffi::c_int,
        pub subject: tls_cert_dname,
        pub issuer: tls_cert_dname,
        pub serial: *const ::core::ffi::c_char,
        pub not_before: time_t,
        pub not_after: time_t,
        pub ext_set: uint32_t,
        pub ext_crit: uint32_t,
        pub basic_constraints_ca: ::core::ffi::c_int,
        pub basic_constraints_pathlen: ::core::ffi::c_int,
        pub key_usage_flags: uint32_t,
        pub extended_key_usage_flags: uint32_t,
        pub subject_alt_names: *mut tls_cert_general_name,
        pub subject_alt_name_count: ::core::ffi::c_int,
        pub fingerprint: *const ::core::ffi::c_uchar,
        pub fingerprint_size: size_t,
    }
    #[c2rust::src_loc = "4:9"]
    pub const TLS_CERT_GNAME_DNS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "5:9"]
    pub const TLS_CERT_GNAME_IPv4: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "6:9"]
    pub const TLS_CERT_GNAME_IPv6: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "7:9"]
    pub const TLS_CERT_GNAME_EMAIL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    #[c2rust::src_loc = "8:9"]
    pub const TLS_CERT_GNAME_URI: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    #[c2rust::src_loc = "20:9"]
    pub const TLS_XKU_SSL_SERVER: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "21:9"]
    pub const TLS_XKU_SSL_CLIENT: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "22:9"]
    pub const TLS_XKU_SMIME: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "23:9"]
    pub const TLS_XKU_CODE_SIGN: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "24:9"]
    pub const TLS_XKU_OCSP_SIGN: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
    #[c2rust::src_loc = "25:9"]
    pub const TLS_XKU_SGC: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const TLS_XKU_TIMESTAMP: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;
    #[c2rust::src_loc = "27:9"]
    pub const TLS_XKU_DVCS: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int;
    #[c2rust::src_loc = "29:9"]
    pub const TLS_EXT_BASIC: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "30:9"]
    pub const TLS_EXT_KEY_USAGE: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "31:9"]
    pub const TLS_EXT_EXTENDED_KEY_USAGE: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
    use super::_size_t_h::size_t;
    use super::_time_t_h::time_t;
    use super::_uint32_t_h::uint32_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:18"]
pub mod _stdio_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "435:1"]
        pub fn snprintf(
            __str: *mut ::core::ffi::c_char,
            __size: size_t,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:18"]
pub mod _malloc_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "54:1"]
        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
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
        #[c2rust::src_loc = "73:1"]
        pub fn memchr(
            __s: *const ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "77:1"]
        pub fn memcpy(
            __dst: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "141:1"]
        pub fn strdup(__s1: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/bn.h:18"]
pub mod bn_h {
    use super::types_h::BIGNUM;
    extern "C" {
        #[c2rust::src_loc = "305:1"]
        pub fn BN_free(a: *mut BIGNUM);
        #[c2rust::src_loc = "345:1"]
        pub fn BN_bn2dec(a: *const BIGNUM) -> *mut ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/evp.h:18"]
pub mod evp_h {
    use super::types_h::EVP_MD;
    extern "C" {
        #[c2rust::src_loc = "577:1"]
        pub fn EVP_MD_get_size(md: *const EVP_MD) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "976:1"]
        pub fn EVP_sha1() -> *const EVP_MD;
        #[c2rust::src_loc = "978:1"]
        pub fn EVP_sha256() -> *const EVP_MD;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/ssl.h:18"]
pub mod ssl_h {
    use super::types_h::SSL;
    extern "C" {
        #[c2rust::src_loc = "2202:8"]
        pub fn SSL_get_verify_result(ssl: *const SSL) -> ::core::ffi::c_long;
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
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/tls/tls.h:18"]
pub mod tls_h {
    #[c2rust::src_loc = "42:9"]
    pub const TLS_NO_CERT: ::core::ffi::c_int = -(5 as ::core::ffi::c_int);
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
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/crypto.h:18"]
pub mod crypto_h {
    extern "C" {
        #[c2rust::src_loc = "387:1"]
        pub fn CRYPTO_free(
            ptr: *mut ::core::ffi::c_void,
            file: *const ::core::ffi::c_char,
            line: ::core::ffi::c_int,
        );
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/obj_mac.h:18"]
pub mod obj_mac_h {
    #[c2rust::src_loc = "2403:9"]
    pub const NID_commonName: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
    #[c2rust::src_loc = "2417:9"]
    pub const NID_countryName: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
    #[c2rust::src_loc = "2422:9"]
    pub const NID_localityName: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    #[c2rust::src_loc = "2427:9"]
    pub const NID_stateOrProvinceName: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    #[c2rust::src_loc = "2432:9"]
    pub const NID_streetAddress: ::core::ffi::c_int = 660 as ::core::ffi::c_int;
    #[c2rust::src_loc = "2437:9"]
    pub const NID_organizationName: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
    #[c2rust::src_loc = "2442:9"]
    pub const NID_organizationalUnitName: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
    #[c2rust::src_loc = "2685:9"]
    pub const NID_key_usage: ::core::ffi::c_int = 83 as ::core::ffi::c_int;
    #[c2rust::src_loc = "2695:9"]
    pub const NID_subject_alt_name: ::core::ffi::c_int = 85 as ::core::ffi::c_int;
    #[c2rust::src_loc = "2705:9"]
    pub const NID_basic_constraints: ::core::ffi::c_int = 87 as ::core::ffi::c_int;
    #[c2rust::src_loc = "2775:9"]
    pub const NID_ext_key_usage: ::core::ffi::c_int = 126 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/x509_vfy.h:18"]
pub mod x509_vfy_h {
    #[c2rust::src_loc = "305:10"]
    pub const X509_V_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/err.h:18"]
pub mod err_h {
    extern "C" {
        #[c2rust::src_loc = "453:1"]
        pub fn ERR_clear_error();
    }
}
use self::_malloc_h::{calloc, free, malloc};
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
use self::_stdio_h::snprintf;
use self::_string_h::{memchr, memcpy, strdup};
use self::_strings_h::strcasecmp;
pub use self::_time_t_h::time_t;
pub use self::_types_h::{__darwin_size_t, __darwin_time_t};
pub use self::_uint32_t_h::uint32_t;
pub use self::asn1_h::{
    asn1_string_st, asn1_type_st, ossl_check_ASN1_OBJECT_freefunc_type,
    ossl_check_ASN1_OBJECT_sk_type, sk_ASN1_OBJECT_freefunc, stack_st_ASN1_OBJECT,
    ASN1_BIT_STRING_free, ASN1_INTEGER_get, ASN1_INTEGER_to_BN, ASN1_OBJECT_free, ASN1_STRING_free,
    ASN1_STRING_get0_data, ASN1_STRING_length, ASN1_STRING_type, ASN1_VALUE_st,
    ASN1_mbstring_ncopy, C2RustUnnamed, ASN1_VALUE, B_ASN1_UTF8STRING, MBSTRING_ASC, MBSTRING_BMP,
    MBSTRING_FLAG, MBSTRING_UNIV, MBSTRING_UTF8, V_ASN1_BMPSTRING, V_ASN1_IA5STRING,
    V_ASN1_NUMERICSTRING, V_ASN1_PRINTABLESTRING, V_ASN1_T61STRING, V_ASN1_UNIVERSALSTRING,
    V_ASN1_UTF8STRING, V_ASN1_VISIBLESTRING,
};
use self::bn_h::{BN_bn2dec, BN_free};
use self::crypto_h::CRYPTO_free;
use self::err_h::ERR_clear_error;
use self::evp_h::{EVP_MD_get_size, EVP_sha1, EVP_sha256};
pub use self::obj_mac_h::{
    NID_basic_constraints, NID_commonName, NID_countryName, NID_ext_key_usage, NID_key_usage,
    NID_localityName, NID_organizationName, NID_organizationalUnitName, NID_stateOrProvinceName,
    NID_streetAddress, NID_subject_alt_name,
};
use self::ssl_h::SSL_get_verify_result;
pub use self::stack_h::{
    stack_st, OPENSSL_sk_freefunc, OPENSSL_sk_num, OPENSSL_sk_pop_free, OPENSSL_sk_value,
    OPENSSL_STACK,
};
pub use self::sys__types_h::__DARWIN_NULL;
pub use self::tls_cert_h::{
    tls_cert, tls_cert_dname, tls_cert_general_name, TLS_CERT_GNAME_IPv4, TLS_CERT_GNAME_IPv6,
    TLS_CERT_GNAME_DNS, TLS_CERT_GNAME_EMAIL, TLS_CERT_GNAME_URI, TLS_EXT_BASIC,
    TLS_EXT_EXTENDED_KEY_USAGE, TLS_EXT_KEY_USAGE, TLS_XKU_CODE_SIGN, TLS_XKU_DVCS,
    TLS_XKU_OCSP_SIGN, TLS_XKU_SGC, TLS_XKU_SMIME, TLS_XKU_SSL_CLIENT, TLS_XKU_SSL_SERVER,
    TLS_XKU_TIMESTAMP,
};
pub use self::tls_h::TLS_NO_CERT;
pub use self::tls_internal_h::{
    tls, tls_asn1_parse_time, tls_config, tls_conninfo, tls_error, tls_keypair, tls_ocsp_info,
    tls_ocsp_query, tls_set_error, tls_set_error_libssl, tls_set_errorx,
};
pub use self::types_h::{
    asn1_object_st, bignum_st, evp_md_st, ssl_ctx_st, ssl_st, x509_st, X509_name_st,
    ASN1_BIT_STRING, ASN1_BMPSTRING, ASN1_BOOLEAN, ASN1_ENUMERATED, ASN1_GENERALIZEDTIME,
    ASN1_GENERALSTRING, ASN1_IA5STRING, ASN1_INTEGER, ASN1_OBJECT, ASN1_OCTET_STRING,
    ASN1_PRINTABLESTRING, ASN1_STRING, ASN1_T61STRING, ASN1_TIME, ASN1_TYPE, ASN1_UNIVERSALSTRING,
    ASN1_UTCTIME, ASN1_UTF8STRING, ASN1_VISIBLESTRING, BIGNUM, EVP_MD, SSL, SSL_CTX, X509,
    X509_NAME,
};
pub use self::x509_h::{
    X509_NAME_ENTRY_get_data, X509_NAME_get_entry, X509_NAME_get_index_by_NID, X509_digest,
    X509_get_ext_d2i, X509_get_issuer_name, X509_get_serialNumber, X509_get_subject_name,
    X509_get_version, X509_getm_notAfter, X509_getm_notBefore, X509_name_entry_st,
    X509v3_KU_CRL_SIGN, X509v3_KU_DATA_ENCIPHERMENT, X509v3_KU_DECIPHER_ONLY,
    X509v3_KU_DIGITAL_SIGNATURE, X509v3_KU_ENCIPHER_ONLY, X509v3_KU_KEY_AGREEMENT,
    X509v3_KU_KEY_CERT_SIGN, X509v3_KU_KEY_ENCIPHERMENT, X509v3_KU_NON_REPUDIATION,
    X509_NAME_ENTRY,
};
pub use self::x509_vfy_h::X509_V_OK;
pub use self::x509v3_h::{
    ossl_check_GENERAL_NAME_freefunc_type, ossl_check_GENERAL_NAME_sk_type,
    ossl_check_const_GENERAL_NAME_sk_type, otherName_st, sk_GENERAL_NAME_freefunc,
    stack_st_GENERAL_NAME, BASIC_CONSTRAINTS_free, BASIC_CONSTRAINTS_st, C2RustUnnamed_0,
    EDIPartyName_st, GENERAL_NAME_free, GENERAL_NAME_st, X509_check_ca,
    X509_get_extended_key_usage, X509_get_key_usage, BASIC_CONSTRAINTS, EDIPARTYNAME,
    EXTENDED_KEY_USAGE, GENERAL_NAME, GEN_DNS, GEN_EMAIL, GEN_IPADD, GEN_URI, KU_CRL_SIGN,
    KU_DATA_ENCIPHERMENT, KU_DECIPHER_ONLY, KU_DIGITAL_SIGNATURE, KU_ENCIPHER_ONLY,
    KU_KEY_AGREEMENT, KU_KEY_CERT_SIGN, KU_KEY_ENCIPHERMENT, KU_NON_REPUDIATION, OTHERNAME,
    XKU_CODE_SIGN, XKU_DVCS, XKU_OCSP_SIGN, XKU_SGC, XKU_SMIME, XKU_SSL_CLIENT, XKU_SSL_SERVER,
    XKU_TIMESTAMP,
};
#[c2rust::src_loc = "36:9"]
pub const UB_COMMON_NAME: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
#[c2rust::src_loc = "37:9"]
pub const UB_COUNTRY_NAME: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
#[c2rust::src_loc = "38:9"]
pub const UB_STATE_NAME: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
#[c2rust::src_loc = "39:9"]
pub const UB_LOCALITY_NAME: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
#[c2rust::src_loc = "40:9"]
pub const UB_STREET_ADDRESS: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
#[c2rust::src_loc = "41:9"]
pub const UB_ORGANIZATION_NAME: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
#[c2rust::src_loc = "42:9"]
pub const UB_ORGANIZATIONAL_UNIT_NAME: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
#[c2rust::src_loc = "44:9"]
pub const UB_GNAME_DNS: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
#[c2rust::src_loc = "45:9"]
pub const UB_GNAME_EMAIL: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
#[c2rust::src_loc = "46:9"]
pub const UB_GNAME_URI: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
#[c2rust::src_loc = "49:1"]
unsafe extern "C" fn tls_parse_bigint(
    mut ctx: *mut tls,
    mut asn1int: *const ASN1_INTEGER,
    mut dst_p: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut small: ::core::ffi::c_long = 0;
    let mut big = ::core::ptr::null_mut::<BIGNUM>();
    let mut tmp = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut buf: [::core::ffi::c_char; 64] = [0; 64];
    *dst_p = ::core::ptr::null::<::core::ffi::c_char>();
    small = ASN1_INTEGER_get(asn1int);
    if small < 0 as ::core::ffi::c_long {
        big = ASN1_INTEGER_to_BN(asn1int, ::core::ptr::null_mut::<BIGNUM>());
        if !big.is_null() {
            tmp = BN_bn2dec(big);
            if !tmp.is_null() {
                *dst_p = strdup(tmp);
            }
            CRYPTO_free(
                tmp as *mut ::core::ffi::c_void,
                b"lib/usual/tls/tls_cert.c\0" as *const u8 as *const ::core::ffi::c_char,
                63 as ::core::ffi::c_int,
            );
        }
        BN_free(big);
    } else {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
            b"%lu\0" as *const u8 as *const ::core::ffi::c_char,
            small,
        );
        *dst_p = strdup(&raw mut buf as *mut ::core::ffi::c_char);
    }
    if !(*dst_p).is_null() {
        return 0 as ::core::ffi::c_int;
    }
    tls_set_errorx(
        ctx,
        b"cannot parse serial\0" as *const u8 as *const ::core::ffi::c_char,
    );
    return -(1 as ::core::ffi::c_int);
}
#[c2rust::src_loc = "87:1"]
unsafe extern "C" fn check_invalid_bytes(
    mut ctx: *mut tls,
    mut data: *const ::core::ffi::c_uchar,
    mut len: ::core::ffi::c_uint,
    mut ascii_only: ::core::ffi::c_int,
    mut desc: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_uint = 0;
    let mut c: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    loop {
        if !(i < len) {
            current_block = 9606288038608642794;
            break;
        }
        c = *data.offset(i as isize) as ::core::ffi::c_uint;
        if ascii_only != 0 && c & 0x80 as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint {
            tls_set_errorx(
                ctx,
                b"invalid %s: contains non-ascii in ascii string\0" as *const u8
                    as *const ::core::ffi::c_char,
                desc,
            );
            current_block = 17825605587671619221;
            break;
        } else {
            if c < 0x20 as ::core::ffi::c_uint {
                if c != '\t' as i32 as ::core::ffi::c_uint
                    && c != '\n' as i32 as ::core::ffi::c_uint
                    && c != '\r' as i32 as ::core::ffi::c_uint
                {
                    tls_set_errorx(
                        ctx,
                        b"invalid %s: contains C0 control char\0" as *const u8
                            as *const ::core::ffi::c_char,
                        desc,
                    );
                    current_block = 17825605587671619221;
                    break;
                }
            } else if c == 0xc2 as ::core::ffi::c_uint
                && i.wrapping_add(1 as ::core::ffi::c_uint) < len
            {
                c = *data.offset(i.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                    as ::core::ffi::c_uint;
                if c >= 0x80 as ::core::ffi::c_uint && c <= 0x9f as ::core::ffi::c_uint {
                    tls_set_errorx(
                        ctx,
                        b"invalid %s: contains C1 control char\0" as *const u8
                            as *const ::core::ffi::c_char,
                        desc,
                    );
                    current_block = 17825605587671619221;
                    break;
                }
            } else if c == 0x7f as ::core::ffi::c_uint {
                tls_set_errorx(
                    ctx,
                    b"invalid %s: contains DEL char\0" as *const u8 as *const ::core::ffi::c_char,
                    desc,
                );
                current_block = 17825605587671619221;
                break;
            }
            i = i.wrapping_add(1);
        }
    }
    match current_block {
        17825605587671619221 => return -(1 as ::core::ffi::c_int),
        _ => return 0 as ::core::ffi::c_int,
    };
}
#[c2rust::src_loc = "122:1"]
unsafe extern "C" fn tls_parse_asn1string(
    mut ctx: *mut tls,
    mut a1str: *mut ASN1_STRING,
    mut dst_p: *mut *const ::core::ffi::c_char,
    mut minchars: ::core::ffi::c_int,
    mut maxchars: ::core::ffi::c_int,
    mut desc: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut format: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut ret = -(1 as ::core::ffi::c_int);
    let mut data = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut a1utf = ::core::ptr::null_mut::<ASN1_STRING>();
    let mut ascii_only = 0 as ::core::ffi::c_int;
    let mut cstr = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut mbres: ::core::ffi::c_int = 0;
    let mut mbconvert = -(1 as ::core::ffi::c_int);
    *dst_p = ::core::ptr::null::<::core::ffi::c_char>();
    format = ASN1_STRING_type(a1str);
    data = ASN1_STRING_get0_data(a1str);
    len = ASN1_STRING_length(a1str);
    if len < minchars {
        tls_set_errorx(
            ctx,
            b"invalid %s: string too short\0" as *const u8 as *const ::core::ffi::c_char,
            desc,
        );
    } else {
        match format {
            V_ASN1_NUMERICSTRING
            | V_ASN1_VISIBLESTRING
            | V_ASN1_PRINTABLESTRING
            | V_ASN1_IA5STRING => {
                if len > maxchars {
                    tls_set_errorx(
                        ctx,
                        b"invalid %s: string too long\0" as *const u8 as *const ::core::ffi::c_char,
                        desc,
                    );
                    current_block = 13366883036718171271;
                } else {
                    ascii_only = 1 as ::core::ffi::c_int;
                    current_block = 13242334135786603907;
                }
            }
            V_ASN1_T61STRING => {
                mbconvert = MBSTRING_ASC;
                current_block = 13242334135786603907;
            }
            V_ASN1_BMPSTRING => {
                mbconvert = MBSTRING_BMP;
                current_block = 13242334135786603907;
            }
            V_ASN1_UNIVERSALSTRING => {
                mbconvert = MBSTRING_UNIV;
                current_block = 13242334135786603907;
            }
            V_ASN1_UTF8STRING => {
                mbconvert = MBSTRING_UTF8;
                current_block = 13242334135786603907;
            }
            _ => {
                tls_set_errorx(
                    ctx,
                    b"invalid %s: unexpected string type\0" as *const u8
                        as *const ::core::ffi::c_char,
                    desc,
                );
                current_block = 13366883036718171271;
            }
        }
        match current_block {
            13366883036718171271 => {}
            _ => {
                if mbconvert != -(1 as ::core::ffi::c_int) {
                    mbres = ASN1_mbstring_ncopy(
                        &raw mut a1utf,
                        data,
                        len,
                        mbconvert,
                        B_ASN1_UTF8STRING as ::core::ffi::c_ulong,
                        minchars as ::core::ffi::c_long,
                        maxchars as ::core::ffi::c_long,
                    );
                    if mbres < 0 as ::core::ffi::c_int {
                        tls_set_error_libssl(
                            ctx,
                            b"invalid %s\0" as *const u8 as *const ::core::ffi::c_char,
                            desc,
                        );
                        current_block = 13366883036718171271;
                    } else if mbres != V_ASN1_UTF8STRING {
                        tls_set_errorx(
                            ctx,
                            b"multibyte conversion failed: expected UTF8 result\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        current_block = 13366883036718171271;
                    } else {
                        data = ASN1_STRING_get0_data(a1utf);
                        len = ASN1_STRING_length(a1utf);
                        current_block = 4068382217303356765;
                    }
                } else {
                    current_block = 4068382217303356765;
                }
                match current_block {
                    13366883036718171271 => {}
                    _ => {
                        if !memchr(
                            data as *const ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            len as size_t,
                        )
                        .is_null()
                        {
                            tls_set_errorx(
                                ctx,
                                b"invalid %s: contains NUL\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                desc,
                            );
                        } else if !(check_invalid_bytes(
                            ctx,
                            data,
                            len as ::core::ffi::c_uint,
                            ascii_only,
                            desc,
                        ) < 0 as ::core::ffi::c_int)
                        {
                            cstr = malloc((len + 1 as ::core::ffi::c_int) as size_t)
                                as *mut ::core::ffi::c_char;
                            if cstr.is_null() {
                                tls_set_error(
                                    ctx,
                                    b"malloc\0" as *const u8 as *const ::core::ffi::c_char,
                                );
                            } else {
                                memcpy(
                                    cstr as *mut ::core::ffi::c_void,
                                    data as *const ::core::ffi::c_void,
                                    len as size_t,
                                );
                                *cstr.offset(len as isize) = 0 as ::core::ffi::c_char;
                                *dst_p = cstr;
                                ret = len;
                            }
                        }
                    }
                }
            }
        }
    }
    ASN1_STRING_free(a1utf);
    return ret;
}
#[c2rust::src_loc = "217:1"]
unsafe extern "C" fn tls_cert_get_dname_string(
    mut ctx: *mut tls,
    mut name: *mut X509_NAME,
    mut nid: ::core::ffi::c_int,
    mut str_p: *mut *const ::core::ffi::c_char,
    mut minchars: ::core::ffi::c_int,
    mut maxchars: ::core::ffi::c_int,
    mut desc: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut loc: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut ne = ::core::ptr::null_mut::<X509_NAME_ENTRY>();
    let mut a1str = ::core::ptr::null_mut::<ASN1_STRING>();
    *str_p = ::core::ptr::null::<::core::ffi::c_char>();
    loc = X509_NAME_get_index_by_NID(name, nid, -(1 as ::core::ffi::c_int));
    if loc < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    ne = X509_NAME_get_entry(name, loc);
    if ne.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    a1str = X509_NAME_ENTRY_get_data(ne);
    if a1str.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    len = tls_parse_asn1string(ctx, a1str, str_p, minchars, maxchars, desc);
    if len < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "240:1"]
unsafe extern "C" fn tls_load_alt_ia5string(
    mut ctx: *mut tls,
    mut ia5str: *mut ASN1_IA5STRING,
    mut cert: *mut tls_cert,
    mut slot_type: ::core::ffi::c_int,
    mut minchars: ::core::ffi::c_int,
    mut maxchars: ::core::ffi::c_int,
    mut desc: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut slot = ::core::ptr::null_mut::<tls_cert_general_name>();
    let mut data = ::core::ptr::null::<::core::ffi::c_char>();
    let mut len: ::core::ffi::c_int = 0;
    slot = (*cert)
        .subject_alt_names
        .offset((*cert).subject_alt_name_count as isize) as *mut tls_cert_general_name;
    len = tls_parse_asn1string(
        ctx,
        ia5str as *mut ASN1_STRING,
        &raw mut data,
        minchars,
        maxchars,
        desc,
    );
    if len < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if len == 1 as ::core::ffi::c_int
        && *data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32
    {
        tls_set_errorx(
            ctx,
            b"invalid %s: single space\0" as *const u8 as *const ::core::ffi::c_char,
            desc,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*slot).name_value = data as *const ::core::ffi::c_void;
    (*slot).name_type = slot_type;
    (*cert).subject_alt_name_count += 1;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "269:1"]
unsafe extern "C" fn tls_load_alt_ipaddr(
    mut ctx: *mut tls,
    mut bin: *mut ASN1_OCTET_STRING,
    mut cert: *mut tls_cert,
) -> ::core::ffi::c_int {
    let mut slot = ::core::ptr::null_mut::<tls_cert_general_name>();
    let mut data = ::core::ptr::null::<::core::ffi::c_void>();
    let mut len: ::core::ffi::c_int = 0;
    slot = (*cert)
        .subject_alt_names
        .offset((*cert).subject_alt_name_count as isize) as *mut tls_cert_general_name;
    len = ASN1_STRING_length(bin);
    data = ASN1_STRING_get0_data(bin) as *const ::core::ffi::c_void;
    if len < 0 as ::core::ffi::c_int {
        tls_set_errorx(
            ctx,
            b"negative length for ipaddress\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if len == 4 as ::core::ffi::c_int {
        (*slot).name_type = TLS_CERT_GNAME_IPv4;
    } else if len == 16 as ::core::ffi::c_int {
        (*slot).name_type = TLS_CERT_GNAME_IPv6;
    } else {
        tls_set_errorx(
            ctx,
            b"invalid length for ipaddress\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*slot).name_value = malloc(len as size_t);
    if (*slot).name_value.is_null() {
        tls_set_error(ctx, b"malloc\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    memcpy(
        (*slot).name_value as *mut ::core::ffi::c_void,
        data,
        len as size_t,
    );
    (*cert).subject_alt_name_count += 1;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "308:1"]
unsafe extern "C" fn tls_cert_get_altnames(
    mut ctx: *mut tls,
    mut cert: *mut tls_cert,
    mut x509_cert: *mut X509,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut altname_stack = ::core::ptr::null_mut::<stack_st_GENERAL_NAME>();
    let mut altname = ::core::ptr::null_mut::<GENERAL_NAME>();
    let mut count: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut rv = -(1 as ::core::ffi::c_int);
    altname_stack = X509_get_ext_d2i(
        x509_cert,
        NID_subject_alt_name,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    ) as *mut stack_st_GENERAL_NAME;
    if altname_stack.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    count = OPENSSL_sk_num(ossl_check_const_GENERAL_NAME_sk_type(altname_stack));
    if count == 0 as ::core::ffi::c_int {
        rv = 0 as ::core::ffi::c_int;
    } else {
        (*cert).subject_alt_names = calloc(
            count as size_t,
            ::core::mem::size_of::<tls_cert_general_name>() as size_t,
        ) as *mut tls_cert_general_name;
        if (*cert).subject_alt_names.is_null() {
            tls_set_error(ctx, b"calloc\0" as *const u8 as *const ::core::ffi::c_char);
        } else {
            i = 0 as ::core::ffi::c_int;
            loop {
                if !(i < count) {
                    current_block = 17478428563724192186;
                    break;
                }
                altname = OPENSSL_sk_value(ossl_check_const_GENERAL_NAME_sk_type(altname_stack), i)
                    as *mut GENERAL_NAME;
                if (*altname).type_0 == GEN_DNS {
                    rv = tls_load_alt_ia5string(
                        ctx,
                        (*altname).d.dNSName,
                        cert,
                        TLS_CERT_GNAME_DNS,
                        1 as ::core::ffi::c_int,
                        UB_GNAME_DNS,
                        b"dns\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else if (*altname).type_0 == GEN_EMAIL {
                    rv = tls_load_alt_ia5string(
                        ctx,
                        (*altname).d.rfc822Name,
                        cert,
                        TLS_CERT_GNAME_EMAIL,
                        1 as ::core::ffi::c_int,
                        UB_GNAME_EMAIL,
                        b"email\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else if (*altname).type_0 == GEN_URI {
                    rv = tls_load_alt_ia5string(
                        ctx,
                        (*altname).d.uniformResourceIdentifier,
                        cert,
                        TLS_CERT_GNAME_URI,
                        1 as ::core::ffi::c_int,
                        UB_GNAME_URI,
                        b"uri\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else if (*altname).type_0 == GEN_IPADD {
                    rv = tls_load_alt_ipaddr(ctx, (*altname).d.iPAddress, cert);
                } else {
                    rv = 0 as ::core::ffi::c_int;
                }
                if rv < 0 as ::core::ffi::c_int {
                    current_block = 8872553975001083413;
                    break;
                }
                i += 1;
            }
            match current_block {
                8872553975001083413 => {}
                _ => {
                    rv = 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    OPENSSL_sk_pop_free(
        ossl_check_GENERAL_NAME_sk_type(altname_stack),
        ossl_check_GENERAL_NAME_freefunc_type(Some(
            GENERAL_NAME_free as unsafe extern "C" fn(*mut GENERAL_NAME) -> (),
        )),
    );
    return rv;
}
#[c2rust::src_loc = "355:1"]
unsafe extern "C" fn tls_get_dname(
    mut ctx: *mut tls,
    mut name: *mut X509_NAME,
    mut dname: *mut tls_cert_dname,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    ret = tls_cert_get_dname_string(
        ctx,
        name,
        NID_commonName,
        &raw mut (*dname).common_name,
        0 as ::core::ffi::c_int,
        UB_COMMON_NAME,
        b"commonName\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if ret == 0 as ::core::ffi::c_int {
        ret = tls_cert_get_dname_string(
            ctx,
            name,
            NID_countryName,
            &raw mut (*dname).country_name,
            0 as ::core::ffi::c_int,
            UB_COUNTRY_NAME,
            b"countryName\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ret == 0 as ::core::ffi::c_int {
        ret = tls_cert_get_dname_string(
            ctx,
            name,
            NID_stateOrProvinceName,
            &raw mut (*dname).state_or_province_name,
            0 as ::core::ffi::c_int,
            UB_STATE_NAME,
            b"stateName\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ret == 0 as ::core::ffi::c_int {
        ret = tls_cert_get_dname_string(
            ctx,
            name,
            NID_localityName,
            &raw mut (*dname).locality_name,
            0 as ::core::ffi::c_int,
            UB_LOCALITY_NAME,
            b"localityName\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ret == 0 as ::core::ffi::c_int {
        ret = tls_cert_get_dname_string(
            ctx,
            name,
            NID_streetAddress,
            &raw mut (*dname).street_address,
            0 as ::core::ffi::c_int,
            UB_STREET_ADDRESS,
            b"streetAddress\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ret == 0 as ::core::ffi::c_int {
        ret = tls_cert_get_dname_string(
            ctx,
            name,
            NID_organizationName,
            &raw mut (*dname).organization_name,
            0 as ::core::ffi::c_int,
            UB_ORGANIZATION_NAME,
            b"organizationName\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ret == 0 as ::core::ffi::c_int {
        ret = tls_cert_get_dname_string(
            ctx,
            name,
            NID_organizationalUnitName,
            &raw mut (*dname).organizational_unit_name,
            0 as ::core::ffi::c_int,
            UB_ORGANIZATIONAL_UNIT_NAME,
            b"organizationalUnitName\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return ret;
}
#[c2rust::src_loc = "387:1"]
unsafe extern "C" fn tls_get_basic_constraints(
    mut ctx: *mut tls,
    mut cert: *mut tls_cert,
    mut x509: *mut X509,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut bc = ::core::ptr::null_mut::<BASIC_CONSTRAINTS>();
    let mut crit: ::core::ffi::c_int = 0;
    let mut ret = -(1 as ::core::ffi::c_int);
    bc = X509_get_ext_d2i(
        x509,
        NID_basic_constraints,
        &raw mut crit,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    ) as *mut BASIC_CONSTRAINTS;
    if bc.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (*cert).ext_set |= TLS_EXT_BASIC as uint32_t;
    if crit != 0 {
        (*cert).ext_crit |= TLS_EXT_BASIC as uint32_t;
    }
    (*cert).basic_constraints_ca = if (*bc).ca != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    if !(*bc).pathlen.is_null() {
        (*cert).basic_constraints_pathlen = ASN1_INTEGER_get((*bc).pathlen) as ::core::ffi::c_int;
        if (*cert).basic_constraints_pathlen < 0 as ::core::ffi::c_int {
            tls_set_error(
                ctx,
                b"BasicConstraints has invalid pathlen\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            current_block = 4354887704369180111;
        } else {
            current_block = 7746791466490516765;
        }
    } else {
        (*cert).basic_constraints_pathlen = -(1 as ::core::ffi::c_int);
        current_block = 7746791466490516765;
    }
    match current_block {
        7746791466490516765 => {
            ret = 0 as ::core::ffi::c_int;
        }
        _ => {}
    }
    BASIC_CONSTRAINTS_free(bc);
    return ret;
}
#[c2rust::src_loc = "417:1"]
unsafe extern "C" fn map_bits(mut map: *const [uint32_t; 2], mut input: uint32_t) -> uint32_t {
    let mut i: uint32_t = 0;
    let mut out: uint32_t = 0 as uint32_t;
    i = 0 as uint32_t;
    while (*map.offset(i as isize))[0 as ::core::ffi::c_int as usize] != 0 {
        if (*map.offset(i as isize))[0 as ::core::ffi::c_int as usize] & input != 0 {
            out |= (*map.offset(i as isize))[1 as ::core::ffi::c_int as usize];
        }
        i = i.wrapping_add(1);
    }
    return out;
}
#[c2rust::src_loc = "427:1"]
unsafe extern "C" fn tls_get_key_usage(
    mut ctx: *mut tls,
    mut cert: *mut tls_cert,
    mut x509: *mut X509,
) -> ::core::ffi::c_int {
    static mut ku_map: [[uint32_t; 2]; 10] = [
        [
            KU_DIGITAL_SIGNATURE as uint32_t,
            KU_DIGITAL_SIGNATURE as uint32_t,
        ],
        [
            KU_NON_REPUDIATION as uint32_t,
            KU_NON_REPUDIATION as uint32_t,
        ],
        [
            KU_KEY_ENCIPHERMENT as uint32_t,
            KU_KEY_ENCIPHERMENT as uint32_t,
        ],
        [
            KU_DATA_ENCIPHERMENT as uint32_t,
            KU_DATA_ENCIPHERMENT as uint32_t,
        ],
        [KU_KEY_AGREEMENT as uint32_t, KU_KEY_AGREEMENT as uint32_t],
        [KU_KEY_CERT_SIGN as uint32_t, KU_KEY_CERT_SIGN as uint32_t],
        [KU_CRL_SIGN as uint32_t, KU_CRL_SIGN as uint32_t],
        [KU_ENCIPHER_ONLY as uint32_t, KU_ENCIPHER_ONLY as uint32_t],
        [KU_DECIPHER_ONLY as uint32_t, KU_DECIPHER_ONLY as uint32_t],
        [
            0 as ::core::ffi::c_int as uint32_t,
            0 as ::core::ffi::c_int as uint32_t,
        ],
    ];
    let mut ku = ::core::ptr::null_mut::<ASN1_BIT_STRING>();
    let mut crit: ::core::ffi::c_int = 0;
    ku = X509_get_ext_d2i(
        x509,
        NID_key_usage,
        &raw mut crit,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    ) as *mut ASN1_BIT_STRING;
    if ku.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (*cert).ext_set |= TLS_EXT_KEY_USAGE as uint32_t;
    if crit != 0 {
        (*cert).ext_crit |= TLS_EXT_KEY_USAGE as uint32_t;
    }
    ASN1_BIT_STRING_free(ku);
    (*cert).key_usage_flags = map_bits(
        &raw const ku_map as *const [uint32_t; 2],
        X509_get_key_usage(x509),
    );
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "457:1"]
unsafe extern "C" fn tls_get_ext_key_usage(
    mut ctx: *mut tls,
    mut cert: *mut tls_cert,
    mut x509: *mut X509,
) -> ::core::ffi::c_int {
    static mut xku_map: [[uint32_t; 2]; 9] = [
        [XKU_SSL_SERVER as uint32_t, TLS_XKU_SSL_SERVER as uint32_t],
        [XKU_SSL_CLIENT as uint32_t, TLS_XKU_SSL_CLIENT as uint32_t],
        [XKU_SMIME as uint32_t, TLS_XKU_SMIME as uint32_t],
        [XKU_CODE_SIGN as uint32_t, TLS_XKU_CODE_SIGN as uint32_t],
        [XKU_SGC as uint32_t, TLS_XKU_SGC as uint32_t],
        [XKU_OCSP_SIGN as uint32_t, TLS_XKU_OCSP_SIGN as uint32_t],
        [XKU_TIMESTAMP as uint32_t, TLS_XKU_TIMESTAMP as uint32_t],
        [XKU_DVCS as uint32_t, TLS_XKU_DVCS as uint32_t],
        [
            0 as ::core::ffi::c_int as uint32_t,
            0 as ::core::ffi::c_int as uint32_t,
        ],
    ];
    let mut xku = ::core::ptr::null_mut::<EXTENDED_KEY_USAGE>();
    let mut crit: ::core::ffi::c_int = 0;
    xku = X509_get_ext_d2i(
        x509,
        NID_ext_key_usage,
        &raw mut crit,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    ) as *mut EXTENDED_KEY_USAGE;
    if xku.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    OPENSSL_sk_pop_free(
        ossl_check_ASN1_OBJECT_sk_type(xku as *mut stack_st_ASN1_OBJECT),
        ossl_check_ASN1_OBJECT_freefunc_type(Some(
            ASN1_OBJECT_free as unsafe extern "C" fn(*mut ASN1_OBJECT) -> (),
        )),
    );
    (*cert).ext_set |= TLS_EXT_EXTENDED_KEY_USAGE as uint32_t;
    if crit != 0 {
        (*cert).ext_crit |= TLS_EXT_EXTENDED_KEY_USAGE as uint32_t;
    }
    (*cert).extended_key_usage_flags = map_bits(
        &raw const xku_map as *const [uint32_t; 2],
        X509_get_extended_key_usage(x509),
    );
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "486:1"]
unsafe extern "C" fn tls_load_extensions(
    mut ctx: *mut tls,
    mut cert: *mut tls_cert,
    mut x509: *mut X509,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    X509_check_ca(x509);
    ret = tls_get_basic_constraints(ctx, cert, x509);
    if ret == 0 as ::core::ffi::c_int {
        ret = tls_get_key_usage(ctx, cert, x509);
    }
    if ret == 0 as ::core::ffi::c_int {
        ret = tls_get_ext_key_usage(ctx, cert, x509);
    }
    if ret == 0 as ::core::ffi::c_int {
        ret = tls_cert_get_altnames(ctx, cert, x509);
    }
    return ret;
}
#[c2rust::src_loc = "506:1"]
unsafe extern "C" fn tls_calc_fingerprint(
    mut ctx: *mut tls,
    mut x509: *mut X509,
    mut algo: *const ::core::ffi::c_char,
    mut outlen: *mut size_t,
) -> *mut ::core::ffi::c_void {
    let mut md = ::core::ptr::null::<EVP_MD>();
    let mut res = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut ret: ::core::ffi::c_int = 0;
    let mut tmplen: ::core::ffi::c_uint = 0;
    let mut mdlen: ::core::ffi::c_uint = 0;
    if !outlen.is_null() {
        *outlen = 0 as size_t;
    }
    if strcasecmp(algo, b"sha1\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        md = EVP_sha1();
    } else if strcasecmp(algo, b"sha256\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        md = EVP_sha256();
    } else {
        tls_set_errorx(
            ctx,
            b"invalid fingerprint algorithm\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return NULL;
    }
    mdlen = EVP_MD_get_size(md) as ::core::ffi::c_uint;
    res = malloc(mdlen as size_t);
    if res.is_null() {
        tls_set_error(ctx, b"malloc\0" as *const u8 as *const ::core::ffi::c_char);
        return NULL;
    }
    ret = X509_digest(x509, md, res as *mut ::core::ffi::c_uchar, &raw mut tmplen);
    if ret != 1 as ::core::ffi::c_int || tmplen != mdlen {
        free(res);
        tls_set_errorx(
            ctx,
            b"X509_digest failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return NULL;
    }
    if !outlen.is_null() {
        *outlen = mdlen as size_t;
    }
    return res;
}
#[c2rust::src_loc = "545:1"]
unsafe extern "C" fn check_verify_error(mut ctx: *mut tls, mut cert: *mut tls_cert) {
    let mut vres = SSL_get_verify_result((*ctx).ssl_conn);
    if vres == X509_V_OK as ::core::ffi::c_long {
        (*cert).successful_verify = 1 as ::core::ffi::c_int;
    } else {
        (*cert).successful_verify = 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
#[c2rust::src_loc = "555:1"]
pub unsafe extern "C" fn tls_parse_cert(
    mut ctx: *mut tls,
    mut cert_p: *mut *mut tls_cert,
    mut fingerprint_algo: *const ::core::ffi::c_char,
    mut x509: *mut X509,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut cert = ::core::ptr::null_mut::<tls_cert>();
    let mut subject = ::core::ptr::null_mut::<X509_NAME>();
    let mut issuer = ::core::ptr::null_mut::<X509_NAME>();
    let mut ret = -(1 as ::core::ffi::c_int);
    let mut version: ::core::ffi::c_long = 0;
    *cert_p = ::core::ptr::null_mut::<tls_cert>();
    version = X509_get_version(x509);
    if version < 0 as ::core::ffi::c_long {
        tls_set_errorx(
            ctx,
            b"invalid version\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    subject = X509_get_subject_name(x509);
    if subject.is_null() {
        tls_set_errorx(
            ctx,
            b"cert does not have subject\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    issuer = X509_get_issuer_name(x509);
    if issuer.is_null() {
        tls_set_errorx(
            ctx,
            b"cert does not have issuer\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    cert = calloc(1 as size_t, ::core::mem::size_of::<tls_cert>() as size_t) as *mut tls_cert;
    if cert.is_null() {
        tls_set_error(ctx, b"calloc\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        (*cert).version = version as ::core::ffi::c_int;
        if !fingerprint_algo.is_null() {
            (*cert).fingerprint = tls_calc_fingerprint(
                ctx,
                x509,
                fingerprint_algo,
                &raw mut (*cert).fingerprint_size,
            ) as *const ::core::ffi::c_uchar;
            if (*cert).fingerprint.is_null() {
                current_block = 14928171562513889795;
            } else {
                current_block = 8457315219000651999;
            }
        } else {
            current_block = 8457315219000651999;
        }
        match current_block {
            14928171562513889795 => {}
            _ => {
                ret = tls_get_dname(ctx, subject, &raw mut (*cert).subject);
                if ret == 0 as ::core::ffi::c_int {
                    ret = tls_get_dname(ctx, issuer, &raw mut (*cert).issuer);
                }
                if ret == 0 as ::core::ffi::c_int {
                    ret = tls_asn1_parse_time(
                        ctx,
                        X509_getm_notBefore(x509),
                        &raw mut (*cert).not_before,
                    );
                }
                if ret == 0 as ::core::ffi::c_int {
                    ret = tls_asn1_parse_time(
                        ctx,
                        X509_getm_notAfter(x509),
                        &raw mut (*cert).not_after,
                    );
                }
                if ret == 0 as ::core::ffi::c_int {
                    ret =
                        tls_parse_bigint(ctx, X509_get_serialNumber(x509), &raw mut (*cert).serial);
                }
                if ret == 0 as ::core::ffi::c_int {
                    ret = tls_load_extensions(ctx, cert, x509);
                }
                if ret == 0 as ::core::ffi::c_int {
                    *cert_p = cert;
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    tls_cert_free(cert);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "615:1"]
pub unsafe extern "C" fn tls_get_peer_cert(
    mut ctx: *mut tls,
    mut cert_p: *mut *mut tls_cert,
    mut fingerprint_algo: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut peer = (*ctx).ssl_peer_cert;
    let mut res: ::core::ffi::c_int = 0;
    *cert_p = ::core::ptr::null_mut::<tls_cert>();
    if peer.is_null() {
        tls_set_errorx(
            ctx,
            b"peer does not have cert\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return TLS_NO_CERT;
    }
    ERR_clear_error();
    res = tls_parse_cert(ctx, cert_p, fingerprint_algo, peer);
    if res == 0 as ::core::ffi::c_int {
        check_verify_error(ctx, *cert_p);
    }
    ERR_clear_error();
    return res;
}
#[c2rust::src_loc = "635:1"]
unsafe extern "C" fn tls_cert_free_dname(mut dname: *mut tls_cert_dname) {
    free((*dname).common_name as *mut ::core::ffi::c_void);
    free((*dname).country_name as *mut ::core::ffi::c_void);
    free((*dname).state_or_province_name as *mut ::core::ffi::c_void);
    free((*dname).locality_name as *mut ::core::ffi::c_void);
    free((*dname).street_address as *mut ::core::ffi::c_void);
    free((*dname).organization_name as *mut ::core::ffi::c_void);
    free((*dname).organizational_unit_name as *mut ::core::ffi::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "646:1"]
pub unsafe extern "C" fn tls_cert_free(mut cert: *mut tls_cert) {
    let mut i: ::core::ffi::c_int = 0;
    if cert.is_null() {
        return;
    }
    tls_cert_free_dname(&raw mut (*cert).issuer);
    tls_cert_free_dname(&raw mut (*cert).subject);
    if (*cert).subject_alt_name_count != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < (*cert).subject_alt_name_count {
            free(
                (*(*cert).subject_alt_names.offset(i as isize)).name_value
                    as *mut ::core::ffi::c_void,
            );
            i += 1;
        }
    }
    free((*cert).subject_alt_names as *mut ::core::ffi::c_void);
    free((*cert).serial as *mut ::core::ffi::c_void);
    free((*cert).fingerprint as *mut ::core::ffi::c_void);
    free(cert as *mut ::core::ffi::c_void);
}
