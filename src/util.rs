
pub mod _types_h {
    
    pub type __uint8_t = u8;
    
    pub type __uint16_t = u16;
    
    pub type __int32_t = i32;
    
    pub type __uint32_t = u32;
    
    pub type __darwin_ct_rune_t = ::core::ffi::c_int;
    
    pub type __darwin_ptrdiff_t = isize;
    
    pub type __darwin_size_t = usize;
    
    pub type __darwin_wchar_t = ::libc::wchar_t;
    
    pub type __darwin_rune_t = __darwin_wchar_t;
    
    pub type __darwin_socklen_t = __uint32_t;
    
    pub type __darwin_ssize_t = isize;
    
    pub type __darwin_time_t = ::core::ffi::c_long;
}

pub mod _u_int32_t_h {
    
    pub type u_int32_t = ::core::ffi::c_uint;
}

pub mod _uintptr_t_h {
    
    pub type uintptr_t = usize;
}

pub mod sys__types_h {
    
    pub type __darwin_gid_t = __uint32_t;
    
    pub type __darwin_pid_t = __int32_t;
    
    pub type __darwin_suseconds_t = __int32_t;
    
    pub type __darwin_uid_t = __uint32_t;
    
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    use super::_types_h::{__int32_t, __uint32_t};
}

pub mod _gid_t_h {
    
    pub type gid_t = __darwin_gid_t;
    use super::sys__types_h::__darwin_gid_t;
}

pub mod _in_addr_t_h {
    
    pub type in_addr_t = __uint32_t;
    use super::_types_h::__uint32_t;
}

pub mod _in_port_t_h {
    
    pub type in_port_t = __uint16_t;
    use super::_types_h::__uint16_t;
}

pub mod _pid_t_h {
    
    pub type pid_t = __darwin_pid_t;
    use super::sys__types_h::__darwin_pid_t;
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

pub mod _ptrdiff_t_h {
    
    pub type ptrdiff_t = __darwin_ptrdiff_t;
    use super::_types_h::__darwin_ptrdiff_t;
}

pub mod _uint8_t_h {
    
    pub type uint8_t = u8;
}

pub mod _uint16_t_h {
    
    pub type uint16_t = u16;
}

pub mod _uint32_t_h {
    
    pub type uint32_t = u32;
}

pub mod _uint64_t_h {
    
    pub type uint64_t = u64;
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
    use super::_types_h::{__darwin_rune_t, __darwin_size_t, __uint32_t};
    extern "C" {
        
        pub static mut _DefaultRuneLocale: _RuneLocale;
    }
}

pub mod tls_h {
    extern "C" {
        
        pub type tls;
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
    
    pub const SO_SNDBUF: ::core::ffi::c_int = 0x1001 as ::core::ffi::c_int;
    
    pub const SO_RCVBUF: ::core::ffi::c_int = 0x1002 as ::core::ffi::c_int;
    
    pub const SOL_SOCKET: ::core::ffi::c_int = 0xffff as ::core::ffi::c_int;
    
    pub const AF_UNIX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    pub const AF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    pub const AF_INET6: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
    use super::_sa_family_t_h::sa_family_t;
    use super::_socklen_t_h::socklen_t;
    use super::_types_h::__uint8_t;
    extern "C" {
        
        pub fn getpeername(
            _: ::core::ffi::c_int,
            _: *mut sockaddr,
            _: *mut socklen_t,
        ) -> ::core::ffi::c_int;
        
        pub fn getsockname(
            _: ::core::ffi::c_int,
            _: *mut sockaddr,
            _: *mut socklen_t,
        ) -> ::core::ffi::c_int;
        
        pub fn setsockopt(
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_int,
            _: *const ::core::ffi::c_void,
            _: socklen_t,
        ) -> ::core::ffi::c_int;
    }
}

pub mod in_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sockaddr_in {
        pub sin_len: __uint8_t,
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [::core::ffi::c_char; 8],
    }
    
    pub const IPPROTO_TCP: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
    use super::_in_addr_t_h::in_addr_t;
    use super::_in_port_t_h::in_port_t;
    use super::_sa_family_t_h::sa_family_t;
    use super::_types_h::__uint8_t;
}

pub mod in6_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct in6_addr {
        pub __u6_addr: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union C2RustUnnamed {
        pub __u6_addr8: [__uint8_t; 16],
        pub __u6_addr16: [__uint16_t; 8],
        pub __u6_addr32: [__uint32_t; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
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

pub mod event_h {
    use super::_timeval_h::timeval;
    use super::event_struct_h::event;
    extern "C" {
        
        pub type event_base;
        
        pub fn event_add(ev: *mut event, timeout: *const timeval) -> ::core::ffi::c_int;
    }
}

pub mod event_struct_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
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
    
    pub union C2RustUnnamed_0 {
        pub ev_io: C2RustUnnamed_3,
        pub ev_signal: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct C2RustUnnamed_1 {
        pub ev_signal_next: C2RustUnnamed_2,
        pub ev_ncalls: ::core::ffi::c_short,
        pub ev_pncalls: *mut ::core::ffi::c_short,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct C2RustUnnamed_2 {
        pub le_next: *mut event,
        pub le_prev: *mut *mut event,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct C2RustUnnamed_3 {
        pub ev_io_next: C2RustUnnamed_4,
        pub ev_timeout: timeval,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct C2RustUnnamed_4 {
        pub le_next: *mut event,
        pub le_prev: *mut *mut event,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union C2RustUnnamed_5 {
        pub ev_next_with_common_timeout: C2RustUnnamed_6,
        pub min_heap_idx: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct C2RustUnnamed_6 {
        pub tqe_next: *mut event,
        pub tqe_prev: *mut *mut event,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
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
    
    pub struct C2RustUnnamed_8 {
        pub tqe_next: *mut event_callback,
        pub tqe_prev: *mut *mut event_callback,
    }
    use super::_timeval_h::timeval;
    use super::_uint8_t_h::uint8_t;
    use super::event_h::event_base;
}



pub mod bouncer_h {
    
    pub type SocketState = ::core::ffi::c_uint;
    
    pub const SV_TESTED: SocketState = 16;
    
    pub const SV_USED: SocketState = 15;
    
    pub const SV_ACTIVE_CANCEL: SocketState = 14;
    
    pub const SV_ACTIVE: SocketState = 13;
    
    pub const SV_IDLE: SocketState = 12;
    
    pub const SV_BEING_CANCELED: SocketState = 11;
    
    pub const SV_LOGIN: SocketState = 10;
    
    pub const SV_JUSTFREE: SocketState = 9;
    
    pub const SV_FREE: SocketState = 8;
    
    pub const CL_ACTIVE_CANCEL: SocketState = 7;
    
    pub const CL_WAITING_CANCEL: SocketState = 6;
    
    pub const CL_ACTIVE: SocketState = 5;
    
    pub const CL_WAITING_LOGIN: SocketState = 4;
    
    pub const CL_WAITING: SocketState = 3;
    
    pub const CL_LOGIN: SocketState = 2;
    
    pub const CL_JUSTFREE: SocketState = 1;
    
    pub const CL_FREE: SocketState = 0;
    
    pub type PacketCallbackFlag = ::core::ffi::c_uint;
    
    pub const CB_HANDLE_COMPLETE_PACKET: PacketCallbackFlag = 2;
    
    pub const CB_WANT_COMPLETE_PACKET: PacketCallbackFlag = 1;
    
    pub const CB_NONE: PacketCallbackFlag = 0;
    
    pub type LoadBalanceHosts = ::core::ffi::c_uint;
    
    pub const LOAD_BALANCE_HOSTS_ROUND_ROBIN: LoadBalanceHosts = 1;
    
    pub const LOAD_BALANCE_HOSTS_DISABLE: LoadBalanceHosts = 0;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    
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
    
    pub struct CallbackState {
        #[bitfield(name = "flag", ty = "PacketCallbackFlag", bits = "0..=7")]
        pub flag: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
        pub pkt: PktHdr,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
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
    
    pub union C2RustUnnamed_9 {
        pub dns_token: *mut DNSToken,
        pub db: *mut PgDatabase,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
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
    // PgStats moved to super::common::types
    pub use super::super::common::types::PgStats;
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union PgAddr {
        pub sa: sockaddr,
        pub sin: sockaddr_in,
        pub sin6: sockaddr_in6,
        pub scred: sockaddr_ucreds,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sockaddr_ucreds {
        pub sin: sockaddr_in,
        pub uid: uid_t,
        pub pid: pid_t,
    }
    
    pub type ReplicationType = ::core::ffi::c_uint;
    
    pub const REPLICATION_PHYSICAL: ReplicationType = 2;
    
    pub const REPLICATION_LOGICAL: ReplicationType = 1;
    
    pub const REPLICATION_NONE: ReplicationType = 0;
    #[inline]
    
    pub unsafe extern "C" fn pga_family(mut a: *const PgAddr) -> ::core::ffi::c_uint {
        (*a).sa.sa_family as ::core::ffi::c_uint
    }
    #[inline]
    
    pub unsafe extern "C" fn pga_is_unix(mut a: *const PgAddr) -> bool {
        (*a).sa.sa_family as ::core::ffi::c_int == AF_UNIX
    }
    use super::_pid_t_h::pid_t;

    use super::_uid_t_h::uid_t;
    use super::_uint16_t_h::uint16_t;
    use super::_uint64_t_h::uint64_t;
    use super::_uint8_t_h::uint8_t;
    use crate::types::{AANode, AATree};
    use crate::types::pg_cryptohash_type;
    use super::dnslookup_h::DNSToken;
    use super::in6_h::sockaddr_in6;
    use super::in_h::sockaddr_in;
    use crate::types::List;
    use super::pktbuf_h::PktBuf;
    use crate::types::{PgClientPreparedStatement, PgServerPreparedStatement};
    use crate::types::PktHdr;
    use super::sbuf_h::SBuf;
    use super::socket_h::{sockaddr, AF_UNIX};
    use crate::types::StatList;
    use crate::types::usec_t;
    use crate::types::VarCache;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    
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
    
    pub struct SBufIO {
        pub sbufio_peek:
            Option<unsafe extern "C" fn(*mut SBuf, *mut ::core::ffi::c_void, size_t) -> ssize_t>,
        pub sbufio_recv:
            Option<unsafe extern "C" fn(*mut SBuf, *mut ::core::ffi::c_void, size_t) -> ssize_t>,
        pub sbufio_send:
            Option<unsafe extern "C" fn(*mut SBuf, *const ::core::ffi::c_void, size_t) -> ssize_t>,
        pub sbufio_close: Option<unsafe extern "C" fn(*mut SBuf) -> ::core::ffi::c_int>,
    }
    
    pub type sbuf_cb_t = Option<unsafe extern "C" fn(*mut SBuf, SBufEvent, *mut MBuf) -> bool>;
    
    pub type SBufEvent = ::core::ffi::c_uint;
    
    pub const SBUF_EV_TLS_READY: SBufEvent = 7;
    
    pub const SBUF_EV_PKT_CALLBACK: SBufEvent = 6;
    
    pub const SBUF_EV_FLUSH: SBufEvent = 5;
    
    pub const SBUF_EV_CONNECT_OK: SBufEvent = 4;
    
    pub const SBUF_EV_CONNECT_FAILED: SBufEvent = 3;
    
    pub const SBUF_EV_SEND_FAILED: SBufEvent = 2;
    
    pub const SBUF_EV_RECV_FAILED: SBufEvent = 1;
    
    pub const SBUF_EV_READ: SBufEvent = 0;
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    use super::_uint8_t_h::uint8_t;
    use super::event_struct_h::event;
    use super::iobuf_h::IOBuf;
    use crate::types::MBuf;
    use super::tls_h::tls;
}

pub mod iobuf_h {
    
    pub type IOBuf = iobuf;
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct iobuf {
        pub done_pos: ::core::ffi::c_uint,
        pub parse_pos: ::core::ffi::c_uint,
        pub recv_pos: ::core::ffi::c_uint,
        pub buf: [uint8_t; 0],
    }
    use super::_uint8_t_h::uint8_t;
}



pub mod pktbuf_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    
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

pub mod dnslookup_h {
    extern "C" {
        
        pub type DNSToken;
    }
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
    use super::_types_h::{__uint16_t, __uint32_t};
}

pub mod _stdio_h {
    use super::_size_t_h::size_t;
    extern "C" {
        
        pub fn snprintf(
            __str: *mut ::core::ffi::c_char,
            __size: size_t,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}

pub mod unistd_h {
    use super::_pid_t_h::pid_t;
    use super::_size_t_h::size_t;
    use super::_uid_t_h::uid_t;
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
    use super::_types_h::__darwin_ct_rune_t;
    use super::runetype_h::_DefaultRuneLocale;
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

pub mod _null_h {
    
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}

pub mod inet_h {
    use super::_socklen_t_h::socklen_t;
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

pub mod usual_socket_h {
    use super::_gid_t_h::gid_t;
    use super::_pid_t_h::pid_t;
    use super::_uid_t_h::uid_t;
    extern "C" {
        
        pub fn socket_setup(sock: ::core::ffi::c_int, non_block: bool) -> bool;
        
        pub fn socket_set_keepalive(
            fd: ::core::ffi::c_int,
            onoff: ::core::ffi::c_int,
            keepidle: ::core::ffi::c_int,
            keepintvl: ::core::ffi::c_int,
            keepcnt: ::core::ffi::c_int,
        ) -> bool;
        
        pub fn usual_getpeercreds(
            fd: ::core::ffi::c_int,
            uid_p: *mut uid_t,
            gid_p: *mut gid_t,
            pid_p: *mut pid_t,
        ) -> ::core::ffi::c_int;
    }
}

pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        
        pub fn memcmp(
            __s1: *const ::core::ffi::c_void,
            __s2: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> ::core::ffi::c_int;
        
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
        
        pub fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
        
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
        
        pub fn strstr(
            __big: *const ::core::ffi::c_char,
            __little: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        
        pub fn strlcpy(
            __dst: *mut ::core::ffi::c_char,
            __source: *const ::core::ffi::c_char,
            __size: size_t,
        ) -> ::core::ffi::c_ulong;
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
    use super::_size_t_h::size_t;
    extern "C" {
        
        pub fn csrandom_bytes(buf: *mut ::core::ffi::c_void, nbytes: size_t);
    }
}

pub mod _stdlib_h {
    extern "C" {
        
        pub fn exit(_: ::core::ffi::c_int) -> !;
    }
}

pub mod errno_h {
    
    pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
    
    pub const ENOSYS: ::core::ffi::c_int = 78 as ::core::ffi::c_int;
    extern "C" {
        
        pub fn __error() -> *mut ::core::ffi::c_int;
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
pub use self::_gid_t_h::gid_t;
pub use self::_in_addr_t_h::in_addr_t;
pub use self::_in_port_t_h::in_port_t;
pub use self::_null_h::NULL;
pub use self::_pid_t_h::pid_t;
pub use self::_ptrdiff_t_h::ptrdiff_t;
pub use self::_sa_family_t_h::sa_family_t;
pub use self::_size_t_h::size_t;
pub use self::_socklen_t_h::socklen_t;
pub use self::_ssize_t_h::ssize_t;
use self::_stdio_h::snprintf;
use self::_stdlib_h::exit;
use self::_string_h::{memcmp, memcpy, memset, strchr, strcmp, strerror, strlcpy, strlen, strstr};
pub use self::_timeval_h::timeval;
pub use self::_types_h::{
    __darwin_ct_rune_t, __darwin_ptrdiff_t, __darwin_rune_t, __darwin_size_t, __darwin_socklen_t,
    __darwin_ssize_t, __darwin_time_t, __darwin_wchar_t, __int32_t, __uint16_t, __uint32_t,
    __uint8_t,
};
pub use self::_u_int32_t_h::u_int32_t;
pub use self::_uid_t_h::uid_t;
pub use self::_uint16_t_h::uint16_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use crate::types::{aatree_cmp_f, aatree_walker_f, AANode, AATree};
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
pub use crate::types::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};
use self::csrandom_h::csrandom_bytes;
pub use self::ctype_h::safe_isspace;

use self::err_h::{ERR_clear_error, ERR_get_error, ERR_reason_error_string};
pub use self::errno_h::{__error, EINVAL, ENOSYS};
use self::event_h::event_add;
pub use self::event_struct_h::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
use self::evp_h::{
    EVP_DigestFinal_ex, EVP_DigestInit, EVP_DigestUpdate, EVP_MD_CTX_free, EVP_MD_CTX_new, EVP_md5,
};
pub use self::in6_h::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use self::in_h::{in_addr, sockaddr_in, IPPROTO_TCP};
use self::inet_h::{inet_ntop, inet_pton};
pub use self::iobuf_h::{iobuf, IOBuf};
pub use crate::types::List;
pub use self::logging_h::{
    cf_verbose, log_fatal, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE,
    LG_STATS, LG_WARNING,
};
pub use crate::types::MBuf;
pub use self::md5_h::MD5_DIGEST_LENGTH;
pub use self::pktbuf_h::PktBuf;
pub use crate::types::{
    PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement,
};
pub use crate::types::PktHdr;
pub use self::runetype_h::{
    _DefaultRuneLocale, _RuneCharClass, _RuneEntry, _RuneLocale, _RuneRange,
};
pub use self::sbuf_h::{
    sbuf_cb_t, SBuf, SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK, SBUF_EV_FLUSH,
    SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY,
};
pub use self::socket_h::{
    getpeername, getsockname, setsockopt, sockaddr, AF_INET, AF_INET6, AF_UNIX, SOL_SOCKET,
    SO_RCVBUF, SO_SNDBUF,
};
pub use crate::types::StatList;
pub use crate::types::{false_0, true_0};
pub use crate::types::{PStr, StrPool};
pub use self::sys__types_h::{
    __darwin_gid_t, __darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t, __DARWIN_NULL,
};
pub use self::tcp_h::TCP_NODELAY;
pub use crate::types::usec_t;

pub use self::types_h::{evp_md_ctx_st, evp_md_st, EVP_MD, EVP_MD_CTX};
use self::unistd_h::{geteuid, gethostname, getpid};
use self::usual_socket_h::{socket_set_keepalive, socket_setup, usual_getpeercreds};
pub use crate::types::{UT_hash_bucket, UT_hash_handle, UT_hash_table};
pub use crate::types::VarCache;
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
        b"(nodb)\0" as *const u8 as *const ::core::ffi::c_char
    };
    user = if !(*sock).login_user_credentials.is_null() {
        &raw mut (*(*sock).login_user_credentials).name as *mut ::core::ffi::c_char
            as *const ::core::ffi::c_char
    } else {
        b"(nouser)\0" as *const u8 as *const ::core::ffi::c_char
    };
    if pga_is_unix(&raw const (*sock).remote_addr) {
        let mut pid = (*sock).remote_addr.scred.pid as ::core::ffi::c_ulong;
        if pid != 0 {
            snprintf(
                &raw mut host6 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 56]>() as size_t,
                b"unix(%lu)\0" as *const u8 as *const ::core::ffi::c_char,
                pid,
            );
            host = &raw mut host6 as *mut ::core::ffi::c_char;
        } else {
            host = b"unix\0" as *const u8 as *const ::core::ffi::c_char;
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
                b"%c-%p: peer-%d@[%s]:%d \0" as *const u8 as *const ::core::ffi::c_char,
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
            b"%c-%p: %s/%s@[%s]:%d \0" as *const u8 as *const ::core::ffi::c_char,
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
                b"%c-%p: peer-%d@%s:%d \0" as *const u8 as *const ::core::ffi::c_char,
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
            b"%c-%p: %s/%s@%s:%d \0" as *const u8 as *const ::core::ffi::c_char,
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
        return b"\0" as *const u8 as *const ::core::ffi::c_char;
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
            b"MD5 authentication failed: out-of-memory\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return false_0 != 0;
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
            b"md5\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            3 as size_t,
        );
        hash2hex(
            &raw mut hash as *mut uint8_t,
            dest.offset(3 as ::core::ffi::c_int as isize),
        );
        return true_0 != 0;
    }
    let mut _log_ctx_0 = NULL;
    log_generic(
        LG_ERROR,
        _log_ctx_0,
        b"MD5 authentication failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
        ERR_reason_error_string(ERR_get_error()),
    );
    EVP_MD_CTX_free(mdctx);
    false_0 != 0
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
    errpos = b"socket_setup\0" as *const u8 as *const ::core::ffi::c_char;
    ok = socket_setup(sock, true_0 != 0);
    if ok {
        if is_unix {
            return true_0 != 0;
        }
        errpos = b"socket_set_keepalive\0" as *const u8 as *const ::core::ffi::c_char;
        ok = socket_set_keepalive(
            sock,
            cf_tcp_keepalive,
            cf_tcp_keepidle,
            cf_tcp_keepintvl,
            cf_tcp_keepcnt,
        );
        if ok {
            if cf_tcp_user_timeout != 0 {
                errpos =
                    b"setsockopt/TCP_USER_TIMEOUT\0" as *const u8 as *const ::core::ffi::c_char;
                *__error() = EINVAL;
            } else {
                if cf_tcp_socket_buffer != 0 {
                    val = cf_tcp_socket_buffer;
                    errpos = b"setsockopt/SO_SNDBUF\0" as *const u8 as *const ::core::ffi::c_char;
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
                        errpos =
                            b"setsockopt/SO_RCVBUF\0" as *const u8 as *const ::core::ffi::c_char;
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
                        errpos =
                            b"setsockopt/TCP_NODELAY\0" as *const u8 as *const ::core::ffi::c_char;
                        res = setsockopt(
                            sock,
                            IPPROTO_TCP,
                            TCP_NODELAY,
                            &raw mut val as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
                        );
                        if res >= 0 as ::core::ffi::c_int {
                            return true_0 != 0;
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
        b"%s(%d) failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
        errpos,
        sock,
        strerror(*__error()),
    );
    false_0 != 0
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
            return false_0 != 0;
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
    true_0 != 0
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
            if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_NOISE,
                    _log_ctx,
                    b"unix peer uid: %d\0" as *const u8 as *const ::core::ffi::c_char,
                    uid as ::core::ffi::c_int,
                );
            }
        } else if *__error() != ENOSYS {
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_WARNING,
                _log_ctx_0,
                b"unix peer uid failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"fill_remote_addr: getpeername(%d) = %s\0" as *const u8
                    as *const ::core::ffi::c_char,
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
                b"fill_local_addr: getsockname(%d) = %s\0" as *const u8
                    as *const ::core::ffi::c_char,
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
            b"src/util.c\0" as *const u8 as *const ::core::ffi::c_char,
            351 as ::core::ffi::c_int,
            b"safe_evtimer_add\0" as *const u8 as *const ::core::ffi::c_char,
            false_0 != 0,
            _log_ctx,
            b"TIMER_BACKUP_SLOTS full\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"pga_copy: AF_UNIX copy not supported\0" as *const u8
                    as *const ::core::ffi::c_char,
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
                b"pga_cmp_addr: unsupported family\0" as *const u8 as *const ::core::ffi::c_char,
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
            res = b"unix\0" as *const u8 as *const ::core::ffi::c_char;
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
            res = b"(bad-af)\0" as *const u8 as *const ::core::ffi::c_char;
        }
    }
    if res.is_null() {
        res = b"(err-ntop)\0" as *const u8 as *const ::core::ffi::c_char;
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
    if strcmp(s, b"unix\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        pga_set(a, AF_UNIX, port);
    } else if strcmp(s, b"*\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
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
            b"[%s]:%d\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut buf as *mut ::core::ffi::c_char,
            pga_port(a),
        );
    } else if pga_family(a) == AF_UNIX as ::core::ffi::c_uint && (*a).scred.pid != 0 {
        snprintf(
            dst,
            dstlen as size_t,
            b"%s:%d$%lu\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut buf as *mut ::core::ffi::c_char,
            pga_port(a),
            (*a).scred.pid as ::core::ffi::c_ulong,
        );
    } else {
        snprintf(
            dst,
            dstlen as size_t,
            b"%s:%d\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"somehost\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"[%s]:%d\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut buf as *mut ::core::ffi::c_char,
            pga_port(a),
        );
    } else if pga_family(a) == AF_UNIX as ::core::ffi::c_uint && (*a).scred.pid != 0 {
        snprintf(
            dst,
            dstlen as size_t,
            b"%s(%lu@%s):%d\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut buf as *mut ::core::ffi::c_char,
            (*a).scred.pid as ::core::ffi::c_ulong,
            cached_hostname(),
            pga_port(a),
        );
    } else {
        snprintf(
            dst,
            dstlen as size_t,
            b"%s:%d\0" as *const u8 as *const ::core::ffi::c_char,
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
        return false_0 != 0;
    }
    cf_set_str(cv, value)
}
#[no_mangle]

pub unsafe extern "C" fn check_reserved_database(mut value: *const ::core::ffi::c_char) -> bool {
    if !value.is_null()
        && strcmp(
            value,
            b"pgbouncer\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    true_0 != 0
}
