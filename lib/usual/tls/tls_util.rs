pub mod _types_h {

    pub type __uint16_t = u16;

    pub type __int32_t = i32;

    pub type __uint32_t = u32;

    pub type __int64_t = i64;

    pub type __uint64_t = u64;

    pub type __darwin_size_t = usize;

    pub type __darwin_ssize_t = isize;

    pub type __darwin_time_t = ::core::ffi::c_long;
}

pub mod sys__types_h {

    pub type __darwin_blkcnt_t = __int64_t;

    pub type __darwin_blksize_t = __int32_t;

    pub type __darwin_dev_t = __int32_t;

    pub type __darwin_gid_t = __uint32_t;

    pub type __darwin_ino64_t = __uint64_t;

    pub type __darwin_mode_t = __uint16_t;

    pub type __darwin_off_t = __int64_t;

    pub type __darwin_uid_t = __uint32_t;

    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    use super::_types_h::{__int32_t, __int64_t, __uint16_t, __uint32_t, __uint64_t};
}

pub mod _dev_t_h {

    pub type dev_t = __darwin_dev_t;
    use super::sys__types_h::__darwin_dev_t;
}

pub mod _blkcnt_t_h {

    pub type blkcnt_t = __darwin_blkcnt_t;
    use super::sys__types_h::__darwin_blkcnt_t;
}

pub mod _blksize_t_h {

    pub type blksize_t = __darwin_blksize_t;
    use super::sys__types_h::__darwin_blksize_t;
}

pub mod _gid_t_h {

    pub type gid_t = __darwin_gid_t;
    use super::sys__types_h::__darwin_gid_t;
}

pub mod _mode_t_h {

    pub type mode_t = __darwin_mode_t;
    use super::sys__types_h::__darwin_mode_t;
}

pub mod _nlink_t_h {

    pub type nlink_t = __uint16_t;
    use super::_types_h::__uint16_t;
}

pub mod _off_t_h {

    pub type off_t = __darwin_off_t;
    use super::sys__types_h::__darwin_off_t;
}

pub mod _uid_t_h {

    pub type uid_t = __darwin_uid_t;
    use super::sys__types_h::__darwin_uid_t;
}

pub mod _size_t_h {

    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}

pub mod _ssize_t_h {

    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
}

pub mod _time_t_h {

    pub type time_t = __darwin_time_t;
    use super::_types_h::__darwin_time_t;
}

pub mod _uint8_t_h {

    pub type uint8_t = u8;
}

pub mod _uint32_t_h {

    pub type uint32_t = u32;
}

pub mod _stdio_h {

    pub type fpos_t = __darwin_off_t;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct __sbuf {
        pub _base: *mut ::core::ffi::c_uchar,
        pub _size: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct __sFILE {
        pub _p: *mut ::core::ffi::c_uchar,
        pub _r: ::core::ffi::c_int,
        pub _w: ::core::ffi::c_int,
        pub _flags: ::core::ffi::c_short,
        pub _file: ::core::ffi::c_short,
        pub _bf: __sbuf,
        pub _lbfsize: ::core::ffi::c_int,
        pub _cookie: *mut ::core::ffi::c_void,
        pub _close: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
        pub _read: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub _seek: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void, fpos_t, ::core::ffi::c_int) -> fpos_t,
        >,
        pub _write: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub _ub: __sbuf,
        pub _extra: *mut __sFILEX,
        pub _ur: ::core::ffi::c_int,
        pub _ubuf: [::core::ffi::c_uchar; 3],
        pub _nbuf: [::core::ffi::c_uchar; 1],
        pub _lb: __sbuf,
        pub _blksize: ::core::ffi::c_int,
        pub _offset: fpos_t,
    }

    pub type FILE = __sFILE;
    use super::_size_t_h::size_t;
    use super::sys__types_h::__darwin_off_t;
    extern "C" {

        pub type __sFILEX;

        pub fn fclose(_: *mut FILE) -> ::core::ffi::c_int;

        pub fn fdopen(_: ::core::ffi::c_int, _: *const ::core::ffi::c_char) -> *mut FILE;

        pub fn snprintf(
            __str: *mut ::core::ffi::c_char,
            __size: size_t,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;

        pub fn asprintf(
            _: *mut *mut ::core::ffi::c_char,
            _: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}

pub mod _timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct timespec {
        pub tv_sec: __darwin_time_t,
        pub tv_nsec: ::core::ffi::c_long,
    }
    use super::_types_h::__darwin_time_t;
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

    pub const TLS_CLIENT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
    use super::_size_t_h::size_t;
    use super::_time_t_h::time_t;
    use super::_uint32_t_h::uint32_t;
    use super::types_h::{SSL, SSL_CTX, X509};
    extern "C" {

        pub type tls_ocsp_query;
    }
}

pub mod types_h {

    pub type X509 = x509_st;

    pub type SSL_CTX = ssl_ctx_st;

    pub type SSL = ssl_st;

    pub type EVP_PKEY = evp_pkey_st;

    pub type BIO = bio_st;

    pub type pem_password_cb = unsafe extern "C" fn(
        *mut ::core::ffi::c_char,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;

    pub type EVP_CIPHER = evp_cipher_st;

    pub type DH = dh_st;
    extern "C" {

        pub type x509_st;

        pub type ssl_ctx_st;

        pub type ssl_st;

        pub type evp_pkey_st;

        pub type bio_st;

        pub type evp_cipher_st;

        pub type dh_st;
    }
}

pub mod bio_h {

    pub type BIO_METHOD = bio_method_st;

    pub const BIO_CTRL_INFO: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    use super::types_h::BIO;
    extern "C" {

        pub type bio_method_st;

        pub fn BIO_new(type_0: *const BIO_METHOD) -> *mut BIO;

        pub fn BIO_ctrl(
            bp: *mut BIO,
            cmd: ::core::ffi::c_int,
            larg: ::core::ffi::c_long,
            parg: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_long;

        pub fn BIO_free_all(a: *mut BIO);

        pub fn BIO_s_mem() -> *const BIO_METHOD;
    }
}

pub mod stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct stat {
        pub st_dev: dev_t,
        pub st_mode: mode_t,
        pub st_nlink: nlink_t,
        pub st_ino: __darwin_ino64_t,
        pub st_uid: uid_t,
        pub st_gid: gid_t,
        pub st_rdev: dev_t,
        pub st_atimespec: timespec,
        pub st_mtimespec: timespec,
        pub st_ctimespec: timespec,
        pub st_birthtimespec: timespec,
        pub st_size: off_t,
        pub st_blocks: blkcnt_t,
        pub st_blksize: blksize_t,
        pub st_flags: __uint32_t,
        pub st_gen: __uint32_t,
        pub st_lspare: __int32_t,
        pub st_qspare: [__int64_t; 2],
    }
    use super::_blkcnt_t_h::blkcnt_t;
    use super::_blksize_t_h::blksize_t;
    use super::_dev_t_h::dev_t;
    use super::_gid_t_h::gid_t;
    use super::_mode_t_h::mode_t;
    use super::_nlink_t_h::nlink_t;
    use super::_off_t_h::off_t;
    use super::_timespec_h::timespec;
    use super::_types_h::{__int32_t, __int64_t, __uint32_t};
    use super::_uid_t_h::uid_t;
    use super::sys__types_h::__darwin_ino64_t;
    extern "C" {

        pub fn fstat(_: ::core::ffi::c_int, _: *mut stat) -> ::core::ffi::c_int;
    }
}

pub mod ssl_h {

    pub type SSL_CIPHER = ssl_cipher_st;

    pub const SSL_CTRL_GET_SHARED_GROUP: ::core::ffi::c_int = 93 as ::core::ffi::c_int;

    pub const SSL_CTRL_GET_PEER_TMP_KEY: ::core::ffi::c_int = 109 as ::core::ffi::c_int;
    use super::types_h::SSL;
    extern "C" {

        pub type ssl_cipher_st;

        pub fn SSL_get_current_cipher(s: *const SSL) -> *const SSL_CIPHER;

        pub fn SSL_CIPHER_get_name(c: *const SSL_CIPHER) -> *const ::core::ffi::c_char;

        pub fn SSL_CIPHER_get_kx_nid(c: *const SSL_CIPHER) -> ::core::ffi::c_int;

        pub fn SSL_get_cipher_list(
            s: *const SSL,
            n: ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char;

        pub fn SSL_ctrl(
            ssl: *mut SSL,
            cmd: ::core::ffi::c_int,
            larg: ::core::ffi::c_long,
            parg: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_long;

        pub fn SSL_get_version(s: *const SSL) -> *const ::core::ffi::c_char;
    }
}

pub mod _malloc_h {
    use super::_size_t_h::size_t;
    extern "C" {

        pub fn calloc(__count: size_t, __size: size_t) -> *mut ::core::ffi::c_void;

        pub fn free(_: *mut ::core::ffi::c_void);
    }
}

pub mod unistd_h {
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    extern "C" {

        pub fn close(_: ::core::ffi::c_int) -> ::core::ffi::c_int;

        pub fn read(_: ::core::ffi::c_int, _: *mut ::core::ffi::c_void, __nbyte: size_t)
            -> ssize_t;
    }
}

pub mod _null_h {

    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}

pub mod evp_h {

    pub const EVP_PKEY_DH: ::core::ffi::c_int = NID_dhKeyAgreement;

    pub const EVP_PKEY_EC: ::core::ffi::c_int = NID_X9_62_id_ecPublicKey;
    use super::obj_mac_h::{NID_X9_62_id_ecPublicKey, NID_dhKeyAgreement};
    use super::types_h::{dh_st, EVP_PKEY};
    extern "C" {

        pub fn EVP_PKEY_get_id(pkey: *const EVP_PKEY) -> ::core::ffi::c_int;

        pub fn EVP_PKEY_get0_DH(pkey: *const EVP_PKEY) -> *const dh_st;

        pub fn EVP_PKEY_free(pkey: *mut EVP_PKEY);
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

        pub fn strchr(
            __s: *const ::core::ffi::c_char,
            __c: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_char;

        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;

        pub fn strdup(__s1: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;

        pub fn strlcpy(
            __dst: *mut ::core::ffi::c_char,
            __source: *const ::core::ffi::c_char,
            __size: size_t,
        ) -> ::core::ffi::c_ulong;
    }
}

pub mod pem_h {
    use super::_stdio_h::FILE;
    use super::types_h::{pem_password_cb, BIO, EVP_CIPHER, EVP_PKEY};
    extern "C" {

        pub fn PEM_read_PrivateKey(
            out: *mut FILE,
            x: *mut *mut EVP_PKEY,
            cb: Option<pem_password_cb>,
            u: *mut ::core::ffi::c_void,
        ) -> *mut EVP_PKEY;

        pub fn PEM_write_bio_PrivateKey(
            out: *mut BIO,
            x: *const EVP_PKEY,
            enc: *const EVP_CIPHER,
            kstr: *const ::core::ffi::c_uchar,
            klen: ::core::ffi::c_int,
            cb: Option<pem_password_cb>,
            u: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
    }
}

pub mod tls_compat_h {
    use super::types_h::EVP_PKEY;
    extern "C" {

        pub fn get_ecdh_curve_nid(pk: *mut EVP_PKEY, nid: *mut ::core::ffi::c_int) -> bool;
    }
}

pub mod dh_h {
    use super::types_h::DH;
    extern "C" {

        pub fn DH_size(dh: *const DH) -> ::core::ffi::c_int;
    }
}

pub mod fcntl_h {

    pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    extern "C" {

        pub fn open(
            _: *const ::core::ffi::c_char,
            _: ::core::ffi::c_int,
            ...
        ) -> ::core::ffi::c_int;
    }
}

pub mod crypto_h {

    pub const OPENSSL_VERSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    extern "C" {

        pub fn OpenSSL_version(type_0: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    }
}

pub mod obj_mac_h {

    pub const NID_X9_62_id_ecPublicKey: ::core::ffi::c_int = 408 as ::core::ffi::c_int;

    pub const NID_dhKeyAgreement: ::core::ffi::c_int = 28 as ::core::ffi::c_int;

    pub const NID_kx_ecdhe: ::core::ffi::c_int = 1038 as ::core::ffi::c_int;

    pub const NID_kx_dhe: ::core::ffi::c_int = 1039 as ::core::ffi::c_int;
}

pub mod objects_h {
    extern "C" {

        pub fn OBJ_nid2sn(n: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    }
}
pub use self::_blkcnt_t_h::blkcnt_t;
pub use self::_blksize_t_h::blksize_t;
pub use self::_dev_t_h::dev_t;
pub use self::_gid_t_h::gid_t;
use self::_malloc_h::{calloc, free};
pub use self::_mode_t_h::mode_t;
pub use self::_nlink_t_h::nlink_t;
pub use self::_null_h::NULL;
pub use self::_off_t_h::off_t;
pub use self::_size_t_h::size_t;
pub use self::_ssize_t_h::ssize_t;
pub use self::_stdio_h::{
    __sFILE, __sFILEX, __sbuf, asprintf, fclose, fdopen, fpos_t, snprintf, FILE,
};
use self::_string_h::{memcpy, memset, strchr, strcmp, strdup, strlcpy};
pub use self::_time_t_h::time_t;
pub use self::_timespec_h::timespec;
pub use self::_types_h::{
    __darwin_size_t, __darwin_ssize_t, __darwin_time_t, __int32_t, __int64_t, __uint16_t,
    __uint32_t, __uint64_t,
};
pub use self::_uid_t_h::uid_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::bio_h::{
    bio_method_st, BIO_ctrl, BIO_free_all, BIO_new, BIO_s_mem, BIO_CTRL_INFO, BIO_METHOD,
};
pub use self::crypto_h::{OpenSSL_version, OPENSSL_VERSION};
use self::dh_h::DH_size;
pub use self::evp_h::{EVP_PKEY_free, EVP_PKEY_get0_DH, EVP_PKEY_get_id, EVP_PKEY_DH, EVP_PKEY_EC};
pub use self::fcntl_h::{open, O_RDONLY};
pub use self::obj_mac_h::{NID_X9_62_id_ecPublicKey, NID_dhKeyAgreement, NID_kx_dhe, NID_kx_ecdhe};
use self::objects_h::OBJ_nid2sn;
use self::pem_h::{PEM_read_PrivateKey, PEM_write_bio_PrivateKey};
pub use self::ssl_h::{
    ssl_cipher_st, SSL_CIPHER_get_kx_nid, SSL_CIPHER_get_name, SSL_ctrl, SSL_get_cipher_list,
    SSL_get_current_cipher, SSL_get_version, SSL_CIPHER, SSL_CTRL_GET_PEER_TMP_KEY,
    SSL_CTRL_GET_SHARED_GROUP,
};
pub use self::stat_h::{fstat, stat};
pub use self::sys__types_h::{
    __darwin_blkcnt_t, __darwin_blksize_t, __darwin_dev_t, __darwin_gid_t, __darwin_ino64_t,
    __darwin_mode_t, __darwin_off_t, __darwin_uid_t, __DARWIN_NULL,
};
use self::tls_compat_h::get_ecdh_curve_nid;
pub use self::tls_internal_h::{
    tls, tls_config, tls_conninfo, tls_error, tls_keypair, tls_ocsp_info, tls_ocsp_query,
    TLS_CLIENT,
};
pub use self::types_h::{
    bio_st, dh_st, evp_cipher_st, evp_pkey_st, pem_password_cb, ssl_ctx_st, ssl_st, x509_st, BIO,
    DH, EVP_CIPHER, EVP_PKEY, SSL, SSL_CTX, X509,
};
use self::unistd_h::{close, read};
#[no_mangle]

pub unsafe extern "C" fn tls_backend_version() -> *const ::core::ffi::c_char {
    OpenSSL_version(OPENSSL_VERSION)
}
#[no_mangle]

pub unsafe extern "C" fn tls_host_port(
    mut hostport: *const ::core::ffi::c_char,
    mut host: *mut *mut ::core::ffi::c_char,
    mut port: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut h = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rv = 1 as ::core::ffi::c_int;
    *host = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *port = ::core::ptr::null_mut::<::core::ffi::c_char>();
    s = strdup(hostport);
    if s.is_null() {
        current_block = 4597018210457273307;
    } else {
        p = s;
        h = p;
        if *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '[' as i32 {
            h = h.offset(1);
            p = strchr(s, ']' as i32);
            if p.is_null() {
                current_block = 4425825671777955447;
            } else {
                let fresh0 = p;
                p = p.offset(1);
                *fresh0 = '\0' as i32 as ::core::ffi::c_char;
                current_block = 10879442775620481940;
            }
        } else {
            current_block = 10879442775620481940;
        }
        match current_block {
            4425825671777955447 => {}
            _ => {
                p = strchr(p, ':' as i32);
                if p.is_null() {
                    current_block = 4425825671777955447;
                } else if !strchr(p.offset(1 as ::core::ffi::c_int as isize), ':' as i32).is_null()
                {
                    current_block = 4425825671777955447;
                } else {
                    let fresh1 = p;
                    p = p.offset(1);
                    *fresh1 = '\0' as i32 as ::core::ffi::c_char;
                    if asprintf(host, b"%s\0" as *const u8 as *const ::core::ffi::c_char, h)
                        == -(1 as ::core::ffi::c_int)
                    {
                        current_block = 4597018210457273307;
                    } else if asprintf(port, b"%s\0" as *const u8 as *const ::core::ffi::c_char, p)
                        == -(1 as ::core::ffi::c_int)
                    {
                        current_block = 4597018210457273307;
                    } else {
                        rv = 0 as ::core::ffi::c_int;
                        current_block = 4425825671777955447;
                    }
                }
            }
        }
    }
    if current_block == 4597018210457273307 {
        free(*host as *mut ::core::ffi::c_void);
        *host = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free(*port as *mut ::core::ffi::c_void);
        *port = ::core::ptr::null_mut::<::core::ffi::c_char>();
        rv = -(1 as ::core::ffi::c_int);
    }
    free(s as *mut ::core::ffi::c_void);
    rv
}

unsafe extern "C" fn tls_password_cb(
    mut buf: *mut ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
    mut _rwflag: ::core::ffi::c_int,
    mut u: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut len: size_t = 0;
    if u.is_null() {
        memset(
            buf as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            size as size_t,
        );
        return 0 as ::core::ffi::c_int;
    }
    len = strlcpy(buf, u as *const ::core::ffi::c_char, size as size_t) as size_t;
    if len >= size as size_t {
        return 0 as ::core::ffi::c_int;
    }
    len as ::core::ffi::c_int
}
#[no_mangle]

pub unsafe extern "C" fn tls_load_file(
    mut name: *const ::core::ffi::c_char,
    mut len: *mut size_t,
    mut password: *mut ::core::ffi::c_char,
) -> *mut uint8_t {
    let mut current_block: u64;
    let mut fp = ::core::ptr::null_mut::<FILE>();
    let mut key = ::core::ptr::null_mut::<EVP_PKEY>();
    let mut bio = ::core::ptr::null_mut::<BIO>();
    let mut buf = ::core::ptr::null_mut::<uint8_t>();
    let mut data = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut st = stat {
        st_dev: 0,
        st_mode: 0,
        st_nlink: 0,
        st_ino: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_atimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_birthtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_size: 0,
        st_blocks: 0,
        st_blksize: 0,
        st_flags: 0,
        st_gen: 0,
        st_lspare: 0,
        st_qspare: [0; 2],
    };
    let mut size: size_t = 0;
    let mut fd = -(1 as ::core::ffi::c_int);
    *len = 0 as size_t;
    fd = open(name, O_RDONLY);
    if fd == -(1 as ::core::ffi::c_int) {
        return ::core::ptr::null_mut::<uint8_t>();
    }
    if password.is_null() {
        if fstat(fd, &raw mut st) != 0 as ::core::ffi::c_int {
            current_block = 1605922262404967633;
        } else {
            size = st.st_size as size_t;
            buf = calloc(1 as size_t, size.wrapping_add(1 as size_t)) as *mut uint8_t;
            if buf.is_null() {
                current_block = 1605922262404967633;
            } else if read(fd, buf as *mut ::core::ffi::c_void, size) != size as ssize_t {
                current_block = 1605922262404967633;
            } else {
                close(fd);
                current_block = 8480421269774563804;
            }
        }
    } else {
        fp = fdopen(fd, b"r\0" as *const u8 as *const ::core::ffi::c_char);
        if fp.is_null() {
            current_block = 1605922262404967633;
        } else {
            fd = -(1 as ::core::ffi::c_int);
            key = PEM_read_PrivateKey(
                fp,
                ::core::ptr::null_mut::<*mut EVP_PKEY>(),
                Some(
                    tls_password_cb
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
                password as *mut ::core::ffi::c_void,
            );
            fclose(fp);
            if key.is_null() {
                current_block = 1605922262404967633;
            } else {
                bio = BIO_new(BIO_s_mem());
                if bio.is_null() {
                    current_block = 1605922262404967633;
                } else if PEM_write_bio_PrivateKey(
                    bio,
                    key,
                    ::core::ptr::null::<EVP_CIPHER>(),
                    ::core::ptr::null::<::core::ffi::c_uchar>(),
                    0 as ::core::ffi::c_int,
                    None,
                    NULL,
                ) == 0
                {
                    current_block = 1605922262404967633;
                } else {
                    size = BIO_ctrl(
                        bio,
                        BIO_CTRL_INFO,
                        0 as ::core::ffi::c_long,
                        &raw mut data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    ) as size_t;
                    if size <= 0 as size_t {
                        current_block = 1605922262404967633;
                    } else {
                        buf = calloc(1 as size_t, size) as *mut uint8_t;
                        if buf.is_null() {
                            current_block = 1605922262404967633;
                        } else {
                            memcpy(
                                buf as *mut ::core::ffi::c_void,
                                data as *const ::core::ffi::c_void,
                                size,
                            );
                            BIO_free_all(bio);
                            EVP_PKEY_free(key);
                            current_block = 8480421269774563804;
                        }
                    }
                }
            }
        }
    }
    match current_block {
        1605922262404967633 => {
            free(buf as *mut ::core::ffi::c_void);
            if fd != -(1 as ::core::ffi::c_int) {
                close(fd);
            }
            if !bio.is_null() {
                BIO_free_all(bio);
            }
            if !key.is_null() {
                EVP_PKEY_free(key);
            }
            ::core::ptr::null_mut::<uint8_t>()
        }
        _ => {
            *len = size;
            buf
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn tls_get_connection_info(
    mut ctx: *mut tls,
    mut buf: *mut ::core::ffi::c_char,
    mut buflen: size_t,
) -> ssize_t {
    let mut conn = (*ctx).ssl_conn;
    let mut ocsp_pfx = b"\0" as *const u8 as *const ::core::ffi::c_char;
    let mut ocsp_info = b"\0" as *const u8 as *const ::core::ffi::c_char;
    let mut proto = b"-\0" as *const u8 as *const ::core::ffi::c_char;
    let mut cipher = b"-\0" as *const u8 as *const ::core::ffi::c_char;
    let mut dh: [::core::ffi::c_char; 64] = [0; 64];
    let mut used_dh_bits = (*ctx).used_dh_bits;
    let mut used_ecdh_nid = (*ctx).used_ecdh_nid;
    let mut ciph_obj = ::core::ptr::null::<SSL_CIPHER>();
    dh[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    if !conn.is_null() {
        proto = SSL_get_version(conn);
        if strcmp(
            proto,
            b"TLSv1.3\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            cipher = SSL_get_cipher_list(conn, 0 as ::core::ffi::c_int);
        } else {
            cipher = SSL_CIPHER_get_name(SSL_get_current_cipher(conn));
        }
        ciph_obj = SSL_get_current_cipher(conn);
        if (*ctx).flags & TLS_CLIENT as uint32_t != 0 {
            let mut pk = ::core::ptr::null_mut::<EVP_PKEY>();
            let mut ok = SSL_ctrl(
                conn,
                SSL_CTRL_GET_PEER_TMP_KEY,
                0 as ::core::ffi::c_long,
                &raw mut pk as *mut ::core::ffi::c_void,
            ) as ::core::ffi::c_int;
            if ok != 0 {
                let mut pk_type = EVP_PKEY_get_id(pk);
                if pk_type == EVP_PKEY_DH {
                    let mut dh_0 = EVP_PKEY_get0_DH(pk) as *const DH;
                    used_dh_bits = DH_size(dh_0) * 8 as ::core::ffi::c_int;
                } else if pk_type == EVP_PKEY_EC {
                    let mut nid: ::core::ffi::c_int = 0;
                    if get_ecdh_curve_nid(pk, &raw mut nid) {
                        used_ecdh_nid = nid;
                    }
                }
                EVP_PKEY_free(pk);
            }
        } else if !ciph_obj.is_null() && used_ecdh_nid == 0 && used_dh_bits == 0 {
            let mut kx = SSL_CIPHER_get_kx_nid(ciph_obj);
            if kx == NID_kx_ecdhe {
                used_ecdh_nid = SSL_ctrl(
                    conn,
                    SSL_CTRL_GET_SHARED_GROUP,
                    0 as ::core::ffi::c_long,
                    NULL,
                ) as ::core::ffi::c_int;
            } else if kx == NID_kx_dhe {
                snprintf(
                    &raw mut dh as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
                    b"/DH=?\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
    }
    if used_dh_bits != 0 {
        snprintf(
            &raw mut dh as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
            b"/DH=%d\0" as *const u8 as *const ::core::ffi::c_char,
            used_dh_bits,
        );
    } else if used_ecdh_nid != 0 {
        snprintf(
            &raw mut dh as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
            b"/ECDH=%s\0" as *const u8 as *const ::core::ffi::c_char,
            OBJ_nid2sn(used_ecdh_nid),
        );
    }
    if !(*ctx).ocsp_result.is_null() {
        ocsp_info = (*ctx).ocsp_result;
        ocsp_pfx = b"/OCSP=\0" as *const u8 as *const ::core::ffi::c_char;
    }
    snprintf(
        buf,
        buflen,
        b"%s/%s%s%s%s\0" as *const u8 as *const ::core::ffi::c_char,
        proto,
        cipher,
        &raw mut dh as *mut ::core::ffi::c_char,
        ocsp_pfx,
        ocsp_info,
    ) as ssize_t
}
