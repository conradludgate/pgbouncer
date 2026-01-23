#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:1"]
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
    #[c2rust::src_loc = "118:1"]
    pub type __darwin_ssize_t = isize;
    #[c2rust::src_loc = "119:1"]
    pub type __darwin_time_t = ::core::ffi::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:1"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:1"]
pub mod sys__types_h {
    #[c2rust::src_loc = "84:1"]
    pub type __darwin_pid_t = __int32_t;
    #[c2rust::src_loc = "86:1"]
    pub type __darwin_suseconds_t = __int32_t;
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_uid_t = __uint32_t;
    use super::_types_h::{__int32_t, __uint32_t};
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_in_addr_t.h:1"]
pub mod _in_addr_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type in_addr_t = __uint32_t;
    use super::_types_h::__uint32_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_in_port_t.h:1"]
pub mod _in_port_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type in_port_t = __uint16_t;
    use super::_types_h::__uint16_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_pid_t.h:1"]
pub mod _pid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pid_t = __darwin_pid_t;
    use super::sys__types_h::__darwin_pid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uid_t.h:1"]
pub mod _uid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uid_t = __darwin_uid_t;
    use super::sys__types_h::__darwin_uid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:1"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ssize_t.h:1"]
pub mod _ssize_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ptrdiff_t.h:1"]
pub mod _ptrdiff_t_h {
    #[c2rust::src_loc = "51:1"]
    pub type ptrdiff_t = __darwin_ptrdiff_t;
    use super::_types_h::__darwin_ptrdiff_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:1"]
pub mod _uint8_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint8_t = u8;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint16_t.h:1"]
pub mod _uint16_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint16_t = u16;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:1"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = u32;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint64_t.h:1"]
pub mod _uint64_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint64_t = u64;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_timeval.h:1"]
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
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/tls/tls.h:1"]
pub mod tls_h {
    extern "C" {
        #[c2rust::src_loc = "66:1"]
        pub type tls;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/time.h:1"]
pub mod time_h {
    #[c2rust::src_loc = "40:1"]
    pub type usec_t = uint64_t;
    use super::_uint64_t_h::uint64_t;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/list.h:1"]
pub mod list_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "35:1"]
    pub struct List {
        pub next: *mut List,
        pub prev: *mut List,
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/statlist.h:1"]
pub mod statlist_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:1"]
    pub struct StatList {
        pub head: List,
        pub cur_count: ::core::ffi::c_int,
    }
    use super::list_h::List;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/aatree.h:1"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_sa_family_t.h:1"]
pub mod _sa_family_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type sa_family_t = __uint8_t;
    use super::_types_h::__uint8_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/socket.h:1"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet/in.h:1"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet6/in6.h:1"]
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
#[c2rust::header_src = "/opt/homebrew/Cellar/libevent/2.1.12_1/include/event2/event.h:1"]
pub mod event_h {
    extern "C" {
        #[c2rust::src_loc = "217:1"]
        pub type event_base;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/libevent/2.1.12_1/include/event2/event_struct.h:1"]
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
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/cryptohash.h:1"]
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
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/uthash.h:1"]
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
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/bouncer.h:1"]
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
    use super::statlist_h::StatList;
    use super::time_h::usec_t;
    use super::varcache_h::VarCache;
    extern "C" {}
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/sbuf.h:1"]
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
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/iobuf.h:1"]
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
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/mbuf.h:1"]
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
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn mbuf_avail_for_read(mut buf: *const MBuf) -> ::core::ffi::c_uint {
        (*buf).write_pos.wrapping_sub((*buf).read_pos)
    }
    #[inline]
    #[c2rust::src_loc = "113:1"]
    pub unsafe extern "C" fn mbuf_written(mut buf: *const MBuf) -> ::core::ffi::c_uint {
        (*buf).write_pos
    }
    #[inline]
    #[c2rust::src_loc = "162:1"]
    pub unsafe extern "C" fn mbuf_get_char(
        mut buf: *mut MBuf,
        mut dst_p: *mut ::core::ffi::c_char,
    ) -> bool {
        if (*buf).read_pos.wrapping_add(1 as ::core::ffi::c_uint) > (*buf).write_pos {
            return false_0 != 0;
        }
        let fresh0 = (*buf).read_pos;
        (*buf).read_pos = (*buf).read_pos.wrapping_add(1);
        *dst_p = *(*buf).data.offset(fresh0 as isize) as ::core::ffi::c_char;
        true_0 != 0
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
        true_0 != 0
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
        true_0 != 0
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
        true_0 != 0
    }
    use super::_size_t_h::size_t;
    use super::_string_h::memchr;
    use super::_uint16_t_h::uint16_t;
    use super::_uint8_t_h::uint8_t;
    use super::stdbool_h::{false_0, true_0};
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/proto.h:1"]
pub mod proto_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "34:1"]
    pub struct PktHdr {
        pub type_0: ::core::ffi::c_uint,
        pub len: ::core::ffi::c_uint,
        pub data: MBuf,
    }
    #[inline]
    #[c2rust::src_loc = "66:1"]
    pub unsafe extern "C" fn incomplete_pkt(mut pkt: *const PktHdr) -> bool {
        mbuf_written(&raw const (*pkt).data) != (*pkt).len
    }
    use super::mbuf_h::{mbuf_written, MBuf};
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/prepare.h:1"]
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
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/varcache.h:1"]
pub mod varcache_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "14:1"]
    pub struct VarCache {
        pub var_list: *mut *mut PStr,
    }
    use super::strpool_h::PStr;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/strpool.h:1"]
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
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/pktbuf.h:1"]
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
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/dnslookup.h:1"]
pub mod dnslookup_h {
    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub type DNSToken;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/logging.h:1"]
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
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/messages.h:1"]
pub mod messages_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2:9"]
    pub struct PgParsePacket {
        pub len: ::core::ffi::c_uint,
        pub name: *const ::core::ffi::c_char,
        pub query_and_parameters_len: size_t,
        pub query_and_parameters: *const ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:9"]
    pub struct PgBindPacket {
        pub len: ::core::ffi::c_uint,
        pub portal: *const ::core::ffi::c_char,
        pub name: *const ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "17:9"]
    pub struct PgDescribePacket {
        pub type_0: ::core::ffi::c_char,
        pub name: *const ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "23:9"]
    pub struct PgClosePacket {
        pub type_0: ::core::ffi::c_char,
        pub name: *const ::core::ffi::c_char,
    }
    #[c2rust::src_loc = "28:9"]
    pub type PreparedStatementAction = ::core::ffi::c_uint;
    #[c2rust::src_loc = "47:2"]
    pub const PS_INSPECT_FAILED: PreparedStatementAction = 3;
    #[c2rust::src_loc = "40:2"]
    pub const PS_HANDLE_FULL_PACKET: PreparedStatementAction = 2;
    #[c2rust::src_loc = "34:2"]
    pub const PS_HANDLE: PreparedStatementAction = 1;
    #[c2rust::src_loc = "29:2"]
    pub const PS_IGNORE: PreparedStatementAction = 0;
    use super::_size_t_h::size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:1"]
pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn memchr(
            __s: *const ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "96:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/objects.h:1"]
pub mod objects_h {
    use super::bouncer_h::PgSocket;
    extern "C" {
        #[c2rust::src_loc = "62:1"]
        pub fn disconnect_client(
            client: *mut PgSocket,
            notify: bool,
            reason: *const ::core::ffi::c_char,
            ...
        );
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/llvm/21.1.8/lib/clang/21/include/stdbool.h:1"]
pub mod stdbool_h {
    #[c2rust::src_loc = "25:9"]
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::_in_addr_t_h::in_addr_t;
pub use self::_in_port_t_h::in_port_t;
pub use self::_pid_t_h::pid_t;
pub use self::_ptrdiff_t_h::ptrdiff_t;
pub use self::_sa_family_t_h::sa_family_t;
pub use self::_size_t_h::size_t;
pub use self::_ssize_t_h::ssize_t;
use self::_string_h::strlen;
pub use self::_timeval_h::timeval;
pub use self::_types_h::{
    __darwin_ptrdiff_t, __darwin_size_t, __darwin_ssize_t, __darwin_time_t, __int32_t, __uint16_t,
    __uint32_t, __uint8_t,
};
pub use self::_uid_t_h::uid_t;
pub use self::_uint16_t_h::uint16_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::aatree_h::{aatree_cmp_f, aatree_walker_f, AANode, AATree};
pub use self::bouncer_h::{
    sockaddr_ucreds, C2RustUnnamed_9, CallbackState, LoadBalanceHosts, PacketCallbackFlag, PgAddr,
    PgCredentials, PgDatabase, PgGlobalUser, PgPool, PgSocket, PgStats, ReplicationType,
    ScramState, SocketState, CB_HANDLE_COMPLETE_PACKET, CB_NONE, CB_WANT_COMPLETE_PACKET,
    CL_ACTIVE, CL_ACTIVE_CANCEL, CL_FREE, CL_JUSTFREE, CL_LOGIN, CL_WAITING, CL_WAITING_CANCEL,
    CL_WAITING_LOGIN, LOAD_BALANCE_HOSTS_DISABLE, LOAD_BALANCE_HOSTS_ROUND_ROBIN,
    REPLICATION_LOGICAL, REPLICATION_NONE, REPLICATION_PHYSICAL, SV_ACTIVE, SV_ACTIVE_CANCEL,
    SV_BEING_CANCELED, SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN, SV_TESTED, SV_USED,
};
pub use self::cryptohash_h::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};

pub use self::event_struct_h::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
pub use self::in6_h::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use self::in_h::{in_addr, sockaddr_in};
pub use self::iobuf_h::{iobuf, IOBuf};
pub use self::list_h::List;
pub use self::logging_h::{
    cf_verbose, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE, LG_STATS,
    LG_WARNING,
};
pub use self::mbuf_h::{
    mbuf_avail_for_read, mbuf_get_bytes, mbuf_get_char, mbuf_get_string, mbuf_get_uint16be,
    mbuf_written, MBuf,
};
pub use self::messages_h::{
    PgBindPacket, PgClosePacket, PgDescribePacket, PgParsePacket, PreparedStatementAction,
    PS_HANDLE, PS_HANDLE_FULL_PACKET, PS_IGNORE, PS_INSPECT_FAILED,
};
use self::objects_h::disconnect_client;
pub use self::pktbuf_h::PktBuf;
pub use self::prepare_h::{
    PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement,
};
pub use self::proto_h::{incomplete_pkt, PktHdr};
pub use self::sbuf_h::{
    sbuf_cb_t, SBuf, SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK, SBUF_EV_FLUSH,
    SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY,
};
pub use self::socket_h::sockaddr;
pub use self::statlist_h::StatList;
pub use self::stdbool_h::{false_0, true_0};
pub use self::strpool_h::{PStr, StrPool};
pub use self::sys__types_h::{__darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t};
pub use self::time_h::usec_t;

pub use self::uthash_h::{UT_hash_bucket, UT_hash_handle, UT_hash_table};
pub use self::varcache_h::VarCache;
#[no_mangle]
#[c2rust::src_loc = "4:1"]
pub unsafe extern "C" fn inspect_parse_packet(
    mut client: *mut PgSocket,
    mut pkt: *mut PktHdr,
) -> PreparedStatementAction {
    let mut statement = ::core::ptr::null::<::core::ffi::c_char>();
    if !mbuf_get_string(&raw mut (*pkt).data, &raw mut statement) {
        return PS_INSPECT_FAILED;
    }
    if *statement == 0 {
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                client as *mut ::core::ffi::c_void,
                b"inspect_parse_packet: type=%c, len=%d, statement=<empty>\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*pkt).type_0,
                (*pkt).len,
            );
        }
        return PS_IGNORE;
    }
    if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_NOISE,
            client as *mut ::core::ffi::c_void,
            b"inspect_parse_packet: type=%c, len=%d, statement=%s\0" as *const u8
                as *const ::core::ffi::c_char,
            (*pkt).type_0,
            (*pkt).len,
            statement,
        );
    }
    PS_HANDLE_FULL_PACKET
}
#[no_mangle]
#[c2rust::src_loc = "21:1"]
pub unsafe extern "C" fn inspect_bind_packet(
    mut client: *mut PgSocket,
    mut pkt: *mut PktHdr,
) -> PreparedStatementAction {
    let mut portal = ::core::ptr::null::<::core::ffi::c_char>();
    let mut statement = ::core::ptr::null::<::core::ffi::c_char>();
    if !mbuf_get_string(&raw mut (*pkt).data, &raw mut portal) {
        return PS_INSPECT_FAILED;
    }
    if !mbuf_get_string(&raw mut (*pkt).data, &raw mut statement) {
        return PS_INSPECT_FAILED;
    }
    if *statement == 0 {
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                client as *mut ::core::ffi::c_void,
                b"inspect_bind_packet: type=%c, len=%d, statement=<empty>\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*pkt).type_0,
                (*pkt).len,
            );
        }
        return PS_IGNORE;
    }
    if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_NOISE,
            client as *mut ::core::ffi::c_void,
            b"inspect_bind_packet: type=%c, len=%d, statement=%s\0" as *const u8
                as *const ::core::ffi::c_char,
            (*pkt).type_0,
            (*pkt).len,
            statement,
        );
    }
    PS_HANDLE
}
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn inspect_describe_or_close_packet(
    mut client: *mut PgSocket,
    mut pkt: *mut PktHdr,
) -> PreparedStatementAction {
    let mut describe: ::core::ffi::c_char = 0;
    let mut statement = ::core::ptr::null::<::core::ffi::c_char>();
    if !mbuf_get_char(&raw mut (*pkt).data, &raw mut describe) {
        return PS_INSPECT_FAILED;
    }
    if describe as ::core::ffi::c_int != 'S' as i32 {
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                client as *mut ::core::ffi::c_void,
                b"inspect_describe_or_close_packet: type=%c, len=%d, P/S=%c\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*pkt).type_0,
                (*pkt).len,
                describe as ::core::ffi::c_int,
            );
        }
        return PS_IGNORE;
    }
    if !mbuf_get_string(&raw mut (*pkt).data, &raw mut statement) {
        return PS_INSPECT_FAILED;
    }
    if *statement == 0 {
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                client as *mut ::core::ffi::c_void,
                b"inspect_describe_or_close_packet: type=%c, len=%d, P/S=%c, statement=<empty>\0"
                    as *const u8 as *const ::core::ffi::c_char,
                (*pkt).type_0,
                (*pkt).len,
                describe as ::core::ffi::c_int,
            );
        }
        return PS_IGNORE;
    }
    if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_NOISE,
            client as *mut ::core::ffi::c_void,
            b"inspect_describe_or_close_packet: type=%c, len=%d, P/S=%c, statement=%s\0"
                as *const u8 as *const ::core::ffi::c_char,
            (*pkt).type_0,
            (*pkt).len,
            describe as ::core::ffi::c_int,
            statement,
        );
    }
    PS_HANDLE
}
#[no_mangle]
#[c2rust::src_loc = "74:1"]
pub unsafe extern "C" fn unmarshall_parse_packet(
    mut client: *mut PgSocket,
    mut pkt: *mut PktHdr,
    mut parse_packet: *mut PgParsePacket,
) -> bool {
    let mut statement = ::core::ptr::null::<::core::ffi::c_char>();
    let mut query = ::core::ptr::null::<::core::ffi::c_char>();
    let mut num_parameters: uint16_t = 0;
    let mut parameter_types_bytes = ::core::ptr::null::<uint8_t>();
    let mut parameters_length: size_t = 0;
    if mbuf_get_string(&raw mut (*pkt).data, &raw mut statement)
        && mbuf_get_string(&raw mut (*pkt).data, &raw mut query)
        && mbuf_get_uint16be(&raw mut (*pkt).data, &raw mut num_parameters)
    {
        parameters_length = (num_parameters as size_t).wrapping_mul(4 as size_t);
        if mbuf_get_bytes(
            &raw mut (*pkt).data,
            parameters_length as ::core::ffi::c_uint,
            &raw mut parameter_types_bytes,
        ) {
            (*parse_packet).len = (*pkt).len;
            (*parse_packet).name = statement;
            (*parse_packet).query_and_parameters_len = strlen(query)
                .wrapping_add(1 as size_t)
                .wrapping_add(::core::mem::size_of::<uint16_t>() as size_t)
                .wrapping_add(parameters_length);
            (*parse_packet).query_and_parameters = query;
            return true_0 != 0;
        }
    }
    disconnect_client(
        client,
        true_0 != 0,
        b"broken Parse packet\0" as *const u8 as *const ::core::ffi::c_char,
    );
    false_0 != 0
}
#[no_mangle]
#[c2rust::src_loc = "120:1"]
pub unsafe extern "C" fn unmarshall_bind_packet(
    mut client: *mut PgSocket,
    mut pkt: *mut PktHdr,
    mut bind_packet: *mut PgBindPacket,
) -> bool {
    let mut portal = ::core::ptr::null::<::core::ffi::c_char>();
    let mut statement = ::core::ptr::null::<::core::ffi::c_char>();
    if mbuf_get_string(&raw mut (*pkt).data, &raw mut portal)
        && mbuf_get_string(&raw mut (*pkt).data, &raw mut statement)
    {
        (*bind_packet).len = (*pkt).len;
        (*bind_packet).portal = portal;
        (*bind_packet).name = statement;
        return true_0 != 0;
    }
    disconnect_client(
        client,
        true_0 != 0,
        b"broken Bind packet\0" as *const u8 as *const ::core::ffi::c_char,
    );
    false_0 != 0
}
#[no_mangle]
#[c2rust::src_loc = "149:1"]
pub unsafe extern "C" fn unmarshall_describe_packet(
    mut client: *mut PgSocket,
    mut pkt: *mut PktHdr,
    mut describe_packet: *mut PgDescribePacket,
) -> bool {
    let mut describe: ::core::ffi::c_char = 0;
    let mut statement = ::core::ptr::null::<::core::ffi::c_char>();
    if incomplete_pkt(pkt) {
        return false_0 != 0;
    }
    if mbuf_get_char(&raw mut (*pkt).data, &raw mut describe)
        && mbuf_get_string(&raw mut (*pkt).data, &raw mut statement)
    {
        (*describe_packet).type_0 = describe;
        (*describe_packet).name = statement;
        return true_0 != 0;
    }
    disconnect_client(
        client,
        true_0 != 0,
        b"broken Describe packet\0" as *const u8 as *const ::core::ffi::c_char,
    );
    false_0 != 0
}
#[no_mangle]
#[c2rust::src_loc = "180:1"]
pub unsafe extern "C" fn unmarshall_close_packet(
    mut client: *mut PgSocket,
    mut pkt: *mut PktHdr,
    mut close_packet: *mut PgClosePacket,
) -> bool {
    let mut type_0: ::core::ffi::c_char = 0;
    let mut name = ::core::ptr::null::<::core::ffi::c_char>();
    if incomplete_pkt(pkt) {
        return false_0 != 0;
    }
    if mbuf_get_char(&raw mut (*pkt).data, &raw mut type_0)
        && mbuf_get_string(&raw mut (*pkt).data, &raw mut name)
    {
        (*close_packet).type_0 = type_0;
        (*close_packet).name = name;
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                client as *mut ::core::ffi::c_void,
                b"unmarshall_close_packet: type=%c, len=%d, S/P=%c, name=%s\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*pkt).type_0,
                (*pkt).len,
                type_0 as ::core::ffi::c_int,
                name,
            );
        }
        return true_0 != 0;
    }
    disconnect_client(
        client,
        true_0 != 0,
        b"broken Close packet\0" as *const u8 as *const ::core::ffi::c_char,
    );
    false_0 != 0
}
#[no_mangle]
#[c2rust::src_loc = "205:1"]
pub unsafe extern "C" fn is_close_named_statement_packet(
    mut close_packet: *mut PgClosePacket,
) -> bool {
    (*close_packet).type_0 as ::core::ffi::c_int == 'S' as i32
        && *(*close_packet).name as ::core::ffi::c_int != 0
}
