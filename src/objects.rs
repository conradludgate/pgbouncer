pub mod internal {

    pub type __builtin_va_list = *mut ::core::ffi::c_char;
}

pub mod sys__types_h {

    pub type __darwin_pid_t = __int32_t;

    pub type __darwin_suseconds_t = __int32_t;

    pub type __darwin_uid_t = __uint32_t;

    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    use crate::types::{__int32_t, __uint32_t};
}

pub mod _time_t_h {

    pub type time_t = __darwin_time_t;
    use crate::types::__darwin_time_t;
}

pub mod _va_list_h {

    pub type va_list = __darwin_va_list;
    use crate::types::__darwin_va_list;
}

pub mod tls_h {
    extern "C" {

        pub type tls;
    }
}

pub mod _socklen_t_h {

    pub type socklen_t = __darwin_socklen_t;
    use crate::types::__darwin_socklen_t;
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

    pub const AF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

    pub const AF_INET6: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
    use crate::types::__uint8_t;
    use crate::types::sa_family_t;
}

pub mod un_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct sockaddr_un {
        pub sun_len: ::core::ffi::c_uchar,
        pub sun_family: sa_family_t,
        pub sun_path: [::core::ffi::c_char; 104],
    }
    use crate::types::sa_family_t;
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
    use crate::types::__uint8_t;
    use crate::types::in_addr_t;
    use crate::types::in_port_t;
    use crate::types::sa_family_t;
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
    use crate::types::in_port_t;
    use crate::types::sa_family_t;
    use crate::types::{__uint16_t, __uint32_t, __uint8_t};
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
    use super::event_h::event_base;
    use crate::types::timeval;
    use crate::types::uint8_t;
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

    pub type PauseMode = ::core::ffi::c_uint;

    pub const P_SUSPEND: PauseMode = 2;

    pub const P_PAUSE: PauseMode = 1;

    pub const P_NONE: PauseMode = 0;

    pub type ShutDownMode = ::core::ffi::c_uint;

    pub const SHUTDOWN_IMMEDIATE: ShutDownMode = 3;

    pub const SHUTDOWN_WAIT_FOR_CLIENTS: ShutDownMode = 2;

    pub const SHUTDOWN_WAIT_FOR_SERVERS: ShutDownMode = 1;

    pub const SHUTDOWN_NONE: ShutDownMode = 0;

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

    pub const AUTH_TYPE_PAM: auth_type = 7;

    pub type auth_type = ::core::ffi::c_uint;

    pub const AUTH_TYPE_REJECT: auth_type = 10;

    pub const AUTH_TYPE_PEER: auth_type = 9;

    pub const AUTH_TYPE_SCRAM_SHA_256: auth_type = 8;

    pub const AUTH_TYPE_LDAP: auth_type = 6;

    pub const AUTH_TYPE_HBA: auth_type = 5;

    pub const AUTH_TYPE_CERT: auth_type = 4;

    pub const AUTH_TYPE_MD5: auth_type = 3;

    pub const AUTH_TYPE_PLAIN: auth_type = 2;

    pub const AUTH_TYPE_TRUST: auth_type = 1;

    pub const AUTH_TYPE_ANY: auth_type = 0;

    pub const PKT_CANCEL: ::core::ffi::c_int = 80877102 as ::core::ffi::c_int;

    pub const POOL_SESSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const POOL_INHERIT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

    pub const CANCELLATION_TTL_MASK: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
    #[inline]

    pub unsafe extern "C" fn pga_is_unix(mut a: *const PgAddr) -> bool {
        (*a).sa.sa_family as ::core::ffi::c_int == AF_UNIX
    }

    pub const RAW_IOBUF_SIZE: ::core::ffi::c_ulong = 12 as ::core::ffi::c_ulong;
    #[inline]

    pub unsafe extern "C" fn first_socket(mut slist: *mut StatList) -> *mut PgSocket {
        if statlist_empty(slist) {
            return ::core::ptr::null_mut::<PgSocket>();
        }
        ((*slist).head.next as *mut ::core::ffi::c_char)
             as *mut PgSocket
    }
    #[inline]

    pub unsafe extern "C" fn last_socket(mut slist: *mut StatList) -> *mut PgSocket {
        if statlist_empty(slist) {
            return ::core::ptr::null_mut::<PgSocket>();
        }
        ((*slist).head.prev as *mut ::core::ffi::c_char)
             as *mut PgSocket
    }

    use crate::types::pid_t;

    use super::dnslookup_h::{DNSContext, DNSToken};
    use super::in6_h::sockaddr_in6;
    use super::in_h::sockaddr_in;
    use super::pktbuf_h::PktBuf;
    use super::sbuf_h::SBuf;
    use super::socket_h::{sockaddr, AF_UNIX};
    use crate::types::pg_cryptohash_type;
    use crate::types::uid_t;
    use crate::types::uint16_t;
    use crate::types::uint64_t;
    use crate::types::uint8_t;
    use crate::types::usec_t;
    use crate::types::List;
    use crate::types::PktHdr;
    use crate::types::VarCache;
    use crate::types::{statlist_empty, StatList};
    use crate::types::{AANode, AATree};
    use crate::types::{PgClientPreparedStatement, PgServerPreparedStatement};
    extern "C" {

        pub static mut cf_sbuf_len: ::core::ffi::c_int;

        pub fn pga_set(a: *mut PgAddr, fam: ::core::ffi::c_int, port: ::core::ffi::c_int);

        pub fn pga_copy(a: *mut PgAddr, sa: *const sockaddr);

        pub fn pga_cmp_addr(a: *const PgAddr, b: *const PgAddr) -> ::core::ffi::c_int;

        pub static mut cf_unix_socket_dir: *mut ::core::ffi::c_char;

        pub static mut cf_peer_id: ::core::ffi::c_int;

        pub static mut cf_res_pool_timeout: usec_t;

        pub static mut cf_autodb_connstr: *mut ::core::ffi::c_char;

        pub static mut cf_server_reset_query: *mut ::core::ffi::c_char;

        pub static mut cf_server_reset_query_always: ::core::ffi::c_int;

        pub static mut cf_server_check_query: *mut ::core::ffi::c_char;

        pub static mut cf_server_check_delay: usec_t;

        pub static mut cf_server_connect_timeout: usec_t;

        pub static mut cf_server_login_retry: usec_t;

        pub static mut cf_server_round_robin: ::core::ffi::c_int;

        pub static mut cf_auth_type: ::core::ffi::c_int;

        pub static mut cf_pause_mode: ::core::ffi::c_int;

        pub static mut cf_shutdown: ::core::ffi::c_int;

        pub static mut cf_log_connections: ::core::ffi::c_int;

        pub static mut cf_log_disconnections: ::core::ffi::c_int;

        pub static mut adns: *mut DNSContext;
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

    pub unsafe extern "C" fn sbuf_is_empty(mut sbuf: *mut SBuf) -> bool {
        iobuf_empty((*sbuf).io)
            && (*sbuf).pkt_remain == 0 as ::core::ffi::c_uint
    }
    #[inline]

    pub unsafe extern "C" fn sbuf_is_closed(mut sbuf: *mut SBuf) -> bool {
        (*sbuf).sock == 0 as ::core::ffi::c_int
    }
    use super::_socklen_t_h::socklen_t;
    use super::_time_t_h::time_t;
    use super::event_struct_h::event;
    use super::iobuf_h::{iobuf_empty, IOBuf};
    use super::pktbuf_h::PktBuf;
    use super::socket_h::sockaddr;
    use super::tls_h::tls;
    use crate::types::size_t;
    use crate::types::ssize_t;
    use crate::types::uint8_t;
    use crate::types::MBuf;
    extern "C" {

        pub fn sbuf_init(sbuf: *mut SBuf, proto_fn: sbuf_cb_t);

        pub fn sbuf_accept(sbuf: *mut SBuf, read_sock: ::core::ffi::c_int, is_unix: bool) -> bool;

        pub fn sbuf_connect(
            sbuf: *mut SBuf,
            sa: *const sockaddr,
            sa_len: socklen_t,
            timeout_sec: time_t,
        ) -> bool;

        pub fn sbuf_pause(sbuf: *mut SBuf) -> bool;

        pub fn sbuf_continue(sbuf: *mut SBuf);

        pub fn sbuf_close(sbuf: *mut SBuf) -> bool;

        pub fn sbuf_queue_packet(sbuf: *mut SBuf, dst: *mut SBuf, pkt: *mut PktBuf) -> bool;

        pub fn sbuf_answer(sbuf: *mut SBuf, buf: *const ::core::ffi::c_void, len: size_t) -> bool;
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
    #[inline]

    pub unsafe extern "C" fn iobuf_empty(mut io: *const IOBuf) -> bool {
        io.is_null() || (*io).done_pos == (*io).recv_pos
    }
    #[inline]

    pub unsafe extern "C" fn iobuf_reset(mut io: *mut IOBuf) {
        (*io).done_pos = 0 as ::core::ffi::c_uint;
        (*io).parse_pos = (*io).done_pos;
        (*io).recv_pos = (*io).parse_pos;
    }
    use crate::types::uint8_t;
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
    use super::bouncer_h::PgSocket;
    use super::event_struct_h::event;
    use crate::types::uint64_t;
    use crate::types::uint8_t;
    extern "C" {

        pub fn pktbuf_static(buf: *mut PktBuf, data: *mut uint8_t, len: ::core::ffi::c_int);

        pub fn pktbuf_free(buf: *mut PktBuf);

        pub fn pktbuf_send_immediate(buf: *mut PktBuf, sk: *mut PgSocket) -> bool;

        pub fn pktbuf_put_uint64(buf: *mut PktBuf, val: uint64_t);

        pub fn pktbuf_write_generic(
            buf: *mut PktBuf,
            type_0: ::core::ffi::c_int,
            fmt: *const ::core::ffi::c_char,
            ...
        );
    }
}

pub mod dnslookup_h {

    pub type adns_callback_f = Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *const sockaddr, ::core::ffi::c_int) -> (),
    >;
    use super::socket_h::sockaddr;
    extern "C" {

        pub type DNSToken;

        pub type DNSContext;

        pub fn adns_resolve(
            ctx: *mut DNSContext,
            name: *const ::core::ffi::c_char,
            cb_func: adns_callback_f,
            arg: *mut ::core::ffi::c_void,
        ) -> *mut DNSToken;

        pub fn adns_cancel(ctx: *mut DNSContext, tk: *mut DNSToken);
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

pub mod objects_h {
    extern "C" {

        pub type Slab;
    }
}

pub mod slab_h {

    pub type slab_init_fn = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    use super::cxalloc_h::CxMem;
    use super::objects_h::Slab;
    extern "C" {

        pub fn slab_create(
            name: *const ::core::ffi::c_char,
            obj_size: ::core::ffi::c_uint,
            align: ::core::ffi::c_uint,
            init_func: slab_init_fn,
            cx: *const CxMem,
        ) -> *mut Slab;

        pub fn slab_destroy(slab: *mut Slab);

        pub fn slab_alloc(slab: *mut Slab) -> *mut ::core::ffi::c_void;

        pub fn slab_free(slab: *mut Slab, obj: *mut ::core::ffi::c_void);

        pub fn slab_active_count(slab: *const Slab) -> ::core::ffi::c_int;
    }
}

pub mod _OSByteOrder_h {
    #[inline]

    pub unsafe extern "C" fn _OSSwapInt16(mut _data: __uint16_t) -> __uint16_t {
        ((_data as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | _data as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as __uint16_t
    }
    use crate::types::__uint16_t;
}

pub mod _stdio_h {
    use crate::types::size_t;

    extern "C" {

        pub fn snprintf(
            __str: *mut ::core::ffi::c_char,
            __size: size_t,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;

        pub fn vsnprintf(
            __str: *mut ::core::ffi::c_char,
            __size: size_t,
            __format: *const ::core::ffi::c_char,
            _: ::core::ffi::VaList,
        ) -> ::core::ffi::c_int;
    }
}

pub mod usual_socket_h {
    use super::socket_h::sockaddr;
    use crate::types::size_t;
    extern "C" {

        pub fn sa2str(
            sa: *const sockaddr,
            buf: *mut ::core::ffi::c_char,
            buflen: size_t,
        ) -> *const ::core::ffi::c_char;
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

        pub fn strtok(
            __str: *mut ::core::ffi::c_char,
            __sep: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;

        pub fn strlcpy(
            __dst: *mut ::core::ffi::c_char,
            __source: *const ::core::ffi::c_char,
            __size: size_t,
        ) -> ::core::ffi::c_ulong;
    }
}

pub mod util_h {
    use super::bouncer_h::PgSocket;
    extern "C" {

        pub fn fill_remote_addr(sk: *mut PgSocket, fd: ::core::ffi::c_int, is_unix: bool);

        pub fn fill_local_addr(sk: *mut PgSocket, fd: ::core::ffi::c_int, is_unix: bool);
    }
}

pub mod admin_h {
    use super::bouncer_h::PgSocket;
    extern "C" {

        pub fn admin_handle_cancel(client: *mut PgSocket);
    }
}

pub mod client_h {
    use super::bouncer_h::PgSocket;
    use super::sbuf_h::{SBuf, SBufEvent};
    use crate::types::MBuf;
    extern "C" {

        pub fn client_proto(sbuf: *mut SBuf, evtype: SBufEvent, pkt: *mut MBuf) -> bool;

        pub fn set_pool(
            client: *mut PgSocket,
            dbname: *const ::core::ffi::c_char,
            username: *const ::core::ffi::c_char,
            password: *const ::core::ffi::c_char,
            takeover: bool,
        ) -> bool;

        pub fn sending_auth_query(client: *mut PgSocket) -> bool;
    }
}

pub mod server_h {
    use super::bouncer_h::{PgDatabase, PgGlobalUser, PgPool, PgSocket};
    use super::sbuf_h::{SBuf, SBufEvent};
    use crate::types::usec_t;
    use crate::types::MBuf;
    extern "C" {

        pub fn server_proto(sbuf: *mut SBuf, evtype: SBufEvent, pkt: *mut MBuf) -> bool;

        pub fn connection_pool_mode(connection: *mut PgSocket) -> ::core::ffi::c_int;

        pub fn pool_pool_size(pool: *mut PgPool) -> ::core::ffi::c_int;

        pub fn pool_server_lifetime(pool: *mut PgPool) -> usec_t;

        pub fn pool_res_pool_size(pool: *mut PgPool) -> ::core::ffi::c_int;

        pub fn database_max_connections(db: *mut PgDatabase) -> ::core::ffi::c_int;

        pub fn user_max_connections(user: *mut PgGlobalUser) -> ::core::ffi::c_int;
    }
}

pub mod scram_h {
    use super::bouncer_h::ScramState;
    extern "C" {

        pub fn free_scram_state(state: *mut ScramState);
    }
}

pub mod janitor_h {
    use super::bouncer_h::PgDatabase;
    use crate::types::AATree;
    extern "C" {

        pub fn kill_database(db: *mut PgDatabase);

        pub fn kill_peer(db: *mut PgDatabase);

        pub fn clear_user_tree_cached_scram_keys(tree: *mut AATree);
    }
}

pub mod _malloc_h {
    extern "C" {

        pub fn free(_: *mut ::core::ffi::c_void);
    }
}

pub mod _stdlib_h {
    extern "C" {

        pub fn exit(_: ::core::ffi::c_int) -> !;
    }
}

pub mod errno_h {
    extern "C" {

        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}

pub mod inet_h {
    extern "C" {

        pub fn inet_pton(
            _: ::core::ffi::c_int,
            _: *const ::core::ffi::c_char,
            _: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
    }
}

pub mod protocol_h {

    pub const PqMsg_Close: ::core::ffi::c_int = 'C' as i32;

    pub const PqMsg_Parse: ::core::ffi::c_int = 'P' as i32;

    pub const PqMsg_Query: ::core::ffi::c_int = 'Q' as i32;

    pub const PqMsg_Terminate: ::core::ffi::c_int = 'X' as i32;

    pub const PqMsg_ParseComplete: ::core::ffi::c_int = '1' as i32;

    pub const PqMsg_CloseComplete: ::core::ffi::c_int = '3' as i32;
}

pub mod loader_h {
    extern "C" {

        pub fn parse_database(
            base: *mut ::core::ffi::c_void,
            name: *const ::core::ffi::c_char,
            connstr: *const ::core::ffi::c_char,
        ) -> bool;
    }
}

pub mod err_h {
    extern "C" {

        pub fn xstrdup(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    }
}

pub mod safeio_h {
    extern "C" {

        pub fn safe_close(fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    }
}
pub use self::_OSByteOrder_h::_OSSwapInt16;
use self::_malloc_h::free;
pub use self::_socklen_t_h::socklen_t;
use self::_stdio_h::{snprintf, vsnprintf};
use self::_stdlib_h::exit;
use self::_string_h::{memcmp, memcpy, memset, strchr, strcmp, strerror, strlcpy, strlen, strtok};
pub use self::_time_t_h::time_t;
pub use self::_va_list_h::va_list;
use self::admin_h::admin_handle_cancel;
pub use self::bouncer_h::{
    adns, auth_type, cf_auth_type, cf_autodb_connstr, cf_log_connections, cf_log_disconnections,
    cf_pause_mode, cf_peer_id, cf_res_pool_timeout, cf_sbuf_len, cf_server_check_delay,
    cf_server_check_query, cf_server_connect_timeout, cf_server_login_retry, cf_server_reset_query,
    cf_server_reset_query_always, cf_server_round_robin, cf_shutdown, cf_unix_socket_dir,
    first_socket, last_socket, pga_cmp_addr, pga_copy, pga_is_unix, pga_set, sockaddr_ucreds,
    C2RustUnnamed_9, CallbackState, LoadBalanceHosts, OutstandingRequest, PacketCallbackFlag,
    PauseMode, PgAddr, PgCredentials, PgDatabase, PgGlobalUser, PgPool, PgSocket, PgStats,
    ReplicationType, ResponseAction, ScramState, ShutDownMode, SocketState, AUTH_TYPE_ANY,
    AUTH_TYPE_CERT, AUTH_TYPE_HBA, AUTH_TYPE_LDAP, AUTH_TYPE_MD5, AUTH_TYPE_PAM, AUTH_TYPE_PEER,
    AUTH_TYPE_PLAIN, AUTH_TYPE_REJECT, AUTH_TYPE_SCRAM_SHA_256, AUTH_TYPE_TRUST,
    CANCELLATION_TTL_MASK, CB_HANDLE_COMPLETE_PACKET, CB_NONE, CB_WANT_COMPLETE_PACKET, CL_ACTIVE,
    CL_ACTIVE_CANCEL, CL_FREE, CL_JUSTFREE, CL_LOGIN, CL_WAITING, CL_WAITING_CANCEL,
    CL_WAITING_LOGIN, LOAD_BALANCE_HOSTS_DISABLE, LOAD_BALANCE_HOSTS_ROUND_ROBIN, PKT_CANCEL,
    POOL_INHERIT, POOL_SESSION, P_NONE, P_PAUSE, P_SUSPEND, RAW_IOBUF_SIZE, RA_FAKE, RA_FORWARD,
    RA_SKIP, REPLICATION_LOGICAL, REPLICATION_NONE, REPLICATION_PHYSICAL, SHUTDOWN_IMMEDIATE,
    SHUTDOWN_NONE, SHUTDOWN_WAIT_FOR_CLIENTS, SHUTDOWN_WAIT_FOR_SERVERS, SV_ACTIVE,
    SV_ACTIVE_CANCEL, SV_BEING_CANCELED, SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN, SV_TESTED,
    SV_USED,
};
use self::client_h::{client_proto, sending_auth_query, set_pool};
pub use self::cxalloc_h::{cx_libc_allocator, CxMem, CxOps};
pub use self::dnslookup_h::{adns_callback_f, adns_cancel, adns_resolve, DNSContext, DNSToken};
use self::err_h::xstrdup;
use self::errno_h::__error;
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
    __darwin_va_list, __int32_t, __uint16_t, __uint32_t, __uint8_t,
};
pub use crate::types::{
    aatree_cmp_f, aatree_init, aatree_insert, aatree_search, aatree_walker_f, AANode, AATree,
};
pub use crate::types::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};

pub use self::event_struct_h::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
pub use self::in6_h::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use self::in_h::{in_addr, sockaddr_in};
use self::inet_h::inet_pton;
pub use self::internal::__builtin_va_list;
pub use self::iobuf_h::{iobuf, iobuf_empty, iobuf_reset, IOBuf};
use self::janitor_h::{clear_user_tree_cached_scram_keys, kill_database, kill_peer};
use self::loader_h::parse_database;
pub use self::logging_h::{
    cf_verbose, log_fatal, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE,
    LG_STATS, LG_WARNING,
};
use self::objects_h::Slab;
pub use self::pktbuf_h::{
    pktbuf_free, pktbuf_put_uint64, pktbuf_send_immediate, pktbuf_static, pktbuf_write_generic,
    PktBuf,
};
pub use crate::lib::usual::mbuf::mbuf_free;
pub use crate::types::MBuf;
pub use crate::types::{
    list_append, list_del, list_empty, list_first, list_init, list_pop, list_prepend, List,
};
pub use crate::types::{PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement};

// External function declarations (defined in prepare.rs)
extern "C" {
    pub fn add_prepared_statement(
        server: *mut bouncer_h::PgSocket,
        server_ps: *mut PgServerPreparedStatement,
    ) -> bool;
    pub fn free_client_prepared_statements(client: *mut bouncer_h::PgSocket);
    pub fn free_server_prepared_statement(stmt: *mut PgServerPreparedStatement);
    pub fn free_server_prepared_statements(server: *mut bouncer_h::PgSocket);
    pub fn unregister_prepared_statement(server: *mut bouncer_h::PgSocket, query_id: u64);
}
pub use crate::types::{free_header, PktHdr};

// External function declarations (defined in proto.rs)
extern "C" {
    pub fn send_pooler_error(
        client: *mut bouncer_h::PgSocket,
        send_ready: bool,
        sqlstate: *const ::core::ffi::c_char,
        level_fatal: bool,
        msg: *const ::core::ffi::c_char,
    ) -> bool;
    pub fn welcome_client(client: *mut bouncer_h::PgSocket) -> bool;
}
pub use self::protocol_h::{
    PqMsg_Close, PqMsg_CloseComplete, PqMsg_Parse, PqMsg_ParseComplete, PqMsg_Query,
    PqMsg_Terminate,
};
use self::safeio_h::safe_close;
pub use self::sbuf_h::{
    sbuf_accept, sbuf_answer, sbuf_cb_t, sbuf_close, sbuf_connect, sbuf_continue, sbuf_init,
    sbuf_is_closed, sbuf_is_empty, sbuf_pause, sbuf_queue_packet, SBuf, SBufEvent, SBufIO,
    SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK, SBUF_EV_FLUSH, SBUF_EV_PKT_CALLBACK, SBUF_EV_READ,
    SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED, SBUF_EV_TLS_READY,
};
use self::scram_h::free_scram_state;
use self::server_h::{
    connection_pool_mode, database_max_connections, pool_pool_size, pool_res_pool_size,
    pool_server_lifetime, server_proto, user_max_connections,
};
pub use self::slab_h::{
    slab_active_count, slab_alloc, slab_create, slab_destroy, slab_free, slab_init_fn,
};
pub use self::socket_h::{sockaddr, AF_INET, AF_INET6, AF_UNIX};
pub use self::sys__types_h::{__darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t, __DARWIN_NULL};
pub use crate::types::{false_0, true_0};
pub use crate::types::{
    statlist_append, statlist_count, statlist_empty, statlist_first, statlist_init, statlist_pop,
    statlist_prepend, statlist_put_before, statlist_remove, StatList,
};
pub use crate::types::{usec_t, USEC};
pub use crate::types::{PStr, StrPool};

pub use self::un_h::sockaddr_un;
use self::usual_socket_h::sa2str;
use self::util_h::{fill_local_addr, fill_remote_addr};
pub use crate::types::VarCache;
pub use crate::types::{UT_hash_bucket, UT_hash_handle, UT_hash_table};
#[no_mangle]

pub static mut user_list: StatList = StatList {
    head: List {
        next: ::core::ptr::null::<List>() as *mut List,
        prev: ::core::ptr::null::<List>() as *mut List,
    },
    cur_count: 0,
};
#[no_mangle]

pub static mut database_list: StatList = StatList {
    head: List {
        next: ::core::ptr::null::<List>() as *mut List,
        prev: ::core::ptr::null::<List>() as *mut List,
    },
    cur_count: 0,
};
#[no_mangle]

pub static mut pool_list: StatList = StatList {
    head: List {
        next: ::core::ptr::null::<List>() as *mut List,
        prev: ::core::ptr::null::<List>() as *mut List,
    },
    cur_count: 0,
};
#[no_mangle]

pub static mut peer_list: StatList = StatList {
    head: List {
        next: ::core::ptr::null::<List>() as *mut List,
        prev: ::core::ptr::null::<List>() as *mut List,
    },
    cur_count: 0,
};
#[no_mangle]

pub static mut peer_pool_list: StatList = StatList {
    head: List {
        next: ::core::ptr::null::<List>() as *mut List,
        prev: ::core::ptr::null::<List>() as *mut List,
    },
    cur_count: 0,
};
#[no_mangle]

pub static mut user_tree: AATree = AATree {
    root: ::core::ptr::null::<AANode>() as *mut AANode,
    count: 0,
    node_cmp: None,
    release_cb: None,
};
#[no_mangle]

pub static mut pam_user_tree: AATree = AATree {
    root: ::core::ptr::null::<AANode>() as *mut AANode,
    count: 0,
    node_cmp: None,
    release_cb: None,
};
#[no_mangle]

pub static mut prepared_statements: *mut PgPreparedStatement =
    ::core::ptr::null::<PgPreparedStatement>() as *mut PgPreparedStatement;
#[no_mangle]

pub static mut login_client_list: StatList = StatList {
    head: List {
        next: ::core::ptr::null::<List>() as *mut List,
        prev: ::core::ptr::null::<List>() as *mut List,
    },
    cur_count: 0,
};
#[no_mangle]

pub static mut server_cache: *mut Slab = ::core::ptr::null::<Slab>() as *mut Slab;
#[no_mangle]

pub static mut client_cache: *mut Slab = ::core::ptr::null::<Slab>() as *mut Slab;
#[no_mangle]

pub static mut db_cache: *mut Slab = ::core::ptr::null::<Slab>() as *mut Slab;
#[no_mangle]

pub static mut peer_cache: *mut Slab = ::core::ptr::null::<Slab>() as *mut Slab;
#[no_mangle]

pub static mut peer_pool_cache: *mut Slab = ::core::ptr::null::<Slab>() as *mut Slab;
#[no_mangle]

pub static mut pool_cache: *mut Slab = ::core::ptr::null::<Slab>() as *mut Slab;
#[no_mangle]

pub static mut user_cache: *mut Slab = ::core::ptr::null::<Slab>() as *mut Slab;
#[no_mangle]

pub static mut credentials_cache: *mut Slab = ::core::ptr::null::<Slab>() as *mut Slab;
#[no_mangle]

pub static mut iobuf_cache: *mut Slab = ::core::ptr::null::<Slab>() as *mut Slab;
#[no_mangle]

pub static mut outstanding_request_cache: *mut Slab = ::core::ptr::null::<Slab>() as *mut Slab;
#[no_mangle]

pub static mut var_list_cache: *mut Slab = ::core::ptr::null::<Slab>() as *mut Slab;
#[no_mangle]

pub static mut server_prepared_statement_cache: *mut Slab =
    ::core::ptr::null::<Slab>() as *mut Slab;
#[no_mangle]

pub static mut last_pgsocket_id: ::core::ffi::c_ulonglong = 0;

static mut justfree_client_list: StatList = StatList {
    head: List {
        next: ::core::ptr::null::<List>() as *mut List,
        prev: ::core::ptr::null::<List>() as *mut List,
    },
    cur_count: 0,
};

static mut justfree_server_list: StatList = StatList {
    head: List {
        next: ::core::ptr::null::<List>() as *mut List,
        prev: ::core::ptr::null::<List>() as *mut List,
    },
    cur_count: 0,
};
#[no_mangle]

pub static mut autodatabase_idle_list: StatList = StatList {
    head: List {
        next: ::core::ptr::null::<List>() as *mut List,
        prev: ::core::ptr::null::<List>() as *mut List,
    },
    cur_count: 0,
};
#[no_mangle]

pub static mut replication_type_parameters: [*const ::core::ffi::c_char; 3] =
    [c"no".as_ptr(), c"database".as_ptr(), c"yes".as_ptr()];
#[no_mangle]

pub unsafe extern "C" fn get_active_client_count() -> ::core::ffi::c_int {
    slab_active_count(client_cache)
}
#[no_mangle]

pub unsafe extern "C" fn get_active_server_count() -> ::core::ffi::c_int {
    slab_active_count(server_cache)
}

unsafe extern "C" fn construct_client(mut obj: *mut ::core::ffi::c_void) {
    let mut client = obj as *mut PgSocket;
    memset(
        client as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PgSocket>() as size_t,
    );
    list_init(&raw mut (*client).head);
    sbuf_init(
        &raw mut (*client).sbuf,
        Some(client_proto as unsafe extern "C" fn(*mut SBuf, SBufEvent, *mut MBuf) -> bool),
    );
    (*client).vars.var_list = slab_alloc(var_list_cache) as *mut *mut PStr;
    (*client).set_state(CL_FREE as SocketState);
    (*client).client_prepared_statements = ::core::ptr::null_mut::<PgClientPreparedStatement>();
    last_pgsocket_id = last_pgsocket_id.wrapping_add(1);
    (*client).id = last_pgsocket_id;
}

unsafe extern "C" fn construct_server(mut obj: *mut ::core::ffi::c_void) {
    let mut server = obj as *mut PgSocket;
    memset(
        server as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PgSocket>() as size_t,
    );
    list_init(&raw mut (*server).head);
    sbuf_init(
        &raw mut (*server).sbuf,
        Some(server_proto as unsafe extern "C" fn(*mut SBuf, SBufEvent, *mut MBuf) -> bool),
    );
    (*server).vars.var_list = slab_alloc(var_list_cache) as *mut *mut PStr;
    (*server).set_state(SV_FREE as SocketState);
    (*server).server_prepared_statements = ::core::ptr::null_mut::<PgServerPreparedStatement>();
    (*server).host = ::core::ptr::null_mut::<::core::ffi::c_char>();
    statlist_init(
        &raw mut (*server).outstanding_requests,
        c"outstanding_requests".as_ptr(),
    );
    last_pgsocket_id = last_pgsocket_id.wrapping_add(1);
    (*server).id = last_pgsocket_id;
}

unsafe extern "C" fn global_user_node_cmp(
    mut userptr: uintptr_t,
    mut node: *mut AANode,
) -> ::core::ffi::c_int {
    let mut name = userptr as *const ::core::ffi::c_char;
    let mut global_user = (node as *mut ::core::ffi::c_char)
        
        as *mut PgGlobalUser;
    strcmp(
        name,
        &raw mut (*global_user).credentials.name as *mut ::core::ffi::c_char,
    )
}

unsafe extern "C" fn credentials_node_cmp(
    mut userptr: uintptr_t,
    mut node: *mut AANode,
) -> ::core::ffi::c_int {
    let mut name = userptr as *const ::core::ffi::c_char;
    let mut credentials = (node as *mut ::core::ffi::c_char)
        
        as *mut PgCredentials;
    strcmp(
        name,
        &raw mut (*credentials).name as *mut ::core::ffi::c_char,
    )
}

unsafe extern "C" fn credentials_node_release(
    mut node: *mut AANode,
    mut _arg: *mut ::core::ffi::c_void,
) {
    let mut user = (node as *mut ::core::ffi::c_char)
        as *mut PgCredentials;
    slab_free(credentials_cache, user as *mut ::core::ffi::c_void);
}
#[no_mangle]

pub unsafe extern "C" fn init_objects() {
    aatree_init(
        &raw mut user_tree,
        Some(
            global_user_node_cmp
                as unsafe extern "C" fn(uintptr_t, *mut AANode) -> ::core::ffi::c_int,
        ),
        None,
    );
    aatree_init(
        &raw mut pam_user_tree,
        Some(
            credentials_node_cmp
                as unsafe extern "C" fn(uintptr_t, *mut AANode) -> ::core::ffi::c_int,
        ),
        None,
    );
    user_cache = slab_create(
        c"user_cache".as_ptr(),
        ::core::mem::size_of::<PgGlobalUser>() as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        None,
        &raw const cx_libc_allocator,
    );
    credentials_cache = slab_create(
        c"credentials_cache".as_ptr(),
        ::core::mem::size_of::<PgCredentials>() as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        None,
        &raw const cx_libc_allocator,
    );
    db_cache = slab_create(
        c"db_cache".as_ptr(),
        ::core::mem::size_of::<PgDatabase>() as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        None,
        &raw const cx_libc_allocator,
    );
    peer_cache = slab_create(
        c"peer_cache".as_ptr(),
        ::core::mem::size_of::<PgDatabase>() as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        None,
        &raw const cx_libc_allocator,
    );
    peer_pool_cache = slab_create(
        c"peer_pool_cache".as_ptr(),
        ::core::mem::size_of::<PgPool>() as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        None,
        &raw const cx_libc_allocator,
    );
    pool_cache = slab_create(
        c"pool_cache".as_ptr(),
        ::core::mem::size_of::<PgPool>() as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        None,
        &raw const cx_libc_allocator,
    );
    outstanding_request_cache = slab_create(
        c"outstanding_request_cache".as_ptr(),
        ::core::mem::size_of::<OutstandingRequest>() as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        None,
        &raw const cx_libc_allocator,
    );
    if user_cache.is_null()
        || db_cache.is_null()
        || peer_cache.is_null()
        || peer_pool_cache.is_null()
        || pool_cache.is_null()
    {
        let mut _log_ctx = NULL;
        log_fatal(
            c"src/objects.c".as_ptr(),
            171 as ::core::ffi::c_int,
            c"init_objects".as_ptr(),
            false,
            _log_ctx,
            c"cannot create initial caches".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
}

unsafe extern "C" fn do_iobuf_reset(mut arg: *mut ::core::ffi::c_void) {
    let mut io = arg as *mut IOBuf;
    iobuf_reset(io);
}
#[no_mangle]

pub unsafe extern "C" fn init_caches() {
    server_cache = slab_create(
        c"server_cache".as_ptr(),
        ::core::mem::size_of::<PgSocket>() as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        Some(construct_server as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        &raw const cx_libc_allocator,
    );
    client_cache = slab_create(
        c"client_cache".as_ptr(),
        ::core::mem::size_of::<PgSocket>() as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        Some(construct_client as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        &raw const cx_libc_allocator,
    );
    iobuf_cache = slab_create(
        c"iobuf_cache".as_ptr(),
        RAW_IOBUF_SIZE.wrapping_add(cf_sbuf_len as ::core::ffi::c_ulong) as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        Some(do_iobuf_reset as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        &raw const cx_libc_allocator,
    );
    var_list_cache = slab_create(
        c"var_list_cache".as_ptr(),
        ::core::mem::size_of::<*mut PStr>().wrapping_mul(get_num_var_cached() as usize)
            as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        None,
        &raw const cx_libc_allocator,
    );
    server_prepared_statement_cache = slab_create(
        c"server_prepared_statement_cache".as_ptr(),
        ::core::mem::size_of::<PgServerPreparedStatement>() as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        None,
        &raw const cx_libc_allocator,
    );
}

unsafe extern "C" fn client_free(mut client: *mut PgSocket) {
    free_client_prepared_statements(client);
    varcache_clean(&raw mut (*client).vars);
    slab_free(
        var_list_cache,
        (*client).vars.var_list as *mut ::core::ffi::c_void,
    );
    slab_free(client_cache, client as *mut ::core::ffi::c_void);
}

unsafe extern "C" fn server_free(mut server: *mut PgSocket) {
    let mut el = ::core::ptr::null_mut::<List>();
    let mut tmp_l = ::core::ptr::null_mut::<List>();
    let mut request = ::core::ptr::null_mut::<OutstandingRequest>();
    el = (*server).outstanding_requests.head.next;
    tmp_l = (*(*server).outstanding_requests.head.next).next;
    while el != &raw mut (*server).outstanding_requests.head {
        request = (el as *mut ::core::ffi::c_char)
            as *mut OutstandingRequest;
        statlist_remove(&raw mut (*server).canceling_clients, el);
        if !(*request).server_ps.is_null() {
            free_server_prepared_statement((*request).server_ps);
        }
        slab_free(
            outstanding_request_cache,
            request as *mut ::core::ffi::c_void,
        );
        el = tmp_l;
        tmp_l = (*tmp_l).next;
    }
    free_server_prepared_statements(server);
    free((*server).host as *mut ::core::ffi::c_void);
    varcache_clean(&raw mut (*server).vars);
    slab_free(
        var_list_cache,
        (*server).vars.var_list as *mut ::core::ffi::c_void,
    );
    slab_free(server_cache, server as *mut ::core::ffi::c_void);
}
#[no_mangle]

pub unsafe extern "C" fn change_client_state(mut client: *mut PgSocket, mut newstate: SocketState) {
    let mut pool = (*client).pool;
    let mut current_block_14: u64;
    match (*client).state() as ::core::ffi::c_int {
        0 => {
            current_block_14 = 17833034027772472439;
        }
        1 => {
            statlist_remove(&raw mut justfree_client_list, &raw mut (*client).head);
            current_block_14 = 17833034027772472439;
        }
        2 => {
            if newstate as ::core::ffi::c_uint
                == CL_WAITING as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                newstate = CL_WAITING_LOGIN;
            }
            statlist_remove(&raw mut login_client_list, &raw mut (*client).head);
            current_block_14 = 17833034027772472439;
        }
        4 => {
            if newstate as ::core::ffi::c_uint
                == CL_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                newstate = CL_LOGIN;
            }
            current_block_14 = 2284361961084658740;
        }
        3 => {
            current_block_14 = 2284361961084658740;
        }
        5 => {
            statlist_remove(&raw mut (*pool).active_client_list, &raw mut (*client).head);
            current_block_14 = 17833034027772472439;
        }
        7 => {
            statlist_remove(
                &raw mut (*pool).active_cancel_req_list,
                &raw mut (*client).head,
            );
            current_block_14 = 17833034027772472439;
        }
        6 => {
            statlist_remove(
                &raw mut (*pool).waiting_cancel_req_list,
                &raw mut (*client).head,
            );
            current_block_14 = 17833034027772472439;
        }
        _ => {
            let mut _log_ctx = NULL;
            log_fatal(
                c"src/objects.c".as_ptr(),
                256 as ::core::ffi::c_int,
                c"change_client_state".as_ptr(),
                false,
                _log_ctx,
                c"bad cur client state: %d".as_ptr(),
                (*client).state() as ::core::ffi::c_int,
            );
            exit(1 as ::core::ffi::c_int);
        }
    }
    if current_block_14 == 2284361961084658740 {
        (*client).set_sent_wait_notification((false));
        statlist_remove(
            &raw mut (*pool).waiting_client_list,
            &raw mut (*client).head,
        );
    }
    (*client).set_state(newstate as SocketState);
    match (*client).state() as ::core::ffi::c_int {
        0 => {
            client_free(client);
        }
        1 => {
            statlist_append(&raw mut justfree_client_list, &raw mut (*client).head);
        }
        2 => {
            statlist_append(&raw mut login_client_list, &raw mut (*client).head);
        }
        3 | 4 => {
            (*client).wait_start = get_cached_time();
            statlist_append(
                &raw mut (*pool).waiting_client_list,
                &raw mut (*client).head,
            );
        }
        5 => {
            statlist_append(&raw mut (*pool).active_client_list, &raw mut (*client).head);
        }
        7 => {
            statlist_append(
                &raw mut (*pool).active_cancel_req_list,
                &raw mut (*client).head,
            );
        }
        6 => {
            statlist_append(
                &raw mut (*pool).waiting_cancel_req_list,
                &raw mut (*client).head,
            );
        }
        _ => {
            let mut _log_ctx_0 = NULL;
            log_fatal(
                c"src/objects.c".as_ptr(),
                287 as ::core::ffi::c_int,
                c"change_client_state".as_ptr(),
                false,
                _log_ctx_0,
                c"bad new client state: %d".as_ptr(),
                (*client).state() as ::core::ffi::c_int,
            );
            exit(1 as ::core::ffi::c_int);
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn change_server_state(mut server: *mut PgSocket, mut newstate: SocketState) {
    let mut pool = (*server).pool;
    match (*server).state() as ::core::ffi::c_int {
        8 => {}
        9 => {
            statlist_remove(&raw mut justfree_server_list, &raw mut (*server).head);
        }
        10 => {
            statlist_remove(&raw mut (*pool).new_server_list, &raw mut (*server).head);
        }
        15 => {
            statlist_remove(&raw mut (*pool).used_server_list, &raw mut (*server).head);
        }
        16 => {
            statlist_remove(&raw mut (*pool).tested_server_list, &raw mut (*server).head);
        }
        11 => {
            statlist_remove(
                &raw mut (*pool).being_canceled_server_list,
                &raw mut (*server).head,
            );
        }
        12 => {
            statlist_remove(&raw mut (*pool).idle_server_list, &raw mut (*server).head);
        }
        13 => {
            statlist_remove(&raw mut (*pool).active_server_list, &raw mut (*server).head);
        }
        14 => {
            statlist_remove(
                &raw mut (*pool).active_cancel_server_list,
                &raw mut (*server).head,
            );
        }
        _ => {
            let mut _log_ctx = NULL;
            log_fatal(
                c"src/objects.c".as_ptr(),
                325 as ::core::ffi::c_int,
                c"change_server_state".as_ptr(),
                false,
                _log_ctx,
                c"bad old server state: %d".as_ptr(),
                (*server).state() as ::core::ffi::c_int,
            );
            exit(1 as ::core::ffi::c_int);
        }
    }
    (*server).set_state(newstate as SocketState);
    match (*server).state() as ::core::ffi::c_int {
        8 => {
            server_free(server);
        }
        9 => {
            statlist_append(&raw mut justfree_server_list, &raw mut (*server).head);
        }
        10 => {
            statlist_append(&raw mut (*pool).new_server_list, &raw mut (*server).head);
        }
        15 => {
            statlist_prepend(&raw mut (*pool).used_server_list, &raw mut (*server).head);
        }
        16 => {
            statlist_append(&raw mut (*pool).tested_server_list, &raw mut (*server).head);
        }
        11 => {
            statlist_append(
                &raw mut (*pool).being_canceled_server_list,
                &raw mut (*server).head,
            );
        }
        12 => {
            if (*server).close_needed() || cf_server_round_robin != 0 {
                statlist_append(&raw mut (*pool).idle_server_list, &raw mut (*server).head);
            } else {
                statlist_prepend(&raw mut (*pool).idle_server_list, &raw mut (*server).head);
            }
        }
        13 => {
            statlist_append(&raw mut (*pool).active_server_list, &raw mut (*server).head);
        }
        14 => {
            statlist_append(
                &raw mut (*pool).active_cancel_server_list,
                &raw mut (*server).head,
            );
        }
        _ => {
            let mut _log_ctx_0 = NULL;
            log_fatal(
                c"src/objects.c".as_ptr(),
                367 as ::core::ffi::c_int,
                c"change_server_state".as_ptr(),
                false,
                _log_ctx_0,
                c"bad server state: %d".as_ptr(),
                (*server).state() as ::core::ffi::c_int,
            );
            exit(1 as ::core::ffi::c_int);
        }
    };
}

unsafe extern "C" fn cmp_pool(mut i1: *mut List, mut i2: *mut List) -> ::core::ffi::c_int {
    let mut p1 = (i1 as *mut ::core::ffi::c_char)
        as *mut PgPool;
    let mut p2 = (i2 as *mut ::core::ffi::c_char)
        as *mut PgPool;
    if (*p1).db != (*p2).db {
        return strcmp(
            &raw mut (*(*p1).db).name as *mut ::core::ffi::c_char,
            &raw mut (*(*p2).db).name as *mut ::core::ffi::c_char,
        );
    }
    if (*p1).user_credentials != (*p2).user_credentials {
        if (*p1).user_credentials.is_null() {
            return 1 as ::core::ffi::c_int;
        }
        if (*p2).user_credentials.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        return strcmp(
            &raw mut (*(*p1).user_credentials).name as *mut ::core::ffi::c_char,
            &raw mut (*(*p2).user_credentials).name as *mut ::core::ffi::c_char,
        );
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn cmp_peer_pool(mut i1: *mut List, mut i2: *mut List) -> ::core::ffi::c_int {
    let mut p1 = (i1 as *mut ::core::ffi::c_char)
        as *mut PgPool;
    let mut p2 = (i2 as *mut ::core::ffi::c_char)
        as *mut PgPool;
    if (*p1).db != (*p2).db {
        return (*(*p1).db).peer_id - (*(*p2).db).peer_id;
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn cmp_user(mut i1: *mut List, mut i2: *mut List) -> ::core::ffi::c_int {
    let mut u1 = (i1 as *mut ::core::ffi::c_char).offset(-(2336 as ::core::ffi::c_ulong as isize))
        as *mut PgGlobalUser;
    let mut u2 = (i2 as *mut ::core::ffi::c_char).offset(-(2336 as ::core::ffi::c_ulong as isize))
        as *mut PgGlobalUser;
    strcmp(
        &raw mut (*u1).credentials.name as *mut ::core::ffi::c_char,
        &raw mut (*u2).credentials.name as *mut ::core::ffi::c_char,
    )
}

unsafe extern "C" fn cmp_peer(mut i1: *mut List, mut i2: *mut List) -> ::core::ffi::c_int {
    let mut db1 = (i1 as *mut ::core::ffi::c_char)
        as *mut PgDatabase;
    let mut db2 = (i2 as *mut ::core::ffi::c_char)
        as *mut PgDatabase;
    (*db1).peer_id - (*db2).peer_id
}

unsafe extern "C" fn cmp_database(mut i1: *mut List, mut i2: *mut List) -> ::core::ffi::c_int {
    let mut db1 = (i1 as *mut ::core::ffi::c_char)
        as *mut PgDatabase;
    let mut db2 = (i2 as *mut ::core::ffi::c_char)
        as *mut PgDatabase;
    strcmp(
        &raw mut (*db1).name as *mut ::core::ffi::c_char,
        &raw mut (*db2).name as *mut ::core::ffi::c_char,
    )
}

unsafe extern "C" fn put_in_order(
    mut newitem: *mut List,
    mut list: *mut StatList,
    mut cmpfn: Option<unsafe extern "C" fn(*mut List, *mut List) -> ::core::ffi::c_int>,
) {
    let mut res: ::core::ffi::c_int = 0;
    let mut item = ::core::ptr::null_mut::<List>();
    item = (*list).head.next;
    while item != &raw mut (*list).head {
        res = cmpfn.expect("non-null function pointer")(item, newitem);
        if res == 0 as ::core::ffi::c_int {
            let mut _log_ctx = NULL;
            log_fatal(
                c"src/objects.c".as_ptr(),
                435 as ::core::ffi::c_int,
                c"put_in_order".as_ptr(),
                false,
                _log_ctx,
                c"put_in_order: found existing elem".as_ptr(),
            );
            exit(1 as ::core::ffi::c_int);
        } else if res > 0 as ::core::ffi::c_int {
            statlist_put_before(list, newitem, item);
            return;
        }
        item = (*item).next;
    }
    statlist_append(list, newitem);
}
#[no_mangle]

pub unsafe extern "C" fn add_peer(
    mut _name: *const ::core::ffi::c_char,
    mut peer_id: ::core::ffi::c_int,
) -> *mut PgDatabase {
    let mut peer = find_peer(peer_id);
    if peer.is_null() {
        peer = slab_alloc(peer_cache) as *mut PgDatabase;
        if peer.is_null() {
            return ::core::ptr::null_mut::<PgDatabase>();
        }
        list_init(&raw mut (*peer).head);
        (*peer).peer_id = peer_id;
        put_in_order(
            &raw mut (*peer).head,
            &raw mut peer_list,
            Some(cmp_peer as unsafe extern "C" fn(*mut List, *mut List) -> ::core::ffi::c_int),
        );
    }
    peer
}
#[no_mangle]

pub unsafe extern "C" fn add_database(mut name: *const ::core::ffi::c_char) -> *mut PgDatabase {
    let mut db = find_database(name);
    if db.is_null() {
        db = slab_alloc(db_cache) as *mut PgDatabase;
        if db.is_null() {
            return ::core::ptr::null_mut::<PgDatabase>();
        }
        list_init(&raw mut (*db).head);
        if strlcpy(
            &raw mut (*db).name as *mut ::core::ffi::c_char,
            name,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
        ) as usize
            >= ::core::mem::size_of::<[::core::ffi::c_char; 64]>()
        {
            let mut _log_ctx = NULL;
            log_generic(LG_WARNING, _log_ctx, c"too long db name: %s".as_ptr(), name);
            slab_free(db_cache, db as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<PgDatabase>();
        }
        aatree_init(
            &raw mut (*db).user_tree,
            Some(
                credentials_node_cmp
                    as unsafe extern "C" fn(uintptr_t, *mut AANode) -> ::core::ffi::c_int,
            ),
            Some(
                credentials_node_release
                    as unsafe extern "C" fn(*mut AANode, *mut ::core::ffi::c_void) -> (),
            ),
        );
        put_in_order(
            &raw mut (*db).head,
            &raw mut database_list,
            Some(cmp_database as unsafe extern "C" fn(*mut List, *mut List) -> ::core::ffi::c_int),
        );
    }
    db
}
#[no_mangle]

pub unsafe extern "C" fn register_auto_database(
    mut name: *const ::core::ffi::c_char,
) -> *mut PgDatabase {
    let mut db = ::core::ptr::null_mut::<PgDatabase>();
    if cf_autodb_connstr.is_null() {
        return ::core::ptr::null_mut::<PgDatabase>();
    }
    if !parse_database(NULL, name, cf_autodb_connstr) {
        return ::core::ptr::null_mut::<PgDatabase>();
    }
    db = find_database(name);
    if !db.is_null() {
        (*db).db_auto = true;
    }
    db
}
#[no_mangle]

pub unsafe extern "C" fn update_global_user_passwd(
    mut user: *mut PgGlobalUser,
    mut passwd: *const ::core::ffi::c_char,
) -> *mut PgGlobalUser {
    passwd = if !passwd.is_null() {
        passwd
    } else {
        c"".as_ptr()
    };
    let mut needed = strlcpy(
        &raw mut (*user).credentials.passwd as *mut ::core::ffi::c_char,
        passwd,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as size_t,
    ) as size_t;
    if (needed >= ::core::mem::size_of::<[::core::ffi::c_char; 2048]>()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            c"bug in %s:%d - string truncated".as_ptr(),
            c"src/objects.c".as_ptr(),
            510 as ::core::ffi::c_int,
        );
    }
    (*user).credentials.dynamic_passwd = strlen(passwd) == 0;
    user
}

unsafe extern "C" fn add_new_global_user(
    mut name: *const ::core::ffi::c_char,
    mut passwd: *const ::core::ffi::c_char,
) -> *mut PgGlobalUser {
    let mut user = slab_alloc(user_cache) as *mut PgGlobalUser;
    if user.is_null() {
        return ::core::ptr::null_mut::<PgGlobalUser>();
    }
    (*user).credentials.global_user = user;
    list_init(&raw mut (*user).head);
    list_init(&raw mut (*user).pool_list);
    let mut needed = strlcpy(
        &raw mut (*user).credentials.name as *mut ::core::ffi::c_char,
        name,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
    ) as size_t;
    if (needed >= ::core::mem::size_of::<[::core::ffi::c_char; 128]>()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            c"bug in %s:%d - string truncated".as_ptr(),
            c"src/objects.c".as_ptr(),
            526 as ::core::ffi::c_int,
        );
    }
    put_in_order(
        &raw mut (*user).head,
        &raw mut user_list,
        Some(cmp_user as unsafe extern "C" fn(*mut List, *mut List) -> ::core::ffi::c_int),
    );
    aatree_insert(
        &raw mut user_tree,
        &raw mut (*user).credentials.name as *mut ::core::ffi::c_char as uintptr_t,
        &raw mut (*user).credentials.tree_node,
    );
    (*user).pool_mode = POOL_INHERIT;
    (*user).pool_size = -(1 as ::core::ffi::c_int);
    (*user).res_pool_size = -(1 as ::core::ffi::c_int);
    update_global_user_passwd(user, passwd)
}
#[no_mangle]

pub unsafe extern "C" fn add_dynamic_credentials(
    mut db: *mut PgDatabase,
    mut name: *const ::core::ffi::c_char,
    mut passwd: *const ::core::ffi::c_char,
) -> *mut PgCredentials {
    let mut credentials = ::core::ptr::null_mut::<PgCredentials>();
    let mut node = ::core::ptr::null_mut::<AANode>();
    node = aatree_search(&raw mut (*db).user_tree, name as uintptr_t);
    credentials = if !node.is_null() {
        (node as *mut ::core::ffi::c_char)
            as *mut PgCredentials
    } else {
        ::core::ptr::null_mut::<PgCredentials>()
    };
    if credentials.is_null() {
        credentials = slab_alloc(credentials_cache) as *mut PgCredentials;
        if credentials.is_null() {
            return ::core::ptr::null_mut::<PgCredentials>();
        }
        let mut needed = strlcpy(
            &raw mut (*credentials).name as *mut ::core::ffi::c_char,
            name,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
        ) as size_t;
        if (needed >= ::core::mem::size_of::<[::core::ffi::c_char; 128]>()) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
        {
            let mut _log_ctx = NULL;
            log_generic(
                LG_WARNING,
                _log_ctx,
                c"bug in %s:%d - string truncated".as_ptr(),
                c"src/objects.c".as_ptr(),
                558 as ::core::ffi::c_int,
            );
        }
        (*credentials).global_user =
            find_or_add_new_global_user(name, ::core::ptr::null::<::core::ffi::c_char>());
        if (*credentials).global_user.is_null() {
            slab_free(credentials_cache, credentials as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<PgCredentials>();
        }
        aatree_insert(
            &raw mut (*db).user_tree,
            &raw mut (*credentials).name as *mut ::core::ffi::c_char as uintptr_t,
            &raw mut (*credentials).tree_node,
        );
    }
    let mut needed_0 = strlcpy(
        &raw mut (*credentials).passwd as *mut ::core::ffi::c_char,
        passwd,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as size_t,
    ) as size_t;
    if (needed_0 >= ::core::mem::size_of::<[::core::ffi::c_char; 2048]>()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_0,
            c"bug in %s:%d - string truncated".as_ptr(),
            c"src/objects.c".as_ptr(),
            569 as ::core::ffi::c_int,
        );
    }
    (*credentials).dynamic_passwd = true;
    credentials
}
#[no_mangle]

pub unsafe extern "C" fn add_pam_credentials(
    mut name: *const ::core::ffi::c_char,
    mut passwd: *const ::core::ffi::c_char,
) -> *mut PgCredentials {
    let mut credentials = ::core::ptr::null_mut::<PgCredentials>();
    let mut node = ::core::ptr::null_mut::<AANode>();
    node = aatree_search(&raw mut pam_user_tree, name as uintptr_t);
    credentials = if !node.is_null() {
        (node as *mut ::core::ffi::c_char)
            as *mut PgCredentials
    } else {
        ::core::ptr::null_mut::<PgCredentials>()
    };
    if credentials.is_null() {
        credentials = slab_alloc(credentials_cache) as *mut PgCredentials;
        if credentials.is_null() {
            return ::core::ptr::null_mut::<PgCredentials>();
        }
        let mut needed = strlcpy(
            &raw mut (*credentials).name as *mut ::core::ffi::c_char,
            name,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
        ) as size_t;
        if (needed >= ::core::mem::size_of::<[::core::ffi::c_char; 128]>()) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
        {
            let mut _log_ctx = NULL;
            log_generic(
                LG_WARNING,
                _log_ctx,
                c"bug in %s:%d - string truncated".as_ptr(),
                c"src/objects.c".as_ptr(),
                589 as ::core::ffi::c_int,
            );
        }
        (*credentials).global_user =
            find_or_add_new_global_user(name, ::core::ptr::null::<::core::ffi::c_char>());
        if (*credentials).global_user.is_null() {
            slab_free(credentials_cache, credentials as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<PgCredentials>();
        }
        aatree_insert(
            &raw mut pam_user_tree,
            &raw mut (*credentials).name as *mut ::core::ffi::c_char as uintptr_t,
            &raw mut (*credentials).tree_node,
        );
    }
    if !passwd.is_null() {
        let mut needed_0 = strlcpy(
            &raw mut (*credentials).passwd as *mut ::core::ffi::c_char,
            passwd,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as size_t,
        ) as size_t;
        if (needed_0 >= ::core::mem::size_of::<[::core::ffi::c_char; 2048]>()) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
        {
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_WARNING,
                _log_ctx_0,
                c"bug in %s:%d - string truncated".as_ptr(),
                c"src/objects.c".as_ptr(),
                600 as ::core::ffi::c_int,
            );
        }
    }
    credentials
}
#[no_mangle]

pub unsafe extern "C" fn force_user_credentials(
    mut db: *mut PgDatabase,
    mut name: *const ::core::ffi::c_char,
    mut passwd: *const ::core::ffi::c_char,
) -> *mut PgCredentials {
    let mut credentials = (*db).forced_user_credentials;
    if credentials.is_null() {
        credentials = slab_alloc(credentials_cache) as *mut PgCredentials;
        if credentials.is_null() {
            return ::core::ptr::null_mut::<PgCredentials>();
        }
        (*credentials).global_user =
            find_or_add_new_global_user(name, ::core::ptr::null::<::core::ffi::c_char>());
        if (*credentials).global_user.is_null() {
            slab_free(credentials_cache, credentials as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<PgCredentials>();
        }
    }
    let mut needed = strlcpy(
        &raw mut (*credentials).name as *mut ::core::ffi::c_char,
        name,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
    ) as size_t;
    if (needed >= ::core::mem::size_of::<[::core::ffi::c_char; 128]>()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            c"bug in %s:%d - string truncated".as_ptr(),
            c"src/objects.c".as_ptr(),
            619 as ::core::ffi::c_int,
        );
    }
    let mut needed_0 = strlcpy(
        &raw mut (*credentials).passwd as *mut ::core::ffi::c_char,
        passwd,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as size_t,
    ) as size_t;
    if (needed_0 >= ::core::mem::size_of::<[::core::ffi::c_char; 2048]>()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_0,
            c"bug in %s:%d - string truncated".as_ptr(),
            c"src/objects.c".as_ptr(),
            620 as ::core::ffi::c_int,
        );
    }
    (*db).forced_user_credentials = credentials;
    credentials
}
#[no_mangle]

pub unsafe extern "C" fn find_peer(mut peer_id: ::core::ffi::c_int) -> *mut PgDatabase {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut peer = ::core::ptr::null_mut::<PgDatabase>();
    item = peer_list.head.next;
    while item != &raw mut peer_list.head {
        peer = (item as *mut ::core::ffi::c_char)
            as *mut PgDatabase;
        if (*peer).peer_id == peer_id {
            return peer;
        }
        item = (*item).next;
    }
    ::core::ptr::null_mut::<PgDatabase>()
}
#[no_mangle]

pub unsafe extern "C" fn find_database(mut name: *const ::core::ffi::c_char) -> *mut PgDatabase {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut db = ::core::ptr::null_mut::<PgDatabase>();
    item = database_list.head.next;
    while item != &raw mut database_list.head {
        db = (item as *mut ::core::ffi::c_char)
            as *mut PgDatabase;
        if strcmp(&raw mut (*db).name as *mut ::core::ffi::c_char, name) == 0 as ::core::ffi::c_int
        {
            return db;
        }
        item = (*item).next;
    }
    item = autodatabase_idle_list.head.next;
    tmp = (*autodatabase_idle_list.head.next).next;
    while item != &raw mut autodatabase_idle_list.head {
        db = (item as *mut ::core::ffi::c_char)
            as *mut PgDatabase;
        if strcmp(&raw mut (*db).name as *mut ::core::ffi::c_char, name) == 0 as ::core::ffi::c_int
        {
            (*db).inactive_time = 0;
            statlist_remove(&raw mut autodatabase_idle_list, &raw mut (*db).head);
            put_in_order(
                &raw mut (*db).head,
                &raw mut database_list,
                Some(
                    cmp_database
                        as unsafe extern "C" fn(*mut List, *mut List) -> ::core::ffi::c_int,
                ),
            );
            return db;
        }
        item = tmp;
        tmp = (*tmp).next;
    }
    ::core::ptr::null_mut::<PgDatabase>()
}
#[no_mangle]

pub unsafe extern "C" fn find_or_register_database(
    mut connection: *mut PgSocket,
    mut name: *const ::core::ffi::c_char,
) -> *mut PgDatabase {
    let mut db = find_database(name);
    if db.is_null() {
        db = register_auto_database(name);
        if !db.is_null() {
            log_generic(
                LG_INFO,
                connection as *mut ::core::ffi::c_void,
                c"registered new auto-database: %s".as_ptr(),
                name,
            );
        }
    }
    db
}
#[no_mangle]

pub unsafe extern "C" fn find_global_user(
    mut name: *const ::core::ffi::c_char,
) -> *mut PgGlobalUser {
    let mut user = ::core::ptr::null_mut::<PgGlobalUser>();
    let mut node = ::core::ptr::null_mut::<AANode>();
    node = aatree_search(&raw mut user_tree, name as uintptr_t);
    user = if !node.is_null() {
        (node as *mut ::core::ffi::c_char)
            as *mut PgCredentials as *mut PgGlobalUser
    } else {
        ::core::ptr::null_mut::<PgGlobalUser>()
    };
    user
}
#[no_mangle]

pub unsafe extern "C" fn find_global_credentials(
    mut name: *const ::core::ffi::c_char,
) -> *mut PgCredentials {
    let mut user = find_global_user(name);
    if user.is_null() {
        return ::core::ptr::null_mut::<PgCredentials>();
    }
    &raw mut (*user).credentials
}

unsafe extern "C" fn new_pool(
    mut db: *mut PgDatabase,
    mut user_credentials: *mut PgCredentials,
) -> *mut PgPool {
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    pool = slab_alloc(pool_cache) as *mut PgPool;
    if pool.is_null() {
        return ::core::ptr::null_mut::<PgPool>();
    }
    list_init(&raw mut (*pool).head);
    list_init(&raw mut (*pool).map_head);
    (*pool).orig_vars.var_list = slab_alloc(var_list_cache) as *mut *mut PStr;
    (*pool).user_credentials = user_credentials;
    (*pool).db = db;
    statlist_init(
        &raw mut (*pool).active_client_list,
        c"active_client_list".as_ptr(),
    );
    statlist_init(
        &raw mut (*pool).waiting_client_list,
        c"waiting_client_list".as_ptr(),
    );
    statlist_init(
        &raw mut (*pool).active_server_list,
        c"active_server_list".as_ptr(),
    );
    statlist_init(
        &raw mut (*pool).idle_server_list,
        c"idle_server_list".as_ptr(),
    );
    statlist_init(
        &raw mut (*pool).tested_server_list,
        c"tested_server_list".as_ptr(),
    );
    statlist_init(
        &raw mut (*pool).used_server_list,
        c"used_server_list".as_ptr(),
    );
    statlist_init(
        &raw mut (*pool).new_server_list,
        c"new_server_list".as_ptr(),
    );
    statlist_init(
        &raw mut (*pool).waiting_cancel_req_list,
        c"waiting_cancel_req_list".as_ptr(),
    );
    statlist_init(
        &raw mut (*pool).active_cancel_req_list,
        c"active_cancel_req_list".as_ptr(),
    );
    statlist_init(
        &raw mut (*pool).active_cancel_server_list,
        c"active_cancel_server_list".as_ptr(),
    );
    statlist_init(
        &raw mut (*pool).being_canceled_server_list,
        c"being_canceled_server_list".as_ptr(),
    );
    list_append(
        &raw mut (*(*user_credentials).global_user).pool_list,
        &raw mut (*pool).map_head,
    );
    put_in_order(
        &raw mut (*pool).head,
        &raw mut pool_list,
        Some(cmp_pool as unsafe extern "C" fn(*mut List, *mut List) -> ::core::ffi::c_int),
    );
    pool
}

unsafe extern "C" fn new_peer_pool(mut db: *mut PgDatabase) -> *mut PgPool {
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    pool = slab_alloc(peer_pool_cache) as *mut PgPool;
    if pool.is_null() {
        return ::core::ptr::null_mut::<PgPool>();
    }
    list_init(&raw mut (*pool).head);
    list_init(&raw mut (*pool).map_head);
    (*pool).orig_vars.var_list = slab_alloc(var_list_cache) as *mut *mut PStr;
    (*pool).db = db;
    statlist_init(
        &raw mut (*pool).new_server_list,
        c"new_server_list".as_ptr(),
    );
    statlist_init(
        &raw mut (*pool).waiting_cancel_req_list,
        c"waiting_cancel_req_list".as_ptr(),
    );
    statlist_init(
        &raw mut (*pool).active_cancel_req_list,
        c"active_cancel_req_list".as_ptr(),
    );
    statlist_init(
        &raw mut (*pool).active_cancel_server_list,
        c"active_cancel_server_list".as_ptr(),
    );
    put_in_order(
        &raw mut (*pool).head,
        &raw mut peer_pool_list,
        Some(cmp_peer_pool as unsafe extern "C" fn(*mut List, *mut List) -> ::core::ffi::c_int),
    );
    pool
}
#[no_mangle]

pub unsafe extern "C" fn get_pool(
    mut db: *mut PgDatabase,
    mut user_credentials: *mut PgCredentials,
) -> *mut PgPool {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    if db.is_null() || user_credentials.is_null() {
        return ::core::ptr::null_mut::<PgPool>();
    }
    item = (*(*user_credentials).global_user).pool_list.next;
    while item != &raw mut (*(*user_credentials).global_user).pool_list {
        pool = (item as *mut ::core::ffi::c_char).offset(-(16 as ::core::ffi::c_ulong as isize))
            as *mut PgPool;
        if (*pool).db == db {
            return pool;
        }
        item = (*item).next;
    }
    new_pool(db, user_credentials)
}
#[no_mangle]

pub unsafe extern "C" fn get_peer_pool(mut db: *mut PgDatabase) -> *mut PgPool {
    if db.is_null() {
        return ::core::ptr::null_mut::<PgPool>();
    }
    if (*db).pool.is_null() {
        (*db).pool = new_peer_pool(db) as *mut PgPool;
    }
    (*db).pool as *mut PgPool
}

unsafe extern "C" fn pause_client(mut client: *mut PgSocket) {
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            client as *mut ::core::ffi::c_void,
            c"pause_client".as_ptr(),
        );
    }
    if cf_shutdown == SHUTDOWN_WAIT_FOR_SERVERS as ::core::ffi::c_int {
        disconnect_client(client, true, c"server shutting down".as_ptr());
        return;
    }
    change_client_state(client, CL_WAITING);
    if !sbuf_pause(&raw mut (*client).sbuf) {
        disconnect_client(client, true, c"pause failed".as_ptr());
    }
}

unsafe extern "C" fn pause_cancel_request(mut client: *mut PgSocket) {
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            client as *mut ::core::ffi::c_void,
            c"pause_cancel_request".as_ptr(),
        );
    }
    change_client_state(client, CL_WAITING_CANCEL);
    if !sbuf_pause(&raw mut (*client).sbuf) {
        disconnect_client(client, true, c"pause cancel request failed".as_ptr());
    }
}
#[no_mangle]

pub unsafe extern "C" fn activate_client(mut client: *mut PgSocket) {
    (*(*client).pool).stats.wait_time = (*(*client).pool)
        .stats
        .wait_time
        .wrapping_add(get_cached_time().wrapping_sub((*client).wait_start));
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            client as *mut ::core::ffi::c_void,
            c"activate_client".as_ptr(),
        );
    }
    change_client_state(client, CL_ACTIVE);
    sbuf_continue(&raw mut (*client).sbuf);
}
#[no_mangle]

pub unsafe extern "C" fn check_fast_fail(mut client: *mut PgSocket) -> bool {
    let mut cnt: ::core::ffi::c_int = 0;
    let mut pool = (*client).pool;
    if pool.is_null() {
        return true;
    }
    if !(*pool).last_login_failed() {
        return true;
    }
    cnt = statlist_count(&raw mut (*pool).active_server_list)
        + statlist_count(&raw mut (*pool).being_canceled_server_list)
        + statlist_count(&raw mut (*pool).idle_server_list)
        + statlist_count(&raw mut (*pool).tested_server_list)
        + statlist_count(&raw mut (*pool).used_server_list)
        + statlist_count(&raw mut (*pool).new_server_list)
        + statlist_count(&raw mut (*pool).active_cancel_server_list)
        - statlist_count(&raw mut (*pool).new_server_list);
    if cnt != 0 {
        return true;
    }
    disconnect_client(
        client,
        true,
        c"server login has been failing, cached error: %s (server_login_retry)".as_ptr(),
        &raw mut (*pool).last_connect_failed_message as *mut ::core::ffi::c_char,
    );
    launch_new_connection(pool, true);
    false
}
#[no_mangle]

pub unsafe extern "C" fn find_server(mut client: *mut PgSocket) -> bool {
    let mut pool = (*client).pool;
    let mut server = ::core::ptr::null_mut::<PgSocket>();
    let mut res: bool = false;
    let mut varchange = false;
    (*client).wait_start = 0;
    if !(*client).link.is_null() {
        return true;
    }
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(
            LG_NOISE,
            client as *mut ::core::ffi::c_void,
            c"find_server: client had no linked server yet".as_ptr(),
        );
    }
    if cf_pause_mode == P_PAUSE as ::core::ffi::c_int
        || (*(*pool).db).db_paused as ::core::ffi::c_int != 0
    {
        server = ::core::ptr::null_mut::<PgSocket>();
    } else if (*client).replication as ::core::ffi::c_uint != 0 && !sending_auth_query(client) {
        launch_new_connection(pool, true);
        server = ::core::ptr::null_mut::<PgSocket>();
    } else {
        loop {
            server = first_socket(&raw mut (*pool).idle_server_list);
            if server.is_null() {
                break;
            }
            if (*server).close_needed() {
                disconnect_server(server, true, c"obsolete connection".as_ptr());
            } else {
                if (*server).ready() {
                    break;
                }
                disconnect_server(server, true, c"idle server got dirty".as_ptr());
            }
        }
        if server.is_null() && !check_fast_fail(client) {
            return false;
        }
    }
    if !server.is_null() && !sending_auth_query(client) {
        res = varcache_apply(server, client, &raw mut varchange);
        if !res {
            disconnect_server(server, true, c"var change failed".as_ptr());
            server = ::core::ptr::null_mut::<PgSocket>();
        }
    }
    if !server.is_null() {
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(
                LG_NOISE,
                client as *mut ::core::ffi::c_void,
                c"linking client to S-%p".as_ptr(),
                server,
            );
        }
        (*client).link = server;
        (*server).link = client;
        (*(*server).pool).stats.server_assignment_count = (*(*server).pool)
            .stats
            .server_assignment_count
            .wrapping_add(1);
        change_server_state(server, SV_ACTIVE);
        if varchange {
            (*server).set_setting_vars(true);
            (*server).set_ready(false);
            res = false;
            if cf_verbose > 1 as ::core::ffi::c_int
            {
                log_generic(
                    LG_NOISE,
                    client as *mut ::core::ffi::c_void,
                    c"pausing client while applying vars".as_ptr(),
                );
            }
            if !sbuf_pause(&raw mut (*client).sbuf) {
                disconnect_client(client, true, c"pause failed".as_ptr());
            }
        } else {
            res = true;
        }
    } else {
        pause_client(client);
        res = false;
    }
    res
}

unsafe extern "C" fn reuse_on_release(mut server: *mut PgSocket) -> bool {
    let mut res = true;
    let mut pool = (*server).pool;
    let mut client = ::core::ptr::null_mut::<PgSocket>();
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            server as *mut ::core::ffi::c_void,
            c"reuse_on_release: replication %d".as_ptr(),
            (*server).replication as ::core::ffi::c_uint,
        );
    }
    client = first_socket(&raw mut (*pool).waiting_client_list);
    if !client.is_null()
        && ((*client).replication as u64 == 0
            || sending_auth_query(client) as ::core::ffi::c_int != 0)
    {
        activate_client(client);
        if (*server).state() as ::core::ffi::c_int == SV_FREE as ::core::ffi::c_int
            || (*server).state() as ::core::ffi::c_int == SV_JUSTFREE as ::core::ffi::c_int
        {
            res = false;
        }
    }
    res
}
#[no_mangle]

pub unsafe extern "C" fn queue_fake_response(
    mut client: *mut PgSocket,
    mut request_type: ::core::ffi::c_char,
) -> bool {
    let mut res = true;
    let mut server = (*client).link;
    if request_type as ::core::ffi::c_int == PqMsg_Parse {
        if cf_verbose > 0 as ::core::ffi::c_int
        {
            log_generic(
                LG_DEBUG,
                client as *mut ::core::ffi::c_void,
                c"Queuing fake ParseComplete packet".as_ptr(),
            );
        }
        let mut _data: [uint8_t; 5] = [0; 5];
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
            ::core::mem::size_of::<[uint8_t; 5]>() as ::core::ffi::c_int,
        );
        pktbuf_write_generic(&raw mut _buf, PqMsg_ParseComplete, c"".as_ptr());
        res = sbuf_queue_packet(
            &raw mut (*server).sbuf,
            &raw mut (*client).sbuf,
            &raw mut _buf,
        );
    } else if request_type as ::core::ffi::c_int == PqMsg_Close {
        if cf_verbose > 0 as ::core::ffi::c_int
        {
            log_generic(
                LG_DEBUG,
                client as *mut ::core::ffi::c_void,
                c"Queuing fake CloseComplete packet".as_ptr(),
            );
        }
        let mut _data_0: [uint8_t; 5] = [0; 5];
        let mut _buf_0 = PktBuf {
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
            &raw mut _buf_0,
            &raw mut _data_0 as *mut uint8_t,
            ::core::mem::size_of::<[uint8_t; 5]>() as ::core::ffi::c_int,
        );
        pktbuf_write_generic(&raw mut _buf_0, PqMsg_CloseComplete, c"".as_ptr());
        res = sbuf_queue_packet(
            &raw mut (*server).sbuf,
            &raw mut (*client).sbuf,
            &raw mut _buf_0,
        );
    } else {
        let mut _log_ctx = NULL;
        log_fatal(
            c"src/objects.c".as_ptr(),
            1021 as ::core::ffi::c_int,
            c"queue_fake_response".as_ptr(),
            false,
            _log_ctx,
            c"Unknown fake request type %c".as_ptr(),
            request_type as ::core::ffi::c_int,
        );
        exit(1 as ::core::ffi::c_int);
    }
    res
}
#[no_mangle]

pub unsafe extern "C" fn find_or_add_new_global_user(
    mut name: *const ::core::ffi::c_char,
    mut passwd: *const ::core::ffi::c_char,
) -> *mut PgGlobalUser {
    let mut user = find_global_user(name);
    if user.is_null() {
        user = add_new_global_user(name, passwd);
    }
    user
}
#[no_mangle]

pub unsafe extern "C" fn find_or_add_new_global_credentials(
    mut name: *const ::core::ffi::c_char,
    mut passwd: *const ::core::ffi::c_char,
) -> *mut PgCredentials {
    let mut user = find_or_add_new_global_user(name, passwd);
    if user.is_null() {
        return ::core::ptr::null_mut::<PgCredentials>();
    }
    &raw mut (*user).credentials
}
#[no_mangle]

pub unsafe extern "C" fn add_outstanding_request(
    mut client: *mut PgSocket,
    mut type_0: ::core::ffi::c_char,
    mut action: ResponseAction,
) -> bool {
    let mut request = ::core::ptr::null_mut::<OutstandingRequest>();
    let mut server = (*client).link;
    if action as ::core::ffi::c_uint == RA_FAKE as ::core::ffi::c_int as ::core::ffi::c_uint
        && statlist_empty(&raw mut (*server).outstanding_requests) as ::core::ffi::c_int != 0
    {
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(
                LG_NOISE,
                client as *mut ::core::ffi::c_void,
                c"add_outstanding_request: queueing fake response right away %c".as_ptr(),
                type_0 as ::core::ffi::c_int,
            );
        }
        return queue_fake_response(client, type_0);
    }
    request = slab_alloc(outstanding_request_cache) as *mut OutstandingRequest;
    if request.is_null() {
        return false;
    }
    (*request).type_0 = type_0;
    (*request).action = action;
    statlist_append(
        &raw mut (*server).outstanding_requests,
        &raw mut (*request).node,
    );
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(
            LG_NOISE,
            client as *mut ::core::ffi::c_void,
            c"add_outstanding_request: added %c, still outstanding %d".as_ptr(),
            type_0 as ::core::ffi::c_int,
            statlist_count(&raw mut (*(*client).link).outstanding_requests),
        );
    }
    true
}
#[no_mangle]

pub unsafe extern "C" fn pop_outstanding_request(
    mut server: *mut PgSocket,
    mut types: *const ::core::ffi::c_char,
    mut skip: *mut bool,
) -> bool {
    let mut request = ::core::ptr::null_mut::<OutstandingRequest>();
    let mut item = statlist_first(&raw mut (*server).outstanding_requests);
    if item.is_null() {
        return false;
    }
    request = (item as *mut ::core::ffi::c_char)
        as *mut OutstandingRequest;
    if (*request).action as ::core::ffi::c_uint
        == RA_FAKE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        log_generic(
            LG_WARNING,
            server as *mut ::core::ffi::c_void,
            c"pop_outstanding_request: unexpected fake request of type %c".as_ptr(),
            (*request).type_0 as ::core::ffi::c_int,
        );
        return false;
    }
    if strchr(
        types as *const ::core::ffi::c_char,
        (*request).type_0 as ::core::ffi::c_int,
    )
    .is_null()
    {
        return false;
    }
    statlist_pop(&raw mut (*server).outstanding_requests);
    if !skip.is_null() {
        *skip = (*request).action as ::core::ffi::c_uint
            == RA_SKIP as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(
            LG_NOISE,
            server as *mut ::core::ffi::c_void,
            c"pop_outstanding_request: popped %c, still outstanding %d, skip %d".as_ptr(),
            (*request).type_0 as ::core::ffi::c_int,
            statlist_count(&raw mut (*server).outstanding_requests),
            ((*request).action as ::core::ffi::c_uint
                == RA_SKIP as ::core::ffi::c_int as ::core::ffi::c_uint)
                as ::core::ffi::c_int,
        );
    }
    if !(*request).server_ps.is_null() {
        free_server_prepared_statement((*request).server_ps);
    }
    slab_free(
        outstanding_request_cache,
        request as *mut ::core::ffi::c_void,
    );
    true
}
#[no_mangle]

pub unsafe extern "C" fn clear_outstanding_requests_until(
    mut server: *mut PgSocket,
    mut types: *const ::core::ffi::c_char,
) -> bool {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    item = (*server).outstanding_requests.head.next;
    tmp = (*(*server).outstanding_requests.head.next).next;
    while item != &raw mut (*server).outstanding_requests.head {
        let mut request = (item as *mut ::core::ffi::c_char)
            
            as *mut OutstandingRequest;
        let mut type_0 = (*request).type_0;
        if type_0 as ::core::ffi::c_int == PqMsg_Parse && (*request).server_ps_query_id > 0 {
            unregister_prepared_statement(server, (*request).server_ps_query_id);
            if cf_verbose > 1 as ::core::ffi::c_int
            {
                log_generic(
                    LG_NOISE,
                    server as *mut ::core::ffi::c_void,
                    c"failed prepared statement 'PGBOUNCER_%llu' removed from server cache, %d cached items".as_ptr(),
                    (*request).server_ps_query_id,
                    if !(*server).server_prepared_statements.is_null() {
                        (*(*(*server).server_prepared_statements).hh.tbl).num_items
                    } else {
                        0 as ::core::ffi::c_uint
                    },
                );
            }
        } else if type_0 as ::core::ffi::c_int == PqMsg_Close && !(*request).server_ps.is_null() {
            if !add_prepared_statement(server, (*request).server_ps) {
                if !(*server).link.is_null() {
                    disconnect_client((*server).link, true, c"out of memory".as_ptr());
                }
                disconnect_server(server, true, c"out of memory".as_ptr());
                return false;
            }
            if cf_verbose > 1 as ::core::ffi::c_int
            {
                log_generic(
                    LG_NOISE,
                    server as *mut ::core::ffi::c_void,
                    c"prepared statement '%s' added back to server cache, %d cached items".as_ptr(),
                    &raw mut (*(*(*request).server_ps).ps).stmt_name as *mut ::core::ffi::c_char,
                    if !(*server).server_prepared_statements.is_null() {
                        (*(*(*server).server_prepared_statements).hh.tbl).num_items
                    } else {
                        0 as ::core::ffi::c_uint
                    },
                );
            }
        }
        statlist_remove(&raw mut (*server).outstanding_requests, item);
        slab_free(
            outstanding_request_cache,
            request as *mut ::core::ffi::c_void,
        );
        if !strchr(
            types as *const ::core::ffi::c_char,
            type_0 as ::core::ffi::c_int,
        )
        .is_null()
        {
            break;
        }
        item = tmp;
        tmp = (*tmp).next;
    }
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(
            LG_NOISE,
            server as *mut ::core::ffi::c_void,
            c"clear_outstanding_requests_until_sync: still outstanding %d".as_ptr(),
            statlist_count(&raw mut (*server).outstanding_requests),
        );
    }
    true
}

unsafe extern "C" fn reset_on_release(mut server: *mut PgSocket) -> bool {
    let mut res: bool = false;
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            server as *mut ::core::ffi::c_void,
            c"resetting: %s".as_ptr(),
            cf_server_reset_query,
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
        c"s".as_ptr(),
        cf_server_reset_query,
    );
    res = pktbuf_send_immediate(&raw mut _buf, server);
    if !res {
        disconnect_server(server, false, c"reset query failed".as_ptr());
    }
    res
}
#[no_mangle]

pub unsafe extern "C" fn life_over(mut server: *mut PgSocket) -> bool {
    let mut pool = (*server).pool;
    let mut lifetime_kill_gap: usec_t = 0;
    let mut now = get_cached_time();
    let mut age: usec_t = now.wrapping_sub((*server).connect_time);
    let mut last_kill: usec_t = now.wrapping_sub((*pool).last_lifetime_disconnect);
    let mut server_lifetime = pool_server_lifetime(pool);
    if age < server_lifetime {
        return false;
    }
    if pool_pool_size(pool) > 0 as ::core::ffi::c_int {
        lifetime_kill_gap = server_lifetime.wrapping_div(pool_pool_size(pool) as usec_t);
    }
    if last_kill >= lifetime_kill_gap {
        return true;
    }
    false
}
#[no_mangle]

pub unsafe extern "C" fn release_server(mut server: *mut PgSocket) -> bool {
    let mut pool = (*server).pool;
    let mut newstate = SV_IDLE;
    let mut cancel_item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    match (*server).state() as ::core::ffi::c_int {
        11 | 13 => {
            if !(*server).link.is_null() {
                (*(*server).link).link = ::core::ptr::null_mut::<PgSocket>();
                (*server).link = ::core::ptr::null_mut::<PgSocket>();
            }
            if *cf_server_reset_query as ::core::ffi::c_int != 0
                && (cf_server_reset_query_always != 0
                    || connection_pool_mode(server) == POOL_SESSION)
            {
                newstate = SV_TESTED;
            } else if cf_server_check_delay == 0
                && *cf_server_check_query as ::core::ffi::c_int != 0
            {
                newstate = SV_USED;
            }
        }
        15 | 16 => {}
        10 => {
            (*pool).set_last_login_failed(false);
            (*pool).set_last_connect_failed(false);
        }
        _ => {
            let mut _log_ctx = NULL;
            log_fatal(
                c"src/objects.c".as_ptr(),
                1241 as ::core::ffi::c_int,
                c"release_server".as_ptr(),
                false,
                _log_ctx,
                c"bad server state: %d".as_ptr(),
                (*server).state() as ::core::ffi::c_int,
            );
            exit(1 as ::core::ffi::c_int);
        }
    }
    cancel_item = (*server).canceling_clients.head.next;
    tmp = (*(*server).canceling_clients.head.next).next;
    while cancel_item != &raw mut (*server).canceling_clients.head {
        let mut cancel_client = (cancel_item as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize))
            as *mut PgSocket;
        if (*cancel_client).state() as ::core::ffi::c_int == CL_WAITING_CANCEL as ::core::ffi::c_int
        {
            (*cancel_client).canceled_server = ::core::ptr::null_mut::<PgSocket>();
            statlist_remove(&raw mut (*server).canceling_clients, cancel_item);
        }
        cancel_item = tmp;
        tmp = (*tmp).next;
    }
    if (*server).state() as ::core::ffi::c_int != SV_LOGIN as ::core::ffi::c_int
        && life_over(server) as ::core::ffi::c_int != 0
    {
        disconnect_server(server, true, c"server lifetime over".as_ptr());
        (*pool).last_lifetime_disconnect = get_cached_time();
        return false;
    }
    if statlist_count(&raw mut (*server).outstanding_requests) > 0 as ::core::ffi::c_int {
        disconnect_server(
            server,
            true,
            c"client disconnected with queries in progress".as_ptr(),
        );
        return true;
    }
    if (*server).close_needed() {
        disconnect_server(server, true, c"close_needed".as_ptr());
        return false;
    }
    if statlist_count(&raw mut (*server).canceling_clients) > 0 as ::core::ffi::c_int {
        change_server_state(server, SV_BEING_CANCELED);
        return true;
    }
    if (*server).replication as u64 != 0 {
        if !(*server).link.is_null() {
            if cf_verbose > 0 as ::core::ffi::c_int
            {
                log_generic(
                    LG_DEBUG,
                    server as *mut ::core::ffi::c_void,
                    c"release_server: new replication connection ready".as_ptr(),
                );
            }
            change_server_state(server, SV_ACTIVE);
            activate_client((*server).link);
            return true;
        } else {
            disconnect_server(server, true, c"replication client was closed".as_ptr());
            return false;
        }
    }
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(
            LG_NOISE,
            server as *mut ::core::ffi::c_void,
            c"release_server: new state=%d".as_ptr(),
            newstate as ::core::ffi::c_uint,
        );
    }
    change_server_state(server, newstate);
    if newstate as ::core::ffi::c_uint == SV_IDLE as ::core::ffi::c_int as ::core::ffi::c_uint {
        return reuse_on_release(server);
    } else if newstate as ::core::ffi::c_uint
        == SV_TESTED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return reset_on_release(server);
    }
    true
}

unsafe extern "C" fn unlink_server(
    mut server: *mut PgSocket,
    mut reason: *const ::core::ffi::c_char,
) {
    let mut client = ::core::ptr::null_mut::<PgSocket>();
    if (*server).link.is_null() {
        return;
    }
    client = (*server).link;
    (*client).link = ::core::ptr::null_mut::<PgSocket>();
    (*server).link = ::core::ptr::null_mut::<PgSocket>();
    if (*client).state() as ::core::ffi::c_int == CL_ACTIVE as ::core::ffi::c_int
        || (*client).state() as ::core::ffi::c_int == CL_WAITING as ::core::ffi::c_int
    {
        disconnect_client(client, true, c"%s".as_ptr(), reason);
    } else if (*client).state() as ::core::ffi::c_int == CL_ACTIVE_CANCEL as ::core::ffi::c_int {
        disconnect_client(client, false, c"successfully sent cancel request".as_ptr());
    } else {
        disconnect_client(client, true, c"bouncer config error".as_ptr());
    };
}
#[no_mangle]

pub unsafe extern "C" fn disconnect_server(
    mut server: *mut PgSocket,
    mut send_term: bool,
    mut reason: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut now = get_cached_time();
    let mut buf: [::core::ffi::c_char; 128] = [0; 128];
    let mut ap: ::core::ffi::VaListImpl;
    let mut cancel_item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    if server.is_null() {
        return;
    }
    ap = args.clone();
    vsnprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
        reason,
        ap.as_va_list(),
    );
    reason = &raw mut buf as *mut ::core::ffi::c_char;
    if cf_log_disconnections != 0 {
        log_generic(
            LG_INFO,
            server as *mut ::core::ffi::c_void,
            c"closing because: %s (age=%llus)".as_ptr(),
            reason,
            now.wrapping_sub((*server).connect_time).wrapping_div(USEC),
        );
    }
    match (*server).state() as ::core::ffi::c_int {
        14 | 13 => {
            unlink_server(server, reason);
        }
        16 | 15 | 12 | 11 => {}
        10 => {
            if !(*server).ready() {
                (*(*server).pool).set_last_login_failed(true);
                (*(*server).pool).set_last_connect_failed(true);
                let mut needed = strlcpy(
                    &raw mut (*(*server).pool).last_connect_failed_message
                        as *mut ::core::ffi::c_char,
                    reason,
                    ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as size_t,
                ) as size_t;
                if (needed >= ::core::mem::size_of::<[::core::ffi::c_char; 100]>())
                    as ::core::ffi::c_int as ::core::ffi::c_long
                    != 0
                {
                    let mut _log_ctx = NULL;
                    log_generic(
                        LG_WARNING,
                        _log_ctx,
                        c"bug in %s:%d - string truncated".as_ptr(),
                        c"src/objects.c".as_ptr(),
                        1383 as ::core::ffi::c_int,
                    );
                }
            } else {
                (*(*server).pool).set_last_connect_failed(false);
                send_term = false;
            }
            if (*server).replication as u64 != 0 {
                unlink_server(server, reason);
            }
        }
        _ => {
            let mut _log_ctx_0 = NULL;
            log_fatal(
                c"src/objects.c".as_ptr(),
                1398 as ::core::ffi::c_int,
                c"disconnect_server".as_ptr(),
                false,
                _log_ctx_0,
                c"bad server state: %d, %s".as_ptr(),
                (*server).state() as ::core::ffi::c_int,
                reason,
            );
            exit(1 as ::core::ffi::c_int);
        }
    }
    cancel_item = (*server).canceling_clients.head.next;
    tmp = (*(*server).canceling_clients.head.next).next;
    while cancel_item != &raw mut (*server).canceling_clients.head {
        let mut cancel_client = (cancel_item as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize))
            as *mut PgSocket;
        (*cancel_client).canceled_server = ::core::ptr::null_mut::<PgSocket>();
        statlist_remove(&raw mut (*server).canceling_clients, cancel_item);
        cancel_item = tmp;
        tmp = (*tmp).next;
    }
    if send_term {
        static mut pkt_term: [uint8_t; 5] = [
            PqMsg_Terminate as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
        ];
        let mut _ignore = sbuf_answer(
            &raw mut (*server).sbuf,
            &raw const pkt_term as *const uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 5]>() as size_t,
        );
    }
    if !(*server).c2rust_unnamed.dns_token.is_null() {
        adns_cancel(adns, (*server).c2rust_unnamed.dns_token);
        (*server).c2rust_unnamed.dns_token = ::core::ptr::null_mut::<DNSToken>();
    }
    free_scram_state(&raw mut (*server).scram_state);
    (*(*(*server).pool).db).connection_count -= 1;
    if !(*(*server).pool).user_credentials.is_null() {
        (*(*(*(*server).pool).user_credentials).global_user).connection_count -= 1;
    }
    change_server_state(server, SV_JUSTFREE);
    if !sbuf_close(&raw mut (*server).sbuf) {
        let mut _log_ctx_1 = NULL;
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(
                LG_NOISE,
                _log_ctx_1,
                c"sbuf_close failed, retry later".as_ptr(),
            );
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn disconnect_client(
    mut client: *mut PgSocket,
    mut notify: bool,
    mut reason: *const ::core::ffi::c_char,
    mut args: ...
) {
    if !reason.is_null() {
        let mut buf: [::core::ffi::c_char; 128] = [0; 128];
        let mut ap: ::core::ffi::VaListImpl;
        ap = args.clone();
        vsnprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
            reason,
            ap.as_va_list(),
        );
        disconnect_client_sqlstate(
            client,
            notify,
            ::core::ptr::null::<::core::ffi::c_char>(),
            &raw mut buf as *mut ::core::ffi::c_char,
        );
    } else {
        disconnect_client_sqlstate(
            client,
            notify,
            ::core::ptr::null::<::core::ffi::c_char>(),
            reason,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn disconnect_client_sqlstate(
    mut client: *mut PgSocket,
    mut notify: bool,
    mut sqlstate: *const ::core::ffi::c_char,
    mut reason: *const ::core::ffi::c_char,
) {
    let mut now = get_cached_time();
    if !(*client).c2rust_unnamed.db.is_null()
        && (*client).contributes_db_client_count() as ::core::ffi::c_int != 0
    {
        (*(*client).c2rust_unnamed.db).client_connection_count -= 1;
    }
    if !(*client).login_user_credentials.is_null()
        && !(*(*client).login_user_credentials).global_user.is_null()
        && (*client).user_connection_counted() as ::core::ffi::c_int != 0
    {
        (*(*(*client).login_user_credentials).global_user).client_connection_count -= 1;
    }
    if cf_log_disconnections != 0 && !reason.is_null() {
        log_generic(
            LG_INFO,
            client as *mut ::core::ffi::c_void,
            c"closing because: %s (age=%llus)".as_ptr(),
            reason,
            now.wrapping_sub((*client).connect_time).wrapping_div(USEC),
        );
    }
    match (*client).state() as ::core::ffi::c_int {
        5 | 2 => {
            if !(*client).link.is_null() {
                let mut server = (*client).link;
                if !(*server).ready() {
                    (*server).link = ::core::ptr::null_mut::<PgSocket>();
                    (*client).link = ::core::ptr::null_mut::<PgSocket>();
                    disconnect_server(
                        server,
                        true,
                        c"client disconnect while server was not ready".as_ptr(),
                    );
                } else if statlist_count(&raw mut (*server).outstanding_requests)
                    > 0 as ::core::ffi::c_int
                {
                    (*server).link = ::core::ptr::null_mut::<PgSocket>();
                    (*client).link = ::core::ptr::null_mut::<PgSocket>();
                    disconnect_server(
                        server,
                        true,
                        c"client disconnected with query in progress".as_ptr(),
                    );
                } else if !sbuf_is_empty(&raw mut (*server).sbuf) {
                    (*server).link = ::core::ptr::null_mut::<PgSocket>();
                    (*client).link = ::core::ptr::null_mut::<PgSocket>();
                    disconnect_server(
                        server,
                        true,
                        c"client disconnect before everything was sent to the server".as_ptr(),
                    );
                } else {
                    release_server(server);
                }
            }
        }
        7 | 6 => {
            if !(*client).link.is_null() {
                let mut server_0 = (*client).link;
                (*server_0).link = ::core::ptr::null_mut::<PgSocket>();
                (*client).link = ::core::ptr::null_mut::<PgSocket>();
                (*server_0).set_ready(true);
                disconnect_server(
                    server_0,
                    false,
                    c"client gave up on cancel request, so we also give up forwarding to server"
                        .as_ptr(),
                );
            }
            if !(*client).canceled_server.is_null() {
                let mut canceled_server = (*client).canceled_server;
                statlist_remove(
                    &raw mut (*canceled_server).canceling_clients,
                    &raw mut (*client).cancel_head,
                );
                (*client).canceled_server = ::core::ptr::null_mut::<PgSocket>();
                if (*canceled_server).state() as ::core::ffi::c_int
                    == SV_BEING_CANCELED as ::core::ffi::c_int
                    && statlist_count(&raw mut (*canceled_server).canceling_clients)
                        == 0 as ::core::ffi::c_int
                {
                    release_server(canceled_server);
                }
            }
        }
        3 | 4 => {
            if (*client).replication as ::core::ffi::c_uint != 0 && !(*client).link.is_null() {
                let mut server_1 = (*client).link;
                (*server_1).link = ::core::ptr::null_mut::<PgSocket>();
                (*client).link = ::core::ptr::null_mut::<PgSocket>();
                disconnect_server(server_1, false, c"replication client disconnected".as_ptr());
            }
        }
        _ => {
            let mut _log_ctx = NULL;
            log_fatal(
                c"src/objects.c".as_ptr(),
                1580 as ::core::ffi::c_int,
                c"disconnect_client_sqlstate".as_ptr(),
                false,
                _log_ctx,
                c"bad client state: %d, %s".as_ptr(),
                (*client).state() as ::core::ffi::c_int,
                if !reason.is_null() {
                    reason
                } else {
                    c"NULL".as_ptr()
                },
            );
            exit(1 as ::core::ffi::c_int);
        }
    }
    if notify
        && !reason.is_null()
        && (*client).state() as ::core::ffi::c_int != CL_WAITING_CANCEL as ::core::ffi::c_int
    {
        send_pooler_error(client, false, sqlstate, true, reason);
    }
    free_header(&raw mut (*client).packet_cb_state.pkt);
    free_scram_state(&raw mut (*client).scram_state);
    if !(*client).login_user_credentials.is_null()
        && (*(*client).login_user_credentials).mock_auth as ::core::ffi::c_int != 0
    {
        free((*client).login_user_credentials as *mut ::core::ffi::c_void);
        (*client).login_user_credentials = ::core::ptr::null_mut::<PgCredentials>();
    }
    if !(*client).c2rust_unnamed.db.is_null()
        && (*(*client).c2rust_unnamed.db).fake as ::core::ffi::c_int != 0
    {
        free((*client).c2rust_unnamed.db as *mut ::core::ffi::c_void);
        (*client).c2rust_unnamed.db = ::core::ptr::null_mut::<PgDatabase>();
    }
    free((*client).startup_options as *mut ::core::ffi::c_void);
    (*client).startup_options = ::core::ptr::null_mut::<::core::ffi::c_char>();
    change_client_state(client, CL_JUSTFREE);
    if !sbuf_close(&raw mut (*client).sbuf) {
        let mut _log_ctx_0 = NULL;
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(
                LG_NOISE,
                _log_ctx_0,
                c"sbuf_close failed, retry later".as_ptr(),
            );
        }
    }
}

unsafe extern "C" fn connect_server(
    mut server: *mut PgSocket,
    mut sa: *const sockaddr,
    mut salen: ::core::ffi::c_int,
) {
    let mut res: bool = false;
    memset(
        &raw mut (*server).remote_addr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PgAddr>() as size_t,
    );
    if (*sa).sa_family as ::core::ffi::c_int == AF_UNIX {
        pga_set(
            &raw mut (*server).remote_addr,
            AF_UNIX,
            (*(*(*server).pool).db).port,
        );
    } else {
        pga_copy(&raw mut (*server).remote_addr, sa);
    }
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            server as *mut ::core::ffi::c_void,
            c"launching new connection to server".as_ptr(),
        );
    }
    res = sbuf_connect(
        &raw mut (*server).sbuf,
        sa,
        salen as socklen_t,
        cf_server_connect_timeout.wrapping_div(USEC) as time_t,
    );
    if !res {
        let mut _log_ctx = NULL;
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(
                LG_NOISE,
                _log_ctx,
                c"failed to launch new connection".as_ptr(),
            );
        }
    }
}

unsafe extern "C" fn dns_callback(
    mut arg: *mut ::core::ffi::c_void,
    mut sa: *const sockaddr,
    mut salen: ::core::ffi::c_int,
) {
    let mut server = arg as *mut PgSocket;
    let mut db = (*(*server).pool).db as *mut PgDatabase;
    let mut sa_in = sockaddr_in {
        sin_len: 0,
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut sa_in6 = sockaddr_in6 {
        sin6_len: 0,
        sin6_family: 0,
        sin6_port: 0,
        sin6_flowinfo: 0,
        sin6_addr: in6_addr {
            __u6_addr: C2RustUnnamed {
                __u6_addr8: [0; 16],
            },
        },
        sin6_scope_id: 0,
    };
    (*server).c2rust_unnamed.dns_token = ::core::ptr::null_mut::<DNSToken>();
    if sa.is_null() {
        disconnect_server(
            server as *mut PgSocket,
            true,
            c"server DNS lookup failed".as_ptr(),
        );
        return;
    } else if (*sa).sa_family as ::core::ffi::c_int == AF_INET {
        let mut buf: [::core::ffi::c_char; 64] = [0; 64];
        memcpy(
            &raw mut sa_in as *mut ::core::ffi::c_void,
            sa as *const ::core::ffi::c_void,
            ::core::mem::size_of::<sockaddr_in>() as size_t,
        );
        sa_in.sin_port = (if 0 != 0 {
            (((*db).port as __uint16_t as ::core::ffi::c_uint & 0xff00 as ::core::ffi::c_uint)
                >> 8 as ::core::ffi::c_int
                | ((*db).port as __uint16_t as ::core::ffi::c_uint & 0xff as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int) as __uint16_t as ::core::ffi::c_int
        } else {
            _OSSwapInt16((*db).port as __uint16_t) as ::core::ffi::c_int
        }) as __uint16_t as in_port_t;
        sa = &raw mut sa_in as *mut sockaddr;
        salen = ::core::mem::size_of::<sockaddr_in>() as ::core::ffi::c_int;
        if cf_verbose > 0 as ::core::ffi::c_int
        {
            log_generic(
                LG_DEBUG,
                server as *mut ::core::ffi::c_void,
                c"dns_callback: inet4: %s".as_ptr(),
                sa2str(
                    sa,
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
                ),
            );
        }
    } else if (*sa).sa_family as ::core::ffi::c_int == AF_INET6 {
        let mut buf_0: [::core::ffi::c_char; 64] = [0; 64];
        memcpy(
            &raw mut sa_in6 as *mut ::core::ffi::c_void,
            sa as *const ::core::ffi::c_void,
            ::core::mem::size_of::<sockaddr_in6>() as size_t,
        );
        sa_in6.sin6_port = (if 0 != 0 {
            (((*db).port as __uint16_t as ::core::ffi::c_uint & 0xff00 as ::core::ffi::c_uint)
                >> 8 as ::core::ffi::c_int
                | ((*db).port as __uint16_t as ::core::ffi::c_uint & 0xff as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int) as __uint16_t as ::core::ffi::c_int
        } else {
            _OSSwapInt16((*db).port as __uint16_t) as ::core::ffi::c_int
        }) as __uint16_t as in_port_t;
        sa = &raw mut sa_in6 as *mut sockaddr;
        salen = ::core::mem::size_of::<sockaddr_in6>() as ::core::ffi::c_int;
        if cf_verbose > 0 as ::core::ffi::c_int
        {
            log_generic(
                LG_DEBUG,
                server as *mut ::core::ffi::c_void,
                c"dns_callback: inet6: %s".as_ptr(),
                sa2str(
                    sa,
                    &raw mut buf_0 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
                ),
            );
        }
    } else {
        disconnect_server(
            server as *mut PgSocket,
            true,
            c"unknown address family: %d".as_ptr(),
            (*sa).sa_family as ::core::ffi::c_int,
        );
        return;
    }
    connect_server(server, sa, salen);
}

unsafe extern "C" fn dns_connect(mut server: *mut PgSocket) {
    let mut current_block: u64;
    let mut sa_un = sockaddr_un {
        sun_len: 0,
        sun_family: 0,
        sun_path: [0; 104],
    };
    let mut sa_in = sockaddr_in {
        sin_len: 0,
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut sa_in6 = sockaddr_in6 {
        sin6_len: 0,
        sin6_family: 0,
        sin6_port: 0,
        sin6_flowinfo: 0,
        sin6_addr: in6_addr {
            __u6_addr: C2RustUnnamed {
                __u6_addr8: [0; 16],
            },
        },
        sin6_scope_id: 0,
    };
    let mut sa = ::core::ptr::null_mut::<sockaddr>();
    let mut db = (*(*server).pool).db as *mut PgDatabase;
    let mut host = ::core::ptr::null::<::core::ffi::c_char>();
    let mut sa_len: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 0;
    let mut host_copy = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(*db).host.is_null() && !strchr((*db).host, ',' as i32).is_null() {
        let mut count = 1 as ::core::ffi::c_int;
        let mut n: ::core::ffi::c_int = 0;
        if (*(*(*server).pool).db).load_balance_hosts as ::core::ffi::c_uint
            == LOAD_BALANCE_HOSTS_DISABLE as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*(*server).pool).last_connect_failed()
        {
            (*(*server).pool).rrcounter = (*(*server).pool).rrcounter.wrapping_add(1);
        }
        let mut p: *const ::core::ffi::c_char = (*db).host;
        while *p != 0 {
            if *p as ::core::ffi::c_int == ',' as i32 {
                count += 1;
            }
            p = p.offset(1);
        }
        host_copy = xstrdup((*db).host);
        host = strtok(host_copy, c",".as_ptr());
        n = 0 as ::core::ffi::c_int;
        while !host.is_null() {
            if (*(*server).pool).rrcounter as ::core::ffi::c_int % count == n {
                break;
            }
            host = strtok(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                c",".as_ptr(),
            );
            n += 1;
        }
        if (*(*(*server).pool).db).load_balance_hosts as ::core::ffi::c_uint
            == LOAD_BALANCE_HOSTS_ROUND_ROBIN as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*(*server).pool).rrcounter = (*(*server).pool).rrcounter.wrapping_add(1);
        }
    } else {
        host = (*db).host;
    }
    if !host.is_null() {
        (*server).host = xstrdup(host);
    }
    if host.is_null()
        || *host as ::core::ffi::c_int == '/' as i32
        || *host as ::core::ffi::c_int == '@' as i32
    {
        let mut unix_dir = ::core::ptr::null::<::core::ffi::c_char>();
        memset(
            &raw mut sa_un as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sockaddr_un>() as size_t,
        );
        sa_un.sun_family = AF_UNIX as sa_family_t;
        unix_dir = if !host.is_null() {
            host
        } else {
            cf_unix_socket_dir as *const ::core::ffi::c_char
        };
        if unix_dir.is_null() || *unix_dir == 0 {
            let mut _log_ctx = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx,
                c"unix socket dir not configured: %s".as_ptr(),
                &raw mut (*db).name as *mut ::core::ffi::c_char,
            );
            disconnect_server(server as *mut PgSocket, false, c"cannot connect".as_ptr());
            current_block = 6484020372121692021;
        } else {
            snprintf(
                &raw mut sa_un.sun_path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 104]>() as size_t,
                c"%s/.s.PGSQL.%d".as_ptr(),
                unix_dir,
                (*db).port,
            );
            if cf_verbose > 1 as ::core::ffi::c_int
            {
                log_generic(
                    LG_NOISE,
                    server as *mut ::core::ffi::c_void,
                    c"unix socket: %s".as_ptr(),
                    &raw mut sa_un.sun_path as *mut ::core::ffi::c_char,
                );
            }
            if *unix_dir as ::core::ffi::c_int
                == '@' as i32
            {
                sa_len = (2 as size_t)
                    .wrapping_add(strlen(&raw mut sa_un.sun_path as *mut ::core::ffi::c_char))
                    as ::core::ffi::c_int;
                sa_un.sun_path[0 as ::core::ffi::c_int as usize] =
                    '\0' as i32 as ::core::ffi::c_char;
            } else {
                sa_len = ::core::mem::size_of::<sockaddr_un>() as ::core::ffi::c_int;
            }
            sa = &raw mut sa_un as *mut sockaddr;
            res = 1 as ::core::ffi::c_int;
            current_block = 6072622540298447352;
        }
    } else {
        if !strchr(host, ':' as i32).is_null() {
            if cf_verbose > 1 as ::core::ffi::c_int
            {
                log_generic(
                    LG_NOISE,
                    server as *mut ::core::ffi::c_void,
                    c"inet6 socket: %s".as_ptr(),
                    host,
                );
            }
            memset(
                &raw mut sa_in6 as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<sockaddr_in6>() as size_t,
            );
            sa_in6.sin6_family = AF_INET6 as sa_family_t;
            res = inet_pton(
                AF_INET6,
                host,
                &raw mut sa_in6.sin6_addr as *mut ::core::ffi::c_void,
            );
            sa_in6.sin6_port = (if 0 != 0 {
                (((*db).port as __uint16_t as ::core::ffi::c_uint & 0xff00 as ::core::ffi::c_uint)
                    >> 8 as ::core::ffi::c_int
                    | ((*db).port as __uint16_t as ::core::ffi::c_uint
                        & 0xff as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int) as __uint16_t
                    as ::core::ffi::c_int
            } else {
                _OSSwapInt16((*db).port as __uint16_t) as ::core::ffi::c_int
            }) as __uint16_t as in_port_t;
            sa = &raw mut sa_in6 as *mut sockaddr;
            sa_len = ::core::mem::size_of::<sockaddr_in6>() as ::core::ffi::c_int;
        } else {
            if cf_verbose > 1 as ::core::ffi::c_int
            {
                log_generic(
                    LG_NOISE,
                    server as *mut ::core::ffi::c_void,
                    c"inet socket: %s".as_ptr(),
                    host,
                );
            }
            memset(
                &raw mut sa_in as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<sockaddr_in>() as size_t,
            );
            sa_in.sin_family = AF_INET as sa_family_t;
            res = inet_pton(
                AF_INET,
                host,
                &raw mut sa_in.sin_addr as *mut ::core::ffi::c_void,
            );
            sa_in.sin_port = (if 0 != 0 {
                (((*db).port as __uint16_t as ::core::ffi::c_uint & 0xff00 as ::core::ffi::c_uint)
                    >> 8 as ::core::ffi::c_int
                    | ((*db).port as __uint16_t as ::core::ffi::c_uint
                        & 0xff as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int) as __uint16_t
                    as ::core::ffi::c_int
            } else {
                _OSSwapInt16((*db).port as __uint16_t) as ::core::ffi::c_int
            }) as __uint16_t as in_port_t;
            sa = &raw mut sa_in as *mut sockaddr;
            sa_len = ::core::mem::size_of::<sockaddr_in>() as ::core::ffi::c_int;
        }
        current_block = 6072622540298447352;
    }
    if current_block == 6072622540298447352 {
        if res != 1 as ::core::ffi::c_int {
            let mut tk = ::core::ptr::null_mut::<DNSToken>();
            if cf_verbose > 1 as ::core::ffi::c_int
            {
                log_generic(
                    LG_NOISE,
                    server as *mut ::core::ffi::c_void,
                    c"dns socket: %s".as_ptr(),
                    host,
                );
            }
            tk = adns_resolve(
                adns,
                host,
                Some(
                    dns_callback
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const sockaddr,
                            ::core::ffi::c_int,
                        ) -> (),
                ),
                server as *mut ::core::ffi::c_void,
            );
            if !tk.is_null() {
                (*server).c2rust_unnamed.dns_token = tk;
            }
        } else {
            connect_server(server, sa, sa_len);
        }
    }
    free(host_copy as *mut ::core::ffi::c_void);
}
#[no_mangle]

pub unsafe extern "C" fn compare_connections_by_time(
    mut lhs: *mut PgSocket,
    mut rhs: *mut PgSocket,
) -> *mut PgSocket {
    if lhs.is_null() {
        return rhs;
    }
    if rhs.is_null() {
        return lhs;
    }
    if (*lhs).request_time < (*rhs).request_time {
        lhs
    } else {
        rhs
    }
}
#[no_mangle]

pub unsafe extern "C" fn evict_connection(mut db: *mut PgDatabase) -> bool {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut oldest_connection = ::core::ptr::null_mut::<PgSocket>();
    item = pool_list.head.next;
    while item != &raw mut pool_list.head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        if (*pool).db == db {
            oldest_connection = compare_connections_by_time(
                oldest_connection,
                last_socket(&raw mut (*pool).idle_server_list),
            );
            if statlist_empty(&raw mut (*pool).waiting_client_list) {
                oldest_connection = compare_connections_by_time(
                    oldest_connection,
                    last_socket(&raw mut (*pool).used_server_list),
                );
                oldest_connection = compare_connections_by_time(
                    oldest_connection,
                    last_socket(&raw mut (*pool).tested_server_list),
                );
            }
        }
        item = (*item).next;
    }
    if !oldest_connection.is_null() {
        disconnect_server(oldest_connection, true, c"evicted".as_ptr());
        return true;
    }
    false
}
#[no_mangle]

pub unsafe extern "C" fn evict_pool_connection(mut pool: *mut PgPool) -> bool {
    let mut oldest_connection = ::core::ptr::null_mut::<PgSocket>();
    oldest_connection = compare_connections_by_time(
        oldest_connection,
        last_socket(&raw mut (*pool).idle_server_list),
    );
    if !oldest_connection.is_null() {
        disconnect_server(oldest_connection, true, c"evicted".as_ptr());
        return true;
    }
    false
}
#[no_mangle]

pub unsafe extern "C" fn evict_user_connection(mut user_credentials: *mut PgCredentials) -> bool {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut oldest_connection = ::core::ptr::null_mut::<PgSocket>();
    item = pool_list.head.next;
    while item != &raw mut pool_list.head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        if (*pool).user_credentials == user_credentials {
            oldest_connection = compare_connections_by_time(
                oldest_connection,
                last_socket(&raw mut (*pool).idle_server_list),
            );
            if statlist_empty(&raw mut (*pool).waiting_client_list) {
                oldest_connection = compare_connections_by_time(
                    oldest_connection,
                    last_socket(&raw mut (*pool).used_server_list),
                );
                oldest_connection = compare_connections_by_time(
                    oldest_connection,
                    last_socket(&raw mut (*pool).tested_server_list),
                );
            }
        }
        item = (*item).next;
    }
    if !oldest_connection.is_null() {
        disconnect_server(oldest_connection, true, c"evicted".as_ptr());
        return true;
    }
    false
}
#[no_mangle]

pub unsafe extern "C" fn launch_new_connection(mut pool: *mut PgPool, mut evict_if_needed: bool) {
    let mut current_block: u64;
    let mut server = ::core::ptr::null_mut::<PgSocket>();
    let mut max: ::core::ffi::c_int = 0;
    let mut _log_ctx = NULL;
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(LG_DEBUG, _log_ctx, c"launch_new_connection: start".as_ptr());
    }
    if !statlist_empty(&raw mut (*pool).new_server_list) {
        let mut _log_ctx_0 = NULL;
        if cf_verbose > 0 as ::core::ffi::c_int
        {
            log_generic(
                LG_DEBUG,
                _log_ctx_0,
                c"launch_new_connection: already progress".as_ptr(),
            );
        }
        return;
    }
    if (*pool).last_connect_failed() {
        let mut now = get_cached_time();
        if now.wrapping_sub((*pool).last_connect_time) < cf_server_login_retry {
            let mut _log_ctx_1 = NULL;
            if cf_verbose > 0 as ::core::ffi::c_int
            {
                log_generic(
                    LG_DEBUG,
                    _log_ctx_1,
                    c"launch_new_connection: last failed, not launching new connection yet, still waiting %llu s".as_ptr(),
                    cf_server_login_retry
                        .wrapping_sub(now.wrapping_sub((*pool).last_connect_time))
                        .wrapping_div(1000000 as ::core::ffi::c_int as usec_t),
                );
            }
            return;
        }
    }
    max = statlist_count(&raw mut (*pool).active_server_list)
        + statlist_count(&raw mut (*pool).being_canceled_server_list)
        + statlist_count(&raw mut (*pool).idle_server_list)
        + statlist_count(&raw mut (*pool).tested_server_list)
        + statlist_count(&raw mut (*pool).used_server_list)
        + statlist_count(&raw mut (*pool).new_server_list)
        + statlist_count(&raw mut (*pool).active_cancel_server_list);
    if (*(*pool).db).peer_id != 0 {
        if max >= pool_pool_size(pool) {
            let mut _log_ctx_2 = NULL;
            if cf_verbose > 0 as ::core::ffi::c_int
            {
                log_generic(
                    LG_DEBUG,
                    _log_ctx_2,
                    c"launch_new_connection: peer pool full (%d >= %d)".as_ptr(),
                    max,
                    pool_pool_size(pool),
                );
            }
            return;
        }
    } else if !statlist_empty(&raw mut (*pool).waiting_cancel_req_list)
        && max < 2 as ::core::ffi::c_int * pool_pool_size(pool)
    {
        let mut _log_ctx_3 = NULL;
        if cf_verbose > 0 as ::core::ffi::c_int
        {
            log_generic(
                LG_DEBUG,
                _log_ctx_3,
                c"launch_new_connection: bypass pool limitations for cancel request".as_ptr(),
            );
        }
    } else {
        if pool_pool_size(pool) > 0 as ::core::ffi::c_int
            && max >= pool_pool_size(pool)
            && (*pool).welcome_msg_ready() as ::core::ffi::c_int != 0
        {
            let mut c = first_socket(&raw mut (*pool).waiting_client_list);
            if cf_res_pool_timeout != 0 && pool_res_pool_size(pool) != 0 {
                let mut now_0 = get_cached_time();
                if !c.is_null() && now_0.wrapping_sub((*c).request_time) >= cf_res_pool_timeout {
                    if max < pool_pool_size(pool) + pool_res_pool_size(pool) {
                        log_generic(
                            LG_WARNING,
                            c as *mut ::core::ffi::c_void,
                            c"taking connection from reserve_pool".as_ptr(),
                        );
                        current_block = 14785029078859074793;
                    } else {
                        current_block = 5529461102203738653;
                    }
                } else {
                    current_block = 5529461102203738653;
                }
            } else {
                current_block = 5529461102203738653;
            }
            match current_block {
                14785029078859074793 => {}
                _ => {
                    if !c.is_null()
                        && (*c).replication as ::core::ffi::c_uint != 0
                        && !sending_auth_query(c)
                    {
                        while evict_if_needed
                            && pool_pool_size(pool) >= max
                        {
                            if !evict_pool_connection(pool) {
                                break;
                            }
                        }
                        if pool_pool_size(pool) < max {
                            current_block = 14785029078859074793;
                        } else {
                            current_block = 12997042908615822766;
                        }
                    } else {
                        current_block = 12997042908615822766;
                    }
                    match current_block {
                        14785029078859074793 => {}
                        _ => {
                            let mut _log_ctx_4 = NULL;
                            if cf_verbose > 0 as ::core::ffi::c_int
                            {
                                log_generic(
                                    LG_DEBUG,
                                    _log_ctx_4,
                                    c"launch_new_connection: pool full (%d >= %d)".as_ptr(),
                                    max,
                                    pool_pool_size(pool),
                                );
                            }
                            return;
                        }
                    }
                }
            }
        }
        max = database_max_connections((*pool).db);
        if max > 0 as ::core::ffi::c_int {
            while evict_if_needed
                && (*(*pool).db).connection_count >= max
            {
                if !evict_connection((*pool).db) {
                    break;
                }
            }
            if (*(*pool).db).connection_count >= max {
                let mut _log_ctx_5 = NULL;
                if cf_verbose > 0 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_DEBUG,
                        _log_ctx_5,
                        c"launch_new_connection: database '%s' full (%d >= %d)".as_ptr(),
                        &raw mut (*(*pool).db).name as *mut ::core::ffi::c_char,
                        (*(*pool).db).connection_count,
                        max,
                    );
                }
                return;
            }
        }
        max = user_max_connections((*(*pool).user_credentials).global_user);
        if max > 0 as ::core::ffi::c_int {
            while evict_if_needed
                && (*(*(*pool).user_credentials).global_user).connection_count >= max
            {
                if !evict_user_connection((*pool).user_credentials) {
                    break;
                }
            }
            if (*(*(*pool).user_credentials).global_user).connection_count >= max {
                let mut _log_ctx_6 = NULL;
                if cf_verbose > 0 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_DEBUG,
                        _log_ctx_6,
                        c"launch_new_connection: user '%s' full (%d >= %d)".as_ptr(),
                        &raw mut (*(*pool).user_credentials).name as *mut ::core::ffi::c_char,
                        (*(*(*pool).user_credentials).global_user).connection_count,
                        max,
                    );
                }
                return;
            }
        }
    }
    server = slab_alloc(server_cache) as *mut PgSocket;
    if server.is_null() {
        let mut _log_ctx_7 = NULL;
        if cf_verbose > 0 as ::core::ffi::c_int
        {
            log_generic(
                LG_DEBUG,
                _log_ctx_7,
                c"launch_new_connection: no memory".as_ptr(),
            );
        }
        return;
    }
    (*server).pool = pool;
    (*server).login_user_credentials = (*(*server).pool).user_credentials;
    (*server).connect_time = get_cached_time();
    statlist_init(
        &raw mut (*server).canceling_clients,
        c"canceling_clients".as_ptr(),
    );
    (*pool).last_connect_time = get_cached_time();
    change_server_state(server, SV_LOGIN);
    (*(*pool).db).connection_count += 1;
    if !(*pool).user_credentials.is_null() {
        (*(*(*pool).user_credentials).global_user).connection_count += 1;
    }
    dns_connect(server as *mut PgSocket);
}
#[no_mangle]

pub unsafe extern "C" fn accept_client(
    mut sock: ::core::ffi::c_int,
    mut is_unix: bool,
) -> *mut PgSocket {
    let mut res: bool = false;
    let mut client = ::core::ptr::null_mut::<PgSocket>();
    client = slab_alloc(client_cache) as *mut PgSocket;
    if client.is_null() {
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            c"cannot allocate client struct".as_ptr(),
        );
        safe_close(sock);
        return ::core::ptr::null_mut::<PgSocket>();
    }
    (*client).request_time = get_cached_time();
    (*client).connect_time = (*client).request_time;
    (*client).query_start = 0;
    fill_remote_addr(client, sock, is_unix);
    fill_local_addr(client, sock, is_unix);
    change_client_state(client, CL_LOGIN);
    res = sbuf_accept(&raw mut (*client).sbuf, sock, is_unix);
    if !res {
        if cf_log_connections != 0
            && cf_verbose > 0 as ::core::ffi::c_int
        {
            log_generic(
                LG_DEBUG,
                client as *mut ::core::ffi::c_void,
                c"failed connection attempt".as_ptr(),
            );
        }
        return ::core::ptr::null_mut::<PgSocket>();
    }
    client
}
#[no_mangle]

pub unsafe extern "C" fn finish_client_login(mut client: *mut PgSocket) -> bool {
    if (*(*client).c2rust_unnamed.db).fake {
        if cf_log_connections != 0 {
            log_generic(
                LG_INFO,
                client as *mut ::core::ffi::c_void,
                c"login failed: db=%s user=%s".as_ptr(),
                &raw mut (*(*client).c2rust_unnamed.db).name as *mut ::core::ffi::c_char,
                &raw mut (*(*client).login_user_credentials).name as *mut ::core::ffi::c_char,
            );
        }
        disconnect_client(
            client,
            true,
            c"no such database: %s".as_ptr(),
            &raw mut (*(*client).c2rust_unnamed.db).name as *mut ::core::ffi::c_char,
        );
        return false;
    }
    if (*(*client).c2rust_unnamed.db).db_disabled {
        disconnect_client(
            client,
            true,
            b"database \"%s\" is disabled\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*(*client).c2rust_unnamed.db).name as *mut ::core::ffi::c_char,
        );
        return false;
    }
    if cf_shutdown != 0
        && strcmp(
            c"pgbouncer".as_ptr(),
            &raw mut (*(*client).c2rust_unnamed.db).name as *mut ::core::ffi::c_char,
        ) != 0
    {
        disconnect_client(client, true, c"pooler is shutting down".as_ptr());
        return false;
    }
    match (*client).state() as ::core::ffi::c_int {
        2 => {
            change_client_state(client, CL_ACTIVE);
        }
        5 => {}
        _ => {
            let mut _log_ctx = NULL;
            log_fatal(
                c"src/objects.c".as_ptr(),
                2058 as ::core::ffi::c_int,
                c"finish_client_login".as_ptr(),
                false,
                _log_ctx,
                c"bad client state: %d".as_ptr(),
                (*client).state() as ::core::ffi::c_int,
            );
            exit(1 as ::core::ffi::c_int);
        }
    }
    (*client).set_wait_for_auth(false);
    if !(*(*client).pool).welcome_msg_ready() {
        let mut _log_ctx_0 = NULL;
        if cf_verbose > 0 as ::core::ffi::c_int
        {
            log_generic(
                LG_DEBUG,
                _log_ctx_0,
                c"finish_client_login: no welcome message, pause".as_ptr(),
            );
        }
        (*client).set_wait_for_welcome(true);
        pause_client(client);
        if cf_pause_mode == P_NONE as ::core::ffi::c_int {
            launch_new_connection((*client).pool, true);
        }
        return false;
    }
    (*client).set_wait_for_welcome(false);
    if !welcome_client(client) {
        return false;
    }
    (*client).set_welcome_sent(true);
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            client as *mut ::core::ffi::c_void,
            c"logged in".as_ptr(),
        );
    }
    true
}

unsafe extern "C" fn accept_cancel_request_for_peer(
    mut peer_id: ::core::ffi::c_int,
    mut req: *mut PgSocket,
) {
    let mut peer = ::core::ptr::null_mut::<PgDatabase>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut ttl = (*req).cancel_key[7 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        & CANCELLATION_TTL_MASK;
    if ttl == 0 as ::core::ffi::c_int {
        disconnect_client(
            req,
            false,
            c"failed to forward cancel request because its TTL was exhausted".as_ptr(),
        );
        return;
    }
    (*req).cancel_key[7 as ::core::ffi::c_int as usize] =
        (*req).cancel_key[7 as ::core::ffi::c_int as usize].wrapping_sub(1);
    peer = find_peer(peer_id);
    if peer.is_null() {
        disconnect_client(
            req,
            false,
            c"could not find peer to forward request to".as_ptr(),
        );
        return;
    }
    let mut _log_ctx = NULL;
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            _log_ctx,
            c"forwarding cancellation request to peer %d".as_ptr(),
            peer_id,
        );
    }
    pool = get_peer_pool(peer);
    if pool.is_null() {
        disconnect_client(req, false, c"out of memory".as_ptr());
        return;
    }
    (*req).pool = pool;
    pause_cancel_request(req);
    launch_new_connection(pool, true);
}
#[no_mangle]

pub unsafe extern "C" fn accept_cancel_request(mut req: *mut PgSocket) {
    let mut pitem = ::core::ptr::null_mut::<List>();
    let mut citem = ::core::ptr::null_mut::<List>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut server = ::core::ptr::null_mut::<PgSocket>();
    let mut client = ::core::ptr::null_mut::<PgSocket>();
    let mut main_client = ::core::ptr::null_mut::<PgSocket>();
    let mut peering_enabled = false;
    peering_enabled = cf_peer_id > 0 as ::core::ffi::c_int;
    if peering_enabled {
        let mut peer_id = (*req).cancel_key[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            + (((*req).cancel_key[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int);
        let mut needs_forwarding_to_peer = cf_peer_id != peer_id;
        if needs_forwarding_to_peer {
            accept_cancel_request_for_peer(peer_id, req);
            return;
        }
        (*req).cancel_key[7 as ::core::ffi::c_int as usize] =
            ((*req).cancel_key[7 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                | CANCELLATION_TTL_MASK) as uint8_t;
    }
    pitem = pool_list.head.next;
    's_45: while pitem != &raw mut pool_list.head {
        pool = (pitem as *mut ::core::ffi::c_char)
            as *mut PgPool;
        citem = (*pool).active_client_list.head.next;
        while citem != &raw mut (*pool).active_client_list.head {
            client = (citem as *mut ::core::ffi::c_char)
                
                as *mut PgSocket;
            if memcmp(
                &raw mut (*client).cancel_key as *mut uint8_t as *const ::core::ffi::c_void,
                &raw mut (*req).cancel_key as *mut uint8_t as *const ::core::ffi::c_void,
                8 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                main_client = client;
                break 's_45;
            } else {
                citem = (*citem).next;
            }
        }
        citem = (*pool).waiting_client_list.head.next;
        while citem != &raw mut (*pool).waiting_client_list.head {
            client = (citem as *mut ::core::ffi::c_char)
                
                as *mut PgSocket;
            if memcmp(
                &raw mut (*client).cancel_key as *mut uint8_t as *const ::core::ffi::c_void,
                &raw mut (*req).cancel_key as *mut uint8_t as *const ::core::ffi::c_void,
                8 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                main_client = client;
                break 's_45;
            } else {
                citem = (*citem).next;
            }
        }
        pitem = (*pitem).next;
    }
    if main_client.is_null() {
        disconnect_client(req, false, c"failed cancel request".as_ptr());
        return;
    }
    if (*(*(*main_client).pool).db).admin {
        disconnect_client(req, false, c"cancel request for console client".as_ptr());
        admin_handle_cancel(main_client);
        return;
    }
    if (*main_client).link.is_null() {
        disconnect_client(req, false, c"cancel request for idle client".as_ptr());
        return;
    }
    server = (*main_client).link;
    if (*server).setting_vars() {
        disconnect_client(
            req,
            false,
            c"ignoring cancel request for server that is setting vars".as_ptr(),
        );
        return;
    }
    (*req).canceled_server = server;
    statlist_append(
        &raw mut (*server).canceling_clients,
        &raw mut (*req).cancel_head,
    );
    (*req).pool = pool;
    pause_cancel_request(req);
    launch_new_connection(pool, true);
}
#[no_mangle]

pub unsafe extern "C" fn forward_cancel_request(mut server: *mut PgSocket) {
    let mut res: bool = false;
    let mut req = first_socket(&raw mut (*(*server).pool).waiting_cancel_req_list);
    let mut forwarding_to_peer = (*(*(*server).pool).db).peer_id != 0 as ::core::ffi::c_int;
    (*server).link = req;
    (*req).link = server;
    if !forwarding_to_peer && (*req).canceled_server.is_null() {
        disconnect_client(
            req,
            false,
            c"not sending cancel request for client that is now idle".as_ptr(),
        );
        return;
    }
    if forwarding_to_peer {
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
            PKT_CANCEL,
            c"b".as_ptr(),
            &raw mut (*req).cancel_key as *mut uint8_t,
            8 as ::core::ffi::c_int,
        );
        res = pktbuf_send_immediate(&raw mut _buf, server);
    } else {
        let mut _data_0: [uint8_t; 16] = [0; 16];
        let mut _buf_0 = PktBuf {
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
            &raw mut _buf_0,
            &raw mut _data_0 as *mut uint8_t,
            ::core::mem::size_of::<[uint8_t; 16]>() as ::core::ffi::c_int,
        );
        pktbuf_write_generic(
            &raw mut _buf_0,
            PKT_CANCEL,
            c"b".as_ptr(),
            &raw mut (*(*req).canceled_server).cancel_key as *mut uint8_t,
            8 as ::core::ffi::c_int,
        );
        res = pktbuf_send_immediate(&raw mut _buf_0, server);
    }
    if !res {
        log_generic(
            LG_WARNING,
            req as *mut ::core::ffi::c_void,
            c"sending cancel request failed: %s".as_ptr(),
            strerror(*__error()),
        );
        disconnect_client(req, false, c"failed to send cancel request".as_ptr());
        return;
    }
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            req as *mut ::core::ffi::c_void,
            c"started sending cancel request".as_ptr(),
        );
    }
    change_client_state(req, CL_ACTIVE_CANCEL);
    change_server_state(server, SV_ACTIVE_CANCEL);
    sbuf_continue(&raw mut (*server).sbuf);
}
#[no_mangle]

pub unsafe extern "C" fn use_client_socket(
    mut fd: ::core::ffi::c_int,
    mut addr: *mut PgAddr,
    mut dbname: *const ::core::ffi::c_char,
    mut username: *const ::core::ffi::c_char,
    mut ckey: uint64_t,
    mut oldfd: ::core::ffi::c_int,
    mut linkfd: ::core::ffi::c_int,
    mut client_enc: *const ::core::ffi::c_char,
    mut std_string: *const ::core::ffi::c_char,
    mut datestyle: *const ::core::ffi::c_char,
    mut timezone: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut scram_client_key: *const ::core::ffi::c_char,
    mut scram_client_key_len: ::core::ffi::c_int,
    mut scram_server_key: *const ::core::ffi::c_char,
    mut scram_server_key_len: ::core::ffi::c_int,
) -> bool {
    let mut db = find_database(dbname);
    let mut client = ::core::ptr::null_mut::<PgSocket>();
    let mut tmp = PktBuf {
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
    if db.is_null() {
        db = register_auto_database(dbname);
        if db.is_null() {
            return true;
        }
    }
    if !scram_client_key.is_null() || !scram_server_key.is_null() {
        let mut credentials = ::core::ptr::null_mut::<PgCredentials>();
        if scram_client_key.is_null() || scram_server_key.is_null() {
            let mut _log_ctx = NULL;
            log_generic(LG_ERROR, _log_ctx, c"incomplete SCRAM key data".as_ptr());
            return false;
        }
        if ::core::mem::size_of::<[uint8_t; 32]>() != scram_client_key_len as usize
            || ::core::mem::size_of::<[uint8_t; 32]>() != scram_server_key_len as usize
        {
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_0,
                c"incompatible SCRAM key data".as_ptr(),
            );
            return false;
        }
        if !(*db).forced_user_credentials.is_null() {
            let mut _log_ctx_1 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_1,
                c"SCRAM key data received for forced user".as_ptr(),
            );
            return false;
        }
        if cf_auth_type == AUTH_TYPE_PAM as ::core::ffi::c_int {
            let mut _log_ctx_2 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_2,
                c"SCRAM key data received for PAM user".as_ptr(),
            );
            return false;
        }
        credentials = find_global_credentials(username);
        if credentials.is_null() && !(*db).auth_user_credentials.is_null() {
            credentials = add_dynamic_credentials(db, username, password);
        }
        if credentials.is_null() {
            return false;
        }
        memcpy(
            &raw mut (*credentials).scram_ClientKey as *mut uint8_t as *mut ::core::ffi::c_void,
            scram_client_key as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
        );
        memcpy(
            &raw mut (*credentials).scram_ServerKey as *mut uint8_t as *mut ::core::ffi::c_void,
            scram_server_key as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
        );
        (*credentials).use_scram_keys = true;
    }
    client = accept_client(fd, pga_is_unix(addr));
    if client.is_null() {
        return false;
    }
    (*client).set_suspended(true);
    if !set_pool(client, dbname, username, password, true) {
        return false;
    }
    change_client_state(client, CL_ACTIVE);
    pktbuf_static(
        &raw mut tmp,
        &raw mut (*client).cancel_key as *mut uint8_t,
        8 as ::core::ffi::c_int,
    );
    pktbuf_put_uint64(&raw mut tmp, ckey);
    (*client).request_time = oldfd as usec_t;
    (*client).query_start = linkfd as usec_t;
    varcache_set(
        &raw mut (*client).vars,
        c"client_encoding".as_ptr(),
        client_enc,
    );
    varcache_set(
        &raw mut (*client).vars,
        c"standard_conforming_strings".as_ptr(),
        std_string,
    );
    varcache_set(&raw mut (*client).vars, c"datestyle".as_ptr(), datestyle);
    varcache_set(&raw mut (*client).vars, c"timezone".as_ptr(), timezone);
    true
}
#[no_mangle]

pub unsafe extern "C" fn use_server_socket(
    mut fd: ::core::ffi::c_int,
    mut addr: *mut PgAddr,
    mut dbname: *const ::core::ffi::c_char,
    mut username: *const ::core::ffi::c_char,
    mut ckey: uint64_t,
    mut oldfd: ::core::ffi::c_int,
    mut linkfd: ::core::ffi::c_int,
    mut client_enc: *const ::core::ffi::c_char,
    mut std_string: *const ::core::ffi::c_char,
    mut datestyle: *const ::core::ffi::c_char,
    mut timezone: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut _scram_client_key: *const ::core::ffi::c_char,
    mut _scram_client_key_len: ::core::ffi::c_int,
    mut _scram_server_key: *const ::core::ffi::c_char,
    mut _scram_server_key_len: ::core::ffi::c_int,
) -> bool {
    let mut db = find_database(dbname);
    let mut credentials = ::core::ptr::null_mut::<PgCredentials>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut server = ::core::ptr::null_mut::<PgSocket>();
    let mut tmp = PktBuf {
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
    let mut res: bool = false;
    if db.is_null() {
        db = register_auto_database(dbname);
        if db.is_null() {
            return true;
        }
    }
    if !(*db).forced_user_credentials.is_null() {
        credentials = (*db).forced_user_credentials;
    } else if cf_auth_type == AUTH_TYPE_PAM as ::core::ffi::c_int {
        credentials = add_pam_credentials(username, password);
    } else {
        credentials = find_global_credentials(username);
    }
    if credentials.is_null() && !(*db).auth_user_credentials.is_null() {
        credentials = add_dynamic_credentials(db, username, password);
    }
    pool = get_pool(db, credentials);
    if pool.is_null() {
        return false;
    }
    server = slab_alloc(server_cache) as *mut PgSocket;
    if server.is_null() {
        return false;
    }
    res = sbuf_accept(&raw mut (*server).sbuf, fd, pga_is_unix(addr));
    if !res {
        return false;
    }
    (*db).connection_count += 1;
    (*server).set_suspended(true);
    (*server).pool = pool;
    (*server).login_user_credentials = credentials;
    (*server).request_time = get_cached_time();
    (*server).connect_time = (*server).request_time;
    (*server).query_start = 0;
    statlist_init(
        &raw mut (*server).canceling_clients,
        c"canceling_clients".as_ptr(),
    );
    fill_remote_addr(server, fd, pga_is_unix(addr));
    fill_local_addr(server, fd, pga_is_unix(addr));
    if linkfd != 0 {
        (*server).set_ready(false);
        change_server_state(server, SV_ACTIVE);
    } else {
        (*server).set_ready(true);
        change_server_state(server, SV_IDLE);
    }
    pktbuf_static(
        &raw mut tmp,
        &raw mut (*server).cancel_key as *mut uint8_t,
        8 as ::core::ffi::c_int,
    );
    pktbuf_put_uint64(&raw mut tmp, ckey);
    (*server).request_time = oldfd as usec_t;
    (*server).query_start = linkfd as usec_t;
    varcache_set(
        &raw mut (*server).vars,
        c"client_encoding".as_ptr(),
        client_enc,
    );
    varcache_set(
        &raw mut (*server).vars,
        c"standard_conforming_strings".as_ptr(),
        std_string,
    );
    varcache_set(&raw mut (*server).vars, c"datestyle".as_ptr(), datestyle);
    varcache_set(&raw mut (*server).vars, c"timezone".as_ptr(), timezone);
    true
}
#[no_mangle]

pub unsafe extern "C" fn for_each_server(
    mut pool: *mut PgPool,
    mut func: Option<unsafe extern "C" fn(*mut PgSocket) -> ()>,
) {
    let mut item = ::core::ptr::null_mut::<List>();
    item = (*pool).idle_server_list.head.next;
    while item != &raw mut (*pool).idle_server_list.head {
        func.expect("non-null function pointer")(
            (item as *mut ::core::ffi::c_char)
                as *mut PgSocket,
        );
        item = (*item).next;
    }
    item = (*pool).used_server_list.head.next;
    while item != &raw mut (*pool).used_server_list.head {
        func.expect("non-null function pointer")(
            (item as *mut ::core::ffi::c_char)
                as *mut PgSocket,
        );
        item = (*item).next;
    }
    item = (*pool).tested_server_list.head.next;
    while item != &raw mut (*pool).tested_server_list.head {
        func.expect("non-null function pointer")(
            (item as *mut ::core::ffi::c_char)
                as *mut PgSocket,
        );
        item = (*item).next;
    }
    item = (*pool).active_server_list.head.next;
    while item != &raw mut (*pool).active_server_list.head {
        func.expect("non-null function pointer")(
            (item as *mut ::core::ffi::c_char)
                as *mut PgSocket,
        );
        item = (*item).next;
    }
    item = (*pool).new_server_list.head.next;
    while item != &raw mut (*pool).new_server_list.head {
        func.expect("non-null function pointer")(
            (item as *mut ::core::ffi::c_char)
                as *mut PgSocket,
        );
        item = (*item).next;
    }
}

unsafe extern "C" fn for_each_server_filtered(
    mut pool: *mut PgPool,
    mut func: Option<unsafe extern "C" fn(*mut PgSocket) -> ()>,
    mut filter: Option<unsafe extern "C" fn(*mut PgSocket, *mut ::core::ffi::c_void) -> bool>,
    mut filter_arg: *mut ::core::ffi::c_void,
) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut sk = ::core::ptr::null_mut::<PgSocket>();
    item = (*pool).idle_server_list.head.next;
    while item != &raw mut (*pool).idle_server_list.head {
        sk = (item as *mut ::core::ffi::c_char)
            as *mut PgSocket;
        if filter.expect("non-null function pointer")(sk, filter_arg) {
            func.expect("non-null function pointer")(sk);
        }
        item = (*item).next;
    }
    item = (*pool).used_server_list.head.next;
    while item != &raw mut (*pool).used_server_list.head {
        sk = (item as *mut ::core::ffi::c_char)
            as *mut PgSocket;
        if filter.expect("non-null function pointer")(sk, filter_arg) {
            func.expect("non-null function pointer")(sk);
        }
        item = (*item).next;
    }
    item = (*pool).tested_server_list.head.next;
    while item != &raw mut (*pool).tested_server_list.head {
        sk = (item as *mut ::core::ffi::c_char)
            as *mut PgSocket;
        if filter.expect("non-null function pointer")(sk, filter_arg) {
            func.expect("non-null function pointer")(sk);
        }
        item = (*item).next;
    }
    item = (*pool).active_server_list.head.next;
    while item != &raw mut (*pool).active_server_list.head {
        sk = (item as *mut ::core::ffi::c_char)
            as *mut PgSocket;
        if filter.expect("non-null function pointer")(sk, filter_arg) {
            func.expect("non-null function pointer")(sk);
        }
        item = (*item).next;
    }
    item = (*pool).new_server_list.head.next;
    while item != &raw mut (*pool).new_server_list.head {
        sk = (item as *mut ::core::ffi::c_char)
            as *mut PgSocket;
        if filter.expect("non-null function pointer")(sk, filter_arg) {
            func.expect("non-null function pointer")(sk);
        }
        item = (*item).next;
    }
}

unsafe extern "C" fn tag_dirty(mut sk: *mut PgSocket) {
    (*sk).set_close_needed(true);
}
#[no_mangle]

pub unsafe extern "C" fn tag_pool_dirty(mut pool: *mut PgPool) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut server = ::core::ptr::null_mut::<PgSocket>();
    if (*(*pool).db).admin {
        return;
    }
    if !(*pool).welcome_msg.is_null() {
        pktbuf_free((*pool).welcome_msg as *mut PktBuf);
        (*pool).welcome_msg = ::core::ptr::null_mut::<PktBuf>();
    }
    (*pool).set_welcome_msg_ready(false);
    for_each_server(
        pool,
        Some(tag_dirty as unsafe extern "C" fn(*mut PgSocket) -> ()),
    );
    item = (*pool).new_server_list.head.next;
    tmp = (*(*pool).new_server_list.head.next).next;
    while item != &raw mut (*pool).new_server_list.head {
        server = (item as *mut ::core::ffi::c_char)
            as *mut PgSocket as *mut PgSocket;
        disconnect_server(
            server as *mut PgSocket,
            true,
            c"connect string changed".as_ptr(),
        );
        item = tmp;
        tmp = (*tmp).next;
    }
}
#[no_mangle]

pub unsafe extern "C" fn tag_database_dirty(mut db: *mut PgDatabase) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    item = pool_list.head.next;
    while item != &raw mut pool_list.head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        if (*pool).db == db {
            tag_pool_dirty(pool);
        }
        item = (*item).next;
    }
}
#[no_mangle]

pub unsafe extern "C" fn tag_autodb_dirty() {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut db = ::core::ptr::null_mut::<PgDatabase>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    item = database_list.head.next;
    while item != &raw mut database_list.head {
        db = (item as *mut ::core::ffi::c_char)
            as *mut PgDatabase;
        if (*db).db_auto {
            register_auto_database(&raw mut (*db).name as *mut ::core::ffi::c_char);
        }
        item = (*item).next;
    }
    item = autodatabase_idle_list.head.next;
    tmp = (*autodatabase_idle_list.head.next).next;
    while item != &raw mut autodatabase_idle_list.head {
        db = (item as *mut ::core::ffi::c_char)
            as *mut PgDatabase;
        if (*db).db_auto {
            register_auto_database(&raw mut (*db).name as *mut ::core::ffi::c_char);
        }
        item = tmp;
        tmp = (*tmp).next;
    }
    item = pool_list.head.next;
    while item != &raw mut pool_list.head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        if (*(*pool).db).db_auto {
            tag_pool_dirty(pool);
        }
        item = (*item).next;
    }
}

unsafe extern "C" fn server_remote_addr_filter(
    mut sk: *mut PgSocket,
    mut arg: *mut ::core::ffi::c_void,
) -> bool {
    let mut addr = arg as *mut PgAddr;
    pga_cmp_addr(&raw mut (*sk).remote_addr, addr) == 0 as ::core::ffi::c_int
}
#[no_mangle]

pub unsafe extern "C" fn tag_host_addr_dirty(
    mut host: *const ::core::ffi::c_char,
    mut sa: *const sockaddr,
) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut addr = PgAddr {
        sa: sockaddr {
            sa_len: 0,
            sa_family: 0,
            sa_data: [0; 14],
        },
    };
    memset(
        &raw mut addr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PgAddr>() as size_t,
    );
    pga_copy(&raw mut addr, sa);
    item = pool_list.head.next;
    while item != &raw mut pool_list.head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        if !(*(*pool).db).host.is_null()
            && strcmp(host, (*(*pool).db).host) == 0 as ::core::ffi::c_int
        {
            for_each_server_filtered(
                pool,
                Some(tag_dirty as unsafe extern "C" fn(*mut PgSocket) -> ()),
                Some(
                    server_remote_addr_filter
                        as unsafe extern "C" fn(*mut PgSocket, *mut ::core::ffi::c_void) -> bool,
                ),
                &raw mut addr as *mut ::core::ffi::c_void,
            );
        }
        item = (*item).next;
    }
}
#[no_mangle]

pub unsafe extern "C" fn reuse_just_freed_objects() {
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut item = ::core::ptr::null_mut::<List>();
    let mut sk = ::core::ptr::null_mut::<PgSocket>();
    let mut close_works = true;
    item = justfree_client_list.head.next;
    tmp = (*justfree_client_list.head.next).next;
    while item != &raw mut justfree_client_list.head {
        sk = (item as *mut ::core::ffi::c_char)
            as *mut PgSocket;
        if sbuf_is_closed(&raw mut (*sk).sbuf) {
            change_client_state(sk, CL_FREE);
        } else if close_works {
            close_works = sbuf_close(&raw mut (*sk).sbuf);
        }
        item = tmp;
        tmp = (*tmp).next;
    }
    item = justfree_server_list.head.next;
    tmp = (*justfree_server_list.head.next).next;
    while item != &raw mut justfree_server_list.head {
        sk = (item as *mut ::core::ffi::c_char)
            as *mut PgSocket;
        if sbuf_is_closed(&raw mut (*sk).sbuf) {
            change_server_state(sk, SV_FREE);
        } else if close_works {
            close_works = sbuf_close(&raw mut (*sk).sbuf);
        }
        item = tmp;
        tmp = (*tmp).next;
    }
}
#[no_mangle]

pub unsafe extern "C" fn objects_cleanup() {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut db = ::core::ptr::null_mut::<PgDatabase>();
    reuse_just_freed_objects();
    reuse_just_freed_objects();
    item = autodatabase_idle_list.head.next;
    tmp = (*autodatabase_idle_list.head.next).next;
    while item != &raw mut autodatabase_idle_list.head {
        db = (item as *mut ::core::ffi::c_char)
            as *mut PgDatabase;
        kill_database(db);
        item = tmp;
        tmp = (*tmp).next;
    }
    item = database_list.head.next;
    tmp = (*database_list.head.next).next;
    while item != &raw mut database_list.head {
        db = (item as *mut ::core::ffi::c_char)
            as *mut PgDatabase;
        kill_database(db);
        item = tmp;
        tmp = (*tmp).next;
    }
    item = peer_list.head.next;
    tmp = (*peer_list.head.next).next;
    while item != &raw mut peer_list.head {
        let mut peer = (item as *mut ::core::ffi::c_char)
            
            as *mut PgDatabase;
        kill_peer(peer);
        item = tmp;
        tmp = (*tmp).next;
    }
    item = justfree_server_list.head.next;
    tmp = (*justfree_server_list.head.next).next;
    while item != &raw mut justfree_server_list.head {
        let mut server = (item as *mut ::core::ffi::c_char)
            
            as *mut PgSocket;
        server_free(server);
        item = tmp;
        tmp = (*tmp).next;
    }
    item = justfree_client_list.head.next;
    tmp = (*justfree_client_list.head.next).next;
    while item != &raw mut justfree_client_list.head {
        let mut client = (item as *mut ::core::ffi::c_char)
            
            as *mut PgSocket;
        client_free(client);
        item = tmp;
        tmp = (*tmp).next;
    }
    clear_user_tree_cached_scram_keys(&raw mut user_tree);
    memset(
        &raw mut login_client_list as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<StatList>() as size_t,
    );
    memset(
        &raw mut user_list as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<StatList>() as size_t,
    );
    memset(
        &raw mut database_list as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<StatList>() as size_t,
    );
    memset(
        &raw mut pool_list as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<StatList>() as size_t,
    );
    memset(
        &raw mut pam_user_tree as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<AATree>() as size_t,
    );
    memset(
        &raw mut user_tree as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<AATree>() as size_t,
    );
    memset(
        &raw mut autodatabase_idle_list as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<StatList>() as size_t,
    );
    slab_destroy(server_cache);
    server_cache = ::core::ptr::null_mut::<Slab>();
    slab_destroy(client_cache);
    client_cache = ::core::ptr::null_mut::<Slab>();
    slab_destroy(db_cache);
    db_cache = ::core::ptr::null_mut::<Slab>();
    slab_destroy(peer_cache);
    peer_cache = ::core::ptr::null_mut::<Slab>();
    slab_destroy(peer_pool_cache);
    peer_pool_cache = ::core::ptr::null_mut::<Slab>();
    slab_destroy(pool_cache);
    pool_cache = ::core::ptr::null_mut::<Slab>();
    slab_destroy(user_cache);
    user_cache = ::core::ptr::null_mut::<Slab>();
    slab_destroy(credentials_cache);
    credentials_cache = ::core::ptr::null_mut::<Slab>();
    slab_destroy(iobuf_cache);
    iobuf_cache = ::core::ptr::null_mut::<Slab>();
    slab_destroy(outstanding_request_cache);
    outstanding_request_cache = ::core::ptr::null_mut::<Slab>();
    slab_destroy(var_list_cache);
    var_list_cache = ::core::ptr::null_mut::<Slab>();
    slab_destroy(server_prepared_statement_cache);
    server_prepared_statement_cache = ::core::ptr::null_mut::<Slab>();
}
unsafe extern "C" fn run_static_initializers() {
    user_list = StatList {
        head: List {
            next: &raw mut user_list.head,
            prev: &raw mut user_list.head,
        },
        cur_count: 0 as ::core::ffi::c_int,
    };
    pool_list = StatList {
        head: List {
            next: &raw mut pool_list.head,
            prev: &raw mut pool_list.head,
        },
        cur_count: 0 as ::core::ffi::c_int,
    };
    peer_pool_list = StatList {
        head: List {
            next: &raw mut peer_pool_list.head,
            prev: &raw mut peer_pool_list.head,
        },
        cur_count: 0 as ::core::ffi::c_int,
    };
    database_list = StatList {
        head: List {
            next: &raw mut database_list.head,
            prev: &raw mut database_list.head,
        },
        cur_count: 0 as ::core::ffi::c_int,
    };
    peer_list = StatList {
        head: List {
            next: &raw mut peer_list.head,
            prev: &raw mut peer_list.head,
        },
        cur_count: 0 as ::core::ffi::c_int,
    };
    autodatabase_idle_list = StatList {
        head: List {
            next: &raw mut autodatabase_idle_list.head,
            prev: &raw mut autodatabase_idle_list.head,
        },
        cur_count: 0 as ::core::ffi::c_int,
    };
    login_client_list = StatList {
        head: List {
            next: &raw mut login_client_list.head,
            prev: &raw mut login_client_list.head,
        },
        cur_count: 0 as ::core::ffi::c_int,
    };
    justfree_server_list = StatList {
        head: List {
            next: &raw mut justfree_server_list.head,
            prev: &raw mut justfree_server_list.head,
        },
        cur_count: 0 as ::core::ffi::c_int,
    };
    justfree_client_list = StatList {
        head: List {
            next: &raw mut justfree_client_list.head,
            prev: &raw mut justfree_client_list.head,
        },
        cur_count: 0 as ::core::ffi::c_int,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

extern "C" {
    pub fn get_cached_time() -> usec_t;
}

extern "C" {
    pub fn get_num_var_cached() -> ::core::ffi::c_int;
    pub fn varcache_set(
        cache: *mut VarCache,
        key: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
    ) -> bool;
    pub fn varcache_apply(
        server: *mut PgSocket,
        client: *mut PgSocket,
        changes_p: *mut bool,
    ) -> bool;
    pub fn varcache_clean(cache: *mut VarCache);
}
