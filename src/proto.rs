#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = *mut ::core::ffi::c_char;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:23"]
pub mod _types_h {
    #[c2rust::src_loc = "32:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "34:1"]
    pub type __uint16_t = u16;
    #[c2rust::src_loc = "35:1"]
    pub type __int32_t = i32;
    #[c2rust::src_loc = "36:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "77:1"]
    pub type __darwin_ptrdiff_t = isize;
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
    #[c2rust::src_loc = "95:1"]
    pub type __darwin_va_list = __builtin_va_list;
    #[c2rust::src_loc = "118:1"]
    pub type __darwin_ssize_t = isize;
    #[c2rust::src_loc = "119:1"]
    pub type __darwin_time_t = ::core::ffi::c_long;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_int32_t.h:23"]
pub mod _int32_t_h {
    #[c2rust::src_loc = "30:1"]
    pub type int32_t = i32;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:23"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:23"]
pub mod sys__types_h {
    #[c2rust::src_loc = "84:1"]
    pub type __darwin_pid_t = __int32_t;
    #[c2rust::src_loc = "86:1"]
    pub type __darwin_suseconds_t = __int32_t;
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_uid_t = __uint32_t;
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    use super::_types_h::{__int32_t, __uint32_t};
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_in_addr_t.h:23"]
pub mod _in_addr_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type in_addr_t = __uint32_t;
    use super::_types_h::__uint32_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_in_port_t.h:23"]
pub mod _in_port_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type in_port_t = __uint16_t;
    use super::_types_h::__uint16_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_pid_t.h:23"]
pub mod _pid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pid_t = __darwin_pid_t;
    use super::sys__types_h::__darwin_pid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uid_t.h:23"]
pub mod _uid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uid_t = __darwin_uid_t;
    use super::sys__types_h::__darwin_uid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:23"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ssize_t.h:23"]
pub mod _ssize_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ptrdiff_t.h:23"]
pub mod _ptrdiff_t_h {
    #[c2rust::src_loc = "51:1"]
    pub type ptrdiff_t = __darwin_ptrdiff_t;
    use super::_types_h::__darwin_ptrdiff_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:23"]
pub mod _uint8_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint8_t = u8;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint16_t.h:23"]
pub mod _uint16_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint16_t = u16;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:23"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = u32;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint64_t.h:23"]
pub mod _uint64_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint64_t = u64;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_va_list.h:23"]
pub mod _va_list_h {
    #[c2rust::src_loc = "44:1"]
    pub type va_list = __darwin_va_list;
    use super::_types_h::__darwin_va_list;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_timeval.h:23"]
pub mod _timeval_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "34:1"]
    pub struct timeval {
        pub tv_sec: __darwin_time_t,
        pub tv_usec: __darwin_suseconds_t,
    }
    use super::_types_h::__darwin_time_t;
    use super::sys__types_h::__darwin_suseconds_t;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/tls/tls.h:23"]
pub mod tls_h {
    extern "C" {
        #[c2rust::src_loc = "66:1"]
        pub type tls;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/time.h:23"]
pub mod time_h {
    #[c2rust::src_loc = "40:1"]
    pub type usec_t = uint64_t;
    use super::_uint64_t_h::uint64_t;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/list.h:23"]
pub mod list_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "35:1"]
    pub struct List {
        pub next: *mut List,
        pub prev: *mut List,
    }
    #[inline]
    #[c2rust::src_loc = "52:1"]
    pub unsafe extern "C" fn list_empty(mut list: *const List) -> ::core::ffi::c_int {
        return ((*list).next == list as *mut List) as ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/statlist.h:23"]
pub mod statlist_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:1"]
    pub struct StatList {
        pub head: List,
        pub cur_count: ::core::ffi::c_int,
    }
    #[inline]
    #[c2rust::src_loc = "120:1"]
    pub unsafe extern "C" fn statlist_empty(mut list: *const StatList) -> bool {
        return list_empty(&raw const (*list).head) != 0;
    }
    use super::list_h::{list_empty, List};
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/aatree.h:23"]
pub mod aatree_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "41:1"]
    pub struct AATree {
        pub root: *mut AANode,
        pub count: ::core::ffi::c_int,
        pub node_cmp: aatree_cmp_f,
        pub release_cb: aatree_walker_f,
    }
    #[c2rust::src_loc = "36:1"]
    pub type aatree_walker_f =
        Option<unsafe extern "C" fn(*mut AANode, *mut ::core::ffi::c_void) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "57:1"]
    pub struct AANode {
        pub left: *mut AANode,
        pub right: *mut AANode,
        pub level: ::core::ffi::c_int,
    }
    #[c2rust::src_loc = "33:1"]
    pub type aatree_cmp_f =
        Option<unsafe extern "C" fn(uintptr_t, *mut AANode) -> ::core::ffi::c_int>;
    use super::_uintptr_t_h::uintptr_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_sa_family_t.h:23"]
pub mod _sa_family_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type sa_family_t = __uint8_t;
    use super::_types_h::__uint8_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/socket.h:23"]
pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "414:1"]
    pub struct sockaddr {
        pub sa_len: __uint8_t,
        pub sa_family: sa_family_t,
        pub sa_data: [::core::ffi::c_char; 14],
    }
    use super::_sa_family_t_h::sa_family_t;
    use super::_types_h::__uint8_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet/in.h:23"]
pub mod in_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "301:1"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "374:1"]
    pub struct sockaddr_in {
        pub sin_len: __uint8_t,
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [::core::ffi::c_char; 8],
    }
    use super::_in_addr_t_h::in_addr_t;
    use super::_in_port_t_h::in_port_t;
    use super::_sa_family_t_h::sa_family_t;
    use super::_types_h::__uint8_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet6/in6.h:23"]
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "170:1"]
    pub struct sockaddr_in6 {
        pub sin6_len: __uint8_t,
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: __uint32_t,
        pub sin6_addr: in6_addr,
        pub sin6_scope_id: __uint32_t,
    }
    use super::_in_port_t_h::in_port_t;
    use super::_sa_family_t_h::sa_family_t;
    use super::_types_h::{__uint16_t, __uint32_t, __uint8_t};
}
#[c2rust::header_src = "/opt/homebrew/Cellar/libevent/2.1.12_1/include/event2/event.h:23"]
pub mod event_h {
    extern "C" {
        #[c2rust::src_loc = "217:1"]
        pub type event_base;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/libevent/2.1.12_1/include/event2/event_struct.h:23"]
pub mod event_struct_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "123:1"]
    pub struct event {
        pub ev_evcallback: event_callback,
        pub ev_timeout_pos: C2RustUnnamed_5,
        pub ev_fd: ::core::ffi::c_int,
        pub ev_base: *mut event_base,
        pub ev_: C2RustUnnamed_0,
        pub ev_events: ::core::ffi::c_short,
        pub ev_res: ::core::ffi::c_short,
        pub ev_timeout: timeval,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "135:2"]
    pub union C2RustUnnamed_0 {
        pub ev_io: C2RustUnnamed_3,
        pub ev_signal: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "143:3"]
    pub struct C2RustUnnamed_1 {
        pub ev_signal_next: C2RustUnnamed_2,
        pub ev_ncalls: ::core::ffi::c_short,
        pub ev_pncalls: *mut ::core::ffi::c_short,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "144:4"]
    pub struct C2RustUnnamed_2 {
        pub le_next: *mut event,
        pub le_prev: *mut *mut event,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "137:3"]
    pub struct C2RustUnnamed_3 {
        pub ev_io_next: C2RustUnnamed_4,
        pub ev_timeout: timeval,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "138:4"]
    pub struct C2RustUnnamed_4 {
        pub le_next: *mut event,
        pub le_prev: *mut *mut event,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "127:2"]
    pub union C2RustUnnamed_5 {
        pub ev_next_with_common_timeout: C2RustUnnamed_6,
        pub min_heap_idx: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "128:3"]
    pub struct C2RustUnnamed_6 {
        pub tqe_next: *mut event,
        pub tqe_prev: *mut *mut event,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "107:1"]
    pub struct event_callback {
        pub evcb_active_next: C2RustUnnamed_8,
        pub evcb_flags: ::core::ffi::c_short,
        pub evcb_pri: uint8_t,
        pub evcb_closure: uint8_t,
        pub evcb_cb_union: C2RustUnnamed_7,
        pub evcb_arg: *mut ::core::ffi::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:9"]
    pub union C2RustUnnamed_7 {
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
    #[c2rust::src_loc = "108:2"]
    pub struct C2RustUnnamed_8 {
        pub tqe_next: *mut event_callback,
        pub tqe_prev: *mut *mut event_callback,
    }
    use super::_timeval_h::timeval;
    use super::_uint8_t_h::uint8_t;
    use super::event_h::event_base;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/cryptohash.h:23"]
pub mod cryptohash_h {
    #[c2rust::src_loc = "19:9"]
    pub type pg_cryptohash_type = ::core::ffi::c_uint;
    #[c2rust::src_loc = "24:2"]
    pub const PG_SHA512: pg_cryptohash_type = 3;
    #[c2rust::src_loc = "23:2"]
    pub const PG_SHA384: pg_cryptohash_type = 2;
    #[c2rust::src_loc = "22:2"]
    pub const PG_SHA256: pg_cryptohash_type = 1;
    #[c2rust::src_loc = "21:2"]
    pub const PG_SHA224: pg_cryptohash_type = 0;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/uthash.h:23"]
pub mod uthash_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1072:9"]
    pub struct UT_hash_bucket {
        pub hh_head: *mut UT_hash_handle,
        pub count: ::core::ffi::c_uint,
        pub expand_mult: ::core::ffi::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1129:9"]
    pub struct UT_hash_handle {
        pub tbl: *mut UT_hash_table,
        pub prev: *mut ::core::ffi::c_void,
        pub next: *mut ::core::ffi::c_void,
        pub hh_prev: *mut UT_hash_handle,
        pub hh_next: *mut UT_hash_handle,
        pub key: *const ::core::ffi::c_void,
        pub keylen: ::core::ffi::c_uint,
        pub hashv: ::core::ffi::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1096:9"]
    pub struct UT_hash_table {
        pub buckets: *mut UT_hash_bucket,
        pub num_buckets: ::core::ffi::c_uint,
        pub log2_num_buckets: ::core::ffi::c_uint,
        pub num_items: ::core::ffi::c_uint,
        pub tail: *mut UT_hash_handle,
        pub hho: ptrdiff_t,
        pub ideal_chain_maxlen: ::core::ffi::c_uint,
        pub nonideal_items: ::core::ffi::c_uint,
        pub ineff_expands: ::core::ffi::c_uint,
        pub noexpand: ::core::ffi::c_uint,
        pub signature: uint32_t,
    }
    use super::_ptrdiff_t_h::ptrdiff_t;
    use super::_uint32_t_h::uint32_t;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/bouncer.h:23"]
pub mod bouncer_h {
    #[c2rust::src_loc = "70:1"]
    pub type SocketState = ::core::ffi::c_uint;
    #[c2rust::src_loc = "88:2"]
    pub const SV_TESTED: SocketState = 16;
    #[c2rust::src_loc = "87:2"]
    pub const SV_USED: SocketState = 15;
    #[c2rust::src_loc = "86:2"]
    pub const SV_ACTIVE_CANCEL: SocketState = 14;
    #[c2rust::src_loc = "85:2"]
    pub const SV_ACTIVE: SocketState = 13;
    #[c2rust::src_loc = "84:2"]
    pub const SV_IDLE: SocketState = 12;
    #[c2rust::src_loc = "83:2"]
    pub const SV_BEING_CANCELED: SocketState = 11;
    #[c2rust::src_loc = "82:2"]
    pub const SV_LOGIN: SocketState = 10;
    #[c2rust::src_loc = "81:2"]
    pub const SV_JUSTFREE: SocketState = 9;
    #[c2rust::src_loc = "80:2"]
    pub const SV_FREE: SocketState = 8;
    #[c2rust::src_loc = "78:2"]
    pub const CL_ACTIVE_CANCEL: SocketState = 7;
    #[c2rust::src_loc = "77:2"]
    pub const CL_WAITING_CANCEL: SocketState = 6;
    #[c2rust::src_loc = "76:2"]
    pub const CL_ACTIVE: SocketState = 5;
    #[c2rust::src_loc = "75:2"]
    pub const CL_WAITING_LOGIN: SocketState = 4;
    #[c2rust::src_loc = "74:2"]
    pub const CL_WAITING: SocketState = 3;
    #[c2rust::src_loc = "73:2"]
    pub const CL_LOGIN: SocketState = 2;
    #[c2rust::src_loc = "72:2"]
    pub const CL_JUSTFREE: SocketState = 1;
    #[c2rust::src_loc = "71:2"]
    pub const CL_FREE: SocketState = 0;
    #[c2rust::src_loc = "131:1"]
    pub type PacketCallbackFlag = ::core::ffi::c_uint;
    #[c2rust::src_loc = "144:2"]
    pub const CB_HANDLE_COMPLETE_PACKET: PacketCallbackFlag = 2;
    #[c2rust::src_loc = "139:2"]
    pub const CB_WANT_COMPLETE_PACKET: PacketCallbackFlag = 1;
    #[c2rust::src_loc = "133:2"]
    pub const CB_NONE: PacketCallbackFlag = 0;
    #[c2rust::src_loc = "147:1"]
    pub type LoadBalanceHosts = ::core::ffi::c_uint;
    #[c2rust::src_loc = "149:2"]
    pub const LOAD_BALANCE_HOSTS_ROUND_ROBIN: LoadBalanceHosts = 1;
    #[c2rust::src_loc = "148:2"]
    pub const LOAD_BALANCE_HOSTS_DISABLE: LoadBalanceHosts = 0;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "663:1"]
    pub struct PgSocket {
        pub head: List,
        pub cancel_head: List,
        pub link: *mut PgSocket,
        pub pool: *mut PgPool,
        pub login_user_credentials: *mut PgCredentials,
        pub id: ::core::ffi::c_ulonglong,
        pub client_auth_type: ::core::ffi::c_int,
        pub outstanding_requests: StatList,
        #[bitfield(name = "state", ty = "SocketState", bits = "0..=7")]
        #[bitfield(name = "contributes_db_client_count", ty = "bool", bits = "8..=8")]
        #[bitfield(name = "user_connection_counted", ty = "bool", bits = "9..=9")]
        #[bitfield(name = "ready", ty = "bool", bits = "10..=10")]
        #[bitfield(name = "idle_tx", ty = "bool", bits = "11..=11")]
        #[bitfield(name = "close_needed", ty = "bool", bits = "12..=12")]
        #[bitfield(name = "setting_vars", ty = "bool", bits = "13..=13")]
        #[bitfield(name = "exec_on_connect", ty = "bool", bits = "14..=14")]
        #[bitfield(name = "resetting", ty = "bool", bits = "15..=15")]
        #[bitfield(name = "copy_mode", ty = "bool", bits = "16..=16")]
        #[bitfield(name = "wait_for_welcome", ty = "bool", bits = "17..=17")]
        #[bitfield(name = "welcome_sent", ty = "bool", bits = "18..=18")]
        #[bitfield(name = "wait_for_user_conn", ty = "bool", bits = "19..=19")]
        #[bitfield(name = "wait_for_user", ty = "bool", bits = "20..=20")]
        #[bitfield(name = "wait_for_auth", ty = "bool", bits = "21..=21")]
        #[bitfield(name = "suspended", ty = "bool", bits = "22..=22")]
        #[bitfield(name = "sent_wait_notification", ty = "bool", bits = "23..=23")]
        #[bitfield(name = "admin_user", ty = "bool", bits = "24..=24")]
        #[bitfield(name = "own_user", ty = "bool", bits = "25..=25")]
        #[bitfield(name = "wait_for_response", ty = "bool", bits = "26..=26")]
        #[bitfield(name = "wait_sslchar", ty = "bool", bits = "27..=27")]
        #[bitfield(name = "query_failed", ty = "bool", bits = "28..=28")]
        pub state_contributes_db_client_count_user_connection_counted_ready_idle_tx_close_needed_setting_vars_exec_on_connect_resetting_copy_mode_wait_for_welcome_welcome_sent_wait_for_user_conn_wait_for_user_wait_for_auth_suspended_sent_wait_notification_admin_user_own_user_wait_for_response_wait_sslchar_query_failed:
            [u8; 4],
        pub replication: ReplicationType,
        pub startup_options: *mut ::core::ffi::c_char,
        pub connect_time: usec_t,
        pub request_time: usec_t,
        pub query_start: usec_t,
        pub xact_start: usec_t,
        pub wait_start: usec_t,
        pub cancel_key: [uint8_t; 8],
        pub canceling_clients: StatList,
        pub canceled_server: *mut PgSocket,
        pub remote_addr: PgAddr,
        pub local_addr: PgAddr,
        pub host: *mut ::core::ffi::c_char,
        pub c2rust_unnamed: C2RustUnnamed_9,
        pub scram_state: ScramState,
        pub vars: VarCache,
        pub client_prepared_statements: *mut PgClientPreparedStatement,
        pub server_prepared_statements: *mut PgServerPreparedStatement,
        pub packet_cb_state: CallbackState,
        pub sbuf: SBuf,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "768:2"]
    pub struct CallbackState {
        #[bitfield(name = "flag", ty = "PacketCallbackFlag", bits = "0..=7")]
        pub flag: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
        pub pkt: PktHdr,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "732:2"]
    pub struct ScramState {
        pub client_nonce: *mut ::core::ffi::c_char,
        pub client_first_message_bare: *mut ::core::ffi::c_char,
        pub client_final_message_without_proof: *mut ::core::ffi::c_char,
        pub server_nonce: *mut ::core::ffi::c_char,
        pub server_first_message: *mut ::core::ffi::c_char,
        pub iterations: ::core::ffi::c_int,
        pub hash_type: pg_cryptohash_type,
        pub key_length: ::core::ffi::c_int,
        pub salt: *mut uint8_t,
        pub saltlen: ::core::ffi::c_int,
        pub SaltedPassword: *mut uint8_t,
        pub cbind_flag: ::core::ffi::c_char,
        pub adhoc: bool,
        pub encoded_salt: *mut ::core::ffi::c_char,
        pub ClientKey: [uint8_t; 32],
        pub StoredKey: [uint8_t; 32],
        pub ServerKey: [uint8_t; 32],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "727:2"]
    pub union C2RustUnnamed_9 {
        pub dns_token: *mut DNSToken,
        pub db: *mut PgDatabase,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "570:1"]
    pub struct PgDatabase {
        pub head: List,
        pub name: [::core::ffi::c_char; 64],
        pub peer_id: ::core::ffi::c_int,
        pub pool: *mut PgPool,
        pub host: *mut ::core::ffi::c_char,
        pub port: ::core::ffi::c_int,
        pub pool_size: ::core::ffi::c_int,
        pub min_pool_size: ::core::ffi::c_int,
        pub res_pool_size: ::core::ffi::c_int,
        pub pool_mode: ::core::ffi::c_int,
        pub max_db_client_connections: ::core::ffi::c_int,
        pub max_db_connections: ::core::ffi::c_int,
        pub server_lifetime: usec_t,
        pub connect_query: *mut ::core::ffi::c_char,
        pub load_balance_hosts: LoadBalanceHosts,
        pub startup_params: *mut PktBuf,
        pub dbname: *const ::core::ffi::c_char,
        pub auth_dbname: *mut ::core::ffi::c_char,
        pub forced_user_credentials: *mut PgCredentials,
        pub auth_user_credentials: *mut PgCredentials,
        pub auth_query: *mut ::core::ffi::c_char,
        pub db_paused: bool,
        pub db_wait_close: bool,
        pub db_dead: bool,
        pub db_auto: bool,
        pub db_disabled: bool,
        pub admin: bool,
        pub fake: bool,
        pub inactive_time: usec_t,
        pub active_stamp: ::core::ffi::c_uint,
        pub connection_count: ::core::ffi::c_int,
        pub client_connection_count: ::core::ffi::c_int,
        pub user_tree: AATree,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "509:1"]
    pub struct PgCredentials {
        pub tree_node: AANode,
        pub name: [::core::ffi::c_char; 128],
        pub passwd: [::core::ffi::c_char; 2048],
        pub mock_auth: bool,
        pub dynamic_passwd: bool,
        pub global_user: *mut PgGlobalUser,
        pub scram_ClientKey: [uint8_t; 32],
        pub scram_ServerKey: [uint8_t; 32],
        pub scram_StoredKey: [uint8_t; 32],
        pub scram_Iiterations: ::core::ffi::c_int,
        pub scram_SaltKey: *mut ::core::ffi::c_char,
        pub use_scram_keys: bool,
        pub adhoc_scram_secrets_cached: bool,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "549:1"]
    pub struct PgGlobalUser {
        pub credentials: PgCredentials,
        pub head: List,
        pub pool_list: List,
        pub pool_mode: ::core::ffi::c_int,
        pub pool_size: ::core::ffi::c_int,
        pub res_pool_size: ::core::ffi::c_int,
        pub transaction_timeout: usec_t,
        pub idle_transaction_timeout: usec_t,
        pub query_timeout: usec_t,
        pub client_idle_timeout: usec_t,
        pub max_user_connections: ::core::ffi::c_int,
        pub max_user_client_connections: ::core::ffi::c_int,
        pub connection_count: ::core::ffi::c_int,
        pub client_connection_count: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "345:1"]
    pub struct PgPool {
        pub head: List,
        pub map_head: List,
        pub db: *mut PgDatabase,
        pub user_credentials: *mut PgCredentials,
        pub active_client_list: StatList,
        pub waiting_client_list: StatList,
        pub waiting_cancel_req_list: StatList,
        pub active_cancel_req_list: StatList,
        pub active_server_list: StatList,
        pub active_cancel_server_list: StatList,
        pub being_canceled_server_list: StatList,
        pub idle_server_list: StatList,
        pub used_server_list: StatList,
        pub tested_server_list: StatList,
        pub new_server_list: StatList,
        pub stats: PgStats,
        pub newer_stats: PgStats,
        pub older_stats: PgStats,
        pub welcome_msg: *mut PktBuf,
        pub orig_vars: VarCache,
        pub last_lifetime_disconnect: usec_t,
        pub last_connect_time: usec_t,
        #[bitfield(name = "last_connect_failed", ty = "bool", bits = "0..=0")]
        pub last_connect_failed: [u8; 1],
        pub last_connect_failed_message: [::core::ffi::c_char; 100],
        #[bitfield(name = "last_login_failed", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "welcome_msg_ready", ty = "bool", bits = "1..=1")]
        pub last_login_failed_welcome_msg_ready: [u8; 1],
        pub rrcounter: uint16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "320:1"]
    pub struct PgStats {
        pub server_assignment_count: uint64_t,
        pub xact_count: uint64_t,
        pub query_count: uint64_t,
        pub server_bytes: uint64_t,
        pub client_bytes: uint64_t,
        pub xact_time: usec_t,
        pub query_time: usec_t,
        pub wait_time: usec_t,
        pub ps_server_parse_count: uint64_t,
        pub ps_client_parse_count: uint64_t,
        pub ps_bind_count: uint64_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "292:1"]
    pub union PgAddr {
        pub sa: sockaddr,
        pub sin: sockaddr_in,
        pub sin6: sockaddr_in6,
        pub scred: sockaddr_ucreds,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "282:1"]
    pub struct sockaddr_ucreds {
        pub sin: sockaddr_in,
        pub uid: uid_t,
        pub pid: pid_t,
    }
    #[c2rust::src_loc = "650:1"]
    pub type ReplicationType = ::core::ffi::c_uint;
    #[c2rust::src_loc = "653:2"]
    pub const REPLICATION_PHYSICAL: ReplicationType = 2;
    #[c2rust::src_loc = "652:2"]
    pub const REPLICATION_LOGICAL: ReplicationType = 1;
    #[c2rust::src_loc = "651:2"]
    pub const REPLICATION_NONE: ReplicationType = 0;
    #[c2rust::src_loc = "247:9"]
    pub const PKT_STARTUP_V2: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
    #[c2rust::src_loc = "248:9"]
    pub const PKT_STARTUP_V3: ::core::ffi::c_int = 0x30000 as ::core::ffi::c_int;
    #[c2rust::src_loc = "249:9"]
    pub const PKT_STARTUP_V3_UNSUPPORTED: ::core::ffi::c_int = 0x30001 as ::core::ffi::c_int;
    #[c2rust::src_loc = "250:9"]
    pub const PKT_STARTUP_V4: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
    #[c2rust::src_loc = "251:9"]
    pub const PKT_CANCEL: ::core::ffi::c_int = 80877102 as ::core::ffi::c_int;
    #[c2rust::src_loc = "252:9"]
    pub const PKT_SSLREQ: ::core::ffi::c_int = 80877103 as ::core::ffi::c_int;
    #[c2rust::src_loc = "253:9"]
    pub const PKT_GSSENCREQ: ::core::ffi::c_int = 80877104 as ::core::ffi::c_int;
    #[c2rust::src_loc = "272:9"]
    pub const CANCELLATION_TTL_MASK: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
    #[inline]
    #[c2rust::src_loc = "917:1"]
    pub unsafe extern "C" fn first_socket(mut slist: *mut StatList) -> *mut PgSocket {
        if statlist_empty(slist) {
            return ::core::ptr::null_mut::<PgSocket>();
        }
        return ((*slist).head.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut PgSocket;
    }

    use super::_pid_t_h::pid_t;
    use super::_uid_t_h::uid_t;
    use super::_uint16_t_h::uint16_t;
    use super::_uint64_t_h::uint64_t;
    use super::_uint8_t_h::uint8_t;
    use super::aatree_h::{AANode, AATree};
    use super::cryptohash_h::pg_cryptohash_type;
    use super::dnslookup_h::DNSToken;
    use super::in6_h::sockaddr_in6;
    use super::in_h::sockaddr_in;
    use super::list_h::List;
    use super::pktbuf_h::PktBuf;
    use super::prepare_h::{PgClientPreparedStatement, PgServerPreparedStatement};
    use super::proto_h::PktHdr;
    use super::sbuf_h::SBuf;
    use super::socket_h::sockaddr;
    use super::statlist_h::{statlist_empty, StatList};
    use super::time_h::usec_t;
    use super::varcache_h::VarCache;
    extern "C" {
        #[c2rust::src_loc = "656:1"]
        pub static mut replication_type_parameters: [*const ::core::ffi::c_char; 3];
        #[c2rust::src_loc = "808:1"]
        pub static mut cf_peer_id: ::core::ffi::c_int;
        #[c2rust::src_loc = "872:1"]
        pub static mut cf_max_packet_size: ::core::ffi::c_uint;
        #[c2rust::src_loc = "886:1"]
        pub static mut cf_log_pooler_errors: ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/sbuf.h:23"]
pub mod sbuf_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "71:1"]
    pub struct SBuf {
        pub ev: event,
        pub wait_type: uint8_t,
        pub pkt_action: uint8_t,
        pub tls_state: uint8_t,
        pub sock: ::core::ffi::c_int,
        pub pkt_remain: ::core::ffi::c_uint,
        pub skip_remain: ::core::ffi::c_uint,
        pub extra_packets: MBuf,
        pub extra_packet_queue_after: bool,
        pub proto_cb: sbuf_cb_t,
        pub dst: *mut SBuf,
        pub io: *mut IOBuf,
        pub ops: *const SBufIO,
        pub tls: *mut tls,
        pub tls_host: *const ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "58:1"]
    pub struct SBufIO {
        pub sbufio_peek:
            Option<unsafe extern "C" fn(*mut SBuf, *mut ::core::ffi::c_void, size_t) -> ssize_t>,
        pub sbufio_recv:
            Option<unsafe extern "C" fn(*mut SBuf, *mut ::core::ffi::c_void, size_t) -> ssize_t>,
        pub sbufio_send:
            Option<unsafe extern "C" fn(*mut SBuf, *const ::core::ffi::c_void, size_t) -> ssize_t>,
        pub sbufio_close: Option<unsafe extern "C" fn(*mut SBuf) -> ::core::ffi::c_int>,
    }
    #[c2rust::src_loc = "54:1"]
    pub type sbuf_cb_t = Option<unsafe extern "C" fn(*mut SBuf, SBufEvent, *mut MBuf) -> bool>;
    #[c2rust::src_loc = "24:9"]
    pub type SBufEvent = ::core::ffi::c_uint;
    #[c2rust::src_loc = "32:2"]
    pub const SBUF_EV_TLS_READY: SBufEvent = 7;
    #[c2rust::src_loc = "31:2"]
    pub const SBUF_EV_PKT_CALLBACK: SBufEvent = 6;
    #[c2rust::src_loc = "30:2"]
    pub const SBUF_EV_FLUSH: SBufEvent = 5;
    #[c2rust::src_loc = "29:2"]
    pub const SBUF_EV_CONNECT_OK: SBufEvent = 4;
    #[c2rust::src_loc = "28:2"]
    pub const SBUF_EV_CONNECT_FAILED: SBufEvent = 3;
    #[c2rust::src_loc = "27:2"]
    pub const SBUF_EV_SEND_FAILED: SBufEvent = 2;
    #[c2rust::src_loc = "26:2"]
    pub const SBUF_EV_RECV_FAILED: SBufEvent = 1;
    #[c2rust::src_loc = "25:2"]
    pub const SBUF_EV_READ: SBufEvent = 0;
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    use super::_uint8_t_h::uint8_t;
    use super::event_struct_h::event;
    use super::iobuf_h::IOBuf;
    use super::mbuf_h::MBuf;
    use super::tls_h::tls;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/iobuf.h:23"]
pub mod iobuf_h {
    #[c2rust::src_loc = "55:1"]
    pub type IOBuf = iobuf;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:1"]
    pub struct iobuf {
        pub done_pos: ::core::ffi::c_uint,
        pub parse_pos: ::core::ffi::c_uint,
        pub recv_pos: ::core::ffi::c_uint,
        pub buf: [uint8_t; 0],
    }
    use super::_uint8_t_h::uint8_t;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/mbuf.h:23"]
pub mod mbuf_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "14:1"]
    pub struct MBuf {
        pub data: *mut uint8_t,
        pub read_pos: ::core::ffi::c_uint,
        pub write_pos: ::core::ffi::c_uint,
        pub alloc_len: ::core::ffi::c_uint,
        pub reader: bool,
        pub fixed: bool,
    }
    #[inline]
    #[c2rust::src_loc = "33:1"]
    pub unsafe extern "C" fn mbuf_init_fixed_reader(
        mut buf: *mut MBuf,
        mut ptr: *const ::core::ffi::c_void,
        mut len: ::core::ffi::c_uint,
    ) {
        (*buf).data = ptr as *mut uint8_t;
        (*buf).read_pos = 0 as ::core::ffi::c_uint;
        (*buf).write_pos = len;
        (*buf).alloc_len = len;
        (*buf).reader = true_0 != 0;
        (*buf).fixed = true_0 != 0;
    }
    #[inline]
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn mbuf_avail_for_read(mut buf: *const MBuf) -> ::core::ffi::c_uint {
        return (*buf).write_pos.wrapping_sub((*buf).read_pos);
    }
    #[inline]
    #[c2rust::src_loc = "152:1"]
    pub unsafe extern "C" fn mbuf_get_byte(mut buf: *mut MBuf, mut dst_p: *mut uint8_t) -> bool {
        if (*buf).read_pos.wrapping_add(1 as ::core::ffi::c_uint) > (*buf).write_pos {
            return false_0 != 0;
        }
        let fresh0 = (*buf).read_pos;
        (*buf).read_pos = (*buf).read_pos.wrapping_add(1);
        *dst_p = *(*buf).data.offset(fresh0 as isize);
        return true_0 != 0;
    }
    #[inline]
    #[c2rust::src_loc = "171:1"]
    pub unsafe extern "C" fn mbuf_get_uint16be(
        mut buf: *mut MBuf,
        mut dst_p: *mut uint16_t,
    ) -> bool {
        let mut a: ::core::ffi::c_uint = 0;
        let mut b: ::core::ffi::c_uint = 0;
        if (*buf).read_pos.wrapping_add(2 as ::core::ffi::c_uint) > (*buf).write_pos {
            return false_0 != 0;
        }
        let fresh1 = (*buf).read_pos;
        (*buf).read_pos = (*buf).read_pos.wrapping_add(1);
        a = *(*buf).data.offset(fresh1 as isize) as ::core::ffi::c_uint;
        let fresh2 = (*buf).read_pos;
        (*buf).read_pos = (*buf).read_pos.wrapping_add(1);
        b = *(*buf).data.offset(fresh2 as isize) as ::core::ffi::c_uint;
        *dst_p = (a << 8 as ::core::ffi::c_int | b) as uint16_t;
        return true_0 != 0;
    }
    #[inline]
    #[c2rust::src_loc = "184:1"]
    pub unsafe extern "C" fn mbuf_get_uint32be(
        mut buf: *mut MBuf,
        mut dst_p: *mut uint32_t,
    ) -> bool {
        let mut a: ::core::ffi::c_uint = 0;
        let mut b: ::core::ffi::c_uint = 0;
        let mut c: ::core::ffi::c_uint = 0;
        let mut d: ::core::ffi::c_uint = 0;
        if (*buf).read_pos.wrapping_add(4 as ::core::ffi::c_uint) > (*buf).write_pos {
            return false_0 != 0;
        }
        let fresh3 = (*buf).read_pos;
        (*buf).read_pos = (*buf).read_pos.wrapping_add(1);
        a = *(*buf).data.offset(fresh3 as isize) as ::core::ffi::c_uint;
        let fresh4 = (*buf).read_pos;
        (*buf).read_pos = (*buf).read_pos.wrapping_add(1);
        b = *(*buf).data.offset(fresh4 as isize) as ::core::ffi::c_uint;
        let fresh5 = (*buf).read_pos;
        (*buf).read_pos = (*buf).read_pos.wrapping_add(1);
        c = *(*buf).data.offset(fresh5 as isize) as ::core::ffi::c_uint;
        let fresh6 = (*buf).read_pos;
        (*buf).read_pos = (*buf).read_pos.wrapping_add(1);
        d = *(*buf).data.offset(fresh6 as isize) as ::core::ffi::c_uint;
        *dst_p = (a << 24 as ::core::ffi::c_int
            | b << 16 as ::core::ffi::c_int
            | c << 8 as ::core::ffi::c_int
            | d) as uint32_t;
        return true_0 != 0;
    }
    #[inline]
    #[c2rust::src_loc = "210:1"]
    pub unsafe extern "C" fn mbuf_get_bytes(
        mut buf: *mut MBuf,
        mut len: ::core::ffi::c_uint,
        mut dst_p: *mut *const uint8_t,
    ) -> bool {
        if (*buf).read_pos.wrapping_add(len) > (*buf).write_pos {
            return false_0 != 0;
        }
        *dst_p = (*buf).data.offset((*buf).read_pos as isize);
        (*buf).read_pos = (*buf).read_pos.wrapping_add(len);
        return true_0 != 0;
    }
    #[inline]
    #[c2rust::src_loc = "221:1"]
    pub unsafe extern "C" fn mbuf_get_chars(
        mut buf: *mut MBuf,
        mut len: ::core::ffi::c_uint,
        mut dst_p: *mut *const ::core::ffi::c_char,
    ) -> bool {
        if (*buf).read_pos.wrapping_add(len) > (*buf).write_pos {
            return false_0 != 0;
        }
        *dst_p = ((*buf).data as *mut ::core::ffi::c_char).offset((*buf).read_pos as isize);
        (*buf).read_pos = (*buf).read_pos.wrapping_add(len);
        return true_0 != 0;
    }
    #[inline]
    #[c2rust::src_loc = "231:1"]
    pub unsafe extern "C" fn mbuf_get_string(
        mut buf: *mut MBuf,
        mut dst_p: *mut *const ::core::ffi::c_char,
    ) -> bool {
        let mut res: *const ::core::ffi::c_char =
            ((*buf).data as *mut ::core::ffi::c_char).offset((*buf).read_pos as isize);
        let mut nul = memchr(
            res as *const ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            mbuf_avail_for_read(buf) as size_t,
        ) as *const uint8_t;
        if nul.is_null() {
            return false_0 != 0;
        }
        *dst_p = res;
        (*buf).read_pos =
            nul.offset(1 as ::core::ffi::c_int as isize)
                .offset_from((*buf).data) as ::core::ffi::c_long as ::core::ffi::c_uint;
        return true_0 != 0;
    }
    #[inline]
    #[c2rust::src_loc = "324:1"]
    pub unsafe extern "C" fn mbuf_copy(mut src: *const MBuf, mut dst: *mut MBuf) {
        *dst = *src;
    }
    #[inline]
    #[c2rust::src_loc = "329:1"]
    pub unsafe extern "C" fn mbuf_slice(
        mut src: *mut MBuf,
        mut len: ::core::ffi::c_uint,
        mut dst: *mut MBuf,
    ) -> bool {
        if len > mbuf_avail_for_read(src) {
            return false_0 != 0;
        }
        mbuf_init_fixed_reader(
            dst,
            (*src).data.offset((*src).read_pos as isize) as *const ::core::ffi::c_void,
            len,
        );
        (*src).read_pos = (*src).read_pos.wrapping_add(len);
        return true_0 != 0;
    }
    use super::_size_t_h::size_t;
    use super::_string_h::memchr;
    use super::_uint16_t_h::uint16_t;
    use super::_uint32_t_h::uint32_t;
    use super::_uint8_t_h::uint8_t;
    use super::stdbool_h::{false_0, true_0};
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/proto.h:23"]
pub mod proto_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "34:1"]
    pub struct PktHdr {
        pub type_0: ::core::ffi::c_uint,
        pub len: ::core::ffi::c_uint,
        pub data: MBuf,
    }
    #[c2rust::src_loc = "20:9"]
    pub const OLD_HEADER_LEN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    #[c2rust::src_loc = "22:9"]
    pub const NEW_HEADER_LEN: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    use super::mbuf_h::MBuf;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/prepare.h:23"]
pub mod prepare_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:9"]
    pub struct PgServerPreparedStatement {
        pub query_id: uint64_t,
        pub hh: UT_hash_handle,
        pub ps: *mut PgPreparedStatement,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:9"]
    pub struct PgPreparedStatement {
        pub hh: UT_hash_handle,
        pub query_id: uint64_t,
        pub use_count: uint32_t,
        pub query_and_parameters_len: size_t,
        pub stmt_name_len: uint8_t,
        pub stmt_name: [::core::ffi::c_char; 31],
        pub query_and_parameters: [::core::ffi::c_char; 0],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "21:9"]
    pub struct PgClientPreparedStatement {
        pub hh: UT_hash_handle,
        pub ps: *mut PgPreparedStatement,
        pub stmt_name: [::core::ffi::c_char; 0],
    }
    use super::_size_t_h::size_t;
    use super::_uint32_t_h::uint32_t;
    use super::_uint64_t_h::uint64_t;
    use super::_uint8_t_h::uint8_t;
    use super::uthash_h::UT_hash_handle;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/varcache.h:23"]
pub mod varcache_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "14:1"]
    pub struct VarCache {
        pub var_list: *mut *mut PStr,
    }
    use super::bouncer_h::PgSocket;
    use super::pktbuf_h::PktBuf;
    use super::strpool_h::PStr;
    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub fn varcache_set(
            cache: *mut VarCache,
            key: *const ::core::ffi::c_char,
            value: *const ::core::ffi::c_char,
        ) -> bool;
        #[c2rust::src_loc = "22:1"]
        pub fn varcache_apply_startup(pkt: *mut PktBuf, client: *mut PgSocket);
        #[c2rust::src_loc = "23:1"]
        pub fn varcache_fill_unset(src: *mut VarCache, dst: *mut PgSocket);
        #[c2rust::src_loc = "25:1"]
        pub fn varcache_add_params(pkt: *mut PktBuf, vars: *mut VarCache);
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/strpool.h:23"]
pub mod strpool_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:1"]
    pub struct PStr {
        pub pool: *mut StrPool,
        pub len: size_t,
        pub refcnt: ::core::ffi::c_int,
        pub str_0: [::core::ffi::c_char; 0],
    }
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "34:1"]
        pub type StrPool;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/pktbuf.h:23"]
pub mod pktbuf_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "26:1"]
    pub struct PktBuf {
        pub buf: *mut uint8_t,
        pub buf_len: ::core::ffi::c_int,
        pub write_pos: ::core::ffi::c_int,
        pub pktlen_pos: ::core::ffi::c_int,
        pub send_pos: ::core::ffi::c_int,
        pub ev: *mut event,
        pub queued_dst: *mut PgSocket,
        #[bitfield(name = "failed", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "sending", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "fixed_buf", ty = "bool", bits = "2..=2")]
        pub failed_sending_fixed_buf: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
    }
    use super::_uint8_t_h::uint8_t;
    use super::bouncer_h::PgSocket;
    use super::event_struct_h::event;
    extern "C" {
        #[c2rust::src_loc = "44:1"]
        pub fn pktbuf_dynamic(start_len: ::core::ffi::c_int) -> *mut PktBuf;
        #[c2rust::src_loc = "45:1"]
        pub fn pktbuf_static(buf: *mut PktBuf, data: *mut uint8_t, len: ::core::ffi::c_int);
        #[c2rust::src_loc = "50:1"]
        pub fn pktbuf_temp() -> *mut PktBuf;
        #[c2rust::src_loc = "56:1"]
        pub fn pktbuf_send_immediate(buf: *mut PktBuf, sk: *mut PgSocket) -> bool;
        #[c2rust::src_loc = "62:1"]
        pub fn pktbuf_start_packet(buf: *mut PktBuf, type_0: ::core::ffi::c_int);
        #[c2rust::src_loc = "67:1"]
        pub fn pktbuf_put_string(buf: *mut PktBuf, str: *const ::core::ffi::c_char);
        #[c2rust::src_loc = "68:1"]
        pub fn pktbuf_put_bytes(
            buf: *mut PktBuf,
            data: *const ::core::ffi::c_void,
            len: ::core::ffi::c_int,
        );
        #[c2rust::src_loc = "69:1"]
        pub fn pktbuf_finish_packet(buf: *mut PktBuf);
        #[c2rust::src_loc = "76:1"]
        pub fn pktbuf_write_generic(
            buf: *mut PktBuf,
            type_0: ::core::ffi::c_int,
            fmt: *const ::core::ffi::c_char,
            ...
        );
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/dnslookup.h:23"]
pub mod dnslookup_h {
    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub type DNSToken;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/logging.h:23"]
pub mod logging_h {
    #[c2rust::src_loc = "47:1"]
    pub type LogLevel = ::core::ffi::c_uint;
    #[c2rust::src_loc = "54:2"]
    pub const LG_NOISE: LogLevel = 6;
    #[c2rust::src_loc = "53:2"]
    pub const LG_DEBUG: LogLevel = 5;
    #[c2rust::src_loc = "52:2"]
    pub const LG_INFO: LogLevel = 4;
    #[c2rust::src_loc = "51:2"]
    pub const LG_STATS: LogLevel = 3;
    #[c2rust::src_loc = "50:2"]
    pub const LG_WARNING: LogLevel = 2;
    #[c2rust::src_loc = "49:2"]
    pub const LG_ERROR: LogLevel = 1;
    #[c2rust::src_loc = "48:2"]
    pub const LG_FATAL: LogLevel = 0;
    extern "C" {
        #[c2rust::src_loc = "85:1"]
        pub static mut cf_verbose: ::core::ffi::c_int;
        #[c2rust::src_loc = "117:1"]
        pub fn log_generic(
            level: LogLevel,
            ctx: *mut ::core::ffi::c_void,
            s: *const ::core::ffi::c_char,
            ...
        );
        #[c2rust::src_loc = "120:1"]
        pub fn log_fatal(
            file: *const ::core::ffi::c_char,
            line: ::core::ffi::c_int,
            func: *const ::core::ffi::c_char,
            show_perror: bool,
            ctx: *mut ::core::ffi::c_void,
            s: *const ::core::ffi::c_char,
            ...
        );
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/scram.h:24"]
pub mod scram_h {
    #[c2rust::src_loc = "31:2"]
    pub const PASSWORD_TYPE_SCRAM_SHA_256: PasswordType = 2;
    #[c2rust::src_loc = "29:2"]
    pub const PASSWORD_TYPE_PLAINTEXT: PasswordType = 0;
    #[c2rust::src_loc = "28:9"]
    pub type PasswordType = ::core::ffi::c_uint;
    #[c2rust::src_loc = "30:2"]
    pub const PASSWORD_TYPE_MD5: PasswordType = 1;
    use super::bouncer_h::{PgCredentials, PgSocket, ScramState};
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn free_scram_state(state: *mut ScramState);
        #[c2rust::src_loc = "34:1"]
        pub fn get_password_type(shadow_pass: *const ::core::ffi::c_char) -> PasswordType;
        #[c2rust::src_loc = "40:1"]
        pub fn build_client_first_message(state: *mut ScramState) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "41:1"]
        pub fn build_client_final_message(
            server: *mut PgSocket,
            credentials: *const PgCredentials,
        ) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "44:1"]
        pub fn read_server_first_message(
            server: *mut PgSocket,
            input: *mut ::core::ffi::c_char,
        ) -> bool;
        #[c2rust::src_loc = "45:1"]
        pub fn read_server_final_message(
            server: *mut PgSocket,
            input: *mut ::core::ffi::c_char,
            ServerSignature: *mut ::core::ffi::c_char,
        ) -> bool;
        #[c2rust::src_loc = "47:1"]
        pub fn verify_server_signature(
            server: *mut PgSocket,
            credentials: *const PgCredentials,
            ServerSignature: *const ::core::ffi::c_char,
            match_0: *mut bool,
        ) -> bool;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:23"]
pub mod _malloc_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "54:1"]
        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "56:1"]
        pub fn free(_: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:23"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:23"]
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
        #[c2rust::src_loc = "80:1"]
        pub fn memmove(
            __dst: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __len: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "89:1"]
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "96:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
        #[c2rust::src_loc = "101:1"]
        pub fn strncmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
            __n: size_t,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/util.h:23"]
pub mod util_h {
    use super::_size_t_h::size_t;
    use super::_uint8_t_h::uint8_t;
    extern "C" {
        #[c2rust::src_loc = "44:1"]
        pub fn pg_md5_encrypt(
            part1: *const ::core::ffi::c_char,
            part2: *const ::core::ffi::c_char,
            p2len: size_t,
            dest: *mut ::core::ffi::c_char,
        ) -> bool;
        #[c2rust::src_loc = "45:1"]
        pub fn get_random_bytes(dest: *mut uint8_t, len: ::core::ffi::c_int);
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/client.h:23"]
pub mod client_h {
    use super::bouncer_h::PgSocket;
    extern "C" {
        #[c2rust::src_loc = "24:1"]
        pub fn sending_auth_query(client: *mut PgSocket) -> bool;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/server.h:23"]
pub mod server_h {
    use super::bouncer_h::PgPool;
    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub fn kill_pool_logins(
            pool: *mut PgPool,
            sqlstate: *const ::core::ffi::c_char,
            msg: *const ::core::ffi::c_char,
        );
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/objects.h:23"]
pub mod objects_h {
    use super::bouncer_h::{PgCredentials, PgSocket};
    extern "C" {
        #[c2rust::src_loc = "47:1"]
        pub fn find_global_credentials(name: *const ::core::ffi::c_char) -> *mut PgCredentials;
        #[c2rust::src_loc = "62:1"]
        pub fn disconnect_client(
            client: *mut PgSocket,
            notify: bool,
            reason: *const ::core::ffi::c_char,
            ...
        );
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/protocol.h:23"]
pub mod protocol_h {
    #[c2rust::src_loc = "31:9"]
    pub const PqMsg_PasswordMessage: ::core::ffi::c_int = 'p' as i32;
    #[c2rust::src_loc = "32:9"]
    pub const PqMsg_SASLInitialResponse: ::core::ffi::c_int = 'p' as i32;
    #[c2rust::src_loc = "33:9"]
    pub const PqMsg_SASLResponse: ::core::ffi::c_int = 'p' as i32;
    #[c2rust::src_loc = "44:9"]
    pub const PqMsg_ErrorResponse: ::core::ffi::c_int = 'E' as i32;
    #[c2rust::src_loc = "48:9"]
    pub const PqMsg_BackendKeyData: ::core::ffi::c_int = 'K' as i32;
    #[c2rust::src_loc = "50:9"]
    pub const PqMsg_AuthenticationRequest: ::core::ffi::c_int = 'R' as i32;
    #[c2rust::src_loc = "51:9"]
    pub const PqMsg_ParameterStatus: ::core::ffi::c_int = 'S' as i32;
    #[c2rust::src_loc = "55:9"]
    pub const PqMsg_ReadyForQuery: ::core::ffi::c_int = 'Z' as i32;
    #[c2rust::src_loc = "74:9"]
    pub const AUTH_REQ_OK: uint32_t = 0 as uint32_t;
    #[c2rust::src_loc = "77:9"]
    pub const AUTH_REQ_PASSWORD: uint32_t = 3 as uint32_t;
    #[c2rust::src_loc = "79:9"]
    pub const AUTH_REQ_MD5: uint32_t = 5 as uint32_t;
    #[c2rust::src_loc = "84:9"]
    pub const AUTH_REQ_SASL: uint32_t = 10 as uint32_t;
    #[c2rust::src_loc = "85:9"]
    pub const AUTH_REQ_SASL_CONT: uint32_t = 11 as uint32_t;
    #[c2rust::src_loc = "86:9"]
    pub const AUTH_REQ_SASL_FIN: uint32_t = 12 as uint32_t;
    use super::_uint32_t_h::uint32_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:23"]
pub mod _stdio_h {
    extern "C" {
        #[c2rust::src_loc = "282:1"]
        pub fn sscanf(
            _: *const ::core::ffi::c_char,
            _: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdlib.h:23"]
pub mod _stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "150:1"]
        pub fn atoi(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "153:1"]
        pub fn atoll(_: *const ::core::ffi::c_char) -> ::core::ffi::c_longlong;
        #[c2rust::src_loc = "160:1"]
        pub fn exit(_: ::core::ffi::c_int) -> !;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/llvm/21.1.8/lib/clang/21/include/stdbool.h:23"]
pub mod stdbool_h {
    #[c2rust::src_loc = "25:9"]
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::_in_addr_t_h::in_addr_t;
pub use self::_in_port_t_h::in_port_t;
pub use self::_int32_t_h::int32_t;
use self::_malloc_h::{free, malloc};
pub use self::_null_h::NULL;
pub use self::_pid_t_h::pid_t;
pub use self::_ptrdiff_t_h::ptrdiff_t;
pub use self::_sa_family_t_h::sa_family_t;
pub use self::_size_t_h::size_t;
pub use self::_ssize_t_h::ssize_t;
use self::_stdio_h::sscanf;
use self::_stdlib_h::{atoi, atoll, exit};
use self::_string_h::{memcpy, memmove, strcmp, strlen, strncmp};
pub use self::_timeval_h::timeval;
pub use self::_types_h::{
    __darwin_ptrdiff_t, __darwin_size_t, __darwin_ssize_t, __darwin_time_t, __darwin_va_list,
    __int32_t, __uint16_t, __uint32_t, __uint8_t,
};
pub use self::_uid_t_h::uid_t;
pub use self::_uint16_t_h::uint16_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::_va_list_h::va_list;
pub use self::aatree_h::{aatree_cmp_f, aatree_walker_f, AANode, AATree};
pub use self::bouncer_h::{
    cf_log_pooler_errors, cf_max_packet_size, cf_peer_id, first_socket,
    replication_type_parameters, sockaddr_ucreds, C2RustUnnamed_9, CallbackState, LoadBalanceHosts,
    PacketCallbackFlag, PgAddr, PgCredentials, PgDatabase, PgGlobalUser, PgPool, PgSocket, PgStats,
    ReplicationType, ScramState, SocketState, CANCELLATION_TTL_MASK, CB_HANDLE_COMPLETE_PACKET,
    CB_NONE, CB_WANT_COMPLETE_PACKET, CL_ACTIVE, CL_ACTIVE_CANCEL, CL_FREE, CL_JUSTFREE, CL_LOGIN,
    CL_WAITING, CL_WAITING_CANCEL, CL_WAITING_LOGIN, LOAD_BALANCE_HOSTS_DISABLE,
    LOAD_BALANCE_HOSTS_ROUND_ROBIN, PKT_CANCEL, PKT_GSSENCREQ, PKT_SSLREQ, PKT_STARTUP_V2,
    PKT_STARTUP_V3, PKT_STARTUP_V3_UNSUPPORTED, PKT_STARTUP_V4, REPLICATION_LOGICAL,
    REPLICATION_NONE, REPLICATION_PHYSICAL, SV_ACTIVE, SV_ACTIVE_CANCEL, SV_BEING_CANCELED,
    SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN, SV_TESTED, SV_USED,
};
use self::client_h::sending_auth_query;
pub use self::cryptohash_h::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};

pub use self::event_struct_h::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
pub use self::in6_h::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use self::in_h::{in_addr, sockaddr_in};
pub use self::internal::__builtin_va_list;
pub use self::iobuf_h::{iobuf, IOBuf};
pub use self::list_h::{list_empty, List};
pub use self::logging_h::{
    cf_verbose, log_fatal, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE,
    LG_STATS, LG_WARNING,
};
pub use self::mbuf_h::{
    mbuf_avail_for_read, mbuf_copy, mbuf_get_byte, mbuf_get_bytes, mbuf_get_chars, mbuf_get_string,
    mbuf_get_uint16be, mbuf_get_uint32be, mbuf_init_fixed_reader, mbuf_slice, MBuf,
};
use self::objects_h::{disconnect_client, find_global_credentials};
pub use self::pktbuf_h::{
    pktbuf_dynamic, pktbuf_finish_packet, pktbuf_put_bytes, pktbuf_put_string,
    pktbuf_send_immediate, pktbuf_start_packet, pktbuf_static, pktbuf_temp, pktbuf_write_generic,
    PktBuf,
};
pub use self::prepare_h::{
    PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement,
};
pub use self::proto_h::{PktHdr, NEW_HEADER_LEN, OLD_HEADER_LEN};
pub use self::protocol_h::{
    PqMsg_AuthenticationRequest, PqMsg_BackendKeyData, PqMsg_ErrorResponse, PqMsg_ParameterStatus,
    PqMsg_PasswordMessage, PqMsg_ReadyForQuery, PqMsg_SASLInitialResponse, PqMsg_SASLResponse,
    AUTH_REQ_MD5, AUTH_REQ_OK, AUTH_REQ_PASSWORD, AUTH_REQ_SASL, AUTH_REQ_SASL_CONT,
    AUTH_REQ_SASL_FIN,
};
pub use self::sbuf_h::{
    sbuf_cb_t, SBuf, SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK, SBUF_EV_FLUSH,
    SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY,
};
pub use self::scram_h::{
    build_client_final_message, build_client_first_message, free_scram_state, get_password_type,
    read_server_final_message, read_server_first_message, verify_server_signature, PasswordType,
    PASSWORD_TYPE_MD5, PASSWORD_TYPE_PLAINTEXT, PASSWORD_TYPE_SCRAM_SHA_256,
};
use self::server_h::kill_pool_logins;
pub use self::socket_h::sockaddr;
pub use self::statlist_h::{statlist_empty, StatList};
pub use self::stdbool_h::{false_0, true_0};
pub use self::strpool_h::{PStr, StrPool};
pub use self::sys__types_h::{__darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t, __DARWIN_NULL};
pub use self::time_h::usec_t;

pub use self::uthash_h::{UT_hash_bucket, UT_hash_handle, UT_hash_table};
use self::util_h::{get_random_bytes, pg_md5_encrypt};
pub use self::varcache_h::{
    varcache_add_params, varcache_apply_startup, varcache_fill_unset, varcache_set, VarCache,
};
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn get_header(mut data: *mut MBuf, mut pkt: *mut PktHdr) -> bool {
    let mut type_0: ::core::ffi::c_uint = 0;
    let mut len: uint32_t = 0;
    let mut got: ::core::ffi::c_uint = 0;
    let mut avail: ::core::ffi::c_uint = 0;
    let mut len16: uint16_t = 0;
    let mut type8: uint8_t = 0;
    let mut code: uint32_t = 0;
    let mut hdr = MBuf {
        data: ::core::ptr::null_mut::<uint8_t>(),
        read_pos: 0,
        write_pos: 0,
        alloc_len: 0,
        reader: false,
        fixed: false,
    };
    let mut ptr = ::core::ptr::null::<uint8_t>();
    mbuf_copy(data, &raw mut hdr);
    if mbuf_avail_for_read(&raw mut hdr) < NEW_HEADER_LEN as ::core::ffi::c_uint {
        let mut _log_ctx = NULL;
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                _log_ctx,
                b"get_header: less than %d bytes available\0" as *const u8
                    as *const ::core::ffi::c_char,
                5 as ::core::ffi::c_int,
            );
        }
        return false_0 != 0;
    }
    if !mbuf_get_byte(&raw mut hdr, &raw mut type8) {
        return false_0 != 0;
    }
    type_0 = type8 as ::core::ffi::c_uint;
    if type_0 != 0 as ::core::ffi::c_uint {
        if !mbuf_get_uint32be(&raw mut hdr, &raw mut len) {
            return false_0 != 0;
        }
        len = len.wrapping_add(1);
        got = NEW_HEADER_LEN as ::core::ffi::c_uint;
    } else {
        if !mbuf_get_byte(&raw mut hdr, &raw mut type8) {
            return false_0 != 0;
        }
        if type8 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            let mut _log_ctx_0 = NULL;
            if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_NOISE,
                    _log_ctx_0,
                    b"get_header: unknown special pkt\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            return false_0 != 0;
        }
        if mbuf_avail_for_read(&raw mut hdr)
            < (OLD_HEADER_LEN - 2 as ::core::ffi::c_int) as ::core::ffi::c_uint
        {
            let mut _log_ctx_1 = NULL;
            if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_NOISE,
                    _log_ctx_1,
                    b"get_header: less than %d bytes for special pkt\0" as *const u8
                        as *const ::core::ffi::c_char,
                    8 as ::core::ffi::c_int,
                );
            }
            return false_0 != 0;
        }
        if !mbuf_get_uint16be(&raw mut hdr, &raw mut len16) {
            return false_0 != 0;
        }
        len = len16 as uint32_t;
        if !mbuf_get_uint32be(&raw mut hdr, &raw mut code) {
            return false_0 != 0;
        }
        if code == PKT_CANCEL as uint32_t {
            type_0 = PKT_CANCEL as ::core::ffi::c_uint;
        } else if code == PKT_SSLREQ as uint32_t {
            type_0 = PKT_SSLREQ as ::core::ffi::c_uint;
        } else if code == PKT_GSSENCREQ as uint32_t {
            type_0 = PKT_GSSENCREQ as ::core::ffi::c_uint;
        } else if code >= PKT_STARTUP_V3 as uint32_t
            && code < PKT_STARTUP_V3_UNSUPPORTED as uint32_t
        {
            type_0 = PKT_STARTUP_V3 as ::core::ffi::c_uint;
        } else if code >= PKT_STARTUP_V3_UNSUPPORTED as uint32_t
            && code < PKT_STARTUP_V4 as uint32_t
        {
            type_0 = PKT_STARTUP_V3_UNSUPPORTED as ::core::ffi::c_uint;
        } else if code == PKT_STARTUP_V2 as uint32_t {
            type_0 = PKT_STARTUP_V2 as ::core::ffi::c_uint;
        } else {
            let mut _log_ctx_2 = NULL;
            if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_NOISE,
                    _log_ctx_2,
                    b"get_header: unknown special pkt: len=%u code=%u\0" as *const u8
                        as *const ::core::ffi::c_char,
                    len,
                    code,
                );
            }
            return false_0 != 0;
        }
        got = OLD_HEADER_LEN as ::core::ffi::c_uint;
    }
    if len < got as uint32_t || len > cf_max_packet_size as uint32_t {
        return false_0 != 0;
    }
    (*pkt).type_0 = type_0;
    (*pkt).len = len as ::core::ffi::c_uint;
    if len > mbuf_avail_for_read(data) as uint32_t {
        avail = mbuf_avail_for_read(data);
    } else {
        avail = len as ::core::ffi::c_uint;
    }
    if !mbuf_slice(data, avail, &raw mut (*pkt).data) {
        return false_0 != 0;
    }
    return mbuf_get_bytes(&raw mut (*pkt).data, got, &raw mut ptr);
}
#[no_mangle]
#[c2rust::src_loc = "153:1"]
pub unsafe extern "C" fn send_pooler_error(
    mut client: *mut PgSocket,
    mut send_ready: bool,
    mut sqlstate: *const ::core::ffi::c_char,
    mut level_fatal: bool,
    mut msg: *const ::core::ffi::c_char,
) -> bool {
    let mut tmpbuf: [uint8_t; 512] = [0; 512];
    let mut buf = PktBuf {
        buf: ::core::ptr::null_mut::<uint8_t>(),
        buf_len: 0,
        write_pos: 0,
        pktlen_pos: 0,
        send_pos: 0,
        ev: ::core::ptr::null_mut::<event>(),
        queued_dst: ::core::ptr::null_mut::<PgSocket>(),
        failed_sending_fixed_buf: [0; 1],
        c2rust_padding: [0; 7],
    };
    if cf_log_pooler_errors != 0 {
        log_generic(
            LG_WARNING,
            client as *mut ::core::ffi::c_void,
            b"pooler error: %s\0" as *const u8 as *const ::core::ffi::c_char,
            msg,
        );
    }
    pktbuf_static(
        &raw mut buf,
        &raw mut tmpbuf as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 512]>() as ::core::ffi::c_int,
    );
    pktbuf_write_generic(
        &raw mut buf,
        PqMsg_ErrorResponse,
        b"cscscsc\0" as *const u8 as *const ::core::ffi::c_char,
        'S' as i32,
        if level_fatal as ::core::ffi::c_int != 0 {
            b"FATAL\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"ERROR\0" as *const u8 as *const ::core::ffi::c_char
        },
        'C' as i32,
        if !sqlstate.is_null() {
            sqlstate
        } else {
            b"08P01\0" as *const u8 as *const ::core::ffi::c_char
        },
        'M' as i32,
        msg,
        0 as ::core::ffi::c_int,
    );
    if send_ready {
        pktbuf_write_generic(
            &raw mut buf,
            PqMsg_ReadyForQuery,
            b"c\0" as *const u8 as *const ::core::ffi::c_char,
            'I' as i32,
        );
    }
    return pktbuf_send_immediate(&raw mut buf, client);
}
#[no_mangle]
#[c2rust::src_loc = "173:1"]
pub unsafe extern "C" fn parse_server_error(
    mut pkt: *mut PktHdr,
    mut level_p: *mut *const ::core::ffi::c_char,
    mut msg_p: *mut *const ::core::ffi::c_char,
    mut sqlstate_p: *mut *const ::core::ffi::c_char,
) {
    let mut level = ::core::ptr::null::<::core::ffi::c_char>();
    let mut msg = ::core::ptr::null::<::core::ffi::c_char>();
    let mut sqlstate = ::core::ptr::null::<::core::ffi::c_char>();
    let mut val = ::core::ptr::null::<::core::ffi::c_char>();
    let mut type_0: uint8_t = 0;
    while mbuf_avail_for_read(&raw mut (*pkt).data) != 0 {
        if !mbuf_get_byte(&raw mut (*pkt).data, &raw mut type_0) {
            break;
        }
        if type_0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            break;
        }
        if !mbuf_get_string(&raw mut (*pkt).data, &raw mut val) {
            break;
        }
        if type_0 as ::core::ffi::c_int == 'S' as i32 {
            level = val;
        } else if type_0 as ::core::ffi::c_int == 'M' as i32 {
            msg = val;
        } else if type_0 as ::core::ffi::c_int == 'C' as i32 {
            sqlstate = val;
        }
    }
    *level_p = level;
    *msg_p = msg;
    *sqlstate_p = sqlstate;
}
#[no_mangle]
#[c2rust::src_loc = "197:1"]
pub unsafe extern "C" fn log_server_error(
    mut note: *const ::core::ffi::c_char,
    mut pkt: *mut PktHdr,
) {
    let mut level = ::core::ptr::null::<::core::ffi::c_char>();
    let mut msg = ::core::ptr::null::<::core::ffi::c_char>();
    let mut sqlstate = ::core::ptr::null::<::core::ffi::c_char>();
    parse_server_error(pkt, &raw mut level, &raw mut msg, &raw mut sqlstate);
    if msg.is_null() || level.is_null() {
        let mut _log_ctx = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx,
            b"%s: partial error message, cannot log\0" as *const u8 as *const ::core::ffi::c_char,
            note,
        );
    } else {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx_0,
            b"%s: %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
            note,
            level,
            msg,
        );
    };
}
#[no_mangle]
#[c2rust::src_loc = "216:1"]
pub unsafe extern "C" fn add_welcome_parameter(
    mut pool: *mut PgPool,
    mut key: *const ::core::ffi::c_char,
    mut val: *const ::core::ffi::c_char,
) -> bool {
    let mut msg = (*pool).welcome_msg as *mut PktBuf;
    if (*pool).welcome_msg_ready() {
        return true_0 != 0;
    }
    if msg.is_null() {
        msg = pktbuf_dynamic(128 as ::core::ffi::c_int);
        if msg.is_null() {
            return false_0 != 0;
        }
        (*pool).welcome_msg = msg as *mut PktBuf;
    }
    if (*msg).write_pos == 0 as ::core::ffi::c_int {
        pktbuf_write_generic(
            msg,
            PqMsg_AuthenticationRequest,
            b"i\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
    }
    if !varcache_set(&raw mut (*pool).orig_vars, key, val) {
        pktbuf_write_generic(
            msg,
            PqMsg_ParameterStatus,
            b"ss\0" as *const u8 as *const ::core::ffi::c_char,
            key,
            val,
        );
    }
    return !(*msg).failed();
}
#[no_mangle]
#[c2rust::src_loc = "242:1"]
pub unsafe extern "C" fn finish_welcome_msg(mut server: *mut PgSocket) {
    let mut pool = (*server).pool;
    if (*pool).welcome_msg_ready() {
        return;
    }
    (*pool).set_welcome_msg_ready((true_0 != 0) as bool);
}
#[no_mangle]
#[c2rust::src_loc = "250:1"]
pub unsafe extern "C" fn welcome_client(mut client: *mut PgSocket) -> bool {
    let mut res: ::core::ffi::c_int = 0;
    let mut pool = (*client).pool;
    let mut pmsg: *const PktBuf = (*pool).welcome_msg;
    let mut msg = ::core::ptr::null_mut::<PktBuf>();
    if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_NOISE,
            client as *mut ::core::ffi::c_void,
            b"P: welcome_client\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    msg = pktbuf_temp() as *mut PktBuf;
    pktbuf_put_bytes(
        msg,
        (*pmsg).buf as *const ::core::ffi::c_void,
        (*pmsg).write_pos,
    );
    varcache_fill_unset(&raw mut (*pool).orig_vars, client);
    varcache_add_params(msg, &raw mut (*client).vars);
    get_random_bytes(
        &raw mut (*client).cancel_key as *mut uint8_t,
        8 as ::core::ffi::c_int,
    );
    if cf_peer_id > 0 as ::core::ffi::c_int {
        (*client).cancel_key[1 as ::core::ffi::c_int as usize] =
            (cf_peer_id & 0xff as ::core::ffi::c_int) as uint8_t;
        (*client).cancel_key[2 as ::core::ffi::c_int as usize] =
            (cf_peer_id >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
        (*client).cancel_key[7 as ::core::ffi::c_int as usize] =
            ((*client).cancel_key[7 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                | CANCELLATION_TTL_MASK) as uint8_t;
    }
    (*client).cancel_key[0 as ::core::ffi::c_int as usize] =
        ((*client).cancel_key[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            & 0x7f as ::core::ffi::c_int) as uint8_t;
    pktbuf_write_generic(
        msg,
        PqMsg_BackendKeyData,
        b"b\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*client).cancel_key as *mut uint8_t,
        8 as ::core::ffi::c_int,
    );
    pktbuf_write_generic(
        msg,
        PqMsg_ReadyForQuery,
        b"c\0" as *const u8 as *const ::core::ffi::c_char,
        'I' as i32,
    );
    if (*msg).failed() {
        disconnect_client(
            client,
            true_0 != 0,
            b"failed to prepare welcome message\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    res = pktbuf_send_immediate(msg, client) as ::core::ffi::c_int;
    if res == 0 {
        disconnect_client(
            client,
            true_0 != 0,
            b"failed to send welcome message\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[c2rust::src_loc = "327:1"]
unsafe extern "C" fn get_srv_psw(mut server: *mut PgSocket) -> *mut PgCredentials {
    let mut db = (*(*server).pool).db;
    let mut credentials = (*(*server).pool).user_credentials;
    if (*credentials).passwd[0 as ::core::ffi::c_int as usize] == 0
        && !(*db).forced_user_credentials.is_null()
    {
        let mut c2 =
            find_global_credentials(&raw mut (*credentials).name as *mut ::core::ffi::c_char);
        if !c2.is_null() {
            return c2;
        }
    }
    return credentials;
}
#[c2rust::src_loc = "342:1"]
unsafe extern "C" fn send_password(
    mut server: *mut PgSocket,
    mut enc_psw: *const ::core::ffi::c_char,
) -> bool {
    let mut res: bool = false;
    let mut _data: [uint8_t; 2056] = [0; 2056];
    let mut _buf = PktBuf {
        buf: ::core::ptr::null_mut::<uint8_t>(),
        buf_len: 0,
        write_pos: 0,
        pktlen_pos: 0,
        send_pos: 0,
        ev: ::core::ptr::null_mut::<event>(),
        queued_dst: ::core::ptr::null_mut::<PgSocket>(),
        failed_sending_fixed_buf: [0; 1],
        c2rust_padding: [0; 7],
    };
    pktbuf_static(
        &raw mut _buf,
        &raw mut _data as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 2056]>() as ::core::ffi::c_int,
    );
    pktbuf_write_generic(
        &raw mut _buf,
        PqMsg_PasswordMessage,
        b"s\0" as *const u8 as *const ::core::ffi::c_char,
        enc_psw,
    );
    res = pktbuf_send_immediate(&raw mut _buf, server);
    return res;
}
#[c2rust::src_loc = "349:1"]
unsafe extern "C" fn login_clear_psw(mut server: *mut PgSocket) -> bool {
    let mut credentials = get_srv_psw(server);
    if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_DEBUG,
            server as *mut ::core::ffi::c_void,
            b"P: send clear password\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return send_password(
        server,
        &raw mut (*credentials).passwd as *mut ::core::ffi::c_char,
    );
}
#[c2rust::src_loc = "356:1"]
unsafe extern "C" fn login_md5_psw(mut server: *mut PgSocket, mut salt: *const uint8_t) -> bool {
    let mut txt: [::core::ffi::c_char; 36] = [0; 36];
    let mut src = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut credentials = get_srv_psw(server);
    if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_DEBUG,
            server as *mut ::core::ffi::c_void,
            b"P: send md5 password\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    match get_password_type(&raw mut (*credentials).passwd as *mut ::core::ffi::c_char)
        as ::core::ffi::c_uint
    {
        0 => {
            if !pg_md5_encrypt(
                &raw mut (*credentials).passwd as *mut ::core::ffi::c_char,
                &raw mut (*credentials).name as *mut ::core::ffi::c_char,
                strlen(&raw mut (*credentials).name as *mut ::core::ffi::c_char),
                &raw mut txt as *mut ::core::ffi::c_char,
            ) {
                return false_0 != 0;
            }
            src =
                (&raw mut txt as *mut ::core::ffi::c_char).offset(3 as ::core::ffi::c_int as isize);
        }
        1 => {
            src = (&raw mut (*credentials).passwd as *mut ::core::ffi::c_char)
                .offset(3 as ::core::ffi::c_int as isize);
        }
        _ => {
            log_generic(
                LG_ERROR,
                server as *mut ::core::ffi::c_void,
                b"cannot do MD5 authentication: wrong password type\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            kill_pool_logins(
                (*server).pool,
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"server login failed: wrong password type\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
    }
    if !pg_md5_encrypt(
        src,
        salt as *mut ::core::ffi::c_char,
        4 as size_t,
        &raw mut txt as *mut ::core::ffi::c_char,
    ) {
        return false_0 != 0;
    }
    return send_password(server, &raw mut txt as *mut ::core::ffi::c_char);
}
#[c2rust::src_loc = "384:1"]
unsafe extern "C" fn login_scram_sha_256(mut server: *mut PgSocket) -> bool {
    let mut credentials = get_srv_psw(server);
    let mut res: bool = false;
    let mut client_first_message = ::core::ptr::null_mut::<::core::ffi::c_char>();
    match get_password_type(&raw mut (*credentials).passwd as *mut ::core::ffi::c_char)
        as ::core::ffi::c_uint
    {
        0 => {}
        2 => {
            if !(*credentials).use_scram_keys {
                log_generic(
                    LG_ERROR,
                    server as *mut ::core::ffi::c_void,
                    b"cannot do SCRAM authentication: password is SCRAM secret but client authentication did not provide SCRAM keys\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                kill_pool_logins(
                    (*server).pool,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"server login failed: wrong password type\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return false_0 != 0;
            }
        }
        _ => {
            log_generic(
                LG_ERROR,
                server as *mut ::core::ffi::c_void,
                b"cannot do SCRAM authentication: wrong password type\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            kill_pool_logins(
                (*server).pool,
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"server login failed: wrong password type\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
    }
    if !(*server).scram_state.client_nonce.is_null() {
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            b"protocol error: duplicate AuthenticationSASL message from server\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    client_first_message = build_client_first_message(&raw mut (*server).scram_state);
    if client_first_message.is_null() {
        return false_0 != 0;
    }
    if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_DEBUG,
            server as *mut ::core::ffi::c_void,
            b"SCRAM client-first-message = \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            client_first_message,
        );
    }
    if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_DEBUG,
            server as *mut ::core::ffi::c_void,
            b"P: send SASLInitialResponse\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _data: [uint8_t; 512] = [0; 512];
    let mut _buf = PktBuf {
        buf: ::core::ptr::null_mut::<uint8_t>(),
        buf_len: 0,
        write_pos: 0,
        pktlen_pos: 0,
        send_pos: 0,
        ev: ::core::ptr::null_mut::<event>(),
        queued_dst: ::core::ptr::null_mut::<PgSocket>(),
        failed_sending_fixed_buf: [0; 1],
        c2rust_padding: [0; 7],
    };
    pktbuf_static(
        &raw mut _buf,
        &raw mut _data as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 512]>() as ::core::ffi::c_int,
    );
    pktbuf_write_generic(
        &raw mut _buf,
        PqMsg_SASLInitialResponse,
        b"sib\0" as *const u8 as *const ::core::ffi::c_char,
        b"SCRAM-SHA-256\0" as *const u8 as *const ::core::ffi::c_char,
        strlen(client_first_message),
        client_first_message,
        strlen(client_first_message),
    );
    res = pktbuf_send_immediate(&raw mut _buf, server);
    free(client_first_message as *mut ::core::ffi::c_void);
    return res;
}
#[c2rust::src_loc = "424:1"]
unsafe extern "C" fn login_scram_sha_256_cont(
    mut server: *mut PgSocket,
    mut datalen: ::core::ffi::c_uint,
    mut data: *const uint8_t,
) -> bool {
    let mut credentials = get_srv_psw(server);
    let mut ibuf = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut input = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut res: bool = false;
    let mut client_final_message = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*server).scram_state.client_nonce.is_null() {
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            b"protocol error: AuthenticationSASLContinue without prior AuthenticationSASL\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    if !(*server).scram_state.server_first_message.is_null() {
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            b"SCRAM exchange protocol error: received second AuthenticationSASLContinue\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    ibuf = malloc(datalen.wrapping_add(1 as ::core::ffi::c_uint) as size_t)
        as *mut ::core::ffi::c_char;
    if ibuf.is_null() {
        return false_0 != 0;
    }
    memcpy(
        ibuf as *mut ::core::ffi::c_void,
        data as *const ::core::ffi::c_void,
        datalen as size_t,
    );
    *ibuf.offset(datalen as isize) = '\0' as i32 as ::core::ffi::c_char;
    input = ibuf;
    if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_DEBUG,
            server as *mut ::core::ffi::c_void,
            b"SCRAM server-first-message = \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            input,
        );
    }
    if !read_server_first_message(server, input) {
        free(ibuf as *mut ::core::ffi::c_void);
        free(client_final_message as *mut ::core::ffi::c_void);
        return false_0 != 0;
    } else {
        client_final_message = build_client_final_message(server, credentials);
        free(ibuf as *mut ::core::ffi::c_void);
        if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_DEBUG,
                server as *mut ::core::ffi::c_void,
                b"SCRAM client-final-message = \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
                client_final_message,
            );
        }
        if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_DEBUG,
                server as *mut ::core::ffi::c_void,
                b"P: send SASLResponse\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        let mut _data: [uint8_t; 512] = [0; 512];
        let mut _buf = PktBuf {
            buf: ::core::ptr::null_mut::<uint8_t>(),
            buf_len: 0,
            write_pos: 0,
            pktlen_pos: 0,
            send_pos: 0,
            ev: ::core::ptr::null_mut::<event>(),
            queued_dst: ::core::ptr::null_mut::<PgSocket>(),
            failed_sending_fixed_buf: [0; 1],
            c2rust_padding: [0; 7],
        };
        pktbuf_static(
            &raw mut _buf,
            &raw mut _data as *mut uint8_t,
            ::core::mem::size_of::<[uint8_t; 512]>() as ::core::ffi::c_int,
        );
        pktbuf_write_generic(
            &raw mut _buf,
            PqMsg_SASLResponse,
            b"b\0" as *const u8 as *const ::core::ffi::c_char,
            client_final_message,
            strlen(client_final_message),
        );
        res = pktbuf_send_immediate(&raw mut _buf, server);
        free(client_final_message as *mut ::core::ffi::c_void);
        return res;
    };
}
#[c2rust::src_loc = "469:1"]
unsafe extern "C" fn login_scram_sha_256_final(
    mut server: *mut PgSocket,
    mut datalen: ::core::ffi::c_uint,
    mut data: *const uint8_t,
) -> bool {
    let mut credentials = get_srv_psw(server);
    let mut ibuf = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut input = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ServerSignature: [::core::ffi::c_char; 32] = [0; 32];
    let mut match_0 = false_0 != 0;
    if (*server).scram_state.server_first_message.is_null() {
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            b"protocol error: AuthenticationSASLFinal without prior AuthenticationSASLContinue\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    ibuf = malloc(datalen.wrapping_add(1 as ::core::ffi::c_uint) as size_t)
        as *mut ::core::ffi::c_char;
    if ibuf.is_null() {
        return false_0 != 0;
    }
    memcpy(
        ibuf as *mut ::core::ffi::c_void,
        data as *const ::core::ffi::c_void,
        datalen as size_t,
    );
    *ibuf.offset(datalen as isize) = '\0' as i32 as ::core::ffi::c_char;
    input = ibuf;
    if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_DEBUG,
            server as *mut ::core::ffi::c_void,
            b"SCRAM server-final-message = \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            input,
        );
    }
    if read_server_final_message(
        server,
        input,
        &raw mut ServerSignature as *mut ::core::ffi::c_char,
    ) {
        if !verify_server_signature(
            server,
            credentials,
            &raw mut ServerSignature as *mut ::core::ffi::c_char,
            &raw mut match_0,
        ) {
            kill_pool_logins(
                (*server).pool,
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"server login failed: failed to verify server signature\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        } else if !match_0 {
            log_generic(
                LG_ERROR,
                server as *mut ::core::ffi::c_void,
                b"invalid server signature\0" as *const u8 as *const ::core::ffi::c_char,
            );
            kill_pool_logins(
                (*server).pool,
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"server login failed: invalid server signature\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        } else {
            free(ibuf as *mut ::core::ffi::c_void);
            return true_0 != 0;
        }
    }
    free(ibuf as *mut ::core::ffi::c_void);
    return false_0 != 0;
}
#[no_mangle]
#[c2rust::src_loc = "512:1"]
pub unsafe extern "C" fn answer_authreq(mut server: *mut PgSocket, mut pkt: *mut PktHdr) -> bool {
    let mut cmd: uint32_t = 0;
    let mut salt = ::core::ptr::null::<uint8_t>();
    let mut res = false_0 != 0;
    if mbuf_avail_for_read(&raw mut (*pkt).data) < 4 as ::core::ffi::c_uint {
        return false_0 != 0;
    }
    if !mbuf_get_uint32be(&raw mut (*pkt).data, &raw mut cmd) {
        return false_0 != 0;
    }
    match cmd {
        0 => {
            if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_DEBUG,
                    server as *mut ::core::ffi::c_void,
                    b"S: auth ok\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            res = true_0 != 0;
        }
        3 => {
            if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_DEBUG,
                    server as *mut ::core::ffi::c_void,
                    b"S: req cleartext password\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            res = login_clear_psw(server);
        }
        5 => {
            if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_DEBUG,
                    server as *mut ::core::ffi::c_void,
                    b"S: req md5-crypted psw\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if !mbuf_get_bytes(
                &raw mut (*pkt).data,
                4 as ::core::ffi::c_uint,
                &raw mut salt,
            ) {
                return false_0 != 0;
            }
            res = login_md5_psw(server, salt);
        }
        10 => {
            let mut selected_mechanism = false_0 != 0;
            if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_DEBUG,
                    server as *mut ::core::ffi::c_void,
                    b"S: req SASL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            loop {
                let mut mech = ::core::ptr::null::<::core::ffi::c_char>();
                if !mbuf_get_string(&raw mut (*pkt).data, &raw mut mech) {
                    return false_0 != 0;
                }
                if *mech.offset(0 as ::core::ffi::c_int as isize) == 0 {
                    break;
                }
                if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0
                {
                    log_generic(
                        LG_DEBUG,
                        server as *mut ::core::ffi::c_void,
                        b"S: SASL advertised mechanism: %s\0" as *const u8
                            as *const ::core::ffi::c_char,
                        mech,
                    );
                }
                if strcmp(
                    mech,
                    b"SCRAM-SHA-256\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    selected_mechanism = true_0 != 0;
                }
                if selected_mechanism {
                    break;
                }
            }
            if !selected_mechanism {
                log_generic(
                    LG_ERROR,
                    server as *mut ::core::ffi::c_void,
                    b"none of the server's SASL authentication mechanisms are supported\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                kill_pool_logins(
                    (*server).pool,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"server login failed: none of the server's SASL authentication mechanisms are supported\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
                res = login_scram_sha_256(server);
            }
        }
        11 => {
            let mut len: ::core::ffi::c_uint = 0;
            let mut data = ::core::ptr::null::<uint8_t>();
            if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_DEBUG,
                    server as *mut ::core::ffi::c_void,
                    b"S: SASL cont\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            len = mbuf_avail_for_read(&raw mut (*pkt).data);
            if !mbuf_get_bytes(&raw mut (*pkt).data, len, &raw mut data) {
                return false_0 != 0;
            }
            res = login_scram_sha_256_cont(server, len, data);
        }
        12 => {
            let mut len_0: ::core::ffi::c_uint = 0;
            let mut data_0 = ::core::ptr::null::<uint8_t>();
            if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_DEBUG,
                    server as *mut ::core::ffi::c_void,
                    b"S: SASL final\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            len_0 = mbuf_avail_for_read(&raw mut (*pkt).data);
            if !mbuf_get_bytes(&raw mut (*pkt).data, len_0, &raw mut data_0) {
                return false_0 != 0;
            }
            res = login_scram_sha_256_final(server, len_0, data_0);
            free_scram_state(&raw mut (*server).scram_state);
        }
        _ => {
            log_generic(
                LG_ERROR,
                server as *mut ::core::ffi::c_void,
                b"unknown/unsupported auth method: %u\0" as *const u8 as *const ::core::ffi::c_char,
                cmd,
            );
            res = false_0 != 0;
        }
    }
    return res;
}
#[no_mangle]
#[c2rust::src_loc = "598:1"]
pub unsafe extern "C" fn send_startup_packet(mut server: *mut PgSocket) -> bool {
    let mut pool = (*server).pool;
    let mut db = (*pool).db;
    let mut username: *const ::core::ffi::c_char =
        &raw mut (*(*(*server).pool).user_credentials).name as *mut ::core::ffi::c_char;
    let mut pkt = pktbuf_temp() as *mut PktBuf;
    let mut client = ::core::ptr::null_mut::<PgSocket>();
    pktbuf_start_packet(pkt, PKT_STARTUP_V3);
    pktbuf_put_bytes(
        pkt,
        (*(*db).startup_params).buf as *const ::core::ffi::c_void,
        (*(*db).startup_params).write_pos,
    );
    client = first_socket(&raw mut (*pool).waiting_client_list);
    if !client.is_null()
        && (*client).replication as ::core::ffi::c_uint != 0
        && !sending_auth_query(client)
    {
        (*server).replication = (*client).replication;
        pktbuf_put_string(
            pkt,
            b"replication\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_DEBUG,
                server as *mut ::core::ffi::c_void,
                b"send_startup_packet: creating replication connection\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        pktbuf_put_string(
            pkt,
            replication_type_parameters[(*server).replication as usize],
        );
        varcache_apply_startup(pkt, client);
        if !(*client).startup_options.is_null() {
            pktbuf_put_string(pkt, b"options\0" as *const u8 as *const ::core::ffi::c_char);
            pktbuf_put_string(pkt, (*client).startup_options);
        }
    }
    pktbuf_put_string(pkt, b"user\0" as *const u8 as *const ::core::ffi::c_char);
    pktbuf_put_string(pkt, username);
    pktbuf_put_string(pkt, b"\0" as *const u8 as *const ::core::ffi::c_char);
    pktbuf_finish_packet(pkt);
    if !pktbuf_send_immediate(pkt, server) {
        return false_0 != 0;
    }
    if (*server).replication as u64 != 0 {
        (*client).link = server;
        (*server).link = client;
    }
    return true_0 != 0;
}
#[no_mangle]
#[c2rust::src_loc = "665:1"]
pub unsafe extern "C" fn send_sslreq_packet(mut server: *mut PgSocket) -> bool {
    let mut res: ::core::ffi::c_int = 0;
    let mut _data: [uint8_t; 16] = [0; 16];
    let mut _buf = PktBuf {
        buf: ::core::ptr::null_mut::<uint8_t>(),
        buf_len: 0,
        write_pos: 0,
        pktlen_pos: 0,
        send_pos: 0,
        ev: ::core::ptr::null_mut::<event>(),
        queued_dst: ::core::ptr::null_mut::<PgSocket>(),
        failed_sending_fixed_buf: [0; 1],
        c2rust_padding: [0; 7],
    };
    pktbuf_static(
        &raw mut _buf,
        &raw mut _data as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 16]>() as ::core::ffi::c_int,
    );
    pktbuf_write_generic(
        &raw mut _buf,
        PKT_SSLREQ,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
    );
    res = pktbuf_send_immediate(&raw mut _buf, server) as ::core::ffi::c_int;
    return res != 0;
}
#[no_mangle]
#[c2rust::src_loc = "681:1"]
pub unsafe extern "C" fn scan_text_result(
    mut pkt: *mut MBuf,
    mut tupdesc: *const ::core::ffi::c_char,
    mut args: ...
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ncol: uint16_t = 0;
    let mut asked: ::core::ffi::c_uint = 0;
    let mut ap: ::core::ffi::VaListImpl;
    asked = strlen(tupdesc) as ::core::ffi::c_uint;
    if !mbuf_get_uint16be(pkt, &raw mut ncol) {
        return -(1 as ::core::ffi::c_int);
    }
    ap = args.clone();
    let mut i = 0 as ::core::ffi::c_uint;
    loop {
        if !(i < asked) {
            current_block = 11793792312832361944;
            break;
        }
        let mut val = ::core::ptr::null::<::core::ffi::c_char>();
        let mut len: uint32_t = 0;
        if i < ncol as ::core::ffi::c_uint {
            if !mbuf_get_uint32be(pkt, &raw mut len) {
                current_block = 2516253395664191498;
                break;
            }
            if (len as int32_t) < 0 as int32_t {
                val = ::core::ptr::null::<::core::ffi::c_char>();
            } else if !mbuf_get_chars(pkt, len as ::core::ffi::c_uint, &raw mut val) {
                current_block = 2516253395664191498;
                break;
            }
            if !val.is_null() {
                let mut xval =
                    (val as *mut ::core::ffi::c_char).offset(-(1 as ::core::ffi::c_int as isize));
                memmove(
                    xval as *mut ::core::ffi::c_void,
                    val as *const ::core::ffi::c_void,
                    len as size_t,
                );
                *xval.offset(len as isize) = 0 as ::core::ffi::c_char;
                val = xval;
            }
        } else {
            val = ::core::ptr::null::<::core::ffi::c_char>();
            len = -(1 as ::core::ffi::c_int) as uint32_t;
        }
        match *tupdesc.offset(i as isize) as ::core::ffi::c_int {
            105 => {
                let mut int_p = ::core::ptr::null_mut::<::core::ffi::c_int>();
                int_p = ap.arg::<*mut ::core::ffi::c_int>();
                *int_p = if !val.is_null() {
                    atoi(val)
                } else {
                    0 as ::core::ffi::c_int
                };
            }
            113 => {
                let mut long_p = ::core::ptr::null_mut::<uint64_t>();
                long_p = ap.arg::<*mut uint64_t>();
                *long_p = (if !val.is_null() {
                    atoll(val)
                } else {
                    0 as ::core::ffi::c_longlong
                }) as uint64_t;
            }
            115 => {
                let mut str_p = ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
                str_p = ap.arg::<*mut *const ::core::ffi::c_char>();
                *str_p = val;
            }
            98 => {
                let mut len_p = ap.arg::<*mut ::core::ffi::c_int>();
                let mut bytes_p = ap.arg::<*mut *mut uint8_t>();
                if !val.is_null() {
                    let mut newlen: ::core::ffi::c_int = 0;
                    if strncmp(
                        val,
                        b"\\x\0" as *const u8 as *const ::core::ffi::c_char,
                        2 as size_t,
                    ) != 0 as ::core::ffi::c_int
                    {
                        let mut _log_ctx = NULL;
                        log_generic(
                            LG_WARNING,
                            _log_ctx,
                            b"invalid bytea value\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        current_block = 2516253395664191498;
                        break;
                    } else {
                        newlen = len.wrapping_sub(2 as uint32_t).wrapping_div(2 as uint32_t)
                            as ::core::ffi::c_int;
                        *len_p = newlen;
                        *bytes_p = malloc(newlen as size_t) as *mut uint8_t;
                        if (*bytes_p).is_null() {
                            current_block = 2516253395664191498;
                            break;
                        }
                        let mut j = 0 as ::core::ffi::c_int;
                        while j < newlen {
                            let mut b: ::core::ffi::c_uint = 0;
                            sscanf(
                                val.offset(2 as ::core::ffi::c_int as isize)
                                    .offset((2 as ::core::ffi::c_int * j) as isize),
                                b"%2x\0" as *const u8 as *const ::core::ffi::c_char,
                                &raw mut b,
                            );
                            *(*bytes_p).offset(j as isize) = b as uint8_t;
                            j += 1;
                        }
                    }
                } else {
                    *len_p = -(1 as ::core::ffi::c_int);
                    *bytes_p = ::core::ptr::null_mut::<uint8_t>();
                }
            }
            _ => {
                let mut _log_ctx_0 = NULL;
                log_fatal(
                    b"src/proto.c\0" as *const u8 as *const ::core::ffi::c_char,
                    772 as ::core::ffi::c_int,
                    b"scan_text_result\0" as *const u8 as *const ::core::ffi::c_char,
                    false_0 != 0,
                    _log_ctx_0,
                    b"bad tupdesc: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    tupdesc,
                );
                exit(1 as ::core::ffi::c_int);
            }
        }
        i = i.wrapping_add(1);
    }
    match current_block {
        11793792312832361944 => return ncol as ::core::ffi::c_int,
        _ => return -(1 as ::core::ffi::c_int),
    };
}
