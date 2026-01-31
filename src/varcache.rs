pub mod bouncer_h {
    pub use crate::types::*;
    pub use c2rust_bitfields::BitfieldStruct;
    extern "C" {}
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
    extern "C" {

        pub fn pktbuf_temp() -> *mut PktBuf;

        pub fn pktbuf_send_immediate(buf: *mut PktBuf, sk: *mut PgSocket) -> bool;

        pub fn pktbuf_start_packet(buf: *mut PktBuf, type_0: ::core::ffi::c_int);

        pub fn pktbuf_put_char(buf: *mut PktBuf, val: ::core::ffi::c_char);

        pub fn pktbuf_put_string(buf: *mut PktBuf, str: *const ::core::ffi::c_char);

        pub fn pktbuf_put_bytes(
            buf: *mut PktBuf,
            data: *const ::core::ffi::c_void,
            len: ::core::ffi::c_int,
        );

        pub fn pktbuf_finish_packet(buf: *mut PktBuf);

        pub fn pktbuf_write_generic(
            buf: *mut PktBuf,
            type_0: ::core::ffi::c_int,
            fmt: *const ::core::ffi::c_char,
            ...
        );
    }






}

pub mod dnslookup_h {

    pub use crate::types::*;

    pub use crate::types::*;

    pub use crate::types::*;
    extern "C" {
    }






}

pub mod cxalloc_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct CxOps {
        pub c_alloc: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
        >,
        pub c_realloc: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                size_t,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub c_free:
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>,
        pub c_destroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct CxMem {
        pub ops: *const CxOps,
        pub ctx: *mut ::core::ffi::c_void,
    }
    use crate::types::size_t;
    extern "C" {

        pub static cx_libc_allocator: CxMem;
    }
}

pub mod string_h {

    pub type str_cb =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char) -> bool>;
    use super::cxalloc_h::CxMem;
    extern "C" {

        pub type StrList;

        pub fn strlist_new(ca: *const CxMem) -> *mut StrList;

        pub fn strlist_free(slist: *mut StrList);

        pub fn strlist_empty(slist: *mut StrList) -> bool;

        pub fn strlist_append(slist: *mut StrList, str: *const ::core::ffi::c_char) -> bool;

        pub fn strlist_pop(slist: *mut StrList) -> *mut ::core::ffi::c_char;

        pub fn parse_word_list(
            s: *const ::core::ffi::c_char,
            cb_func: str_cb,
            cb_arg: *mut ::core::ffi::c_void,
        ) -> bool;
    }
}

pub mod _ctype_h {
    #[inline]

    pub unsafe extern "C" fn tolower(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        __tolower(_c as __darwin_ct_rune_t) as ::core::ffi::c_int
    }
    use crate::types::__darwin_ct_rune_t;
    extern "C" {

        pub fn __tolower(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    }
}

pub mod ctype_h {
    #[inline]

    pub unsafe extern "C" fn safe_tolower(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        tolower(c as ::core::ffi::c_uchar as ::core::ffi::c_int)
    }
    use super::_ctype_h::tolower;
}

pub mod _strings_h {
    extern "C" {

        pub fn strcasecmp(
            _: *const ::core::ffi::c_char,
            _: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}

pub mod protocol_h {

    pub const PqMsg_Query: ::core::ffi::c_int = 'Q' as i32;

    pub const PqMsg_ParameterStatus: ::core::ffi::c_int = 'S' as i32;
}

pub mod pgutil_h {
    extern "C" {

        pub fn pg_quote_literal(
            _dst: *mut ::core::ffi::c_char,
            _src: *const ::core::ffi::c_char,
            dstlen: ::core::ffi::c_int,
        ) -> bool;
    }
}
pub use self::_ctype_h::{__tolower, tolower};
use crate::types::{free, malloc};
use crate::types::snprintf;
use crate::types::exit;
use crate::types::{memset, strcmp, strdup, strlen};
use self::_strings_h::strcasecmp;
pub use self::bouncer_h::{
    sockaddr_ucreds, C2RustUnnamed_9, CallbackState, LoadBalanceHosts, PacketCallbackFlag, PgAddr,
    PgCredentials, PgDatabase, PgGlobalUser, PgPool, PgSocket, PgStats, ReplicationType,
    ScramState, SocketState, CB_HANDLE_COMPLETE_PACKET, CB_NONE, CB_WANT_COMPLETE_PACKET,
    CL_ACTIVE, CL_ACTIVE_CANCEL, CL_FREE, CL_JUSTFREE, CL_LOGIN, CL_WAITING, CL_WAITING_CANCEL,
    CL_WAITING_LOGIN, LOAD_BALANCE_HOSTS_DISABLE, LOAD_BALANCE_HOSTS_ROUND_ROBIN,
    REPLICATION_LOGICAL, REPLICATION_NONE, REPLICATION_PHYSICAL, SV_ACTIVE, SV_ACTIVE_CANCEL,
    SV_BEING_CANCELED, SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN, SV_TESTED, SV_USED,
};
pub use self::ctype_h::safe_tolower;
pub use self::cxalloc_h::{cx_libc_allocator, CxMem, CxOps};
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
    __darwin_ct_rune_t, __darwin_ptrdiff_t, __darwin_size_t, __darwin_ssize_t, __darwin_time_t,
    __int32_t, __uint16_t, __uint32_t, __uint8_t,
};
pub use crate::types::{aatree_cmp_f, aatree_walker_f, AANode, AATree};
pub use crate::types::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};

pub use crate::types::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
pub use crate::types::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use crate::types::{in_addr, sockaddr_in};
pub use self::iobuf_h::{iobuf, IOBuf};
pub use crate::types::{
    cf_verbose, log_fatal, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE,
    LG_STATS, LG_WARNING,
};
use self::pgutil_h::pg_quote_literal;
pub use self::pktbuf_h::{
    pktbuf_finish_packet, pktbuf_put_bytes, pktbuf_put_char, pktbuf_put_string,
    pktbuf_send_immediate, pktbuf_start_packet, pktbuf_temp, pktbuf_write_generic, PktBuf,
};
pub use self::protocol_h::{PqMsg_ParameterStatus, PqMsg_Query};
pub use self::sbuf_h::{
    sbuf_cb_t, SBuf, SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK, SBUF_EV_FLUSH,
    SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY,
};
pub use crate::types::sockaddr;
pub use self::string_h::{
    parse_word_list, str_cb, strlist_append, strlist_empty, strlist_free, strlist_new, strlist_pop,
    StrList,
};
pub use crate::types::{__darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t, __DARWIN_NULL};
pub use crate::types::usec_t;
pub use crate::types::List;
pub use crate::types::MBuf;
pub use crate::types::PktHdr;
pub use crate::types::StatList;
pub use crate::types::{false_0, true_0};
pub use crate::types::{PStr, StrPool};
pub use crate::types::{PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement};

pub use crate::types::VarCache;
pub use crate::types::{
    UT_hash_bucket, UT_hash_handle, UT_hash_table, HASH_BKT_CAPACITY_THRESH,
    HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE,
};
#[derive(Copy, Clone)]
#[repr(C)]

pub struct var_lookup {
    pub name: *const ::core::ffi::c_char,
    pub idx: ::core::ffi::c_int,
    pub hh: UT_hash_handle,
}

static mut num_var_cached: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

static mut lookup_map: *mut var_lookup = ::core::ptr::null::<var_lookup>() as *mut var_lookup;

static mut vpool: *mut StrPool = ::core::ptr::null::<StrPool>() as *mut StrPool;
#[inline]

unsafe extern "C" fn get_value(mut cache: *mut VarCache, mut lk: *const var_lookup) -> *mut PStr {
    *(*cache).var_list.offset((*lk).idx as isize)
}

unsafe extern "C" fn sl_add(
    mut arg: *mut ::core::ffi::c_void,
    mut s: *const ::core::ffi::c_char,
) -> bool {
    strlist_append(arg as *mut StrList, s)
}
#[no_mangle]

pub unsafe extern "C" fn get_num_var_cached() -> ::core::ffi::c_int {
    num_var_cached
}

unsafe extern "C" fn init_var_lookup_from_config(
    mut cf_track_extra_parameters: *const ::core::ffi::c_char,
    mut num_vars: *mut ::core::ffi::c_int,
) {
    let mut var_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut lookup = ::core::ptr::null_mut::<var_lookup>();
    let mut sl = strlist_new(::core::ptr::null::<CxMem>());
    if !parse_word_list(
        cf_track_extra_parameters,
        Some(
            sl_add
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                ) -> bool,
        ),
        sl as *mut ::core::ffi::c_void,
    ) {
        let mut _log_ctx = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx,
            c"failed to parse track_extra_parameters in config %s".as_ptr(),
            cf_track_extra_parameters,
        );
        exit(1 as ::core::ffi::c_int);
    }
    while !strlist_empty(sl) {
        var_name = strlist_pop(sl);
        if var_name.is_null() {
            continue;
        }
        let mut _uthash_hfstr_keylen = strlen(var_name) as ::core::ffi::c_uint;
        lookup = ::core::ptr::null_mut::<var_lookup>();
        if !lookup_map.is_null() {
            let mut _hf_hashv: ::core::ffi::c_uint = 0;
            let mut _hj_i: ::core::ffi::c_uint = 0;
            let mut _hj_j: ::core::ffi::c_uint = 0;
            let mut _hj_k: ::core::ffi::c_uint = 0;
            let mut _hj_key = var_name as *const ::core::ffi::c_uchar;
            _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
            _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
            _hj_i = _hj_j;
            _hj_k = _uthash_hfstr_keylen;
            while _hj_k >= 12 as ::core::ffi::c_uint {
                _hj_i = _hj_i.wrapping_add(
                    (safe_tolower(
                        *_hj_key as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (safe_tolower(*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (safe_tolower(*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (safe_tolower(*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_j = _hj_j.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (safe_tolower(*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (safe_tolower(*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (safe_tolower(*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hf_hashv = _hf_hashv.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (safe_tolower(*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (safe_tolower(*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (safe_tolower(*_hj_key.offset(11 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_i = _hj_i.wrapping_sub(_hj_j);
                _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
                _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                _hj_j = _hj_j.wrapping_sub(_hj_i);
                _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
                _hj_i = _hj_i.wrapping_sub(_hj_j);
                _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
                _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                _hj_j = _hj_j.wrapping_sub(_hj_i);
                _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
                _hj_i = _hj_i.wrapping_sub(_hj_j);
                _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
                _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                _hj_j = _hj_j.wrapping_sub(_hj_i);
                _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
                _hj_key = _hj_key.offset(12 as ::core::ffi::c_int as isize);
                _hj_k = _hj_k.wrapping_sub(12 as ::core::ffi::c_uint);
            }
            _hf_hashv = _hf_hashv.wrapping_add(_uthash_hfstr_keylen);
            let mut current_block_58: u64;
            match _hj_k {
                11 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (safe_tolower(*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_58 = 15275504483753031639;
                }
                10 => {
                    current_block_58 = 15275504483753031639;
                }
                9 => {
                    current_block_58 = 14514390918547092561;
                }
                8 => {
                    current_block_58 = 1205942404857220978;
                }
                7 => {
                    current_block_58 = 10244489138959580834;
                }
                6 => {
                    current_block_58 = 7231708513041901785;
                }
                5 => {
                    current_block_58 = 9781868647333494730;
                }
                4 => {
                    current_block_58 = 12285225998683619597;
                }
                3 => {
                    current_block_58 = 4389422820558755489;
                }
                2 => {
                    current_block_58 = 6122795728994222029;
                }
                1 => {
                    current_block_58 = 16419358174521838934;
                }
                _ => {
                    current_block_58 = 7420279277351916581;
                }
            }
            if current_block_58 == 15275504483753031639 {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_58 = 14514390918547092561;
            }
            if current_block_58 == 14514390918547092561 {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_58 = 1205942404857220978;
            }
            if current_block_58 == 1205942404857220978 {
                _hj_j = _hj_j.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_58 = 10244489138959580834;
            }
            if current_block_58 == 10244489138959580834 {
                _hj_j = _hj_j.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_58 = 7231708513041901785;
            }
            if current_block_58 == 7231708513041901785 {
                _hj_j = _hj_j.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_58 = 9781868647333494730;
            }
            if current_block_58 == 9781868647333494730 {
                _hj_j = _hj_j.wrapping_add(safe_tolower(
                    *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                ) as ::core::ffi::c_uint);
                current_block_58 = 12285225998683619597;
            }
            if current_block_58 == 12285225998683619597 {
                _hj_i = _hj_i.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_58 = 4389422820558755489;
            }
            if current_block_58 == 4389422820558755489 {
                _hj_i = _hj_i.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_58 = 6122795728994222029;
            }
            if current_block_58 == 6122795728994222029 {
                _hj_i = _hj_i.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_58 = 16419358174521838934;
            }
            if current_block_58 == 16419358174521838934 {
                _hj_i = _hj_i.wrapping_add(safe_tolower(
                    *_hj_key as ::core::ffi::c_int,
                ) as ::core::ffi::c_uint);
            }
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
            lookup = ::core::ptr::null_mut::<var_lookup>();
            if !lookup_map.is_null() {
                let mut _hf_bkt: ::core::ffi::c_uint = 0;
                _hf_bkt = _hf_hashv
                    & (*(*lookup_map).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                if !(*(*(*lookup_map).hh.tbl).buckets.offset(_hf_bkt as isize))
                    .hh_head
                    .is_null()
                {
                    lookup = ((*(*(*lookup_map).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                        as *mut ::core::ffi::c_char)
                        .offset(-(*(*lookup_map).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut var_lookup
                        as *mut var_lookup;
                } else {
                    lookup = ::core::ptr::null_mut::<var_lookup>();
                }
                while !lookup.is_null() {
                    if (*lookup).hh.hashv == _hf_hashv
                        && (*lookup).hh.keylen == _uthash_hfstr_keylen
                        && strcasecmp((*lookup).hh.key as *const ::core::ffi::c_char, var_name)
                            == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                    if !(*lookup).hh.hh_next.is_null() {
                        lookup = ((*lookup).hh.hh_next as *mut ::core::ffi::c_char)
                            .offset(-(*(*lookup_map).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut var_lookup as *mut var_lookup;
                    } else {
                        lookup = ::core::ptr::null_mut::<var_lookup>();
                    }
                }
            }
        }
        if !lookup.is_null() {
            continue;
        }
        lookup = malloc(::core::mem::size_of::<var_lookup>() as size_t) as *mut var_lookup;
        (*lookup).name = strdup(var_name);
        let fresh3 = *num_vars;
        *num_vars += 1;
        (*lookup).idx = fresh3;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i_0: ::core::ffi::c_uint = 0;
        let mut _hj_j_0: ::core::ffi::c_uint = 0;
        let mut _hj_k_0: ::core::ffi::c_uint = 0;
        let mut _hj_key_0 = (*lookup).name as *const ::core::ffi::c_uchar;
        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_0 = _hj_j_0;
        _hj_k_0 = strlen((*lookup).name) as ::core::ffi::c_uint;
        while _hj_k_0 >= 12 as ::core::ffi::c_uint {
            _hj_i_0 = _hj_i_0.wrapping_add(
                (safe_tolower(
                    *_hj_key_0 as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (safe_tolower(*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_j_0 = _hj_j_0.wrapping_add(
                (safe_tolower(
                    *_hj_key_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (safe_tolower(*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _ha_hashv = _ha_hashv.wrapping_add(
                (safe_tolower(
                    *_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (safe_tolower(*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(*_hj_key_0.offset(11 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
            _hj_key_0 = _hj_key_0.offset(12 as ::core::ffi::c_int as isize);
            _hj_k_0 = _hj_k_0.wrapping_sub(12 as ::core::ffi::c_uint);
        }
        _ha_hashv = _ha_hashv.wrapping_add(strlen((*lookup).name) as ::core::ffi::c_uint);
        let mut current_block_180: u64;
        match _hj_k_0 {
            11 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (safe_tolower(
                        *_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_180 = 8153286781957891950;
            }
            10 => {
                current_block_180 = 8153286781957891950;
            }
            9 => {
                current_block_180 = 9978657853023849837;
            }
            8 => {
                current_block_180 = 14748375350202154019;
            }
            7 => {
                current_block_180 = 986562491153128823;
            }
            6 => {
                current_block_180 = 2419899655327389259;
            }
            5 => {
                current_block_180 = 18057295174972137893;
            }
            4 => {
                current_block_180 = 428438469909915749;
            }
            3 => {
                current_block_180 = 1152747300973462357;
            }
            2 => {
                current_block_180 = 9357788886370794354;
            }
            1 => {
                current_block_180 = 17622820276845907441;
            }
            _ => {
                current_block_180 = 4534765400774009001;
            }
        }
        if current_block_180 == 8153286781957891950 {
            _ha_hashv = _ha_hashv.wrapping_add(
                (safe_tolower(
                    *_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_180 = 9978657853023849837;
        }
        if current_block_180 == 9978657853023849837 {
            _ha_hashv = _ha_hashv.wrapping_add(
                (safe_tolower(
                    *_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_180 = 14748375350202154019;
        }
        if current_block_180 == 14748375350202154019 {
            _hj_j_0 = _hj_j_0.wrapping_add(
                (safe_tolower(
                    *_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_180 = 986562491153128823;
        }
        if current_block_180 == 986562491153128823 {
            _hj_j_0 = _hj_j_0.wrapping_add(
                (safe_tolower(
                    *_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_180 = 2419899655327389259;
        }
        if current_block_180 == 2419899655327389259 {
            _hj_j_0 = _hj_j_0.wrapping_add(
                (safe_tolower(
                    *_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_180 = 18057295174972137893;
        }
        if current_block_180 == 18057295174972137893 {
            _hj_j_0 = _hj_j_0.wrapping_add(safe_tolower(
                *_hj_key_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint);
            current_block_180 = 428438469909915749;
        }
        if current_block_180 == 428438469909915749 {
            _hj_i_0 = _hj_i_0.wrapping_add(
                (safe_tolower(
                    *_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_180 = 1152747300973462357;
        }
        if current_block_180 == 1152747300973462357 {
            _hj_i_0 = _hj_i_0.wrapping_add(
                (safe_tolower(
                    *_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_180 = 9357788886370794354;
        }
        if current_block_180 == 9357788886370794354 {
            _hj_i_0 = _hj_i_0.wrapping_add(
                (safe_tolower(
                    *_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_180 = 17622820276845907441;
        }
        if current_block_180 == 17622820276845907441 {
            _hj_i_0 = _hj_i_0.wrapping_add(safe_tolower(
                *_hj_key_0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint);
        }
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
        _hj_i_0 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
        _ha_hashv ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
        _hj_i_0 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
        _ha_hashv ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
        _hj_i_0 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
        _ha_hashv ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
        let mut _ha_oomed = 0 as ::core::ffi::c_int;
        (*lookup).hh.hashv = _ha_hashv;
        (*lookup).hh.key = (*lookup).name as *const ::core::ffi::c_void;
        (*lookup).hh.keylen = strlen((*lookup).name) as ::core::ffi::c_uint;
        if lookup_map.is_null() {
            (*lookup).hh.next = NULL;
            (*lookup).hh.prev = NULL;
            (*lookup).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as size_t)
                as *mut UT_hash_table as *mut UT_hash_table;
            if (*lookup).hh.tbl.is_null() {
                _ha_oomed = 1 as ::core::ffi::c_int;
            } else {
                memset(
                    (*lookup).hh.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UT_hash_table>() as size_t,
                );
                (*(*lookup).hh.tbl).tail = &raw mut (*lookup).hh as *mut UT_hash_handle;
                (*(*lookup).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*lookup).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*lookup).hh.tbl).hho = (&raw mut (*lookup).hh as *mut ::core::ffi::c_char)
                    .offset_from(lookup as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as ptrdiff_t;
                (*(*lookup).hh.tbl).buckets = malloc(
                    (32 as size_t).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                ) as *mut UT_hash_bucket;
                (*(*lookup).hh.tbl).signature = HASH_SIGNATURE as uint32_t;
                if (*(*lookup).hh.tbl).buckets.is_null() {
                    _ha_oomed = 1 as ::core::ffi::c_int;
                    free((*lookup).hh.tbl as *mut ::core::ffi::c_void);
                } else {
                    memset(
                        (*(*lookup).hh.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as size_t)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                    );
                    if _ha_oomed != 0 {
                        free((*(*lookup).hh.tbl).buckets as *mut ::core::ffi::c_void);
                        free((*lookup).hh.tbl as *mut ::core::ffi::c_void);
                    }
                }
            }
            if _ha_oomed == 0 {
                lookup_map = lookup;
            }
        } else {
            (*lookup).hh.tbl = (*lookup_map).hh.tbl;
            (*lookup).hh.next = NULL;
            (*lookup).hh.prev = ((*(*lookup_map).hh.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*lookup_map).hh.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*lookup_map).hh.tbl).tail).next = lookup as *mut ::core::ffi::c_void;
            (*(*lookup_map).hh.tbl).tail = &raw mut (*lookup).hh as *mut UT_hash_handle;
        }
        if _ha_oomed == 0 {
            let mut _ha_bkt: ::core::ffi::c_uint = 0;
            (*(*lookup_map).hh.tbl).num_items = (*(*lookup_map).hh.tbl).num_items.wrapping_add(1);
            _ha_bkt = _ha_hashv
                & (*(*lookup_map).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _ha_head: *mut UT_hash_bucket =
                (*(*lookup_map).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UT_hash_bucket;
            (*_ha_head).count += 1;
            (*lookup).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
            (*lookup).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
            if !(*_ha_head).hh_head.is_null() {
                (*(*_ha_head).hh_head).hh_prev = &raw mut (*lookup).hh as *mut UT_hash_handle;
            }
            (*_ha_head).hh_head = &raw mut (*lookup).hh as *mut UT_hash_handle;
            if (*_ha_head).count
                >= (*_ha_head)
                    .expand_mult
                    .wrapping_add(1 as ::core::ffi::c_uint)
                    .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                && (*(*lookup).hh.tbl).noexpand == 0
            {
                let mut _he_bkt: ::core::ffi::c_uint = 0;
                let mut _he_bkt_i: ::core::ffi::c_uint = 0;
                let mut _he_thh = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _he_hh_nxt = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _he_new_buckets = ::core::ptr::null_mut::<UT_hash_bucket>();
                let mut _he_newbkt = ::core::ptr::null_mut::<UT_hash_bucket>();
                _he_new_buckets = malloc(
                    (::core::mem::size_of::<UT_hash_bucket>() as size_t)
                        .wrapping_mul((*(*lookup).hh.tbl).num_buckets as size_t)
                        .wrapping_mul(2 as size_t),
                ) as *mut UT_hash_bucket;
                if _he_new_buckets.is_null() {
                    _ha_oomed = 1 as ::core::ffi::c_int;
                } else {
                    memset(
                        _he_new_buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (::core::mem::size_of::<UT_hash_bucket>() as size_t)
                            .wrapping_mul((*(*lookup).hh.tbl).num_buckets as size_t)
                            .wrapping_mul(2 as size_t),
                    );
                    (*(*lookup).hh.tbl).ideal_chain_maxlen = ((*(*lookup).hh.tbl).num_items
                        >> (*(*lookup).hh.tbl)
                            .log2_num_buckets
                            .wrapping_add(1 as ::core::ffi::c_uint))
                    .wrapping_add(
                        if (*(*lookup).hh.tbl).num_items
                            & (*(*lookup).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint)
                            != 0 as ::core::ffi::c_uint
                        {
                            1 as ::core::ffi::c_uint
                        } else {
                            0 as ::core::ffi::c_uint
                        },
                    );
                    (*(*lookup).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                    _he_bkt_i = 0 as ::core::ffi::c_uint;
                    while _he_bkt_i < (*(*lookup).hh.tbl).num_buckets {
                        _he_thh = (*(*(*lookup).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                            as *mut UT_hash_handle;
                        while !_he_thh.is_null() {
                            _he_hh_nxt = (*_he_thh).hh_next;
                            _he_bkt = (*_he_thh).hashv
                                & (*(*lookup).hh.tbl)
                                    .num_buckets
                                    .wrapping_mul(2 as ::core::ffi::c_uint)
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            _he_newbkt =
                                _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                            (*_he_newbkt).count += 1;
                            if (*_he_newbkt).count > (*(*lookup).hh.tbl).ideal_chain_maxlen {
                                (*(*lookup).hh.tbl).nonideal_items =
                                    (*(*lookup).hh.tbl).nonideal_items.wrapping_add(1);
                                if (*_he_newbkt).count
                                    > (*_he_newbkt)
                                        .expand_mult
                                        .wrapping_mul((*(*lookup).hh.tbl).ideal_chain_maxlen)
                                {
                                    (*_he_newbkt).expand_mult += 1;
                                }
                            }
                            (*_he_thh).hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                            (*_he_thh).hh_next = (*_he_newbkt).hh_head as *mut UT_hash_handle;
                            if !(*_he_newbkt).hh_head.is_null() {
                                (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                            }
                            (*_he_newbkt).hh_head = _he_thh as *mut UT_hash_handle;
                            _he_thh = _he_hh_nxt;
                        }
                        _he_bkt_i = _he_bkt_i.wrapping_add(1);
                    }
                    free((*(*lookup).hh.tbl).buckets as *mut ::core::ffi::c_void);
                    (*(*lookup).hh.tbl).num_buckets = (*(*lookup).hh.tbl)
                        .num_buckets
                        .wrapping_mul(2 as ::core::ffi::c_uint);
                    (*(*lookup).hh.tbl).log2_num_buckets =
                        (*(*lookup).hh.tbl).log2_num_buckets.wrapping_add(1);
                    (*(*lookup).hh.tbl).buckets = _he_new_buckets;
                    (*(*lookup).hh.tbl).ineff_expands = if (*(*lookup).hh.tbl).nonideal_items
                        > (*(*lookup).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                    {
                        (*(*lookup).hh.tbl)
                            .ineff_expands
                            .wrapping_add(1 as ::core::ffi::c_uint)
                    } else {
                        0 as ::core::ffi::c_uint
                    };
                    if (*(*lookup).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                        (*(*lookup).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                    }
                }
                if _ha_oomed != 0 {
                    let mut _hd_head: *mut UT_hash_bucket =
                        (*(*lookup_map).hh.tbl).buckets.offset(_ha_bkt as isize)
                            as *mut UT_hash_bucket;
                    (*_hd_head).count -= 1;
                    if (*_hd_head).hh_head == &raw mut (*lookup).hh {
                        (*_hd_head).hh_head = (*lookup).hh.hh_next as *mut UT_hash_handle;
                    }
                    if !(*lookup).hh.hh_prev.is_null() {
                        (*(*lookup).hh.hh_prev).hh_next = (*lookup).hh.hh_next;
                    }
                    if !(*lookup).hh.hh_next.is_null() {
                        (*(*lookup).hh.hh_next).hh_prev = (*lookup).hh.hh_prev;
                    }
                }
            }
            if _ha_oomed != 0 {
                let mut _hd_hh_item: *mut UT_hash_handle = &raw mut (*lookup).hh;
                let mut _hd_bkt: ::core::ffi::c_uint = 0;
                _hd_bkt = (*_hd_hh_item).hashv
                    & (*(*lookup_map).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let fresh4 = &mut (*(*(*lookup_map).hh.tbl).buckets.offset(_hd_bkt as isize)).count;
                *fresh4 = (*fresh4).wrapping_add(1);
                (*_hd_hh_item).hh_next = ::core::ptr::null_mut::<UT_hash_handle>();
                (*_hd_hh_item).hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _hd_hh_del: *const UT_hash_handle = &raw mut (*lookup).hh;
                if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
                    free((*(*lookup_map).hh.tbl).buckets as *mut ::core::ffi::c_void);
                    free((*lookup_map).hh.tbl as *mut ::core::ffi::c_void);
                    lookup_map = ::core::ptr::null_mut::<var_lookup>();
                } else {
                    let mut _hd_bkt_0: ::core::ffi::c_uint = 0;
                    if std::ptr::eq(_hd_hh_del, (*(*lookup_map).hh.tbl).tail) {
                        (*(*lookup_map).hh.tbl).tail = ((*_hd_hh_del).prev
                            as *mut ::core::ffi::c_char)
                            .offset((*(*lookup_map).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut UT_hash_handle
                            as *mut UT_hash_handle;
                    }
                    if !(*_hd_hh_del).prev.is_null() {
                        let fresh5 = &mut (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                            .offset((*(*lookup_map).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut UT_hash_handle))
                            .next;
                        *fresh5 = (*_hd_hh_del).next;
                    } else {
                        lookup_map = (*_hd_hh_del).next as *mut var_lookup as *mut var_lookup;
                    }
                    if !(*_hd_hh_del).next.is_null() {
                        let fresh6 = &mut (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                            .offset((*(*lookup_map).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut UT_hash_handle))
                            .prev;
                        *fresh6 = (*_hd_hh_del).prev;
                    }
                    _hd_bkt_0 = (*_hd_hh_del).hashv
                        & (*(*lookup_map).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    let mut _hd_head_0: *mut UT_hash_bucket =
                        (*(*lookup_map).hh.tbl).buckets.offset(_hd_bkt_0 as isize)
                            as *mut UT_hash_bucket;
                    (*_hd_head_0).count -= 1;
                    if std::ptr::eq((*_hd_head_0).hh_head, _hd_hh_del) {
                        (*_hd_head_0).hh_head = (*_hd_hh_del).hh_next as *mut UT_hash_handle;
                    }
                    if !(*_hd_hh_del).hh_prev.is_null() {
                        (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
                    }
                    if !(*_hd_hh_del).hh_next.is_null() {
                        (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
                    }
                    (*(*lookup_map).hh.tbl).num_items =
                        (*(*lookup_map).hh.tbl).num_items.wrapping_sub(1);
                }
                (*lookup).hh.tbl = ::core::ptr::null_mut::<UT_hash_table>();
                let mut _log_ctx_0 = NULL;
                log_fatal(
                    c"src/varcache.c".as_ptr(),
                    81 as ::core::ffi::c_int,
                    c"init_var_lookup_from_config".as_ptr(),
                    false,
                    _log_ctx_0,
                    c"out of memory".as_ptr(),
                );
                exit(1 as ::core::ffi::c_int);
            }
        } else {
            (*lookup).hh.tbl = ::core::ptr::null_mut::<UT_hash_table>();
            let mut _log_ctx_1 = NULL;
            log_fatal(
                c"src/varcache.c".as_ptr(),
                81 as ::core::ffi::c_int,
                c"init_var_lookup_from_config".as_ptr(),
                false,
                _log_ctx_1,
                c"out of memory".as_ptr(),
            );
            exit(1 as ::core::ffi::c_int);
        }
        free(var_name as *mut ::core::ffi::c_void);
    }
    strlist_free(sl);
}
#[no_mangle]

pub unsafe extern "C" fn init_var_lookup(
    mut cf_track_extra_parameters: *const ::core::ffi::c_char,
) {
    let mut names: [*const ::core::ffi::c_char; 6] = [
        c"DateStyle".as_ptr(),
        c"client_encoding".as_ptr(),
        c"TimeZone".as_ptr(),
        c"standard_conforming_strings".as_ptr(),
        c"application_name".as_ptr(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut idx = 0 as ::core::ffi::c_int;
    let mut lookup = ::core::ptr::null_mut::<var_lookup>();
    while !names[idx as usize].is_null() {
        lookup = malloc(::core::mem::size_of::<var_lookup>() as size_t) as *mut var_lookup;
        (*lookup).name = names[idx as usize];
        (*lookup).idx = idx;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i: ::core::ffi::c_uint = 0;
        let mut _hj_j: ::core::ffi::c_uint = 0;
        let mut _hj_k: ::core::ffi::c_uint = 0;
        let mut _hj_key = (*lookup).name as *const ::core::ffi::c_uchar;
        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i = _hj_j;
        _hj_k = strlen((*lookup).name) as ::core::ffi::c_uint;
        while _hj_k >= 12 as ::core::ffi::c_uint {
            _hj_i = _hj_i.wrapping_add(
                (safe_tolower(
                    *_hj_key as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_j = _hj_j.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _ha_hashv = _ha_hashv.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(*_hj_key.offset(11 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_ha_hashv);
            _hj_i ^= _ha_hashv >> 13 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_ha_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
            _ha_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_ha_hashv);
            _hj_i ^= _ha_hashv >> 12 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_ha_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
            _ha_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_ha_hashv);
            _hj_i ^= _ha_hashv >> 3 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_ha_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
            _ha_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
            _hj_key = _hj_key.offset(12 as ::core::ffi::c_int as isize);
            _hj_k = _hj_k.wrapping_sub(12 as ::core::ffi::c_uint);
        }
        _ha_hashv = _ha_hashv.wrapping_add(strlen((*lookup).name) as ::core::ffi::c_uint);
        let mut current_block_54: u64;
        match _hj_k {
            11 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_54 = 442998152379714613;
            }
            10 => {
                current_block_54 = 442998152379714613;
            }
            9 => {
                current_block_54 = 14916313239819575723;
            }
            8 => {
                current_block_54 = 11772761646808455830;
            }
            7 => {
                current_block_54 = 5536219776913685678;
            }
            6 => {
                current_block_54 = 3967905580732798218;
            }
            5 => {
                current_block_54 = 765793381763078134;
            }
            4 => {
                current_block_54 = 3105948935974009916;
            }
            3 => {
                current_block_54 = 16488506295619998735;
            }
            2 => {
                current_block_54 = 2165477741955893522;
            }
            1 => {
                current_block_54 = 16420434121503669123;
            }
            _ => {
                current_block_54 = 1434579379687443766;
            }
        }
        if current_block_54 == 442998152379714613 {
            _ha_hashv = _ha_hashv.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_54 = 14916313239819575723;
        }
        if current_block_54 == 14916313239819575723 {
            _ha_hashv = _ha_hashv.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_54 = 11772761646808455830;
        }
        if current_block_54 == 11772761646808455830 {
            _hj_j = _hj_j.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_54 = 5536219776913685678;
        }
        if current_block_54 == 5536219776913685678 {
            _hj_j = _hj_j.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_54 = 3967905580732798218;
        }
        if current_block_54 == 3967905580732798218 {
            _hj_j = _hj_j.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_54 = 765793381763078134;
        }
        if current_block_54 == 765793381763078134 {
            _hj_j = _hj_j.wrapping_add(safe_tolower(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint);
            current_block_54 = 3105948935974009916;
        }
        if current_block_54 == 3105948935974009916 {
            _hj_i = _hj_i.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_54 = 16488506295619998735;
        }
        if current_block_54 == 16488506295619998735 {
            _hj_i = _hj_i.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_54 = 2165477741955893522;
        }
        if current_block_54 == 2165477741955893522 {
            _hj_i = _hj_i.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_54 = 16420434121503669123;
        }
        if current_block_54 == 16420434121503669123 {
            _hj_i = _hj_i.wrapping_add(safe_tolower(
                *_hj_key as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint);
        }
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_ha_hashv);
        _hj_i ^= _ha_hashv >> 13 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_ha_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
        _ha_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_ha_hashv);
        _hj_i ^= _ha_hashv >> 12 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_ha_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
        _ha_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_ha_hashv);
        _hj_i ^= _ha_hashv >> 3 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_ha_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
        _ha_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
        let mut _ha_oomed = 0 as ::core::ffi::c_int;
        (*lookup).hh.hashv = _ha_hashv;
        (*lookup).hh.key = (*lookup).name as *const ::core::ffi::c_void;
        (*lookup).hh.keylen = strlen((*lookup).name) as ::core::ffi::c_uint;
        if lookup_map.is_null() {
            (*lookup).hh.next = NULL;
            (*lookup).hh.prev = NULL;
            (*lookup).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as size_t)
                as *mut UT_hash_table as *mut UT_hash_table;
            if (*lookup).hh.tbl.is_null() {
                _ha_oomed = 1 as ::core::ffi::c_int;
            } else {
                memset(
                    (*lookup).hh.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UT_hash_table>() as size_t,
                );
                (*(*lookup).hh.tbl).tail = &raw mut (*lookup).hh as *mut UT_hash_handle;
                (*(*lookup).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*lookup).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*lookup).hh.tbl).hho = (&raw mut (*lookup).hh as *mut ::core::ffi::c_char)
                    .offset_from(lookup as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as ptrdiff_t;
                (*(*lookup).hh.tbl).buckets = malloc(
                    (32 as size_t).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                ) as *mut UT_hash_bucket;
                (*(*lookup).hh.tbl).signature = HASH_SIGNATURE as uint32_t;
                if (*(*lookup).hh.tbl).buckets.is_null() {
                    _ha_oomed = 1 as ::core::ffi::c_int;
                    free((*lookup).hh.tbl as *mut ::core::ffi::c_void);
                } else {
                    memset(
                        (*(*lookup).hh.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as size_t)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                    );
                    if _ha_oomed != 0 {
                        free((*(*lookup).hh.tbl).buckets as *mut ::core::ffi::c_void);
                        free((*lookup).hh.tbl as *mut ::core::ffi::c_void);
                    }
                }
            }
            if _ha_oomed == 0 {
                lookup_map = lookup;
            }
        } else {
            (*lookup).hh.tbl = (*lookup_map).hh.tbl;
            (*lookup).hh.next = NULL;
            (*lookup).hh.prev = ((*(*lookup_map).hh.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*lookup_map).hh.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*lookup_map).hh.tbl).tail).next = lookup as *mut ::core::ffi::c_void;
            (*(*lookup_map).hh.tbl).tail = &raw mut (*lookup).hh as *mut UT_hash_handle;
        }
        if _ha_oomed == 0 {
            let mut _ha_bkt: ::core::ffi::c_uint = 0;
            (*(*lookup_map).hh.tbl).num_items = (*(*lookup_map).hh.tbl).num_items.wrapping_add(1);
            _ha_bkt = _ha_hashv
                & (*(*lookup_map).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _ha_head: *mut UT_hash_bucket =
                (*(*lookup_map).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UT_hash_bucket;
            (*_ha_head).count += 1;
            (*lookup).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
            (*lookup).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
            if !(*_ha_head).hh_head.is_null() {
                (*(*_ha_head).hh_head).hh_prev = &raw mut (*lookup).hh as *mut UT_hash_handle;
            }
            (*_ha_head).hh_head = &raw mut (*lookup).hh as *mut UT_hash_handle;
            if (*_ha_head).count
                >= (*_ha_head)
                    .expand_mult
                    .wrapping_add(1 as ::core::ffi::c_uint)
                    .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                && (*(*lookup).hh.tbl).noexpand == 0
            {
                let mut _he_bkt: ::core::ffi::c_uint = 0;
                let mut _he_bkt_i: ::core::ffi::c_uint = 0;
                let mut _he_thh = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _he_hh_nxt = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _he_new_buckets = ::core::ptr::null_mut::<UT_hash_bucket>();
                let mut _he_newbkt = ::core::ptr::null_mut::<UT_hash_bucket>();
                _he_new_buckets = malloc(
                    (::core::mem::size_of::<UT_hash_bucket>() as size_t)
                        .wrapping_mul((*(*lookup).hh.tbl).num_buckets as size_t)
                        .wrapping_mul(2 as size_t),
                ) as *mut UT_hash_bucket;
                if _he_new_buckets.is_null() {
                    _ha_oomed = 1 as ::core::ffi::c_int;
                } else {
                    memset(
                        _he_new_buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (::core::mem::size_of::<UT_hash_bucket>() as size_t)
                            .wrapping_mul((*(*lookup).hh.tbl).num_buckets as size_t)
                            .wrapping_mul(2 as size_t),
                    );
                    (*(*lookup).hh.tbl).ideal_chain_maxlen = ((*(*lookup).hh.tbl).num_items
                        >> (*(*lookup).hh.tbl)
                            .log2_num_buckets
                            .wrapping_add(1 as ::core::ffi::c_uint))
                    .wrapping_add(
                        if (*(*lookup).hh.tbl).num_items
                            & (*(*lookup).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint)
                            != 0 as ::core::ffi::c_uint
                        {
                            1 as ::core::ffi::c_uint
                        } else {
                            0 as ::core::ffi::c_uint
                        },
                    );
                    (*(*lookup).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                    _he_bkt_i = 0 as ::core::ffi::c_uint;
                    while _he_bkt_i < (*(*lookup).hh.tbl).num_buckets {
                        _he_thh = (*(*(*lookup).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                            as *mut UT_hash_handle;
                        while !_he_thh.is_null() {
                            _he_hh_nxt = (*_he_thh).hh_next;
                            _he_bkt = (*_he_thh).hashv
                                & (*(*lookup).hh.tbl)
                                    .num_buckets
                                    .wrapping_mul(2 as ::core::ffi::c_uint)
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            _he_newbkt =
                                _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                            (*_he_newbkt).count += 1;
                            if (*_he_newbkt).count > (*(*lookup).hh.tbl).ideal_chain_maxlen {
                                (*(*lookup).hh.tbl).nonideal_items =
                                    (*(*lookup).hh.tbl).nonideal_items.wrapping_add(1);
                                if (*_he_newbkt).count
                                    > (*_he_newbkt)
                                        .expand_mult
                                        .wrapping_mul((*(*lookup).hh.tbl).ideal_chain_maxlen)
                                {
                                    (*_he_newbkt).expand_mult += 1;
                                }
                            }
                            (*_he_thh).hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                            (*_he_thh).hh_next = (*_he_newbkt).hh_head as *mut UT_hash_handle;
                            if !(*_he_newbkt).hh_head.is_null() {
                                (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                            }
                            (*_he_newbkt).hh_head = _he_thh as *mut UT_hash_handle;
                            _he_thh = _he_hh_nxt;
                        }
                        _he_bkt_i = _he_bkt_i.wrapping_add(1);
                    }
                    free((*(*lookup).hh.tbl).buckets as *mut ::core::ffi::c_void);
                    (*(*lookup).hh.tbl).num_buckets = (*(*lookup).hh.tbl)
                        .num_buckets
                        .wrapping_mul(2 as ::core::ffi::c_uint);
                    (*(*lookup).hh.tbl).log2_num_buckets =
                        (*(*lookup).hh.tbl).log2_num_buckets.wrapping_add(1);
                    (*(*lookup).hh.tbl).buckets = _he_new_buckets;
                    (*(*lookup).hh.tbl).ineff_expands = if (*(*lookup).hh.tbl).nonideal_items
                        > (*(*lookup).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                    {
                        (*(*lookup).hh.tbl)
                            .ineff_expands
                            .wrapping_add(1 as ::core::ffi::c_uint)
                    } else {
                        0 as ::core::ffi::c_uint
                    };
                    if (*(*lookup).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                        (*(*lookup).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                    }
                }
                if _ha_oomed != 0 {
                    let mut _hd_head: *mut UT_hash_bucket =
                        (*(*lookup_map).hh.tbl).buckets.offset(_ha_bkt as isize)
                            as *mut UT_hash_bucket;
                    (*_hd_head).count -= 1;
                    if (*_hd_head).hh_head == &raw mut (*lookup).hh {
                        (*_hd_head).hh_head = (*lookup).hh.hh_next as *mut UT_hash_handle;
                    }
                    if !(*lookup).hh.hh_prev.is_null() {
                        (*(*lookup).hh.hh_prev).hh_next = (*lookup).hh.hh_next;
                    }
                    if !(*lookup).hh.hh_next.is_null() {
                        (*(*lookup).hh.hh_next).hh_prev = (*lookup).hh.hh_prev;
                    }
                }
            }
            if _ha_oomed != 0 {
                let mut _hd_hh_item: *mut UT_hash_handle = &raw mut (*lookup).hh;
                let mut _hd_bkt: ::core::ffi::c_uint = 0;
                _hd_bkt = (*_hd_hh_item).hashv
                    & (*(*lookup_map).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let fresh0 = &mut (*(*(*lookup_map).hh.tbl).buckets.offset(_hd_bkt as isize)).count;
                *fresh0 = (*fresh0).wrapping_add(1);
                (*_hd_hh_item).hh_next = ::core::ptr::null_mut::<UT_hash_handle>();
                (*_hd_hh_item).hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _hd_hh_del: *const UT_hash_handle = &raw mut (*lookup).hh;
                if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
                    free((*(*lookup_map).hh.tbl).buckets as *mut ::core::ffi::c_void);
                    free((*lookup_map).hh.tbl as *mut ::core::ffi::c_void);
                    lookup_map = ::core::ptr::null_mut::<var_lookup>();
                } else {
                    let mut _hd_bkt_0: ::core::ffi::c_uint = 0;
                    if std::ptr::eq(_hd_hh_del, (*(*lookup_map).hh.tbl).tail) {
                        (*(*lookup_map).hh.tbl).tail = ((*_hd_hh_del).prev
                            as *mut ::core::ffi::c_char)
                            .offset((*(*lookup_map).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut UT_hash_handle
                            as *mut UT_hash_handle;
                    }
                    if !(*_hd_hh_del).prev.is_null() {
                        let fresh1 = &mut (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                            .offset((*(*lookup_map).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut UT_hash_handle))
                            .next;
                        *fresh1 = (*_hd_hh_del).next;
                    } else {
                        lookup_map = (*_hd_hh_del).next as *mut var_lookup as *mut var_lookup;
                    }
                    if !(*_hd_hh_del).next.is_null() {
                        let fresh2 = &mut (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                            .offset((*(*lookup_map).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut UT_hash_handle))
                            .prev;
                        *fresh2 = (*_hd_hh_del).prev;
                    }
                    _hd_bkt_0 = (*_hd_hh_del).hashv
                        & (*(*lookup_map).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    let mut _hd_head_0: *mut UT_hash_bucket =
                        (*(*lookup_map).hh.tbl).buckets.offset(_hd_bkt_0 as isize)
                            as *mut UT_hash_bucket;
                    (*_hd_head_0).count -= 1;
                    if std::ptr::eq((*_hd_head_0).hh_head, _hd_hh_del) {
                        (*_hd_head_0).hh_head = (*_hd_hh_del).hh_next as *mut UT_hash_handle;
                    }
                    if !(*_hd_hh_del).hh_prev.is_null() {
                        (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
                    }
                    if !(*_hd_hh_del).hh_next.is_null() {
                        (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
                    }
                    (*(*lookup_map).hh.tbl).num_items =
                        (*(*lookup_map).hh.tbl).num_items.wrapping_sub(1);
                }
                (*lookup).hh.tbl = ::core::ptr::null_mut::<UT_hash_table>();
                let mut _log_ctx = NULL;
                log_fatal(
                    c"src/varcache.c".as_ptr(),
                    101 as ::core::ffi::c_int,
                    c"init_var_lookup".as_ptr(),
                    false,
                    _log_ctx,
                    c"out of memory".as_ptr(),
                );
                exit(1 as ::core::ffi::c_int);
            }
        } else {
            (*lookup).hh.tbl = ::core::ptr::null_mut::<UT_hash_table>();
            let mut _log_ctx_0 = NULL;
            log_fatal(
                c"src/varcache.c".as_ptr(),
                101 as ::core::ffi::c_int,
                c"init_var_lookup".as_ptr(),
                false,
                _log_ctx_0,
                c"out of memory".as_ptr(),
            );
            exit(1 as ::core::ffi::c_int);
        }
        idx += 1;
    }
    init_var_lookup_from_config(cf_track_extra_parameters, &raw mut idx);
    num_var_cached = idx;
}
#[no_mangle]

pub unsafe extern "C" fn varcache_set(
    mut cache: *mut VarCache,
    mut key: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) -> bool {
    let mut lk = ::core::ptr::null::<var_lookup>();
    let mut pstr = ::core::ptr::null_mut::<PStr>();
    if vpool.is_null() {
        vpool = strpool_create(&raw const cx_libc_allocator);
        if vpool.is_null() {
            return false;
        }
    }
    let mut _uthash_hfstr_keylen = strlen(key) as ::core::ffi::c_uint;
    lk = ::core::ptr::null::<var_lookup>();
    if !lookup_map.is_null() {
        let mut _hf_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i: ::core::ffi::c_uint = 0;
        let mut _hj_j: ::core::ffi::c_uint = 0;
        let mut _hj_k: ::core::ffi::c_uint = 0;
        let mut _hj_key = key as *const ::core::ffi::c_uchar;
        _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i = _hj_j;
        _hj_k = _uthash_hfstr_keylen;
        while _hj_k >= 12 as ::core::ffi::c_uint {
            _hj_i = _hj_i.wrapping_add(
                (safe_tolower(
                    *_hj_key as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_j = _hj_j.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hf_hashv = _hf_hashv.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(*_hj_key.offset(11 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
            _hj_key = _hj_key.offset(12 as ::core::ffi::c_int as isize);
            _hj_k = _hj_k.wrapping_sub(12 as ::core::ffi::c_uint);
        }
        _hf_hashv = _hf_hashv.wrapping_add(_uthash_hfstr_keylen);
        let mut current_block_57: u64;
        match _hj_k {
            11 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_57 = 9437074751588942005;
            }
            10 => {
                current_block_57 = 9437074751588942005;
            }
            9 => {
                current_block_57 = 4405230302860763820;
            }
            8 => {
                current_block_57 = 9434220525754491920;
            }
            7 => {
                current_block_57 = 1270338533819009205;
            }
            6 => {
                current_block_57 = 3731882003230796202;
            }
            5 => {
                current_block_57 = 8650755990458127310;
            }
            4 => {
                current_block_57 = 4760653373201335839;
            }
            3 => {
                current_block_57 = 11469350245903324201;
            }
            2 => {
                current_block_57 = 11617363695825217706;
            }
            1 => {
                current_block_57 = 14328819089967601331;
            }
            _ => {
                current_block_57 = 13460095289871124136;
            }
        }
        if current_block_57 == 9437074751588942005 {
            _hf_hashv = _hf_hashv.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_57 = 4405230302860763820;
        }
        if current_block_57 == 4405230302860763820 {
            _hf_hashv = _hf_hashv.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_57 = 9434220525754491920;
        }
        if current_block_57 == 9434220525754491920 {
            _hj_j = _hj_j.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_57 = 1270338533819009205;
        }
        if current_block_57 == 1270338533819009205 {
            _hj_j = _hj_j.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_57 = 3731882003230796202;
        }
        if current_block_57 == 3731882003230796202 {
            _hj_j = _hj_j.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_57 = 8650755990458127310;
        }
        if current_block_57 == 8650755990458127310 {
            _hj_j = _hj_j.wrapping_add(safe_tolower(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint);
            current_block_57 = 4760653373201335839;
        }
        if current_block_57 == 4760653373201335839 {
            _hj_i = _hj_i.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_57 = 11469350245903324201;
        }
        if current_block_57 == 11469350245903324201 {
            _hj_i = _hj_i.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_57 = 11617363695825217706;
        }
        if current_block_57 == 11617363695825217706 {
            _hj_i = _hj_i.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_57 = 14328819089967601331;
        }
        if current_block_57 == 14328819089967601331 {
            _hj_i = _hj_i.wrapping_add(safe_tolower(
                *_hj_key as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint);
        }
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
        lk = ::core::ptr::null::<var_lookup>();
        if !lookup_map.is_null() {
            let mut _hf_bkt: ::core::ffi::c_uint = 0;
            _hf_bkt = _hf_hashv
                & (*(*lookup_map).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            if !(*(*(*lookup_map).hh.tbl).buckets.offset(_hf_bkt as isize))
                .hh_head
                .is_null()
            {
                lk = ((*(*(*lookup_map).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                    as *mut ::core::ffi::c_char)
                    .offset(-(*(*lookup_map).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *const var_lookup
                    as *const var_lookup;
            } else {
                lk = ::core::ptr::null::<var_lookup>();
            }
            while !lk.is_null() {
                if (*lk).hh.hashv == _hf_hashv
                    && (*lk).hh.keylen == _uthash_hfstr_keylen
                    && strcasecmp((*lk).hh.key as *const ::core::ffi::c_char, key)
                        == 0 as ::core::ffi::c_int
                {
                    break;
                }
                if !(*lk).hh.hh_next.is_null() {
                    lk = ((*lk).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*lookup_map).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *const var_lookup
                        as *const var_lookup;
                } else {
                    lk = ::core::ptr::null::<var_lookup>();
                }
            }
        }
    }
    if lk.is_null() {
        return false;
    }
    strpool_decref(*(*cache).var_list.offset((*lk).idx as isize));
    let fresh7 = &mut *(*cache).var_list.offset((*lk).idx as isize);
    *fresh7 = ::core::ptr::null_mut::<PStr>();
    if value.is_null() {
        return false;
    }
    pstr = strpool_get(vpool, value, strlen(value) as ssize_t);
    if pstr.is_null() {
        return false;
    }
    let fresh8 = &mut *(*cache).var_list.offset((*lk).idx as isize);
    *fresh8 = pstr;
    true
}

unsafe extern "C" fn variable_is_guc_list_quote(mut key: *const ::core::ffi::c_char) -> bool {
    if strcasecmp(c"search_path".as_ptr(), key) == 0 as ::core::ffi::c_int {
        return true;
    }
    false
}

unsafe extern "C" fn apply_var(
    mut pkt: *mut PktBuf,
    mut key: *const ::core::ffi::c_char,
    mut cval: *const PStr,
    mut sval: *const PStr,
) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_char; 300] = [0; 300];
    let mut qbuf: [::core::ffi::c_char; 128] = [0; 128];
    let mut len: ::core::ffi::c_uint = 0;
    let mut tmp = ::core::ptr::null::<::core::ffi::c_char>();
    if cval.is_null() || sval.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if cval == sval {
        return 0 as ::core::ffi::c_int;
    }
    if strcasecmp(
        &raw const (*cval).str_0 as *const ::core::ffi::c_char,
        &raw const (*sval).str_0 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if variable_is_guc_list_quote(key) {
        if strcmp(
            &raw const (*cval).str_0 as *const ::core::ffi::c_char,
            b"\"\"\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            tmp = c"''".as_ptr();
        } else {
            tmp = &raw const (*cval).str_0 as *const ::core::ffi::c_char;
        }
    } else if pg_quote_literal(
        &raw mut qbuf as *mut ::core::ffi::c_char,
        &raw const (*cval).str_0 as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as ::core::ffi::c_int,
    ) {
        tmp = &raw mut qbuf as *mut ::core::ffi::c_char;
    } else {
        return 0 as ::core::ffi::c_int;
    }
    len = snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 300]>() as size_t,
        c"SET %s=%s;".as_ptr(),
        key,
        tmp,
    ) as ::core::ffi::c_uint;
    if (len as usize) < ::core::mem::size_of::<[::core::ffi::c_char; 300]>() {
        pktbuf_put_bytes(
            pkt,
            &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            len as ::core::ffi::c_int,
        );
    } else {
        let mut buf2 = malloc(
            (::core::mem::size_of::<::core::ffi::c_char>() as size_t).wrapping_mul(len as size_t),
        ) as *mut ::core::ffi::c_char;
        if buf2.is_null() {
            let mut _log_ctx = NULL;
            log_generic(
                LG_FATAL,
                _log_ctx,
                c"failed to allocate memory in apply_var".as_ptr(),
            );
            exit(1 as ::core::ffi::c_int);
        }
        snprintf(buf2, len as size_t, c"SET %s=%s;".as_ptr(), key, tmp);
        pktbuf_put_bytes(
            pkt,
            buf2 as *const ::core::ffi::c_void,
            len as ::core::ffi::c_int,
        );
        free(buf2 as *mut ::core::ffi::c_void);
    }
    1 as ::core::ffi::c_int
}
#[no_mangle]

pub unsafe extern "C" fn varcache_apply(
    mut server: *mut PgSocket,
    mut client: *mut PgSocket,
    mut changes_p: *mut bool,
) -> bool {
    let mut changes = 0 as ::core::ffi::c_int;
    let mut cval = ::core::ptr::null_mut::<PStr>();
    let mut sval = ::core::ptr::null_mut::<PStr>();
    let mut lk = ::core::ptr::null::<var_lookup>();
    let mut tmp = ::core::ptr::null::<var_lookup>();
    let mut sql_ofs: ::core::ffi::c_int = 0;
    let mut pkt = pktbuf_temp();
    pktbuf_start_packet(pkt as *mut PktBuf, PqMsg_Query);
    sql_ofs = (*pkt).write_pos;
    lk = lookup_map;
    tmp = (if !lookup_map.is_null() {
        (*lookup_map).hh.next
    } else {
        NULL
    }) as *const var_lookup as *const var_lookup;
    while !lk.is_null() {
        sval = get_value(&raw mut (*server).vars, lk);
        cval = get_value(&raw mut (*client).vars, lk);
        changes += apply_var(pkt as *mut PktBuf, (*lk).name, cval, sval);
        lk = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *const var_lookup
            as *const var_lookup;
    }
    *changes_p = changes > 0 as ::core::ffi::c_int;
    if changes == 0 {
        return true;
    }
    pktbuf_put_char(pkt as *mut PktBuf, 0 as ::core::ffi::c_char);
    pktbuf_finish_packet(pkt as *mut PktBuf);
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            server as *mut ::core::ffi::c_void,
            c"varcache_apply: %s".as_ptr(),
            (*pkt).buf.offset(sql_ofs as isize),
        );
    }
    pktbuf_send_immediate(pkt as *mut PktBuf, server)
}
#[no_mangle]

pub unsafe extern "C" fn varcache_set_canonical(
    mut server: *mut PgSocket,
    mut client: *mut PgSocket,
) {
    let mut server_val = ::core::ptr::null_mut::<PStr>();
    let mut client_val = ::core::ptr::null_mut::<PStr>();
    let mut lk = ::core::ptr::null::<var_lookup>();
    let mut tmp = ::core::ptr::null::<var_lookup>();
    lk = lookup_map;
    tmp = (if !lookup_map.is_null() {
        (*lookup_map).hh.next
    } else {
        NULL
    }) as *const var_lookup as *const var_lookup;
    while !lk.is_null() {
        server_val = *(*server).vars.var_list.offset((*lk).idx as isize);
        client_val = *(*client).vars.var_list.offset((*lk).idx as isize);
        if !client_val.is_null() && !server_val.is_null() && client_val != server_val {
            if cf_verbose > 0 as ::core::ffi::c_int
            {
                log_generic(
                    LG_DEBUG,
                    client as *mut ::core::ffi::c_void,
                    c"varcache_set_canonical: setting %s to its canonical version %s -> %s"
                        .as_ptr(),
                    (*lk).name,
                    &raw mut (*client_val).str_0 as *mut ::core::ffi::c_char,
                    &raw mut (*server_val).str_0 as *mut ::core::ffi::c_char,
                );
            }
            strpool_incref(server_val);
            strpool_decref(client_val);
            let fresh11 = &mut *(*client).vars.var_list.offset((*lk).idx as isize);
            *fresh11 = server_val;
        }
        lk = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *const var_lookup
            as *const var_lookup;
    }
}
#[no_mangle]

pub unsafe extern "C" fn varcache_apply_startup(mut pkt: *mut PktBuf, mut client: *mut PgSocket) {
    let mut lk = ::core::ptr::null::<var_lookup>();
    let mut tmp = ::core::ptr::null::<var_lookup>();
    lk = lookup_map;
    tmp = (if !lookup_map.is_null() {
        (*lookup_map).hh.next
    } else {
        NULL
    }) as *const var_lookup as *const var_lookup;
    while !lk.is_null() {
        let mut val = get_value(&raw mut (*client).vars, lk);
        if !val.is_null() {
            if cf_verbose > 0 as ::core::ffi::c_int
            {
                log_generic(
                    LG_DEBUG,
                    client as *mut ::core::ffi::c_void,
                    c"varcache_apply_startup: %s=%s".as_ptr(),
                    (*lk).name,
                    &raw mut (*val).str_0 as *mut ::core::ffi::c_char,
                );
            }
            pktbuf_put_string(pkt, (*lk).name);
            pktbuf_put_string(pkt, &raw mut (*val).str_0 as *mut ::core::ffi::c_char);
        }
        lk = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *const var_lookup
            as *const var_lookup;
    }
}
#[no_mangle]

pub unsafe extern "C" fn varcache_fill_unset(mut src: *mut VarCache, mut dst: *mut PgSocket) {
    let mut srcval = ::core::ptr::null_mut::<PStr>();
    let mut dstval = ::core::ptr::null_mut::<PStr>();
    let mut lk = ::core::ptr::null::<var_lookup>();
    let mut tmp = ::core::ptr::null::<var_lookup>();
    lk = lookup_map;
    tmp = (if !lookup_map.is_null() {
        (*lookup_map).hh.next
    } else {
        NULL
    }) as *const var_lookup as *const var_lookup;
    while !lk.is_null() {
        srcval = *(*src).var_list.offset((*lk).idx as isize);
        dstval = *(*dst).vars.var_list.offset((*lk).idx as isize);
        if dstval.is_null() {
            strpool_incref(srcval);
            let fresh9 = &mut *(*dst).vars.var_list.offset((*lk).idx as isize);
            *fresh9 = srcval;
        }
        lk = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *const var_lookup
            as *const var_lookup;
    }
}
#[no_mangle]

pub unsafe extern "C" fn varcache_clean(mut cache: *mut VarCache) {
    let mut i = 0 as ::core::ffi::c_int;
    while i < num_var_cached {
        strpool_decref(*(*cache).var_list.offset(i as isize));
        let fresh10 = &mut *(*cache).var_list.offset(i as isize);
        *fresh10 = ::core::ptr::null_mut::<PStr>();
        i += 1;
    }
}
#[no_mangle]

pub unsafe extern "C" fn varcache_add_params(mut pkt: *mut PktBuf, mut vars: *mut VarCache) {
    let mut val = ::core::ptr::null_mut::<PStr>();
    let mut lk = ::core::ptr::null::<var_lookup>();
    let mut tmp = ::core::ptr::null::<var_lookup>();
    lk = lookup_map;
    tmp = (if !lookup_map.is_null() {
        (*lookup_map).hh.next
    } else {
        NULL
    }) as *const var_lookup as *const var_lookup;
    while !lk.is_null() {
        val = *(*vars).var_list.offset((*lk).idx as isize);
        if !val.is_null() {
            pktbuf_write_generic(
                pkt,
                PqMsg_ParameterStatus,
                c"ss".as_ptr(),
                (*lk).name,
                &raw mut (*val).str_0 as *mut ::core::ffi::c_char,
            );
        }
        lk = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *const var_lookup
            as *const var_lookup;
    }
}
#[no_mangle]

pub unsafe extern "C" fn varcache_deinit() {
    strpool_free(vpool);
    vpool = ::core::ptr::null_mut::<StrPool>();
}

extern "C" {
    pub fn strpool_create(ca: *const CxMem) -> *mut StrPool;
    pub fn strpool_free(sp: *mut StrPool);
    pub fn strpool_get(
        sp: *mut StrPool,
        str: *const ::core::ffi::c_char,
        len: ssize_t,
    ) -> *mut PStr;
    pub fn strpool_incref(str: *mut PStr);
    pub fn strpool_decref(str: *mut PStr);
}
