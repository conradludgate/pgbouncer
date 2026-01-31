pub mod runetype_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _RuneEntry {
        pub __min: __darwin_rune_t,
        pub __max: __darwin_rune_t,
        pub __map: __darwin_rune_t,
        pub __types: *mut __uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _RuneRange {
        pub __nranges: ::core::ffi::c_int,
        pub __ranges: *mut _RuneEntry,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _RuneCharClass {
        pub __name: [::core::ffi::c_char; 14],
        pub __mask: __uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _RuneLocale {
        pub __magic: [::core::ffi::c_char; 8],
        pub __encoding: [::core::ffi::c_char; 32],
        pub __sgetrune: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                __darwin_size_t,
                *mut *const ::core::ffi::c_char,
            ) -> __darwin_rune_t,
        >,
        pub __sputrune: Option<
            unsafe extern "C" fn(
                __darwin_rune_t,
                *mut ::core::ffi::c_char,
                __darwin_size_t,
                *mut *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub __invalid_rune: __darwin_rune_t,
        pub __runetype: [__uint32_t; 256],
        pub __maplower: [__darwin_rune_t; 256],
        pub __mapupper: [__darwin_rune_t; 256],
        pub __runetype_ext: _RuneRange,
        pub __maplower_ext: _RuneRange,
        pub __mapupper_ext: _RuneRange,
        pub __variable: *mut ::core::ffi::c_void,
        pub __variable_len: ::core::ffi::c_int,
        pub __ncharclasses: ::core::ffi::c_int,
        pub __charclasses: *mut _RuneCharClass,
    }
    use crate::types::{__darwin_rune_t, __darwin_size_t, __uint32_t};
    extern "C" {

        pub static mut _DefaultRuneLocale: _RuneLocale;
    }
}

pub mod cfparser_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct CfValue {
        pub value_p: *mut ::core::ffi::c_void,
        pub extra: *const ::core::ffi::c_void,
        pub key_name: *const ::core::ffi::c_char,
        pub buf: *mut ::core::ffi::c_char,
        pub buflen: ::core::ffi::c_int,
    }
    extern "C" {

        pub fn cf_set_str(cv: *mut CfValue, value: *const ::core::ffi::c_char) -> bool;
    }
}

pub mod bouncer_h {

    pub use crate::types::*;

    pub use c2rust_bitfields::BitfieldStruct;

    extern "C" {




    pub static mut cf_listen_port: ::core::ffi::c_int;




    pub static mut cf_tcp_keepalive: ::core::ffi::c_int;




    pub static mut cf_tcp_keepcnt: ::core::ffi::c_int;




    pub static mut cf_tcp_keepidle: ::core::ffi::c_int;




    pub static mut cf_tcp_keepintvl: ::core::ffi::c_int;




    pub static mut cf_tcp_socket_buffer: ::core::ffi::c_int;




    pub static mut cf_tcp_user_timeout: ::core::ffi::c_int;



    }

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

// OpenSSL types - use openssl-sys crate
pub mod types_h {
    pub type EVP_MD_CTX = openssl_sys::EVP_MD_CTX;
    pub type EVP_MD = openssl_sys::EVP_MD;
    pub type evp_md_ctx_st = openssl_sys::EVP_MD_CTX;
    pub type evp_md_st = openssl_sys::EVP_MD;
}

pub mod _OSByteOrder_h {
    #[inline]

    pub unsafe extern "C" fn _OSSwapInt16(mut _data: __uint16_t) -> __uint16_t {
        ((_data as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | _data as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as __uint16_t
    }
    #[inline]

    pub unsafe extern "C" fn _OSSwapInt32(mut _data: __uint32_t) -> __uint32_t {
        _data = _data.swap_bytes() as __uint32_t;
        _data
    }
    use crate::types::{__uint16_t, __uint32_t};
}

pub mod unistd_h {

    pub use crate::types::*;
    extern "C" {

        pub fn geteuid() -> uid_t;

        pub fn getpid() -> pid_t;

        pub fn gethostname(_: *mut ::core::ffi::c_char, __namelen: size_t) -> ::core::ffi::c_int;
    }


}

pub mod _ctype_h {

    pub const _CTYPE_S: ::core::ffi::c_long = 0x4000 as ::core::ffi::c_long;
    #[inline]

    pub unsafe extern "C" fn isascii(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        (_c & !(0x7f as ::core::ffi::c_int) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
    }
    #[inline]

    pub unsafe extern "C" fn __istype(
        mut _c: __darwin_ct_rune_t,
        mut _f: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int {
        if isascii(_c as ::core::ffi::c_int) != 0 {
            (_DefaultRuneLocale.__runetype[_c as usize] as ::core::ffi::c_ulong & _f != 0)
                as ::core::ffi::c_int
        } else {
            (__maskrune(_c, _f) != 0) as ::core::ffi::c_int
        }
    }
    #[inline]

    pub unsafe extern "C" fn isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        __istype(_c as __darwin_ct_rune_t, _CTYPE_S as ::core::ffi::c_ulong)
    }
    use super::runetype_h::_DefaultRuneLocale;
    use crate::types::__darwin_ct_rune_t;
    extern "C" {

        pub fn __maskrune(_: __darwin_ct_rune_t, _: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    }
}

pub mod ctype_h {
    #[inline]

    pub unsafe extern "C" fn safe_isspace(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        isspace(c as ::core::ffi::c_uchar as ::core::ffi::c_int)
    }
    use super::_ctype_h::isspace;
}

pub mod inet_h {
    use crate::types::socklen_t;
    extern "C" {

        pub fn inet_ntop(
            _: ::core::ffi::c_int,
            _: *const ::core::ffi::c_void,
            _: *mut ::core::ffi::c_char,
            __size: socklen_t,
        ) -> *const ::core::ffi::c_char;

        pub fn inet_pton(
            _: ::core::ffi::c_int,
            _: *const ::core::ffi::c_char,
            _: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
    }
}

// OpenSSL EVP functions - use openssl-sys crate
pub mod evp_h {
    pub use openssl_sys::EVP_DigestFinal_ex;
    pub use openssl_sys::EVP_DigestInit;
    pub use openssl_sys::EVP_DigestUpdate;
    pub use openssl_sys::EVP_MD_CTX_free;
    pub use openssl_sys::EVP_MD_CTX_new;
    pub use openssl_sys::EVP_md5;
}

pub mod csrandom_h {
    use crate::types::size_t;
    extern "C" {

        pub fn csrandom_bytes(buf: *mut ::core::ffi::c_void, nbytes: size_t);
    }
}

pub mod tcp_h {

    pub const TCP_NODELAY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
}

pub mod md5_h {

    pub const MD5_DIGEST_LENGTH: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
}
// OpenSSL error functions - use openssl-sys crate
pub mod err_h {
    pub use openssl_sys::ERR_clear_error;
    pub use openssl_sys::ERR_get_error;
    pub use openssl_sys::ERR_reason_error_string;
}
pub use self::_OSByteOrder_h::{_OSSwapInt16, _OSSwapInt32};
pub use self::_ctype_h::{__istype, __maskrune, isascii, isspace, _CTYPE_S};
pub use crate::types::gid_t;
pub use crate::types::socklen_t;
use crate::types::snprintf;
use crate::types::exit;
use crate::types::{memcmp, memcpy, memset, strchr, strcmp, strerror, strlcpy, strlen, strstr};
pub use crate::types::u_int32_t;
pub use self::bouncer_h::{
    cf_listen_port, cf_tcp_keepalive, cf_tcp_keepcnt, cf_tcp_keepidle, cf_tcp_keepintvl,
    cf_tcp_socket_buffer, cf_tcp_user_timeout, pga_family, pga_is_unix, sockaddr_ucreds,
    C2RustUnnamed_9, CallbackState, LoadBalanceHosts, PacketCallbackFlag, PgAddr, PgCredentials,
    PgDatabase, PgGlobalUser, PgPool, PgSocket, PgStats, ReplicationType, ScramState, SocketState,
    CB_HANDLE_COMPLETE_PACKET, CB_NONE, CB_WANT_COMPLETE_PACKET, CL_ACTIVE, CL_ACTIVE_CANCEL,
    CL_FREE, CL_JUSTFREE, CL_LOGIN, CL_WAITING, CL_WAITING_CANCEL, CL_WAITING_LOGIN,
    LOAD_BALANCE_HOSTS_DISABLE, LOAD_BALANCE_HOSTS_ROUND_ROBIN, REPLICATION_LOGICAL,
    REPLICATION_NONE, REPLICATION_PHYSICAL, SV_ACTIVE, SV_ACTIVE_CANCEL, SV_BEING_CANCELED,
    SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN, SV_TESTED, SV_USED,
};
pub use self::cfparser_h::{cf_set_str, CfValue};
use self::csrandom_h::csrandom_bytes;
pub use self::ctype_h::safe_isspace;
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
    __darwin_ct_rune_t, __darwin_ptrdiff_t, __darwin_rune_t, __darwin_size_t, __darwin_socklen_t,
    __darwin_ssize_t, __darwin_time_t, __darwin_wchar_t, __int32_t, __uint16_t, __uint32_t,
    __uint8_t,
};
pub use crate::types::{aatree_cmp_f, aatree_walker_f, AANode, AATree};
pub use crate::types::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};

use self::err_h::{ERR_clear_error, ERR_get_error, ERR_reason_error_string};
pub use crate::types::{__error, EINVAL, ENOSYS};
use crate::types::event_add;
pub use crate::types::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
use self::evp_h::{
    EVP_DigestFinal_ex, EVP_DigestInit, EVP_DigestUpdate, EVP_MD_CTX_free, EVP_MD_CTX_new, EVP_md5,
};
pub use crate::types::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use crate::types::{in_addr, sockaddr_in, IPPROTO_TCP};
use self::inet_h::{inet_ntop, inet_pton};
pub use self::iobuf_h::{iobuf, IOBuf};
pub use crate::types::{
    cf_verbose, log_fatal, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE,
    LG_STATS, LG_WARNING,
};
pub use self::md5_h::MD5_DIGEST_LENGTH;
pub use self::pktbuf_h::PktBuf;
pub use self::runetype_h::{
    _DefaultRuneLocale, _RuneCharClass, _RuneEntry, _RuneLocale, _RuneRange,
};
pub use self::sbuf_h::{
    sbuf_cb_t, SBuf, SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK, SBUF_EV_FLUSH,
    SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY,
};
pub use crate::types::{
    getpeername, getsockname, setsockopt, sockaddr, AF_INET, AF_INET6, AF_UNIX, SOL_SOCKET,
    SO_RCVBUF, SO_SNDBUF,
};
pub use crate::types::{
    __darwin_gid_t, __darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t, __DARWIN_NULL,
};
pub use self::tcp_h::TCP_NODELAY;
pub use crate::types::usec_t;
pub use crate::types::List;
pub use crate::types::MBuf;
pub use crate::types::PktHdr;
pub use crate::types::StatList;
pub use crate::types::{false_0, true_0};
pub use crate::types::{PStr, StrPool};
pub use crate::types::{PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement};

pub use self::types_h::{evp_md_ctx_st, evp_md_st, EVP_MD, EVP_MD_CTX};
use self::unistd_h::{geteuid, gethostname, getpid};
use crate::types::{socket_set_keepalive, socket_setup, usual_getpeercreds};
pub use crate::types::VarCache;
pub use crate::types::{UT_hash_bucket, UT_hash_handle, UT_hash_table};
#[derive(Copy, Clone)]
#[repr(C)]

pub struct timer_slot {
    pub ev: *mut event,
    pub tv: timeval,
}
#[no_mangle]

pub unsafe extern "C" fn log_socket_prefix(
    mut _lev: LogLevel,
    mut ctx: *mut ::core::ffi::c_void,
    mut dst: *mut ::core::ffi::c_char,
    mut dstlen: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut sock = ctx as *const PgSocket;
    let mut user = ::core::ptr::null::<::core::ffi::c_char>();
    let mut db = ::core::ptr::null::<::core::ffi::c_char>();
    let mut host = ::core::ptr::null::<::core::ffi::c_char>();
    let mut host6: [::core::ffi::c_char; 56] = [0; 56];
    let mut peer_id: ::core::ffi::c_int = 0;
    let mut port: ::core::ffi::c_int = 0;
    let mut stype: ::core::ffi::c_char = 0;
    if sock.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    stype = (if (*sock).state() as ::core::ffi::c_int >= SV_FREE as ::core::ffi::c_int {
        'S' as i32
    } else {
        'C' as i32
    }) as ::core::ffi::c_char;
    port = pga_port(&raw const (*sock).remote_addr);
    peer_id = if !(*sock).pool.is_null() {
        (*(*(*sock).pool).db).peer_id
    } else {
        0 as ::core::ffi::c_int
    };
    db = if !(*sock).pool.is_null() {
        &raw mut (*(*(*sock).pool).db).name as *mut ::core::ffi::c_char
            as *const ::core::ffi::c_char
    } else {
        c"(nodb)".as_ptr()
    };
    user = if !(*sock).login_user_credentials.is_null() {
        &raw mut (*(*sock).login_user_credentials).name as *mut ::core::ffi::c_char
            as *const ::core::ffi::c_char
    } else {
        c"(nouser)".as_ptr()
    };
    if pga_is_unix(&raw const (*sock).remote_addr) {
        let mut pid = (*sock).remote_addr.scred.pid as ::core::ffi::c_ulong;
        if pid != 0 {
            snprintf(
                &raw mut host6 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 56]>() as size_t,
                c"unix(%lu)".as_ptr(),
                pid,
            );
            host = &raw mut host6 as *mut ::core::ffi::c_char;
        } else {
            host = c"unix".as_ptr();
        }
    } else {
        host = pga_ntop(
            &raw const (*sock).remote_addr,
            &raw mut host6 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 56]>() as ::core::ffi::c_int,
        );
    }
    if pga_family(&raw const (*sock).remote_addr) == AF_INET6 as ::core::ffi::c_uint {
        if peer_id != 0 {
            return snprintf(
                dst,
                dstlen as size_t,
                c"%c-%p: peer-%d@[%s]:%d ".as_ptr(),
                stype as ::core::ffi::c_int,
                sock,
                peer_id,
                host,
                port,
            );
        }
        snprintf(
            dst,
            dstlen as size_t,
            c"%c-%p: %s/%s@[%s]:%d ".as_ptr(),
            stype as ::core::ffi::c_int,
            sock,
            db,
            user,
            host,
            port,
        )
    } else {
        if peer_id != 0 {
            return snprintf(
                dst,
                dstlen as size_t,
                c"%c-%p: peer-%d@%s:%d ".as_ptr(),
                stype as ::core::ffi::c_int,
                sock,
                peer_id,
                host,
                port,
            );
        }
        snprintf(
            dst,
            dstlen as size_t,
            c"%c-%p: %s/%s@%s:%d ".as_ptr(),
            stype as ::core::ffi::c_int,
            sock,
            db,
            user,
            host,
            port,
        )
    }
}
#[no_mangle]

pub unsafe extern "C" fn bin2hex(
    mut src: *const uint8_t,
    mut srclen: ::core::ffi::c_uint,
    mut dst: *mut ::core::ffi::c_char,
    mut dstlen: ::core::ffi::c_uint,
) -> *const ::core::ffi::c_char {
    let mut i: ::core::ffi::c_uint = 0;
    let mut j: ::core::ffi::c_uint = 0;
    static mut hextbl: [::core::ffi::c_char; 17] = unsafe {
        ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"0123456789abcdef\0")
    };
    if dstlen == 0 {
        return c"".as_ptr();
    }
    if srclen
        .wrapping_mul(2 as ::core::ffi::c_uint)
        .wrapping_add(1 as ::core::ffi::c_uint)
        > dstlen
    {
        srclen = dstlen
            .wrapping_sub(1 as ::core::ffi::c_uint)
            .wrapping_div(2 as ::core::ffi::c_uint);
    }
    j = 0 as ::core::ffi::c_uint;
    i = j;
    while i < srclen {
        let fresh0 = j;
        j = j.wrapping_add(1);
        *dst.offset(fresh0 as isize) = hextbl
            [(*src.offset(i as isize) as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as usize];
        let fresh1 = j;
        j = j.wrapping_add(1);
        *dst.offset(fresh1 as isize) = hextbl
            [(*src.offset(i as isize) as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize];
        i = i.wrapping_add(1);
    }
    *dst.offset(j as isize) = 0 as ::core::ffi::c_char;
    dst
}

unsafe extern "C" fn hash2hex(mut hash: *const uint8_t, mut dst: *mut ::core::ffi::c_char) {
    bin2hex(
        hash,
        MD5_DIGEST_LENGTH as ::core::ffi::c_uint,
        dst,
        (16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    );
}
#[no_mangle]

pub unsafe extern "C" fn pg_md5_encrypt(
    mut part1: *const ::core::ffi::c_char,
    mut part2: *const ::core::ffi::c_char,
    mut part2len: size_t,
    mut dest: *mut ::core::ffi::c_char,
) -> bool {
    let mut mdctx = ::core::ptr::null_mut::<EVP_MD_CTX>();
    let mut hash: [uint8_t; 16] = [0; 16];
    ERR_clear_error();
    mdctx = EVP_MD_CTX_new();
    if mdctx.is_null() {
        let mut _log_ctx = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx,
            c"MD5 authentication failed: out-of-memory".as_ptr(),
        );
        return false;
    }
    if (EVP_DigestInit(mdctx, EVP_md5()) != 0)
        && (EVP_DigestUpdate(mdctx, part1 as *const ::core::ffi::c_void, strlen(part1)) != 0)
        && (EVP_DigestUpdate(mdctx, part2 as *const ::core::ffi::c_void, part2len) != 0)
        && (EVP_DigestFinal_ex(
            mdctx,
            &raw mut hash as *mut ::core::ffi::c_uchar,
            ::core::ptr::null_mut::<::core::ffi::c_uint>(),
        ) != 0)
    {
        EVP_MD_CTX_free(mdctx);
        memcpy(
            dest as *mut ::core::ffi::c_void,
            c"md5".as_ptr() as *const ::core::ffi::c_void,
            3 as size_t,
        );
        hash2hex(
            &raw mut hash as *mut uint8_t,
            dest.offset(3 as ::core::ffi::c_int as isize),
        );
        return true;
    }
    let mut _log_ctx_0 = NULL;
    log_generic(
        LG_ERROR,
        _log_ctx_0,
        c"MD5 authentication failed: %s".as_ptr(),
        ERR_reason_error_string(ERR_get_error()),
    );
    EVP_MD_CTX_free(mdctx);
    false
}
#[no_mangle]

pub unsafe extern "C" fn get_random_bytes(mut dest: *mut uint8_t, mut len: ::core::ffi::c_int) {
    csrandom_bytes(dest as *mut ::core::ffi::c_void, len as size_t);
}
#[no_mangle]

pub unsafe extern "C" fn tune_socket(mut sock: ::core::ffi::c_int, mut is_unix: bool) -> bool {
    let mut current_block: u64;
    let mut res: ::core::ffi::c_int = 0;
    let mut val: ::core::ffi::c_int = 0;
    let mut errpos = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ok: bool = false;
    errpos = c"socket_setup".as_ptr();
    ok = socket_setup(sock, true);
    if ok {
        if is_unix {
            return true;
        }
        errpos = c"socket_set_keepalive".as_ptr();
        ok = socket_set_keepalive(
            sock,
            cf_tcp_keepalive,
            cf_tcp_keepidle,
            cf_tcp_keepintvl,
            cf_tcp_keepcnt,
        );
        if ok {
            if cf_tcp_user_timeout != 0 {
                errpos = c"setsockopt/TCP_USER_TIMEOUT".as_ptr();
                *__error() = EINVAL;
            } else {
                if cf_tcp_socket_buffer != 0 {
                    val = cf_tcp_socket_buffer;
                    errpos = c"setsockopt/SO_SNDBUF".as_ptr();
                    res = setsockopt(
                        sock,
                        SOL_SOCKET,
                        SO_SNDBUF,
                        &raw mut val as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
                    );
                    if res < 0 as ::core::ffi::c_int {
                        current_block = 10495585853427032907;
                    } else {
                        val = cf_tcp_socket_buffer;
                        errpos = c"setsockopt/SO_RCVBUF".as_ptr();
                        res = setsockopt(
                            sock,
                            SOL_SOCKET,
                            SO_RCVBUF,
                            &raw mut val as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
                        );
                        if res < 0 as ::core::ffi::c_int {
                            current_block = 10495585853427032907;
                        } else {
                            current_block = 5143058163439228106;
                        }
                    }
                } else {
                    current_block = 5143058163439228106;
                }
                match current_block {
                    10495585853427032907 => {}
                    _ => {
                        val = 1 as ::core::ffi::c_int;
                        errpos = c"setsockopt/TCP_NODELAY".as_ptr();
                        res = setsockopt(
                            sock,
                            IPPROTO_TCP,
                            TCP_NODELAY,
                            &raw mut val as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
                        );
                        if res >= 0 as ::core::ffi::c_int {
                            return true;
                        }
                    }
                }
            }
        }
    }
    let mut _log_ctx = NULL;
    log_generic(
        LG_WARNING,
        _log_ctx,
        c"%s(%d) failed: %s".as_ptr(),
        errpos,
        sock,
        strerror(*__error()),
    );
    false
}
#[no_mangle]

pub unsafe extern "C" fn strlist_contains(
    mut liststr: *const ::core::ffi::c_char,
    mut str: *const ::core::ffi::c_char,
) -> bool {
    let mut c: ::core::ffi::c_int = 0;
    let mut len = strlen(str) as ::core::ffi::c_int;
    let mut p = ::core::ptr::null::<::core::ffi::c_char>();
    let mut listpos = liststr;
    loop {
        p = strstr(listpos, str);
        if p.is_null() {
            return false;
        }
        listpos = p.offset(len as isize);
        if *listpos != 0 {
            listpos = listpos.offset(1);
        }
        if p > liststr {
            c = *p.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int;
            if safe_isspace(c) == 0 && c != ',' as i32 {
                continue;
            }
        }
        c = *p.offset(len as isize) as ::core::ffi::c_int;
        if !(c != 0 as ::core::ffi::c_int && safe_isspace(c) == 0 && c != ',' as i32) {
            break;
        }
    }
    true
}
#[no_mangle]

pub unsafe extern "C" fn fill_remote_addr(
    mut sk: *mut PgSocket,
    mut fd: ::core::ffi::c_int,
    mut is_unix: bool,
) {
    let mut dst: *mut PgAddr = &raw mut (*sk).remote_addr;
    let mut len: socklen_t = ::core::mem::size_of::<PgAddr>() as socklen_t;
    let mut err: ::core::ffi::c_int = 0;
    if is_unix {
        let mut uid: uid_t = 0 as uid_t;
        let mut gid: gid_t = 0 as gid_t;
        let mut pid: pid_t = 0 as pid_t;
        pga_set(dst, AF_UNIX, cf_listen_port);
        if usual_getpeercreds(fd, &raw mut uid, &raw mut gid, &raw mut pid)
            >= 0 as ::core::ffi::c_int
        {
            let mut _log_ctx = NULL;
            if cf_verbose > 1 as ::core::ffi::c_int
            {
                log_generic(
                    LG_NOISE,
                    _log_ctx,
                    c"unix peer uid: %d".as_ptr(),
                    uid as ::core::ffi::c_int,
                );
            }
        } else if *__error() != ENOSYS {
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_WARNING,
                _log_ctx_0,
                c"unix peer uid failed: %s".as_ptr(),
                strerror(*__error()),
            );
        }
        (*dst).scred.uid = uid;
        (*dst).scred.pid = pid;
    } else {
        err = getpeername(fd, dst as *mut sockaddr, &raw mut len);
        if err < 0 as ::core::ffi::c_int {
            let mut _log_ctx_1 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_1,
                c"fill_remote_addr: getpeername(%d) = %s".as_ptr(),
                fd,
                strerror(*__error()),
            );
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn fill_local_addr(
    mut sk: *mut PgSocket,
    mut fd: ::core::ffi::c_int,
    mut is_unix: bool,
) {
    let mut dst: *mut PgAddr = &raw mut (*sk).local_addr;
    let mut len: socklen_t = ::core::mem::size_of::<PgAddr>() as socklen_t;
    let mut err: ::core::ffi::c_int = 0;
    if is_unix {
        pga_set(dst, AF_UNIX, cf_listen_port);
        (*dst).scred.uid = geteuid();
        (*dst).scred.pid = getpid();
    } else {
        err = getsockname(fd, dst as *mut sockaddr, &raw mut len);
        if err < 0 as ::core::ffi::c_int {
            let mut _log_ctx = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx,
                c"fill_local_addr: getsockname(%d) = %s".as_ptr(),
                fd,
                strerror(*__error()),
            );
        }
    };
}

pub const TIMER_BACKUP_SLOTS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;

static mut timer_backup_list: [timer_slot; 10] = [timer_slot {
    ev: ::core::ptr::null::<event>() as *mut event,
    tv: timeval {
        tv_sec: 0,
        tv_usec: 0,
    },
}; 10];

static mut timer_backup_used: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]

pub unsafe extern "C" fn safe_evtimer_add(mut ev: *mut event, mut tv: *mut timeval) {
    let mut res: ::core::ffi::c_int = 0;
    let mut ts = ::core::ptr::null_mut::<timer_slot>();
    res = event_add(ev, tv);
    if res >= 0 as ::core::ffi::c_int {
        return;
    }
    if timer_backup_used >= TIMER_BACKUP_SLOTS {
        let mut _log_ctx = NULL;
        log_fatal(
            c"src/util.c".as_ptr(),
            351 as ::core::ffi::c_int,
            c"safe_evtimer_add".as_ptr(),
            false,
            _log_ctx,
            c"TIMER_BACKUP_SLOTS full".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    let fresh2 = timer_backup_used;
    timer_backup_used += 1;
    ts = (&raw mut timer_backup_list as *mut timer_slot).offset(fresh2 as isize) as *mut timer_slot;
    (*ts).ev = ev;
    (*ts).tv = *tv;
}
#[no_mangle]

pub unsafe extern "C" fn rescue_timers() {
    let mut ts = ::core::ptr::null_mut::<timer_slot>();
    while timer_backup_used != 0 {
        ts = (&raw mut timer_backup_list as *mut timer_slot)
            .offset((timer_backup_used - 1 as ::core::ffi::c_int) as isize)
            as *mut timer_slot;
        if event_add((*ts).ev, &raw mut (*ts).tv) < 0 as ::core::ffi::c_int {
            break;
        }
        timer_backup_used -= 1;
    }
}
#[no_mangle]

pub unsafe extern "C" fn pga_port(mut a: *const PgAddr) -> ::core::ffi::c_int {
    if (*a).sa.sa_family as ::core::ffi::c_int == AF_INET6 {
        (if 0 != 0 {
            (((*a).sin6.sin6_port as ::core::ffi::c_uint & 0xff00 as ::core::ffi::c_uint)
                >> 8 as ::core::ffi::c_int
                | ((*a).sin6.sin6_port as ::core::ffi::c_uint & 0xff as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int) as __uint16_t as ::core::ffi::c_int
        } else {
            _OSSwapInt16((*a).sin6.sin6_port as __uint16_t) as ::core::ffi::c_int
        }) as __uint16_t as ::core::ffi::c_int
    } else {
        (if 0 != 0 {
            (((*a).sin.sin_port as ::core::ffi::c_uint & 0xff00 as ::core::ffi::c_uint)
                >> 8 as ::core::ffi::c_int
                | ((*a).sin.sin_port as ::core::ffi::c_uint & 0xff as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int) as __uint16_t as ::core::ffi::c_int
        } else {
            _OSSwapInt16((*a).sin.sin_port as __uint16_t) as ::core::ffi::c_int
        }) as __uint16_t as ::core::ffi::c_int
    }
}
#[no_mangle]

pub unsafe extern "C" fn pga_set(
    mut a: *mut PgAddr,
    mut af: ::core::ffi::c_int,
    mut port: ::core::ffi::c_int,
) {
    memset(
        a as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PgAddr>() as size_t,
    );
    if af == AF_INET6 {
        (*a).sin6.sin6_family = af as sa_family_t;
        (*a).sin6.sin6_port = (if 0 != 0 {
            ((port as __uint16_t as ::core::ffi::c_uint & 0xff00 as ::core::ffi::c_uint)
                >> 8 as ::core::ffi::c_int
                | (port as __uint16_t as ::core::ffi::c_uint & 0xff as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int) as __uint16_t as ::core::ffi::c_int
        } else {
            _OSSwapInt16(port as __uint16_t) as ::core::ffi::c_int
        }) as __uint16_t as in_port_t;
    } else {
        (*a).sin.sin_family = af as sa_family_t;
        (*a).sin.sin_port = (if 0 != 0 {
            ((port as __uint16_t as ::core::ffi::c_uint & 0xff00 as ::core::ffi::c_uint)
                >> 8 as ::core::ffi::c_int
                | (port as __uint16_t as ::core::ffi::c_uint & 0xff as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int) as __uint16_t as ::core::ffi::c_int
        } else {
            _OSSwapInt16(port as __uint16_t) as ::core::ffi::c_int
        }) as __uint16_t as in_port_t;
    };
}
#[no_mangle]

pub unsafe extern "C" fn pga_copy(mut a: *mut PgAddr, mut sa: *const sockaddr) {
    match (*sa).sa_family as ::core::ffi::c_int {
        AF_INET => {
            memcpy(
                &raw mut (*a).sin as *mut ::core::ffi::c_void,
                sa as *const ::core::ffi::c_void,
                ::core::mem::size_of::<sockaddr_in>() as size_t,
            );
        }
        AF_INET6 => {
            memcpy(
                &raw mut (*a).sin6 as *mut ::core::ffi::c_void,
                sa as *const ::core::ffi::c_void,
                ::core::mem::size_of::<sockaddr_in6>() as size_t,
            );
        }
        AF_UNIX => {
            let mut _log_ctx = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx,
                c"pga_copy: AF_UNIX copy not supported".as_ptr(),
            );
        }
        _ => {}
    };
}
#[no_mangle]

pub unsafe extern "C" fn pga_cmp_addr(
    mut a: *const PgAddr,
    mut b: *const PgAddr,
) -> ::core::ffi::c_int {
    if pga_family(a) != pga_family(b) {
        return pga_family(a).wrapping_sub(pga_family(b)) as ::core::ffi::c_int;
    }
    match pga_family(a) {
        2 => memcmp(
            &raw const (*a).sin.sin_addr as *const ::core::ffi::c_void,
            &raw const (*b).sin.sin_addr as *const ::core::ffi::c_void,
            ::core::mem::size_of::<in_addr>() as size_t,
        ),
        30 => memcmp(
            &raw const (*a).sin6.sin6_addr as *const ::core::ffi::c_void,
            &raw const (*b).sin6.sin6_addr as *const ::core::ffi::c_void,
            ::core::mem::size_of::<in6_addr>() as size_t,
        ),
        _ => {
            let mut _log_ctx = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx,
                c"pga_cmp_addr: unsupported family".as_ptr(),
            );
            0 as ::core::ffi::c_int
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn pga_ntop(
    mut a: *const PgAddr,
    mut dst: *mut ::core::ffi::c_char,
    mut dstlen: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut res = ::core::ptr::null::<::core::ffi::c_char>();
    let mut buf: [::core::ffi::c_char; 56] = [0; 56];
    memset(
        &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_char; 56]>() as size_t,
    );
    match pga_family(a) {
        1 => {
            res = c"unix".as_ptr();
        }
        2 => {
            res = inet_ntop(
                AF_INET,
                &raw const (*a).sin.sin_addr as *const ::core::ffi::c_void,
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 56]>() as socklen_t,
            );
        }
        30 => {
            res = inet_ntop(
                AF_INET6,
                &raw const (*a).sin6.sin6_addr as *const ::core::ffi::c_void,
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 56]>() as socklen_t,
            );
        }
        _ => {
            res = c"(bad-af)".as_ptr();
        }
    }
    if res.is_null() {
        res = c"(err-ntop)".as_ptr();
    }
    strlcpy(dst, res, dstlen as size_t);
    dst
}
#[no_mangle]

pub unsafe extern "C" fn pga_pton(
    mut a: *mut PgAddr,
    mut s: *const ::core::ffi::c_char,
    mut port: ::core::ffi::c_int,
) -> bool {
    let mut res = 1 as ::core::ffi::c_int;
    if strcmp(s, c"unix".as_ptr()) == 0 as ::core::ffi::c_int {
        pga_set(a, AF_UNIX, port);
    } else if strcmp(s, c"*".as_ptr()) == 0 as ::core::ffi::c_int {
        pga_set(a, AF_INET, port);
        (*a).sin.sin_addr.s_addr = (if 0 != 0 {
            (0 as ::core::ffi::c_int as u_int32_t as __uint32_t & 0xff000000 as __uint32_t)
                >> 24 as ::core::ffi::c_int
                | (0 as ::core::ffi::c_int as u_int32_t as __uint32_t & 0xff0000 as __uint32_t)
                    >> 8 as ::core::ffi::c_int
                | (0 as ::core::ffi::c_int as u_int32_t as __uint32_t & 0xff00 as __uint32_t)
                    << 8 as ::core::ffi::c_int
                | (0 as ::core::ffi::c_int as u_int32_t as __uint32_t & 0xff as __uint32_t)
                    << 24 as ::core::ffi::c_int
        } else {
            _OSSwapInt32(0 as ::core::ffi::c_int as __uint32_t)
        }) as in_addr_t;
    } else if !strchr(s, ':' as i32).is_null() {
        pga_set(a, AF_INET6, port);
        res = inet_pton(
            AF_INET6,
            s,
            &raw mut (*a).sin6.sin6_addr as *mut ::core::ffi::c_void,
        );
    } else {
        pga_set(a, AF_INET, port);
        res = inet_pton(
            AF_INET,
            s,
            &raw mut (*a).sin.sin_addr as *mut ::core::ffi::c_void,
        );
    }
    if res == 0 as ::core::ffi::c_int {
        *__error() = EINVAL;
    }
    res > 0 as ::core::ffi::c_int
}
#[no_mangle]

pub unsafe extern "C" fn pga_str(
    mut a: *const PgAddr,
    mut dst: *mut ::core::ffi::c_char,
    mut dstlen: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut buf: [::core::ffi::c_char; 56] = [0; 56];
    pga_ntop(
        a,
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 56]>() as ::core::ffi::c_int,
    );
    if pga_family(a) == AF_INET6 as ::core::ffi::c_uint {
        snprintf(
            dst,
            dstlen as size_t,
            c"[%s]:%d".as_ptr(),
            &raw mut buf as *mut ::core::ffi::c_char,
            pga_port(a),
        );
    } else if pga_family(a) == AF_UNIX as ::core::ffi::c_uint && (*a).scred.pid != 0 {
        snprintf(
            dst,
            dstlen as size_t,
            c"%s:%d$%lu".as_ptr(),
            &raw mut buf as *mut ::core::ffi::c_char,
            pga_port(a),
            (*a).scred.pid as ::core::ffi::c_ulong,
        );
    } else {
        snprintf(
            dst,
            dstlen as size_t,
            c"%s:%d".as_ptr(),
            &raw mut buf as *mut ::core::ffi::c_char,
            pga_port(a),
        );
    }
    dst
}

unsafe extern "C" fn cached_hostname() -> *const ::core::ffi::c_char {
    static mut cache: [::core::ffi::c_char; 256] = [0; 256];
    let mut err: ::core::ffi::c_int = 0;
    if cache[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        err = gethostname(
            &raw mut cache as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
        );
        if err != 0 as ::core::ffi::c_int {
            strlcpy(
                &raw mut cache as *mut ::core::ffi::c_char,
                c"somehost".as_ptr(),
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
            );
        }
    }
    &raw mut cache as *mut ::core::ffi::c_char
}
#[no_mangle]

pub unsafe extern "C" fn pga_details(
    mut a: *const PgAddr,
    mut dst: *mut ::core::ffi::c_char,
    mut dstlen: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut buf: [::core::ffi::c_char; 56] = [0; 56];
    pga_ntop(
        a,
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 56]>() as ::core::ffi::c_int,
    );
    if pga_family(a) == AF_INET6 as ::core::ffi::c_uint {
        snprintf(
            dst,
            dstlen as size_t,
            c"[%s]:%d".as_ptr(),
            &raw mut buf as *mut ::core::ffi::c_char,
            pga_port(a),
        );
    } else if pga_family(a) == AF_UNIX as ::core::ffi::c_uint && (*a).scred.pid != 0 {
        snprintf(
            dst,
            dstlen as size_t,
            c"%s(%lu@%s):%d".as_ptr(),
            &raw mut buf as *mut ::core::ffi::c_char,
            (*a).scred.pid as ::core::ffi::c_ulong,
            cached_hostname(),
            pga_port(a),
        );
    } else {
        snprintf(
            dst,
            dstlen as size_t,
            c"%s:%d".as_ptr(),
            &raw mut buf as *mut ::core::ffi::c_char,
            pga_port(a),
        );
    }
    dst
}
#[no_mangle]

pub unsafe extern "C" fn cf_set_authdb(
    mut cv: *mut CfValue,
    mut value: *const ::core::ffi::c_char,
) -> bool {
    if !check_reserved_database(value) {
        let mut _log_ctx = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx,
            b"cannot use the reserved \"%s\" database as an auth_dbname\0" as *const u8
                as *const ::core::ffi::c_char,
            value,
        );
        return false;
    }
    cf_set_str(cv, value)
}
#[no_mangle]

pub unsafe extern "C" fn check_reserved_database(mut value: *const ::core::ffi::c_char) -> bool {
    if !value.is_null() && strcmp(value, c"pgbouncer".as_ptr()) == 0 as ::core::ffi::c_int {
        return false;
    }
    true
}
