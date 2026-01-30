
pub mod _types_h {
    
    pub type __uint8_t = u8;
    
    pub type __int32_t = i32;
    
    pub type __uint32_t = u32;
    
    pub type __darwin_size_t = usize;
    
    pub type __darwin_socklen_t = __uint32_t;
    
    pub type __darwin_time_t = ::core::ffi::c_long;
}

pub mod _int32_t_h {
    
    pub type int32_t = i32;
}

pub mod _uintptr_t_h {
    
    pub type uintptr_t = usize;
}

pub mod sys__types_h {
    
    pub type __darwin_suseconds_t = __int32_t;
    
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    use super::_types_h::__int32_t;
}





pub mod _timeval_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct timeval {
        pub tv_sec: __darwin_time_t,
        pub tv_usec: __darwin_suseconds_t,
    }
    use super::_types_h::__darwin_time_t;
    use super::sys__types_h::__darwin_suseconds_t;
}





pub mod _sa_family_t_h {
    
    pub type sa_family_t = __uint8_t;
    use super::_types_h::__uint8_t;
}

pub mod _socklen_t_h {
    
    pub type socklen_t = __darwin_socklen_t;
    use super::_types_h::__darwin_socklen_t;
}

pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sockaddr {
        pub sa_len: __uint8_t,
        pub sa_family: sa_family_t,
        pub sa_data: [::core::ffi::c_char; 14],
    }
    
    pub const SOCK_STREAM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    use super::_sa_family_t_h::sa_family_t;
    use super::_types_h::__uint8_t;
}

pub mod netdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct addrinfo {
        pub ai_flags: ::core::ffi::c_int,
        pub ai_family: ::core::ffi::c_int,
        pub ai_socktype: ::core::ffi::c_int,
        pub ai_protocol: ::core::ffi::c_int,
        pub ai_addrlen: socklen_t,
        pub ai_canonname: *mut ::core::ffi::c_char,
        pub ai_addr: *mut sockaddr,
        pub ai_next: *mut addrinfo,
    }
    use super::_socklen_t_h::socklen_t;
    use super::socket_h::sockaddr;
}

pub mod event_h {
    
    pub type event_callback_fn = Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_short,
            *mut ::core::ffi::c_void,
        ) -> (),
    >;
    use super::event_struct_h::event;
    extern "C" {
        
        pub type event_base;
        
        pub fn event_assign(
            _: *mut event,
            _: *mut event_base,
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_short,
            _: event_callback_fn,
            _: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
        
        pub fn event_del(_: *mut event) -> ::core::ffi::c_int;
    }
}

pub mod event_struct_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct event {
        pub ev_evcallback: event_callback,
        pub ev_timeout_pos: C2RustUnnamed_4,
        pub ev_fd: ::core::ffi::c_int,
        pub ev_base: *mut event_base,
        pub ev_: C2RustUnnamed,
        pub ev_events: ::core::ffi::c_short,
        pub ev_res: ::core::ffi::c_short,
        pub ev_timeout: timeval,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union C2RustUnnamed {
        pub ev_io: C2RustUnnamed_2,
        pub ev_signal: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct C2RustUnnamed_0 {
        pub ev_signal_next: C2RustUnnamed_1,
        pub ev_ncalls: ::core::ffi::c_short,
        pub ev_pncalls: *mut ::core::ffi::c_short,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct C2RustUnnamed_1 {
        pub le_next: *mut event,
        pub le_prev: *mut *mut event,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct C2RustUnnamed_2 {
        pub ev_io_next: C2RustUnnamed_3,
        pub ev_timeout: timeval,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct C2RustUnnamed_3 {
        pub le_next: *mut event,
        pub le_prev: *mut *mut event,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union C2RustUnnamed_4 {
        pub ev_next_with_common_timeout: C2RustUnnamed_5,
        pub min_heap_idx: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct C2RustUnnamed_5 {
        pub tqe_next: *mut event,
        pub tqe_prev: *mut *mut event,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct event_callback {
        pub evcb_active_next: C2RustUnnamed_7,
        pub evcb_flags: ::core::ffi::c_short,
        pub evcb_pri: uint8_t,
        pub evcb_closure: uint8_t,
        pub evcb_cb_union: C2RustUnnamed_6,
        pub evcb_arg: *mut ::core::ffi::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union C2RustUnnamed_6 {
        pub evcb_callback: Option<
            unsafe extern "C" fn(
                ::core::ffi::c_int,
                ::core::ffi::c_short,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
        pub evcb_selfcb:
            Option<unsafe extern "C" fn(*mut event_callback, *mut ::core::ffi::c_void) -> ()>,
        pub evcb_evfinalize:
            Option<unsafe extern "C" fn(*mut event, *mut ::core::ffi::c_void) -> ()>,
        pub evcb_cbfinalize:
            Option<unsafe extern "C" fn(*mut event_callback, *mut ::core::ffi::c_void) -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct C2RustUnnamed_7 {
        pub tqe_next: *mut event_callback,
        pub tqe_prev: *mut *mut event_callback,
    }
    use super::_timeval_h::timeval;
    use crate::types::uint8_t;
    use super::event_h::event_base;
}

pub mod dnslookup_h {
    
    pub type adns_callback_f = Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *const sockaddr, ::core::ffi::c_int) -> (),
    >;
    
    pub type adns_walk_name_f = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            *const addrinfo,
            usec_t,
        ) -> (),
    >;
    
    pub type adns_walk_zone_f = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            uint32_t,
            ::core::ffi::c_int,
        ) -> (),
    >;
    use crate::types::uint32_t;
    use super::netdb_h::addrinfo;
    use super::socket_h::sockaddr;
    use crate::types::usec_t;
}

pub mod logging_h {
    
    pub type LogLevel = ::core::ffi::c_uint;
    
    pub const LG_NOISE: LogLevel = 6;
    
    pub const LG_DEBUG: LogLevel = 5;
    
    pub const LG_INFO: LogLevel = 4;
    
    pub const LG_STATS: LogLevel = 3;
    
    pub const LG_WARNING: LogLevel = 2;
    
    pub const LG_ERROR: LogLevel = 1;
    
    pub const LG_FATAL: LogLevel = 0;
    extern "C" {
        
        pub static mut cf_verbose: ::core::ffi::c_int;
        
        pub fn log_generic(
            level: LogLevel,
            ctx: *mut ::core::ffi::c_void,
            s: *const ::core::ffi::c_char,
            ...
        );
    }
}

pub mod dns_h {
    
    pub type evdns_getaddrinfo_cb = Option<
        unsafe extern "C" fn(::core::ffi::c_int, *mut addrinfo, *mut ::core::ffi::c_void) -> (),
    >;
    
    pub const DNS_OPTION_SEARCH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    pub const DNS_OPTION_NAMESERVERS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    pub const DNS_OPTION_MISC: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    pub const DNS_OPTION_HOSTSFILE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    
    pub const DNS_OPTIONS_ALL: ::core::ffi::c_int = DNS_OPTION_SEARCH
        | DNS_OPTION_NAMESERVERS
        | DNS_OPTION_MISC
        | DNS_OPTION_HOSTSFILE
        | 0 as ::core::ffi::c_int;
    use super::event_h::event_base;
    use super::netdb_h::addrinfo;
    extern "C" {
        
        pub type evdns_base;
        
        pub type evdns_getaddrinfo_request;
        
        pub fn evdns_base_new(
            event_base: *mut event_base,
            initialize_nameservers: ::core::ffi::c_int,
        ) -> *mut evdns_base;
        
        pub fn evdns_base_free(base: *mut evdns_base, fail_requests: ::core::ffi::c_int);
        
        pub fn evdns_base_resolv_conf_parse(
            base: *mut evdns_base,
            flags: ::core::ffi::c_int,
            filename: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        
        pub fn evdns_getaddrinfo(
            dns_base: *mut evdns_base,
            nodename: *const ::core::ffi::c_char,
            servname: *const ::core::ffi::c_char,
            hints_in: *const addrinfo,
            cb: evdns_getaddrinfo_cb,
            arg: *mut ::core::ffi::c_void,
        ) -> *mut evdns_getaddrinfo_request;
    }
}

pub mod _malloc_h {
    use crate::types::size_t;
    extern "C" {
        
        pub fn calloc(__count: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
        
        pub fn free(_: *mut ::core::ffi::c_void);
    }
}

pub mod _null_h {
    
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}

pub mod usual_socket_h {
    use crate::types::size_t;
    use super::socket_h::sockaddr;
    extern "C" {
        
        pub fn sa2str(
            sa: *const sockaddr,
            buf: *mut ::core::ffi::c_char,
            buflen: size_t,
        ) -> *const ::core::ffi::c_char;
    }
}

pub mod util_h {
    use super::netdb_h::addrinfo;
    extern "C" {
        
        pub fn evutil_freeaddrinfo(ai: *mut addrinfo);
    }
}

pub mod _string_h {
    use crate::types::size_t;
    extern "C" {
        
        pub fn memcmp(
            __s1: *const ::core::ffi::c_void,
            __s2: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> ::core::ffi::c_int;
        
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
        
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
        
        pub fn strdup(__s1: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    }
}

pub mod bouncer_h {
    use super::event_h::event_base;
    use crate::types::usec_t;
    extern "C" {
        
        pub static mut pgb_event_base: *mut event_base;
        
        pub static mut cf_dns_max_ttl: usec_t;
        
        pub static mut cf_dns_nxdomain_ttl: usec_t;
        
        pub static mut cf_resolv_conf: *mut ::core::ffi::c_char;
    }
}

pub mod include_util_h {
    use super::_timeval_h::timeval;
    use super::event_struct_h::event;
    extern "C" {
        
        pub fn safe_evtimer_add(ev: *mut event, tv: *mut timeval);
    }
}

pub mod objects_h {
    use super::socket_h::sockaddr;
    extern "C" {
        
        pub fn tag_host_addr_dirty(host: *const ::core::ffi::c_char, sa: *const sockaddr);
    }
}


pub mod _strings_h {
    extern "C" {
        
        pub fn strcasecmp(
            _: *const ::core::ffi::c_char,
            _: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}
pub use self::_int32_t_h::int32_t;
use self::_malloc_h::{calloc, free};
pub use self::_null_h::NULL;
pub use self::_sa_family_t_h::sa_family_t;
pub use crate::types::size_t;
pub use self::_socklen_t_h::socklen_t;
use self::_string_h::{memcmp, memset, strchr, strcmp, strdup, strlen};
use self::_strings_h::strcasecmp;
pub use self::_timeval_h::timeval;
pub use self::_types_h::{
    __darwin_size_t, __darwin_socklen_t, __darwin_time_t, __int32_t, __uint32_t, __uint8_t,
};
pub use crate::types::uint32_t;
pub use crate::types::uint64_t;
pub use crate::types::uint8_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use crate::types::{aatree_cmp_f, aatree_destroy, aatree_init, aatree_insert, aatree_search, aatree_walk, aatree_walker_f, AANode, AATree, AATreeWalkType, AA_WALK_IN_ORDER, AA_WALK_POST_ORDER, AA_WALK_PRE_ORDER};
use self::bouncer_h::{cf_dns_max_ttl, cf_dns_nxdomain_ttl, cf_resolv_conf, pgb_event_base};
pub use self::dns_h::{
    evdns_base, evdns_base_free, evdns_base_new, evdns_base_resolv_conf_parse, evdns_getaddrinfo,
    evdns_getaddrinfo_cb, evdns_getaddrinfo_request, DNS_OPTIONS_ALL, DNS_OPTION_HOSTSFILE,
    DNS_OPTION_MISC, DNS_OPTION_NAMESERVERS, DNS_OPTION_SEARCH,
};
pub use self::dnslookup_h::{adns_callback_f, adns_walk_name_f, adns_walk_zone_f};
pub use self::event_h::{event_assign, event_base, event_callback_fn, event_del};
pub use self::event_struct_h::{
    event, event_callback, C2RustUnnamed, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2,
    C2RustUnnamed_3, C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7,
};
use self::include_util_h::safe_evtimer_add;
pub use crate::types::{list_append, list_del, list_empty, list_first, list_init, list_pop, List};
pub use self::logging_h::{
    cf_verbose, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE, LG_STATS,
    LG_WARNING,
};
pub use self::netdb_h::addrinfo;
use self::objects_h::tag_host_addr_dirty;
pub use self::socket_h::{sockaddr, SOCK_STREAM};
pub use crate::types::{statlist_append, statlist_count, statlist_init, statlist_remove, StatList};
pub use crate::types::{false_0, true_0};
pub use self::sys__types_h::{__darwin_suseconds_t, __DARWIN_NULL};
pub use crate::types::{usec_t, USEC};
use self::usual_socket_h::sa2str;
use self::util_h::evutil_freeaddrinfo;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct DNSToken {
    pub node: List,
    pub cb_func: adns_callback_f,
    pub cb_arg: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct DNSContext {
    pub req_tree: AATree,
    pub edns: *mut ::core::ffi::c_void,
    pub zone_tree: AATree,
    pub zone_list: List,
    pub cur_zone: *mut DNSZone,
    pub ev_zone_timer: event,
    pub zone_state: ::core::ffi::c_int,
    pub active: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct DNSZone {
    pub lnode: List,
    pub tnode: AANode,
    pub host_list: StatList,
    pub zonename: *mut ::core::ffi::c_char,
    pub serial: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct DNSRequest {
    pub node: AANode,
    pub znode: List,
    pub ctx: *mut DNSContext,
    pub zone: *mut DNSZone,
    pub ucb_list: List,
    pub name: *mut ::core::ffi::c_char,
    pub namelen: ::core::ffi::c_int,
    pub done: bool,
    pub result: *mut addrinfo,
    pub current: *mut addrinfo,
    pub oldres: *mut addrinfo,
    pub res_ttl: usec_t,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct WalkInfo {
    pub name_cb: adns_walk_name_f,
    pub zone_cb: adns_walk_zone_f,
    pub arg: *mut ::core::ffi::c_void,
}

pub const cf_dns_zone_check_period: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]

pub unsafe extern "C" fn adns_get_backend() -> *const ::core::ffi::c_char {
    b"evdns2\0" as *const u8 as *const ::core::ffi::c_char
}

unsafe extern "C" fn _evdns_base_resolv_conf_parse_err_to_string(
    mut err: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    match err {
        0 => b"no error\0" as *const u8 as *const ::core::ffi::c_char,
        1 => b"failed to open file\0" as *const u8 as *const ::core::ffi::c_char,
        2 => b"failed to stat file\0" as *const u8 as *const ::core::ffi::c_char,
        3 => b"file too large\0" as *const u8 as *const ::core::ffi::c_char,
        4 => b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
        5 => b"short read from file\0" as *const u8 as *const ::core::ffi::c_char,
        6 => b"no nameservers listed in the file\0" as *const u8 as *const ::core::ffi::c_char,
        _ => b"[Unknown error code]\0" as *const u8 as *const ::core::ffi::c_char,
    }
}

unsafe extern "C" fn impl_init(mut ctx: *mut DNSContext) -> bool {
    if !cf_resolv_conf.is_null()
        && *cf_resolv_conf.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
    {
        let mut err: ::core::ffi::c_int = 0;
        (*ctx).edns =
            evdns_base_new(pgb_event_base, 0 as ::core::ffi::c_int) as *mut ::core::ffi::c_void;
        if (*ctx).edns.is_null() {
            let mut _log_ctx = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx,
                b"evdns_base_new failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
        err = evdns_base_resolv_conf_parse(
            (*ctx).edns as *mut evdns_base,
            DNS_OPTIONS_ALL,
            cf_resolv_conf,
        );
        if err != 0 {
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_0,
                b"evdns parsing of \"%s\" failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
                cf_resolv_conf,
                _evdns_base_resolv_conf_parse_err_to_string(err),
            );
            return false_0 != 0;
        }
    } else {
        (*ctx).edns =
            evdns_base_new(pgb_event_base, 1 as ::core::ffi::c_int) as *mut ::core::ffi::c_void;
        if (*ctx).edns.is_null() {
            let mut _log_ctx_1 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_1,
                b"evdns_base_new failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
    }
    true_0 != 0
}

unsafe extern "C" fn impl_launch_query(mut req: *mut DNSRequest) {
    static mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: SOCK_STREAM,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_canonname: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        ai_addr: ::core::ptr::null::<sockaddr>() as *mut sockaddr,
        ai_next: ::core::ptr::null::<addrinfo>() as *mut addrinfo,
    };
    let mut gai_req = ::core::ptr::null_mut::<evdns_getaddrinfo_request>();
    let mut dns = (*(*req).ctx).edns as *mut evdns_base;
    gai_req = evdns_getaddrinfo(
        dns,
        (*req).name,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw const hints,
        Some(
            got_result_gai
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut addrinfo,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        req as *mut ::core::ffi::c_void,
    );
    let mut _log_ctx = NULL;
    if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_NOISE,
            _log_ctx,
            b"dns: evdns_getaddrinfo(%s)=%p\0" as *const u8 as *const ::core::ffi::c_char,
            (*req).name,
            gai_req,
        );
    }
}

unsafe extern "C" fn impl_release(mut ctx: *mut DNSContext) {
    let mut dns = (*ctx).edns as *mut evdns_base;
    evdns_base_free(dns, 0 as ::core::ffi::c_int);
}

unsafe extern "C" fn deliver_info(mut req: *mut DNSRequest) {
    let mut ctx = (*req).ctx;
    let mut ucb = ::core::ptr::null_mut::<DNSToken>();
    let mut el = ::core::ptr::null_mut::<List>();
    let mut ai: *const addrinfo = (*req).current;
    let mut sabuf: [::core::ffi::c_char; 128] = [0; 128];
    (*ctx).active -= 1;
    loop {
        el = list_pop(&raw mut (*req).ucb_list);
        if el.is_null() {
            return;
        }
        ucb = (el as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut DNSToken;
        let mut _log_ctx = NULL;
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                _log_ctx,
                b"dns: deliver_info(%s) addr=%s\0" as *const u8 as *const ::core::ffi::c_char,
                (*req).name,
                if !ai.is_null() {
                    sa2str(
                        (*ai).ai_addr,
                        &raw mut sabuf as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
                    )
                } else {
                    b"NULL\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
        }
        (*ucb).cb_func.expect("non-null function pointer")(
            (*ucb).cb_arg,
            if !ai.is_null() {
                (*ai).ai_addr
            } else {
                ::core::ptr::null_mut::<sockaddr>()
            },
            (if !ai.is_null() {
                (*ai).ai_addrlen
            } else {
                0 as socklen_t
            }) as ::core::ffi::c_int,
        );
        free(ucb as *mut ::core::ffi::c_void);
        if !ai.is_null() {
            (*req).current = (*ai).ai_next;
            if (*req).current.is_null() {
                (*req).current = (*req).result;
            }
        }
    }
}

unsafe extern "C" fn req_cmp(mut arg: uintptr_t, mut node: *mut AANode) -> ::core::ffi::c_int {
    let mut s1: *const ::core::ffi::c_char = arg as *mut ::core::ffi::c_char;
    let mut req = (node as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
        as *mut DNSRequest;
    strcmp(s1, (*req).name)
}

unsafe extern "C" fn req_reset(mut req: *mut DNSRequest) {
    (*req).done = false_0 != 0;
    if !(*req).result.is_null() {
        if !(*req).oldres.is_null() {
            evutil_freeaddrinfo((*req).oldres);
        }
        (*req).oldres = (*req).result;
    }
    (*req).current = ::core::ptr::null_mut::<addrinfo>();
    (*req).result = (*req).current;
}

unsafe extern "C" fn req_free(mut node: *mut AANode, mut _arg: *mut ::core::ffi::c_void) {
    let mut ucb = ::core::ptr::null_mut::<DNSToken>();
    let mut req = ::core::ptr::null_mut::<DNSRequest>();
    let mut el = ::core::ptr::null_mut::<List>();
    req = (node as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
        as *mut DNSRequest;
    loop {
        el = list_pop(&raw mut (*req).ucb_list);
        if el.is_null() {
            break;
        }
        ucb = (el as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut DNSToken;
        free(ucb as *mut ::core::ffi::c_void);
    }
    req_reset(req);
    if !(*req).oldres.is_null() {
        evutil_freeaddrinfo((*req).oldres);
        (*req).oldres = ::core::ptr::null_mut::<addrinfo>();
    }
    if !(*req).zone.is_null() {
        statlist_remove(&raw mut (*(*req).zone).host_list, &raw mut (*req).znode);
    }
    free((*req).name as *mut ::core::ffi::c_void);
    free(req as *mut ::core::ffi::c_void);
}
#[no_mangle]

pub unsafe extern "C" fn adns_create_context() -> *mut DNSContext {
    let mut ctx = ::core::ptr::null_mut::<DNSContext>();
    let mut _log_ctx = NULL;
    if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_DEBUG,
            _log_ctx,
            b"adns_create_context: %s\0" as *const u8 as *const ::core::ffi::c_char,
            adns_get_backend(),
        );
    }
    ctx = calloc(1 as size_t, ::core::mem::size_of::<DNSContext>() as size_t) as *mut DNSContext;
    if ctx.is_null() {
        return ::core::ptr::null_mut::<DNSContext>();
    }
    aatree_init(
        &raw mut (*ctx).req_tree,
        Some(req_cmp as unsafe extern "C" fn(uintptr_t, *mut AANode) -> ::core::ffi::c_int),
        Some(req_free as unsafe extern "C" fn(*mut AANode, *mut ::core::ffi::c_void) -> ()),
    );
    zone_init(ctx);
    if !impl_init(ctx) {
        adns_free_context(ctx);
        return ::core::ptr::null_mut::<DNSContext>();
    }
    ctx
}
#[no_mangle]

pub unsafe extern "C" fn adns_free_context(mut ctx: *mut DNSContext) {
    if !ctx.is_null() {
        impl_release(ctx);
        aatree_destroy(&raw mut (*ctx).req_tree);
        zone_free(ctx);
        free(ctx as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]

pub unsafe extern "C" fn adns_resolve(
    mut ctx: *mut DNSContext,
    mut name: *const ::core::ffi::c_char,
    mut cb_func: adns_callback_f,
    mut cb_arg: *mut ::core::ffi::c_void,
) -> *mut DNSToken {
    let mut current_block: u64;
    let mut namelen = strlen(name) as ::core::ffi::c_int;
    let mut req = ::core::ptr::null_mut::<DNSRequest>();
    let mut ucb = ::core::ptr::null_mut::<DNSToken>();
    let mut node = ::core::ptr::null_mut::<AANode>();
    node = aatree_search(&raw mut (*ctx).req_tree, name as uintptr_t);
    if !node.is_null() {
        req = (node as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut DNSRequest;
        current_block = 6009453772311597924;
    } else {
        let mut _log_ctx = NULL;
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                _log_ctx,
                b"dns: new req: %s\0" as *const u8 as *const ::core::ffi::c_char,
                name,
            );
        }
        req =
            calloc(1 as size_t, ::core::mem::size_of::<DNSRequest>() as size_t) as *mut DNSRequest;
        if req.is_null() {
            current_block = 4318489846687653863;
        } else {
            (*req).name = strdup(name);
            if (*req).name.is_null() {
                free(req as *mut ::core::ffi::c_void);
                current_block = 4318489846687653863;
            } else {
                (*req).ctx = ctx;
                (*req).namelen = namelen;
                list_init(&raw mut (*req).ucb_list);
                list_init(&raw mut (*req).znode);
                aatree_insert(
                    &raw mut (*ctx).req_tree,
                    (*req).name as uintptr_t,
                    &raw mut (*req).node,
                );
                zone_register(ctx, req);
                (*ctx).active += 1;
                impl_launch_query(req);
                current_block = 6009453772311597924;
            }
        }
    }
    if current_block == 6009453772311597924 {
        ucb = calloc(1 as size_t, ::core::mem::size_of::<DNSToken>() as size_t) as *mut DNSToken;
        if !ucb.is_null() {
            list_init(&raw mut (*ucb).node);
            (*ucb).cb_func = cb_func;
            (*ucb).cb_arg = cb_arg;
            list_append(&raw mut (*req).ucb_list, &raw mut (*ucb).node);
            if (*req).done {
                if (*req).res_ttl < get_cached_time() {
                    let mut _log_ctx_0 = NULL;
                    if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                        as ::core::ffi::c_long
                        != 0
                    {
                        log_generic(
                            LG_NOISE,
                            _log_ctx_0,
                            b"dns: ttl over: %s\0" as *const u8 as *const ::core::ffi::c_char,
                            (*req).name,
                        );
                    }
                    req_reset(req);
                    (*ctx).active += 1;
                    impl_launch_query(req);
                } else {
                    deliver_info(req);
                }
            }
            return if (*req).done as ::core::ffi::c_int != 0 {
                ::core::ptr::null_mut::<DNSToken>()
            } else {
                ucb
            };
        }
    }
    let mut _log_ctx_1 = NULL;
    log_generic(
        LG_WARNING,
        _log_ctx_1,
        b"dns(%s): req failed, no mem\0" as *const u8 as *const ::core::ffi::c_char,
        name,
    );
    cb_func.expect("non-null function pointer")(
        cb_arg,
        ::core::ptr::null::<sockaddr>(),
        0 as ::core::ffi::c_int,
    );
    ::core::ptr::null_mut::<DNSToken>()
}

unsafe extern "C" fn cmp_addrinfo(
    mut a1: *const addrinfo,
    mut a2: *const addrinfo,
) -> ::core::ffi::c_int {
    if (*a1).ai_family != (*a2).ai_family {
        return (*a1).ai_family - (*a2).ai_family;
    }
    if (*a1).ai_addrlen != (*a2).ai_addrlen {
        return (*a1).ai_addrlen.wrapping_sub((*a2).ai_addrlen) as ::core::ffi::c_int;
    }
    memcmp(
        (*a1).ai_addr as *const ::core::ffi::c_void,
        (*a2).ai_addr as *const ::core::ffi::c_void,
        (*a1).ai_addrlen as size_t,
    )
}

unsafe extern "C" fn check_req_result_changes(mut req: *mut DNSRequest) {
    let mut ai = ::core::ptr::null_mut::<addrinfo>();
    let mut aj = ::core::ptr::null_mut::<addrinfo>();
    ai = (*req).oldres;
    while !ai.is_null() {
        let mut found = false_0 != 0;
        aj = (*req).result;
        while !aj.is_null() {
            if cmp_addrinfo(ai, aj) == 0 as ::core::ffi::c_int {
                found = true_0 != 0;
                break;
            } else {
                aj = (*aj).ai_next;
            }
        }
        if !found {
            tag_host_addr_dirty((*req).name, (*ai).ai_addr);
        }
        ai = (*ai).ai_next;
    }
}

unsafe extern "C" fn got_result_gai(
    mut result: ::core::ffi::c_int,
    mut res: *mut addrinfo,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut req = arg as *mut DNSRequest;
    req_reset(req);
    if result == 0 as ::core::ffi::c_int && !res.is_null() {
        (*req).result = res;
        (*req).current = res;
        if !(*req).oldres.is_null() {
            check_req_result_changes(req);
        }
        if cf_verbose > 1 as ::core::ffi::c_int {
            let mut ai: *const addrinfo = res;
            let mut n = 0 as ::core::ffi::c_int;
            let mut buf: [::core::ffi::c_char; 128] = [0; 128];
            while !ai.is_null() {
                let mut _log_ctx = NULL;
                if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0
                {
                    let fresh0 = n;
                    n += 1;
                    log_generic(
                        LG_NOISE,
                        _log_ctx,
                        b"DNS: %s[%d] = %s [%s]\0" as *const u8 as *const ::core::ffi::c_char,
                        (*req).name,
                        fresh0,
                        sa2str(
                            (*ai).ai_addr,
                            &raw mut buf as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
                        ),
                        if (*ai).ai_socktype == 1 as ::core::ffi::c_int {
                            b"STREAM\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"OTHER\0" as *const u8 as *const ::core::ffi::c_char
                        },
                    );
                }
                ai = (*ai).ai_next;
            }
        }
        (*req).res_ttl = get_cached_time().wrapping_add(cf_dns_max_ttl);
    } else {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_0,
            b"DNS lookup failed: %s: result=%d\0" as *const u8 as *const ::core::ffi::c_char,
            (*req).name,
            result,
        );
        (*req).res_ttl = get_cached_time().wrapping_add(cf_dns_nxdomain_ttl);
    }
    (*req).done = true_0 != 0;
    deliver_info(req);
}
#[no_mangle]

pub unsafe extern "C" fn adns_cancel(mut _ctx: *mut DNSContext, mut tk: *mut DNSToken) {
    list_del(&raw mut (*tk).node);
    memset(
        tk as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<DNSToken>() as size_t,
    );
    free(tk as *mut ::core::ffi::c_void);
}
#[no_mangle]

pub unsafe extern "C" fn adns_info(
    mut ctx: *mut DNSContext,
    mut names: *mut ::core::ffi::c_int,
    mut zones: *mut ::core::ffi::c_int,
    mut queries: *mut ::core::ffi::c_int,
    mut pending: *mut ::core::ffi::c_int,
) {
    *names = (*ctx).req_tree.count;
    *zones = (*ctx).zone_tree.count;
    *queries = (*ctx).active;
    *pending = 0 as ::core::ffi::c_int;
}

unsafe extern "C" fn zone_item_free(mut n: *mut AANode, mut _arg: *mut ::core::ffi::c_void) {
    let mut z = (n as *mut ::core::ffi::c_char).offset(-(16 as ::core::ffi::c_ulong as isize))
        as *mut DNSZone;
    list_del(&raw mut (*z).lnode);
    free((*z).zonename as *mut ::core::ffi::c_void);
    free(z as *mut ::core::ffi::c_void);
}

unsafe extern "C" fn zone_item_cmp(mut val1: uintptr_t, mut n2: *mut AANode) -> ::core::ffi::c_int {
    let mut name1 = val1 as *const ::core::ffi::c_char;
    let mut z2 = (n2 as *mut ::core::ffi::c_char).offset(-(16 as ::core::ffi::c_ulong as isize))
        as *mut DNSZone;
    strcasecmp(name1, (*z2).zonename)
}

unsafe extern "C" fn zone_init(mut ctx: *mut DNSContext) {
    aatree_init(
        &raw mut (*ctx).zone_tree,
        Some(zone_item_cmp as unsafe extern "C" fn(uintptr_t, *mut AANode) -> ::core::ffi::c_int),
        Some(zone_item_free as unsafe extern "C" fn(*mut AANode, *mut ::core::ffi::c_void) -> ()),
    );
    list_init(&raw mut (*ctx).zone_list);
}

unsafe extern "C" fn zone_free(mut ctx: *mut DNSContext) {
    aatree_destroy(&raw mut (*ctx).zone_tree);
}

unsafe extern "C" fn zone_register(mut ctx: *mut DNSContext, mut req: *mut DNSRequest) {
    let mut z = ::core::ptr::null_mut::<DNSZone>();
    let mut n = ::core::ptr::null_mut::<AANode>();
    let mut name = ::core::ptr::null::<::core::ffi::c_char>();
    let mut _log_ctx = NULL;
    if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_DEBUG,
            _log_ctx,
            b"zone_register(%s)\0" as *const u8 as *const ::core::ffi::c_char,
            (*req).name,
        );
    }
    name = strchr((*req).name, '.' as i32);
    if name.is_null()
        || *name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        return;
    }
    name = name.offset(1);
    let mut _log_ctx_0 = NULL;
    if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_DEBUG,
            _log_ctx_0,
            b"zone_register(%s): name=%s\0" as *const u8 as *const ::core::ffi::c_char,
            (*req).name,
            name,
        );
    }
    n = aatree_search(&raw mut (*ctx).zone_tree, name as uintptr_t);
    if !n.is_null() {
        z = (n as *mut ::core::ffi::c_char).offset(-(16 as ::core::ffi::c_ulong as isize))
            as *mut DNSZone;
        (*req).zone = z as *mut DNSZone;
        statlist_append(&raw mut (*z).host_list, &raw mut (*req).znode);
        return;
    }
    z = calloc(1 as size_t, ::core::mem::size_of::<DNSZone>() as size_t) as *mut DNSZone;
    if z.is_null() {
        return;
    }
    (*z).zonename = strdup(name);
    if (*z).zonename.is_null() {
        free(z as *mut ::core::ffi::c_void);
        return;
    }
    statlist_init(
        &raw mut (*z).host_list,
        b"host_list\0" as *const u8 as *const ::core::ffi::c_char,
    );
    list_init(&raw mut (*z).lnode);
    aatree_insert(
        &raw mut (*ctx).zone_tree,
        (*z).zonename as uintptr_t,
        &raw mut (*z).tnode,
    );
    list_append(&raw mut (*ctx).zone_list, &raw mut (*z).lnode);
    statlist_append(&raw mut (*z).host_list, &raw mut (*req).znode);
    (*req).zone = z as *mut DNSZone;
}

unsafe extern "C" fn zone_timer(
    mut _fd: ::core::ffi::c_int,
    mut _flg: ::core::ffi::c_short,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut ctx = arg as *mut DNSContext;
    let mut el = ::core::ptr::null_mut::<List>();
    let mut z = ::core::ptr::null_mut::<DNSZone>();
    if list_empty(&raw mut (*ctx).zone_list) != 0 {
        (*ctx).zone_state = 0 as ::core::ffi::c_int;
        return;
    }
    el = list_first(&raw mut (*ctx).zone_list);
    z = (el as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
        as *mut DNSZone;
    (*ctx).zone_state = 1 as ::core::ffi::c_int;
    (*ctx).cur_zone = z;
    (*ctx).active += 1;
}

unsafe extern "C" fn launch_zone_timer(mut ctx: *mut DNSContext) {
    let mut tv = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    tv.tv_sec = (cf_dns_zone_check_period as usec_t).wrapping_div(USEC) as __darwin_time_t;
    tv.tv_usec = (cf_dns_zone_check_period as usec_t).wrapping_rem(USEC) as __darwin_suseconds_t;
    event_assign(
        &raw mut (*ctx).ev_zone_timer,
        pgb_event_base,
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_short,
        Some(
            zone_timer
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_short,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        ctx as *mut ::core::ffi::c_void,
    );
    safe_evtimer_add(&raw mut (*ctx).ev_zone_timer, &raw mut tv);
    (*ctx).zone_state = 2 as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn adns_zone_cache_maint(mut ctx: *mut DNSContext) {
    if cf_dns_zone_check_period == 0 {
        if (*ctx).zone_state == 2 as ::core::ffi::c_int {
            event_del(&raw mut (*ctx).ev_zone_timer);
            (*ctx).zone_state = 0 as ::core::ffi::c_int;
        }
        (*ctx).cur_zone = ::core::ptr::null_mut::<DNSZone>();
    } else if (*ctx).zone_state == 0 as ::core::ffi::c_int {
        if list_empty(&raw mut (*ctx).zone_list) != 0 {
            return;
        }
        launch_zone_timer(ctx);
    }
}

unsafe extern "C" fn zone_requeue(mut ctx: *mut DNSContext, mut z: *mut DNSZone) {
    let mut el = ::core::ptr::null_mut::<List>();
    let mut req = ::core::ptr::null_mut::<DNSRequest>();
    el = (*z).host_list.head.next;
    while el != &raw mut (*z).host_list.head {
        req = (el as *mut ::core::ffi::c_char).offset(-(24 as ::core::ffi::c_ulong as isize))
            as *mut DNSRequest;
        if (*req).done {
            (*req).res_ttl = 0 as usec_t;
            (*ctx).active += 1;
            impl_launch_query(req);
        }
        el = (*el).next;
    }
}

unsafe extern "C" fn got_zone_serial(mut ctx: *mut DNSContext, mut serial: *mut uint32_t) {
    let mut z = (*ctx).cur_zone;
    let mut el = ::core::ptr::null_mut::<List>();
    (*ctx).active -= 1;
    if (*ctx).zone_state == 0 || z.is_null() {
        return;
    }
    if !serial.is_null() {
        let mut s1: int32_t = (*z).serial as int32_t;
        let mut s2: int32_t = *serial as int32_t;
        let mut ds: int32_t = s2 - s1;
        if ds > 0 as int32_t {
            let mut _log_ctx = NULL;
            log_generic(
                LG_INFO,
                _log_ctx,
                b"zone '%s' serial changed: old=%u new=%u\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*z).zonename,
                (*z).serial,
                *serial,
            );
            (*z).serial = *serial;
            zone_requeue(ctx, z);
        } else {
            let mut _log_ctx_0 = NULL;
            if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_DEBUG,
                    _log_ctx_0,
                    b"zone '%s' unchanged: serial=%u\0" as *const u8 as *const ::core::ffi::c_char,
                    (*z).zonename,
                    *serial,
                );
            }
        }
    } else {
        let mut _log_ctx_1 = NULL;
        if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_DEBUG,
                _log_ctx_1,
                b"failure to get zone '%s' serial\0" as *const u8 as *const ::core::ffi::c_char,
                (*z).zonename,
            );
        }
    }
    el = (*z).lnode.next;
    if el != &raw mut (*ctx).zone_list {
        z = (el as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut DNSZone;
        (*ctx).cur_zone = z;
        (*ctx).active += 1;
    } else {
        launch_zone_timer(ctx);
    };
}

unsafe extern "C" fn walk_name(mut n: *mut AANode, mut arg: *mut ::core::ffi::c_void) {
    let mut w = arg as *mut WalkInfo;
    let mut req = (n as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
        as *mut DNSRequest;
    (*w).name_cb.expect("non-null function pointer")(
        (*w).arg,
        (*req).name,
        (*req).result,
        (*req).res_ttl,
    );
}

unsafe extern "C" fn walk_zone(mut n: *mut AANode, mut arg: *mut ::core::ffi::c_void) {
    let mut w = arg as *mut WalkInfo;
    let mut z = (n as *mut ::core::ffi::c_char).offset(-(16 as ::core::ffi::c_ulong as isize))
        as *mut DNSZone;
    (*w).zone_cb.expect("non-null function pointer")(
        (*w).arg,
        (*z).zonename,
        (*z).serial,
        statlist_count(&raw mut (*z).host_list),
    );
}
#[no_mangle]

pub unsafe extern "C" fn adns_walk_names(
    mut ctx: *mut DNSContext,
    mut cb: adns_walk_name_f,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut w = WalkInfo {
        name_cb: None,
        zone_cb: None,
        arg: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    w.name_cb = cb;
    w.arg = arg;
    aatree_walk(
        &raw mut (*ctx).req_tree,
        AA_WALK_IN_ORDER,
        Some(walk_name as unsafe extern "C" fn(*mut AANode, *mut ::core::ffi::c_void) -> ()),
        &raw mut w as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]

pub unsafe extern "C" fn adns_walk_zones(
    mut ctx: *mut DNSContext,
    mut cb: adns_walk_zone_f,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut w = WalkInfo {
        name_cb: None,
        zone_cb: None,
        arg: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    w.zone_cb = cb;
    w.arg = arg;
    aatree_walk(
        &raw mut (*ctx).zone_tree,
        AA_WALK_IN_ORDER,
        Some(walk_zone as unsafe extern "C" fn(*mut AANode, *mut ::core::ffi::c_void) -> ()),
        &raw mut w as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]

pub unsafe extern "C" fn adns_per_loop(mut _ctx: *mut DNSContext) {}


extern "C" {
    pub fn get_cached_time() -> usec_t;
}