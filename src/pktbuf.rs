
pub mod internal {
    
    pub type __builtin_va_list = *mut ::core::ffi::c_char;
}

pub mod _types_h {
    
    pub type __uint8_t = u8;
    
    pub type __uint16_t = u16;
    
    pub type __int32_t = i32;
    
    pub type __uint32_t = u32;
    
    pub type __darwin_ptrdiff_t = isize;
    
    pub type __darwin_size_t = usize;
    
    pub type __darwin_va_list = __builtin_va_list;
    
    pub type __darwin_ssize_t = isize;
    
    pub type __darwin_time_t = ::core::ffi::c_long;
    use super::internal::__builtin_va_list;
}

pub mod _int64_t_h {
    
    pub type int64_t = i64;
}

pub mod _uintptr_t_h {
    
    pub type uintptr_t = usize;
}

pub mod sys__types_h {
    
    pub type __darwin_pid_t = __int32_t;
    
    pub type __darwin_suseconds_t = __int32_t;
    
    pub type __darwin_uid_t = __uint32_t;
    
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    use super::_types_h::{__int32_t, __uint32_t};
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

pub mod _va_list_h {
    
    pub type va_list = __darwin_va_list;
    use super::_types_h::__darwin_va_list;
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

pub mod tls_h {
    extern "C" {
        
        pub type tls;
    }
}

pub mod time_h {
    
    pub type usec_t = uint64_t;
    use super::_uint64_t_h::uint64_t;
    extern "C" {
        
        pub fn format_time_s(
            time: usec_t,
            dest: *mut ::core::ffi::c_char,
            destlen: ::core::ffi::c_uint,
        ) -> *mut ::core::ffi::c_char;
    }
}

pub mod list_h {
    pub use super::super::common::types::List;
}

pub mod statlist_h {
    pub use super::super::common::types::StatList;
    pub use super::list_h::List;
}

pub mod aatree_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct AATree {
        pub root: *mut AANode,
        pub count: ::core::ffi::c_int,
        pub node_cmp: aatree_cmp_f,
        pub release_cb: aatree_walker_f,
    }
    
    pub type aatree_walker_f =
        Option<unsafe extern "C" fn(*mut AANode, *mut ::core::ffi::c_void) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct AANode {
        pub left: *mut AANode,
        pub right: *mut AANode,
        pub level: ::core::ffi::c_int,
    }
    
    pub type aatree_cmp_f =
        Option<unsafe extern "C" fn(uintptr_t, *mut AANode) -> ::core::ffi::c_int>;
    use super::_uintptr_t_h::uintptr_t;
}

pub mod _sa_family_t_h {
    
    pub type sa_family_t = __uint8_t;
    use super::_types_h::__uint8_t;
}

pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sockaddr {
        pub sa_len: __uint8_t,
        pub sa_family: sa_family_t,
        pub sa_data: [::core::ffi::c_char; 14],
    }
    use super::_sa_family_t_h::sa_family_t;
    use super::_types_h::__uint8_t;
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
    
    pub type event_callback_fn = Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_short,
            *mut ::core::ffi::c_void,
        ) -> (),
    >;
    
    pub const EV_WRITE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    use super::_timeval_h::timeval;
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

pub mod cryptohash_h {
    
    pub type pg_cryptohash_type = ::core::ffi::c_uint;
    
    pub const PG_SHA512: pg_cryptohash_type = 3;
    
    pub const PG_SHA384: pg_cryptohash_type = 2;
    
    pub const PG_SHA256: pg_cryptohash_type = 1;
    
    pub const PG_SHA224: pg_cryptohash_type = 0;
}

pub mod uthash_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct UT_hash_bucket {
        pub hh_head: *mut UT_hash_handle,
        pub count: ::core::ffi::c_uint,
        pub expand_mult: ::core::ffi::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
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
    use super::_pid_t_h::pid_t;
    use super::_uid_t_h::uid_t;
    use super::_uint16_t_h::uint16_t;
    use super::_uint64_t_h::uint64_t;
    use super::_uint8_t_h::uint8_t;
    use super::aatree_h::{AANode, AATree};
    use super::cryptohash_h::pg_cryptohash_type;
    use super::dnslookup_h::DNSToken;
    use super::event_h::event_base;
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
    extern "C" {
        
        pub static mut pgb_event_base: *mut event_base;
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
    #[inline]
    
    pub unsafe extern "C" fn sbuf_op_send(
        mut sbuf: *mut SBuf,
        mut buf: *const ::core::ffi::c_void,
        mut len: size_t,
    ) -> ssize_t {
        (*(*sbuf).ops)
            .sbufio_send
            .expect("non-null function pointer")(sbuf, buf, len)
    }
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    use super::_uint8_t_h::uint8_t;
    use super::event_struct_h::event;
    use super::iobuf_h::IOBuf;
    use super::mbuf_h::MBuf;
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

pub mod mbuf_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct MBuf {
        pub data: *mut uint8_t,
        pub read_pos: ::core::ffi::c_uint,
        pub write_pos: ::core::ffi::c_uint,
        pub alloc_len: ::core::ffi::c_uint,
        pub reader: bool,
        pub fixed: bool,
    }
    use super::_uint8_t_h::uint8_t;
}

pub mod proto_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct PktHdr {
        pub type_0: ::core::ffi::c_uint,
        pub len: ::core::ffi::c_uint,
        pub data: MBuf,
    }
    use super::bouncer_h::PgSocket;
    use super::mbuf_h::MBuf;
    extern "C" {
        
        pub fn send_pooler_error(
            client: *mut PgSocket,
            send_ready: bool,
            sqlstate: *const ::core::ffi::c_char,
            level_fatal: bool,
            msg: *const ::core::ffi::c_char,
        ) -> bool;
    }
}

pub mod prepare_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct PgServerPreparedStatement {
        pub query_id: uint64_t,
        pub hh: UT_hash_handle,
        pub ps: *mut PgPreparedStatement,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
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

pub mod varcache_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct VarCache {
        pub var_list: *mut *mut PStr,
    }
    use super::strpool_h::PStr;
}

pub mod strpool_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct PStr {
        pub pool: *mut StrPool,
        pub len: size_t,
        pub refcnt: ::core::ffi::c_int,
        pub str_0: [::core::ffi::c_char; 0],
    }
    use super::_size_t_h::size_t;
    extern "C" {
        
        pub type StrPool;
    }
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

pub mod _stdio_h {
    use super::_size_t_h::size_t;
    extern "C" {
        
        pub fn sprintf(
            _: *mut ::core::ffi::c_char,
            _: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        
        pub fn snprintf(
            __str: *mut ::core::ffi::c_char,
            __size: size_t,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}

pub mod _malloc_h {
    use super::_size_t_h::size_t;
    extern "C" {
        
        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
        
        pub fn calloc(__count: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
        
        pub fn free(_: *mut ::core::ffi::c_void);
        
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t)
            -> *mut ::core::ffi::c_void;
    }
}

pub mod base_h {
    #[inline]
    
    pub unsafe extern "C" fn zmalloc(mut len: size_t) -> *mut ::core::ffi::c_void {
        calloc(1 as size_t, len)
    }
    use super::_malloc_h::calloc;
    use super::_size_t_h::size_t;
}

pub mod _null_h {
    
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
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
        
        pub fn strcpy(
            __dst: *mut ::core::ffi::c_char,
            __src: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        
        pub fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
        
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}

pub mod _stdlib_h {
    extern "C" {
        
        pub fn exit(_: ::core::ffi::c_int) -> !;
    }
}

pub mod errno_h {
    
    pub const EAGAIN: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
    extern "C" {
        
        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}

pub mod stdbool_h {
    
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}

pub mod protocol_h {
    
    pub const PqMsg_Bind: ::core::ffi::c_int = 'B' as i32;
    
    pub const PqMsg_Describe: ::core::ffi::c_int = 'D' as i32;
    
    pub const PqMsg_Execute: ::core::ffi::c_int = 'E' as i32;
    
    pub const PqMsg_Parse: ::core::ffi::c_int = 'P' as i32;
    
    pub const PqMsg_Sync: ::core::ffi::c_int = 'S' as i32;
    
    pub const PqMsg_DataRow: ::core::ffi::c_int = 'D' as i32;
    
    pub const PqMsg_RowDescription: ::core::ffi::c_int = 'T' as i32;
}
pub use self::_in_addr_t_h::in_addr_t;
pub use self::_in_port_t_h::in_port_t;
pub use self::_int64_t_h::int64_t;
use self::_malloc_h::{free, malloc, realloc};
pub use self::_null_h::NULL;
pub use self::_pid_t_h::pid_t;
pub use self::_ptrdiff_t_h::ptrdiff_t;
pub use self::_sa_family_t_h::sa_family_t;
pub use self::_size_t_h::size_t;
pub use self::_ssize_t_h::ssize_t;
use self::_stdio_h::{snprintf, sprintf};
use self::_stdlib_h::exit;
use self::_string_h::{memcpy, memset, strcpy, strerror, strlen};
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
pub use self::base_h::zmalloc;
pub use self::bouncer_h::{
    pgb_event_base, sockaddr_ucreds, C2RustUnnamed_9, CallbackState, LoadBalanceHosts,
    PacketCallbackFlag, PgAddr, PgCredentials, PgDatabase, PgGlobalUser, PgPool, PgSocket, PgStats,
    ReplicationType, ScramState, SocketState, CB_HANDLE_COMPLETE_PACKET, CB_NONE,
    CB_WANT_COMPLETE_PACKET, CL_ACTIVE, CL_ACTIVE_CANCEL, CL_FREE, CL_JUSTFREE, CL_LOGIN,
    CL_WAITING, CL_WAITING_CANCEL, CL_WAITING_LOGIN, LOAD_BALANCE_HOSTS_DISABLE,
    LOAD_BALANCE_HOSTS_ROUND_ROBIN, REPLICATION_LOGICAL, REPLICATION_NONE, REPLICATION_PHYSICAL,
    SV_ACTIVE, SV_ACTIVE_CANCEL, SV_BEING_CANCELED, SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN,
    SV_TESTED, SV_USED,
};
pub use self::cryptohash_h::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};

pub use self::errno_h::{__error, EAGAIN};
pub use self::event_h::{event_add, event_assign, event_base, event_callback_fn, EV_WRITE};
pub use self::event_struct_h::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
pub use self::in6_h::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use self::in_h::{in_addr, sockaddr_in};
pub use self::internal::__builtin_va_list;
pub use self::iobuf_h::{iobuf, IOBuf};
pub use self::list_h::List;
pub use self::logging_h::{
    cf_verbose, log_fatal, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE,
    LG_STATS, LG_WARNING,
};
pub use self::mbuf_h::MBuf;
pub use self::pktbuf_h::PktBuf;
pub use self::prepare_h::{
    PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement,
};
pub use self::proto_h::{send_pooler_error, PktHdr};
pub use self::protocol_h::{
    PqMsg_Bind, PqMsg_DataRow, PqMsg_Describe, PqMsg_Execute, PqMsg_Parse, PqMsg_RowDescription,
    PqMsg_Sync,
};
pub use self::sbuf_h::{
    sbuf_cb_t, sbuf_op_send, SBuf, SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK,
    SBUF_EV_FLUSH, SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY,
};
pub use self::socket_h::sockaddr;
pub use self::statlist_h::StatList;
pub use self::stdbool_h::{false_0, true_0};
pub use self::strpool_h::{PStr, StrPool};
pub use self::sys__types_h::{__darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t, __DARWIN_NULL};
pub use self::time_h::{format_time_s, usec_t};

pub use self::uthash_h::{UT_hash_bucket, UT_hash_handle, UT_hash_table};
pub use self::varcache_h::VarCache;

pub const BYTEAOID: ::core::ffi::c_int = 17 as ::core::ffi::c_int;

pub const INT8OID: ::core::ffi::c_int = 20 as ::core::ffi::c_int;

pub const INT4OID: ::core::ffi::c_int = 23 as ::core::ffi::c_int;

pub const TEXTOID: ::core::ffi::c_int = 25 as ::core::ffi::c_int;

pub const NUMERICOID: ::core::ffi::c_int = 1700 as ::core::ffi::c_int;

unsafe extern "C" fn pktbuf_free_internal(mut buf: *mut PktBuf) {
    if buf.is_null() || (*buf).fixed_buf() as ::core::ffi::c_int != 0 {
        return;
    }
    let mut _log_ctx = NULL;
    if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_DEBUG,
            _log_ctx,
            b"pktbuf_free(%p)\0" as *const u8 as *const ::core::ffi::c_char,
            buf,
        );
    }
    free((*buf).buf as *mut ::core::ffi::c_void);
    free((*buf).ev as *mut ::core::ffi::c_void);
    free(buf as *mut ::core::ffi::c_void);
}

static mut temp_pktbuf: *mut PktBuf = ::core::ptr::null::<PktBuf>() as *mut PktBuf;
#[no_mangle]

pub unsafe extern "C" fn pktbuf_free(mut buf: *mut PktBuf) {
    if buf == temp_pktbuf {
        return;
    }
    pktbuf_free_internal(buf);
}
#[no_mangle]

pub unsafe extern "C" fn pktbuf_dynamic(mut start_len: ::core::ffi::c_int) -> *mut PktBuf {
    let mut buf = zmalloc(::core::mem::size_of::<PktBuf>() as size_t) as *mut PktBuf;
    let mut _log_ctx = NULL;
    if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_DEBUG,
            _log_ctx,
            b"pktbuf_dynamic(%d): %p\0" as *const u8 as *const ::core::ffi::c_char,
            start_len,
            buf,
        );
    }
    if buf.is_null() {
        return ::core::ptr::null_mut::<PktBuf>();
    }
    (*buf).ev = zmalloc(::core::mem::size_of::<event>() as size_t) as *mut event;
    if (*buf).ev.is_null() {
        pktbuf_free(buf);
        return ::core::ptr::null_mut::<PktBuf>();
    }
    (*buf).buf = malloc(start_len as size_t) as *mut uint8_t;
    if (*buf).buf.is_null() {
        pktbuf_free(buf);
        return ::core::ptr::null_mut::<PktBuf>();
    }
    (*buf).buf_len = start_len;
    buf
}
#[no_mangle]

pub unsafe extern "C" fn pktbuf_reset(mut pkt: *mut PktBuf) {
    (*pkt).set_failed(false_0 != 0);
    (*pkt).write_pos = 0 as ::core::ffi::c_int;
    (*pkt).pktlen_pos = 0 as ::core::ffi::c_int;
    (*pkt).send_pos = 0 as ::core::ffi::c_int;
    (*pkt).set_sending(false_0 != 0);
}
#[no_mangle]

pub unsafe extern "C" fn pktbuf_static(
    mut buf: *mut PktBuf,
    mut data: *mut uint8_t,
    mut len: ::core::ffi::c_int,
) {
    memset(
        buf as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PktBuf>() as size_t,
    );
    (*buf).buf = data;
    (*buf).buf_len = len;
    (*buf).set_fixed_buf(true_0 != 0);
}
#[no_mangle]

pub unsafe extern "C" fn pktbuf_temp() -> *mut PktBuf {
    if temp_pktbuf.is_null() {
        temp_pktbuf = pktbuf_dynamic(512 as ::core::ffi::c_int);
    }
    if temp_pktbuf.is_null() {
        let mut _log_ctx = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx,
            b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    pktbuf_reset(temp_pktbuf as *mut PktBuf);
    temp_pktbuf as *mut PktBuf
}
#[no_mangle]

pub unsafe extern "C" fn pktbuf_cleanup() {
    pktbuf_free_internal(temp_pktbuf);
    temp_pktbuf = ::core::ptr::null_mut::<PktBuf>();
}
#[no_mangle]

pub unsafe extern "C" fn pktbuf_send_immediate(
    mut buf: *mut PktBuf,
    mut sk: *mut PgSocket,
) -> bool {
    let mut pos = (*buf).buf.offset((*buf).send_pos as isize);
    let mut amount = (*buf).write_pos - (*buf).send_pos;
    let mut res: ssize_t = 0;
    if (*buf).failed() {
        return false_0 != 0;
    }
    res = sbuf_op_send(
        &raw mut (*sk).sbuf,
        pos as *const ::core::ffi::c_void,
        amount as size_t,
    );
    if res < 0 as ssize_t {
        let mut _log_ctx = NULL;
        if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_DEBUG,
                _log_ctx,
                b"pktbuf_send_immediate: %s\0" as *const u8 as *const ::core::ffi::c_char,
                strerror(*__error()),
            );
        }
    }
    res == amount as ssize_t
}

unsafe extern "C" fn pktbuf_send_func(
    mut fd: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_short,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut buf = arg as *mut PktBuf;
    let mut sbuf: *mut SBuf = &raw mut (*(*buf).queued_dst).sbuf;
    let mut amount: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 0;
    let mut _log_ctx = NULL;
    if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_DEBUG,
            _log_ctx,
            b"pktbuf_send_func(%lld, %d, %p)\0" as *const u8 as *const ::core::ffi::c_char,
            fd as int64_t,
            flags as ::core::ffi::c_int,
            buf,
        );
    }
    if (*buf).failed() {
        return;
    }
    amount = (*buf).write_pos - (*buf).send_pos;
    res = sbuf_op_send(
        sbuf,
        (*buf).buf.offset((*buf).send_pos as isize) as *const ::core::ffi::c_void,
        amount as size_t,
    ) as ::core::ffi::c_int;
    if res < 0 as ::core::ffi::c_int {
        if *__error() == EAGAIN {
            res = 0 as ::core::ffi::c_int;
        } else {
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_0,
                b"pktbuf_send_func: %s\0" as *const u8 as *const ::core::ffi::c_char,
                strerror(*__error()),
            );
            pktbuf_free(buf);
            return;
        }
    }
    (*buf).send_pos += res;
    if (*buf).send_pos < (*buf).write_pos {
        event_assign(
            (*buf).ev,
            pgb_event_base,
            fd,
            EV_WRITE as ::core::ffi::c_short,
            Some(
                pktbuf_send_func
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_short,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
            buf as *mut ::core::ffi::c_void,
        );
        res = event_add((*buf).ev, ::core::ptr::null::<timeval>());
        if res < 0 as ::core::ffi::c_int {
            let mut _log_ctx_1 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_1,
                b"pktbuf_send_func: %s\0" as *const u8 as *const ::core::ffi::c_char,
                strerror(*__error()),
            );
            pktbuf_free(buf);
        }
    } else {
        pktbuf_free(buf);
    };
}
#[no_mangle]

pub unsafe extern "C" fn pktbuf_send_queued(mut buf: *mut PktBuf, mut sk: *mut PgSocket) -> bool {
    if (*buf).failed() {
        pktbuf_free(buf);
        send_pooler_error(
            sk,
            true_0 != 0,
            ::core::ptr::null::<::core::ffi::c_char>(),
            false_0 != 0,
            b"result prepare failed\0" as *const u8 as *const ::core::ffi::c_char,
        )
    } else {
        (*buf).set_sending(true_0 != 0);
        (*buf).queued_dst = sk;
        pktbuf_send_func(
            (*sk).sbuf.sock,
            EV_WRITE as ::core::ffi::c_short,
            buf as *mut ::core::ffi::c_void,
        );
        true_0 != 0
    }
}

unsafe extern "C" fn make_room(mut buf: *mut PktBuf, mut len: ::core::ffi::c_int) {
    let mut newlen = (*buf).buf_len;
    let mut need = (*buf).write_pos + len;
    let mut ptr = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if newlen >= need {
        return;
    }
    if (*buf).failed() {
        return;
    }
    if (*buf).fixed_buf() {
        (*buf).set_failed(true_0 != 0);
        return;
    }
    while newlen < need {
        newlen *= 2 as ::core::ffi::c_int;
    }
    let mut _log_ctx = NULL;
    if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_DEBUG,
            _log_ctx,
            b"make_room(%p, %d): realloc newlen=%d\0" as *const u8 as *const ::core::ffi::c_char,
            buf,
            len,
            newlen,
        );
    }
    ptr = realloc((*buf).buf as *mut ::core::ffi::c_void, newlen as size_t);
    if ptr.is_null() {
        (*buf).set_failed(true_0 != 0);
    } else {
        (*buf).buf = ptr as *mut uint8_t;
        (*buf).buf_len = newlen;
    };
}
#[no_mangle]

pub unsafe extern "C" fn pktbuf_put_char(mut buf: *mut PktBuf, mut val: ::core::ffi::c_char) {
    make_room(buf, 1 as ::core::ffi::c_int);
    if (*buf).failed() {
        return;
    }
    let fresh0 = (*buf).write_pos;
    (*buf).write_pos += 1;
    *(*buf).buf.offset(fresh0 as isize) = val as uint8_t;
}
#[no_mangle]

pub unsafe extern "C" fn pktbuf_put_uint16(mut buf: *mut PktBuf, mut val: uint16_t) {
    make_room(buf, 4 as ::core::ffi::c_int);
    if (*buf).failed() {
        return;
    }
    let fresh1 = (*buf).write_pos;
    (*buf).write_pos += 1;
    *(*buf).buf.offset(fresh1 as isize) = (val as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
        & 255 as ::core::ffi::c_int) as uint8_t;
    let fresh2 = (*buf).write_pos;
    (*buf).write_pos += 1;
    *(*buf).buf.offset(fresh2 as isize) =
        (val as ::core::ffi::c_int & 255 as ::core::ffi::c_int) as uint8_t;
}
#[no_mangle]

pub unsafe extern "C" fn pktbuf_put_uint32(mut buf: *mut PktBuf, mut val: uint32_t) {
    let mut pos = ::core::ptr::null_mut::<uint8_t>();
    make_room(buf, 4 as ::core::ffi::c_int);
    if (*buf).failed() {
        return;
    }
    pos = (*buf).buf.offset((*buf).write_pos as isize);
    *pos.offset(0 as ::core::ffi::c_int as isize) =
        (val >> 24 as ::core::ffi::c_int & 255 as uint32_t) as uint8_t;
    *pos.offset(1 as ::core::ffi::c_int as isize) =
        (val >> 16 as ::core::ffi::c_int & 255 as uint32_t) as uint8_t;
    *pos.offset(2 as ::core::ffi::c_int as isize) =
        (val >> 8 as ::core::ffi::c_int & 255 as uint32_t) as uint8_t;
    *pos.offset(3 as ::core::ffi::c_int as isize) = (val & 255 as uint32_t) as uint8_t;
    (*buf).write_pos += 4 as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn pktbuf_put_uint64(mut buf: *mut PktBuf, mut val: uint64_t) {
    pktbuf_put_uint32(buf, (val >> 32 as ::core::ffi::c_int) as uint32_t);
    pktbuf_put_uint32(buf, val as uint32_t);
}
#[no_mangle]

pub unsafe extern "C" fn pktbuf_put_bytes(
    mut buf: *mut PktBuf,
    mut data: *const ::core::ffi::c_void,
    mut len: ::core::ffi::c_int,
) {
    if len == 0 as ::core::ffi::c_int {
        return;
    }
    make_room(buf, len);
    if (*buf).failed() {
        return;
    }
    memcpy(
        (*buf).buf.offset((*buf).write_pos as isize) as *mut ::core::ffi::c_void,
        data,
        len as size_t,
    );
    (*buf).write_pos += len;
}
#[no_mangle]

pub unsafe extern "C" fn pktbuf_put_string(
    mut buf: *mut PktBuf,
    mut str: *const ::core::ffi::c_char,
) {
    let mut len = strlen(str) as ::core::ffi::c_int;
    pktbuf_put_bytes(
        buf,
        str as *const ::core::ffi::c_void,
        len + 1 as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn pktbuf_start_packet(mut buf: *mut PktBuf, mut type_0: ::core::ffi::c_int) {
    if (*buf).failed() {
        return;
    }
    if type_0 < 256 as ::core::ffi::c_int {
        pktbuf_put_char(buf, type_0 as ::core::ffi::c_char);
        (*buf).pktlen_pos = (*buf).write_pos;
        pktbuf_put_uint32(buf, 0 as uint32_t);
    } else {
        (*buf).pktlen_pos = (*buf).write_pos;
        pktbuf_put_uint32(buf, 0 as uint32_t);
        pktbuf_put_uint32(buf, type_0 as uint32_t);
    };
}
#[no_mangle]

pub unsafe extern "C" fn pktbuf_finish_packet(mut buf: *mut PktBuf) {
    let mut pos = ::core::ptr::null_mut::<uint8_t>();
    let mut len: ::core::ffi::c_uint = 0;
    if (*buf).failed() {
        return;
    }
    len = ((*buf).write_pos - (*buf).pktlen_pos) as ::core::ffi::c_uint;
    pos = (*buf).buf.offset((*buf).pktlen_pos as isize);
    (*buf).pktlen_pos = 0 as ::core::ffi::c_int;
    let fresh3 = pos;
    pos = pos.offset(1);
    *fresh3 = (len >> 24 as ::core::ffi::c_int & 255 as ::core::ffi::c_uint) as uint8_t;
    let fresh4 = pos;
    pos = pos.offset(1);
    *fresh4 = (len >> 16 as ::core::ffi::c_int & 255 as ::core::ffi::c_uint) as uint8_t;
    let fresh5 = pos;
    pos = pos.offset(1);
    *fresh5 = (len >> 8 as ::core::ffi::c_int & 255 as ::core::ffi::c_uint) as uint8_t;
    let fresh6 = pos;
    pos = pos.offset(1);
    *fresh6 = (len & 255 as ::core::ffi::c_uint) as uint8_t;
}
#[no_mangle]

pub unsafe extern "C" fn pktbuf_write_generic(
    mut buf: *mut PktBuf,
    mut type_0: ::core::ffi::c_int,
    mut pktdesc: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    let mut len: ::core::ffi::c_int = 0;
    let mut adesc = pktdesc;
    let mut bin = ::core::ptr::null_mut::<uint8_t>();
    pktbuf_start_packet(buf, type_0);
    ap = args.clone();
    while *adesc != 0 {
        match *adesc as ::core::ffi::c_int {
            99 => {
                pktbuf_put_char(buf, ap.arg::<::core::ffi::c_int>() as ::core::ffi::c_char);
            }
            104 => {
                pktbuf_put_uint16(buf, ap.arg::<::core::ffi::c_int>() as uint16_t);
            }
            105 => {
                pktbuf_put_uint32(buf, ap.arg::<::core::ffi::c_int>() as uint32_t);
            }
            113 => {
                pktbuf_put_uint64(buf, ap.arg::<uint64_t>());
            }
            115 => {
                pktbuf_put_string(buf, ap.arg::<*mut ::core::ffi::c_char>());
            }
            98 => {
                bin = ap.arg::<*mut uint8_t>();
                len = ap.arg::<::core::ffi::c_int>();
                pktbuf_put_bytes(buf, bin as *const ::core::ffi::c_void, len);
            }
            _ => {
                let mut _log_ctx = NULL;
                log_fatal(
                    b"src/pktbuf.c\0" as *const u8 as *const ::core::ffi::c_char,
                    353 as ::core::ffi::c_int,
                    b"pktbuf_write_generic\0" as *const u8 as *const ::core::ffi::c_char,
                    false_0 != 0,
                    _log_ctx,
                    b"bad pktdesc: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    pktdesc,
                );
                exit(1 as ::core::ffi::c_int);
            }
        }
        adesc = adesc.offset(1);
    }
    pktbuf_finish_packet(buf);
}
#[no_mangle]

pub unsafe extern "C" fn pktbuf_write_RowDescription(
    mut buf: *mut PktBuf,
    mut tupdesc: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    let mut name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut ncol = strlen(tupdesc) as ::core::ffi::c_int;
    let mut _log_ctx = NULL;
    if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_NOISE,
            _log_ctx,
            b"write RowDescription\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    pktbuf_start_packet(buf, PqMsg_RowDescription);
    pktbuf_put_uint16(buf, ncol as uint16_t);
    ap = args.clone();
    i = 0 as ::core::ffi::c_int;
    while i < ncol {
        name = ap.arg::<*mut ::core::ffi::c_char>();
        pktbuf_put_string(buf, name);
        pktbuf_put_uint32(buf, 0 as uint32_t);
        pktbuf_put_uint16(buf, 0 as uint16_t);
        if *tupdesc.offset(i as isize) as ::core::ffi::c_int == 's' as i32 {
            pktbuf_put_uint32(buf, TEXTOID as uint32_t);
            pktbuf_put_uint16(buf, -(1 as ::core::ffi::c_int) as uint16_t);
        } else if *tupdesc.offset(i as isize) as ::core::ffi::c_int == 'b' as i32 {
            pktbuf_put_uint32(buf, BYTEAOID as uint32_t);
            pktbuf_put_uint16(buf, -(1 as ::core::ffi::c_int) as uint16_t);
        } else if *tupdesc.offset(i as isize) as ::core::ffi::c_int == 'i' as i32 {
            pktbuf_put_uint32(buf, INT4OID as uint32_t);
            pktbuf_put_uint16(buf, 4 as uint16_t);
        } else if *tupdesc.offset(i as isize) as ::core::ffi::c_int == 'q' as i32 {
            pktbuf_put_uint32(buf, INT8OID as uint32_t);
            pktbuf_put_uint16(buf, 8 as uint16_t);
        } else if *tupdesc.offset(i as isize) as ::core::ffi::c_int == 'N' as i32 {
            pktbuf_put_uint32(buf, NUMERICOID as uint32_t);
            pktbuf_put_uint16(buf, -(1 as ::core::ffi::c_int) as uint16_t);
        } else if *tupdesc.offset(i as isize) as ::core::ffi::c_int == 'T' as i32 {
            pktbuf_put_uint32(buf, TEXTOID as uint32_t);
            pktbuf_put_uint16(buf, -(1 as ::core::ffi::c_int) as uint16_t);
        } else {
            let mut _log_ctx_0 = NULL;
            log_fatal(
                b"src/pktbuf.c\0" as *const u8 as *const ::core::ffi::c_char,
                412 as ::core::ffi::c_int,
                b"pktbuf_write_RowDescription\0" as *const u8 as *const ::core::ffi::c_char,
                false_0 != 0,
                _log_ctx_0,
                b"bad tupdesc\0" as *const u8 as *const ::core::ffi::c_char,
            );
            exit(1 as ::core::ffi::c_int);
        }
        pktbuf_put_uint32(buf, -(1 as ::core::ffi::c_int) as uint32_t);
        pktbuf_put_uint16(buf, 0 as uint16_t);
        i += 1;
    }
    pktbuf_finish_packet(buf);
}
#[no_mangle]

pub unsafe extern "C" fn pktbuf_write_DataRow(
    mut buf: *mut PktBuf,
    mut tupdesc: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut ncol = strlen(tupdesc) as ::core::ffi::c_int;
    let mut ap: ::core::ffi::VaListImpl;
    pktbuf_start_packet(buf, PqMsg_DataRow);
    pktbuf_put_uint16(buf, ncol as uint16_t);
    ap = args.clone();
    let mut i = 0 as ::core::ffi::c_int;
    while i < ncol {
        let mut tmp: [::core::ffi::c_char; 100] = [0; 100];
        let mut val = ::core::ptr::null::<::core::ffi::c_char>();
        if *tupdesc.offset(i as isize) as ::core::ffi::c_int == 'i' as i32 {
            snprintf(
                &raw mut tmp as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as size_t,
                b"%d\0" as *const u8 as *const ::core::ffi::c_char,
                ap.arg::<::core::ffi::c_int>(),
            );
            val = &raw mut tmp as *mut ::core::ffi::c_char;
        } else if *tupdesc.offset(i as isize) as ::core::ffi::c_int == 'q' as i32
            || *tupdesc.offset(i as isize) as ::core::ffi::c_int == 'N' as i32
        {
            snprintf(
                &raw mut tmp as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as size_t,
                b"%llu\0" as *const u8 as *const ::core::ffi::c_char,
                ap.arg::<uint64_t>(),
            );
            val = &raw mut tmp as *mut ::core::ffi::c_char;
        } else if *tupdesc.offset(i as isize) as ::core::ffi::c_int == 's' as i32 {
            val = ap.arg::<*mut ::core::ffi::c_char>();
        } else if *tupdesc.offset(i as isize) as ::core::ffi::c_int == 'b' as i32 {
            let mut blen = ap.arg::<::core::ffi::c_int>();
            if blen >= 0 as ::core::ffi::c_int {
                let mut bval = ap.arg::<*mut uint8_t>();
                let mut required: size_t = (2 as ::core::ffi::c_int
                    + blen * 2 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as size_t;
                if required > ::core::mem::size_of::<[::core::ffi::c_char; 100]>() {
                    let mut _log_ctx = NULL;
                    log_fatal(
                        b"src/pktbuf.c\0" as *const u8 as *const ::core::ffi::c_char,
                        462 as ::core::ffi::c_int,
                        b"pktbuf_write_DataRow\0" as *const u8 as *const ::core::ffi::c_char,
                        false_0 != 0,
                        _log_ctx,
                        b"byte array too long (%zu > %zu)\0" as *const u8
                            as *const ::core::ffi::c_char,
                        required,
                        ::core::mem::size_of::<[::core::ffi::c_char; 100]>(),
                    );
                    exit(1 as ::core::ffi::c_int);
                }
                strcpy(
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    b"\\x\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let mut j = 0 as ::core::ffi::c_int;
                while j < blen {
                    sprintf(
                        (&raw mut tmp as *mut ::core::ffi::c_char).offset(
                            (2 as ::core::ffi::c_int + j * 2 as ::core::ffi::c_int) as isize,
                        ),
                        b"%02x\0" as *const u8 as *const ::core::ffi::c_char,
                        *bval.offset(j as isize) as ::core::ffi::c_int,
                    );
                    j += 1;
                }
                val = &raw mut tmp as *mut ::core::ffi::c_char;
            } else {
                ap.arg::<*mut uint8_t>();
                val = ::core::ptr::null::<::core::ffi::c_char>();
            }
        } else if *tupdesc.offset(i as isize) as ::core::ffi::c_int == 'T' as i32 {
            let mut time: usec_t = ap.arg::<usec_t>();
            val = format_time_s(
                time,
                &raw mut tmp as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_uint,
            );
        } else {
            let mut _log_ctx_0 = NULL;
            log_fatal(
                b"src/pktbuf.c\0" as *const u8 as *const ::core::ffi::c_char,
                475 as ::core::ffi::c_int,
                b"pktbuf_write_DataRow\0" as *const u8 as *const ::core::ffi::c_char,
                false_0 != 0,
                _log_ctx_0,
                b"bad tupdesc: %s\0" as *const u8 as *const ::core::ffi::c_char,
                tupdesc,
            );
            exit(1 as ::core::ffi::c_int);
        }
        if !val.is_null() {
            let mut len = strlen(val) as ::core::ffi::c_int;
            pktbuf_put_uint32(buf, len as uint32_t);
            pktbuf_put_bytes(buf, val as *const ::core::ffi::c_void, len);
        } else {
            pktbuf_put_uint32(buf, -(1 as ::core::ffi::c_int) as uint32_t);
        }
        i += 1;
    }
    pktbuf_finish_packet(buf);
}
#[no_mangle]

pub unsafe extern "C" fn pktbuf_write_ExtQuery(
    mut buf: *mut PktBuf,
    mut query: *const ::core::ffi::c_char,
    mut nargs: ::core::ffi::c_int,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    let mut val = ::core::ptr::null::<::core::ffi::c_char>();
    let mut len: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    pktbuf_write_generic(
        buf,
        PqMsg_Parse,
        b"csh\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        query,
        0 as ::core::ffi::c_int,
    );
    pktbuf_start_packet(buf, PqMsg_Bind);
    pktbuf_put_char(buf, 0 as ::core::ffi::c_char);
    pktbuf_put_char(buf, 0 as ::core::ffi::c_char);
    pktbuf_put_uint16(buf, 0 as uint16_t);
    pktbuf_put_uint16(buf, nargs as uint16_t);
    ap = args.clone();
    i = 0 as ::core::ffi::c_int;
    while i < nargs {
        val = ap.arg::<*mut ::core::ffi::c_char>();
        len = strlen(val) as ::core::ffi::c_int;
        pktbuf_put_uint32(buf, len as uint32_t);
        pktbuf_put_bytes(buf, val as *const ::core::ffi::c_void, len);
        i += 1;
    }
    pktbuf_put_uint16(buf, 0 as uint16_t);
    pktbuf_finish_packet(buf);
    pktbuf_write_generic(
        buf,
        PqMsg_Describe,
        b"cc\0" as *const u8 as *const ::core::ffi::c_char,
        'P' as i32,
        0 as ::core::ffi::c_int,
    );
    pktbuf_write_generic(
        buf,
        PqMsg_Execute,
        b"ci\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    pktbuf_write_generic(
        buf,
        PqMsg_Sync,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
