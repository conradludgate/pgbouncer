
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
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    extern "C" {
        
        pub type tls;
        
        pub fn tls_get_connection_info(
            ctx: *mut tls,
            buf: *mut ::core::ffi::c_char,
            buflen: size_t,
        ) -> ssize_t;
    }
}

pub mod time_h {
    
    pub type usec_t = uint64_t;
    use super::_uint64_t_h::uint64_t;
    extern "C" {
        
        pub fn get_cached_time() -> usec_t;
    }
}

pub mod list_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct List {
        pub next: *mut List,
        pub prev: *mut List,
    }
    #[inline]
    
    pub unsafe extern "C" fn list_empty(mut list: *const List) -> ::core::ffi::c_int {
        std::ptr::eq((*list).next, list) as ::core::ffi::c_int
    }
    #[inline]
    
    pub unsafe extern "C" fn list_del(mut item: *mut List) -> *mut List {
        (*(*item).prev).next = (*item).next;
        (*(*item).next).prev = (*item).prev;
        (*item).prev = item;
        (*item).next = (*item).prev;
        item
    }
    #[inline]
    
    pub unsafe extern "C" fn list_pop(mut list: *mut List) -> *mut List {
        if list_empty(list) != 0 {
            return ::core::ptr::null_mut::<List>();
        }
        list_del((*list).next)
    }
}

pub mod statlist_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct StatList {
        pub head: List,
        pub cur_count: ::core::ffi::c_int,
    }
    #[inline]
    
    pub unsafe extern "C" fn statlist_count(mut list: *const StatList) -> ::core::ffi::c_int {
        (*list).cur_count
    }
    #[inline]
    
    pub unsafe extern "C" fn statlist_pop(mut list: *mut StatList) -> *mut List {
        let mut item = list_pop(&raw mut (*list).head);
        if !item.is_null() {
            (*list).cur_count -= 1;
        }
        item
    }
    #[inline]
    
    pub unsafe extern "C" fn statlist_empty(mut list: *const StatList) -> bool {
        list_empty(&raw const (*list).head) != 0
    }
    use super::list_h::{list_empty, list_pop, List};
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
    
    pub const AF_UNIX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
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
    
    pub type SSLMode = ::core::ffi::c_uint;
    
    pub const SSLMODE_VERIFY_FULL: SSLMode = 5;
    
    pub const SSLMODE_VERIFY_CA: SSLMode = 4;
    
    pub const SSLMODE_REQUIRE: SSLMode = 3;
    
    pub const SSLMODE_PREFER: SSLMode = 2;
    
    pub const SSLMODE_ALLOW: SSLMode = 1;
    
    pub const SSLMODE_DISABLED: SSLMode = 0;
    
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
    
    pub const RA_FAKE: ResponseAction = 2;
    
    pub const RA_SKIP: ResponseAction = 1;
    
    pub const RA_FORWARD: ResponseAction = 0;
    
    pub type ResponseAction = ::core::ffi::c_uint;
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct OutstandingRequest {
        pub node: List,
        pub type_0: ::core::ffi::c_char,
        pub action: ResponseAction,
        pub server_ps: *mut PgServerPreparedStatement,
        pub server_ps_query_id: uint64_t,
    }
    
    pub const POOL_SESSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    pub const POOL_STMT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    pub const POOL_INHERIT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    pub const BACKENDKEY_LEN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    #[inline]
    
    pub unsafe extern "C" fn pga_is_unix(mut a: *const PgAddr) -> bool {
        (*a).sa.sa_family as ::core::ffi::c_int == AF_UNIX
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
    use super::socket_h::{sockaddr, AF_UNIX};
    use super::statlist_h::StatList;
    use super::time_h::usec_t;
    use super::varcache_h::VarCache;
    extern "C" {
        
        pub fn pga_str(
            a: *const PgAddr,
            dst: *mut ::core::ffi::c_char,
            dstlen: ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char;
        
        pub static mut cf_pool_mode: ::core::ffi::c_int;
        
        pub static mut cf_default_pool_size: ::core::ffi::c_int;
        
        pub static mut cf_min_pool_size: ::core::ffi::c_int;
        
        pub static mut cf_res_pool_size: ::core::ffi::c_int;
        
        pub static mut cf_max_db_connections: ::core::ffi::c_int;
        
        pub static mut cf_max_db_client_connections: ::core::ffi::c_int;
        
        pub static mut cf_max_user_connections: ::core::ffi::c_int;
        
        pub static mut cf_max_user_client_connections: ::core::ffi::c_int;
        
        pub static mut cf_server_lifetime: usec_t;
        
        pub static mut cf_log_connections: ::core::ffi::c_int;
        
        pub static mut cf_max_prepared_statements: ::core::ffi::c_int;
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
    use super::mbuf_h::MBuf;
    use super::tls_h::tls;
    extern "C" {
        
        pub static mut server_connect_sslmode: ::core::ffi::c_int;
        
        pub fn sbuf_tls_connect(sbuf: *mut SBuf, hostname: *const ::core::ffi::c_char) -> bool;
        
        pub fn sbuf_continue(sbuf: *mut SBuf);
        
        pub fn sbuf_prepare_send(sbuf: *mut SBuf, dst: *mut SBuf, amount: ::core::ffi::c_uint);
        
        pub fn sbuf_prepare_skip(sbuf: *mut SBuf, amount: ::core::ffi::c_uint);
    }
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
    #[inline]
    
    pub unsafe extern "C" fn mbuf_avail_for_read(mut buf: *const MBuf) -> ::core::ffi::c_uint {
        (*buf).write_pos.wrapping_sub((*buf).read_pos)
    }
    #[inline]
    
    pub unsafe extern "C" fn mbuf_written(mut buf: *const MBuf) -> ::core::ffi::c_uint {
        (*buf).write_pos
    }
    #[inline]
    
    pub unsafe extern "C" fn mbuf_get_byte(mut buf: *mut MBuf, mut dst_p: *mut uint8_t) -> bool {
        if (*buf).read_pos.wrapping_add(1 as ::core::ffi::c_uint) > (*buf).write_pos {
            return false_0 != 0;
        }
        let fresh0 = (*buf).read_pos;
        (*buf).read_pos = (*buf).read_pos.wrapping_add(1);
        *dst_p = *(*buf).data.offset(fresh0 as isize);
        true_0 != 0
    }
    #[inline]
    
    pub unsafe extern "C" fn mbuf_get_char(
        mut buf: *mut MBuf,
        mut dst_p: *mut ::core::ffi::c_char,
    ) -> bool {
        if (*buf).read_pos.wrapping_add(1 as ::core::ffi::c_uint) > (*buf).write_pos {
            return false_0 != 0;
        }
        let fresh1 = (*buf).read_pos;
        (*buf).read_pos = (*buf).read_pos.wrapping_add(1);
        *dst_p = *(*buf).data.offset(fresh1 as isize) as ::core::ffi::c_char;
        true_0 != 0
    }
    #[inline]
    
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
    use super::_uint8_t_h::uint8_t;
    use super::stdbool_h::{false_0, true_0};
}

pub mod proto_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct PktHdr {
        pub type_0: ::core::ffi::c_uint,
        pub len: ::core::ffi::c_uint,
        pub data: MBuf,
    }
    
    pub const OLD_HEADER_LEN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    
    pub const NEW_HEADER_LEN: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    #[inline]
    
    pub unsafe extern "C" fn incomplete_pkt(mut pkt: *const PktHdr) -> bool {
        mbuf_written(&raw const (*pkt).data) != (*pkt).len
    }
    #[inline]
    
    pub unsafe extern "C" fn incomplete_header(mut data: *const MBuf) -> bool {
        let mut avail = mbuf_avail_for_read(data) as uint32_t;
        if avail >= OLD_HEADER_LEN as uint32_t {
            return false_0 != 0;
        }
        if avail < NEW_HEADER_LEN as uint32_t {
            return true_0 != 0;
        }
        *(*data).data.offset((*data).read_pos as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    }
    #[inline]
    
    pub unsafe extern "C" fn pkt_desc(mut pkt: *const PktHdr) -> ::core::ffi::c_char {
        (if (*pkt).type_0 > 256 as ::core::ffi::c_uint {
            '!' as i32 as ::core::ffi::c_uint
        } else {
            (*pkt).type_0
        }) as ::core::ffi::c_char
    }
    use super::_uint32_t_h::uint32_t;

    use super::bouncer_h::{PgPool, PgSocket};
    use super::mbuf_h::{mbuf_avail_for_read, mbuf_written, MBuf};
    use super::stdbool_h::{false_0, true_0};
    extern "C" {
        
        pub fn get_header(data: *mut MBuf, pkt: *mut PktHdr) -> bool;
        
        pub fn log_server_error(note: *const ::core::ffi::c_char, pkt: *mut PktHdr);
        
        pub fn parse_server_error(
            pkt: *mut PktHdr,
            level_p: *mut *const ::core::ffi::c_char,
            msg_p: *mut *const ::core::ffi::c_char,
            sqlstate_p: *mut *const ::core::ffi::c_char,
        );
        
        pub fn add_welcome_parameter(
            pool: *mut PgPool,
            key: *const ::core::ffi::c_char,
            val: *const ::core::ffi::c_char,
        ) -> bool;
        
        pub fn finish_welcome_msg(server: *mut PgSocket);
        
        pub fn answer_authreq(server: *mut PgSocket, pkt: *mut PktHdr) -> bool;
        
        pub fn send_startup_packet(server: *mut PgSocket) -> bool;
        
        pub fn send_sslreq_packet(server: *mut PgSocket) -> bool;
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
    use super::bouncer_h::PgSocket;
    use super::uthash_h::UT_hash_handle;
    extern "C" {
        
        pub fn free_client_prepared_statements(client: *mut PgSocket);
        
        pub fn free_server_prepared_statements(server: *mut PgSocket);
    }
}

pub mod varcache_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct VarCache {
        pub var_list: *mut *mut PStr,
    }
    use super::bouncer_h::PgSocket;
    use super::strpool_h::PStr;
    extern "C" {
        
        pub fn varcache_set(
            cache: *mut VarCache,
            key: *const ::core::ffi::c_char,
            value: *const ::core::ffi::c_char,
        ) -> bool;
        
        pub fn varcache_set_canonical(server: *mut PgSocket, client: *mut PgSocket);
    }
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
    extern "C" {
        
        pub fn pktbuf_static(buf: *mut PktBuf, data: *mut uint8_t, len: ::core::ffi::c_int);
        
        pub fn pktbuf_send_immediate(buf: *mut PktBuf, sk: *mut PgSocket) -> bool;
        
        pub fn pktbuf_write_generic(
            buf: *mut PktBuf,
            type_0: ::core::ffi::c_int,
            fmt: *const ::core::ffi::c_char,
            ...
        );
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

pub mod objects_h {
    use super::bouncer_h::PgSocket;
    extern "C" {
        
        pub type Slab;
        
        pub static mut outstanding_request_cache: *mut Slab;
        
        pub fn release_server(server: *mut PgSocket) -> bool;
        
        pub fn disconnect_server(
            server: *mut PgSocket,
            notify: bool,
            reason: *const ::core::ffi::c_char,
            ...
        );
        
        pub fn disconnect_client(
            client: *mut PgSocket,
            notify: bool,
            reason: *const ::core::ffi::c_char,
            ...
        );
        
        pub fn disconnect_client_sqlstate(
            client: *mut PgSocket,
            notify: bool,
            sqlstate: *const ::core::ffi::c_char,
            reason: *const ::core::ffi::c_char,
        );
        
        pub fn pop_outstanding_request(
            client: *mut PgSocket,
            types: *const ::core::ffi::c_char,
            skip: *mut bool,
        ) -> bool;
        
        pub fn clear_outstanding_requests_until(
            server: *mut PgSocket,
            types: *const ::core::ffi::c_char,
        ) -> bool;
        
        pub fn queue_fake_response(
            client: *mut PgSocket,
            request_type: ::core::ffi::c_char,
        ) -> bool;
        
        pub fn forward_cancel_request(server: *mut PgSocket);
    }
}

pub mod _null_h {
    
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}

pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        
        pub fn memchr(
            __s: *const ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        
        pub fn memcpy(
            __dst: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}

pub mod util_h {
    use super::bouncer_h::PgSocket;
    extern "C" {
        
        pub fn fill_local_addr(sk: *mut PgSocket, fd: ::core::ffi::c_int, is_unix: bool);
    }
}

pub mod client_h {
    use super::bouncer_h::PgSocket;
    use super::proto_h::PktHdr;
    extern "C" {
        
        pub fn handle_auth_query_response(client: *mut PgSocket, pkt: *mut PktHdr) -> bool;
    }
}

pub mod slab_h {
    use super::objects_h::Slab;
    extern "C" {
        
        pub fn slab_free(slab: *mut Slab, obj: *mut ::core::ffi::c_void);
    }
}

pub mod takeover_h {
    use super::bouncer_h::PgSocket;
    extern "C" {
        
        pub fn takeover_login(bouncer: *mut PgSocket) -> bool;
        
        pub fn takeover_login_failed();
    }
}

pub mod _stdlib_h {
    extern "C" {
        
        pub fn exit(_: ::core::ffi::c_int) -> !;
    }
}

pub mod stdbool_h {
    
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}

pub mod protocol_h {
    
    pub const PqMsg_Bind: ::core::ffi::c_int = 'B' as i32;
    
    pub const PqMsg_Close: ::core::ffi::c_int = 'C' as i32;
    
    pub const PqMsg_Describe: ::core::ffi::c_int = 'D' as i32;
    
    pub const PqMsg_Execute: ::core::ffi::c_int = 'E' as i32;
    
    pub const PqMsg_FunctionCall: ::core::ffi::c_int = 'F' as i32;
    
    pub const PqMsg_Parse: ::core::ffi::c_int = 'P' as i32;
    
    pub const PqMsg_Query: ::core::ffi::c_int = 'Q' as i32;
    
    pub const PqMsg_Sync: ::core::ffi::c_int = 'S' as i32;
    
    pub const PqMsg_CopyFail: ::core::ffi::c_int = 'f' as i32;
    
    pub const PqMsg_ParseComplete: ::core::ffi::c_uint = 49 as ::core::ffi::c_uint;
    
    pub const PqMsg_BindComplete: ::core::ffi::c_uint = 50 as ::core::ffi::c_uint;
    
    pub const PqMsg_CloseComplete: ::core::ffi::c_uint = 51 as ::core::ffi::c_uint;
    
    pub const PqMsg_NotificationResponse: ::core::ffi::c_uint = 65 as ::core::ffi::c_uint;
    
    pub const PqMsg_CommandComplete: ::core::ffi::c_uint = 67 as ::core::ffi::c_uint;
    
    pub const PqMsg_DataRow: ::core::ffi::c_uint = 68 as ::core::ffi::c_uint;
    
    pub const PqMsg_ErrorResponse: ::core::ffi::c_uint = 69 as ::core::ffi::c_uint;
    
    pub const PqMsg_CopyInResponse: ::core::ffi::c_uint = 71 as ::core::ffi::c_uint;
    
    pub const PqMsg_CopyOutResponse: ::core::ffi::c_uint = 72 as ::core::ffi::c_uint;
    
    pub const PqMsg_EmptyQueryResponse: ::core::ffi::c_uint = 73 as ::core::ffi::c_uint;
    
    pub const PqMsg_BackendKeyData: ::core::ffi::c_uint = 75 as ::core::ffi::c_uint;
    
    pub const PqMsg_NoticeResponse: ::core::ffi::c_uint = 78 as ::core::ffi::c_uint;
    
    pub const PqMsg_AuthenticationRequest: ::core::ffi::c_uint = 82 as ::core::ffi::c_uint;
    
    pub const PqMsg_ParameterStatus: ::core::ffi::c_uint = 83 as ::core::ffi::c_uint;
    
    pub const PqMsg_RowDescription: ::core::ffi::c_uint = 84 as ::core::ffi::c_uint;
    
    pub const PqMsg_FunctionCallResponse: ::core::ffi::c_uint = 86 as ::core::ffi::c_uint;
    
    pub const PqMsg_CopyBothResponse: ::core::ffi::c_uint = 87 as ::core::ffi::c_uint;
    
    pub const PqMsg_ReadyForQuery: ::core::ffi::c_uint = 90 as ::core::ffi::c_uint;
    
    pub const PqMsg_NoData: ::core::ffi::c_uint = 110 as ::core::ffi::c_uint;
    
    pub const PqMsg_PortalSuspended: ::core::ffi::c_uint = 115 as ::core::ffi::c_uint;
    
    pub const PqMsg_ParameterDescription: ::core::ffi::c_uint = 116 as ::core::ffi::c_uint;
    
    pub const PqMsg_CopyDone: ::core::ffi::c_int = 'c' as i32;
    
    pub const PqMsg_CopyData: ::core::ffi::c_uint = 100 as ::core::ffi::c_uint;
}
pub use self::_in_addr_t_h::in_addr_t;
pub use self::_in_port_t_h::in_port_t;
pub use self::_null_h::NULL;
pub use self::_pid_t_h::pid_t;
pub use self::_ptrdiff_t_h::ptrdiff_t;
pub use self::_sa_family_t_h::sa_family_t;
pub use self::_size_t_h::size_t;
pub use self::_ssize_t_h::ssize_t;
use self::_stdlib_h::exit;
use self::_string_h::{memcpy, strcmp};
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
    cf_default_pool_size, cf_log_connections, cf_max_db_client_connections, cf_max_db_connections,
    cf_max_prepared_statements, cf_max_user_client_connections, cf_max_user_connections,
    cf_min_pool_size, cf_pool_mode, cf_res_pool_size, cf_server_lifetime, pga_is_unix, pga_str,
    sockaddr_ucreds, C2RustUnnamed_9, CallbackState, LoadBalanceHosts, OutstandingRequest,
    PacketCallbackFlag, PgAddr, PgCredentials, PgDatabase, PgGlobalUser, PgPool, PgSocket, PgStats,
    ReplicationType, ResponseAction, SSLMode, ScramState, SocketState, BACKENDKEY_LEN,
    CB_HANDLE_COMPLETE_PACKET, CB_NONE, CB_WANT_COMPLETE_PACKET, CL_ACTIVE, CL_ACTIVE_CANCEL,
    CL_FREE, CL_JUSTFREE, CL_LOGIN, CL_WAITING, CL_WAITING_CANCEL, CL_WAITING_LOGIN,
    LOAD_BALANCE_HOSTS_DISABLE, LOAD_BALANCE_HOSTS_ROUND_ROBIN, POOL_INHERIT, POOL_SESSION,
    POOL_STMT, RA_FAKE, RA_FORWARD, RA_SKIP, REPLICATION_LOGICAL, REPLICATION_NONE,
    REPLICATION_PHYSICAL, SSLMODE_ALLOW, SSLMODE_DISABLED, SSLMODE_PREFER, SSLMODE_REQUIRE,
    SSLMODE_VERIFY_CA, SSLMODE_VERIFY_FULL, SV_ACTIVE, SV_ACTIVE_CANCEL, SV_BEING_CANCELED,
    SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN, SV_TESTED, SV_USED,
};
use self::client_h::handle_auth_query_response;
pub use self::cryptohash_h::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};

pub use self::event_struct_h::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
pub use self::in6_h::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use self::in_h::{in_addr, sockaddr_in};
pub use self::iobuf_h::{iobuf, IOBuf};
pub use self::list_h::{list_del, list_empty, list_pop, List};
pub use self::logging_h::{
    cf_verbose, log_fatal, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE,
    LG_STATS, LG_WARNING,
};
pub use self::mbuf_h::{
    mbuf_avail_for_read, mbuf_get_byte, mbuf_get_bytes, mbuf_get_char, mbuf_get_string,
    mbuf_written, MBuf,
};
use self::objects_h::{
    clear_outstanding_requests_until, disconnect_client, disconnect_client_sqlstate,
    disconnect_server, forward_cancel_request, outstanding_request_cache, pop_outstanding_request,
    queue_fake_response, release_server,
};
pub use self::pktbuf_h::{pktbuf_send_immediate, pktbuf_static, pktbuf_write_generic, PktBuf};
pub use self::prepare_h::{
    free_client_prepared_statements, free_server_prepared_statements, PgClientPreparedStatement,
    PgPreparedStatement, PgServerPreparedStatement,
};
pub use self::proto_h::{
    add_welcome_parameter, answer_authreq, finish_welcome_msg, get_header, incomplete_header,
    incomplete_pkt, log_server_error, parse_server_error, pkt_desc, send_sslreq_packet,
    send_startup_packet, PktHdr, NEW_HEADER_LEN, OLD_HEADER_LEN,
};
pub use self::protocol_h::{
    PqMsg_AuthenticationRequest, PqMsg_BackendKeyData, PqMsg_Bind, PqMsg_BindComplete, PqMsg_Close,
    PqMsg_CloseComplete, PqMsg_CommandComplete, PqMsg_CopyBothResponse, PqMsg_CopyData,
    PqMsg_CopyDone, PqMsg_CopyFail, PqMsg_CopyInResponse, PqMsg_CopyOutResponse, PqMsg_DataRow,
    PqMsg_Describe, PqMsg_EmptyQueryResponse, PqMsg_ErrorResponse, PqMsg_Execute,
    PqMsg_FunctionCall, PqMsg_FunctionCallResponse, PqMsg_NoData, PqMsg_NoticeResponse,
    PqMsg_NotificationResponse, PqMsg_ParameterDescription, PqMsg_ParameterStatus, PqMsg_Parse,
    PqMsg_ParseComplete, PqMsg_PortalSuspended, PqMsg_Query, PqMsg_ReadyForQuery,
    PqMsg_RowDescription, PqMsg_Sync,
};
pub use self::sbuf_h::{
    sbuf_cb_t, sbuf_continue, sbuf_prepare_send, sbuf_prepare_skip, sbuf_tls_connect,
    server_connect_sslmode, SBuf, SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK,
    SBUF_EV_FLUSH, SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY,
};
use self::slab_h::slab_free;
pub use self::socket_h::{sockaddr, AF_UNIX};
pub use self::statlist_h::{statlist_count, statlist_empty, statlist_pop, StatList};
pub use self::stdbool_h::{false_0, true_0};
pub use self::strpool_h::{PStr, StrPool};
pub use self::sys__types_h::{__darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t, __DARWIN_NULL};
use self::takeover_h::{takeover_login, takeover_login_failed};
pub use self::time_h::{get_cached_time, usec_t};
use self::tls_h::tls_get_connection_info;
pub use self::uthash_h::{UT_hash_bucket, UT_hash_handle, UT_hash_table};
use self::util_h::fill_local_addr;
pub use self::varcache_h::{varcache_set, varcache_set_canonical, VarCache};

pub const ERRCODE_CANNOT_CONNECT_NOW: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"57P03\0") };

unsafe extern "C" fn load_parameter(
    mut server: *mut PgSocket,
    mut pkt: *mut PktHdr,
    mut startup: bool,
) -> bool {
    let mut key = ::core::ptr::null::<::core::ffi::c_char>();
    let mut val = ::core::ptr::null::<::core::ffi::c_char>();
    let mut client = (*server).link;
    if incomplete_pkt(pkt) {
        return false_0 != 0;
    }
    if mbuf_get_string(&raw mut (*pkt).data, &raw mut key)
        && mbuf_get_string(&raw mut (*pkt).data, &raw mut val)
    {
        if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_DEBUG,
                server as *mut ::core::ffi::c_void,
                b"S: param: %s = %s\0" as *const u8 as *const ::core::ffi::c_char,
                key,
                val,
            );
        }
        varcache_set(&raw mut (*server).vars, key, val);
        if !client.is_null() {
            if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_DEBUG,
                    client as *mut ::core::ffi::c_void,
                    b"setting client var: %s='%s'\0" as *const u8 as *const ::core::ffi::c_char,
                    key,
                    val,
                );
            }
            varcache_set(&raw mut (*client).vars, key, val);
        }
        if startup && !add_welcome_parameter((*server).pool, key, val) {
            disconnect_server(
                server,
                true_0 != 0,
                b"failed to store ParameterStatus\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
        return true_0 != 0;
    }
    disconnect_server(
        server,
        true_0 != 0,
        b"broken ParameterStatus packet\0" as *const u8 as *const ::core::ffi::c_char,
    );
    false_0 != 0
}
#[no_mangle]

pub unsafe extern "C" fn kill_pool_logins(
    mut pool: *mut PgPool,
    mut sqlstate: *const ::core::ffi::c_char,
    mut msg: *const ::core::ffi::c_char,
) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut client = ::core::ptr::null_mut::<PgSocket>();
    if statlist_count(&raw mut (*pool).active_server_list)
        + statlist_count(&raw mut (*pool).being_canceled_server_list)
        + statlist_count(&raw mut (*pool).idle_server_list)
        + statlist_count(&raw mut (*pool).tested_server_list)
        + statlist_count(&raw mut (*pool).used_server_list)
        != 0 as ::core::ffi::c_int
        && (*pool).welcome_msg_ready() as ::core::ffi::c_int != 0
    {
        return;
    }
    item = (*pool).waiting_client_list.head.next;
    tmp = (*(*pool).waiting_client_list.head.next).next;
    while item != &raw mut (*pool).waiting_client_list.head {
        client = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgSocket;
        disconnect_client_sqlstate(client, true_0 != 0, sqlstate, msg);
        item = tmp;
        tmp = (*tmp).next;
    }
}
#[no_mangle]

pub unsafe extern "C" fn kill_pool_logins_server_error(
    mut pool: *mut PgPool,
    mut errpkt: *mut PktHdr,
) -> *const ::core::ffi::c_char {
    let mut level = ::core::ptr::null::<::core::ffi::c_char>();
    let mut sqlstate = ::core::ptr::null::<::core::ffi::c_char>();
    let mut msg = ::core::ptr::null::<::core::ffi::c_char>();
    parse_server_error(errpkt, &raw mut level, &raw mut msg, &raw mut sqlstate);
    let mut _log_ctx = NULL;
    log_generic(
        LG_WARNING,
        _log_ctx,
        b"server login failed: %s %s\0" as *const u8 as *const ::core::ffi::c_char,
        level,
        msg,
    );
    if strcmp(sqlstate, ERRCODE_CANNOT_CONNECT_NOW.as_ptr()) != 0 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                _log_ctx_0,
                b"kill_pool_logins_server_error: sqlstate: %s\0" as *const u8
                    as *const ::core::ffi::c_char,
                sqlstate,
            );
        }
        kill_pool_logins(pool, sqlstate, msg);
    }
    msg
}

unsafe extern "C" fn handle_server_startup(
    mut server: *mut PgSocket,
    mut pkt: *mut PktHdr,
) -> bool {
    let mut sbuf: *mut SBuf = &raw mut (*server).sbuf;
    let mut msg = ::core::ptr::null::<::core::ffi::c_char>();
    let mut res = false_0 != 0;
    let mut ckey = ::core::ptr::null::<uint8_t>();
    if incomplete_pkt(pkt) {
        disconnect_server(
            server,
            true_0 != 0,
            b"partial pkt in login phase\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    if (*server).exec_on_connect() {
        's_46: {
            match (*pkt).type_0 {
                90 | 83 => {
                    break 's_46;
                }
                69 => {
                    log_server_error(
                        b"S: error while executing exec_on_query\0" as *const u8
                            as *const ::core::ffi::c_char,
                        pkt,
                    );
                }
                _ => {}
            }
            sbuf_prepare_skip(sbuf, (*pkt).len);
            return true_0 != 0;
        }
    }
    let mut current_block_60: u64;
    match (*pkt).type_0 {
        69 => {
            if (*server).replication as u64 == 0 {
                msg = kill_pool_logins_server_error((*server).pool, pkt);
                disconnect_server(
                    server,
                    true_0 != 0,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    msg as *mut ::core::ffi::c_char,
                );
            } else {
                log_server_error(
                    b"S: login failed\0" as *const u8 as *const ::core::ffi::c_char,
                    pkt,
                );
                disconnect_server(
                    server,
                    true_0 != 0,
                    b"login failed\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
        82 => {
            if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_DEBUG,
                    server as *mut ::core::ffi::c_void,
                    b"calling login_answer\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            res = answer_authreq(server, pkt);
            if !res {
                disconnect_server(
                    server,
                    false_0 != 0,
                    b"failed to answer authreq\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
        83 => {
            res = load_parameter(server, pkt, true_0 != 0);
        }
        90 => {
            if (*server).exec_on_connect() {
                (*server).set_exec_on_connect(false_0 != 0);
                current_block_60 = 18435049525520518667;
            } else if !(*(*(*server).pool).db).connect_query.is_null() {
                (*server).set_exec_on_connect(true_0 != 0);
                if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0
                {
                    log_generic(
                        LG_DEBUG,
                        server as *mut ::core::ffi::c_void,
                        b"server connect ok, send exec_on_connect\0" as *const u8
                            as *const ::core::ffi::c_char,
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
                    PqMsg_Query,
                    b"s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*(*(*server).pool).db).connect_query,
                );
                res = pktbuf_send_immediate(&raw mut _buf, server);
                if !res {
                    disconnect_server(
                        server,
                        false_0 != 0,
                        b"exec_on_connect query failed\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                current_block_60 = 6721012065216013753;
            } else {
                current_block_60 = 18435049525520518667;
            }
            match current_block_60 {
                6721012065216013753 => {}
                _ => {
                    if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        as ::core::ffi::c_long
                        != 0
                    {
                        log_generic(
                            LG_DEBUG,
                            server as *mut ::core::ffi::c_void,
                            b"server login ok, start accepting queries\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    (*server).set_ready(true_0 != 0);
                    finish_welcome_msg(server);
                    res = release_server(server);
                    if res as ::core::ffi::c_int != 0
                        && (*(*(*server).pool).db).admin as ::core::ffi::c_int != 0
                    {
                        res = takeover_login(server);
                    }
                }
            }
        }
        75 => {
            if !mbuf_get_bytes(
                &raw mut (*pkt).data,
                BACKENDKEY_LEN as ::core::ffi::c_uint,
                &raw mut ckey,
            ) {
                disconnect_server(
                    server,
                    true_0 != 0,
                    b"bad cancel key\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return false_0 != 0;
            }
            memcpy(
                &raw mut (*server).cancel_key as *mut uint8_t as *mut ::core::ffi::c_void,
                ckey as *const ::core::ffi::c_void,
                BACKENDKEY_LEN as size_t,
            );
            res = true_0 != 0;
        }
        78 => {
            if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_NOISE,
                    server as *mut ::core::ffi::c_void,
                    b"skipping pkt: %c\0" as *const u8 as *const ::core::ffi::c_char,
                    pkt_desc(pkt) as ::core::ffi::c_int,
                );
            }
            res = true_0 != 0;
        }
        _ => {
            log_generic(
                LG_ERROR,
                server as *mut ::core::ffi::c_void,
                b"unknown pkt from server: '%c'\0" as *const u8 as *const ::core::ffi::c_char,
                pkt_desc(pkt) as ::core::ffi::c_int,
            );
            disconnect_server(
                server,
                true_0 != 0,
                b"unknown pkt from server\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    if res {
        sbuf_prepare_skip(sbuf, (*pkt).len);
    }
    res
}
#[no_mangle]

pub unsafe extern "C" fn connection_pool_mode(mut connection: *mut PgSocket) -> ::core::ffi::c_int {
    if (*connection).replication as u64 != 0 {
        return POOL_SESSION;
    }
    probably_wrong_pool_pool_mode((*connection).pool)
}
#[no_mangle]

pub unsafe extern "C" fn probably_wrong_pool_pool_mode(
    mut pool: *mut PgPool,
) -> ::core::ffi::c_int {
    let mut pool_mode = (*(*(*pool).user_credentials).global_user).pool_mode;
    if pool_mode == POOL_INHERIT {
        pool_mode = (*(*pool).db).pool_mode;
    }
    if pool_mode == POOL_INHERIT {
        pool_mode = cf_pool_mode;
    }
    pool_mode
}
#[no_mangle]

pub unsafe extern "C" fn pool_pool_size(mut pool: *mut PgPool) -> ::core::ffi::c_int {
    let mut user_pool_size = if !(*pool).user_credentials.is_null() {
        (*(*(*pool).user_credentials).global_user).pool_size
    } else {
        -(1 as ::core::ffi::c_int)
    };
    if user_pool_size >= 0 as ::core::ffi::c_int {
        user_pool_size
    } else if (*(*pool).db).pool_size >= 0 as ::core::ffi::c_int {
        (*(*pool).db).pool_size
    } else {
        cf_default_pool_size
    }
}
#[no_mangle]

pub unsafe extern "C" fn pool_min_pool_size(mut pool: *mut PgPool) -> ::core::ffi::c_int {
    database_min_pool_size((*pool).db)
}
#[no_mangle]

pub unsafe extern "C" fn pool_server_lifetime(mut pool: *mut PgPool) -> usec_t {
    if (*(*pool).db).server_lifetime == 0 as usec_t {
        cf_server_lifetime
    } else {
        (*(*pool).db).server_lifetime
    }
}
#[no_mangle]

pub unsafe extern "C" fn database_min_pool_size(mut db: *mut PgDatabase) -> ::core::ffi::c_int {
    if (*db).min_pool_size < 0 as ::core::ffi::c_int {
        cf_min_pool_size
    } else {
        (*db).min_pool_size
    }
}
#[no_mangle]

pub unsafe extern "C" fn pool_res_pool_size(mut pool: *mut PgPool) -> ::core::ffi::c_int {
    let mut user_res_pool_size = if !(*pool).user_credentials.is_null() {
        (*(*(*pool).user_credentials).global_user).res_pool_size
    } else {
        -(1 as ::core::ffi::c_int)
    };
    if user_res_pool_size >= 0 as ::core::ffi::c_int {
        user_res_pool_size
    } else if (*(*pool).db).res_pool_size >= 0 as ::core::ffi::c_int {
        (*(*pool).db).res_pool_size
    } else {
        cf_res_pool_size
    }
}
#[no_mangle]

pub unsafe extern "C" fn database_max_client_connections(
    mut db: *mut PgDatabase,
) -> ::core::ffi::c_int {
    if (*db).max_db_client_connections <= 0 as ::core::ffi::c_int {
        cf_max_db_client_connections
    } else {
        (*db).max_db_client_connections
    }
}
#[no_mangle]

pub unsafe extern "C" fn database_max_connections(mut db: *mut PgDatabase) -> ::core::ffi::c_int {
    if (*db).max_db_connections <= 0 as ::core::ffi::c_int {
        cf_max_db_connections
    } else {
        (*db).max_db_connections
    }
}
#[no_mangle]

pub unsafe extern "C" fn user_max_connections(mut user: *mut PgGlobalUser) -> ::core::ffi::c_int {
    if (*user).max_user_connections <= 0 as ::core::ffi::c_int {
        cf_max_user_connections
    } else {
        (*user).max_user_connections
    }
}
#[no_mangle]

pub unsafe extern "C" fn user_client_max_connections(
    mut user: *mut PgGlobalUser,
) -> ::core::ffi::c_int {
    if (*user).max_user_client_connections <= 0 as ::core::ffi::c_int {
        cf_max_user_client_connections
    } else {
        (*user).max_user_client_connections
    }
}

unsafe extern "C" fn handle_server_work(mut server: *mut PgSocket, mut pkt: *mut PktHdr) -> bool {
    let mut ready = false_0 != 0;
    let mut idle_tx = false_0 != 0;
    let mut state: ::core::ffi::c_char = 0;
    let mut sbuf: *mut SBuf = &raw mut (*server).sbuf;
    let mut client = (*server).link;
    let mut async_response = false_0 != 0;
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut ignore_packet = false_0 != 0;
    match (*pkt).type_0 {
        90 => {
            if !mbuf_get_char(&raw mut (*pkt).data, &raw mut state) {
                return false_0 != 0;
            }
            let mut fresh2: [::core::ffi::c_char; 4] = [
                PqMsg_Sync as ::core::ffi::c_char,
                PqMsg_Query as ::core::ffi::c_char,
                PqMsg_FunctionCall as ::core::ffi::c_char,
                '\0' as i32 as ::core::ffi::c_char,
            ];
            if !pop_outstanding_request(
                server,
                &raw mut fresh2 as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
                &raw mut ignore_packet,
            ) && (*server).query_failed() as ::core::ffi::c_int != 0
            {
                let mut fresh3: [::core::ffi::c_char; 2] = [
                    PqMsg_Sync as ::core::ffi::c_char,
                    '\0' as i32 as ::core::ffi::c_char,
                ];
                if !clear_outstanding_requests_until(
                    server,
                    &raw mut fresh3 as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
                ) {
                    return false_0 != 0;
                }
            }
            (*server).set_query_failed(false_0 != 0);
            if state as ::core::ffi::c_int == 'I' as i32 {
                ready = true_0 != 0;
            } else if connection_pool_mode(server) == POOL_STMT {
                disconnect_server(
                    server,
                    true_0 != 0,
                    b"transaction blocks not allowed in statement pooling mode\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return false_0 != 0;
            } else if state as ::core::ffi::c_int == 'T' as i32
                || state as ::core::ffi::c_int == 'E' as i32
            {
                idle_tx = true_0 != 0;
            }
        }
        83 => {
            if !load_parameter(server, pkt, false_0 != 0) {
                return false_0 != 0;
            }
        }
        69 => {
            if (*server).setting_vars() {
                log_server_error(
                    b"varcache_apply failed\0" as *const u8 as *const ::core::ffi::c_char,
                    pkt,
                );
                disconnect_server(
                    server,
                    true_0 != 0,
                    b"invalid server parameter\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return false_0 != 0;
            }
            if (*server).copy_mode() {
                if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0
                {
                    log_generic(
                        LG_DEBUG,
                        server as *mut ::core::ffi::c_void,
                        b"COPY failed\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                (*server).set_copy_mode(false_0 != 0);
                let mut fresh4: [::core::ffi::c_char; 3] = [
                    PqMsg_CopyDone as ::core::ffi::c_char,
                    PqMsg_CopyFail as ::core::ffi::c_char,
                    '\0' as i32 as ::core::ffi::c_char,
                ];
                if !clear_outstanding_requests_until(
                    server,
                    &raw mut fresh4 as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
                ) {
                    return false_0 != 0;
                }
            }
            (*server).set_query_failed(true_0 != 0);
        }
        67 => {
            if (*server).copy_mode() {
                if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0
                {
                    log_generic(
                        LG_DEBUG,
                        server as *mut ::core::ffi::c_void,
                        b"COPY finished\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                (*server).set_copy_mode(false_0 != 0);
                let mut fresh5: [::core::ffi::c_char; 2] = [
                    PqMsg_CopyDone as ::core::ffi::c_char,
                    '\0' as i32 as ::core::ffi::c_char,
                ];
                if !clear_outstanding_requests_until(
                    server,
                    &raw mut fresh5 as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
                ) {
                    return false_0 != 0;
                }
            }
            if connection_pool_mode(server) != POOL_SESSION
                && cf_max_prepared_statements != 0 as ::core::ffi::c_int
                && ((*pkt).len
                    == (1 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 15 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    || (*pkt).len
                        == (1 as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            + 12 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint)
            {
                let mut tag = ::core::ptr::null::<::core::ffi::c_char>();
                if mbuf_get_string(&raw mut (*pkt).data, &raw mut tag) {
                    if strcmp(
                        tag,
                        b"DEALLOCATE ALL\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                        || strcmp(
                            tag,
                            b"DISCARD ALL\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    {
                        free_server_prepared_statements(server);
                        if !client.is_null() {
                            free_client_prepared_statements(client);
                        }
                    }
                } else {
                    return false_0 != 0;
                }
            }
            let mut fresh6: [::core::ffi::c_char; 2] = [
                PqMsg_Execute as ::core::ffi::c_char,
                '\0' as i32 as ::core::ffi::c_char,
            ];
            pop_outstanding_request(
                server,
                &raw mut fresh6 as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
                &raw mut ignore_packet,
            );
        }
        65 => {
            idle_tx = (*server).idle_tx();
            ready = (*server).ready();
            async_response = true_0 != 0;
        }
        71 | 87 => {
            if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_DEBUG,
                    server as *mut ::core::ffi::c_void,
                    b"COPY started\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            (*server).set_copy_mode(true_0 != 0);
        }
        49 => {
            let mut fresh7: [::core::ffi::c_char; 2] = [
                PqMsg_Parse as ::core::ffi::c_char,
                '\0' as i32 as ::core::ffi::c_char,
            ];
            pop_outstanding_request(
                server,
                &raw mut fresh7 as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
                &raw mut ignore_packet,
            );
        }
        50 => {
            let mut fresh8: [::core::ffi::c_char; 2] = [
                PqMsg_Bind as ::core::ffi::c_char,
                '\0' as i32 as ::core::ffi::c_char,
            ];
            pop_outstanding_request(
                server,
                &raw mut fresh8 as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
                &raw mut ignore_packet,
            );
        }
        51 => {
            let mut fresh9: [::core::ffi::c_char; 2] = [
                PqMsg_Close as ::core::ffi::c_char,
                '\0' as i32 as ::core::ffi::c_char,
            ];
            pop_outstanding_request(
                server,
                &raw mut fresh9 as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
                &raw mut ignore_packet,
            );
        }
        110 | 84 => {
            let mut fresh10: [::core::ffi::c_char; 2] = [
                PqMsg_Describe as ::core::ffi::c_char,
                '\0' as i32 as ::core::ffi::c_char,
            ];
            pop_outstanding_request(
                server,
                &raw mut fresh10 as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
                &raw mut ignore_packet,
            );
        }
        73 | 115 => {
            let mut fresh11: [::core::ffi::c_char; 2] = [
                PqMsg_Execute as ::core::ffi::c_char,
                '\0' as i32 as ::core::ffi::c_char,
            ];
            pop_outstanding_request(
                server,
                &raw mut fresh11 as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
                &raw mut ignore_packet,
            );
        }
        78 | 72 | 116 | 99 | 102 | 86 | 100 | 68 => {}
        _ => {
            log_generic(
                LG_ERROR,
                server as *mut ::core::ffi::c_void,
                b"unknown pkt: '%c'\0" as *const u8 as *const ::core::ffi::c_char,
                pkt_desc(pkt) as ::core::ffi::c_int,
            );
            disconnect_server(
                server,
                true_0 != 0,
                b"unknown pkt\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
    }
    (*server).set_idle_tx(idle_tx);
    (*server).set_ready(ready);
    (*(*server).pool).stats.server_bytes = (*(*server).pool)
        .stats
        .server_bytes
        .wrapping_add((*pkt).len as uint64_t);
    if (*server).setting_vars() {
        sbuf_prepare_skip(sbuf, (*pkt).len);
    } else if !client.is_null() {
        if (*client).state() as ::core::ffi::c_int == CL_LOGIN as ::core::ffi::c_int {
            return handle_auth_query_response(client, pkt);
        } else if ignore_packet {
            if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_NOISE,
                    server as *mut ::core::ffi::c_void,
                    b"not forwarding packet with type '%c' from server\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*pkt).type_0,
                );
            }
            sbuf_prepare_skip(sbuf, (*pkt).len);
        } else {
            sbuf_prepare_send(sbuf, &raw mut (*client).sbuf, (*pkt).len);
            if statlist_count(&raw mut (*server).outstanding_requests) == 0 as ::core::ffi::c_int {
                if ready as ::core::ffi::c_int != 0 || idle_tx as ::core::ffi::c_int != 0 {
                    if (*client).query_start != 0 {
                        let mut total: usec_t = 0;
                        total = get_cached_time().wrapping_sub((*client).query_start);
                        (*client).query_start = 0 as usec_t;
                        (*(*server).pool).stats.query_time =
                            (*(*server).pool).stats.query_time.wrapping_add(total);
                        if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            log_generic(
                                LG_DEBUG,
                                client as *mut ::core::ffi::c_void,
                                b"query time: %d us\0" as *const u8 as *const ::core::ffi::c_char,
                                total as ::core::ffi::c_int,
                            );
                        }
                    } else if !async_response {
                        log_generic(
                            LG_WARNING,
                            client as *mut ::core::ffi::c_void,
                            b"FIXME: query end, but query_start == 0\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                }
                if ready {
                    if (*client).xact_start != 0 {
                        let mut total_0: usec_t = 0;
                        total_0 = get_cached_time().wrapping_sub((*client).xact_start);
                        (*client).xact_start = 0 as usec_t;
                        (*(*server).pool).stats.xact_time =
                            (*(*server).pool).stats.xact_time.wrapping_add(total_0);
                        if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            log_generic(
                                LG_DEBUG,
                                client as *mut ::core::ffi::c_void,
                                b"transaction time: %d us\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                total_0 as ::core::ffi::c_int,
                            );
                        }
                    } else if !async_response {
                        log_generic(
                            LG_WARNING,
                            client as *mut ::core::ffi::c_void,
                            b"FIXME: transaction end, but xact_start == 0\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                }
            }
            item = (*server).outstanding_requests.head.next;
            tmp = (*(*server).outstanding_requests.head.next).next;
            while item != &raw mut (*server).outstanding_requests.head {
                let mut request = (item as *mut ::core::ffi::c_char)
                    .offset(-(0 as ::core::ffi::c_ulong as isize))
                    as *mut OutstandingRequest;
                if (*request).action as ::core::ffi::c_uint
                    != RA_FAKE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    break;
                }
                statlist_pop(&raw mut (*server).outstanding_requests);
                (*sbuf).extra_packet_queue_after = true_0 != 0;
                if !queue_fake_response(client, (*request).type_0) {
                    disconnect_client(
                        client,
                        true_0 != 0,
                        b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    disconnect_server(
                        (*client).link,
                        true_0 != 0,
                        b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    return false_0 != 0;
                }
                slab_free(
                    outstanding_request_cache,
                    request as *mut ::core::ffi::c_void,
                );
                item = tmp;
                tmp = (*tmp).next;
            }
        }
    } else {
        if (*server).state() as ::core::ffi::c_int != SV_TESTED as ::core::ffi::c_int {
            log_generic(
                LG_WARNING,
                server as *mut ::core::ffi::c_void,
                b"got packet '%c' from server when not linked\0" as *const u8
                    as *const ::core::ffi::c_char,
                pkt_desc(pkt) as ::core::ffi::c_int,
            );
        }
        sbuf_prepare_skip(sbuf, (*pkt).len);
    }
    true_0 != 0
}

unsafe extern "C" fn handle_connect(mut server: *mut PgSocket) -> bool {
    let mut res = false_0 != 0;
    let mut pool = (*server).pool;
    let mut buf: [::core::ffi::c_char; 88] = [0; 88];
    let mut is_unix = pga_is_unix(&raw mut (*server).remote_addr);
    fill_local_addr(server, (*server).sbuf.sock, is_unix);
    if cf_log_connections != 0 {
        if pga_is_unix(&raw mut (*server).remote_addr) {
            log_generic(
                LG_INFO,
                server as *mut ::core::ffi::c_void,
                b"new connection to server\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            log_generic(
                LG_INFO,
                server as *mut ::core::ffi::c_void,
                b"new connection to server (from %s)\0" as *const u8 as *const ::core::ffi::c_char,
                pga_str(
                    &raw mut (*server).local_addr,
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 88]>() as ::core::ffi::c_int,
                ),
            );
        }
    }
    if !statlist_empty(&raw mut (*pool).waiting_cancel_req_list) {
        if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_DEBUG,
                server as *mut ::core::ffi::c_void,
                b"use it for pending cancel req\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        forward_cancel_request(server);
    } else if (*(*pool).db).peer_id != 0 {
        (*server).set_ready(true_0 != 0);
        disconnect_server(
            server,
            false_0 != 0,
            b"peer server was not necessary anymore, because client cancel connection was already closed\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        if server_connect_sslmode > SSLMODE_DISABLED as ::core::ffi::c_int && !is_unix {
            if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_NOISE,
                    server as *mut ::core::ffi::c_void,
                    b"P: SSL request\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            res = send_sslreq_packet(server);
            if res {
                (*server).set_wait_sslchar(true_0 != 0);
            }
        } else {
            if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_NOISE,
                    server as *mut ::core::ffi::c_void,
                    b"P: startup\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            res = send_startup_packet(server);
        }
        if !res {
            disconnect_server(
                server,
                false_0 != 0,
                b"startup pkt failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    res
}

unsafe extern "C" fn handle_sslchar(mut server: *mut PgSocket, mut data: *mut MBuf) -> bool {
    let mut schar: uint8_t = '?' as i32 as uint8_t;
    let mut ok: bool = false;
    (*server).set_wait_sslchar(false_0 != 0);
    ok = mbuf_get_byte(data, &raw mut schar);
    if !ok || schar as ::core::ffi::c_int != 'S' as i32 && schar as ::core::ffi::c_int != 'N' as i32
    {
        disconnect_server(
            server,
            false_0 != 0,
            b"bad sslreq answer\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    if mbuf_avail_for_read(data) != 0 as ::core::ffi::c_uint {
        disconnect_server(
            server,
            false_0 != 0,
            b"received unencrypted data after SSL response\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    if schar as ::core::ffi::c_int == 'S' as i32 {
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                server as *mut ::core::ffi::c_void,
                b"launching tls\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        ok = sbuf_tls_connect(&raw mut (*server).sbuf, (*server).host);
    } else if server_connect_sslmode >= SSLMODE_REQUIRE as ::core::ffi::c_int {
        disconnect_server(
            server,
            false_0 != 0,
            b"server refused SSL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return false_0 != 0;
    } else {
        ok = send_startup_packet(server);
    }
    if ok {
        sbuf_prepare_skip(&raw mut (*server).sbuf, 1 as ::core::ffi::c_uint);
    } else {
        disconnect_server(
            server,
            false_0 != 0,
            b"sslreq processing failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    ok
}
#[no_mangle]

pub unsafe extern "C" fn server_proto(
    mut sbuf: *mut SBuf,
    mut evtype: SBufEvent,
    mut data: *mut MBuf,
) -> bool {
    let mut res = false_0 != 0;
    let mut server = (sbuf as *mut ::core::ffi::c_char)
        .offset(-(520 as ::core::ffi::c_ulong as isize)) as *mut PgSocket;
    let mut pool = (*server).pool;
    let mut pkt = PktHdr {
        type_0: 0,
        len: 0,
        data: MBuf {
            data: ::core::ptr::null_mut::<uint8_t>(),
            read_pos: 0,
            write_pos: 0,
            alloc_len: 0,
            reader: false,
            fixed: false,
        },
    };
    let mut infobuf: [::core::ffi::c_char; 96] = [0; 96];
    if (*server).state() as ::core::ffi::c_int == SV_JUSTFREE as ::core::ffi::c_int {
        return false_0 != 0;
    }
    match evtype as ::core::ffi::c_uint {
        1 => {
            if (*server).state() as ::core::ffi::c_int == SV_ACTIVE_CANCEL as ::core::ffi::c_int {
                disconnect_server(
                    server,
                    false_0 != 0,
                    b"successfully sent cancel request\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            } else {
                disconnect_server(
                    server,
                    false_0 != 0,
                    b"server conn crashed?\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
        2 => {
            disconnect_client(
                (*server).link,
                false_0 != 0,
                b"unexpected eof\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        0 => {
            if (*server).wait_sslchar() {
                res = handle_sslchar(server, data);
            } else if incomplete_header(data) {
                if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0
                {
                    log_generic(
                        LG_NOISE,
                        server as *mut ::core::ffi::c_void,
                        b"S: got partial header, trying to wait a bit\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else if !get_header(data, &raw mut pkt) {
                disconnect_server(
                    server,
                    true_0 != 0,
                    b"bad pkt header\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
                if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0
                {
                    log_generic(
                        LG_NOISE,
                        server as *mut ::core::ffi::c_void,
                        b"read pkt='%c', len=%u\0" as *const u8 as *const ::core::ffi::c_char,
                        pkt_desc(&raw mut pkt) as ::core::ffi::c_int,
                        pkt.len,
                    );
                }
                (*server).request_time = get_cached_time();
                match (*server).state() as ::core::ffi::c_int {
                    10 => {
                        res = handle_server_startup(server, &raw mut pkt);
                    }
                    11..=16 => {
                        res = handle_server_work(server, &raw mut pkt);
                    }
                    _ => {
                        let mut _log_ctx = NULL;
                        log_fatal(
                            b"src/server.c\0" as *const u8 as *const ::core::ffi::c_char,
                            801 as ::core::ffi::c_int,
                            b"server_proto\0" as *const u8 as *const ::core::ffi::c_char,
                            false_0 != 0,
                            _log_ctx,
                            b"server_proto: server in bad state: %d\0" as *const u8
                                as *const ::core::ffi::c_char,
                            (*server).state() as ::core::ffi::c_int,
                        );
                        exit(1 as ::core::ffi::c_int);
                    }
                }
            }
        }
        3 => {
            disconnect_server(
                server,
                false_0 != 0,
                b"connect failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        4 => {
            if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_DEBUG,
                    server as *mut ::core::ffi::c_void,
                    b"S: connect ok\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            (*server).request_time = get_cached_time();
            res = handle_connect(server);
        }
        5 => {
            res = true_0 != 0;
            if (*server).ready() {
                if (*server).setting_vars() {
                    let mut client = (*server).link;
                    varcache_set_canonical(server, client);
                    (*server).set_setting_vars(false_0 != 0);
                    let mut _log_ctx_0 = NULL;
                    if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                        as ::core::ffi::c_long
                        != 0
                    {
                        log_generic(
                            LG_NOISE,
                            _log_ctx_0,
                            b"done setting vars unpausing client\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    sbuf_continue(&raw mut (*client).sbuf);
                } else if connection_pool_mode(server) != POOL_SESSION
                    || (*server).state() as ::core::ffi::c_int == SV_TESTED as ::core::ffi::c_int
                    || (*server).resetting() as ::core::ffi::c_int != 0
                {
                    (*server).set_resetting(false_0 != 0);
                    let mut current_block_46: u64;
                    match (*server).state() as ::core::ffi::c_int {
                        13 | 14 | 16 => {
                            if !(*server).link.is_null() {
                                if statlist_count(&raw mut (*server).outstanding_requests)
                                    > 0 as ::core::ffi::c_int
                                {
                                    current_block_46 = 11441799814184323368;
                                } else {
                                    current_block_46 = 16415152177862271243;
                                }
                            } else {
                                current_block_46 = 16415152177862271243;
                            }
                            match current_block_46 {
                                11441799814184323368 => {}
                                _ => {
                                    release_server(server);
                                }
                            }
                        }
                        11 | 12 => {}
                        _ => {
                            log_generic(
                                LG_WARNING,
                                server as *mut ::core::ffi::c_void,
                                b"EV_FLUSH with state=%d\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*server).state() as ::core::ffi::c_int,
                            );
                        }
                    }
                }
            }
        }
        6 => {
            log_generic(
                LG_WARNING,
                server as *mut ::core::ffi::c_void,
                b"SBUF_EV_PKT_CALLBACK with state=%d\0" as *const u8 as *const ::core::ffi::c_char,
                (*server).state() as ::core::ffi::c_int,
            );
        }
        7 => {
            tls_get_connection_info(
                (*server).sbuf.tls,
                &raw mut infobuf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 96]>() as size_t,
            );
            if cf_log_connections != 0 {
                log_generic(
                    LG_INFO,
                    server as *mut ::core::ffi::c_void,
                    b"SSL established: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut infobuf as *mut ::core::ffi::c_char,
                );
            } else if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_NOISE,
                    server as *mut ::core::ffi::c_void,
                    b"SSL established: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut infobuf as *mut ::core::ffi::c_char,
                );
            }
            (*server).request_time = get_cached_time();
            res = send_startup_packet(server);
            if res {
                sbuf_continue(&raw mut (*server).sbuf);
            } else {
                disconnect_server(
                    server,
                    false_0 != 0,
                    b"TLS startup failed\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
        _ => {}
    }
    if !res && (*(*pool).db).admin as ::core::ffi::c_int != 0 {
        takeover_login_failed();
    }
    res
}
