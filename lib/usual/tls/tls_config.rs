#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:18"]
pub mod _types_h {
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:18"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:18"]
pub mod _uint8_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint8_t = u8;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:18"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = u32;
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/types.h:18"]
pub mod types_h {
    #[c2rust::src_loc = "197:1"]
    pub type SSL_CTX = ssl_ctx_st;
    extern "C" {
        #[c2rust::src_loc = "197:9"]
        pub type ssl_ctx_st;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/tls/tls_internal.h:24"]
pub mod tls_internal_h {
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
    #[c2rust::src_loc = "26:9"]
    pub const _PATH_SSL_CA_FILE: [::core::ffi::c_char; 18] = USUAL_TLS_CA_FILE;
    use super::_size_t_h::size_t;
    use super::_uint32_t_h::uint32_t;
    use super::config_h::USUAL_TLS_CA_FILE;
    extern "C" {
        #[c2rust::src_loc = "152:1"]
        pub fn tls_config_set_errorx(
            cfg: *mut tls_config,
            fmt: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/ssl.h:18"]
pub mod ssl_h {
    #[c2rust::src_loc = "233:1"]
    pub type SSL_METHOD = ssl_method_st;
    use super::types_h::SSL_CTX;
    extern "C" {
        #[c2rust::src_loc = "233:9"]
        pub type ssl_method_st;
        #[c2rust::src_loc = "1620:8"]
        pub fn SSL_CTX_set_cipher_list(
            _: *mut SSL_CTX,
            str: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1621:8"]
        pub fn SSL_CTX_new(meth: *const SSL_METHOD) -> *mut SSL_CTX;
        #[c2rust::src_loc = "1625:1"]
        pub fn SSL_CTX_free(_: *mut SSL_CTX);
        #[c2rust::src_loc = "2058:8"]
        pub fn TLS_method() -> *const SSL_METHOD;
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:18"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:18"]
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
        #[c2rust::src_loc = "141:1"]
        pub fn strdup(__s1: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "204:1"]
        pub fn strsep(
            __stringp: *mut *mut ::core::ffi::c_char,
            __delim: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/string.h:18"]
pub mod string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "153:1"]
        pub fn usual_explicit_bzero(buf: *mut ::core::ffi::c_void, len: size_t);
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/config.h:18"]
pub mod config_h {
    #[c2rust::src_loc = "396:9"]
    pub const USUAL_TLS_CA_FILE: [::core::ffi::c_char; 18] = unsafe {
        ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"/etc/ssl/cert.pem\0")
    };
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:18"]
pub mod sys__types_h {
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
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
    #[c2rust::src_loc = "33:9"]
    pub const TLS_PROTOCOL_TLSv1: ::core::ffi::c_int =
        TLS_PROTOCOL_TLSv1_0 | TLS_PROTOCOL_TLSv1_1 | TLS_PROTOCOL_TLSv1_2 | TLS_PROTOCOL_TLSv1_3;
    #[c2rust::src_loc = "36:9"]
    pub const TLS_PROTOCOLS_ALL: ::core::ffi::c_int = TLS_PROTOCOL_TLSv1;
    #[c2rust::src_loc = "37:9"]
    pub const TLS_PROTOCOLS_DEFAULT: ::core::ffi::c_int =
        TLS_PROTOCOL_TLSv1_2 | TLS_PROTOCOL_TLSv1_3;
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
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/obj_mac.h:18"]
pub mod obj_mac_h {
    #[c2rust::src_loc = "18:9"]
    pub const NID_undef: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/opt/homebrew/Cellar/openssl@3/3.6.0/include/openssl/objects.h:18"]
pub mod objects_h {
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn OBJ_txt2nid(s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    }
}
use self::_malloc_h::{calloc, free, malloc};
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
use self::_string_h::{memcpy, strdup, strlen, strsep};
use self::_strings_h::strcasecmp;
pub use self::_types_h::__darwin_size_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::config_h::USUAL_TLS_CA_FILE;
pub use self::obj_mac_h::NID_undef;
use self::objects_h::OBJ_txt2nid;
pub use self::ssl_h::{
    ssl_method_st, SSL_CTX_free, SSL_CTX_new, SSL_CTX_set_cipher_list, TLS_method, SSL_METHOD,
};
use self::string_h::usual_explicit_bzero;
pub use self::sys__types_h::__DARWIN_NULL;
pub use self::tls_h::{
    TLS_PROTOCOL_TLSv1, TLS_PROTOCOL_TLSv1_0, TLS_PROTOCOL_TLSv1_1, TLS_PROTOCOL_TLSv1_2,
    TLS_PROTOCOL_TLSv1_3, TLS_PROTOCOLS_ALL, TLS_PROTOCOLS_DEFAULT,
};
pub use self::tls_internal_h::{
    tls_config, tls_config_set_errorx, tls_error, tls_keypair, _PATH_SSL_CA_FILE,
};
pub use self::types_h::{ssl_ctx_st, SSL_CTX};
#[c2rust::src_loc = "26:1"]
unsafe extern "C" fn set_string(
    mut dest: *mut *const ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    free(*dest as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    *dest = ::core::ptr::null::<::core::ffi::c_char>();
    if !src.is_null() {
        *dest = strdup(src);
        if (*dest).is_null() {
            return -(1 as ::core::ffi::c_int);
        }
    }
    0 as ::core::ffi::c_int
}
#[c2rust::src_loc = "37:1"]
unsafe extern "C" fn memdup(
    mut in_0: *const ::core::ffi::c_void,
    mut len: size_t,
) -> *mut ::core::ffi::c_void {
    let mut out = ::core::ptr::null_mut::<::core::ffi::c_void>();
    out = malloc(len);
    if out.is_null() {
        return NULL;
    }
    memcpy(out, in_0, len);
    out
}
#[c2rust::src_loc = "47:1"]
unsafe extern "C" fn set_mem(
    mut dest: *mut *mut ::core::ffi::c_char,
    mut destlen: *mut size_t,
    mut src: *const ::core::ffi::c_void,
    mut srclen: size_t,
) -> ::core::ffi::c_int {
    free(*dest as *mut ::core::ffi::c_void);
    *dest = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *destlen = 0 as size_t;
    if !src.is_null() {
        *dest = memdup(src, srclen) as *mut ::core::ffi::c_char;
        if (*dest).is_null() {
            return -(1 as ::core::ffi::c_int);
        }
    }
    *destlen = srclen;
    0 as ::core::ffi::c_int
}
#[no_mangle]
#[c2rust::src_loc = "60:1"]
pub unsafe extern "C" fn tls_keypair_new() -> *mut tls_keypair {
    calloc(1 as size_t, ::core::mem::size_of::<tls_keypair>() as size_t) as *mut tls_keypair
}
#[no_mangle]
#[c2rust::src_loc = "65:1"]
pub unsafe extern "C" fn tls_keypair_set_cert_file(
    mut keypair: *mut tls_keypair,
    mut cert_file: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    set_string(&raw mut (*keypair).cert_file, cert_file)
}
#[c2rust::src_loc = "70:1"]
unsafe extern "C" fn tls_keypair_set_cert_mem(
    mut keypair: *mut tls_keypair,
    mut cert: *const uint8_t,
    mut len: size_t,
) -> ::core::ffi::c_int {
    set_mem(
        &raw mut (*keypair).cert_mem,
        &raw mut (*keypair).cert_len,
        cert as *const ::core::ffi::c_void,
        len,
    )
}
#[c2rust::src_loc = "76:1"]
unsafe extern "C" fn tls_keypair_set_key_file(
    mut keypair: *mut tls_keypair,
    mut key_file: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    set_string(&raw mut (*keypair).key_file, key_file)
}
#[c2rust::src_loc = "81:1"]
unsafe extern "C" fn tls_keypair_set_key_mem(
    mut keypair: *mut tls_keypair,
    mut key: *const uint8_t,
    mut len: size_t,
) -> ::core::ffi::c_int {
    if !(*keypair).key_mem.is_null() {
        usual_explicit_bzero(
            (*keypair).key_mem as *mut ::core::ffi::c_void,
            (*keypair).key_len,
        );
    }
    set_mem(
        &raw mut (*keypair).key_mem,
        &raw mut (*keypair).key_len,
        key as *const ::core::ffi::c_void,
        len,
    )
}
#[c2rust::src_loc = "89:1"]
unsafe extern "C" fn tls_keypair_clear(mut keypair: *mut tls_keypair) {
    tls_keypair_set_cert_mem(keypair, ::core::ptr::null::<uint8_t>(), 0 as size_t);
    tls_keypair_set_key_mem(keypair, ::core::ptr::null::<uint8_t>(), 0 as size_t);
}
#[c2rust::src_loc = "95:1"]
unsafe extern "C" fn tls_keypair_free(mut keypair: *mut tls_keypair) {
    if keypair.is_null() {
        return;
    }
    tls_keypair_clear(keypair);
    free((*keypair).cert_file as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    free((*keypair).cert_mem as *mut ::core::ffi::c_void);
    free((*keypair).key_file as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    free((*keypair).key_mem as *mut ::core::ffi::c_void);
    free(keypair as *mut ::core::ffi::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "110:1"]
pub unsafe extern "C" fn tls_config_new() -> *mut tls_config {
    let mut config = ::core::ptr::null_mut::<tls_config>();
    config = calloc(1 as size_t, ::core::mem::size_of::<tls_config>() as size_t) as *mut tls_config;
    if config.is_null() {
        return ::core::ptr::null_mut::<tls_config>();
    }
    (*config).keypair = tls_keypair_new();
    if !(*config).keypair.is_null()
        && (tls_config_set_ca_file(config, _PATH_SSL_CA_FILE.as_ptr()) == 0 as ::core::ffi::c_int)
        && (tls_config_set_dheparams(config, b"none\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int)
        && (tls_config_set_ecdhecurve(config, b"auto\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int)
        && (tls_config_set_ciphers(
            config,
            b"secure\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
    {
        tls_config_set_protocols(config, TLS_PROTOCOLS_DEFAULT as uint32_t);
        tls_config_set_verify_depth(config, 6 as ::core::ffi::c_int);
        tls_config_prefer_ciphers_server(config);
        tls_config_verify(config);
        return config;
    }
    tls_config_free(config);
    ::core::ptr::null_mut::<tls_config>()
}
#[no_mangle]
#[c2rust::src_loc = "146:1"]
pub unsafe extern "C" fn tls_config_free(mut config: *mut tls_config) {
    let mut kp = ::core::ptr::null_mut::<tls_keypair>();
    let mut nkp = ::core::ptr::null_mut::<tls_keypair>();
    if config.is_null() {
        return;
    }
    kp = (*config).keypair;
    while !kp.is_null() {
        nkp = (*kp).next;
        tls_keypair_free(kp);
        kp = nkp;
    }
    free((*config).error.msg as *mut ::core::ffi::c_void);
    free((*config).ca_file as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    free((*config).ca_mem as *mut ::core::ffi::c_void);
    free((*config).ca_path as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    free((*config).ciphers as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    free((*config).cipher_suites as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    free(config as *mut ::core::ffi::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "169:1"]
pub unsafe extern "C" fn tls_config_error(
    mut config: *mut tls_config,
) -> *const ::core::ffi::c_char {
    (*config).error.msg
}
#[no_mangle]
#[c2rust::src_loc = "174:1"]
pub unsafe extern "C" fn tls_config_clear_keys(mut config: *mut tls_config) {
    let mut kp = ::core::ptr::null_mut::<tls_keypair>();
    kp = (*config).keypair;
    while !kp.is_null() {
        tls_keypair_clear(kp);
        kp = (*kp).next;
    }
    tls_config_set_ca_mem(config, ::core::ptr::null::<uint8_t>(), 0 as size_t);
}
#[no_mangle]
#[c2rust::src_loc = "184:1"]
pub unsafe extern "C" fn tls_config_parse_protocols(
    mut protocols: *mut uint32_t,
    mut protostr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut proto: uint32_t = 0;
    let mut protos: uint32_t = 0 as uint32_t;
    let mut s = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut negate: ::core::ffi::c_int = 0;
    s = strdup(protostr);
    if s.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    q = s;
    loop {
        p = strsep(
            &raw mut q,
            b",:\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if p.is_null() {
            break;
        }
        while *p as ::core::ffi::c_int == ' ' as i32 || *p as ::core::ffi::c_int == '\t' as i32 {
            p = p.offset(1);
        }
        negate = 0 as ::core::ffi::c_int;
        if *p as ::core::ffi::c_int == '!' as i32 {
            negate = 1 as ::core::ffi::c_int;
            p = p.offset(1);
        }
        if negate != 0 && protos == 0 as uint32_t {
            protos = TLS_PROTOCOLS_ALL as uint32_t;
        }
        proto = 0 as uint32_t;
        if strcasecmp(p, b"all\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            proto = TLS_PROTOCOLS_ALL as uint32_t;
        } else if strcasecmp(p, b"default\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            || strcasecmp(p, b"secure\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            proto = TLS_PROTOCOLS_DEFAULT as uint32_t;
        }
        if strcasecmp(p, b"tlsv1\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            proto = TLS_PROTOCOL_TLSv1 as uint32_t;
        } else if strcasecmp(p, b"tlsv1.0\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            proto = TLS_PROTOCOL_TLSv1_0 as uint32_t;
        } else if strcasecmp(p, b"tlsv1.1\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            proto = TLS_PROTOCOL_TLSv1_1 as uint32_t;
        } else if strcasecmp(p, b"tlsv1.2\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            proto = TLS_PROTOCOL_TLSv1_2 as uint32_t;
        } else if strcasecmp(p, b"tlsv1.3\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            proto = TLS_PROTOCOL_TLSv1_3 as uint32_t;
        }
        if proto == 0 as uint32_t {
            free(s as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
        if negate != 0 {
            protos &= !proto;
        } else {
            protos |= proto;
        }
    }
    *protocols = protos;
    free(s as *mut ::core::ffi::c_void);
    0 as ::core::ffi::c_int
}
#[no_mangle]
#[c2rust::src_loc = "242:1"]
pub unsafe extern "C" fn tls_config_set_ca_file(
    mut config: *mut tls_config,
    mut ca_file: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    set_string(&raw mut (*config).ca_file, ca_file)
}
#[no_mangle]
#[c2rust::src_loc = "247:1"]
pub unsafe extern "C" fn tls_config_set_ca_path(
    mut config: *mut tls_config,
    mut ca_path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    set_string(&raw mut (*config).ca_path, ca_path)
}
#[no_mangle]
#[c2rust::src_loc = "252:1"]
pub unsafe extern "C" fn tls_config_set_ca_mem(
    mut config: *mut tls_config,
    mut ca: *const uint8_t,
    mut len: size_t,
) -> ::core::ffi::c_int {
    set_mem(
        &raw mut (*config).ca_mem,
        &raw mut (*config).ca_len,
        ca as *const ::core::ffi::c_void,
        len,
    )
}
#[no_mangle]
#[c2rust::src_loc = "257:1"]
pub unsafe extern "C" fn tls_config_set_cert_file(
    mut config: *mut tls_config,
    mut cert_file: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    tls_keypair_set_cert_file((*config).keypair, cert_file)
}
#[no_mangle]
#[c2rust::src_loc = "262:1"]
pub unsafe extern "C" fn tls_config_set_cert_mem(
    mut config: *mut tls_config,
    mut cert: *const uint8_t,
    mut len: size_t,
) -> ::core::ffi::c_int {
    tls_keypair_set_cert_mem((*config).keypair, cert, len)
}
#[no_mangle]
#[c2rust::src_loc = "269:1"]
pub unsafe extern "C" fn tls_config_set_ciphers(
    mut config: *mut tls_config,
    mut ciphers: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ssl_ctx = ::core::ptr::null_mut::<SSL_CTX>();
    if ciphers.is_null()
        || strcasecmp(
            ciphers,
            b"default\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcasecmp(
            ciphers,
            b"secure\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcasecmp(
            ciphers,
            b"normal\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcasecmp(
            ciphers,
            b"fast\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        return set_string(&raw mut (*config).ciphers, ciphers);
    }
    ssl_ctx = SSL_CTX_new(TLS_method());
    if ssl_ctx.is_null() {
        tls_config_set_errorx(
            config,
            b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if SSL_CTX_set_cipher_list(ssl_ctx, ciphers) != 1 as ::core::ffi::c_int {
        tls_config_set_errorx(
            config,
            b"no ciphers for '%s'\0" as *const u8 as *const ::core::ffi::c_char,
            ciphers,
        );
    } else {
        SSL_CTX_free(ssl_ctx);
        return set_string(&raw mut (*config).ciphers, ciphers);
    }
    SSL_CTX_free(ssl_ctx);
    -(1 as ::core::ffi::c_int)
}
#[no_mangle]
#[c2rust::src_loc = "310:1"]
pub unsafe extern "C" fn tls_config_set_ciphers_v13(
    mut config: *mut tls_config,
    mut ciphers: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if (*config).protocols & TLS_PROTOCOL_TLSv1_3 as uint32_t != 0
        && (!ciphers.is_null() && strlen(ciphers) > 0 as size_t)
    {
        return set_string(&raw mut (*config).cipher_suites, ciphers);
    }
    0 as ::core::ffi::c_int
}
#[no_mangle]
#[c2rust::src_loc = "324:1"]
pub unsafe extern "C" fn tls_config_set_dheparams(
    mut config: *mut tls_config,
    mut params: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut keylen: ::core::ffi::c_int = 0;
    if params.is_null()
        || strcasecmp(params, b"none\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        keylen = 0 as ::core::ffi::c_int;
    } else if strcasecmp(params, b"auto\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        keylen = -(1 as ::core::ffi::c_int);
    } else {
        tls_config_set_errorx(
            config,
            b"invalid dhe param '%s'\0" as *const u8 as *const ::core::ffi::c_char,
            params,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*config).dheparams = keylen;
    0 as ::core::ffi::c_int
}
#[no_mangle]
#[c2rust::src_loc = "342:1"]
pub unsafe extern "C" fn tls_config_set_ecdhecurve(
    mut config: *mut tls_config,
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut nid: ::core::ffi::c_int = 0;
    if name.is_null()
        || strcasecmp(name, b"none\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        nid = NID_undef;
    } else if strcasecmp(name, b"auto\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        nid = -(1 as ::core::ffi::c_int);
    } else {
        nid = OBJ_txt2nid(name);
        if nid == NID_undef {
            tls_config_set_errorx(
                config,
                b"invalid ecdhe curve '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                name,
            );
            return -(1 as ::core::ffi::c_int);
        }
    }
    (*config).ecdhecurve = nid;
    0 as ::core::ffi::c_int
}
#[no_mangle]
#[c2rust::src_loc = "360:1"]
pub unsafe extern "C" fn tls_config_set_key_file(
    mut config: *mut tls_config,
    mut key_file: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    tls_keypair_set_key_file((*config).keypair, key_file)
}
#[no_mangle]
#[c2rust::src_loc = "365:1"]
pub unsafe extern "C" fn tls_config_set_key_mem(
    mut config: *mut tls_config,
    mut key: *const uint8_t,
    mut len: size_t,
) -> ::core::ffi::c_int {
    tls_keypair_set_key_mem((*config).keypair, key, len)
}
#[no_mangle]
#[c2rust::src_loc = "371:1"]
pub unsafe extern "C" fn tls_config_set_keypair_file(
    mut config: *mut tls_config,
    mut cert_file: *const ::core::ffi::c_char,
    mut key_file: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if tls_config_set_cert_file(config, cert_file) != 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if tls_config_set_key_file(config, key_file) != 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    0 as ::core::ffi::c_int
}
#[no_mangle]
#[c2rust::src_loc = "382:1"]
pub unsafe extern "C" fn tls_config_set_keypair_mem(
    mut config: *mut tls_config,
    mut cert: *const uint8_t,
    mut cert_len: size_t,
    mut key: *const uint8_t,
    mut key_len: size_t,
) -> ::core::ffi::c_int {
    if tls_config_set_cert_mem(config, cert, cert_len) != 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if tls_config_set_key_mem(config, key, key_len) != 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    0 as ::core::ffi::c_int
}
#[no_mangle]
#[c2rust::src_loc = "393:1"]
pub unsafe extern "C" fn tls_config_set_ocsp_stapling_file(
    mut config: *mut tls_config,
    mut blob_file: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !blob_file.is_null() {
        tls_config_set_ocsp_stapling_mem(config, ::core::ptr::null::<uint8_t>(), 0 as size_t);
    }
    set_string(&raw mut (*config).ocsp_file, blob_file)
}
#[no_mangle]
#[c2rust::src_loc = "401:1"]
pub unsafe extern "C" fn tls_config_set_ocsp_stapling_mem(
    mut config: *mut tls_config,
    mut blob: *const uint8_t,
    mut len: size_t,
) -> ::core::ffi::c_int {
    if !blob.is_null() {
        tls_config_set_ocsp_stapling_file(config, ::core::ptr::null::<::core::ffi::c_char>());
    }
    set_mem(
        &raw mut (*config).ocsp_mem,
        &raw mut (*config).ocsp_len,
        blob as *const ::core::ffi::c_void,
        len,
    )
}
#[no_mangle]
#[c2rust::src_loc = "409:1"]
pub unsafe extern "C" fn tls_config_set_protocols(
    mut config: *mut tls_config,
    mut protocols: uint32_t,
) {
    (*config).protocols = protocols;
}
#[no_mangle]
#[c2rust::src_loc = "414:1"]
pub unsafe extern "C" fn tls_config_set_verify_depth(
    mut config: *mut tls_config,
    mut verify_depth: ::core::ffi::c_int,
) {
    (*config).verify_depth = verify_depth;
}
#[no_mangle]
#[c2rust::src_loc = "419:1"]
pub unsafe extern "C" fn tls_config_prefer_ciphers_client(mut config: *mut tls_config) {
    (*config).ciphers_server = 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "424:1"]
pub unsafe extern "C" fn tls_config_prefer_ciphers_server(mut config: *mut tls_config) {
    (*config).ciphers_server = 1 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "429:1"]
pub unsafe extern "C" fn tls_config_insecure_noverifycert(mut config: *mut tls_config) {
    (*config).verify_cert = 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "434:1"]
pub unsafe extern "C" fn tls_config_insecure_noverifyname(mut config: *mut tls_config) {
    (*config).verify_name = 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "439:1"]
pub unsafe extern "C" fn tls_config_insecure_noverifytime(mut config: *mut tls_config) {
    (*config).verify_time = 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "444:1"]
pub unsafe extern "C" fn tls_config_verify(mut config: *mut tls_config) {
    (*config).verify_cert = 1 as ::core::ffi::c_int;
    (*config).verify_name = 1 as ::core::ffi::c_int;
    (*config).verify_time = 1 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "451:1"]
pub unsafe extern "C" fn tls_config_verify_client(mut config: *mut tls_config) {
    (*config).verify_client = 1 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "456:1"]
pub unsafe extern "C" fn tls_config_verify_client_optional(mut config: *mut tls_config) {
    (*config).verify_client = 2 as ::core::ffi::c_int;
}
