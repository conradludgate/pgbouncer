
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

pub mod time_h {
    
    pub type usec_t = uint64_t;
    use super::_uint64_t_h::uint64_t;
}

pub mod list_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct List {
        pub next: *mut List,
        pub prev: *mut List,
    }
}

pub mod statlist_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct StatList {
        pub head: List,
        pub cur_count: ::core::ffi::c_int,
    }
    use super::list_h::List;
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
    extern "C" {
        
        pub type event_base;
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
    use super::_size_t_h::size_t;
    use super::_uint8_t_h::uint8_t;
    extern "C" {
        
        pub type pg_cryptohash_ctx;
        
        pub fn pg_cryptohash_create(type_0: pg_cryptohash_type) -> *mut pg_cryptohash_ctx;
        
        pub fn pg_cryptohash_init(ctx: *mut pg_cryptohash_ctx) -> ::core::ffi::c_int;
        
        pub fn pg_cryptohash_update(
            ctx: *mut pg_cryptohash_ctx,
            data: *const uint8_t,
            len: size_t,
        ) -> ::core::ffi::c_int;
        
        pub fn pg_cryptohash_final(
            ctx: *mut pg_cryptohash_ctx,
            dest: *mut uint8_t,
            len: size_t,
        ) -> ::core::ffi::c_int;
        
        pub fn pg_cryptohash_free(ctx: *mut pg_cryptohash_ctx);
        
        pub fn pg_cryptohash_error(ctx: *mut pg_cryptohash_ctx) -> *const ::core::ffi::c_char;
    }
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    
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
    use super::mbuf_h::MBuf;
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
        
        pub fn log_generic(
            level: LogLevel,
            ctx: *mut ::core::ffi::c_void,
            s: *const ::core::ffi::c_char,
            ...
        );
    }
}

pub mod scram_h {
    
    pub type PasswordType = ::core::ffi::c_uint;
    
    pub const PASSWORD_TYPE_SCRAM_SHA_256: PasswordType = 2;
    
    pub const PASSWORD_TYPE_MD5: PasswordType = 1;
    
    pub const PASSWORD_TYPE_PLAINTEXT: PasswordType = 0;
    extern "C" {
        
        pub static mut cf_scram_iterations: ::core::ffi::c_int;
    }
}

pub mod hmac_h {
    use super::_size_t_h::size_t;
    use super::_uint8_t_h::uint8_t;
    use super::cryptohash_h::pg_cryptohash_type;
    extern "C" {
        
        pub type pg_hmac_ctx;
        
        pub fn pg_hmac_create(type_0: pg_cryptohash_type) -> *mut pg_hmac_ctx;
        
        pub fn pg_hmac_init(
            ctx: *mut pg_hmac_ctx,
            key: *const uint8_t,
            len: size_t,
        ) -> ::core::ffi::c_int;
        
        pub fn pg_hmac_update(
            ctx: *mut pg_hmac_ctx,
            data: *const uint8_t,
            len: size_t,
        ) -> ::core::ffi::c_int;
        
        pub fn pg_hmac_final(
            ctx: *mut pg_hmac_ctx,
            dest: *mut uint8_t,
            len: size_t,
        ) -> ::core::ffi::c_int;
        
        pub fn pg_hmac_free(ctx: *mut pg_hmac_ctx);
        
        pub fn pg_hmac_error(ctx: *mut pg_hmac_ctx) -> *const ::core::ffi::c_char;
    }
}

pub mod saslprep_h {
    
    pub const SASLPREP_SUCCESS: pg_saslprep_rc = 0;
    
    pub type pg_saslprep_rc = ::core::ffi::c_int;
    
    pub const SASLPREP_PROHIBITED: pg_saslprep_rc = -3;
    
    pub const SASLPREP_INVALID_UTF8: pg_saslprep_rc = -2;
    
    pub const SASLPREP_OOM: pg_saslprep_rc = -1;
    extern "C" {
        
        pub fn pg_saslprep(
            input: *const ::core::ffi::c_char,
            output: *mut *mut ::core::ffi::c_char,
        ) -> pg_saslprep_rc;
    }
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

pub mod _malloc_h {
    use super::_size_t_h::size_t;
    extern "C" {
        
        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
        
        pub fn free(_: *mut ::core::ffi::c_void);
    }
}

pub mod _null_h {
    
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
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
        
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
        
        pub fn strncmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
            __n: size_t,
        ) -> ::core::ffi::c_int;
        
        pub fn strspn(
            __s: *const ::core::ffi::c_char,
            __charset: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_ulong;
        
        pub fn strtok(
            __str: *mut ::core::ffi::c_char,
            __sep: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        
        pub fn strdup(__s1: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
        
        pub fn strlcat(
            __dst: *mut ::core::ffi::c_char,
            __source: *const ::core::ffi::c_char,
            __size: size_t,
        ) -> ::core::ffi::c_ulong;
    }
}

pub mod util_h {
    
    pub const MD5_PASSWD_LEN: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
    use super::_uint8_t_h::uint8_t;
    extern "C" {
        
        pub fn get_random_bytes(dest: *mut uint8_t, len: ::core::ffi::c_int);
    }
}

pub mod scram_common_h {
    
    pub const SCRAM_SHA_256_KEY_LEN: ::core::ffi::c_int = PG_SHA256_DIGEST_LENGTH;
    
    pub const SCRAM_RAW_NONCE_LEN: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
    
    pub const SCRAM_DEFAULT_SALT_LEN: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    use super::_uint8_t_h::uint8_t;
    use super::cryptohash_h::pg_cryptohash_type;
    use super::sha2_h::PG_SHA256_DIGEST_LENGTH;
    extern "C" {
        
        pub fn scram_SaltedPassword(
            password: *const ::core::ffi::c_char,
            hash_type: pg_cryptohash_type,
            key_length: ::core::ffi::c_int,
            salt: *const uint8_t,
            saltlen: ::core::ffi::c_int,
            iterations: ::core::ffi::c_int,
            result: *mut uint8_t,
            errstr: *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        
        pub fn scram_H(
            input: *const uint8_t,
            hash_type: pg_cryptohash_type,
            key_length: ::core::ffi::c_int,
            result: *mut uint8_t,
            errstr: *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        
        pub fn scram_ClientKey(
            salted_password: *const uint8_t,
            hash_type: pg_cryptohash_type,
            key_length: ::core::ffi::c_int,
            result: *mut uint8_t,
            errstr: *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        
        pub fn scram_ServerKey(
            salted_password: *const uint8_t,
            hash_type: pg_cryptohash_type,
            key_length: ::core::ffi::c_int,
            result: *mut uint8_t,
            errstr: *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}

pub mod base64_h {
    use super::_uint8_t_h::uint8_t;
    extern "C" {
        
        pub fn pg_b64_encode(
            src: *const uint8_t,
            len: ::core::ffi::c_int,
            dst: *mut ::core::ffi::c_char,
            dstlen: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        
        pub fn pg_b64_decode(
            src: *const ::core::ffi::c_char,
            len: ::core::ffi::c_int,
            dst: *mut uint8_t,
            dstlen: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        
        pub fn pg_b64_enc_len(srclen: ::core::ffi::c_int) -> ::core::ffi::c_int;
        
        pub fn pg_b64_dec_len(srclen: ::core::ffi::c_int) -> ::core::ffi::c_int;
    }
}

pub mod limits_h {
    
    pub const INT_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
}

pub mod stdbool_h {
    
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}

pub mod _stdlib_h {
    extern "C" {
        
        pub fn strtol(
            __str: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
            __base: ::core::ffi::c_int,
        ) -> ::core::ffi::c_long;
    }
}

pub mod errno_h {
    extern "C" {
        
        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}

pub mod sha2_h {
    
    pub const PG_SHA256_DIGEST_LENGTH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
}
pub use self::_in_addr_t_h::in_addr_t;
pub use self::_in_port_t_h::in_port_t;
use self::_malloc_h::{free, malloc};
pub use self::_null_h::NULL;
pub use self::_pid_t_h::pid_t;
pub use self::_ptrdiff_t_h::ptrdiff_t;
pub use self::_sa_family_t_h::sa_family_t;
pub use self::_size_t_h::size_t;
pub use self::_ssize_t_h::ssize_t;
use self::_stdio_h::snprintf;
use self::_stdlib_h::strtol;
use self::_string_h::{
    memcmp, memcpy, memset, strcmp, strdup, strlcat, strlen, strncmp, strspn, strtok,
};
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
use self::base64_h::{pg_b64_dec_len, pg_b64_decode, pg_b64_enc_len, pg_b64_encode};
pub use self::bouncer_h::{
    sockaddr_ucreds, C2RustUnnamed_9, CallbackState, LoadBalanceHosts, PacketCallbackFlag, PgAddr,
    PgCredentials, PgDatabase, PgGlobalUser, PgPool, PgSocket, PgStats, ReplicationType,
    ScramState, SocketState, CB_HANDLE_COMPLETE_PACKET, CB_NONE, CB_WANT_COMPLETE_PACKET,
    CL_ACTIVE, CL_ACTIVE_CANCEL, CL_FREE, CL_JUSTFREE, CL_LOGIN, CL_WAITING, CL_WAITING_CANCEL,
    CL_WAITING_LOGIN, LOAD_BALANCE_HOSTS_DISABLE, LOAD_BALANCE_HOSTS_ROUND_ROBIN,
    REPLICATION_LOGICAL, REPLICATION_NONE, REPLICATION_PHYSICAL, SV_ACTIVE, SV_ACTIVE_CANCEL,
    SV_BEING_CANCELED, SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN, SV_TESTED, SV_USED,
};
pub use self::cryptohash_h::{
    pg_cryptohash_create, pg_cryptohash_ctx, pg_cryptohash_error, pg_cryptohash_final,
    pg_cryptohash_free, pg_cryptohash_init, pg_cryptohash_type, pg_cryptohash_update, PG_SHA224,
    PG_SHA256, PG_SHA384, PG_SHA512,
};

use self::errno_h::__error;

pub use self::event_struct_h::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
use self::hmac_h::{
    pg_hmac_create, pg_hmac_ctx, pg_hmac_error, pg_hmac_final, pg_hmac_free, pg_hmac_init,
    pg_hmac_update,
};
pub use self::in6_h::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use self::in_h::{in_addr, sockaddr_in};
pub use self::iobuf_h::{iobuf, IOBuf};
pub use self::limits_h::INT_MAX;
pub use self::list_h::List;
pub use self::logging_h::{
    log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE, LG_STATS, LG_WARNING,
};
pub use self::mbuf_h::MBuf;
pub use self::pktbuf_h::PktBuf;
pub use self::prepare_h::{
    PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement,
};
pub use self::proto_h::PktHdr;
pub use self::saslprep_h::{
    pg_saslprep, pg_saslprep_rc, SASLPREP_INVALID_UTF8, SASLPREP_OOM, SASLPREP_PROHIBITED,
    SASLPREP_SUCCESS,
};
pub use self::sbuf_h::{
    sbuf_cb_t, SBuf, SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK, SBUF_EV_FLUSH,
    SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY,
};
pub use self::scram_common_h::{
    scram_ClientKey, scram_H, scram_SaltedPassword, scram_ServerKey, SCRAM_DEFAULT_SALT_LEN,
    SCRAM_RAW_NONCE_LEN, SCRAM_SHA_256_KEY_LEN,
};
pub use self::scram_h::{
    cf_scram_iterations, PasswordType, PASSWORD_TYPE_MD5, PASSWORD_TYPE_PLAINTEXT,
    PASSWORD_TYPE_SCRAM_SHA_256,
};
pub use self::sha2_h::PG_SHA256_DIGEST_LENGTH;
pub use self::socket_h::sockaddr;
pub use self::statlist_h::StatList;
pub use self::stdbool_h::{false_0, true_0};
pub use self::strpool_h::{PStr, StrPool};
pub use self::sys__types_h::{__darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t, __DARWIN_NULL};
pub use self::time_h::usec_t;

pub use self::uthash_h::{UT_hash_bucket, UT_hash_handle, UT_hash_table};
pub use self::util_h::{get_random_bytes, MD5_PASSWD_LEN};
pub use self::varcache_h::VarCache;
#[no_mangle]

pub unsafe extern "C" fn free_scram_state(mut state: *mut ScramState) {
    free((*state).client_nonce as *mut ::core::ffi::c_void);
    free((*state).client_first_message_bare as *mut ::core::ffi::c_void);
    free((*state).client_final_message_without_proof as *mut ::core::ffi::c_void);
    free((*state).server_nonce as *mut ::core::ffi::c_void);
    free((*state).server_first_message as *mut ::core::ffi::c_void);
    free((*state).SaltedPassword as *mut ::core::ffi::c_void);
    free((*state).salt as *mut ::core::ffi::c_void);
    free((*state).encoded_salt as *mut ::core::ffi::c_void);
    memset(
        state as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<ScramState>() as size_t,
    );
}

unsafe extern "C" fn is_scram_printable(mut p: *mut ::core::ffi::c_char) -> bool {
    while *p != 0 {
        if (*p as ::core::ffi::c_int) < 0x21 as ::core::ffi::c_int
            || *p as ::core::ffi::c_int > 0x7e as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == 0x2c as ::core::ffi::c_int
        {
            return false_0 != 0;
        }
        p = p.offset(1);
    }
    true_0 != 0
}

unsafe extern "C" fn sanitize_char(mut c: ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    static mut buf: [::core::ffi::c_char; 5] = [0; 5];
    if c as ::core::ffi::c_int >= 0x21 as ::core::ffi::c_int
        && c as ::core::ffi::c_int <= 0x7e as ::core::ffi::c_int
    {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t,
            b"'%c'\0" as *const u8 as *const ::core::ffi::c_char,
            c as ::core::ffi::c_int,
        );
    } else {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t,
            b"0x%02x\0" as *const u8 as *const ::core::ffi::c_char,
            c as ::core::ffi::c_uchar as ::core::ffi::c_int,
        );
    }
    &raw mut buf as *mut ::core::ffi::c_char
}

unsafe extern "C" fn read_attr_value(
    mut sk: *mut PgSocket,
    mut input: *mut *mut ::core::ffi::c_char,
    mut attr: ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut begin = *input;
    let mut end = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if *begin as ::core::ffi::c_int != attr as ::core::ffi::c_int {
        log_generic(
            LG_ERROR,
            sk as *mut ::core::ffi::c_void,
            b"malformed SCRAM message (attribute \"%c\" expected)\0" as *const u8
                as *const ::core::ffi::c_char,
            attr as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    begin = begin.offset(1);
    if *begin as ::core::ffi::c_int != '=' as i32 {
        log_generic(
            LG_ERROR,
            sk as *mut ::core::ffi::c_void,
            b"malformed SCRAM message (expected \"=\" after attribute \"%c\")\0" as *const u8
                as *const ::core::ffi::c_char,
            attr as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    begin = begin.offset(1);
    end = begin;
    while *end as ::core::ffi::c_int != 0 && *end as ::core::ffi::c_int != ',' as i32 {
        end = end.offset(1);
    }
    if *end != 0 {
        *end = '\0' as i32 as ::core::ffi::c_char;
        *input = end.offset(1 as ::core::ffi::c_int as isize);
    } else {
        *input = end;
    }
    begin
}

unsafe extern "C" fn read_any_attr(
    mut sk: *mut PgSocket,
    mut input: *mut *mut ::core::ffi::c_char,
    mut attr_p: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut begin = *input;
    let mut end = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut attr = *begin;
    if !(attr as ::core::ffi::c_int >= 'A' as i32 && attr as ::core::ffi::c_int <= 'Z' as i32
        || attr as ::core::ffi::c_int >= 'a' as i32 && attr as ::core::ffi::c_int <= 'z' as i32)
    {
        log_generic(
            LG_ERROR,
            sk as *mut ::core::ffi::c_void,
            b"malformed SCRAM message (attribute expected, but found invalid character \"%s\")\0"
                as *const u8 as *const ::core::ffi::c_char,
            sanitize_char(attr),
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !attr_p.is_null() {
        *attr_p = attr;
    }
    begin = begin.offset(1);
    if *begin as ::core::ffi::c_int != '=' as i32 {
        log_generic(
            LG_ERROR,
            sk as *mut ::core::ffi::c_void,
            b"malformed SCRAM message (expected character \"=\" after attribute \"%c\")\0"
                as *const u8 as *const ::core::ffi::c_char,
            attr as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    begin = begin.offset(1);
    end = begin;
    while *end as ::core::ffi::c_int != 0 && *end as ::core::ffi::c_int != ',' as i32 {
        end = end.offset(1);
    }
    if *end != 0 {
        *end = '\0' as i32 as ::core::ffi::c_char;
        *input = end.offset(1 as ::core::ffi::c_int as isize);
    } else {
        *input = end;
    }
    begin
}

unsafe extern "C" fn parse_scram_secret(
    mut secret: *const ::core::ffi::c_char,
    mut iterations: *mut ::core::ffi::c_int,
    mut salt: *mut *mut ::core::ffi::c_char,
    mut stored_key: *mut uint8_t,
    mut server_key: *mut uint8_t,
) -> bool {
    let mut s = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut scheme_str = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut salt_str = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut iterations_str = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut storedkey_str = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut serverkey_str = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut decoded_len: ::core::ffi::c_int = 0;
    let mut decoded_salt_buf = ::core::ptr::null_mut::<uint8_t>();
    let mut decoded_stored_buf = ::core::ptr::null_mut::<uint8_t>();
    let mut decoded_server_buf = ::core::ptr::null_mut::<uint8_t>();
    s = strdup(secret);
    if !s.is_null() {
        scheme_str = strtok(s, b"$\0" as *const u8 as *const ::core::ffi::c_char);
        if !scheme_str.is_null() {
            iterations_str = strtok(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                b":\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !iterations_str.is_null() {
                salt_str = strtok(
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    b"$\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if !salt_str.is_null() {
                    storedkey_str = strtok(
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        b":\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if !storedkey_str.is_null() {
                        serverkey_str = strtok(
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            b"\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        if !serverkey_str.is_null()
                            && (strcmp(
                                scheme_str,
                                b"SCRAM-SHA-256\0" as *const u8 as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int)
                        {
                            *__error() = 0 as ::core::ffi::c_int;
                            *iterations =
                                strtol(iterations_str, &raw mut p, 10 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int;
                            if !(*p as ::core::ffi::c_int != 0
                                || *__error() != 0 as ::core::ffi::c_int)
                            {
                                decoded_len =
                                    pg_b64_dec_len(strlen(salt_str) as ::core::ffi::c_int);
                                decoded_salt_buf = malloc(decoded_len as size_t) as *mut uint8_t;
                                if !decoded_salt_buf.is_null() {
                                    decoded_len = pg_b64_decode(
                                        salt_str,
                                        strlen(salt_str) as ::core::ffi::c_int,
                                        decoded_salt_buf,
                                        decoded_len,
                                    );
                                    free(decoded_salt_buf as *mut ::core::ffi::c_void);
                                    if decoded_len >= 0 as ::core::ffi::c_int {
                                        *salt = strdup(salt_str);
                                        if !(*salt).is_null() {
                                            decoded_len = pg_b64_dec_len(
                                                strlen(storedkey_str) as ::core::ffi::c_int
                                            );
                                            decoded_stored_buf =
                                                malloc(decoded_len as size_t) as *mut uint8_t;
                                            if !decoded_stored_buf.is_null() {
                                                decoded_len = pg_b64_decode(
                                                    storedkey_str,
                                                    strlen(storedkey_str) as ::core::ffi::c_int,
                                                    decoded_stored_buf,
                                                    decoded_len,
                                                );
                                                if decoded_len == SCRAM_SHA_256_KEY_LEN {
                                                    memcpy(
                                                        stored_key as *mut ::core::ffi::c_void,
                                                        decoded_stored_buf
                                                            as *const ::core::ffi::c_void,
                                                        SCRAM_SHA_256_KEY_LEN as size_t,
                                                    );
                                                    decoded_len =
                                                        pg_b64_dec_len(strlen(serverkey_str)
                                                            as ::core::ffi::c_int);
                                                    decoded_server_buf =
                                                        malloc(decoded_len as size_t)
                                                            as *mut uint8_t;
                                                    if !decoded_server_buf.is_null() {
                                                        decoded_len = pg_b64_decode(
                                                            serverkey_str,
                                                            strlen(serverkey_str)
                                                                as ::core::ffi::c_int,
                                                            decoded_server_buf,
                                                            decoded_len,
                                                        );
                                                        if decoded_len == SCRAM_SHA_256_KEY_LEN {
                                                            memcpy(
                                                                server_key
                                                                    as *mut ::core::ffi::c_void,
                                                                decoded_server_buf
                                                                    as *const ::core::ffi::c_void,
                                                                SCRAM_SHA_256_KEY_LEN as size_t,
                                                            );
                                                            free(
                                                                decoded_stored_buf
                                                                    as *mut ::core::ffi::c_void,
                                                            );
                                                            free(
                                                                decoded_server_buf
                                                                    as *mut ::core::ffi::c_void,
                                                            );
                                                            free(s as *mut ::core::ffi::c_void);
                                                            return true_0 != 0;
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
            }
        }
    }
    free(decoded_stored_buf as *mut ::core::ffi::c_void);
    free(decoded_server_buf as *mut ::core::ffi::c_void);
    free(s as *mut ::core::ffi::c_void);
    free(*salt as *mut ::core::ffi::c_void);
    *salt = ::core::ptr::null_mut::<::core::ffi::c_char>();
    false_0 != 0
}

pub const MD5_PASSWD_CHARSET: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"0123456789abcdef\0")
};
#[no_mangle]

pub unsafe extern "C" fn get_password_type(
    mut shadow_pass: *const ::core::ffi::c_char,
) -> PasswordType {
    let mut encoded_salt = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut iterations: ::core::ffi::c_int = 0;
    let mut stored_key: [uint8_t; 32] = [0; 32];
    let mut server_key: [uint8_t; 32] = [0; 32];
    if strncmp(
        shadow_pass,
        b"md5\0" as *const u8 as *const ::core::ffi::c_char,
        3 as size_t,
    ) == 0 as ::core::ffi::c_int
        && strlen(shadow_pass) == MD5_PASSWD_LEN as size_t
        && strspn(
            shadow_pass.offset(3 as ::core::ffi::c_int as isize),
            MD5_PASSWD_CHARSET.as_ptr(),
        ) == (MD5_PASSWD_LEN - 3 as ::core::ffi::c_int) as ::core::ffi::c_ulong
    {
        return PASSWORD_TYPE_MD5;
    }
    if parse_scram_secret(
        shadow_pass,
        &raw mut iterations,
        &raw mut encoded_salt,
        &raw mut stored_key as *mut uint8_t,
        &raw mut server_key as *mut uint8_t,
    ) {
        free(encoded_salt as *mut ::core::ffi::c_void);
        return PASSWORD_TYPE_SCRAM_SHA_256;
    }
    free(encoded_salt as *mut ::core::ffi::c_void);
    PASSWORD_TYPE_PLAINTEXT
}
#[no_mangle]

pub unsafe extern "C" fn build_client_first_message(
    mut state: *mut ScramState,
) -> *mut ::core::ffi::c_char {
    let mut raw_nonce: [uint8_t; 19] = [0; 19];
    let mut encoded_len: ::core::ffi::c_int = 0;
    let mut len: size_t = 0;
    let mut result = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*state).hash_type = PG_SHA256;
    (*state).key_length = SCRAM_SHA_256_KEY_LEN;
    get_random_bytes(&raw mut raw_nonce as *mut uint8_t, SCRAM_RAW_NONCE_LEN);
    encoded_len = pg_b64_enc_len(SCRAM_RAW_NONCE_LEN);
    (*state).client_nonce =
        malloc((encoded_len + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
    if !(*state).client_nonce.is_null() {
        encoded_len = pg_b64_encode(
            &raw mut raw_nonce as *mut uint8_t,
            SCRAM_RAW_NONCE_LEN,
            (*state).client_nonce,
            encoded_len,
        );
        if encoded_len >= 0 as ::core::ffi::c_int {
            *(*state).client_nonce.offset(encoded_len as isize) =
                '\0' as i32 as ::core::ffi::c_char;
            len = (8 as size_t)
                .wrapping_add(strlen((*state).client_nonce))
                .wrapping_add(1 as size_t);
            result = malloc(len) as *mut ::core::ffi::c_char;
            if !result.is_null() {
                snprintf(
                    result,
                    len,
                    b"n,,n=,r=%s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*state).client_nonce,
                );
                (*state).client_first_message_bare =
                    strdup(result.offset(3 as ::core::ffi::c_int as isize));
                if !(*state).client_first_message_bare.is_null() {
                    return result;
                }
            }
        }
    }
    free(result as *mut ::core::ffi::c_void);
    free((*state).client_nonce as *mut ::core::ffi::c_void);
    free((*state).client_first_message_bare as *mut ::core::ffi::c_void);
    ::core::ptr::null_mut::<::core::ffi::c_char>()
}
#[no_mangle]

pub unsafe extern "C" fn build_client_final_message(
    mut server: *mut PgSocket,
    mut credentials: *const PgCredentials,
) -> *mut ::core::ffi::c_char {
    let mut state: *mut ScramState = &raw mut (*server).scram_state;
    let mut buf: [::core::ffi::c_char; 512] = [0; 512];
    let mut len: size_t = 0;
    let mut client_proof: [uint8_t; 32] = [0; 32];
    let mut enclen: ::core::ffi::c_int = 0;
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
        b"c=biws,r=%s\0" as *const u8 as *const ::core::ffi::c_char,
        (*state).server_nonce,
    );
    (*state).client_final_message_without_proof = strdup(&raw mut buf as *mut ::core::ffi::c_char);
    if !(*state).client_final_message_without_proof.is_null()
        && calculate_client_proof(
            server,
            credentials,
            &raw mut buf as *mut ::core::ffi::c_char,
            &raw mut client_proof as *mut uint8_t,
        )
    {
        len = strlcat(
            &raw mut buf as *mut ::core::ffi::c_char,
            b",p=\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
        ) as size_t;
        enclen = pg_b64_enc_len(::core::mem::size_of::<[uint8_t; 32]>() as ::core::ffi::c_int);
        enclen = pg_b64_encode(
            &raw mut client_proof as *mut uint8_t,
            SCRAM_SHA_256_KEY_LEN,
            (&raw mut buf as *mut ::core::ffi::c_char).add(len),
            enclen,
        );
        if enclen >= 0 as ::core::ffi::c_int {
            len = len.wrapping_add(enclen as size_t);
            buf[len as usize] = '\0' as i32 as ::core::ffi::c_char;
            return strdup(&raw mut buf as *mut ::core::ffi::c_char);
        }
    }
    ::core::ptr::null_mut::<::core::ffi::c_char>()
}
#[no_mangle]

pub unsafe extern "C" fn read_server_first_message(
    mut server: *mut PgSocket,
    mut input: *mut ::core::ffi::c_char,
) -> bool {
    let mut state: *mut ScramState = &raw mut (*server).scram_state;
    let mut server_nonce = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut encoded_salt = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut decoded_salt_len: ::core::ffi::c_int = 0;
    let mut iterations_str = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut endptr = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut iterations: ::core::ffi::c_int = 0;
    (*state).server_first_message = strdup(input);
    if (*state).server_first_message.is_null() {
        return false_0 != 0;
    }
    server_nonce = read_attr_value(server, &raw mut input, 'r' as i32 as ::core::ffi::c_char);
    if server_nonce.is_null() {
        return false_0 != 0;
    }
    if strlen(server_nonce) < strlen((*state).client_nonce)
        || memcmp(
            server_nonce as *const ::core::ffi::c_void,
            (*state).client_nonce as *const ::core::ffi::c_void,
            strlen((*state).client_nonce),
        ) != 0 as ::core::ffi::c_int
    {
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            b"invalid SCRAM response (nonce mismatch)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    (*state).server_nonce = strdup(server_nonce);
    if (*state).server_nonce.is_null() {
        return false_0 != 0;
    }
    encoded_salt = read_attr_value(server, &raw mut input, 's' as i32 as ::core::ffi::c_char);
    if encoded_salt.is_null() {
        return false_0 != 0;
    }
    decoded_salt_len = pg_b64_dec_len(strlen(encoded_salt) as ::core::ffi::c_int);
    (*state).salt = malloc(decoded_salt_len as size_t) as *mut uint8_t;
    if (*state).salt.is_null() {
        return false_0 != 0;
    }
    (*state).saltlen = pg_b64_decode(
        encoded_salt,
        strlen(encoded_salt) as ::core::ffi::c_int,
        (*state).salt,
        decoded_salt_len,
    );
    if (*state).saltlen < 0 as ::core::ffi::c_int {
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            b"malformed SCRAM message (invalid salt)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    iterations_str = read_attr_value(server, &raw mut input, 'i' as i32 as ::core::ffi::c_char);
    if iterations_str.is_null() {
        return false_0 != 0;
    }
    iterations =
        strtol(iterations_str, &raw mut endptr, 10 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if *endptr as ::core::ffi::c_int != '\0' as i32 || iterations < 1 as ::core::ffi::c_int {
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            b"malformed SCRAM message (invalid iteration count)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    (*state).iterations = iterations;
    if *input as ::core::ffi::c_int != '\0' as i32 {
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            b"malformed SCRAM message (garbage at end of server-first-message)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    true_0 != 0
}
#[no_mangle]

pub unsafe extern "C" fn read_server_final_message(
    mut server: *mut PgSocket,
    mut input: *mut ::core::ffi::c_char,
    mut ServerSignature: *mut ::core::ffi::c_char,
) -> bool {
    let mut encoded_server_signature = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut decoded_server_signature = ::core::ptr::null_mut::<uint8_t>();
    let mut server_signature_len: ::core::ffi::c_int = 0;
    if *input as ::core::ffi::c_int == 'e' as i32 {
        let mut errmsg = read_attr_value(server, &raw mut input, 'e' as i32 as ::core::ffi::c_char);
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            b"error received from server in SCRAM exchange: %s\0" as *const u8
                as *const ::core::ffi::c_char,
            errmsg,
        );
    } else {
        encoded_server_signature =
            read_attr_value(server, &raw mut input, 'v' as i32 as ::core::ffi::c_char);
        if !encoded_server_signature.is_null() {
            if *input as ::core::ffi::c_int != '\0' as i32 {
                log_generic(
                    LG_ERROR,
                    server as *mut ::core::ffi::c_void,
                    b"malformed SCRAM message (garbage at end of server-final-message)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            server_signature_len =
                pg_b64_dec_len(strlen(encoded_server_signature) as ::core::ffi::c_int);
            decoded_server_signature = malloc(server_signature_len as size_t) as *mut uint8_t;
            if !decoded_server_signature.is_null() {
                server_signature_len = pg_b64_decode(
                    encoded_server_signature,
                    strlen(encoded_server_signature) as ::core::ffi::c_int,
                    decoded_server_signature,
                    server_signature_len,
                );
                if server_signature_len != SCRAM_SHA_256_KEY_LEN {
                    log_generic(
                        LG_ERROR,
                        server as *mut ::core::ffi::c_void,
                        b"malformed SCRAM message (malformed server signature)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                } else {
                    memcpy(
                        ServerSignature as *mut ::core::ffi::c_void,
                        decoded_server_signature as *const ::core::ffi::c_void,
                        SCRAM_SHA_256_KEY_LEN as size_t,
                    );
                    free(decoded_server_signature as *mut ::core::ffi::c_void);
                    return true_0 != 0;
                }
            }
        }
    }
    free(decoded_server_signature as *mut ::core::ffi::c_void);
    false_0 != 0
}

unsafe extern "C" fn calculate_client_proof(
    mut server: *mut PgSocket,
    mut credentials: *const PgCredentials,
    mut client_final_message_without_proof: *const ::core::ffi::c_char,
    mut result: *mut uint8_t,
) -> bool {
    let mut current_block: u64;
    let mut state: *mut ScramState = &raw mut (*server).scram_state;
    let mut rc = SASLPREP_SUCCESS;
    let mut prep_password = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut StoredKey: [uint8_t; 32] = [0; 32];
    let mut ClientKey: [uint8_t; 32] = [0; 32];
    let mut ClientSignature: [uint8_t; 32] = [0; 32];
    let mut i: ::core::ffi::c_int = 0;
    let mut ctx = ::core::ptr::null_mut::<pg_hmac_ctx>();
    let mut errstr = ::core::ptr::null::<::core::ffi::c_char>();
    ctx = pg_hmac_create((*state).hash_type);
    if ctx.is_null() {
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            b"HMAC context creation failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
            pg_hmac_error(::core::ptr::null_mut::<pg_hmac_ctx>()),
        );
    } else {
        if (*credentials).use_scram_keys {
            memcpy(
                &raw mut ClientKey as *mut uint8_t as *mut ::core::ffi::c_void,
                &raw const (*credentials).scram_ClientKey as *const uint8_t
                    as *const ::core::ffi::c_void,
                SCRAM_SHA_256_KEY_LEN as size_t,
            );
            current_block = 4956146061682418353;
        } else {
            rc = pg_saslprep(
                &raw const (*credentials).passwd as *const ::core::ffi::c_char,
                &raw mut prep_password,
            );
            if rc as ::core::ffi::c_int == SASLPREP_OOM as ::core::ffi::c_int {
                current_block = 13714201677023511950;
            } else {
                if rc as ::core::ffi::c_int != SASLPREP_SUCCESS as ::core::ffi::c_int {
                    prep_password =
                        strdup(&raw const (*credentials).passwd as *const ::core::ffi::c_char);
                    if prep_password.is_null() {
                        current_block = 13714201677023511950;
                    } else {
                        current_block = 2979737022853876585;
                    }
                } else {
                    current_block = 2979737022853876585;
                }
                match current_block {
                    13714201677023511950 => {}
                    _ => {
                        (*state).SaltedPassword =
                            malloc(SCRAM_SHA_256_KEY_LEN as size_t) as *mut uint8_t;
                        if (*state).SaltedPassword.is_null() {
                            current_block = 13714201677023511950;
                        } else if scram_SaltedPassword(
                            prep_password,
                            (*state).hash_type,
                            (*state).key_length,
                            (*state).salt,
                            (*state).saltlen,
                            (*state).iterations,
                            (*state).SaltedPassword,
                            &raw mut errstr,
                        ) < 0 as ::core::ffi::c_int
                            || scram_ClientKey(
                                (*state).SaltedPassword,
                                (*state).hash_type,
                                (*state).key_length,
                                &raw mut ClientKey as *mut uint8_t,
                                &raw mut errstr,
                            ) < 0 as ::core::ffi::c_int
                        {
                            log_generic(
                                LG_ERROR,
                                server as *mut ::core::ffi::c_void,
                                b"SCRAM key derivation failed: %s\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                errstr,
                            );
                            current_block = 13714201677023511950;
                        } else {
                            current_block = 4956146061682418353;
                        }
                    }
                }
            }
        }
        match current_block {
            13714201677023511950 => {}
            _ => {
                if scram_H(
                    &raw mut ClientKey as *mut uint8_t,
                    (*state).hash_type,
                    (*state).key_length,
                    &raw mut StoredKey as *mut uint8_t,
                    &raw mut errstr,
                ) < 0 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_ERROR,
                        server as *mut ::core::ffi::c_void,
                        b"SCRAM hash computation failed: %s\0" as *const u8
                            as *const ::core::ffi::c_char,
                        errstr,
                    );
                } else if pg_hmac_init(
                    ctx,
                    &raw mut StoredKey as *mut uint8_t,
                    (*state).key_length as size_t,
                ) < 0 as ::core::ffi::c_int
                    || pg_hmac_update(
                        ctx,
                        (*state).client_first_message_bare as *mut uint8_t,
                        strlen((*state).client_first_message_bare),
                    ) < 0 as ::core::ffi::c_int
                    || pg_hmac_update(
                        ctx,
                        b",\0" as *const u8 as *const ::core::ffi::c_char as *mut uint8_t,
                        1 as size_t,
                    ) < 0 as ::core::ffi::c_int
                    || pg_hmac_update(
                        ctx,
                        (*state).server_first_message as *mut uint8_t,
                        strlen((*state).server_first_message),
                    ) < 0 as ::core::ffi::c_int
                    || pg_hmac_update(
                        ctx,
                        b",\0" as *const u8 as *const ::core::ffi::c_char as *mut uint8_t,
                        1 as size_t,
                    ) < 0 as ::core::ffi::c_int
                    || pg_hmac_update(
                        ctx,
                        client_final_message_without_proof as *mut uint8_t,
                        strlen(client_final_message_without_proof),
                    ) < 0 as ::core::ffi::c_int
                    || pg_hmac_final(
                        ctx,
                        &raw mut ClientSignature as *mut uint8_t,
                        (*state).key_length as size_t,
                    ) < 0 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_ERROR,
                        server as *mut ::core::ffi::c_void,
                        b"HMAC computation failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
                        pg_hmac_error(ctx),
                    );
                } else {
                    i = 0 as ::core::ffi::c_int;
                    while i < (*state).key_length {
                        *result.offset(i as isize) = (ClientKey[i as usize] as ::core::ffi::c_int
                            ^ ClientSignature[i as usize] as ::core::ffi::c_int)
                            as uint8_t;
                        i += 1;
                    }
                    free(prep_password as *mut ::core::ffi::c_void);
                    pg_hmac_free(ctx);
                    return true_0 != 0;
                }
            }
        }
    }
    free(prep_password as *mut ::core::ffi::c_void);
    pg_hmac_free(ctx);
    false_0 != 0
}
#[no_mangle]

pub unsafe extern "C" fn verify_server_signature(
    mut server: *mut PgSocket,
    mut credentials: *const PgCredentials,
    mut ServerSignature: *const ::core::ffi::c_char,
    mut match_0: *mut bool,
) -> bool {
    let mut state: *mut ScramState = &raw mut (*server).scram_state;
    let mut expected_ServerSignature: [uint8_t; 32] = [0; 32];
    let mut ServerKey: [uint8_t; 32] = [0; 32];
    let mut ctx = ::core::ptr::null_mut::<pg_hmac_ctx>();
    let mut errstr = ::core::ptr::null::<::core::ffi::c_char>();
    ctx = pg_hmac_create((*state).hash_type);
    if ctx.is_null() {
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            b"HMAC context creation failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
            pg_hmac_error(::core::ptr::null_mut::<pg_hmac_ctx>()),
        );
        return false_0 != 0;
    }
    if (*credentials).use_scram_keys {
        memcpy(
            &raw mut ServerKey as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw const (*credentials).scram_ServerKey as *const uint8_t
                as *const ::core::ffi::c_void,
            SCRAM_SHA_256_KEY_LEN as size_t,
        );
    } else if scram_ServerKey(
        (*state).SaltedPassword,
        (*state).hash_type,
        (*state).key_length,
        &raw mut ServerKey as *mut uint8_t,
        &raw mut errstr,
    ) < 0 as ::core::ffi::c_int
    {
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            b"SCRAM server key derivation failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
            errstr,
        );
        pg_hmac_free(ctx);
        return false_0 != 0;
    }
    if pg_hmac_init(
        ctx,
        &raw mut ServerKey as *mut uint8_t,
        (*state).key_length as size_t,
    ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            (*state).client_first_message_bare as *mut uint8_t,
            strlen((*state).client_first_message_bare),
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            b",\0" as *const u8 as *const ::core::ffi::c_char as *mut uint8_t,
            1 as size_t,
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            (*state).server_first_message as *mut uint8_t,
            strlen((*state).server_first_message),
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            b",\0" as *const u8 as *const ::core::ffi::c_char as *mut uint8_t,
            1 as size_t,
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            (*state).client_final_message_without_proof as *mut uint8_t,
            strlen((*state).client_final_message_without_proof),
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_final(
            ctx,
            &raw mut expected_ServerSignature as *mut uint8_t,
            (*state).key_length as size_t,
        ) < 0 as ::core::ffi::c_int
    {
        log_generic(
            LG_ERROR,
            server as *mut ::core::ffi::c_void,
            b"HMAC server signature computation failed: %s\0" as *const u8
                as *const ::core::ffi::c_char,
            pg_hmac_error(ctx),
        );
        pg_hmac_free(ctx);
        return false_0 != 0;
    }
    pg_hmac_free(ctx);
    if memcmp(
        &raw mut expected_ServerSignature as *mut uint8_t as *const ::core::ffi::c_void,
        ServerSignature as *const ::core::ffi::c_void,
        (*state).key_length as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        *match_0 = false_0 != 0;
    } else {
        *match_0 = true_0 != 0;
    }
    true_0 != 0
}
#[no_mangle]

pub unsafe extern "C" fn read_client_first_message(
    mut client: *mut PgSocket,
    mut input: *mut ::core::ffi::c_char,
) -> bool {
    let mut current_block: u64;
    let mut state: *mut ScramState = &raw mut (*client).scram_state;
    let mut client_first_message_bare = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut client_nonce = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut client_nonce_copy = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*state).cbind_flag = *input;
    match *input as ::core::ffi::c_int {
        110 => {
            input = input.offset(1);
            current_block = 8515828400728868193;
        }
        121 => {
            input = input.offset(1);
            current_block = 8515828400728868193;
        }
        112 => {
            log_generic(
                LG_ERROR,
                client as *mut ::core::ffi::c_void,
                b"client requires SCRAM channel binding, but it is not supported\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            current_block = 5151340100945259836;
        }
        _ => {
            log_generic(
                LG_ERROR,
                client as *mut ::core::ffi::c_void,
                b"malformed SCRAM message (unexpected channel-binding flag \"%s\")\0" as *const u8
                    as *const ::core::ffi::c_char,
                sanitize_char(*input),
            );
            current_block = 5151340100945259836;
        }
    }
    if current_block == 8515828400728868193 {
        if *input as ::core::ffi::c_int != ',' as i32 {
            log_generic(
                LG_ERROR,
                client as *mut ::core::ffi::c_void,
                b"malformed SCRAM message (comma expected, but found character \"%s\")\0"
                    as *const u8 as *const ::core::ffi::c_char,
                sanitize_char(*input),
            );
        } else {
            input = input.offset(1);
            if *input as ::core::ffi::c_int == 'a' as i32 {
                log_generic(
                    LG_ERROR,
                    client as *mut ::core::ffi::c_void,
                    b"client uses authorization identity, but it is not supported\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            } else if *input as ::core::ffi::c_int != ',' as i32 {
                log_generic(
                    LG_ERROR,
                    client as *mut ::core::ffi::c_void,
                    b"malformed SCRAM message (unexpected attribute \"%s\" in client-first-message)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    sanitize_char(*input),
                );
            } else {
                input = input.offset(1);
                client_first_message_bare = strdup(input);
                if !client_first_message_bare.is_null() {
                    if *input as ::core::ffi::c_int == 'm' as i32 {
                        log_generic(
                            LG_ERROR,
                            client as *mut ::core::ffi::c_void,
                            b"client requires an unsupported SCRAM extension\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    } else {
                        read_attr_value(client, &raw mut input, 'n' as i32 as ::core::ffi::c_char);
                        client_nonce = read_attr_value(
                            client,
                            &raw mut input,
                            'r' as i32 as ::core::ffi::c_char,
                        );
                        if !client_nonce.is_null() {
                            if !is_scram_printable(client_nonce) {
                                log_generic(
                                    LG_ERROR,
                                    client as *mut ::core::ffi::c_void,
                                    b"non-printable characters in SCRAM nonce\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                            } else {
                                client_nonce_copy = strdup(client_nonce);
                                if !client_nonce_copy.is_null() {
                                    loop {
                                        if *input as ::core::ffi::c_int == '\0' as i32 {
                                            current_block = 13550086250199790493;
                                            break;
                                        }
                                        if read_any_attr(
                                            client,
                                            &raw mut input,
                                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                        )
                                        .is_null()
                                        {
                                            current_block = 5151340100945259836;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        5151340100945259836 => {}
                                        _ => {
                                            (*state).client_first_message_bare =
                                                client_first_message_bare;
                                            (*state).client_nonce = client_nonce_copy;
                                            return true_0 != 0;
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
    free(client_first_message_bare as *mut ::core::ffi::c_void);
    free(client_nonce_copy as *mut ::core::ffi::c_void);
    false_0 != 0
}
#[no_mangle]

pub unsafe extern "C" fn read_client_final_message(
    mut client: *mut PgSocket,
    mut raw_input: *const uint8_t,
    mut input: *mut ::core::ffi::c_char,
    mut client_final_nonce_p: *mut *const ::core::ffi::c_char,
    mut proof_p: *mut *mut ::core::ffi::c_char,
) -> bool {
    let mut state: *mut ScramState = &raw mut (*client).scram_state;
    let mut input_start: *const ::core::ffi::c_char = input;
    let mut attr: ::core::ffi::c_char = 0;
    let mut channel_binding = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut client_final_nonce = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut proof_start = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut value = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut encoded_proof = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut proof = ::core::ptr::null_mut::<uint8_t>();
    let mut prooflen: ::core::ffi::c_int = 0;
    channel_binding = read_attr_value(client, &raw mut input, 'c' as i32 as ::core::ffi::c_char);
    if !channel_binding.is_null() {
        if !(strcmp(
            channel_binding,
            b"biws\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && (*state).cbind_flag as ::core::ffi::c_int == 'n' as i32)
            && !(strcmp(
                channel_binding,
                b"eSws\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                && (*state).cbind_flag as ::core::ffi::c_int == 'y' as i32)
        {
            log_generic(
                LG_ERROR,
                client as *mut ::core::ffi::c_void,
                b"unexpected SCRAM channel-binding attribute in client-final-message\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        } else {
            client_final_nonce =
                read_attr_value(client, &raw mut input, 'r' as i32 as ::core::ffi::c_char);
            loop {
                proof_start = input.offset(-(1 as ::core::ffi::c_int as isize));
                value = read_any_attr(client, &raw mut input, &raw mut attr);
                if !(!value.is_null() && attr as ::core::ffi::c_int != 'p' as i32) {
                    break;
                }
            }
            if value.is_null() {
                log_generic(
                    LG_ERROR,
                    client as *mut ::core::ffi::c_void,
                    b"could not read proof\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
                encoded_proof = value;
                prooflen = pg_b64_dec_len(strlen(encoded_proof) as ::core::ffi::c_int);
                proof = malloc(prooflen as size_t) as *mut uint8_t;
                if proof.is_null() {
                    log_generic(
                        LG_ERROR,
                        client as *mut ::core::ffi::c_void,
                        b"could not decode proof\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else {
                    prooflen = pg_b64_decode(
                        encoded_proof,
                        strlen(encoded_proof) as ::core::ffi::c_int,
                        proof,
                        prooflen,
                    );
                    if prooflen != SCRAM_SHA_256_KEY_LEN {
                        log_generic(
                            LG_ERROR,
                            client as *mut ::core::ffi::c_void,
                            b"malformed SCRAM message (malformed proof in client-final-message)\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    } else if *input as ::core::ffi::c_int != '\0' as i32 {
                        log_generic(
                            LG_ERROR,
                            client as *mut ::core::ffi::c_void,
                            b"malformed SCRAM message (garbage at the end of client-final-message)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                        (*state).client_final_message_without_proof = malloc(
                            (proof_start.offset_from(input_start) as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long) as size_t,
                        )
                            as *mut ::core::ffi::c_char;
                        if !(*state).client_final_message_without_proof.is_null() {
                            memcpy(
                                (*state).client_final_message_without_proof
                                    as *mut ::core::ffi::c_void,
                                raw_input as *const ::core::ffi::c_void,
                                proof_start.offset_from(input_start) as ::core::ffi::c_long
                                    as size_t,
                            );
                            *(*state).client_final_message_without_proof.offset(
                                proof_start.offset_from(input_start) as ::core::ffi::c_long
                                    as isize,
                            ) = '\0' as i32 as ::core::ffi::c_char;
                            *client_final_nonce_p = client_final_nonce;
                            *proof_p = proof as *mut ::core::ffi::c_char;
                            return true_0 != 0;
                        }
                    }
                }
            }
        }
    }
    free(proof as *mut ::core::ffi::c_void);
    false_0 != 0
}

unsafe extern "C" fn build_adhoc_scram_secret(
    mut plain_password: *const ::core::ffi::c_char,
    mut state: *mut ScramState,
) -> bool {
    let mut password = ::core::ptr::null::<::core::ffi::c_char>();
    let mut prep_password = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rc = SASLPREP_SUCCESS;
    let mut saltbuf: [uint8_t; 16] = [0; 16];
    let mut encoded_len: ::core::ffi::c_int = 0;
    let mut salted_password: [uint8_t; 32] = [0; 32];
    let mut errstr = ::core::ptr::null::<::core::ffi::c_char>();
    rc = pg_saslprep(plain_password, &raw mut prep_password);
    if rc as ::core::ffi::c_int != SASLPREP_OOM as ::core::ffi::c_int {
        if rc as ::core::ffi::c_int == SASLPREP_SUCCESS as ::core::ffi::c_int {
            password = prep_password;
        } else {
            password = plain_password;
        }
        get_random_bytes(
            &raw mut saltbuf as *mut uint8_t,
            ::core::mem::size_of::<[uint8_t; 16]>() as ::core::ffi::c_int,
        );
        (*state).adhoc = true_0 != 0;
        (*state).iterations = cf_scram_iterations;
        encoded_len = pg_b64_enc_len(::core::mem::size_of::<[uint8_t; 16]>() as ::core::ffi::c_int);
        (*state).encoded_salt =
            malloc((encoded_len + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
        if !(*state).encoded_salt.is_null() {
            encoded_len = pg_b64_encode(
                &raw mut saltbuf as *mut uint8_t,
                ::core::mem::size_of::<[uint8_t; 16]>() as ::core::ffi::c_int,
                (*state).encoded_salt,
                encoded_len,
            );
            if encoded_len >= 0 as ::core::ffi::c_int {
                *(*state).encoded_salt.offset(encoded_len as isize) =
                    '\0' as i32 as ::core::ffi::c_char;
                scram_SaltedPassword(
                    password,
                    (*state).hash_type,
                    (*state).key_length,
                    &raw mut saltbuf as *mut uint8_t,
                    ::core::mem::size_of::<[uint8_t; 16]>() as ::core::ffi::c_int,
                    (*state).iterations,
                    &raw mut salted_password as *mut uint8_t,
                    &raw mut errstr,
                );
                scram_ClientKey(
                    &raw mut salted_password as *mut uint8_t,
                    (*state).hash_type,
                    (*state).key_length,
                    &raw mut (*state).StoredKey as *mut uint8_t,
                    &raw mut errstr,
                );
                scram_H(
                    &raw mut (*state).StoredKey as *mut uint8_t,
                    (*state).hash_type,
                    (*state).key_length,
                    &raw mut (*state).StoredKey as *mut uint8_t,
                    &raw mut errstr,
                );
                scram_ServerKey(
                    &raw mut salted_password as *mut uint8_t,
                    (*state).hash_type,
                    (*state).key_length,
                    &raw mut (*state).ServerKey as *mut uint8_t,
                    &raw mut errstr,
                );
                free(prep_password as *mut ::core::ffi::c_void);
                return true_0 != 0;
            }
        }
    }
    free(prep_password as *mut ::core::ffi::c_void);
    false_0 != 0
}

unsafe extern "C" fn scram_mock_salt(
    mut username: *const ::core::ffi::c_char,
    mut saltbuf: *mut uint8_t,
) -> bool {
    static mut mock_auth_nonce: [uint8_t; 32] = [0; 32];
    static mut mock_auth_nonce_initialized: bool = false_0 != 0;
    let mut ctx = ::core::ptr::null_mut::<pg_cryptohash_ctx>();
    let mut sha_digest: [uint8_t; 32] = [0; 32];
    if !mock_auth_nonce_initialized {
        get_random_bytes(
            &raw mut mock_auth_nonce as *mut uint8_t,
            ::core::mem::size_of::<[uint8_t; 32]>() as ::core::ffi::c_int,
        );
        mock_auth_nonce_initialized = true_0 != 0;
    }
    ctx = pg_cryptohash_create(PG_SHA256);
    if ctx.is_null() {
        let mut _log_ctx = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx,
            b"could not create cryptohash context\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    if pg_cryptohash_init(ctx) < 0 as ::core::ffi::c_int
        || pg_cryptohash_update(ctx, username as *mut uint8_t, strlen(username))
            < 0 as ::core::ffi::c_int
        || pg_cryptohash_update(
            ctx,
            &raw mut mock_auth_nonce as *mut uint8_t,
            ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
        ) < 0 as ::core::ffi::c_int
        || pg_cryptohash_final(
            ctx,
            &raw mut sha_digest as *mut uint8_t,
            ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
        ) < 0 as ::core::ffi::c_int
    {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx_0,
            b"could not generate mock salt: %s\0" as *const u8 as *const ::core::ffi::c_char,
            pg_cryptohash_error(ctx),
        );
        pg_cryptohash_free(ctx);
        return false_0 != 0;
    }
    pg_cryptohash_free(ctx);
    memcpy(
        saltbuf as *mut ::core::ffi::c_void,
        &raw mut sha_digest as *mut uint8_t as *const ::core::ffi::c_void,
        SCRAM_DEFAULT_SALT_LEN as size_t,
    );
    true_0 != 0
}

unsafe extern "C" fn build_mock_scram_secret(
    mut username: *const ::core::ffi::c_char,
    mut state: *mut ScramState,
) -> bool {
    let mut saltbuf: [uint8_t; 16] = [0; 16];
    let mut encoded_len: ::core::ffi::c_int = 0;
    (*state).iterations = cf_scram_iterations;
    if scram_mock_salt(username, &raw mut saltbuf as *mut uint8_t) {
        encoded_len = pg_b64_enc_len(::core::mem::size_of::<[uint8_t; 16]>() as ::core::ffi::c_int);
        (*state).encoded_salt =
            malloc((encoded_len + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
        if !(*state).encoded_salt.is_null() {
            encoded_len = pg_b64_encode(
                &raw mut saltbuf as *mut uint8_t,
                ::core::mem::size_of::<[uint8_t; 16]>() as ::core::ffi::c_int,
                (*state).encoded_salt,
                encoded_len,
            );
            if encoded_len >= 0 as ::core::ffi::c_int {
                *(*state).encoded_salt.offset(encoded_len as isize) =
                    '\0' as i32 as ::core::ffi::c_char;
                return true_0 != 0;
            }
        }
    }
    false_0 != 0
}
#[no_mangle]

pub unsafe extern "C" fn build_server_first_message(
    mut state: *mut ScramState,
    mut user: *mut PgCredentials,
    mut stored_secret: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut current_block: u64;
    let mut raw_nonce: [uint8_t; 19] = [0; 19];
    let mut encoded_len: ::core::ffi::c_int = 0;
    let mut len: size_t = 0;
    let mut result = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*state).hash_type = PG_SHA256;
    (*state).key_length = SCRAM_SHA_256_KEY_LEN;
    if stored_secret.is_null() {
        if !build_mock_scram_secret(&raw mut (*user).name as *mut ::core::ffi::c_char, state) {
            current_block = 4658626102049014135;
        } else {
            current_block = 5601891728916014340;
        }
    } else if (*user).adhoc_scram_secrets_cached {
        (*state).adhoc = true_0 != 0;
        (*state).iterations = (*user).scram_Iiterations;
        (*state).encoded_salt = strdup((*user).scram_SaltKey);
        memcpy(
            &raw mut (*state).StoredKey as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw mut (*user).scram_StoredKey as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
        );
        memcpy(
            &raw mut (*state).ServerKey as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw mut (*user).scram_ServerKey as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
        );
        current_block = 5601891728916014340;
    } else {
        match get_password_type(stored_secret) as ::core::ffi::c_uint {
            2 => {
                current_block = 1894949217534896604;
                match current_block {
                    2508738315967109217 => {
                        if !build_adhoc_scram_secret(stored_secret, state) {
                            current_block = 4658626102049014135;
                        } else {
                            current_block = 8236137900636309791;
                        }
                    }
                    _ => {
                        if !parse_scram_secret(
                            stored_secret,
                            &raw mut (*state).iterations,
                            &raw mut (*state).encoded_salt,
                            &raw mut (*state).StoredKey as *mut uint8_t,
                            &raw mut (*state).ServerKey as *mut uint8_t,
                        ) {
                            current_block = 4658626102049014135;
                        } else {
                            current_block = 8236137900636309791;
                        }
                    }
                }
                match current_block {
                    4658626102049014135 => {}
                    _ => {
                        if !(*user).dynamic_passwd {
                            (*user).scram_Iiterations = (*state).iterations;
                            (*user).scram_SaltKey = strdup((*state).encoded_salt);
                            memcpy(
                                &raw mut (*user).scram_StoredKey as *mut uint8_t
                                    as *mut ::core::ffi::c_void,
                                &raw mut (*state).StoredKey as *mut uint8_t
                                    as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
                            );
                            memcpy(
                                &raw mut (*user).scram_ServerKey as *mut uint8_t
                                    as *mut ::core::ffi::c_void,
                                &raw mut (*state).ServerKey as *mut uint8_t
                                    as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
                            );
                            (*user).adhoc_scram_secrets_cached = true_0 != 0;
                        }
                        current_block = 5601891728916014340;
                    }
                }
            }
            0 => {
                current_block = 2508738315967109217;
                match current_block {
                    2508738315967109217 => {
                        if !build_adhoc_scram_secret(stored_secret, state) {
                            current_block = 4658626102049014135;
                        } else {
                            current_block = 8236137900636309791;
                        }
                    }
                    _ => {
                        if !parse_scram_secret(
                            stored_secret,
                            &raw mut (*state).iterations,
                            &raw mut (*state).encoded_salt,
                            &raw mut (*state).StoredKey as *mut uint8_t,
                            &raw mut (*state).ServerKey as *mut uint8_t,
                        ) {
                            current_block = 4658626102049014135;
                        } else {
                            current_block = 8236137900636309791;
                        }
                    }
                }
                match current_block {
                    4658626102049014135 => {}
                    _ => {
                        if !(*user).dynamic_passwd {
                            (*user).scram_Iiterations = (*state).iterations;
                            (*user).scram_SaltKey = strdup((*state).encoded_salt);
                            memcpy(
                                &raw mut (*user).scram_StoredKey as *mut uint8_t
                                    as *mut ::core::ffi::c_void,
                                &raw mut (*state).StoredKey as *mut uint8_t
                                    as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
                            );
                            memcpy(
                                &raw mut (*user).scram_ServerKey as *mut uint8_t
                                    as *mut ::core::ffi::c_void,
                                &raw mut (*state).ServerKey as *mut uint8_t
                                    as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
                            );
                            (*user).adhoc_scram_secrets_cached = true_0 != 0;
                        }
                        current_block = 5601891728916014340;
                    }
                }
            }
            _ => {
                current_block = 4658626102049014135;
            }
        }
    }
    if current_block == 5601891728916014340 {
        get_random_bytes(&raw mut raw_nonce as *mut uint8_t, SCRAM_RAW_NONCE_LEN);
        encoded_len = pg_b64_enc_len(SCRAM_RAW_NONCE_LEN);
        (*state).server_nonce =
            malloc((encoded_len + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
        if !(*state).server_nonce.is_null() {
            encoded_len = pg_b64_encode(
                &raw mut raw_nonce as *mut uint8_t,
                SCRAM_RAW_NONCE_LEN,
                (*state).server_nonce,
                encoded_len,
            );
            if encoded_len >= 0 as ::core::ffi::c_int {
                *(*state).server_nonce.offset(encoded_len as isize) =
                    '\0' as i32 as ::core::ffi::c_char;
                len = (2 as size_t)
                    .wrapping_add(strlen((*state).client_nonce))
                    .wrapping_add(strlen((*state).server_nonce))
                    .wrapping_add(3 as size_t)
                    .wrapping_add(strlen((*state).encoded_salt))
                    .wrapping_add(3 as size_t)
                    .wrapping_add(10 as size_t)
                    .wrapping_add(1 as size_t);
                result = malloc(len) as *mut ::core::ffi::c_char;
                if !result.is_null() {
                    snprintf(
                        result,
                        len,
                        b"r=%s%s,s=%s,i=%u\0" as *const u8 as *const ::core::ffi::c_char,
                        (*state).client_nonce,
                        (*state).server_nonce,
                        (*state).encoded_salt,
                        (*state).iterations,
                    );
                    (*state).server_first_message = result;
                    return result;
                }
            }
        }
    }
    free((*state).server_nonce as *mut ::core::ffi::c_void);
    free((*state).server_first_message as *mut ::core::ffi::c_void);
    ::core::ptr::null_mut::<::core::ffi::c_char>()
}

unsafe extern "C" fn compute_server_signature(
    mut client: *mut PgSocket,
    mut state: *mut ScramState,
) -> *mut ::core::ffi::c_char {
    let mut ServerSignature: [uint8_t; 32] = [0; 32];
    let mut server_signature_base64 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut siglen: ::core::ffi::c_int = 0;
    let mut ctx = ::core::ptr::null_mut::<pg_hmac_ctx>();
    ctx = pg_hmac_create((*state).hash_type);
    if ctx.is_null() {
        log_generic(
            LG_ERROR,
            client as *mut ::core::ffi::c_void,
            b"HMAC context creation failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
            pg_hmac_error(::core::ptr::null_mut::<pg_hmac_ctx>()),
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if pg_hmac_init(
        ctx,
        &raw mut (*state).ServerKey as *mut uint8_t,
        (*state).key_length as size_t,
    ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            (*state).client_first_message_bare as *mut uint8_t,
            strlen((*state).client_first_message_bare),
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            b",\0" as *const u8 as *const ::core::ffi::c_char as *mut uint8_t,
            1 as size_t,
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            (*state).server_first_message as *mut uint8_t,
            strlen((*state).server_first_message),
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            b",\0" as *const u8 as *const ::core::ffi::c_char as *mut uint8_t,
            1 as size_t,
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            (*state).client_final_message_without_proof as *mut uint8_t,
            strlen((*state).client_final_message_without_proof),
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_final(
            ctx,
            &raw mut ServerSignature as *mut uint8_t,
            (*state).key_length as size_t,
        ) < 0 as ::core::ffi::c_int
    {
        log_generic(
            LG_ERROR,
            client as *mut ::core::ffi::c_void,
            b"HMAC operation failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
            pg_hmac_error(ctx),
        );
        pg_hmac_free(ctx);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    siglen = pg_b64_enc_len(SCRAM_SHA_256_KEY_LEN);
    server_signature_base64 =
        malloc((siglen + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
    if server_signature_base64.is_null() {
        pg_hmac_free(ctx);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    siglen = pg_b64_encode(
        &raw mut ServerSignature as *mut uint8_t,
        SCRAM_SHA_256_KEY_LEN,
        server_signature_base64,
        siglen,
    );
    if siglen < 0 as ::core::ffi::c_int {
        free(server_signature_base64 as *mut ::core::ffi::c_void);
        pg_hmac_free(ctx);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    *server_signature_base64.offset(siglen as isize) = '\0' as i32 as ::core::ffi::c_char;
    pg_hmac_free(ctx);
    server_signature_base64
}
#[no_mangle]

pub unsafe extern "C" fn build_server_final_message(
    mut client: *mut PgSocket,
) -> *mut ::core::ffi::c_char {
    let mut state: *mut ScramState = &raw mut (*client).scram_state;
    let mut server_signature = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: size_t = 0;
    let mut result = ::core::ptr::null_mut::<::core::ffi::c_char>();
    server_signature = compute_server_signature(client, state);
    if !server_signature.is_null() {
        len = (2 as size_t)
            .wrapping_add(strlen(server_signature))
            .wrapping_add(1 as size_t);
        if len < INT_MAX as size_t {
            result = malloc(len) as *mut ::core::ffi::c_char;
            if !result.is_null() {
                snprintf(
                    result,
                    len,
                    b"v=%s\0" as *const u8 as *const ::core::ffi::c_char,
                    server_signature,
                );
                free(server_signature as *mut ::core::ffi::c_void);
                return result;
            }
        }
    }
    free(server_signature as *mut ::core::ffi::c_void);
    ::core::ptr::null_mut::<::core::ffi::c_char>()
}
#[no_mangle]

pub unsafe extern "C" fn verify_final_nonce(
    mut state: *const ScramState,
    mut client_final_nonce: *const ::core::ffi::c_char,
) -> bool {
    let mut client_nonce_len = strlen((*state).client_nonce);
    let mut server_nonce_len = strlen((*state).server_nonce);
    let mut final_nonce_len = strlen(client_final_nonce);
    if final_nonce_len != client_nonce_len.wrapping_add(server_nonce_len) {
        return false_0 != 0;
    }
    if memcmp(
        client_final_nonce as *const ::core::ffi::c_void,
        (*state).client_nonce as *const ::core::ffi::c_void,
        client_nonce_len,
    ) != 0 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if memcmp(
        client_final_nonce.add(client_nonce_len) as *const ::core::ffi::c_void,
        (*state).server_nonce as *const ::core::ffi::c_void,
        server_nonce_len,
    ) != 0 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    true_0 != 0
}
#[no_mangle]

pub unsafe extern "C" fn verify_client_proof(
    mut client: *mut PgSocket,
    mut ClientProof: *const ::core::ffi::c_char,
) -> bool {
    let mut state: *mut ScramState = &raw mut (*client).scram_state;
    let mut ClientSignature: [uint8_t; 32] = [0; 32];
    let mut client_StoredKey: [uint8_t; 32] = [0; 32];
    let mut ctx = ::core::ptr::null_mut::<pg_hmac_ctx>();
    let mut i: ::core::ffi::c_int = 0;
    let mut errstr = ::core::ptr::null::<::core::ffi::c_char>();
    ctx = pg_hmac_create((*state).hash_type);
    if ctx.is_null() {
        log_generic(
            LG_ERROR,
            client as *mut ::core::ffi::c_void,
            b"HMAC context creation failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
            pg_hmac_error(::core::ptr::null_mut::<pg_hmac_ctx>()),
        );
        return false_0 != 0;
    }
    if pg_hmac_init(
        ctx,
        &raw mut (*state).StoredKey as *mut uint8_t,
        (*state).key_length as size_t,
    ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            (*state).client_first_message_bare as *mut uint8_t,
            strlen((*state).client_first_message_bare),
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            b",\0" as *const u8 as *const ::core::ffi::c_char as *mut uint8_t,
            1 as size_t,
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            (*state).server_first_message as *mut uint8_t,
            strlen((*state).server_first_message),
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            b",\0" as *const u8 as *const ::core::ffi::c_char as *mut uint8_t,
            1 as size_t,
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_update(
            ctx,
            (*state).client_final_message_without_proof as *mut uint8_t,
            strlen((*state).client_final_message_without_proof),
        ) < 0 as ::core::ffi::c_int
        || pg_hmac_final(
            ctx,
            &raw mut ClientSignature as *mut uint8_t,
            (*state).key_length as size_t,
        ) < 0 as ::core::ffi::c_int
    {
        log_generic(
            LG_ERROR,
            client as *mut ::core::ffi::c_void,
            b"HMAC operation failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
            pg_hmac_error(ctx),
        );
        pg_hmac_free(ctx);
        return false_0 != 0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*state).key_length {
        (*state).ClientKey[i as usize] = (*ClientProof.offset(i as isize) as ::core::ffi::c_int
            ^ ClientSignature[i as usize] as ::core::ffi::c_int)
            as uint8_t;
        i += 1;
    }
    if scram_H(
        &raw mut (*state).ClientKey as *mut uint8_t,
        (*state).hash_type,
        (*state).key_length,
        &raw mut client_StoredKey as *mut uint8_t,
        &raw mut errstr,
    ) < 0 as ::core::ffi::c_int
    {
        pg_hmac_free(ctx);
        return false_0 != 0;
    }
    if memcmp(
        &raw mut client_StoredKey as *mut uint8_t as *const ::core::ffi::c_void,
        &raw mut (*state).StoredKey as *mut uint8_t as *const ::core::ffi::c_void,
        (*state).key_length as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        pg_hmac_free(ctx);
        return false_0 != 0;
    }
    pg_hmac_free(ctx);
    true_0 != 0
}
#[no_mangle]

pub unsafe extern "C" fn scram_verify_plain_password(
    mut client: *mut PgSocket,
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut secret: *const ::core::ffi::c_char,
) -> bool {
    let mut encoded_salt = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut salt = ::core::ptr::null_mut::<uint8_t>();
    let mut saltlen: ::core::ffi::c_int = 0;
    let mut iterations: ::core::ffi::c_int = 0;
    let mut salted_password: [uint8_t; 32] = [0; 32];
    let mut stored_key: [uint8_t; 32] = [0; 32];
    let mut server_key: [uint8_t; 32] = [0; 32];
    let mut computed_key: [uint8_t; 32] = [0; 32];
    let mut prep_password = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rc = SASLPREP_SUCCESS;
    let mut result = false_0 != 0;
    let mut errstr = ::core::ptr::null::<::core::ffi::c_char>();
    if !parse_scram_secret(
        secret,
        &raw mut iterations,
        &raw mut encoded_salt,
        &raw mut stored_key as *mut uint8_t,
        &raw mut server_key as *mut uint8_t,
    ) {
        log_generic(
            LG_WARNING,
            client as *mut ::core::ffi::c_void,
            b"invalid SCRAM secret for user \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            username,
        );
    } else {
        saltlen = pg_b64_dec_len(strlen(encoded_salt) as ::core::ffi::c_int);
        salt = malloc(saltlen as size_t) as *mut uint8_t;
        if !salt.is_null() {
            saltlen = pg_b64_decode(
                encoded_salt,
                strlen(encoded_salt) as ::core::ffi::c_int,
                salt,
                saltlen,
            );
            if saltlen < 0 as ::core::ffi::c_int {
                log_generic(
                    LG_WARNING,
                    client as *mut ::core::ffi::c_void,
                    b"invalid SCRAM secret for user \"%s\"\0" as *const u8
                        as *const ::core::ffi::c_char,
                    username,
                );
            } else {
                rc = pg_saslprep(password, &raw mut prep_password);
                if rc as ::core::ffi::c_int == SASLPREP_SUCCESS as ::core::ffi::c_int {
                    password = prep_password;
                }
                scram_SaltedPassword(
                    password,
                    PG_SHA256,
                    SCRAM_SHA_256_KEY_LEN,
                    salt,
                    saltlen,
                    iterations,
                    &raw mut salted_password as *mut uint8_t,
                    &raw mut errstr,
                );
                scram_ServerKey(
                    &raw mut salted_password as *mut uint8_t,
                    PG_SHA256,
                    SCRAM_SHA_256_KEY_LEN,
                    &raw mut computed_key as *mut uint8_t,
                    &raw mut errstr,
                );
                result = memcmp(
                    &raw mut computed_key as *mut uint8_t as *const ::core::ffi::c_void,
                    &raw mut server_key as *mut uint8_t as *const ::core::ffi::c_void,
                    SCRAM_SHA_256_KEY_LEN as size_t,
                ) == 0 as ::core::ffi::c_int;
            }
        }
    }
    free(encoded_salt as *mut ::core::ffi::c_void);
    free(salt as *mut ::core::ffi::c_void);
    free(prep_password as *mut ::core::ffi::c_void);
    result
}
