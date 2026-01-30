
pub mod _types_h {
    
    pub type __uint8_t = u8;
    
    pub type __uint16_t = u16;
    
    pub type __int32_t = i32;
    
    pub type __uint32_t = u32;
    
    pub type __darwin_ptrdiff_t = isize;
    
    pub type __darwin_size_t = usize;
    
    pub type __darwin_ssize_t = isize;
    
    pub type __darwin_time_t = ::core::ffi::c_long;
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


pub mod list_h {
    pub use super::super::common::types::List;
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
    
    pub const EV_PERSIST: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
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
    use crate::types::{AANode, AATree};
    use super::cryptohash_h::pg_cryptohash_type;
    use super::dnslookup_h::DNSToken;
    use super::event_h::event_base;
    use super::in6_h::sockaddr_in6;
    use super::in_h::sockaddr_in;
    use super::list_h::List;
    use super::pktbuf_h::PktBuf;
    use crate::types::{PgClientPreparedStatement, PgServerPreparedStatement};
    use crate::types::PktHdr;
    use super::sbuf_h::SBuf;
    use super::socket_h::sockaddr;
    use crate::types::StatList;
    use crate::types::usec_t;
    use super::varcache_h::VarCache;
    extern "C" {
        
        pub static mut pgb_event_base: *mut event_base;
        
        pub static mut cf_stats_period: ::core::ffi::c_int;
        
        pub static mut cf_log_stats: ::core::ffi::c_int;
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

pub mod varcache_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct VarCache {
        pub var_list: *mut *mut PStr,
    }
    use crate::types::PStr;
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
    extern "C" {
        
        pub fn pktbuf_dynamic(start_len: ::core::ffi::c_int) -> *mut PktBuf;
        
        pub fn pktbuf_write_RowDescription(
            buf: *mut PktBuf,
            tupdesc: *const ::core::ffi::c_char,
            ...
        );
        
        pub fn pktbuf_write_DataRow(buf: *mut PktBuf, tupdesc: *const ::core::ffi::c_char, ...);
    }
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
        
        pub fn log_generic(
            level: LogLevel,
            ctx: *mut ::core::ffi::c_void,
            s: *const ::core::ffi::c_char,
            ...
        );
    }
}

pub mod _null_h {
    
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}

pub mod admin_h {
    use super::bouncer_h::PgSocket;
    use super::pktbuf_h::PktBuf;
    extern "C" {
        
        pub fn admin_error(console: *mut PgSocket, fmt: *const ::core::ffi::c_char, ...) -> bool;
        
        pub fn admin_flush(
            admin: *mut PgSocket,
            buf: *mut PktBuf,
            desc: *const ::core::ffi::c_char,
        ) -> bool;
    }
}

pub mod objects_h {

    use crate::types::StatList;
    extern "C" {
        
        pub static mut pool_list: StatList;
    }
}

pub mod errno_h {
    extern "C" {
        
        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}

pub mod stdbool_h {
    
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
}

pub mod _string_h {
    extern "C" {
        
        pub fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    }
}
pub use self::_in_addr_t_h::in_addr_t;
pub use self::_in_port_t_h::in_port_t;
pub use self::_null_h::NULL;
pub use self::_pid_t_h::pid_t;
pub use self::_ptrdiff_t_h::ptrdiff_t;
pub use self::_sa_family_t_h::sa_family_t;
pub use self::_size_t_h::size_t;
pub use self::_ssize_t_h::ssize_t;
use self::_string_h::strerror;
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
pub use crate::types::{aatree_cmp_f, aatree_walker_f, AANode, AATree};
use self::admin_h::{admin_error, admin_flush};
pub use self::bouncer_h::{
    cf_log_stats, cf_stats_period, pgb_event_base, sockaddr_ucreds, C2RustUnnamed_9, CallbackState,
    LoadBalanceHosts, PacketCallbackFlag, PgAddr, PgCredentials, PgDatabase, PgGlobalUser, PgPool,
    PgSocket, PgStats, ReplicationType, ScramState, SocketState, CB_HANDLE_COMPLETE_PACKET,
    CB_NONE, CB_WANT_COMPLETE_PACKET, CL_ACTIVE, CL_ACTIVE_CANCEL, CL_FREE, CL_JUSTFREE, CL_LOGIN,
    CL_WAITING, CL_WAITING_CANCEL, CL_WAITING_LOGIN, LOAD_BALANCE_HOSTS_DISABLE,
    LOAD_BALANCE_HOSTS_ROUND_ROBIN, REPLICATION_LOGICAL, REPLICATION_NONE, REPLICATION_PHYSICAL,
    SV_ACTIVE, SV_ACTIVE_CANCEL, SV_BEING_CANCELED, SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN,
    SV_TESTED, SV_USED,
};
pub use self::cryptohash_h::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};

use self::errno_h::__error;
pub use self::event_h::{event_add, event_assign, event_base, event_callback_fn, EV_PERSIST};
pub use self::event_struct_h::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
pub use self::in6_h::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use self::in_h::{in_addr, sockaddr_in};
pub use self::iobuf_h::{iobuf, IOBuf};
pub use self::list_h::List;
pub use self::logging_h::{
    log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE, LG_STATS, LG_WARNING,
};
pub use crate::types::MBuf;
use self::objects_h::pool_list;
pub use self::pktbuf_h::{
    pktbuf_dynamic, pktbuf_write_DataRow, pktbuf_write_RowDescription, PktBuf,
};
pub use crate::types::{
    PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement,
};
pub use crate::types::PktHdr;
pub use self::sbuf_h::{
    sbuf_cb_t, SBuf, SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK, SBUF_EV_FLUSH,
    SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY,
};
pub use self::socket_h::sockaddr;
pub use crate::types::StatList;
pub use self::stdbool_h::true_0;
pub use crate::types::{PStr, StrPool};
pub use self::sys__types_h::{__darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t, __DARWIN_NULL};
pub use crate::types::{usec_t, USEC};

pub use self::uthash_h::{UT_hash_bucket, UT_hash_handle, UT_hash_table};
pub use self::varcache_h::VarCache;

static mut ev_stats: event = event {
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

static mut old_stamp: usec_t = 0;

static mut new_stamp: usec_t = 0;

unsafe fn reset_stats(stat: *mut PgStats) {
    (*stat).server_bytes = 0;
    (*stat).client_bytes = 0;
    (*stat).server_assignment_count = 0;
    (*stat).query_count = 0;
    (*stat).query_time = 0;
    (*stat).xact_count = 0;
    (*stat).xact_time = 0;
    (*stat).wait_time = 0;
    (*stat).ps_client_parse_count = 0;
    (*stat).ps_server_parse_count = 0;
    (*stat).ps_bind_count = 0;
}

unsafe fn stat_add(total: *mut PgStats, stat: *mut PgStats) {
    (*total).server_bytes += (*stat).server_bytes;
    (*total).client_bytes += (*stat).client_bytes;
    (*total).server_assignment_count += (*stat).server_assignment_count;
    (*total).query_count += (*stat).query_count;
    (*total).query_time += (*stat).query_time;
    (*total).xact_count += (*stat).xact_count;
    (*total).xact_time += (*stat).xact_time;
    (*total).wait_time += (*stat).wait_time;
    (*total).ps_client_parse_count += (*stat).ps_client_parse_count;
    (*total).ps_server_parse_count += (*stat).ps_server_parse_count;
    (*total).ps_bind_count += (*stat).ps_bind_count;
}

unsafe fn calc_average(avg: *mut PgStats, cur: *mut PgStats, old: *mut PgStats) {
    let dur: usec_t = get_cached_time() - old_stamp;
    reset_stats(avg);
    if dur == 0 {
        return;
    }

    let query_count = (*cur).query_count - (*old).query_count;
    let xact_count = (*cur).xact_count - (*old).xact_count;
    let server_assignment_count = (*cur).server_assignment_count - (*old).server_assignment_count;

    (*avg).query_count = (USEC * query_count) / dur;
    (*avg).xact_count = (USEC * xact_count) / dur;
    (*avg).server_assignment_count = (USEC * server_assignment_count) / dur;
    (*avg).client_bytes = (USEC * ((*cur).client_bytes - (*old).client_bytes)) / dur;
    (*avg).server_bytes = (USEC * ((*cur).server_bytes - (*old).server_bytes)) / dur;

    if query_count > 0 {
        (*avg).query_time = ((*cur).query_time - (*old).query_time) / query_count;
    }
    if xact_count > 0 {
        (*avg).xact_time = ((*cur).xact_time - (*old).xact_time) / xact_count;
    }
    if server_assignment_count > 0 {
        (*avg).wait_time = ((*cur).wait_time - (*old).wait_time) / server_assignment_count;
    }

    let ps_client_parse_count = (*cur).ps_client_parse_count - (*old).ps_client_parse_count;
    let ps_server_parse_count = (*cur).ps_server_parse_count - (*old).ps_server_parse_count;
    let ps_bind_count = (*cur).ps_bind_count - (*old).ps_bind_count;

    (*avg).ps_client_parse_count = (USEC * ps_client_parse_count) / dur;
    (*avg).ps_server_parse_count = (USEC * ps_server_parse_count) / dur;
    (*avg).ps_bind_count = (USEC * ps_bind_count) / dur;
}

unsafe extern "C" fn write_stats(
    mut buf: *mut PktBuf,
    mut stat: *mut PgStats,
    mut old: *mut PgStats,
    mut dbname: *mut ::core::ffi::c_char,
) {
    let mut avg = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    calc_average(&raw mut avg, stat, old);
    pktbuf_write_DataRow(
        buf,
        b"sNNNNNNNNNNNNNNNNNNNNNN\0" as *const u8 as *const ::core::ffi::c_char,
        dbname,
        (*stat).server_assignment_count,
        (*stat).xact_count,
        (*stat).query_count,
        (*stat).client_bytes,
        (*stat).server_bytes,
        (*stat).xact_time,
        (*stat).query_time,
        (*stat).wait_time,
        (*stat).ps_client_parse_count,
        (*stat).ps_server_parse_count,
        (*stat).ps_bind_count,
        avg.server_assignment_count,
        avg.xact_count,
        avg.query_count,
        avg.client_bytes,
        avg.server_bytes,
        avg.xact_time,
        avg.query_time,
        avg.wait_time,
        avg.ps_client_parse_count,
        avg.ps_server_parse_count,
        avg.ps_bind_count,
    );
}
#[no_mangle]

pub unsafe extern "C" fn admin_database_stats(
    mut client: *mut PgSocket,
    mut pool_list_0: *mut StatList,
) -> bool {
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut item = ::core::ptr::null_mut::<List>();
    let mut cur_db = ::core::ptr::null_mut::<PgDatabase>();
    let mut st_db = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    let mut old_db = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    reset_stats(&raw mut st_db);
    reset_stats(&raw mut old_db);
    buf = pktbuf_dynamic(512 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(
            client,
            b"no mem\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return true_0 != 0;
    }
    pktbuf_write_RowDescription(
        buf,
        b"sNNNNNNNNNNNNNNNNNNNNNN\0" as *const u8 as *const ::core::ffi::c_char,
        b"database\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_server_assignment_count\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_xact_count\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_query_count\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_received\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_sent\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_xact_time\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_query_time\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_wait_time\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_client_parse_count\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_server_parse_count\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_bind_count\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_server_assignment_count\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_xact_count\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_query_count\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_recv\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_sent\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_xact_time\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_query_time\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_wait_time\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_client_parse_count\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_server_parse_count\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_bind_count\0" as *const u8 as *const ::core::ffi::c_char,
    );
    item = (*pool_list_0).head.next;
    while item != &raw mut (*pool_list_0).head {
        pool = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgPool;
        if cur_db.is_null() {
            cur_db = (*pool).db;
        }
        if (*pool).db != cur_db {
            write_stats(
                buf,
                &raw mut st_db,
                &raw mut old_db,
                &raw mut (*cur_db).name as *mut ::core::ffi::c_char,
            );
            cur_db = (*pool).db;
            reset_stats(&raw mut st_db);
            reset_stats(&raw mut old_db);
        }
        stat_add(&raw mut st_db, &raw mut (*pool).stats);
        stat_add(&raw mut old_db, &raw mut (*pool).older_stats);
        item = (*item).next;
    }
    if !cur_db.is_null() {
        write_stats(
            buf,
            &raw mut st_db,
            &raw mut old_db,
            &raw mut (*cur_db).name as *mut ::core::ffi::c_char,
        );
    }
    admin_flush(
        client,
        buf,
        b"SHOW\0" as *const u8 as *const ::core::ffi::c_char,
    );
    true_0 != 0
}

unsafe extern "C" fn write_stats_totals(
    mut buf: *mut PktBuf,
    mut stat: *mut PgStats,
    mut _old: *mut PgStats,
    mut dbname: *mut ::core::ffi::c_char,
) {
    pktbuf_write_DataRow(
        buf,
        b"sNNNNNNNNNNN\0" as *const u8 as *const ::core::ffi::c_char,
        dbname,
        (*stat).server_assignment_count,
        (*stat).xact_count,
        (*stat).query_count,
        (*stat).client_bytes,
        (*stat).server_bytes,
        (*stat).xact_time,
        (*stat).query_time,
        (*stat).wait_time,
        (*stat).ps_client_parse_count,
        (*stat).ps_server_parse_count,
        (*stat).ps_bind_count,
    );
}
#[no_mangle]

pub unsafe extern "C" fn admin_database_stats_totals(
    mut client: *mut PgSocket,
    mut pool_list_0: *mut StatList,
) -> bool {
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut item = ::core::ptr::null_mut::<List>();
    let mut cur_db = ::core::ptr::null_mut::<PgDatabase>();
    let mut st_db = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    let mut old_db = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    reset_stats(&raw mut st_db);
    reset_stats(&raw mut old_db);
    buf = pktbuf_dynamic(512 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(
            client,
            b"no mem\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return true_0 != 0;
    }
    pktbuf_write_RowDescription(
        buf,
        b"sNNNNNNNNNNN\0" as *const u8 as *const ::core::ffi::c_char,
        b"database\0" as *const u8 as *const ::core::ffi::c_char,
        b"server_assignment_count\0" as *const u8 as *const ::core::ffi::c_char,
        b"xact_count\0" as *const u8 as *const ::core::ffi::c_char,
        b"query_count\0" as *const u8 as *const ::core::ffi::c_char,
        b"bytes_received\0" as *const u8 as *const ::core::ffi::c_char,
        b"bytes_sent\0" as *const u8 as *const ::core::ffi::c_char,
        b"xact_time\0" as *const u8 as *const ::core::ffi::c_char,
        b"query_time\0" as *const u8 as *const ::core::ffi::c_char,
        b"wait_time\0" as *const u8 as *const ::core::ffi::c_char,
        b"client_parse_count\0" as *const u8 as *const ::core::ffi::c_char,
        b"server_parse_count\0" as *const u8 as *const ::core::ffi::c_char,
        b"bind_count\0" as *const u8 as *const ::core::ffi::c_char,
    );
    item = (*pool_list_0).head.next;
    while item != &raw mut (*pool_list_0).head {
        pool = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgPool;
        if cur_db.is_null() {
            cur_db = (*pool).db;
        }
        if (*pool).db != cur_db {
            write_stats_totals(
                buf,
                &raw mut st_db,
                &raw mut old_db,
                &raw mut (*cur_db).name as *mut ::core::ffi::c_char,
            );
            cur_db = (*pool).db;
            reset_stats(&raw mut st_db);
            reset_stats(&raw mut old_db);
        }
        stat_add(&raw mut st_db, &raw mut (*pool).stats);
        stat_add(&raw mut old_db, &raw mut (*pool).older_stats);
        item = (*item).next;
    }
    if !cur_db.is_null() {
        write_stats_totals(
            buf,
            &raw mut st_db,
            &raw mut old_db,
            &raw mut (*cur_db).name as *mut ::core::ffi::c_char,
        );
    }
    admin_flush(
        client,
        buf,
        b"SHOW\0" as *const u8 as *const ::core::ffi::c_char,
    );
    true_0 != 0
}

unsafe extern "C" fn write_stats_averages(
    mut buf: *mut PktBuf,
    mut stat: *mut PgStats,
    mut old: *mut PgStats,
    mut dbname: *mut ::core::ffi::c_char,
) {
    let mut avg = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    calc_average(&raw mut avg, stat, old);
    pktbuf_write_DataRow(
        buf,
        b"sNNNNNNNNNNN\0" as *const u8 as *const ::core::ffi::c_char,
        dbname,
        avg.server_assignment_count,
        avg.xact_count,
        avg.query_count,
        avg.client_bytes,
        avg.server_bytes,
        avg.xact_time,
        avg.query_time,
        avg.wait_time,
        avg.ps_client_parse_count,
        avg.ps_server_parse_count,
        avg.ps_bind_count,
    );
}
#[no_mangle]

pub unsafe extern "C" fn admin_database_stats_averages(
    mut client: *mut PgSocket,
    mut pool_list_0: *mut StatList,
) -> bool {
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut item = ::core::ptr::null_mut::<List>();
    let mut cur_db = ::core::ptr::null_mut::<PgDatabase>();
    let mut st_db = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    let mut old_db = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    reset_stats(&raw mut st_db);
    reset_stats(&raw mut old_db);
    buf = pktbuf_dynamic(512 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(
            client,
            b"no mem\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return true_0 != 0;
    }
    pktbuf_write_RowDescription(
        buf,
        b"sNNNNNNNNNNN\0" as *const u8 as *const ::core::ffi::c_char,
        b"database\0" as *const u8 as *const ::core::ffi::c_char,
        b"server_assignment_count\0" as *const u8 as *const ::core::ffi::c_char,
        b"xact_count\0" as *const u8 as *const ::core::ffi::c_char,
        b"query_count\0" as *const u8 as *const ::core::ffi::c_char,
        b"bytes_received\0" as *const u8 as *const ::core::ffi::c_char,
        b"bytes_sent\0" as *const u8 as *const ::core::ffi::c_char,
        b"xact_time\0" as *const u8 as *const ::core::ffi::c_char,
        b"query_time\0" as *const u8 as *const ::core::ffi::c_char,
        b"wait_time\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_client_parse_count\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_server_parse_count\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_bind_count\0" as *const u8 as *const ::core::ffi::c_char,
    );
    item = (*pool_list_0).head.next;
    while item != &raw mut (*pool_list_0).head {
        pool = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgPool;
        if cur_db.is_null() {
            cur_db = (*pool).db;
        }
        if (*pool).db != cur_db {
            write_stats_averages(
                buf,
                &raw mut st_db,
                &raw mut old_db,
                &raw mut (*cur_db).name as *mut ::core::ffi::c_char,
            );
            cur_db = (*pool).db;
            reset_stats(&raw mut st_db);
            reset_stats(&raw mut old_db);
        }
        stat_add(&raw mut st_db, &raw mut (*pool).stats);
        stat_add(&raw mut old_db, &raw mut (*pool).older_stats);
        item = (*item).next;
    }
    if !cur_db.is_null() {
        write_stats_averages(
            buf,
            &raw mut st_db,
            &raw mut old_db,
            &raw mut (*cur_db).name as *mut ::core::ffi::c_char,
        );
    }
    admin_flush(
        client,
        buf,
        b"SHOW\0" as *const u8 as *const ::core::ffi::c_char,
    );
    true_0 != 0
}
#[no_mangle]

pub unsafe extern "C" fn show_stat_totals(
    mut client: *mut PgSocket,
    mut pool_list_0: *mut StatList,
) -> bool {
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut item = ::core::ptr::null_mut::<List>();
    let mut st_total = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    let mut old_total = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    let mut avg = PgStats {
        server_assignment_count: 0,
        xact_count: 0,
        query_count: 0,
        server_bytes: 0,
        client_bytes: 0,
        xact_time: 0,
        query_time: 0,
        wait_time: 0,
        ps_server_parse_count: 0,
        ps_client_parse_count: 0,
        ps_bind_count: 0,
    };
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    reset_stats(&raw mut st_total);
    reset_stats(&raw mut old_total);
    buf = pktbuf_dynamic(512 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(
            client,
            b"no mem\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return true_0 != 0;
    }
    item = (*pool_list_0).head.next;
    while item != &raw mut (*pool_list_0).head {
        pool = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgPool;
        stat_add(&raw mut st_total, &raw mut (*pool).stats);
        stat_add(&raw mut old_total, &raw mut (*pool).older_stats);
        item = (*item).next;
    }
    calc_average(&raw mut avg, &raw mut st_total, &raw mut old_total);
    pktbuf_write_RowDescription(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"name\0" as *const u8 as *const ::core::ffi::c_char,
        b"value\0" as *const u8 as *const ::core::ffi::c_char,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_server_assignment_count\0" as *const u8 as *const ::core::ffi::c_char,
        st_total.server_assignment_count,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_xact_count\0" as *const u8 as *const ::core::ffi::c_char,
        st_total.xact_count,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_query_count\0" as *const u8 as *const ::core::ffi::c_char,
        st_total.query_count,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_client_bytes\0" as *const u8 as *const ::core::ffi::c_char,
        st_total.client_bytes,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_server_bytes\0" as *const u8 as *const ::core::ffi::c_char,
        st_total.server_bytes,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_xact_time\0" as *const u8 as *const ::core::ffi::c_char,
        st_total.xact_time,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_query_time\0" as *const u8 as *const ::core::ffi::c_char,
        st_total.query_time,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_wait_time\0" as *const u8 as *const ::core::ffi::c_char,
        st_total.wait_time,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_ps_client_parse_count\0" as *const u8 as *const ::core::ffi::c_char,
        st_total.ps_client_parse_count,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_ps_server_parse_count\0" as *const u8 as *const ::core::ffi::c_char,
        st_total.ps_server_parse_count,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"total_ps_bind_count\0" as *const u8 as *const ::core::ffi::c_char,
        st_total.ps_bind_count,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_server_assignment_count\0" as *const u8 as *const ::core::ffi::c_char,
        avg.server_assignment_count,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_xact_count\0" as *const u8 as *const ::core::ffi::c_char,
        avg.xact_count,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_query_count\0" as *const u8 as *const ::core::ffi::c_char,
        avg.query_count,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_client_bytes\0" as *const u8 as *const ::core::ffi::c_char,
        avg.client_bytes,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_server_bytes\0" as *const u8 as *const ::core::ffi::c_char,
        avg.server_bytes,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_xact_time\0" as *const u8 as *const ::core::ffi::c_char,
        avg.xact_time,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_query_time\0" as *const u8 as *const ::core::ffi::c_char,
        avg.query_time,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_wait_time\0" as *const u8 as *const ::core::ffi::c_char,
        avg.wait_time,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_ps_client_parse_count\0" as *const u8 as *const ::core::ffi::c_char,
        avg.ps_client_parse_count,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_ps_server_parse_count\0" as *const u8 as *const ::core::ffi::c_char,
        avg.ps_server_parse_count,
    );
    pktbuf_write_DataRow(
        buf,
        b"sN\0" as *const u8 as *const ::core::ffi::c_char,
        b"avg_ps_bind_count\0" as *const u8 as *const ::core::ffi::c_char,
        avg.ps_bind_count,
    );
    admin_flush(
        client,
        buf,
        b"SHOW\0" as *const u8 as *const ::core::ffi::c_char,
    );
    true_0 != 0
}

unsafe extern "C" fn refresh_stats(
    _s: ::core::ffi::c_int,
    _flags: ::core::ffi::c_short,
    _arg: *mut ::core::ffi::c_void,
) {
    let mut item: *mut List;
    let mut pool: *mut PgPool;
    let mut old_total: PgStats = ::core::mem::zeroed();
    let mut cur_total: PgStats = ::core::mem::zeroed();
    let mut avg: PgStats = ::core::mem::zeroed();

    reset_stats(&raw mut old_total);
    reset_stats(&raw mut cur_total);
    old_stamp = new_stamp;
    new_stamp = get_cached_time();

    item = pool_list.head.next;
    while item != &raw mut pool_list.head {
        pool = item as *mut PgPool;
        (*pool).older_stats = (*pool).newer_stats;
        (*pool).newer_stats = (*pool).stats;
        if cf_log_stats != 0 {
            stat_add(&raw mut cur_total, &raw mut (*pool).stats);
            stat_add(&raw mut old_total, &raw mut (*pool).older_stats);
        }
        item = (*item).next;
    }

    calc_average(&raw mut avg, &raw mut cur_total, &raw mut old_total);

    if cf_log_stats != 0 {
        log_generic(
            LG_INFO,
            NULL,
            c"stats: %llu xacts/s, %llu queries/s, %llu client parses/s, %llu server parses/s, %llu binds/s, in %llu B/s, out %llu B/s, xact %llu us, query %llu us, wait %llu us".as_ptr(),
            avg.xact_count,
            avg.query_count,
            avg.ps_client_parse_count,
            avg.ps_server_parse_count,
            avg.ps_bind_count,
            avg.client_bytes,
            avg.server_bytes,
            avg.xact_time,
            avg.query_time,
            avg.wait_time,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn stats_setup() {
    let mut period = timeval {
        tv_sec: cf_stats_period as __darwin_time_t,
        tv_usec: 0,
    };
    new_stamp = get_cached_time();
    old_stamp = new_stamp - USEC;

    event_assign(
        &raw mut ev_stats,
        pgb_event_base,
        -1,
        EV_PERSIST as ::core::ffi::c_short,
        Some(refresh_stats),
        NULL,
    );

    if event_add(&raw mut ev_stats, &raw mut period) < 0 {
        log_generic(
            LG_WARNING,
            NULL,
            c"event_add failed: %s".as_ptr(),
            strerror(*__error()),
        );
    }
}


extern "C" {
    pub fn get_cached_time() -> usec_t;
}