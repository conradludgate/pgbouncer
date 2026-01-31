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
    use crate::types::blkcnt_t;
    use crate::types::blksize_t;
    use crate::types::dev_t;
    use crate::types::gid_t;
    use crate::types::mode_t;
    use crate::types::nlink_t;
    use crate::types::off_t;
    use crate::types::timespec;
    use crate::types::__darwin_ino64_t;
    use crate::types::uid_t;
    use crate::types::{__int32_t, __int64_t, __uint32_t};
    extern "C" {

        pub fn lstat(_: *const ::core::ffi::c_char, _: *mut stat) -> ::core::ffi::c_int;
    }
}

pub mod un_h {

    pub use crate::types::*;


}

pub mod netdb_h {

    pub use crate::types::*;

    pub use crate::types::*;
    extern "C" {

        pub fn freeaddrinfo(_: *mut addrinfo);

        pub fn gai_strerror(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;

        pub fn getaddrinfo(
            _: *const ::core::ffi::c_char,
            _: *const ::core::ffi::c_char,
            _: *const addrinfo,
            _: *mut *mut addrinfo,
        ) -> ::core::ffi::c_int;
    }




}

pub mod bouncer_h {

    pub use crate::types::*;

    pub use c2rust_bitfields::BitfieldStruct;

    extern "C" {




    pub static mut pgb_event_base: *mut event_base;




    pub fn pga_port(a: *const PgAddr) -> ::core::ffi::c_int;




    pub fn pga_set(a: *mut PgAddr, fam: ::core::ffi::c_int, port: ::core::ffi::c_int);




    pub fn pga_copy(a: *mut PgAddr, sa: *const sockaddr);




    pub fn pga_ntop(



    a: *const PgAddr,



    dst: *mut ::core::ffi::c_char,



    dstlen: ::core::ffi::c_int,



    ) -> *const ::core::ffi::c_char;




    pub fn pga_str(



    a: *const PgAddr,



    dst: *mut ::core::ffi::c_char,



    dstlen: ::core::ffi::c_int,



    ) -> *const ::core::ffi::c_char;




    pub static mut cf_unix_socket_dir: *mut ::core::ffi::c_char;




    pub static mut cf_unix_socket_mode: ::core::ffi::c_int;




    pub static mut cf_unix_socket_group: *mut ::core::ffi::c_char;




    pub static mut cf_listen_addr: *mut ::core::ffi::c_char;




    pub static mut cf_listen_port: ::core::ffi::c_int;




    pub static mut cf_listen_backlog: ::core::ffi::c_int;




    pub static mut cf_pause_mode: ::core::ffi::c_int;




    pub static mut cf_so_reuseport: ::core::ffi::c_int;




    pub static mut cf_tcp_defer_accept: ::core::ffi::c_int;



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

pub mod string_h {

    pub type str_cb =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char) -> bool>;
    extern "C" {

        pub fn parse_word_list(
            s: *const ::core::ffi::c_char,
            cb_func: str_cb,
            cb_arg: *mut ::core::ffi::c_void,
        ) -> bool;
    }
}

pub mod pooler_h {

    pub type pooler_cb = Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int, *const PgAddr) -> bool,
    >;
    use super::bouncer_h::PgAddr;
}

pub mod system_h {
    use crate::types::mode_t;
    extern "C" {

        pub fn change_file_mode(
            fn_0: *const ::core::ffi::c_char,
            mode: mode_t,
            user: *const ::core::ffi::c_char,
            group: *const ::core::ffi::c_char,
        );
    }
}

pub mod util_h {
    use crate::types::event;
    use crate::types::timeval;
    extern "C" {

        pub fn tune_socket(sock: ::core::ffi::c_int, is_unix: bool) -> bool;

        pub fn safe_evtimer_add(ev: *mut event, tv: *mut timeval);
    }
}

pub mod objects_h {

    pub use crate::types::*;
    extern "C" {

        pub fn accept_client(sock: ::core::ffi::c_int, is_unix: bool) -> *mut PgSocket;
    }


}

pub mod unistd_h {
    extern "C" {

        pub fn unlink(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    }
}

pub use crate::types::blkcnt_t;
pub use crate::types::blksize_t;
pub use crate::types::dev_t;
pub use crate::types::gid_t;
use crate::types::{calloc, free};
pub use crate::types::mode_t;
pub use crate::types::nlink_t;
pub use crate::types::off_t;
pub use crate::types::socklen_t;
use crate::types::snprintf;
use crate::types::{atexit, exit};
use crate::types::{memset, strcmp, strerror, strlen};
pub use crate::types::timespec;
pub use self::bouncer_h::{
    cf_listen_addr, cf_listen_backlog, cf_listen_port, cf_pause_mode, cf_so_reuseport,
    cf_tcp_defer_accept, cf_unix_socket_dir, cf_unix_socket_group, cf_unix_socket_mode, pga_copy,
    pga_is_unix, pga_ntop, pga_port, pga_set, pga_str, pgb_event_base, sockaddr_ucreds,
    C2RustUnnamed_9, CallbackState, LoadBalanceHosts, PacketCallbackFlag, PauseMode, PgAddr,
    PgCredentials, PgDatabase, PgGlobalUser, PgPool, PgSocket, PgStats, ReplicationType,
    ScramState, SocketState, CB_HANDLE_COMPLETE_PACKET, CB_NONE, CB_WANT_COMPLETE_PACKET,
    CL_ACTIVE, CL_ACTIVE_CANCEL, CL_FREE, CL_JUSTFREE, CL_LOGIN, CL_WAITING, CL_WAITING_CANCEL,
    CL_WAITING_LOGIN, DEFAULT_UNIX_SOCKET_DIR, LOAD_BALANCE_HOSTS_DISABLE,
    LOAD_BALANCE_HOSTS_ROUND_ROBIN, P_NONE, P_PAUSE, P_SUSPEND, REPLICATION_LOGICAL,
    REPLICATION_NONE, REPLICATION_PHYSICAL, SD_LISTEN_FDS_START, SV_ACTIVE, SV_ACTIVE_CANCEL,
    SV_BEING_CANCELED, SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN, SV_TESTED, SV_USED,
};
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
    __darwin_ptrdiff_t, __darwin_size_t, __darwin_socklen_t, __darwin_ssize_t, __darwin_time_t,
    __int32_t, __int64_t, __uint16_t, __uint32_t, __uint64_t, __uint8_t,
};
pub use crate::types::{aatree_cmp_f, aatree_walker_f, AANode, AATree};
pub use crate::types::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};

pub use crate::types::{__error, EAGAIN, ECONNABORTED, EINVAL};
pub use crate::types::{
    event_add, event_assign, event_base, event_callback_fn, event_del, EV_PERSIST, EV_READ,
};
pub use crate::types::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
pub use crate::types::{in6_addr, sockaddr_in6, C2RustUnnamed, IPV6_V6ONLY};
pub use crate::types::{in_addr, sockaddr_in, IPPROTO_IPV6, IPPROTO_TCP};
pub use self::iobuf_h::{iobuf, IOBuf};
pub use crate::types::{
    cf_verbose, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE, LG_STATS,
    LG_WARNING,
};
pub use self::netdb_h::{addrinfo, freeaddrinfo, gai_strerror, getaddrinfo, AI_PASSIVE};
use self::objects_h::accept_client;
pub use self::pktbuf_h::PktBuf;
pub use self::pooler_h::pooler_cb;
use crate::types::{safe_accept, safe_close};
pub use self::sbuf_h::{
    sbuf_cb_t, SBuf, SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK, SBUF_EV_FLUSH,
    SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY,
};
pub use crate::types::{
    bind, getsockname, listen, setsockopt, sockaddr, sockaddr_storage, socket, AF_INET, AF_INET6,
    AF_UNIX, AF_UNSPEC, SOCK_STREAM, SOL_SOCKET, SO_REUSEADDR, SO_REUSEPORT,
};
pub use self::stat_h::{lstat, stat};
pub use self::string_h::{parse_word_list, str_cb};
pub use crate::types::{
    __darwin_blkcnt_t, __darwin_blksize_t, __darwin_dev_t, __darwin_gid_t, __darwin_ino64_t,
    __darwin_mode_t, __darwin_off_t, __darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t,
    __DARWIN_NULL,
};
use self::system_h::change_file_mode;
pub use crate::types::usec_t;
pub use crate::types::MBuf;
pub use crate::types::PktHdr;
pub use crate::types::{false_0, true_0};
pub use crate::types::{list_append, list_del, list_init, List};
pub use crate::types::{statlist_append, statlist_count, statlist_remove, StatList};
pub use crate::types::{PStr, StrPool};
pub use crate::types::{PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement};

pub use self::un_h::sockaddr_un;
use self::unistd_h::unlink;
use crate::types::sa2str;
use self::util_h::{safe_evtimer_add, tune_socket};
pub use crate::types::VarCache;
pub use crate::types::{UT_hash_bucket, UT_hash_handle, UT_hash_table};
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ListenSocket {
    pub node: List,
    pub fd: ::core::ffi::c_int,
    pub active: bool,
    pub ev: event,
    pub addr: PgAddr,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub union C2RustUnnamed_10 {
    pub in_0: sockaddr_in,
    pub in6: sockaddr_in6,
    pub un: sockaddr_un,
    pub sa: sockaddr,
}

static mut sock_list: StatList = StatList {
    head: List {
        next: ::core::ptr::null::<List>() as *mut List,
        prev: ::core::ptr::null::<List>() as *mut List,
    },
    cur_count: 0,
};

static mut hints: addrinfo = addrinfo {
    ai_flags: AI_PASSIVE,
    ai_family: AF_UNSPEC,
    ai_socktype: SOCK_STREAM,
    ai_protocol: IPPROTO_TCP,
    ai_addrlen: 0,
    ai_canonname: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    ai_addr: ::core::ptr::null::<sockaddr>() as *mut sockaddr,
    ai_next: ::core::ptr::null::<addrinfo>() as *mut addrinfo,
};

static mut need_active: bool = false;

static mut pooler_active: bool = false;

static mut listen_addr_empty: bool = true;

static mut ev_err: event = event {
    ev_evcallback: event_callback {
        evcb_active_next: C2RustUnnamed_8 {
            tqe_next: ::core::ptr::null::<event_callback>() as *mut event_callback,
            tqe_prev: ::core::ptr::null::<*mut event_callback>() as *mut *mut event_callback,
        },
        evcb_flags: 0,
        evcb_pri: 0,
        evcb_closure: 0,
        evcb_cb_union: C2RustUnnamed_7 {
            evcb_callback: None,
        },
        evcb_arg: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    },
    ev_timeout_pos: C2RustUnnamed_5 {
        ev_next_with_common_timeout: C2RustUnnamed_6 {
            tqe_next: ::core::ptr::null::<event>() as *mut event,
            tqe_prev: ::core::ptr::null::<*mut event>() as *mut *mut event,
        },
    },
    ev_fd: 0,
    ev_base: ::core::ptr::null::<event_base>() as *mut event_base,
    ev_: C2RustUnnamed_0 {
        ev_io: C2RustUnnamed_3 {
            ev_io_next: C2RustUnnamed_4 {
                le_next: ::core::ptr::null::<event>() as *mut event,
                le_prev: ::core::ptr::null::<*mut event>() as *mut *mut event,
            },
            ev_timeout: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        },
    },
    ev_events: 0,
    ev_res: 0,
    ev_timeout: timeval {
        tv_sec: 0,
        tv_usec: 0,
    },
};

static mut err_timeout: timeval = timeval {
    tv_sec: 5 as __darwin_time_t,
    tv_usec: 0 as __darwin_suseconds_t,
};
#[no_mangle]

pub unsafe extern "C" fn cleanup_tcp_sockets() {
    let mut ls = ::core::ptr::null_mut::<ListenSocket>();
    let mut el = ::core::ptr::null_mut::<List>();
    let mut tmp_l = ::core::ptr::null_mut::<List>();
    if cf_pause_mode == P_SUSPEND as ::core::ffi::c_int {
        return;
    }
    el = sock_list.head.next;
    tmp_l = (*sock_list.head.next).next;
    while el != &raw mut sock_list.head {
        ls = (el as *mut ::core::ffi::c_char)
            as *mut ListenSocket;
        if !pga_is_unix(&raw mut (*ls).addr) {
            if event_del(&raw mut (*ls).ev) < 0 as ::core::ffi::c_int {
                let mut _log_ctx = NULL;
                log_generic(
                    LG_WARNING,
                    _log_ctx,
                    c"cleanup_sockets, event_del: %s".as_ptr(),
                    strerror(*__error()),
                );
            }
            if (*ls).fd > 0 as ::core::ffi::c_int {
                safe_close((*ls).fd);
                (*ls).fd = 0 as ::core::ffi::c_int;
            }
            statlist_remove(&raw mut sock_list, el);
            free(ls as *mut ::core::ffi::c_void);
        }
        el = tmp_l;
        tmp_l = (*tmp_l).next;
    }
}
#[no_mangle]

pub unsafe extern "C" fn cleanup_unix_sockets() {
    let mut ls = ::core::ptr::null_mut::<ListenSocket>();
    let mut el = ::core::ptr::null_mut::<List>();
    let mut tmp_l = ::core::ptr::null_mut::<List>();
    if cf_pause_mode == P_SUSPEND as ::core::ffi::c_int {
        return;
    }
    el = sock_list.head.next;
    tmp_l = (*sock_list.head.next).next;
    while el != &raw mut sock_list.head {
        ls = (el as *mut ::core::ffi::c_char)
            as *mut ListenSocket;
        if event_del(&raw mut (*ls).ev) < 0 as ::core::ffi::c_int {
            let mut _log_ctx = NULL;
            log_generic(
                LG_WARNING,
                _log_ctx,
                c"cleanup_sockets, event_del: %s".as_ptr(),
                strerror(*__error()),
            );
        }
        if (*ls).fd > 0 as ::core::ffi::c_int {
            safe_close((*ls).fd);
            (*ls).fd = 0 as ::core::ffi::c_int;
        }
        if pga_is_unix(&raw mut (*ls).addr)
            && *cf_unix_socket_dir as ::core::ffi::c_int
                != '@' as i32
        {
            let mut buf: [::core::ffi::c_char; 126] = [0; 126];
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 126]>() as size_t,
                c"%s/.s.PGSQL.%d".as_ptr(),
                cf_unix_socket_dir,
                cf_listen_port,
            );
            unlink(&raw mut buf as *mut ::core::ffi::c_char);
        }
        statlist_remove(&raw mut sock_list, el);
        free(ls as *mut ::core::ffi::c_void);
        el = tmp_l;
        tmp_l = (*tmp_l).next;
    }
}

unsafe extern "C" fn add_listen(
    mut af: ::core::ffi::c_int,
    mut sa: *const sockaddr,
    mut salen: ::core::ffi::c_int,
) -> bool {
    let mut current_block: u64;
    let mut ls = ::core::ptr::null_mut::<ListenSocket>();
    let mut sock: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 128] = [0; 128];
    let mut errpos = ::core::ptr::null::<::core::ffi::c_char>();
    let mut _log_ctx = NULL;
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            _log_ctx,
            c"add_listen: %s".as_ptr(),
            sa2str(
                sa,
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
            ),
        );
    }
    errpos = c"socket".as_ptr();
    sock = socket(af, SOCK_STREAM, 0 as ::core::ffi::c_int);
    if sock >= 0 as ::core::ffi::c_int {
        if af != AF_UNIX {
            let mut val = 1 as ::core::ffi::c_int;
            errpos = c"setsockopt".as_ptr();
            res = setsockopt(
                sock,
                SOL_SOCKET,
                SO_REUSEADDR,
                &raw mut val as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
            );
            if res < 0 as ::core::ffi::c_int {
                current_block = 5649001695661142926;
            } else {
                current_block = 2968425633554183086;
            }
        } else {
            current_block = 2968425633554183086;
        }
        match current_block {
            5649001695661142926 => {}
            _ => {
                if af == AF_INET6 {
                    let mut val_0 = 1 as ::core::ffi::c_int;
                    errpos = c"setsockopt/IPV6_V6ONLY".as_ptr();
                    res = setsockopt(
                        sock,
                        IPPROTO_IPV6,
                        IPV6_V6ONLY,
                        &raw mut val_0 as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
                    );
                    if res < 0 as ::core::ffi::c_int {
                        current_block = 5649001695661142926;
                    } else {
                        current_block = 17407779659766490442;
                    }
                } else {
                    current_block = 17407779659766490442;
                }
                match current_block {
                    5649001695661142926 => {}
                    _ => {
                        if af != AF_UNIX && cf_so_reuseport != 0 {
                            let mut val_1 = 1 as ::core::ffi::c_int;
                            errpos = c"setsockopt/SO_REUSEPORT".as_ptr();
                            res = setsockopt(
                                sock,
                                SOL_SOCKET,
                                SO_REUSEPORT,
                                &raw mut val_1 as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
                            );
                            if res < 0 as ::core::ffi::c_int {
                                current_block = 5649001695661142926;
                            } else {
                                current_block = 12124785117276362961;
                            }
                        } else {
                            current_block = 12124785117276362961;
                        }
                        match current_block {
                            5649001695661142926 => {}
                            _ => {
                                errpos = c"bind".as_ptr();
                                res = bind(sock, sa, salen as socklen_t);
                                if res >= 0 as ::core::ffi::c_int {
                                    errpos = c"tune_socket".as_ptr();
                                    if tune_socket(sock, af == AF_UNIX) {
                                        errpos = c"listen".as_ptr();
                                        res = listen(sock, cf_listen_backlog);
                                        if res >= 0 as ::core::ffi::c_int {
                                            errpos = c"calloc".as_ptr();
                                            ls = calloc(
                                                1 as size_t,
                                                ::core::mem::size_of::<ListenSocket>() as size_t,
                                            )
                                                as *mut ListenSocket;
                                            if !ls.is_null() {
                                                list_init(&raw mut (*ls).node);
                                                (*ls).fd = sock;
                                                if (*sa).sa_family as ::core::ffi::c_int == AF_UNIX
                                                {
                                                    pga_set(
                                                        &raw mut (*ls).addr,
                                                        AF_UNIX,
                                                        cf_listen_port,
                                                    );
                                                } else {
                                                    pga_copy(&raw mut (*ls).addr, sa);
                                                }
                                                if af == AF_UNIX {
                                                    if *cf_unix_socket_dir
                                                        
                                                        as ::core::ffi::c_int
                                                        != '@' as i32
                                                    {
                                                        let mut un = sa as *mut sockaddr_un;
                                                        change_file_mode(
                                                            &raw mut (*un).sun_path
                                                                as *mut ::core::ffi::c_char,
                                                            cf_unix_socket_mode as mode_t,
                                                            ::core::ptr::null::<::core::ffi::c_char>(
                                                            ),
                                                            cf_unix_socket_group,
                                                        );
                                                    }
                                                } else {
                                                    tune_accept(sock, cf_tcp_defer_accept != 0);
                                                }
                                                let mut _log_ctx_0 = NULL;
                                                log_generic(
                                                    LG_INFO,
                                                    _log_ctx_0,
                                                    c"listening on %s".as_ptr(),
                                                    sa2str(
                                                        sa,
                                                        &raw mut buf as *mut ::core::ffi::c_char,
                                                        ::core::mem::size_of::<
                                                            [::core::ffi::c_char; 128],
                                                        >(
                                                        )
                                                            as size_t,
                                                    ),
                                                );
                                                statlist_append(
                                                    &raw mut sock_list,
                                                    &raw mut (*ls).node,
                                                );
                                                return true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let mut _log_ctx_1 = NULL;
    log_generic(
        LG_WARNING,
        _log_ctx_1,
        c"cannot listen on %s: %s(): %s".as_ptr(),
        sa2str(
            sa,
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
        ),
        errpos,
        strerror(*__error()),
    );
    if sock >= 0 as ::core::ffi::c_int {
        safe_close(sock);
    }
    false
}

unsafe extern "C" fn create_unix_socket(
    mut socket_dir: *const ::core::ffi::c_char,
    mut listen_port: ::core::ffi::c_int,
) {
    let mut un = sockaddr_un {
        sun_len: 0,
        sun_family: 0,
        sun_path: [0; 104],
    };
    let mut addrlen: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 0;
    let mut lockfile: [::core::ffi::c_char; 116] = [0; 116];
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
    memset(
        &raw mut un as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_un>() as size_t,
    );
    un.sun_family = AF_UNIX as sa_family_t;
    snprintf(
        &raw mut un.sun_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 104]>() as size_t,
        c"%s/.s.PGSQL.%d".as_ptr(),
        socket_dir,
        listen_port,
    );
    if *socket_dir as ::core::ffi::c_int == '@' as i32 {
        addrlen = (2 as size_t)
            .wrapping_add(strlen(&raw mut un.sun_path as *mut ::core::ffi::c_char))
            as ::core::ffi::c_int;
        un.sun_path[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    } else {
        addrlen = ::core::mem::size_of::<sockaddr_un>() as ::core::ffi::c_int;
    }
    if *socket_dir as ::core::ffi::c_int != '@' as i32 {
        snprintf(
            &raw mut lockfile as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 116]>() as size_t,
            c"%s.lock".as_ptr(),
            &raw mut un.sun_path as *mut ::core::ffi::c_char,
        );
        res = lstat(&raw mut lockfile as *mut ::core::ffi::c_char, &raw mut st);
        if res == 0 as ::core::ffi::c_int {
            let mut _log_ctx = NULL;
            log_generic(
                LG_FATAL,
                _log_ctx,
                c"unix port %d is in use".as_ptr(),
                listen_port,
            );
            exit(1 as ::core::ffi::c_int);
        }
        unlink(&raw mut un.sun_path as *mut ::core::ffi::c_char);
    }
    if !add_listen(AF_UNIX, &raw mut un as *const sockaddr, addrlen) {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_0,
            c"failed to create unix socket".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
}

unsafe extern "C" fn tune_accept(mut _sock: ::core::ffi::c_int, mut on: bool) {
    let mut act = if on {
        c"install".as_ptr()
    } else {
        c"uninstall".as_ptr()
    };
    let mut res = 0 as ::core::ffi::c_int;
    if on {
        *__error() = EINVAL;
        res = -(1 as ::core::ffi::c_int);
    }
    if res < 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            c"tune_accept: %s TCP_DEFER_ACCEPT: %s".as_ptr(),
            act,
            strerror(*__error()),
        );
    }
}
#[no_mangle]

pub unsafe extern "C" fn pooler_tune_accept(mut on: bool) {
    let mut el = ::core::ptr::null_mut::<List>();
    let mut ls = ::core::ptr::null_mut::<ListenSocket>();
    el = sock_list.head.next;
    while el != &raw mut sock_list.head {
        ls = (el as *mut ::core::ffi::c_char)
            as *mut ListenSocket;
        if !pga_is_unix(&raw mut (*ls).addr) {
            tune_accept((*ls).fd, on);
        }
        el = (*el).next;
    }
}

unsafe extern "C" fn err_wait_func(
    mut _sock: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut _arg: *mut ::core::ffi::c_void,
) {
    if cf_pause_mode != P_SUSPEND as ::core::ffi::c_int {
        resume_pooler();
    }
}

unsafe extern "C" fn addrpair(
    mut src: *const PgAddr,
    mut dst: *const PgAddr,
) -> *const ::core::ffi::c_char {
    static mut ip1buf: [::core::ffi::c_char; 56] = [0; 56];
    static mut ip2buf: [::core::ffi::c_char; 56] = [0; 56];
    static mut buf: [::core::ffi::c_char; 128] = [0; 128];
    let mut ip1 = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ip2 = ::core::ptr::null::<::core::ffi::c_char>();
    if pga_is_unix(src) {
        return c"unix->unix".as_ptr();
    }
    ip1 = pga_ntop(
        src,
        &raw mut ip1buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 56]>() as ::core::ffi::c_int,
    );
    ip2 = pga_ntop(
        dst,
        &raw mut ip2buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 56]>() as ::core::ffi::c_int,
    );
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
        c"%s:%d -> %s:%d".as_ptr(),
        ip1,
        pga_port(src),
        ip2,
        pga_port(dst),
    );
    &raw mut buf as *mut ::core::ffi::c_char
}

unsafe extern "C" fn conninfo(mut sk: *const PgSocket) -> *const ::core::ffi::c_char {
    if (*sk).state() as ::core::ffi::c_int >= SV_FREE as ::core::ffi::c_int {
        addrpair(&raw const (*sk).local_addr, &raw const (*sk).remote_addr)
    } else {
        addrpair(&raw const (*sk).remote_addr, &raw const (*sk).local_addr)
    }
}

unsafe extern "C" fn pool_accept(
    mut sock: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_short,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut ls = arg as *mut ListenSocket;
    let mut fd: ::core::ffi::c_int = 0;
    let mut client = ::core::ptr::null_mut::<PgSocket>();
    let mut raddr = C2RustUnnamed_10 {
        in_0: sockaddr_in {
            sin_len: 0,
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        },
    };
    let mut len: socklen_t = ::core::mem::size_of::<C2RustUnnamed_10>() as socklen_t;
    let mut is_unix = pga_is_unix(&raw mut (*ls).addr);
    if flags as ::core::ffi::c_int & EV_READ == 0 {
        let mut _log_ctx = NULL;
        log_generic(LG_WARNING, _log_ctx, c"no EV_READ in pool_accept".as_ptr());
        return;
    }
    loop {
        fd = safe_accept(sock, &raw mut raddr.sa, &raw mut len);
        if fd < 0 as ::core::ffi::c_int {
            if *__error() == EAGAIN {
                return;
            } else if *__error() == ECONNABORTED {
                return;
            }
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_0,
                c"accept() failed: %s".as_ptr(),
                strerror(*__error()),
            );
            event_assign(
                &raw mut ev_err,
                pgb_event_base,
                -(1 as ::core::ffi::c_int),
                0 as ::core::ffi::c_short,
                Some(
                    err_wait_func
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            ::core::ffi::c_short,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
            safe_evtimer_add(&raw mut ev_err, &raw mut err_timeout);
            suspend_pooler();
            return;
        }
        let mut _log_ctx_1 = NULL;
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(LG_NOISE, _log_ctx_1, c"new fd from accept=%d".as_ptr(), fd);
        }
        if is_unix {
            client = accept_client(fd, true);
        } else {
            client = accept_client(fd, false);
        }
        if !client.is_null()
            && cf_verbose > 0 as ::core::ffi::c_int
        {
            log_generic(
                LG_DEBUG,
                client as *mut ::core::ffi::c_void,
                c"P: got connection: %s".as_ptr(),
                conninfo(client),
            );
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn use_pooler_socket(
    mut sock: ::core::ffi::c_int,
    mut is_unix: bool,
) -> bool {
    let mut ls = ::core::ptr::null_mut::<ListenSocket>();
    let mut res: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 56] = [0; 56];
    if !tune_socket(sock, is_unix) {
        return false;
    }
    ls = calloc(
        1 as size_t,
        ::core::mem::size_of::<ListenSocket>() as size_t,
    ) as *mut ListenSocket;
    if ls.is_null() {
        return false;
    }
    (*ls).fd = sock;
    if is_unix {
        pga_set(&raw mut (*ls).addr, AF_UNIX, cf_listen_port);
    } else {
        let mut ss = sockaddr_storage {
            ss_len: 0,
            ss_family: 0,
            __ss_pad1: [0; 6],
            __ss_align: 0,
            __ss_pad2: [0; 112],
        };
        let mut len: socklen_t = ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
        res = getsockname(sock, &raw mut ss as *mut sockaddr, &raw mut len);
        if res < 0 as ::core::ffi::c_int {
            let mut _log_ctx = NULL;
            log_generic(LG_ERROR, _log_ctx, c"getsockname failed".as_ptr());
            free(ls as *mut ::core::ffi::c_void);
            return false;
        }
        pga_copy(&raw mut (*ls).addr, &raw mut ss as *mut sockaddr);
    }
    let mut _log_ctx_0 = NULL;
    log_generic(
        LG_INFO,
        _log_ctx_0,
        c"got pooler socket: %s".as_ptr(),
        pga_str(
            &raw mut (*ls).addr,
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 56]>() as ::core::ffi::c_int,
        ),
    );
    statlist_append(&raw mut sock_list, &raw mut (*ls).node);
    true
}
#[no_mangle]

pub unsafe extern "C" fn suspend_pooler() {
    let mut el = ::core::ptr::null_mut::<List>();
    let mut ls = ::core::ptr::null_mut::<ListenSocket>();
    need_active = false;
    el = sock_list.head.next;
    while el != &raw mut sock_list.head {
        ls = (el as *mut ::core::ffi::c_char)
            as *mut ListenSocket;
        if (*ls).active {
            if event_del(&raw mut (*ls).ev) < 0 as ::core::ffi::c_int {
                let mut _log_ctx = NULL;
                log_generic(
                    LG_WARNING,
                    _log_ctx,
                    c"suspend_pooler, event_del: %s".as_ptr(),
                    strerror(*__error()),
                );
                return;
            }
            (*ls).active = false;
        }
        el = (*el).next;
    }
    pooler_active = false;
}
#[no_mangle]

pub unsafe extern "C" fn resume_pooler() {
    let mut el = ::core::ptr::null_mut::<List>();
    let mut ls = ::core::ptr::null_mut::<ListenSocket>();
    need_active = true;
    el = sock_list.head.next;
    while el != &raw mut sock_list.head {
        ls = (el as *mut ::core::ffi::c_char)
            as *mut ListenSocket;
        if !(*ls).active {
            event_assign(
                &raw mut (*ls).ev,
                pgb_event_base,
                (*ls).fd,
                (EV_READ | EV_PERSIST) as ::core::ffi::c_short,
                Some(
                    pool_accept
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            ::core::ffi::c_short,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                ),
                ls as *mut ::core::ffi::c_void,
            );
            if event_add(&raw mut (*ls).ev, ::core::ptr::null::<timeval>())
                < 0 as ::core::ffi::c_int
            {
                let mut _log_ctx = NULL;
                log_generic(
                    LG_WARNING,
                    _log_ctx,
                    c"event_add failed: %s".as_ptr(),
                    strerror(*__error()),
                );
                return;
            }
            (*ls).active = true;
        }
        el = (*el).next;
    }
    pooler_active = true;
}
#[no_mangle]

pub unsafe extern "C" fn per_loop_pooler_maint() {
    if need_active && !pooler_active {
        resume_pooler();
    } else if !need_active && pooler_active {
        suspend_pooler();
    }
}

unsafe extern "C" fn parse_addr(
    mut _arg: *mut ::core::ffi::c_void,
    mut addr: *const ::core::ffi::c_char,
) -> bool {
    let mut res: ::core::ffi::c_int = 0;
    let mut service: [::core::ffi::c_char; 64] = [0; 64];
    let mut ai = ::core::ptr::null_mut::<addrinfo>();
    let mut gaires = ::core::ptr::null_mut::<addrinfo>();
    if *addr == 0 {
        return true;
    }
    listen_addr_empty = false;
    if strcmp(addr, c"*".as_ptr()) == 0 as ::core::ffi::c_int {
        addr = ::core::ptr::null::<::core::ffi::c_char>();
    }
    snprintf(
        &raw mut service as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
        c"%d".as_ptr(),
        cf_listen_port,
    );
    res = getaddrinfo(
        addr,
        &raw mut service as *mut ::core::ffi::c_char,
        &raw const hints,
        &raw mut gaires,
    );
    if res != 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx,
            c"getaddrinfo('%s', '%d') = %s [%d]".as_ptr(),
            if !addr.is_null() { addr } else { c"*".as_ptr() },
            cf_listen_port,
            gai_strerror(res),
            res,
        );
        exit(1 as ::core::ffi::c_int);
    }
    ai = gaires;
    while !ai.is_null() {
        add_listen(
            (*ai).ai_family,
            (*ai).ai_addr,
            (*ai).ai_addrlen as ::core::ffi::c_int,
        );
        ai = (*ai).ai_next;
    }
    freeaddrinfo(gaires);
    true
}
#[no_mangle]

pub unsafe extern "C" fn pooler_setup() {
    let mut n: ::core::ffi::c_int = 0;
    n = 0 as ::core::ffi::c_int;
    if n > 0 as ::core::ffi::c_int {
        if !cf_listen_addr.is_null() && *cf_listen_addr as ::core::ffi::c_int != 0 {
            let mut _log_ctx = NULL;
            log_generic(
                LG_WARNING,
                _log_ctx,
                c"sockets passed from service manager, cf_listen_addr ignored".as_ptr(),
            );
        }
        if !cf_unix_socket_dir.is_null()
            && *cf_unix_socket_dir as ::core::ffi::c_int != 0
            && strcmp(cf_unix_socket_dir, DEFAULT_UNIX_SOCKET_DIR.as_ptr())
                != 0 as ::core::ffi::c_int
        {
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_WARNING,
                _log_ctx_0,
                c"sockets passed from service manager, cf_unix_socket_dir ignored".as_ptr(),
            );
        }
        let mut i = 0 as ::core::ffi::c_int;
        while i < n {
            let mut fd = SD_LISTEN_FDS_START + i;
            let mut ls = ::core::ptr::null_mut::<ListenSocket>();
            let mut ok = true;
            ls = calloc(
                1 as size_t,
                ::core::mem::size_of::<ListenSocket>() as size_t,
            ) as *mut ListenSocket;
            if ls.is_null() {
                let mut _log_ctx_1 = NULL;
                log_generic(LG_FATAL, _log_ctx_1, c"out of memory".as_ptr());
                exit(1 as ::core::ffi::c_int);
            }
            list_init(&raw mut (*ls).node);
            (*ls).fd = fd;
            if !ok {
                let mut _log_ctx_2 = NULL;
                log_generic(
                    LG_FATAL,
                    _log_ctx_2,
                    c"failed to set up socket passed from service manager (fd %d)".as_ptr(),
                    fd,
                );
                exit(1 as ::core::ffi::c_int);
            }
            let mut _log_ctx_3 = NULL;
            log_generic(
                LG_INFO,
                _log_ctx_3,
                c"socket passed from service manager (fd %d)".as_ptr(),
                fd,
            );
            statlist_append(&raw mut sock_list, &raw mut (*ls).node);
            i += 1;
        }
    } else {
        let mut ok_0: bool = false;
        static mut init_done: bool = false;
        if !init_done {
            atexit(Some(cleanup_tcp_sockets as unsafe extern "C" fn() -> ()));
            atexit(Some(cleanup_unix_sockets as unsafe extern "C" fn() -> ()));
            init_done = true;
        }
        ok_0 = parse_word_list(
            cf_listen_addr,
            Some(
                parse_addr
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                    ) -> bool,
            ),
            NULL,
        );
        if !ok_0 {
            let mut _log_ctx_4 = NULL;
            log_generic(
                LG_FATAL,
                _log_ctx_4,
                c"failed to parse listen_addr list: %s".as_ptr(),
                cf_listen_addr,
            );
            exit(1 as ::core::ffi::c_int);
        }
        if !listen_addr_empty && statlist_count(&raw mut sock_list) == 0 {
            let mut _log_ctx_5 = NULL;
            log_generic(
                LG_FATAL,
                _log_ctx_5,
                c"failed to listen on any address in listen_addr list: %s".as_ptr(),
                cf_listen_addr,
            );
            exit(1 as ::core::ffi::c_int);
        }
        if !cf_unix_socket_dir.is_null() && *cf_unix_socket_dir as ::core::ffi::c_int != 0 {
            create_unix_socket(cf_unix_socket_dir, cf_listen_port);
        }
    }
    if statlist_count(&raw mut sock_list) == 0 {
        let mut _log_ctx_6 = NULL;
        log_generic(LG_FATAL, _log_ctx_6, c"nowhere to listen on".as_ptr());
        exit(1 as ::core::ffi::c_int);
    }
    resume_pooler();
}
#[no_mangle]

pub unsafe extern "C" fn for_each_pooler_fd(
    mut cbfunc: pooler_cb,
    mut arg: *mut ::core::ffi::c_void,
) -> bool {
    let mut el = ::core::ptr::null_mut::<List>();
    let mut ls = ::core::ptr::null_mut::<ListenSocket>();
    let mut ok: bool = false;
    el = sock_list.head.next;
    while el != &raw mut sock_list.head {
        ls = (el as *mut ::core::ffi::c_char)
            as *mut ListenSocket;
        ok = cbfunc.expect("non-null function pointer")(arg, (*ls).fd, &raw mut (*ls).addr);
        if !ok {
            return false;
        }
        el = (*el).next;
    }
    true
}
unsafe extern "C" fn run_static_initializers() {
    sock_list = StatList {
        head: List {
            next: &raw mut sock_list.head,
            prev: &raw mut sock_list.head,
        },
        cur_count: 0 as ::core::ffi::c_int,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
