pub mod internal {

    pub type __builtin_va_list = *mut ::core::ffi::c_char;
}

pub mod sys__types_h {

    pub type __darwin_gid_t = __uint32_t;

    pub type __darwin_off_t = __int64_t;

    pub type __darwin_pid_t = __int32_t;

    pub type __darwin_suseconds_t = __int32_t;

    pub type __darwin_uid_t = __uint32_t;

    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    use crate::types::{__int32_t, __int64_t, __uint32_t};
}

pub mod _gid_t_h {

    pub type gid_t = __darwin_gid_t;
    use super::sys__types_h::__darwin_gid_t;
}

pub mod _va_list_h {

    pub type va_list = __darwin_va_list;
    use crate::types::__darwin_va_list;
}

pub mod tls_h {
    use crate::types::size_t;
    use crate::types::ssize_t;
    extern "C" {

        pub type tls;

        pub fn tls_get_connection_info(
            ctx: *mut tls,
            buf: *mut ::core::ffi::c_char,
            buflen: size_t,
        ) -> ssize_t;
    }
}

pub mod _iovec_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct iovec {
        pub iov_base: *mut ::core::ffi::c_void,
        pub iov_len: size_t,
    }
    use crate::types::size_t;
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
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct CfLookup {
        pub name: *const ::core::ffi::c_char,
        pub value: ::core::ffi::c_int,
    }
    extern "C" {

        pub fn cf_get_lookup(cv: *mut CfValue) -> *const ::core::ffi::c_char;
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
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct msghdr {
        pub msg_name: *mut ::core::ffi::c_void,
        pub msg_namelen: socklen_t,
        pub msg_iov: *mut iovec,
        pub msg_iovlen: ::core::ffi::c_int,
        pub msg_control: *mut ::core::ffi::c_void,
        pub msg_controllen: socklen_t,
        pub msg_flags: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct cmsghdr {
        pub cmsg_len: socklen_t,
        pub cmsg_level: ::core::ffi::c_int,
        pub cmsg_type: ::core::ffi::c_int,
    }

    pub const SOL_SOCKET: ::core::ffi::c_int = 0xffff as ::core::ffi::c_int;

    pub const AF_UNIX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    pub const SCM_RIGHTS: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    use super::_iovec_t_h::iovec;
    use super::_socklen_t_h::socklen_t;
    use crate::types::__uint8_t;
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
    extern "C" {

        pub type event_base;

        pub fn event_base_loopbreak(_: *mut event_base) -> ::core::ffi::c_int;
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

    pub const AUTH_TYPE_PAM: auth_type = 7;

    pub const AUTH_TYPE_ANY: auth_type = 0;

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

    pub const POOL_STMT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

    pub const POOL_INHERIT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    #[inline]

    pub unsafe extern "C" fn pga_is_unix(mut a: *const PgAddr) -> bool {
        (*a).sa.sa_family as ::core::ffi::c_int == AF_UNIX
    }
    #[inline]

    pub unsafe extern "C" fn first_socket(mut slist: *mut StatList) -> *mut PgSocket {
        if statlist_empty(slist) {
            return ::core::ptr::null_mut::<PgSocket>();
        }
        ((*slist).head.next as *mut ::core::ffi::c_char)
             as *mut PgSocket
    }

    use crate::types::pid_t;

    use super::cfparser_h::CfLookup;
    use super::dnslookup_h::{DNSContext, DNSToken};
    use super::event_h::event_base;
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

        pub static mut pgb_event_base: *mut event_base;

        pub fn pga_port(a: *const PgAddr) -> ::core::ffi::c_int;

        pub fn pga_ntop(
            a: *const PgAddr,
            dst: *mut ::core::ffi::c_char,
            dstlen: ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char;

        pub static mut cf_listen_port: ::core::ffi::c_int;

        pub static mut cf_default_pool_size: ::core::ffi::c_int;

        pub static mut cf_min_pool_size: ::core::ffi::c_int;

        pub static mut cf_res_pool_size: ::core::ffi::c_int;

        pub static mut cf_server_lifetime: usec_t;

        pub static mut cf_auth_type: ::core::ffi::c_int;

        pub static mut cf_admin_users: *mut ::core::ffi::c_char;

        pub static mut cf_stats_users: *mut ::core::ffi::c_char;

        pub static mut cf_pause_mode: ::core::ffi::c_int;

        pub static mut cf_shutdown: ::core::ffi::c_int;

        pub static mut cf_log_connections: ::core::ffi::c_int;

        pub static pool_mode_map: [CfLookup; 0];

        pub static load_balance_hosts_map: [CfLookup; 0];

        pub static mut g_suspend_start: usec_t;

        pub static mut adns: *mut DNSContext;

        pub fn load_config() -> bool;

        pub fn set_config_param(
            key: *const ::core::ffi::c_char,
            val: *const ::core::ffi::c_char,
        ) -> bool;

        pub fn config_for_each(
            param_cb: Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    bool,
                ) -> (),
            >,
            arg: *mut ::core::ffi::c_void,
        );
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

    pub unsafe extern "C" fn sbuf_op_send(
        mut sbuf: *mut SBuf,
        mut buf: *const ::core::ffi::c_void,
        mut len: size_t,
    ) -> ssize_t {
        (*(*sbuf).ops)
            .sbufio_send
            .expect("non-null function pointer")(sbuf, buf, len)
    }
    use super::event_struct_h::event;
    use super::iobuf_h::{iobuf_empty, IOBuf};
    use super::tls_h::tls;
    use crate::types::size_t;
    use crate::types::ssize_t;
    use crate::types::uint8_t;
    use crate::types::MBuf;
    extern "C" {

        pub fn sbuf_tls_setup() -> bool;

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
    #[inline]

    pub unsafe extern "C" fn iobuf_empty(mut io: *const IOBuf) -> bool {
        io.is_null() || (*io).done_pos == (*io).recv_pos
    }
    #[inline]

    pub unsafe extern "C" fn iobuf_amount_pending(mut buf: *const IOBuf) -> ::core::ffi::c_uint {
        (*buf).parse_pos.wrapping_sub((*buf).done_pos)
    }
    #[inline]

    pub unsafe extern "C" fn iobuf_amount_parse(mut buf: *const IOBuf) -> ::core::ffi::c_uint {
        (*buf).recv_pos.wrapping_sub((*buf).parse_pos)
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
    use crate::types::uint8_t;
    extern "C" {

        pub fn pktbuf_dynamic(start_len: ::core::ffi::c_int) -> *mut PktBuf;

        pub fn pktbuf_static(buf: *mut PktBuf, data: *mut uint8_t, len: ::core::ffi::c_int);

        pub fn pktbuf_temp() -> *mut PktBuf;

        pub fn pktbuf_send_immediate(buf: *mut PktBuf, sk: *mut PgSocket) -> bool;

        pub fn pktbuf_send_queued(buf: *mut PktBuf, sk: *mut PgSocket) -> bool;

        pub fn pktbuf_put_string(buf: *mut PktBuf, str: *const ::core::ffi::c_char);

        pub fn pktbuf_write_generic(
            buf: *mut PktBuf,
            type_0: ::core::ffi::c_int,
            fmt: *const ::core::ffi::c_char,
            ...
        );

        pub fn pktbuf_write_RowDescription(
            buf: *mut PktBuf,
            tupdesc: *const ::core::ffi::c_char,
            ...
        );

        pub fn pktbuf_write_DataRow(buf: *mut PktBuf, tupdesc: *const ::core::ffi::c_char, ...);
    }
}

pub mod dnslookup_h {

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
    use super::netdb_h::addrinfo;
    use crate::types::uint32_t;
    use crate::types::usec_t;
    extern "C" {

        pub type DNSToken;

        pub type DNSContext;

        pub fn adns_info(
            ctx: *mut DNSContext,
            names: *mut ::core::ffi::c_int,
            zones: *mut ::core::ffi::c_int,
            queries: *mut ::core::ffi::c_int,
            pending: *mut ::core::ffi::c_int,
        );

        pub fn adns_walk_names(
            ctx: *mut DNSContext,
            cb: adns_walk_name_f,
            arg: *mut ::core::ffi::c_void,
        );

        pub fn adns_walk_zones(
            ctx: *mut DNSContext,
            cb: adns_walk_zone_f,
            arg: *mut ::core::ffi::c_void,
        );
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

pub mod _regex_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct regmatch_t {
        pub rm_so: regoff_t,
        pub rm_eo: regoff_t,
    }

    pub type regoff_t = __darwin_off_t;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct regex_t {
        pub re_magic: ::core::ffi::c_int,
        pub re_nsub: size_t,
        pub re_endp: *const ::core::ffi::c_char,
        pub re_g: *mut re_guts,
    }

    pub const REG_EXTENDED: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;

    pub const REG_ICASE: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
    use super::sys__types_h::__darwin_off_t;
    use crate::types::size_t;
    extern "C" {

        pub type re_guts;

        pub fn regcomp(
            _: *mut regex_t,
            _: *const ::core::ffi::c_char,
            _: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;

        pub fn regexec(
            _: *const regex_t,
            _: *const ::core::ffi::c_char,
            __nmatch: size_t,
            __pmatch: *mut regmatch_t,
            _: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;

        pub fn regfree(_: *mut regex_t);
    }
}

pub mod slab_h {

    pub type slab_stat_fn = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            ::core::ffi::c_uint,
            ::core::ffi::c_uint,
            ::core::ffi::c_uint,
        ) -> (),
    >;
    use super::objects_h::Slab;
    extern "C" {

        pub fn slab_free_count(slab: *const Slab) -> ::core::ffi::c_int;

        pub fn slab_active_count(slab: *const Slab) -> ::core::ffi::c_int;

        pub fn slab_stats(cb_func: slab_stat_fn, cb_arg: *mut ::core::ffi::c_void);
    }
}

pub mod objects_h {
    use super::bouncer_h::{PgCredentials, PgDatabase, PgGlobalUser, PgPool, PgSocket};

    use crate::types::StatList;
    extern "C" {

        pub type Slab;

        pub static mut user_list: StatList;

        pub static mut pool_list: StatList;

        pub static mut peer_pool_list: StatList;

        pub static mut database_list: StatList;

        pub static mut peer_list: StatList;

        pub static mut login_client_list: StatList;

        pub static mut client_cache: *mut Slab;

        pub static mut server_cache: *mut Slab;

        pub fn find_database(name: *const ::core::ffi::c_char) -> *mut PgDatabase;

        pub fn find_or_register_database(
            connection: *mut PgSocket,
            name: *const ::core::ffi::c_char,
        ) -> *mut PgDatabase;

        pub fn find_global_user(name: *const ::core::ffi::c_char) -> *mut PgGlobalUser;

        pub fn get_pool(db: *mut PgDatabase, user_credentials: *mut PgCredentials) -> *mut PgPool;

        pub fn disconnect_client(
            client: *mut PgSocket,
            notify: bool,
            reason: *const ::core::ffi::c_char,
            ...
        );

        pub fn add_database(name: *const ::core::ffi::c_char) -> *mut PgDatabase;

        pub fn force_user_credentials(
            db: *mut PgDatabase,
            username: *const ::core::ffi::c_char,
            passwd: *const ::core::ffi::c_char,
        ) -> *mut PgCredentials;

        pub fn find_or_add_new_global_user(
            name: *const ::core::ffi::c_char,
            passwd: *const ::core::ffi::c_char,
        ) -> *mut PgGlobalUser;

        pub fn tag_database_dirty(db: *mut PgDatabase);
    }
}

pub mod pooler_h {

    pub type pooler_cb = Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int, *const PgAddr) -> bool,
    >;
    use super::bouncer_h::PgAddr;
    extern "C" {

        pub fn suspend_pooler();

        pub fn cleanup_tcp_sockets();

        pub fn for_each_pooler_fd(cb: pooler_cb, arg: *mut ::core::ffi::c_void) -> bool;
    }
}

pub mod _stdio_h {
    use crate::types::size_t;

    extern "C" {

        pub fn sscanf(
            _: *const ::core::ffi::c_char,
            _: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;

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

pub mod unistd_h {
    use super::_gid_t_h::gid_t;
    use crate::types::uid_t;
    extern "C" {

        pub fn getuid() -> uid_t;

        pub fn getpeereid(
            _: ::core::ffi::c_int,
            _: *mut uid_t,
            _: *mut gid_t,
        ) -> ::core::ffi::c_int;
    }
}

pub mod usual_socket_h {
    use super::socket_h::sockaddr;
    use crate::types::size_t;
    extern "C" {

        pub fn socket_set_nonblocking(sock: ::core::ffi::c_int, non_block: bool) -> bool;

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
    }
}

pub mod stats_h {
    use super::bouncer_h::PgSocket;
    use crate::types::StatList;
    extern "C" {

        pub fn admin_database_stats(client: *mut PgSocket, pool_list_0: *mut StatList) -> bool;

        pub fn admin_database_stats_totals(
            client: *mut PgSocket,
            pool_list_0: *mut StatList,
        ) -> bool;

        pub fn admin_database_stats_averages(
            client: *mut PgSocket,
            pool_list_0: *mut StatList,
        ) -> bool;

        pub fn show_stat_totals(client: *mut PgSocket, pool_list_0: *mut StatList) -> bool;
    }
}

pub mod server_h {
    use super::bouncer_h::{PgDatabase, PgGlobalUser, PgPool};
    extern "C" {

        pub fn probably_wrong_pool_pool_mode(pool: *mut PgPool) -> ::core::ffi::c_int;

        pub fn database_max_connections(db: *mut PgDatabase) -> ::core::ffi::c_int;

        pub fn database_max_client_connections(db: *mut PgDatabase) -> ::core::ffi::c_int;

        pub fn user_max_connections(user: *mut PgGlobalUser) -> ::core::ffi::c_int;

        pub fn user_client_max_connections(user: *mut PgGlobalUser) -> ::core::ffi::c_int;
    }
}

pub mod endian_h {
    #[inline]

    pub unsafe extern "C" fn usual_bswap32(mut x: uint32_t) -> uint32_t {
        x.swap_bytes()
    }
    #[inline]

    pub unsafe extern "C" fn usual_be32dec(mut p: *const ::core::ffi::c_void) -> uint32_t {
        let mut tmp: uint32_t = 0;
        memcpy(
            &raw mut tmp as *mut ::core::ffi::c_void,
            p,
            ::core::mem::size_of::<uint32_t>() as size_t,
        );
        usual_bswap32(tmp)
    }
    use super::_string_h::memcpy;
    use crate::types::size_t;
    use crate::types::uint32_t;
}

pub mod safeio_h {
    use super::socket_h::msghdr;
    use crate::types::ssize_t;
    extern "C" {

        pub fn safe_sendmsg(
            fd: ::core::ffi::c_int,
            msg: *const msghdr,
            flags: ::core::ffi::c_int,
        ) -> ssize_t;
    }
}

pub mod _param_h {

    pub const __DARWIN_ALIGNBYTES32: usize =
        ::core::mem::size_of::<__uint32_t>().wrapping_sub(1_usize);
    use crate::types::__uint32_t;
}

pub mod janitor_h {
    use super::bouncer_h::PgPool;
    extern "C" {

        pub fn resume_all();

        pub fn kill_pool(pool: *mut PgPool);
    }
}

pub mod client_h {
    use super::bouncer_h::PgSocket;
    extern "C" {

        pub fn check_db_connection_count(client: *mut PgSocket) -> bool;

        pub fn check_user_connection_count(client: *mut PgSocket) -> bool;
    }
}

pub mod config_h {

    pub const PACKAGE_STRING: [::core::ffi::c_char; 17] = unsafe {
        ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"PgBouncer 1.25.1\0")
    };
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

pub mod _strings_h {
    extern "C" {

        pub fn strcasecmp(
            _: *const ::core::ffi::c_char,
            _: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}

pub mod util_h {
    extern "C" {

        pub fn strlist_contains(
            liststr: *const ::core::ffi::c_char,
            str: *const ::core::ffi::c_char,
        ) -> bool;
    }
}

pub mod protocol_h {

    pub const PqMsg_Bind: ::core::ffi::c_uint = 66 as ::core::ffi::c_uint;

    pub const PqMsg_Execute: ::core::ffi::c_uint = 69 as ::core::ffi::c_uint;

    pub const PqMsg_Parse: ::core::ffi::c_uint = 80 as ::core::ffi::c_uint;

    pub const PqMsg_Query: ::core::ffi::c_uint = 81 as ::core::ffi::c_uint;

    pub const PqMsg_Terminate: ::core::ffi::c_uint = 88 as ::core::ffi::c_uint;

    pub const PqMsg_CommandComplete: ::core::ffi::c_int = 'C' as i32;

    pub const PqMsg_NoticeResponse: ::core::ffi::c_int = 'N' as i32;

    pub const PqMsg_AuthenticationRequest: ::core::ffi::c_int = 'R' as i32;

    pub const PqMsg_ParameterStatus: ::core::ffi::c_int = 'S' as i32;

    pub const PqMsg_ReadyForQuery: ::core::ffi::c_int = 'Z' as i32;
}
pub use self::_gid_t_h::gid_t;
pub use self::_iovec_t_h::iovec;
pub use self::_param_h::__DARWIN_ALIGNBYTES32;
pub use self::_regex_h::{
    re_guts, regcomp, regex_t, regexec, regfree, regmatch_t, regoff_t, REG_EXTENDED, REG_ICASE,
};
pub use self::_socklen_t_h::socklen_t;
use self::_stdio_h::{snprintf, sscanf, vsnprintf};
use self::_stdlib_h::exit;
use self::_string_h::{memcpy, memset, strchr, strcmp, strerror, strlen, strstr};
use self::_strings_h::strcasecmp;
pub use self::_va_list_h::va_list;
pub use self::bouncer_h::{
    adns, auth_type, cf_admin_users, cf_auth_type, cf_default_pool_size, cf_listen_port,
    cf_log_connections, cf_min_pool_size, cf_pause_mode, cf_res_pool_size, cf_server_lifetime,
    cf_shutdown, cf_stats_users, config_for_each, first_socket, g_suspend_start,
    load_balance_hosts_map, load_config, pga_is_unix, pga_ntop, pga_port, pgb_event_base,
    pool_mode_map, set_config_param, sockaddr_ucreds, C2RustUnnamed_9, CallbackState,
    LoadBalanceHosts, PacketCallbackFlag, PauseMode, PgAddr, PgCredentials, PgDatabase,
    PgGlobalUser, PgPool, PgSocket, PgStats, ReplicationType, ScramState, ShutDownMode,
    SocketState, AUTH_TYPE_ANY, AUTH_TYPE_CERT, AUTH_TYPE_HBA, AUTH_TYPE_LDAP, AUTH_TYPE_MD5,
    AUTH_TYPE_PAM, AUTH_TYPE_PEER, AUTH_TYPE_PLAIN, AUTH_TYPE_REJECT, AUTH_TYPE_SCRAM_SHA_256,
    AUTH_TYPE_TRUST, CB_HANDLE_COMPLETE_PACKET, CB_NONE, CB_WANT_COMPLETE_PACKET, CL_ACTIVE,
    CL_ACTIVE_CANCEL, CL_FREE, CL_JUSTFREE, CL_LOGIN, CL_WAITING, CL_WAITING_CANCEL,
    CL_WAITING_LOGIN, LOAD_BALANCE_HOSTS_DISABLE, LOAD_BALANCE_HOSTS_ROUND_ROBIN, POOL_INHERIT,
    POOL_STMT, P_NONE, P_PAUSE, P_SUSPEND, REPLICATION_LOGICAL, REPLICATION_NONE,
    REPLICATION_PHYSICAL, SHUTDOWN_IMMEDIATE, SHUTDOWN_NONE, SHUTDOWN_WAIT_FOR_CLIENTS,
    SHUTDOWN_WAIT_FOR_SERVERS, SV_ACTIVE, SV_ACTIVE_CANCEL, SV_BEING_CANCELED, SV_FREE, SV_IDLE,
    SV_JUSTFREE, SV_LOGIN, SV_TESTED, SV_USED,
};
pub use self::cfparser_h::{cf_get_lookup, CfLookup, CfValue};
use self::client_h::{check_db_connection_count, check_user_connection_count};
pub use self::config_h::PACKAGE_STRING;
pub use self::dnslookup_h::{
    adns_info, adns_walk_name_f, adns_walk_names, adns_walk_zone_f, adns_walk_zones, DNSContext,
    DNSToken,
};
pub use self::endian_h::{usual_be32dec, usual_bswap32};
use self::errno_h::__error;
use self::event_h::event_base_loopbreak;
pub use self::event_struct_h::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
pub use self::in6_h::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use self::in_h::{in_addr, sockaddr_in};
pub use self::internal::__builtin_va_list;
pub use self::iobuf_h::{iobuf, iobuf_amount_parse, iobuf_amount_pending, iobuf_empty, IOBuf};
use self::janitor_h::{kill_pool, resume_all};
pub use self::logging_h::{
    cf_verbose, log_fatal, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE,
    LG_STATS, LG_WARNING,
};
pub use self::netdb_h::addrinfo;
use self::objects_h::{
    add_database, client_cache, database_list, disconnect_client, find_database, find_global_user,
    find_or_add_new_global_user, find_or_register_database, force_user_credentials, get_pool,
    login_client_list, peer_list, peer_pool_list, pool_list, server_cache, tag_database_dirty,
    user_list,
};
pub use self::pktbuf_h::{
    pktbuf_dynamic, pktbuf_put_string, pktbuf_send_immediate, pktbuf_send_queued, pktbuf_static,
    pktbuf_temp, pktbuf_write_DataRow, pktbuf_write_RowDescription, pktbuf_write_generic, PktBuf,
};
pub use self::pooler_h::{cleanup_tcp_sockets, for_each_pooler_fd, pooler_cb, suspend_pooler};
pub use crate::lib::usual::mbuf::{
    mbuf_avail_for_read, mbuf_get_string, mbuf_get_uint32be, mbuf_get_uint64be,
    mbuf_init_fixed_reader, mbuf_written,
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
pub use crate::types::MBuf;
pub use crate::types::NULL;
pub use crate::types::{
    __darwin_ptrdiff_t, __darwin_size_t, __darwin_socklen_t, __darwin_ssize_t, __darwin_time_t,
    __darwin_va_list, __int32_t, __int64_t, __uint16_t, __uint32_t, __uint8_t,
};
pub use crate::types::{aatree_cmp_f, aatree_walker_f, AANode, AATree};
pub use crate::types::{incomplete_pkt, pkt_desc, PktHdr};
pub use crate::types::{list_empty, List};
pub use crate::types::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};
pub use crate::types::{PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement};

// External function declaration (defined in proto.rs)
extern "C" {
    pub fn send_pooler_error(
        client: *mut bouncer_h::PgSocket,
        send_ready: bool,
        sqlstate: *const ::core::ffi::c_char,
        level_fatal: bool,
        msg: *const ::core::ffi::c_char,
    ) -> bool;
}
pub use self::protocol_h::{
    PqMsg_AuthenticationRequest, PqMsg_Bind, PqMsg_CommandComplete, PqMsg_Execute,
    PqMsg_NoticeResponse, PqMsg_ParameterStatus, PqMsg_Parse, PqMsg_Query, PqMsg_ReadyForQuery,
    PqMsg_Terminate,
};
use self::safeio_h::safe_sendmsg;
pub use self::sbuf_h::{
    sbuf_cb_t, sbuf_is_empty, sbuf_op_send, sbuf_prepare_skip, sbuf_tls_setup, SBuf, SBufEvent,
    SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK, SBUF_EV_FLUSH, SBUF_EV_PKT_CALLBACK,
    SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED, SBUF_EV_TLS_READY,
};
use self::server_h::{
    database_max_client_connections, database_max_connections, probably_wrong_pool_pool_mode,
    user_client_max_connections, user_max_connections,
};
pub use self::slab_h::{slab_active_count, slab_free_count, slab_stat_fn, slab_stats};
pub use self::socket_h::{cmsghdr, msghdr, sockaddr, AF_UNIX, SCM_RIGHTS, SOL_SOCKET};
use self::stats_h::{
    admin_database_stats, admin_database_stats_averages, admin_database_stats_totals,
    show_stat_totals,
};
pub use self::sys__types_h::{
    __darwin_gid_t, __darwin_off_t, __darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t,
    __DARWIN_NULL,
};
use self::tls_h::tls_get_connection_info;
use self::unistd_h::{getpeereid, getuid};
use self::usual_socket_h::{sa2str, socket_set_nonblocking};
use self::util_h::strlist_contains;
pub use crate::types::{false_0, true_0};
pub use crate::types::{statlist_count, statlist_empty, StatList};
pub use crate::types::{usec_t, USEC};
pub use crate::types::{
    NumVars, VAppName, VClientEncoding, VDateStyle, VStdStr, VTimeZone, VarCache, VarCacheIdx,
};
pub use crate::types::{PStr, StrPool};
pub use crate::types::{UT_hash_bucket, UT_hash_handle, UT_hash_table};
#[derive(Copy, Clone)]
#[repr(C)]

pub struct FakeParam {
    pub name: *const ::core::ffi::c_char,
    pub value: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct cmd_lookup {
    pub word: *const ::core::ffi::c_char,
    pub func: cmd_func_t,
}

pub type cmd_func_t =
    Option<unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool>;

pub const MAX_GROUPS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;

pub const CMD_NAME: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const CMD_ARG: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const SET_KEY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SET_VAL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

static mut cmd_normal_rx: [::core::ffi::c_char; 90] = unsafe {
    ::core::mem::transmute::<
        [u8; 90],
        [::core::ffi::c_char; 90],
    >(
        *b"^[ \t\n\r]*(\"([^\"]+|\"\")*\"|[0-9a-z_]+)([ \t\n\r]+(\"([^\"]+|\"\")*\"|[0-9a-z_]+))?[ \t\n\r]*(;[ \t\n\r]*)?$\0",
    )
};

static mut cmd_set_word_rx: [::core::ffi::c_char; 110] = unsafe {
    ::core::mem::transmute::<
        [u8; 110],
        [::core::ffi::c_char; 110],
    >(
        *b"^[ \t\n\r]*set[ \t\n\r]+(\"([^\"]+|\"\")*\"|[0-9a-z_]+)[ \t\n\r]*(=|to)[ \t\n\r]*(\"([^\"]+|\"\")*\"|[0-9a-z_]+)[ \t\n\r]*(;[ \t\n\r]*)?$\0",
    )
};

static mut cmd_set_str_rx: [::core::ffi::c_char; 98] = unsafe {
    ::core::mem::transmute::<
        [u8; 98],
        [::core::ffi::c_char; 98],
    >(
        *b"^[ \t\n\r]*set[ \t\n\r]+(\"([^\"]+|\"\")*\"|[0-9a-z_]+)[ \t\n\r]*(=|to)[ \t\n\r]*('([^']|'')*')[ \t\n\r]*(;[ \t\n\r]*)?$\0",
    )
};

static mut rc_cmd: regex_t = regex_t {
    re_magic: 0,
    re_nsub: 0,
    re_endp: ::core::ptr::null::<::core::ffi::c_char>(),
    re_g: ::core::ptr::null::<re_guts>() as *mut re_guts,
};

static mut rc_set_word: regex_t = regex_t {
    re_magic: 0,
    re_nsub: 0,
    re_endp: ::core::ptr::null::<::core::ffi::c_char>(),
    re_g: ::core::ptr::null::<re_guts>() as *mut re_guts,
};

static mut rc_set_str: regex_t = regex_t {
    re_magic: 0,
    re_nsub: 0,
    re_endp: ::core::ptr::null::<::core::ffi::c_char>(),
    re_g: ::core::ptr::null::<re_guts>() as *mut re_guts,
};

static mut admin_pool: *mut PgPool = ::core::ptr::null::<PgPool>() as *mut PgPool;

static mut current_query: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
#[no_mangle]

pub unsafe extern "C" fn admin_cleanup() {
    regfree(&raw mut rc_cmd);
    regfree(&raw mut rc_set_str);
    regfree(&raw mut rc_set_word);
    admin_pool = ::core::ptr::null_mut::<PgPool>();
}

unsafe extern "C" fn syntax_error(mut admin: *mut PgSocket) -> bool {
    admin_error(
        admin,
        c"invalid command '%s', use SHOW HELP;".as_ptr(),
        if !current_query.is_null() {
            current_query
        } else {
            c"<no query>".as_ptr()
        },
    )
}

unsafe extern "C" fn exec_cmd(
    mut lookup: *mut cmd_lookup,
    mut admin: *mut PgSocket,
    mut cmd: *const ::core::ffi::c_char,
    mut arg: *const ::core::ffi::c_char,
) -> bool {
    while !(*lookup).word.is_null() {
        if strcasecmp((*lookup).word, cmd) == 0 as ::core::ffi::c_int {
            return (*lookup).func.expect("non-null function pointer")(admin, arg);
        }
        lookup = lookup.offset(1);
    }
    syntax_error(admin)
}
#[no_mangle]

pub unsafe extern "C" fn admin_error(
    mut admin: *mut PgSocket,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ...
) -> bool {
    let mut str: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut ap: ::core::ffi::VaListImpl;
    let mut res = true;
    ap = args.clone();
    vsnprintf(
        &raw mut str as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
        fmt,
        ap.as_va_list(),
    );
    let mut _log_ctx = NULL;
    log_generic(
        LG_ERROR,
        _log_ctx,
        c"%s".as_ptr(),
        &raw mut str as *mut ::core::ffi::c_char,
    );
    if !admin.is_null() {
        res = send_pooler_error(
            admin,
            true,
            ::core::ptr::null::<::core::ffi::c_char>(),
            false,
            &raw mut str as *mut ::core::ffi::c_char,
        );
    }
    res
}

unsafe extern "C" fn count_paused_databases() -> ::core::ffi::c_int {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut db = ::core::ptr::null_mut::<PgDatabase>();
    let mut cnt = 0 as ::core::ffi::c_int;
    item = database_list.head.next;
    while item != &raw mut database_list.head {
        db = (item as *mut ::core::ffi::c_char)
            as *mut PgDatabase;
        cnt += (*db).db_paused as ::core::ffi::c_int;
        item = (*item).next;
    }
    cnt
}

unsafe extern "C" fn count_db_active(mut db: *mut PgDatabase) -> ::core::ffi::c_int {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut cnt = 0 as ::core::ffi::c_int;
    item = pool_list.head.next;
    while item != &raw mut pool_list.head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        if (*pool).db == db {
            cnt += statlist_count(&raw mut (*pool).active_server_list)
                + statlist_count(&raw mut (*pool).being_canceled_server_list)
                + statlist_count(&raw mut (*pool).idle_server_list)
                + statlist_count(&raw mut (*pool).tested_server_list)
                + statlist_count(&raw mut (*pool).used_server_list)
                + statlist_count(&raw mut (*pool).new_server_list)
                + statlist_count(&raw mut (*pool).active_cancel_server_list);
        }
        item = (*item).next;
    }
    cnt
}
#[no_mangle]

pub unsafe extern "C" fn admin_flush(
    mut admin: *mut PgSocket,
    mut buf: *mut PktBuf,
    mut desc: *const ::core::ffi::c_char,
) -> bool {
    pktbuf_write_generic(buf, PqMsg_CommandComplete, c"s".as_ptr(), desc);
    pktbuf_write_generic(buf, PqMsg_ReadyForQuery, c"c".as_ptr(), 'I' as i32);
    pktbuf_send_queued(buf, admin)
}
#[no_mangle]

pub unsafe extern "C" fn admin_ready(
    mut admin: *mut PgSocket,
    mut desc: *const ::core::ffi::c_char,
) -> bool {
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
    let mut tmp: [uint8_t; 512] = [0; 512];
    pktbuf_static(
        &raw mut buf,
        &raw mut tmp as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 512]>() as ::core::ffi::c_int,
    );
    pktbuf_write_generic(&raw mut buf, PqMsg_CommandComplete, c"s".as_ptr(), desc);
    pktbuf_write_generic(&raw mut buf, PqMsg_ReadyForQuery, c"c".as_ptr(), 'I' as i32);
    pktbuf_send_immediate(&raw mut buf, admin)
}

static mut fake_param_list: [FakeParam; 6] = [
    FakeParam {
        name: c"client_encoding".as_ptr(),
        value: c"UTF-8".as_ptr(),
    },
    FakeParam {
        name: c"default_transaction_isolation".as_ptr(),
        value: c"read committed".as_ptr(),
    },
    FakeParam {
        name: c"standard_conforming_strings".as_ptr(),
        value: c"on".as_ptr(),
    },
    FakeParam {
        name: c"datestyle".as_ptr(),
        value: c"ISO".as_ptr(),
    },
    FakeParam {
        name: c"timezone".as_ptr(),
        value: c"GMT".as_ptr(),
    },
    FakeParam {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        value: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];

unsafe extern "C" fn fake_show(
    mut admin: *mut PgSocket,
    mut name: *const ::core::ffi::c_char,
) -> bool {
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    let mut p = ::core::ptr::null::<FakeParam>();
    let mut got = false;
    p = &raw const fake_param_list as *const FakeParam;
    while !(*p).name.is_null() {
        if strcasecmp(name, (*p).name) == 0 as ::core::ffi::c_int {
            got = true;
            break;
        } else {
            p = p.offset(1);
        }
    }
    if got {
        buf = pktbuf_dynamic(256 as ::core::ffi::c_int);
        if !buf.is_null() {
            pktbuf_write_RowDescription(buf, c"s".as_ptr(), (*p).name);
            pktbuf_write_DataRow(buf, c"s".as_ptr(), (*p).value);
            admin_flush(admin, buf, c"SHOW".as_ptr());
        } else {
            admin_error(admin, c"no mem".as_ptr());
        }
    }
    got
}

unsafe extern "C" fn fake_set(
    mut admin: *mut PgSocket,
    mut key: *const ::core::ffi::c_char,
    mut _val: *const ::core::ffi::c_char,
) -> bool {
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    let mut p = ::core::ptr::null::<FakeParam>();
    let mut got = false;
    p = &raw const fake_param_list as *const FakeParam;
    while !(*p).name.is_null() {
        if strcasecmp(key, (*p).name) == 0 as ::core::ffi::c_int {
            got = true;
            break;
        } else {
            p = p.offset(1);
        }
    }
    if got {
        buf = pktbuf_dynamic(256 as ::core::ffi::c_int);
        if !buf.is_null() {
            pktbuf_write_generic(
                buf,
                PqMsg_NoticeResponse,
                c"sscss".as_ptr(),
                c"SNOTICE".as_ptr(),
                c"C00000".as_ptr(),
                'M' as i32,
                c"SET ignored".as_ptr(),
                c"".as_ptr(),
            );
            admin_flush(admin, buf, c"SET".as_ptr());
        } else {
            admin_error(admin, c"no mem".as_ptr());
        }
    }
    got
}

unsafe extern "C" fn admin_set(
    mut admin: *mut PgSocket,
    mut key: *const ::core::ffi::c_char,
    mut val: *const ::core::ffi::c_char,
) -> bool {
    let mut tmp: [::core::ffi::c_char; 512] = [0; 512];
    let mut ok: bool = false;
    if fake_set(admin, key, val) {
        return true;
    }
    if (*admin).admin_user() {
        ok = set_config_param(key, val);
        if ok {
            let mut buf = pktbuf_dynamic(256 as ::core::ffi::c_int);
            if buf.is_null() {
                return admin_error(admin, c"no mem".as_ptr());
            }
            if (!strstr(key, c"_tls_".as_ptr()).is_null()
                || !strstr(key, c"_tls13_".as_ptr()).is_null())
                && !sbuf_tls_setup()
            {
                pktbuf_write_generic(
                    buf,
                    PqMsg_NoticeResponse,
                    c"sscss".as_ptr(),
                    c"SNOTICE".as_ptr(),
                    c"C00000".as_ptr(),
                    'M' as i32,
                    c"TLS settings could not be applied, still using old configuration".as_ptr(),
                    c"".as_ptr(),
                );
            }
            snprintf(
                &raw mut tmp as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
                c"SET %s=%s".as_ptr(),
                key,
                val,
            );
            admin_flush(admin, buf, &raw mut tmp as *mut ::core::ffi::c_char)
        } else {
            admin_error(admin, c"SET failed".as_ptr())
        }
    } else {
        admin_error(admin, c"admin access needed".as_ptr())
    }
}

unsafe extern "C" fn send_one_fd(
    mut admin: *mut PgSocket,
    mut fd: ::core::ffi::c_int,
    mut task: *const ::core::ffi::c_char,
    mut user: *const ::core::ffi::c_char,
    mut db: *const ::core::ffi::c_char,
    mut addr: *const ::core::ffi::c_char,
    mut port: ::core::ffi::c_int,
    mut ckey: uint64_t,
    mut link: ::core::ffi::c_int,
    mut client_enc: *const ::core::ffi::c_char,
    mut std_strings: *const ::core::ffi::c_char,
    mut datestyle: *const ::core::ffi::c_char,
    mut timezone: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut scram_client_key: *const uint8_t,
    mut scram_client_key_len: ::core::ffi::c_int,
    mut scram_server_key: *const uint8_t,
    mut scram_server_key_len: ::core::ffi::c_int,
) -> bool {
    let mut msg = msghdr {
        msg_name: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_namelen: 0,
        msg_iov: ::core::ptr::null_mut::<iovec>(),
        msg_iovlen: 0,
        msg_control: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut cmsg = ::core::ptr::null_mut::<cmsghdr>();
    let mut iovec = iovec {
        iov_base: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        iov_len: 0,
    };
    let mut res: ssize_t = 0;
    let mut cntbuf: [uint8_t; 16] = [0; 16];
    let mut pkt = pktbuf_temp();
    pktbuf_write_DataRow(
        pkt as *mut PktBuf,
        c"issssiqisssssbb".as_ptr(),
        fd,
        task,
        user,
        db,
        addr,
        port,
        ckey,
        link,
        client_enc,
        std_strings,
        datestyle,
        timezone,
        password,
        scram_client_key_len,
        scram_client_key,
        scram_server_key_len,
        scram_server_key,
    );
    if (*pkt).failed() {
        return false;
    }
    iovec.iov_base = (*pkt).buf as *mut ::core::ffi::c_void;
    iovec.iov_len = (*pkt).write_pos as size_t;
    memset(
        &raw mut msg as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<msghdr>() as size_t,
    );
    msg.msg_iov = &raw mut iovec;
    msg.msg_iovlen = 1 as ::core::ffi::c_int;
    if pga_is_unix(&raw mut (*admin).remote_addr)
        && (*admin).own_user()
        && (*admin).sbuf.tls.is_null()
    {
        msg.msg_control = &raw mut cntbuf as *mut uint8_t as *mut ::core::ffi::c_void;
        msg.msg_controllen = ::core::mem::size_of::<[uint8_t; 16]>() as socklen_t;
        cmsg = if msg.msg_controllen as usize >= ::core::mem::size_of::<cmsghdr>() {
            msg.msg_control as *mut cmsghdr
        } else {
            ::core::ptr::null_mut::<cmsghdr>()
        };
        (*cmsg).cmsg_level = SOL_SOCKET;
        (*cmsg).cmsg_type = SCM_RIGHTS;
        (*cmsg).cmsg_len = (::core::mem::size_of::<cmsghdr>().wrapping_add(__DARWIN_ALIGNBYTES32)
            & !__DARWIN_ALIGNBYTES32)
            .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as __darwin_size_t)
            as socklen_t;
        memcpy(
            (cmsg as *mut ::core::ffi::c_uchar).add(
                ::core::mem::size_of::<cmsghdr>().wrapping_add(__DARWIN_ALIGNBYTES32)
                    & !__DARWIN_ALIGNBYTES32,
            ) as *mut ::core::ffi::c_void,
            &raw mut fd as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        );
        msg.msg_controllen = (*cmsg).cmsg_len;
    }
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            admin as *mut ::core::ffi::c_void,
            c"sending socket list: fd=%d, len=%d".as_ptr(),
            fd,
            msg.msg_controllen as ::core::ffi::c_int,
        );
    }
    if msg.msg_controllen != 0 {
        res = safe_sendmsg((*admin).sbuf.sock, &raw mut msg, 0 as ::core::ffi::c_int);
    } else {
        res = sbuf_op_send(
            &raw mut (*admin).sbuf,
            (*pkt).buf as *const ::core::ffi::c_void,
            (*pkt).write_pos as size_t,
        );
    }
    if res < 0 as ssize_t {
        let mut _log_ctx = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx,
            c"send_one_fd: sendmsg error: %s".as_ptr(),
            strerror(*__error()),
        );
        return false;
    } else if res as size_t != iovec.iov_len {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx_0,
            c"send_one_fd: partial sendmsg".as_ptr(),
        );
        return false;
    }
    true
}

unsafe extern "C" fn show_one_fd(mut admin: *mut PgSocket, mut sk: *mut PgSocket) -> bool {
    let mut addr: *mut PgAddr = &raw mut (*sk).remote_addr;
    let mut tmp = MBuf {
        data: ::core::ptr::null_mut::<uint8_t>(),
        read_pos: 0,
        write_pos: 0,
        alloc_len: 0,
        reader: false,
        fixed: false,
    };
    let mut v: *mut VarCache = &raw mut (*sk).vars;
    let mut ckey: uint64_t = 0;
    let mut client_encoding: *const PStr = *(*v)
        .var_list
        .offset(VClientEncoding as ::core::ffi::c_int as isize);
    let mut std_strings: *const PStr =
        *(*v).var_list.offset(VStdStr as ::core::ffi::c_int as isize);
    let mut datestyle: *const PStr = *(*v)
        .var_list
        .offset(VDateStyle as ::core::ffi::c_int as isize);
    let mut timezone: *const PStr = *(*v)
        .var_list
        .offset(VTimeZone as ::core::ffi::c_int as isize);
    let mut addrbuf: [::core::ffi::c_char; 56] = [0; 56];
    let mut password = ::core::ptr::null::<::core::ffi::c_char>();
    let mut send_scram_keys = false;
    if !(*sk).sbuf.tls.is_null() || !(*sk).link.is_null() && !(*(*sk).link).sbuf.tls.is_null() {
        return true;
    }
    mbuf_init_fixed_reader(
        &raw mut tmp,
        &raw mut (*sk).cancel_key as *mut uint8_t as *const ::core::ffi::c_void,
        8 as ::core::ffi::c_uint,
    );
    if !mbuf_get_uint64be(&raw mut tmp, &raw mut ckey) {
        return false;
    }
    if !(*sk).pool.is_null()
        && !(*(*(*sk).pool).db).auth_user_credentials.is_null()
        && !(*sk).login_user_credentials.is_null()
        && find_global_user(
            &raw mut (*(*sk).login_user_credentials).name as *mut ::core::ffi::c_char,
        )
        .is_null()
    {
        password = &raw mut (*(*sk).login_user_credentials).passwd as *mut ::core::ffi::c_char;
    }
    if cf_auth_type == AUTH_TYPE_PAM as ::core::ffi::c_int
        && find_global_user(
            &raw mut (*(*sk).login_user_credentials).name as *mut ::core::ffi::c_char,
        )
        .is_null()
    {
        password = &raw mut (*(*sk).login_user_credentials).passwd as *mut ::core::ffi::c_char;
    }
    if !(*sk).pool.is_null()
        && !(*(*sk).pool).user_credentials.is_null()
        && (*(*(*sk).pool).user_credentials).use_scram_keys as ::core::ffi::c_int != 0
    {
        send_scram_keys = true;
    }
    send_one_fd(
        admin,
        (*sk).sbuf.sock,
        if (*sk).state() as ::core::ffi::c_int >= SV_FREE as ::core::ffi::c_int {
            c"server".as_ptr()
        } else {
            c"client".as_ptr()
        },
        if !(*sk).login_user_credentials.is_null() {
            &raw mut (*(*sk).login_user_credentials).name as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        },
        if !(*sk).pool.is_null() {
            &raw mut (*(*(*sk).pool).db).name as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        },
        pga_ntop(
            addr,
            &raw mut addrbuf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 56]>() as ::core::ffi::c_int,
        ),
        pga_port(addr),
        ckey,
        if !(*sk).link.is_null() {
            (*(*sk).link).sbuf.sock
        } else {
            0 as ::core::ffi::c_int
        },
        if !client_encoding.is_null() {
            &raw const (*client_encoding).str_0 as *const ::core::ffi::c_char
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        },
        if !std_strings.is_null() {
            &raw const (*std_strings).str_0 as *const ::core::ffi::c_char
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        },
        if !datestyle.is_null() {
            &raw const (*datestyle).str_0 as *const ::core::ffi::c_char
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        },
        if !timezone.is_null() {
            &raw const (*timezone).str_0 as *const ::core::ffi::c_char
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        },
        password,
        if send_scram_keys {
            &raw mut (*(*(*sk).pool).user_credentials).scram_ClientKey as *mut uint8_t
        } else {
            ::core::ptr::null_mut::<uint8_t>()
        },
        if send_scram_keys {
            ::core::mem::size_of::<[uint8_t; 32]>() as ::core::ffi::c_int
        } else {
            -(1 as ::core::ffi::c_int)
        },
        if send_scram_keys {
            &raw mut (*(*(*sk).pool).user_credentials).scram_ServerKey as *mut uint8_t
        } else {
            ::core::ptr::null_mut::<uint8_t>()
        },
        if send_scram_keys {
            ::core::mem::size_of::<[uint8_t; 32]>() as ::core::ffi::c_int
        } else {
            -(1 as ::core::ffi::c_int)
        },
    )
}

unsafe extern "C" fn show_pooler_cb(
    mut arg: *mut ::core::ffi::c_void,
    mut fd: ::core::ffi::c_int,
    mut a: *const PgAddr,
) -> bool {
    let mut buf: [::core::ffi::c_char; 56] = [0; 56];
    send_one_fd(
        arg as *mut PgSocket,
        fd,
        c"pooler".as_ptr(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        pga_ntop(
            a,
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 56]>() as ::core::ffi::c_int,
        ),
        pga_port(a),
        0,
        0 as ::core::ffi::c_int,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<uint8_t>(),
        -(1 as ::core::ffi::c_int),
        ::core::ptr::null::<uint8_t>(),
        -(1 as ::core::ffi::c_int),
    )
}

unsafe extern "C" fn show_pooler_fds(mut admin: *mut PgSocket) -> bool {
    for_each_pooler_fd(
        Some(
            show_pooler_cb
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const PgAddr,
                ) -> bool,
        ),
        admin as *mut ::core::ffi::c_void,
    )
}

unsafe extern "C" fn show_fds_from_list(mut admin: *mut PgSocket, mut list: *mut StatList) -> bool {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut sk = ::core::ptr::null_mut::<PgSocket>();
    let mut res = true;
    item = (*list).head.next;
    while item != &raw mut (*list).head {
        sk = (item as *mut ::core::ffi::c_char)
            as *mut PgSocket;
        res = show_one_fd(admin, sk);
        if !res {
            break;
        }
        item = (*item).next;
    }
    res
}

unsafe extern "C" fn admin_show_fds(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut res: bool = false;
    if !(*admin).admin_user() {
        return admin_error(admin, c"admin access needed".as_ptr());
    }
    socket_set_nonblocking((*admin).sbuf.sock, false);
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
    pktbuf_write_RowDescription(
        &raw mut _buf,
        c"issssiqisssssbb".as_ptr(),
        c"fd".as_ptr(),
        c"task".as_ptr(),
        c"user".as_ptr(),
        c"database".as_ptr(),
        c"addr".as_ptr(),
        c"port".as_ptr(),
        c"cancel".as_ptr(),
        c"link".as_ptr(),
        c"client_encoding".as_ptr(),
        c"std_strings".as_ptr(),
        c"datestyle".as_ptr(),
        c"timezone".as_ptr(),
        c"password".as_ptr(),
        c"scram_client_key".as_ptr(),
        c"scram_server_key".as_ptr(),
    );
    res = pktbuf_send_immediate(&raw mut _buf, admin);
    if res {
        res = show_pooler_fds(admin);
    }
    if res {
        res = show_fds_from_list(admin, &raw mut login_client_list);
    }
    item = pool_list.head.next;
    while item != &raw mut pool_list.head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        if !(*(*pool).db).admin {
            res = res && show_fds_from_list(admin, &raw mut (*pool).active_client_list);
            res = res && show_fds_from_list(admin, &raw mut (*pool).waiting_client_list);
            res = res && show_fds_from_list(admin, &raw mut (*pool).active_server_list);
            res = res && show_fds_from_list(admin, &raw mut (*pool).idle_server_list);
            res = res && show_fds_from_list(admin, &raw mut (*pool).used_server_list);
            res = res && show_fds_from_list(admin, &raw mut (*pool).tested_server_list);
            res = res && show_fds_from_list(admin, &raw mut (*pool).new_server_list);
            if !res {
                break;
            }
        }
        item = (*item).next;
    }
    if res {
        res = admin_ready(admin, c"SHOW".as_ptr());
    }
    socket_set_nonblocking((*admin).sbuf.sock, true);
    res
}

unsafe extern "C" fn admin_show_databases(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    let mut db = ::core::ptr::null_mut::<PgDatabase>();
    let mut item = ::core::ptr::null_mut::<List>();
    let mut f_user = ::core::ptr::null::<::core::ffi::c_char>();
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    let mut cv = CfValue {
        value_p: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        extra: ::core::ptr::null::<::core::ffi::c_void>(),
        key_name: ::core::ptr::null::<::core::ffi::c_char>(),
        buf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        buflen: 0,
    };
    let mut load_balance_hosts_lookup = CfValue {
        value_p: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        extra: ::core::ptr::null::<::core::ffi::c_void>(),
        key_name: ::core::ptr::null::<::core::ffi::c_char>(),
        buf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        buflen: 0,
    };
    let mut pool_mode_str = ::core::ptr::null::<::core::ffi::c_char>();
    let mut server_lifetime_secs: usec_t = 0;
    let mut load_balance_hosts_str = ::core::ptr::null::<::core::ffi::c_char>();
    cv.extra = &raw const pool_mode_map as *const CfLookup as *const ::core::ffi::c_void;
    load_balance_hosts_lookup.extra =
        &raw const load_balance_hosts_map as *const CfLookup as *const ::core::ffi::c_void;
    buf = pktbuf_dynamic(256 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(admin, c"no mem".as_ptr());
        return true;
    }
    pktbuf_write_RowDescription(
        buf,
        c"ssissiiiissiiiiii".as_ptr(),
        c"name".as_ptr(),
        c"host".as_ptr(),
        c"port".as_ptr(),
        c"database".as_ptr(),
        c"force_user".as_ptr(),
        c"pool_size".as_ptr(),
        c"min_pool_size".as_ptr(),
        c"reserve_pool_size".as_ptr(),
        c"server_lifetime".as_ptr(),
        c"pool_mode".as_ptr(),
        c"load_balance_hosts".as_ptr(),
        c"max_connections".as_ptr(),
        c"current_connections".as_ptr(),
        c"max_client_connections".as_ptr(),
        c"current_client_connections".as_ptr(),
        c"paused".as_ptr(),
        c"disabled".as_ptr(),
    );
    item = database_list.head.next;
    while item != &raw mut database_list.head {
        db = (item as *mut ::core::ffi::c_char)
            as *mut PgDatabase;
        server_lifetime_secs = (if (*db).server_lifetime > 0 {
            (*db).server_lifetime
        } else {
            cf_server_lifetime
        })
        .wrapping_div(USEC);
        f_user = if !(*db).forced_user_credentials.is_null() {
            &raw mut (*(*db).forced_user_credentials).name as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
        pool_mode_str = ::core::ptr::null::<::core::ffi::c_char>();
        load_balance_hosts_str = ::core::ptr::null::<::core::ffi::c_char>();
        cv.value_p = &raw mut (*db).pool_mode as *mut ::core::ffi::c_void;
        load_balance_hosts_lookup.value_p =
            &raw mut (*db).load_balance_hosts as *mut ::core::ffi::c_void;
        if (*db).pool_mode != POOL_INHERIT {
            pool_mode_str = cf_get_lookup(&raw mut cv);
        }
        if !(*db).host.is_null() && !strchr((*db).host, ',' as i32).is_null() {
            load_balance_hosts_str = cf_get_lookup(&raw mut load_balance_hosts_lookup);
        }
        pktbuf_write_DataRow(
            buf,
            c"ssissiiiissiiiiii".as_ptr(),
            &raw mut (*db).name as *mut ::core::ffi::c_char,
            (*db).host,
            (*db).port,
            (*db).dbname,
            f_user,
            if (*db).pool_size >= 0 as ::core::ffi::c_int {
                (*db).pool_size
            } else {
                cf_default_pool_size
            },
            if (*db).min_pool_size >= 0 as ::core::ffi::c_int {
                (*db).min_pool_size
            } else {
                cf_min_pool_size
            },
            if (*db).res_pool_size >= 0 as ::core::ffi::c_int {
                (*db).res_pool_size
            } else {
                cf_res_pool_size
            },
            server_lifetime_secs,
            pool_mode_str,
            load_balance_hosts_str,
            database_max_connections(db),
            (*db).connection_count,
            database_max_client_connections(db),
            (*db).client_connection_count,
            (*db).db_paused as ::core::ffi::c_int,
            (*db).db_disabled as ::core::ffi::c_int,
        );
        item = (*item).next;
    }
    admin_flush(admin, buf, c"SHOW".as_ptr());
    true
}

unsafe extern "C" fn admin_show_peers(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    let mut peer = ::core::ptr::null_mut::<PgDatabase>();
    let mut item = ::core::ptr::null_mut::<List>();
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    buf = pktbuf_dynamic(256 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(admin, c"no mem".as_ptr());
        return true;
    }
    pktbuf_write_RowDescription(
        buf,
        c"isii".as_ptr(),
        c"peer_id".as_ptr(),
        c"host".as_ptr(),
        c"port".as_ptr(),
        c"pool_size".as_ptr(),
    );
    item = peer_list.head.next;
    while item != &raw mut peer_list.head {
        peer = (item as *mut ::core::ffi::c_char)
            as *mut PgDatabase;
        pktbuf_write_DataRow(
            buf,
            c"isii".as_ptr(),
            (*peer).peer_id,
            (*peer).host,
            (*peer).port,
            if (*peer).pool_size >= 0 as ::core::ffi::c_int {
                (*peer).pool_size
            } else {
                cf_default_pool_size
            },
        );
        item = (*item).next;
    }
    admin_flush(admin, buf, c"SHOW".as_ptr());
    true
}

unsafe extern "C" fn admin_show_lists(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    let mut buf = pktbuf_dynamic(256 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(admin, c"no mem".as_ptr());
        return true;
    }
    pktbuf_write_RowDescription(buf, c"si".as_ptr(), c"list".as_ptr(), c"items".as_ptr());
    pktbuf_write_DataRow(
        buf,
        c"si".as_ptr(),
        c"databases".as_ptr(),
        statlist_count(&raw mut database_list),
    );
    pktbuf_write_DataRow(
        buf,
        c"si".as_ptr(),
        c"users".as_ptr(),
        statlist_count(&raw mut user_list),
    );
    pktbuf_write_DataRow(
        buf,
        c"si".as_ptr(),
        c"peers".as_ptr(),
        statlist_count(&raw mut peer_list),
    );
    pktbuf_write_DataRow(
        buf,
        c"si".as_ptr(),
        c"pools".as_ptr(),
        statlist_count(&raw mut pool_list),
    );
    pktbuf_write_DataRow(
        buf,
        c"si".as_ptr(),
        c"peer_pools".as_ptr(),
        statlist_count(&raw mut peer_pool_list),
    );
    pktbuf_write_DataRow(
        buf,
        c"si".as_ptr(),
        c"free_clients".as_ptr(),
        slab_free_count(client_cache),
    );
    pktbuf_write_DataRow(
        buf,
        c"si".as_ptr(),
        c"used_clients".as_ptr(),
        slab_active_count(client_cache),
    );
    pktbuf_write_DataRow(
        buf,
        c"si".as_ptr(),
        c"login_clients".as_ptr(),
        statlist_count(&raw mut login_client_list),
    );
    pktbuf_write_DataRow(
        buf,
        c"si".as_ptr(),
        c"free_servers".as_ptr(),
        slab_free_count(server_cache),
    );
    pktbuf_write_DataRow(
        buf,
        c"si".as_ptr(),
        c"used_servers".as_ptr(),
        slab_active_count(server_cache),
    );
    let mut names: ::core::ffi::c_int = 0;
    let mut zones: ::core::ffi::c_int = 0;
    let mut qry: ::core::ffi::c_int = 0;
    let mut pend: ::core::ffi::c_int = 0;
    adns_info(
        adns,
        &raw mut names,
        &raw mut zones,
        &raw mut qry,
        &raw mut pend,
    );
    pktbuf_write_DataRow(buf, c"si".as_ptr(), c"dns_names".as_ptr(), names);
    pktbuf_write_DataRow(buf, c"si".as_ptr(), c"dns_zones".as_ptr(), zones);
    pktbuf_write_DataRow(buf, c"si".as_ptr(), c"dns_queries".as_ptr(), qry);
    pktbuf_write_DataRow(buf, c"si".as_ptr(), c"dns_pending".as_ptr(), pend);
    admin_flush(admin, buf, c"SHOW".as_ptr());
    true
}

unsafe extern "C" fn admin_show_users(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut buf = pktbuf_dynamic(256 as ::core::ffi::c_int);
    let mut cv = CfValue {
        value_p: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        extra: ::core::ptr::null::<::core::ffi::c_void>(),
        key_name: ::core::ptr::null::<::core::ffi::c_char>(),
        buf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        buflen: 0,
    };
    let mut pool_size_str: [::core::ffi::c_char; 12] =
        ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"\0\0\0\0\0\0\0\0\0\0\0\0");
    let mut res_pool_size_str: [::core::ffi::c_char; 12] =
        ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"\0\0\0\0\0\0\0\0\0\0\0\0");
    let mut pool_mode_str = ::core::ptr::null::<::core::ffi::c_char>();
    if buf.is_null() {
        admin_error(admin, c"no mem".as_ptr());
        return true;
    }
    cv.extra = &raw const pool_mode_map as *const CfLookup as *const ::core::ffi::c_void;
    pktbuf_write_RowDescription(
        buf,
        c"ssssiiii".as_ptr(),
        c"name".as_ptr(),
        c"pool_size".as_ptr(),
        c"reserve_pool_size".as_ptr(),
        c"pool_mode".as_ptr(),
        c"max_user_connections".as_ptr(),
        c"current_connections".as_ptr(),
        c"max_user_client_connections".as_ptr(),
        c"current_client_connections".as_ptr(),
    );
    item = user_list.head.next;
    while item != &raw mut user_list.head {
        let mut user = (item as *mut ::core::ffi::c_char)
            .offset(-(2336 as ::core::ffi::c_ulong as isize))
            as *mut PgGlobalUser;
        if (*user).pool_size >= 0 as ::core::ffi::c_int {
            snprintf(
                &raw mut pool_size_str as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 12]>() as size_t,
                c"%9d".as_ptr(),
                (*user).pool_size,
            );
        }
        if (*user).res_pool_size >= 0 as ::core::ffi::c_int {
            snprintf(
                &raw mut res_pool_size_str as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 12]>() as size_t,
                c"%9d".as_ptr(),
                (*user).res_pool_size,
            );
        }
        pool_mode_str = ::core::ptr::null::<::core::ffi::c_char>();
        cv.value_p = &raw mut (*user).pool_mode as *mut ::core::ffi::c_void;
        if (*user).pool_mode != POOL_INHERIT {
            pool_mode_str = cf_get_lookup(&raw mut cv);
        }
        pktbuf_write_DataRow(
            buf,
            c"ssssiiii".as_ptr(),
            &raw mut (*user).credentials.name as *mut ::core::ffi::c_char,
            &raw mut pool_size_str as *mut ::core::ffi::c_char,
            &raw mut res_pool_size_str as *mut ::core::ffi::c_char,
            pool_mode_str,
            user_max_connections(user),
            (*user).connection_count,
            user_client_max_connections(user),
            (*user).client_connection_count,
        );
        item = (*item).next;
    }
    admin_flush(admin, buf, c"SHOW".as_ptr());
    true
}

pub const SKF_STD: [::core::ffi::c_char; 22] = unsafe {
    ::core::mem::transmute::<[u8; 22], [::core::ffi::c_char; 22]>(*b"ssssssisiTTiiississii\0")
};

pub const SKF_DBG: [::core::ffi::c_char; 29] = unsafe {
    ::core::mem::transmute::<[u8; 29], [::core::ffi::c_char; 29]>(
        *b"ssssssisiTTiiississiiiiiiiii\0",
    )
};

unsafe extern "C" fn socket_header(mut buf: *mut PktBuf, mut debug: bool) {
    pktbuf_write_RowDescription(
        buf,
        if debug {
            SKF_DBG.as_ptr()
        } else {
            SKF_STD.as_ptr()
        },
        c"type".as_ptr(),
        c"user".as_ptr(),
        c"database".as_ptr(),
        c"replication".as_ptr(),
        c"state".as_ptr(),
        c"addr".as_ptr(),
        c"port".as_ptr(),
        c"local_addr".as_ptr(),
        c"local_port".as_ptr(),
        c"connect_time".as_ptr(),
        c"request_time".as_ptr(),
        c"wait".as_ptr(),
        c"wait_us".as_ptr(),
        c"close_needed".as_ptr(),
        c"ptr".as_ptr(),
        c"link".as_ptr(),
        c"remote_pid".as_ptr(),
        c"tls".as_ptr(),
        c"application_name".as_ptr(),
        c"prepared_statements".as_ptr(),
        c"id".as_ptr(),
        c"recv_pos".as_ptr(),
        c"pkt_pos".as_ptr(),
        c"pkt_remain".as_ptr(),
        c"send_pos".as_ptr(),
        c"send_remain".as_ptr(),
        c"pkt_avail".as_ptr(),
        c"send_avail".as_ptr(),
    );
}

unsafe extern "C" fn adr2txt(
    mut adr: *const PgAddr,
    mut dst: *mut ::core::ffi::c_char,
    mut dstlen: ::core::ffi::c_uint,
) {
    pga_ntop(adr, dst, dstlen as ::core::ffi::c_int);
}

unsafe extern "C" fn socket_row(
    mut buf: *mut PktBuf,
    mut sk: *mut PgSocket,
    mut state: *const ::core::ffi::c_char,
    mut debug: bool,
) {
    let mut pkt_avail = 0 as ::core::ffi::c_int;
    let mut send_avail = 0 as ::core::ffi::c_int;
    let mut remote_pid: ::core::ffi::c_int = 0;
    let mut prepared_statement_count = 0 as ::core::ffi::c_int;
    let mut ptrbuf: [::core::ffi::c_char; 128] = [0; 128];
    let mut linkbuf: [::core::ffi::c_char; 128] = [0; 128];
    let mut l_addr: [::core::ffi::c_char; 56] = [0; 56];
    let mut r_addr: [::core::ffi::c_char; 56] = [0; 56];
    let mut io = (*sk).sbuf.io;
    let mut infobuf: [::core::ffi::c_char; 96] = ::core::mem::transmute::<
        [u8; 96],
        [::core::ffi::c_char; 96],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut v: *mut VarCache = &raw mut (*sk).vars;
    let mut application_name: *const PStr = *(*v)
        .var_list
        .offset(VAppName as ::core::ffi::c_int as isize);
    let mut now = get_cached_time();
    let mut wait_time: usec_t = if (*sk).query_start != 0 {
        now.wrapping_sub((*sk).query_start)
    } else {
        0
    };
    let mut replication = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !io.is_null() {
        pkt_avail = iobuf_amount_parse((*sk).sbuf.io) as ::core::ffi::c_int;
        send_avail = iobuf_amount_pending((*sk).sbuf.io) as ::core::ffi::c_int;
    }
    adr2txt(
        &raw mut (*sk).remote_addr,
        &raw mut r_addr as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 56]>() as ::core::ffi::c_uint,
    );
    adr2txt(
        &raw mut (*sk).local_addr,
        &raw mut l_addr as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 56]>() as ::core::ffi::c_uint,
    );
    snprintf(
        &raw mut ptrbuf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
        c"%p".as_ptr(),
        sk,
    );
    if !(*sk).link.is_null() {
        snprintf(
            &raw mut linkbuf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
            c"%p".as_ptr(),
            (*sk).link,
        );
    } else {
        linkbuf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    }
    if pga_is_unix(&raw mut (*sk).remote_addr) {
        remote_pid = (*sk).remote_addr.scred.pid as ::core::ffi::c_int;
    } else {
        remote_pid = 0 as ::core::ffi::c_int;
    }
    if (*sk).state() as ::core::ffi::c_int >= SV_FREE as ::core::ffi::c_int
        && remote_pid == 0 as ::core::ffi::c_int
    {
        remote_pid =
            usual_be32dec(&raw mut (*sk).cancel_key as *mut uint8_t as *const ::core::ffi::c_void)
                as ::core::ffi::c_int;
    }
    if !(*sk).sbuf.tls.is_null() {
        tls_get_connection_info(
            (*sk).sbuf.tls,
            &raw mut infobuf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 96]>() as size_t,
        );
    }
    if (*sk).state() as ::core::ffi::c_int >= SV_FREE as ::core::ffi::c_int {
        prepared_statement_count = (if !(*sk).server_prepared_statements.is_null() {
            (*(*(*sk).server_prepared_statements).hh.tbl).num_items
        } else {
            0 as ::core::ffi::c_uint
        }) as ::core::ffi::c_int;
    } else {
        prepared_statement_count = (if !(*sk).client_prepared_statements.is_null() {
            (*(*(*sk).client_prepared_statements).hh.tbl).num_items
        } else {
            0 as ::core::ffi::c_uint
        }) as ::core::ffi::c_int;
    }
    if (*sk).replication as ::core::ffi::c_uint
        == REPLICATION_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        replication = c"none".as_ptr() as *mut ::core::ffi::c_char;
    } else if (*sk).replication as ::core::ffi::c_uint
        == REPLICATION_LOGICAL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        replication = c"logical".as_ptr() as *mut ::core::ffi::c_char;
    } else {
        replication = c"physical".as_ptr() as *mut ::core::ffi::c_char;
    }
    pktbuf_write_DataRow(
        buf,
        if debug {
            SKF_DBG.as_ptr()
        } else {
            SKF_STD.as_ptr()
        },
        if (*sk).state() as ::core::ffi::c_int >= SV_FREE as ::core::ffi::c_int {
            c"S".as_ptr()
        } else {
            c"C".as_ptr()
        },
        if !(*sk).login_user_credentials.is_null() {
            &raw mut (*(*sk).login_user_credentials).name as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_char
        } else {
            c"(nouser)".as_ptr()
        },
        if !(*sk).pool.is_null() && (*(*(*sk).pool).db).peer_id == 0 {
            &raw mut (*(*(*sk).pool).db).name as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_char
        } else {
            c"(nodb)".as_ptr()
        },
        replication,
        if (*sk).link.is_null() && strcmp(state, c"active".as_ptr()) == 0 as ::core::ffi::c_int {
            c"idle".as_ptr()
        } else {
            state
        },
        &raw mut r_addr as *mut ::core::ffi::c_char,
        pga_port(&raw mut (*sk).remote_addr),
        &raw mut l_addr as *mut ::core::ffi::c_char,
        pga_port(&raw mut (*sk).local_addr),
        (*sk).connect_time,
        (*sk).request_time,
        wait_time.wrapping_div(USEC) as ::core::ffi::c_int,
        wait_time.wrapping_rem(USEC) as ::core::ffi::c_int,
        (*sk).close_needed() as ::core::ffi::c_int,
        &raw mut ptrbuf as *mut ::core::ffi::c_char,
        &raw mut linkbuf as *mut ::core::ffi::c_char,
        remote_pid,
        &raw mut infobuf as *mut ::core::ffi::c_char,
        if !application_name.is_null() {
            &raw const (*application_name).str_0 as *const ::core::ffi::c_char
        } else {
            c"".as_ptr()
        },
        prepared_statement_count,
        (*sk).id,
        if !io.is_null() {
            (*io).recv_pos
        } else {
            0 as ::core::ffi::c_uint
        },
        if !io.is_null() {
            (*io).parse_pos
        } else {
            0 as ::core::ffi::c_uint
        },
        (*sk).sbuf.pkt_remain,
        if !io.is_null() {
            (*io).done_pos
        } else {
            0 as ::core::ffi::c_uint
        },
        0 as ::core::ffi::c_int,
        pkt_avail,
        send_avail,
    );
}

unsafe extern "C" fn show_socket_list(
    mut buf: *mut PktBuf,
    mut list: *mut StatList,
    mut state: *const ::core::ffi::c_char,
    mut debug: bool,
) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut sk = ::core::ptr::null_mut::<PgSocket>();
    item = (*list).head.next;
    while item != &raw mut (*list).head {
        sk = (item as *mut ::core::ffi::c_char)
            as *mut PgSocket;
        socket_row(buf, sk, state, debug);
        item = (*item).next;
    }
}

unsafe extern "C" fn admin_show_clients(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut buf = pktbuf_dynamic(256 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(admin, c"no mem".as_ptr());
        return true;
    }
    socket_header(buf, false);
    item = pool_list.head.next;
    while item != &raw mut pool_list.head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        show_socket_list(
            buf,
            &raw mut (*pool).active_client_list,
            c"active".as_ptr(),
            false,
        );
        show_socket_list(
            buf,
            &raw mut (*pool).waiting_client_list,
            c"waiting".as_ptr(),
            false,
        );
        show_socket_list(
            buf,
            &raw mut (*pool).active_cancel_req_list,
            c"active_cancel_req".as_ptr(),
            false,
        );
        show_socket_list(
            buf,
            &raw mut (*pool).waiting_cancel_req_list,
            c"waiting_cancel_req".as_ptr(),
            false,
        );
        item = (*item).next;
    }
    item = peer_pool_list.head.next;
    while item != &raw mut peer_pool_list.head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        show_socket_list(
            buf,
            &raw mut (*pool).active_cancel_req_list,
            c"active_cancel_req".as_ptr(),
            false,
        );
        show_socket_list(
            buf,
            &raw mut (*pool).waiting_cancel_req_list,
            c"waiting_cancel_req".as_ptr(),
            false,
        );
        item = (*item).next;
    }
    admin_flush(admin, buf, c"SHOW".as_ptr());
    true
}

unsafe extern "C" fn admin_show_servers(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    buf = pktbuf_dynamic(256 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(admin, c"no mem".as_ptr());
        return true;
    }
    socket_header(buf, false);
    item = pool_list.head.next;
    while item != &raw mut pool_list.head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        show_socket_list(
            buf,
            &raw mut (*pool).active_server_list,
            c"active".as_ptr(),
            false,
        );
        show_socket_list(
            buf,
            &raw mut (*pool).idle_server_list,
            c"idle".as_ptr(),
            false,
        );
        show_socket_list(
            buf,
            &raw mut (*pool).used_server_list,
            c"used".as_ptr(),
            false,
        );
        show_socket_list(
            buf,
            &raw mut (*pool).tested_server_list,
            c"tested".as_ptr(),
            false,
        );
        show_socket_list(
            buf,
            &raw mut (*pool).new_server_list,
            c"new".as_ptr(),
            false,
        );
        show_socket_list(
            buf,
            &raw mut (*pool).active_cancel_server_list,
            c"active_cancel".as_ptr(),
            false,
        );
        show_socket_list(
            buf,
            &raw mut (*pool).being_canceled_server_list,
            c"being_canceled".as_ptr(),
            false,
        );
        item = (*item).next;
    }
    item = peer_pool_list.head.next;
    while item != &raw mut peer_pool_list.head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        show_socket_list(
            buf,
            &raw mut (*pool).new_server_list,
            c"new".as_ptr(),
            false,
        );
        show_socket_list(
            buf,
            &raw mut (*pool).active_cancel_server_list,
            c"active_cancel".as_ptr(),
            false,
        );
        item = (*item).next;
    }
    admin_flush(admin, buf, c"SHOW".as_ptr());
    true
}

unsafe extern "C" fn admin_show_sockets(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    buf = pktbuf_dynamic(256 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(admin, c"no mem".as_ptr());
        return true;
    }
    socket_header(buf, true);
    item = pool_list.head.next;
    while item != &raw mut pool_list.head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        show_socket_list(
            buf,
            &raw mut (*pool).active_client_list,
            c"cl_active".as_ptr(),
            true,
        );
        show_socket_list(
            buf,
            &raw mut (*pool).waiting_client_list,
            c"cl_waiting".as_ptr(),
            true,
        );
        show_socket_list(
            buf,
            &raw mut (*pool).active_server_list,
            c"sv_active".as_ptr(),
            true,
        );
        show_socket_list(
            buf,
            &raw mut (*pool).idle_server_list,
            c"sv_idle".as_ptr(),
            true,
        );
        show_socket_list(
            buf,
            &raw mut (*pool).used_server_list,
            c"sv_used".as_ptr(),
            true,
        );
        show_socket_list(
            buf,
            &raw mut (*pool).tested_server_list,
            c"sv_tested".as_ptr(),
            true,
        );
        show_socket_list(
            buf,
            &raw mut (*pool).new_server_list,
            c"sv_login".as_ptr(),
            true,
        );
        item = (*item).next;
    }
    show_socket_list(buf, &raw mut login_client_list, c"cl_login".as_ptr(), true);
    admin_flush(admin, buf, c"SHOW".as_ptr());
    true
}

unsafe extern "C" fn show_active_socket_list(
    mut buf: *mut PktBuf,
    mut list: *mut StatList,
    mut state: *const ::core::ffi::c_char,
) {
    let mut item = ::core::ptr::null_mut::<List>();
    item = (*list).head.next;
    while item != &raw mut (*list).head {
        let mut sk = (item as *mut ::core::ffi::c_char)
             as *mut PgSocket;
        if !sbuf_is_empty(&raw mut (*sk).sbuf) {
            socket_row(buf, sk, state, true);
        }
        item = (*item).next;
    }
}

unsafe extern "C" fn admin_show_active_sockets(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    buf = pktbuf_dynamic(256 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(admin, c"no mem".as_ptr());
        return true;
    }
    socket_header(buf, true);
    item = pool_list.head.next;
    while item != &raw mut pool_list.head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        show_active_socket_list(
            buf,
            &raw mut (*pool).active_client_list,
            c"cl_active".as_ptr(),
        );
        show_active_socket_list(
            buf,
            &raw mut (*pool).waiting_client_list,
            c"cl_waiting".as_ptr(),
        );
        show_active_socket_list(
            buf,
            &raw mut (*pool).active_server_list,
            c"sv_active".as_ptr(),
        );
        show_active_socket_list(buf, &raw mut (*pool).idle_server_list, c"sv_idle".as_ptr());
        show_active_socket_list(buf, &raw mut (*pool).used_server_list, c"sv_used".as_ptr());
        show_active_socket_list(
            buf,
            &raw mut (*pool).tested_server_list,
            c"sv_tested".as_ptr(),
        );
        show_active_socket_list(buf, &raw mut (*pool).new_server_list, c"sv_login".as_ptr());
        item = (*item).next;
    }
    show_active_socket_list(buf, &raw mut login_client_list, c"cl_login".as_ptr());
    admin_flush(admin, buf, c"SHOW".as_ptr());
    true
}

unsafe extern "C" fn admin_show_pools(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    let mut waiter = ::core::ptr::null_mut::<PgSocket>();
    let mut now = get_cached_time();
    let mut max_wait: usec_t = 0;
    let mut cv = CfValue {
        value_p: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        extra: ::core::ptr::null::<::core::ffi::c_void>(),
        key_name: ::core::ptr::null::<::core::ffi::c_char>(),
        buf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        buflen: 0,
    };
    let mut load_balance_hosts_lookup = CfValue {
        value_p: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        extra: ::core::ptr::null::<::core::ffi::c_void>(),
        key_name: ::core::ptr::null::<::core::ffi::c_char>(),
        buf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        buflen: 0,
    };
    let mut pool_mode: ::core::ffi::c_int = 0;
    let mut load_balance_hosts_str = ::core::ptr::null::<::core::ffi::c_char>();
    cv.extra = &raw const pool_mode_map as *const CfLookup as *const ::core::ffi::c_void;
    cv.value_p = &raw mut pool_mode as *mut ::core::ffi::c_void;
    load_balance_hosts_lookup.extra =
        &raw const load_balance_hosts_map as *const CfLookup as *const ::core::ffi::c_void;
    buf = pktbuf_dynamic(256 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(admin, c"no mem".as_ptr());
        return true;
    }
    pktbuf_write_RowDescription(
        buf,
        c"ssiiiiiiiiiiiiiss".as_ptr(),
        c"database".as_ptr(),
        c"user".as_ptr(),
        c"cl_active".as_ptr(),
        c"cl_waiting".as_ptr(),
        c"cl_active_cancel_req".as_ptr(),
        c"cl_waiting_cancel_req".as_ptr(),
        c"sv_active".as_ptr(),
        c"sv_active_cancel".as_ptr(),
        c"sv_being_canceled".as_ptr(),
        c"sv_idle".as_ptr(),
        c"sv_used".as_ptr(),
        c"sv_tested".as_ptr(),
        c"sv_login".as_ptr(),
        c"maxwait".as_ptr(),
        c"maxwait_us".as_ptr(),
        c"pool_mode".as_ptr(),
        c"load_balance_hosts".as_ptr(),
    );
    item = pool_list.head.next;
    while item != &raw mut pool_list.head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        waiter = first_socket(&raw mut (*pool).waiting_client_list);
        max_wait = if !waiter.is_null() && (*waiter).query_start != 0 {
            now.wrapping_sub((*waiter).query_start)
        } else {
            0
        };
        pool_mode = probably_wrong_pool_pool_mode(pool);
        load_balance_hosts_str = ::core::ptr::null::<::core::ffi::c_char>();
        load_balance_hosts_lookup.value_p =
            &raw mut (*(*pool).db).load_balance_hosts as *mut ::core::ffi::c_void;
        if !(*(*pool).db).host.is_null() && !strchr((*(*pool).db).host, ',' as i32).is_null() {
            load_balance_hosts_str = cf_get_lookup(&raw mut load_balance_hosts_lookup);
        }
        pktbuf_write_DataRow(
            buf,
            c"ssiiiiiiiiiiiiiss".as_ptr(),
            &raw mut (*(*pool).db).name as *mut ::core::ffi::c_char,
            &raw mut (*(*pool).user_credentials).name as *mut ::core::ffi::c_char,
            statlist_count(&raw mut (*pool).active_client_list),
            statlist_count(&raw mut (*pool).waiting_client_list),
            statlist_count(&raw mut (*pool).active_cancel_req_list),
            statlist_count(&raw mut (*pool).waiting_cancel_req_list),
            statlist_count(&raw mut (*pool).active_server_list),
            statlist_count(&raw mut (*pool).active_cancel_server_list),
            statlist_count(&raw mut (*pool).being_canceled_server_list),
            statlist_count(&raw mut (*pool).idle_server_list),
            statlist_count(&raw mut (*pool).used_server_list),
            statlist_count(&raw mut (*pool).tested_server_list),
            statlist_count(&raw mut (*pool).new_server_list),
            max_wait.wrapping_div(USEC) as ::core::ffi::c_int,
            max_wait.wrapping_rem(USEC) as ::core::ffi::c_int,
            cf_get_lookup(&raw mut cv),
            load_balance_hosts_str,
        );
        item = (*item).next;
    }
    admin_flush(admin, buf, c"SHOW".as_ptr());
    true
}

unsafe extern "C" fn admin_show_peer_pools(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    buf = pktbuf_dynamic(256 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(admin, c"no mem".as_ptr());
        return true;
    }
    pktbuf_write_RowDescription(
        buf,
        c"iiiii".as_ptr(),
        c"peer_id".as_ptr(),
        c"cl_active_cancel_req".as_ptr(),
        c"cl_waiting_cancel_req".as_ptr(),
        c"sv_active_cancel".as_ptr(),
        c"sv_login".as_ptr(),
    );
    item = peer_pool_list.head.next;
    while item != &raw mut peer_pool_list.head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        pktbuf_write_DataRow(
            buf,
            c"iiiii".as_ptr(),
            (*(*pool).db).peer_id,
            statlist_count(&raw mut (*pool).active_cancel_req_list),
            statlist_count(&raw mut (*pool).waiting_cancel_req_list),
            statlist_count(&raw mut (*pool).active_cancel_server_list),
            statlist_count(&raw mut (*pool).new_server_list),
        );
        item = (*item).next;
    }
    admin_flush(admin, buf, c"SHOW".as_ptr());
    true
}

unsafe extern "C" fn slab_stat_cb(
    mut arg: *mut ::core::ffi::c_void,
    mut slab_name: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_uint,
    mut free: ::core::ffi::c_uint,
    mut total: ::core::ffi::c_uint,
) {
    let mut buf = arg as *mut PktBuf;
    let mut alloc = total.wrapping_mul(size);
    pktbuf_write_DataRow(
        buf,
        c"siiii".as_ptr(),
        slab_name,
        size,
        total.wrapping_sub(free),
        free,
        alloc,
    );
}

unsafe extern "C" fn admin_show_mem(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    buf = pktbuf_dynamic(256 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(admin, c"no mem".as_ptr());
        return true;
    }
    pktbuf_write_RowDescription(
        buf,
        c"siiii".as_ptr(),
        c"name".as_ptr(),
        c"size".as_ptr(),
        c"used".as_ptr(),
        c"free".as_ptr(),
        c"memtotal".as_ptr(),
    );
    slab_stats(
        Some(
            slab_stat_cb
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_uint,
                    ::core::ffi::c_uint,
                    ::core::ffi::c_uint,
                ) -> (),
        ),
        buf as *mut ::core::ffi::c_void,
    );
    admin_flush(admin, buf, c"SHOW".as_ptr());
    true
}

unsafe extern "C" fn admin_show_state(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    buf = pktbuf_dynamic(64 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(admin, c"no mem".as_ptr());
        return true;
    }
    pktbuf_write_RowDescription(buf, c"ss".as_ptr(), c"key".as_ptr(), c"value".as_ptr());
    pktbuf_write_DataRow(
        buf,
        c"ss".as_ptr(),
        c"active".as_ptr(),
        if cf_pause_mode == P_NONE as ::core::ffi::c_int {
            c"yes".as_ptr()
        } else {
            c"no".as_ptr()
        },
    );
    pktbuf_write_DataRow(
        buf,
        c"ss".as_ptr(),
        c"paused".as_ptr(),
        if cf_pause_mode == P_PAUSE as ::core::ffi::c_int {
            c"yes".as_ptr()
        } else {
            c"no".as_ptr()
        },
    );
    pktbuf_write_DataRow(
        buf,
        c"ss".as_ptr(),
        c"suspended".as_ptr(),
        if cf_pause_mode == P_SUSPEND as ::core::ffi::c_int {
            c"yes".as_ptr()
        } else {
            c"no".as_ptr()
        },
    );
    admin_flush(admin, buf, c"SHOW".as_ptr());
    true
}

unsafe extern "C" fn dns_name_cb(
    mut arg: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut ai: *const addrinfo,
    mut ttl: usec_t,
) {
    let mut buf = arg as *mut PktBuf;
    let mut s = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut end = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut adrs: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut now = get_cached_time();
    end = (&raw mut adrs as *mut ::core::ffi::c_char)
        .add(::core::mem::size_of::<[::core::ffi::c_char; 1024]>())
        .offset(-(2 as ::core::ffi::c_int as isize));
    s = &raw mut adrs as *mut ::core::ffi::c_char;
    while !ai.is_null() && s < end {
        if s != &raw mut adrs as *mut ::core::ffi::c_char {
            let fresh5 = s;
            s = s.offset(1);
            *fresh5 = ',' as i32 as ::core::ffi::c_char;
        }
        sa2str(
            (*ai).ai_addr,
            s,
            end.offset_from(s) as ::core::ffi::c_long as size_t,
        );
        s = s.add(strlen(s));
        ai = (*ai).ai_next;
    }
    *s = 0 as ::core::ffi::c_char;
    pktbuf_write_DataRow(
        buf,
        c"sqs".as_ptr(),
        name,
        if ttl < now {
            0
        } else {
            ttl.wrapping_sub(now).wrapping_div(USEC)
        },
        &raw mut adrs as *mut ::core::ffi::c_char,
    );
}

unsafe extern "C" fn admin_show_dns_hosts(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    buf = pktbuf_dynamic(256 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(admin, c"no mem".as_ptr());
        return true;
    }
    pktbuf_write_RowDescription(
        buf,
        c"sqs".as_ptr(),
        c"hostname".as_ptr(),
        c"ttl".as_ptr(),
        c"addrs".as_ptr(),
    );
    adns_walk_names(
        adns,
        Some(
            dns_name_cb
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    *const addrinfo,
                    usec_t,
                ) -> (),
        ),
        buf as *mut ::core::ffi::c_void,
    );
    admin_flush(admin, buf, c"SHOW".as_ptr());
    true
}

unsafe extern "C" fn dns_zone_cb(
    mut arg: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut serial: uint32_t,
    mut nhosts: ::core::ffi::c_int,
) {
    let mut buf = arg as *mut PktBuf;
    pktbuf_write_DataRow(buf, c"sqi".as_ptr(), name, serial as uint64_t, nhosts);
}

unsafe extern "C" fn admin_show_dns_zones(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    buf = pktbuf_dynamic(256 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(admin, c"no mem".as_ptr());
        return true;
    }
    pktbuf_write_RowDescription(
        buf,
        c"sqi".as_ptr(),
        c"zonename".as_ptr(),
        c"serial".as_ptr(),
        c"count".as_ptr(),
    );
    adns_walk_zones(
        adns,
        Some(
            dns_zone_cb
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    uint32_t,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        buf as *mut ::core::ffi::c_void,
    );
    admin_flush(admin, buf, c"SHOW".as_ptr());
    true
}

unsafe extern "C" fn show_one_param(
    mut arg: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut val: *const ::core::ffi::c_char,
    mut defval: *const ::core::ffi::c_char,
    mut reloadable: bool,
) {
    let mut buf = arg as *mut PktBuf;
    pktbuf_write_DataRow(
        buf,
        c"ssss".as_ptr(),
        name,
        val,
        defval,
        if reloadable {
            c"yes".as_ptr()
        } else {
            c"no".as_ptr()
        },
    );
}

unsafe extern "C" fn admin_show_config(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    buf = pktbuf_dynamic(256 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(admin, c"no mem".as_ptr());
        return true;
    }
    pktbuf_write_RowDescription(
        buf,
        c"ssss".as_ptr(),
        c"key".as_ptr(),
        c"value".as_ptr(),
        c"default".as_ptr(),
        c"changeable".as_ptr(),
    );
    config_for_each(
        Some(
            show_one_param
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    bool,
                ) -> (),
        ),
        buf as *mut ::core::ffi::c_void,
    );
    admin_flush(admin, buf, c"SHOW".as_ptr());
    true
}

unsafe extern "C" fn admin_cmd_reload(
    mut admin: *mut PgSocket,
    mut arg: *const ::core::ffi::c_char,
) -> bool {
    let mut ok = true;
    if !arg.is_null() && *arg as ::core::ffi::c_int != 0 {
        return syntax_error(admin);
    }
    if !(*admin).admin_user() {
        return admin_error(admin, c"admin access needed".as_ptr());
    }
    let mut _log_ctx = NULL;
    log_generic(LG_INFO, _log_ctx, c"RELOAD command issued".as_ptr());
    if !load_config() {
        ok = false;
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx_0,
            c"RELOAD Failed, see logs for more details".as_ptr(),
        );
    }
    if !sbuf_tls_setup() {
        ok = false;
        let mut _log_ctx_1 = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx_1,
            c"TLS configuration could not be reloaded, keeping old configuration".as_ptr(),
        );
    }
    if ok {
        admin_ready(admin, c"RELOAD".as_ptr())
    } else {
        send_pooler_error(
            admin,
            true,
            c"F0000".as_ptr(),
            false,
            c"RELOAD failed, see logs for additional details".as_ptr(),
        )
    }
}

unsafe extern "C" fn admin_cmd_shutdown(
    mut admin: *mut PgSocket,
    mut arg: *const ::core::ffi::c_char,
) -> bool {
    let mut mode = SHUTDOWN_IMMEDIATE;
    if !arg.is_null() && *arg as ::core::ffi::c_int != 0 {
        if strcasecmp(arg, c"WAIT_FOR_CLIENTS".as_ptr()) == 0 as ::core::ffi::c_int {
            mode = SHUTDOWN_WAIT_FOR_CLIENTS;
        } else if strcasecmp(arg, c"WAIT_FOR_SERVERS".as_ptr()) == 0 as ::core::ffi::c_int {
            mode = SHUTDOWN_WAIT_FOR_SERVERS;
        } else {
            return syntax_error(admin);
        }
    }
    if !(*admin).admin_user() {
        return admin_error(admin, c"admin access needed".as_ptr());
    }
    cf_shutdown = mode as ::core::ffi::c_int;
    if mode as ::core::ffi::c_uint
        == SHUTDOWN_IMMEDIATE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut _log_ctx = NULL;
        log_generic(LG_INFO, _log_ctx, c"SHUTDOWN command issued".as_ptr());
        event_base_loopbreak(pgb_event_base);
        true
    } else {
        if mode as ::core::ffi::c_uint
            == SHUTDOWN_WAIT_FOR_SERVERS as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            cf_pause_mode = P_PAUSE as ::core::ffi::c_int;
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_INFO,
                _log_ctx_0,
                c"SHUTDOWN WAIT_FOR_SERVERS command issued".as_ptr(),
            );
        } else {
            let mut _log_ctx_1 = NULL;
            log_generic(
                LG_INFO,
                _log_ctx_1,
                c"SHUTDOWN WAIT_FOR_CLIENTS command issued".as_ptr(),
            );
        }
        cleanup_tcp_sockets();
        admin_ready(admin, c"SHUTDOWN".as_ptr())
    }
}

unsafe extern "C" fn full_resume() {
    let mut tmp_mode = cf_pause_mode;
    cf_pause_mode = P_NONE as ::core::ffi::c_int;
    if tmp_mode == P_SUSPEND as ::core::ffi::c_int {
        resume_all();
    }
}

unsafe extern "C" fn admin_cmd_resume(
    mut admin: *mut PgSocket,
    mut arg: *const ::core::ffi::c_char,
) -> bool {
    if !(*admin).admin_user() {
        return admin_error(admin, c"admin access needed".as_ptr());
    }
    if *arg == 0 {
        let mut _log_ctx = NULL;
        log_generic(LG_INFO, _log_ctx, c"RESUME command issued".as_ptr());
        if cf_shutdown != 0 {
            return admin_error(admin, c"pooler is shutting down".as_ptr());
        } else if cf_pause_mode != P_NONE as ::core::ffi::c_int {
            full_resume();
        } else {
            return admin_error(admin, c"pooler is not paused/suspended".as_ptr());
        }
    } else {
        let mut db = find_database(arg);
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_INFO,
            _log_ctx_0,
            c"RESUME '%s' command issued".as_ptr(),
            arg,
        );
        if db.is_null() {
            return admin_error(admin, c"no such database: %s".as_ptr(), arg);
        }
        if !(*db).db_paused {
            return admin_error(admin, c"database %s is not paused".as_ptr(), arg);
        }
        (*db).db_paused = false;
    }
    admin_ready(admin, c"RESUME".as_ptr())
}

unsafe extern "C" fn admin_cmd_suspend(
    mut admin: *mut PgSocket,
    mut arg: *const ::core::ffi::c_char,
) -> bool {
    if !arg.is_null() && *arg as ::core::ffi::c_int != 0 {
        return syntax_error(admin);
    }
    if !(*admin).admin_user() {
        return admin_error(admin, c"admin access needed".as_ptr());
    }
    if cf_pause_mode != 0 {
        return admin_error(admin, c"already suspended/paused".as_ptr());
    }
    if count_paused_databases() > 0 as ::core::ffi::c_int {
        return admin_error(admin, c"cannot suspend with paused databases".as_ptr());
    }
    let mut _log_ctx = NULL;
    log_generic(LG_INFO, _log_ctx, c"SUSPEND command issued".as_ptr());
    cf_pause_mode = P_SUSPEND as ::core::ffi::c_int;
    (*admin).set_wait_for_response(true);
    suspend_pooler();
    g_suspend_start = get_cached_time();
    true
}

unsafe extern "C" fn admin_cmd_pause(
    mut admin: *mut PgSocket,
    mut arg: *const ::core::ffi::c_char,
) -> bool {
    if !(*admin).admin_user() {
        return admin_error(admin, c"admin access needed".as_ptr());
    }
    if cf_pause_mode != 0 {
        return admin_error(admin, c"already suspended/paused".as_ptr());
    }
    if *arg == 0 {
        let mut _log_ctx = NULL;
        log_generic(LG_INFO, _log_ctx, c"PAUSE command issued".as_ptr());
        cf_pause_mode = P_PAUSE as ::core::ffi::c_int;
        (*admin).set_wait_for_response(true);
    } else {
        let mut db = ::core::ptr::null_mut::<PgDatabase>();
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_INFO,
            _log_ctx_0,
            c"PAUSE '%s' command issued".as_ptr(),
            arg,
        );
        db = find_or_register_database(admin, arg);
        if db.is_null() {
            return admin_error(admin, c"no such database: %s".as_ptr(), arg);
        }
        if db == (*(*admin).pool).db {
            return admin_error(admin, c"cannot pause admin db: %s".as_ptr(), arg);
        }
        (*db).db_paused = true;
        if count_db_active(db) > 0 as ::core::ffi::c_int {
            (*admin).set_wait_for_response(true);
        } else {
            return admin_ready(admin, c"PAUSE".as_ptr());
        }
    }
    true
}

unsafe extern "C" fn admin_cmd_reconnect(
    mut admin: *mut PgSocket,
    mut arg: *const ::core::ffi::c_char,
) -> bool {
    if !(*admin).admin_user() {
        return admin_error(admin, c"admin access needed".as_ptr());
    }
    if *arg == 0 {
        let mut item = ::core::ptr::null_mut::<List>();
        let mut pool = ::core::ptr::null_mut::<PgPool>();
        let mut _log_ctx = NULL;
        log_generic(LG_INFO, _log_ctx, c"RECONNECT command issued".as_ptr());
        item = pool_list.head.next;
        while item != &raw mut pool_list.head {
            pool = (item as *mut ::core::ffi::c_char)
                as *mut PgPool;
            if !(*(*pool).db).admin {
                tag_database_dirty((*pool).db);
            }
            item = (*item).next;
        }
    } else {
        let mut db = ::core::ptr::null_mut::<PgDatabase>();
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_INFO,
            _log_ctx_0,
            c"RECONNECT '%s' command issued".as_ptr(),
            arg,
        );
        db = find_or_register_database(admin, arg);
        if db.is_null() {
            return admin_error(admin, c"no such database: %s".as_ptr(), arg);
        }
        if db == (*(*admin).pool).db {
            return admin_error(admin, c"cannot reconnect admin db: %s".as_ptr(), arg);
        }
        tag_database_dirty(db);
    }
    admin_ready(admin, c"RECONNECT".as_ptr())
}

unsafe extern "C" fn admin_cmd_disable(
    mut admin: *mut PgSocket,
    mut arg: *const ::core::ffi::c_char,
) -> bool {
    let mut db = ::core::ptr::null_mut::<PgDatabase>();
    if !(*admin).admin_user() {
        return admin_error(admin, c"admin access needed".as_ptr());
    }
    if *arg == 0 {
        return admin_error(admin, c"a database is required".as_ptr());
    }
    let mut _log_ctx = NULL;
    log_generic(
        LG_INFO,
        _log_ctx,
        c"DISABLE '%s' command issued".as_ptr(),
        arg,
    );
    db = find_or_register_database(admin, arg);
    if db.is_null() {
        return admin_error(admin, c"no such database: %s".as_ptr(), arg);
    }
    if (*db).admin {
        return admin_error(admin, c"cannot disable admin db: %s".as_ptr(), arg);
    }
    (*db).db_disabled = true;
    admin_ready(admin, c"DISABLE".as_ptr())
}

unsafe extern "C" fn admin_cmd_enable(
    mut admin: *mut PgSocket,
    mut arg: *const ::core::ffi::c_char,
) -> bool {
    let mut db = ::core::ptr::null_mut::<PgDatabase>();
    if !(*admin).admin_user() {
        return admin_error(admin, c"admin access needed".as_ptr());
    }
    if *arg == 0 {
        return admin_error(admin, c"a database is required".as_ptr());
    }
    let mut _log_ctx = NULL;
    log_generic(
        LG_INFO,
        _log_ctx,
        c"ENABLE '%s' command issued".as_ptr(),
        arg,
    );
    db = find_database(arg);
    if db.is_null() {
        return admin_error(admin, c"no such database: %s".as_ptr(), arg);
    }
    if (*db).admin {
        return admin_error(admin, c"cannot disable admin db: %s".as_ptr(), arg);
    }
    (*db).db_disabled = false;
    admin_ready(admin, c"ENABLE".as_ptr())
}

unsafe extern "C" fn find_socket_in_list(
    mut target_id: ::core::ffi::c_ulonglong,
    mut sockets: *mut StatList,
) -> *mut PgSocket {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut socket = ::core::ptr::null_mut::<PgSocket>();
    item = (*sockets).head.next;
    while item != &raw mut (*sockets).head {
        socket = (item as *mut ::core::ffi::c_char)
            as *mut PgSocket;
        if target_id == (*socket).id {
            return socket;
        }
        item = (*item).next;
    }
    ::core::ptr::null_mut::<PgSocket>()
}

unsafe extern "C" fn find_client_global(mut target_id: ::core::ffi::c_ulonglong) -> *mut PgSocket {
    let mut kill_client = ::core::ptr::null_mut::<PgSocket>();
    let mut item = ::core::ptr::null_mut::<List>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    item = pool_list.head.next;
    while item != &raw mut pool_list.head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        kill_client = find_socket_in_list(target_id, &raw mut (*pool).active_client_list);
        if !kill_client.is_null() {
            return kill_client;
        }
        kill_client = find_socket_in_list(target_id, &raw mut (*pool).waiting_client_list);
        if !kill_client.is_null() {
            return kill_client;
        }
        kill_client = find_socket_in_list(target_id, &raw mut (*pool).active_cancel_req_list);
        if !kill_client.is_null() {
            return kill_client;
        }
        kill_client = find_socket_in_list(target_id, &raw mut (*pool).waiting_cancel_req_list);
        if !kill_client.is_null() {
            return kill_client;
        }
        item = (*item).next;
    }
    item = peer_pool_list.head.next;
    while item != &raw mut peer_pool_list.head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        kill_client = find_socket_in_list(target_id, &raw mut (*pool).active_cancel_req_list);
        if !kill_client.is_null() {
            return kill_client;
        }
        kill_client = find_socket_in_list(target_id, &raw mut (*pool).waiting_cancel_req_list);
        if !kill_client.is_null() {
            return kill_client;
        }
        item = (*item).next;
    }
    ::core::ptr::null_mut::<PgSocket>()
}

unsafe extern "C" fn admin_cmd_kill_client(
    mut admin: *mut PgSocket,
    mut arg: *const ::core::ffi::c_char,
) -> bool {
    let mut kill_client = ::core::ptr::null_mut::<PgSocket>();
    let mut target_id = 0 as ::core::ffi::c_ulonglong;
    if sscanf(arg, c"%llu".as_ptr(), &raw mut target_id) != 1 as ::core::ffi::c_int {
        return admin_error(admin, c"invalid client pointer supplied".as_ptr());
    }
    kill_client = find_client_global(target_id);
    if kill_client.is_null() {
        return admin_error(admin, c"client not found".as_ptr());
    }
    disconnect_client(kill_client, true, c"admin forced disconnect".as_ptr());
    admin_ready(admin, c"KILL_CLIENT".as_ptr())
}

unsafe extern "C" fn admin_cmd_kill(
    mut admin: *mut PgSocket,
    mut arg: *const ::core::ffi::c_char,
) -> bool {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    if !(*admin).admin_user() {
        return admin_error(admin, c"admin access needed".as_ptr());
    }
    if cf_pause_mode != 0 {
        return admin_error(admin, c"already suspended/paused".as_ptr());
    }
    if *arg == 0 {
        let mut _log_ctx = NULL;
        log_generic(LG_INFO, _log_ctx, c"KILL command issued".as_ptr());
        item = pool_list.head.next;
        tmp = (*pool_list.head.next).next;
        while item != &raw mut pool_list.head {
            pool = (item as *mut ::core::ffi::c_char)
                as *mut PgPool;
            if !(*(*pool).db).admin {
                (*(*pool).db).db_paused = true;
                kill_pool(pool);
            }
            item = tmp;
            tmp = (*tmp).next;
        }
    } else {
        let mut db = ::core::ptr::null_mut::<PgDatabase>();
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_INFO,
            _log_ctx_0,
            c"KILL '%s' command issued".as_ptr(),
            arg,
        );
        db = find_or_register_database(admin, arg);
        if db.is_null() {
            return admin_error(admin, c"no such database: %s".as_ptr(), arg);
        }
        if db == (*(*admin).pool).db {
            return admin_error(admin, c"cannot kill admin db: %s".as_ptr(), arg);
        }
        (*db).db_paused = true;
        item = pool_list.head.next;
        tmp = (*pool_list.head.next).next;
        while item != &raw mut pool_list.head {
            pool = (item as *mut ::core::ffi::c_char)
                as *mut PgPool;
            if (*pool).db == db {
                kill_pool(pool);
            }
            item = tmp;
            tmp = (*tmp).next;
        }
    }
    admin_ready(admin, c"KILL".as_ptr())
}

unsafe extern "C" fn admin_cmd_wait_close(
    mut admin: *mut PgSocket,
    mut arg: *const ::core::ffi::c_char,
) -> bool {
    if !(*admin).admin_user() {
        return admin_error(admin, c"admin access needed".as_ptr());
    }
    if *arg == 0 {
        let mut item = ::core::ptr::null_mut::<List>();
        let mut pool = ::core::ptr::null_mut::<PgPool>();
        let mut active = 0 as ::core::ffi::c_int;
        let mut _log_ctx = NULL;
        log_generic(LG_INFO, _log_ctx, c"WAIT_CLOSE command issued".as_ptr());
        item = pool_list.head.next;
        while item != &raw mut pool_list.head {
            let mut db = ::core::ptr::null_mut::<PgDatabase>();
            pool = (item as *mut ::core::ffi::c_char)
                as *mut PgPool;
            db = (*pool).db;
            (*db).db_wait_close = true;
            active += count_db_active(db);
            item = (*item).next;
        }
        if active > 0 as ::core::ffi::c_int {
            (*admin).set_wait_for_response(true);
        } else {
            return admin_ready(admin, c"WAIT_CLOSE".as_ptr());
        }
    } else {
        let mut db_0 = ::core::ptr::null_mut::<PgDatabase>();
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_INFO,
            _log_ctx_0,
            c"WAIT_CLOSE '%s' command issued".as_ptr(),
            arg,
        );
        db_0 = find_or_register_database(admin, arg);
        if db_0.is_null() {
            return admin_error(admin, c"no such database: %s".as_ptr(), arg);
        }
        if db_0 == (*(*admin).pool).db {
            return admin_error(admin, c"cannot wait in admin db: %s".as_ptr(), arg);
        }
        (*db_0).db_wait_close = true;
        if count_db_active(db_0) > 0 as ::core::ffi::c_int {
            (*admin).set_wait_for_response(true);
        } else {
            return admin_ready(admin, c"WAIT_CLOSE".as_ptr());
        }
    }
    true
}

unsafe extern "C" fn copy_arg(
    mut src: *const ::core::ffi::c_char,
    mut glist: *mut regmatch_t,
    mut gnum: ::core::ffi::c_int,
    mut dst: *mut ::core::ffi::c_char,
    mut dstmax: ::core::ffi::c_uint,
    mut qchar: ::core::ffi::c_char,
) -> bool {
    let mut g: *mut regmatch_t = glist.offset(gnum as isize) as *mut regmatch_t;
    let mut len: ::core::ffi::c_uint = 0;
    let mut s = ::core::ptr::null::<::core::ffi::c_char>();
    let mut d = dst;
    let mut i: ::core::ffi::c_uint = 0;
    if (*g).rm_so < 0 as regoff_t || (*g).rm_eo < 0 as regoff_t {
        *dst = 0 as ::core::ffi::c_char;
        return true;
    }
    len = ((*g).rm_eo - (*g).rm_so) as ::core::ffi::c_uint;
    s = src.offset((*g).rm_so as isize);
    if len >= dstmax {
        *dst = 0 as ::core::ffi::c_char;
        return false;
    }
    if *s as ::core::ffi::c_int == qchar as ::core::ffi::c_int {
        i = 1 as ::core::ffi::c_uint;
        while i < len.wrapping_sub(1 as ::core::ffi::c_uint) {
            if *s.offset(i as isize) as ::core::ffi::c_int == qchar as ::core::ffi::c_int
                && *s.offset(i.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                    as ::core::ffi::c_int
                    == qchar as ::core::ffi::c_int
            {
                i = i.wrapping_add(1);
            }
            let fresh4 = d;
            d = d.offset(1);
            *fresh4 = *s.offset(i as isize);
            i = i.wrapping_add(1);
        }
        len = d.offset_from(dst) as ::core::ffi::c_long as ::core::ffi::c_uint;
    } else {
        memcpy(
            dst as *mut ::core::ffi::c_void,
            s as *const ::core::ffi::c_void,
            len as size_t,
        );
    }
    *dst.offset(len as isize) = 0 as ::core::ffi::c_char;
    true
}

unsafe extern "C" fn admin_show_help(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    let mut res: bool = false;
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
        PqMsg_NoticeResponse,
        c"sssss".as_ptr(),
        c"SNOTICE".as_ptr(),
        c"C00000".as_ptr(),
        c"MConsole usage".as_ptr(),
        c"D\n\tSHOW HELP|CONFIG|DATABASES|POOLS|CLIENTS|SERVERS|USERS|VERSION\n\tSHOW PEERS|PEER_POOLS\n\tSHOW FDS|SOCKETS|ACTIVE_SOCKETS|LISTS|MEM|STATE\n\tSHOW DNS_HOSTS|DNS_ZONES\n\tSHOW STATS|STATS_TOTALS|STATS_AVERAGES|TOTALS\n\tSET key = arg\n\tRELOAD\n\tPAUSE [<db>]\n\tRESUME [<db>]\n\tDISABLE <db>\n\tENABLE <db>\n\tRECONNECT [<db>]\n\tKILL [<db>]\n\tKILL_CLIENT <client_id>\n\tSUSPEND\n\tSHUTDOWN\n\tSHUTDOWN WAIT_FOR_SERVERS|WAIT_FOR_CLIENTS\n\tWAIT_CLOSE [<db>]".as_ptr(),
        c"".as_ptr(),
    );
    res = pktbuf_send_immediate(&raw mut _buf, admin);
    if res {
        res = admin_ready(admin, c"SHOW".as_ptr());
    }
    res
}

unsafe extern "C" fn admin_show_version(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    buf = pktbuf_dynamic(128 as ::core::ffi::c_int);
    if buf.is_null() {
        admin_error(admin, c"no mem".as_ptr());
        return true;
    }
    pktbuf_write_RowDescription(buf, c"s".as_ptr(), c"version".as_ptr());
    pktbuf_write_DataRow(buf, c"s".as_ptr(), PACKAGE_STRING.as_ptr());
    admin_flush(admin, buf, c"SHOW".as_ptr());
    true
}

unsafe extern "C" fn admin_show_stats(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    admin_database_stats(admin, &raw mut pool_list)
}

unsafe extern "C" fn admin_show_stats_totals(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    admin_database_stats_totals(admin, &raw mut pool_list)
}

unsafe extern "C" fn admin_show_stats_averages(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    admin_database_stats_averages(admin, &raw mut pool_list)
}

unsafe extern "C" fn admin_show_totals(
    mut admin: *mut PgSocket,
    mut _arg: *const ::core::ffi::c_char,
) -> bool {
    show_stat_totals(admin, &raw mut pool_list)
}

static mut show_map: [cmd_lookup; 23] = unsafe {
    [
        cmd_lookup {
            word: c"clients".as_ptr(),
            func: Some(
                admin_show_clients
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"config".as_ptr(),
            func: Some(
                admin_show_config
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"databases".as_ptr(),
            func: Some(
                admin_show_databases
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"fds".as_ptr(),
            func: Some(
                admin_show_fds
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"help".as_ptr(),
            func: Some(
                admin_show_help
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"lists".as_ptr(),
            func: Some(
                admin_show_lists
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"peers".as_ptr(),
            func: Some(
                admin_show_peers
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"peer_pools".as_ptr(),
            func: Some(
                admin_show_peer_pools
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"pools".as_ptr(),
            func: Some(
                admin_show_pools
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"servers".as_ptr(),
            func: Some(
                admin_show_servers
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"sockets".as_ptr(),
            func: Some(
                admin_show_sockets
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"active_sockets".as_ptr(),
            func: Some(
                admin_show_active_sockets
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"stats".as_ptr(),
            func: Some(
                admin_show_stats
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"stats_totals".as_ptr(),
            func: Some(
                admin_show_stats_totals
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"stats_averages".as_ptr(),
            func: Some(
                admin_show_stats_averages
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"users".as_ptr(),
            func: Some(
                admin_show_users
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"version".as_ptr(),
            func: Some(
                admin_show_version
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"totals".as_ptr(),
            func: Some(
                admin_show_totals
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"mem".as_ptr(),
            func: Some(
                admin_show_mem
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"dns_hosts".as_ptr(),
            func: Some(
                admin_show_dns_hosts
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"dns_zones".as_ptr(),
            func: Some(
                admin_show_dns_zones
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"state".as_ptr(),
            func: Some(
                admin_show_state
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: ::core::ptr::null::<::core::ffi::c_char>(),
            func: None,
        },
    ]
};

unsafe extern "C" fn admin_cmd_show(
    mut admin: *mut PgSocket,
    mut arg: *const ::core::ffi::c_char,
) -> bool {
    if fake_show(admin, arg) {
        return true;
    }
    exec_cmd(
        &raw mut show_map as *mut cmd_lookup,
        admin,
        arg,
        ::core::ptr::null::<::core::ffi::c_char>(),
    )
}

static mut cmd_list: [cmd_lookup; 14] = unsafe {
    [
        cmd_lookup {
            word: c"disable".as_ptr(),
            func: Some(
                admin_cmd_disable
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"enable".as_ptr(),
            func: Some(
                admin_cmd_enable
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"kill".as_ptr(),
            func: Some(
                admin_cmd_kill
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"kill_client".as_ptr(),
            func: Some(
                admin_cmd_kill_client
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"pause".as_ptr(),
            func: Some(
                admin_cmd_pause
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"reconnect".as_ptr(),
            func: Some(
                admin_cmd_reconnect
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"reload".as_ptr(),
            func: Some(
                admin_cmd_reload
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"resume".as_ptr(),
            func: Some(
                admin_cmd_resume
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"select".as_ptr(),
            func: Some(
                admin_cmd_show
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"show".as_ptr(),
            func: Some(
                admin_cmd_show
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"shutdown".as_ptr(),
            func: Some(
                admin_cmd_shutdown
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"suspend".as_ptr(),
            func: Some(
                admin_cmd_suspend
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: c"wait_close".as_ptr(),
            func: Some(
                admin_cmd_wait_close
                    as unsafe extern "C" fn(*mut PgSocket, *const ::core::ffi::c_char) -> bool,
            ),
        },
        cmd_lookup {
            word: ::core::ptr::null::<::core::ffi::c_char>(),
            func: None,
        },
    ]
};

unsafe extern "C" fn admin_parse_query(
    mut admin: *mut PgSocket,
    mut q: *const ::core::ffi::c_char,
) -> bool {
    let mut current_block: u64;
    let mut grp: [regmatch_t; 10] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 10];
    let mut cmd: [::core::ffi::c_char; 16] = [0; 16];
    let mut arg: [::core::ffi::c_char; 64] = [0; 64];
    let mut val: [::core::ffi::c_char; 256] = [0; 256];
    let mut res: bool = false;
    let mut ok: bool = false;
    current_query = q;
    if regexec(
        &raw mut rc_cmd,
        q,
        MAX_GROUPS as size_t,
        &raw mut grp as *mut regmatch_t,
        0 as ::core::ffi::c_int,
    ) == 0 as ::core::ffi::c_int
    {
        ok = copy_arg(
            q,
            &raw mut grp as *mut regmatch_t,
            CMD_NAME,
            &raw mut cmd as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as ::core::ffi::c_uint,
            '"' as i32 as ::core::ffi::c_char,
        );
        if !ok {
            current_block = 7776497614696034663;
        } else {
            ok = copy_arg(
                q,
                &raw mut grp as *mut regmatch_t,
                CMD_ARG,
                &raw mut arg as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as ::core::ffi::c_uint,
                '"' as i32 as ::core::ffi::c_char,
            );
            if !ok {
                current_block = 7776497614696034663;
            } else {
                res = exec_cmd(
                    &raw mut cmd_list as *mut cmd_lookup,
                    admin,
                    &raw mut cmd as *mut ::core::ffi::c_char,
                    &raw mut arg as *mut ::core::ffi::c_char,
                );
                current_block = 15345379368963487090;
            }
        }
    } else if regexec(
        &raw mut rc_set_str,
        q,
        MAX_GROUPS as size_t,
        &raw mut grp as *mut regmatch_t,
        0 as ::core::ffi::c_int,
    ) == 0 as ::core::ffi::c_int
    {
        ok = copy_arg(
            q,
            &raw mut grp as *mut regmatch_t,
            SET_KEY,
            &raw mut arg as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as ::core::ffi::c_uint,
            '"' as i32 as ::core::ffi::c_char,
        );
        if !ok || arg[0 as ::core::ffi::c_int as usize] == 0 {
            current_block = 7776497614696034663;
        } else {
            ok = copy_arg(
                q,
                &raw mut grp as *mut regmatch_t,
                SET_VAL,
                &raw mut val as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_uint,
                '\'' as i32 as ::core::ffi::c_char,
            );
            if !ok {
                current_block = 7776497614696034663;
            } else {
                res = admin_set(
                    admin,
                    &raw mut arg as *mut ::core::ffi::c_char,
                    &raw mut val as *mut ::core::ffi::c_char,
                );
                current_block = 15345379368963487090;
            }
        }
    } else if regexec(
        &raw mut rc_set_word,
        q,
        MAX_GROUPS as size_t,
        &raw mut grp as *mut regmatch_t,
        0 as ::core::ffi::c_int,
    ) == 0 as ::core::ffi::c_int
    {
        ok = copy_arg(
            q,
            &raw mut grp as *mut regmatch_t,
            SET_KEY,
            &raw mut arg as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as ::core::ffi::c_uint,
            '"' as i32 as ::core::ffi::c_char,
        );
        if !ok || arg[0 as ::core::ffi::c_int as usize] == 0 {
            current_block = 7776497614696034663;
        } else {
            ok = copy_arg(
                q,
                &raw mut grp as *mut regmatch_t,
                SET_VAL,
                &raw mut val as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_uint,
                '"' as i32 as ::core::ffi::c_char,
            );
            if !ok {
                current_block = 7776497614696034663;
            } else {
                res = admin_set(
                    admin,
                    &raw mut arg as *mut ::core::ffi::c_char,
                    &raw mut val as *mut ::core::ffi::c_char,
                );
                current_block = 15345379368963487090;
            }
        }
    } else {
        res = syntax_error(admin);
        current_block = 15345379368963487090;
    }
    if current_block == 7776497614696034663 {
        res = admin_error(admin, c"bad arguments".as_ptr());
    }
    current_query = ::core::ptr::null::<::core::ffi::c_char>();
    if !res {
        disconnect_client(admin, true, c"failure".as_ptr());
    }
    res
}
#[no_mangle]

pub unsafe extern "C" fn admin_handle_client(
    mut admin: *mut PgSocket,
    mut pkt: *mut PktHdr,
) -> bool {
    let mut q = ::core::ptr::null::<::core::ffi::c_char>();
    let mut res: bool = false;
    if incomplete_pkt(pkt) {
        disconnect_client(admin, true, c"incomplete pkt".as_ptr());
        return false;
    }
    match (*pkt).type_0 {
        81 => {
            if !mbuf_get_string(&raw mut (*pkt).data, &raw mut q) {
                disconnect_client(admin, true, c"incomplete query".as_ptr());
                return false;
            }
            let mut _log_ctx = NULL;
            if cf_verbose > 0 as ::core::ffi::c_int
            {
                log_generic(LG_DEBUG, _log_ctx, c"got admin query: %s".as_ptr(), q);
            }
            res = admin_parse_query(admin, q);
            if res {
                sbuf_prepare_skip(&raw mut (*admin).sbuf, (*pkt).len);
            }
            return res;
        }
        88 => {
            disconnect_client(admin, false, c"close req".as_ptr());
        }
        80 | 66 | 69 => {
            admin_error(
                admin,
                c"extended query protocol not supported by admin console".as_ptr(),
            );
            disconnect_client(admin, true, c"bad packet".as_ptr());
        }
        _ => {
            admin_error(
                admin,
                c"unsupported packet type for admin console: %d".as_ptr(),
                pkt_desc(pkt) as ::core::ffi::c_int,
            );
            disconnect_client(admin, true, c"bad packet".as_ptr());
        }
    }
    false
}
#[no_mangle]

pub unsafe extern "C" fn admin_pre_login(
    mut client: *mut PgSocket,
    mut username: *const ::core::ffi::c_char,
) -> bool {
    (*client).set_admin_user(false);
    (*client).set_own_user(false);
    if pga_is_unix(&raw mut (*client).remote_addr) {
        let mut peer_uid: uid_t = 0;
        let mut peer_gid: gid_t = 0;
        let mut res: ::core::ffi::c_int = 0;
        res = getpeereid((*client).sbuf.sock, &raw mut peer_uid, &raw mut peer_gid);
        if res >= 0 as ::core::ffi::c_int
            && peer_uid == getuid()
            && strcmp(c"pgbouncer".as_ptr(), username) == 0 as ::core::ffi::c_int
        {
            (*client).login_user_credentials = (*(*admin_pool).db).forced_user_credentials;
            (*client).set_own_user(true);
            (*client).set_admin_user(true);
            if !check_db_connection_count(client) {
                return false;
            }
            if cf_log_connections != 0 {
                log_generic(
                    LG_INFO,
                    client as *mut ::core::ffi::c_void,
                    c"pgbouncer access from unix socket".as_ptr(),
                );
            }
            return true;
        }
    }
    if cf_auth_type == AUTH_TYPE_ANY as ::core::ffi::c_int {
        if strlist_contains(cf_admin_users, username) {
            (*client).login_user_credentials = (*(*admin_pool).db).forced_user_credentials;
            (*client).set_admin_user(true);
            if !check_db_connection_count(client) {
                return false;
            }
            return true;
        } else if strlist_contains(cf_stats_users, username) {
            (*client).login_user_credentials = (*(*admin_pool).db).forced_user_credentials;
            if !check_db_connection_count(client) {
                return false;
            }
            if !check_user_connection_count(client) {
                return false;
            }
            return true;
        }
    }
    false
}
#[no_mangle]

pub unsafe extern "C" fn admin_post_login(mut client: *mut PgSocket) -> bool {
    let mut username: *const ::core::ffi::c_char =
        &raw mut (*(*client).login_user_credentials).name as *mut ::core::ffi::c_char;
    if cf_auth_type == AUTH_TYPE_ANY as ::core::ffi::c_int {
        return true;
    }
    if (*client).admin_user()
        || strlist_contains(cf_admin_users, username)
    {
        (*client).set_admin_user(true);
        return true;
    } else if strlist_contains(cf_stats_users, username) {
        return true;
    }
    disconnect_client(client, true, c"not allowed".as_ptr());
    false
}
#[no_mangle]

pub unsafe extern "C" fn admin_setup() {
    let mut db = ::core::ptr::null_mut::<PgDatabase>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut user = ::core::ptr::null_mut::<PgGlobalUser>();
    let mut msg = ::core::ptr::null_mut::<PktBuf>();
    let mut res: ::core::ffi::c_int = 0;
    db = add_database(c"pgbouncer".as_ptr());
    if db.is_null() {
        let mut _log_ctx = NULL;
        log_generic(LG_FATAL, _log_ctx, c"no memory for admin database".as_ptr());
        exit(1 as ::core::ffi::c_int);
    }
    (*db).port = cf_listen_port;
    (*db).pool_size = 2 as ::core::ffi::c_int;
    (*db).admin = true;
    (*db).pool_mode = POOL_STMT;
    if force_user_credentials(db, c"pgbouncer".as_ptr(), c"".as_ptr()).is_null() {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_0,
            c"no mem on startup - cannot alloc pgbouncer user".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    pool = get_pool(db, (*db).forced_user_credentials);
    if pool.is_null() {
        let mut _log_ctx_1 = NULL;
        log_generic(LG_FATAL, _log_ctx_1, c"cannot create admin pool?".as_ptr());
        exit(1 as ::core::ffi::c_int);
    }
    admin_pool = pool;
    user = find_or_add_new_global_user(c"pgbouncer".as_ptr(), c"".as_ptr());
    if user.is_null() {
        let mut _log_ctx_2 = NULL;
        log_generic(LG_FATAL, _log_ctx_2, c"cannot create admin user?".as_ptr());
        exit(1 as ::core::ffi::c_int);
    }
    msg = pktbuf_dynamic(128 as ::core::ffi::c_int);
    if msg.is_null() {
        let mut _log_ctx_3 = NULL;
        log_generic(LG_FATAL, _log_ctx_3, c"out of memory".as_ptr());
        exit(1 as ::core::ffi::c_int);
    }
    pktbuf_write_generic(
        msg,
        PqMsg_AuthenticationRequest,
        c"i".as_ptr(),
        0 as ::core::ffi::c_int,
    );
    pktbuf_write_generic(
        msg,
        PqMsg_ParameterStatus,
        c"ss".as_ptr(),
        c"server_version".as_ptr(),
        c"1.25.1/bouncer".as_ptr(),
    );
    pktbuf_write_generic(
        msg,
        PqMsg_ParameterStatus,
        c"ss".as_ptr(),
        c"client_encoding".as_ptr(),
        c"UTF8".as_ptr(),
    );
    pktbuf_write_generic(
        msg,
        PqMsg_ParameterStatus,
        c"ss".as_ptr(),
        c"server_encoding".as_ptr(),
        c"UTF8".as_ptr(),
    );
    pktbuf_write_generic(
        msg,
        PqMsg_ParameterStatus,
        c"ss".as_ptr(),
        c"DateStyle".as_ptr(),
        c"ISO".as_ptr(),
    );
    pktbuf_write_generic(
        msg,
        PqMsg_ParameterStatus,
        c"ss".as_ptr(),
        c"TimeZone".as_ptr(),
        c"GMT".as_ptr(),
    );
    pktbuf_write_generic(
        msg,
        PqMsg_ParameterStatus,
        c"ss".as_ptr(),
        c"standard_conforming_strings".as_ptr(),
        c"on".as_ptr(),
    );
    pktbuf_write_generic(
        msg,
        PqMsg_ParameterStatus,
        c"ss".as_ptr(),
        c"is_superuser".as_ptr(),
        c"on".as_ptr(),
    );
    if (*msg).failed() {
        let mut _log_ctx_4 = NULL;
        log_generic(LG_FATAL, _log_ctx_4, c"admin welcome failed".as_ptr());
        exit(1 as ::core::ffi::c_int);
    }
    (*pool).welcome_msg = msg as *mut PktBuf;
    (*pool).set_welcome_msg_ready(true);
    msg = pktbuf_dynamic(128 as ::core::ffi::c_int);
    if msg.is_null() {
        let mut _log_ctx_5 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_5,
            c"cannot create admin startup pkt".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    (*db).startup_params = msg as *mut PktBuf;
    pktbuf_put_string(msg, c"database".as_ptr());
    (*db).dbname = c"pgbouncer".as_ptr();
    pktbuf_put_string(msg, (*db).dbname);
    res = regcomp(
        &raw mut rc_cmd,
        &raw const cmd_normal_rx as *const ::core::ffi::c_char,
        REG_EXTENDED | REG_ICASE,
    );
    if res != 0 as ::core::ffi::c_int {
        let mut _log_ctx_6 = NULL;
        log_fatal(
            c"src/admin.c".as_ptr(),
            1927 as ::core::ffi::c_int,
            c"admin_setup".as_ptr(),
            false,
            _log_ctx_6,
            c"cmd regex compilation error".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    res = regcomp(
        &raw mut rc_set_word,
        &raw const cmd_set_word_rx as *const ::core::ffi::c_char,
        REG_EXTENDED | REG_ICASE,
    );
    if res != 0 as ::core::ffi::c_int {
        let mut _log_ctx_7 = NULL;
        log_fatal(
            c"src/admin.c".as_ptr(),
            1930 as ::core::ffi::c_int,
            c"admin_setup".as_ptr(),
            false,
            _log_ctx_7,
            c"set/word regex compilation error".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    res = regcomp(
        &raw mut rc_set_str,
        &raw const cmd_set_str_rx as *const ::core::ffi::c_char,
        REG_EXTENDED | REG_ICASE,
    );
    if res != 0 as ::core::ffi::c_int {
        let mut _log_ctx_8 = NULL;
        log_fatal(
            c"src/admin.c".as_ptr(),
            1933 as ::core::ffi::c_int,
            c"admin_setup".as_ptr(),
            false,
            _log_ctx_8,
            c"set/str regex compilation error".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
}
#[no_mangle]

pub unsafe extern "C" fn admin_pause_done() {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut admin = ::core::ptr::null_mut::<PgSocket>();
    let mut res: bool = false;
    item = (*admin_pool).active_client_list.head.next;
    tmp = (*(*admin_pool).active_client_list.head.next).next;
    while item != &raw mut (*admin_pool).active_client_list.head {
        admin = (item as *mut ::core::ffi::c_char)
            as *mut PgSocket;
        if (*admin).wait_for_response() {
            match cf_pause_mode {
                1 => {
                    res = admin_ready(admin, c"PAUSE".as_ptr());
                }
                2 => {
                    res = admin_ready(admin, c"SUSPEND".as_ptr());
                }
                _ => {
                    if count_paused_databases() > 0 as ::core::ffi::c_int {
                        res = admin_ready(admin, c"PAUSE".as_ptr());
                    } else {
                        let mut _log_ctx = NULL;
                        log_fatal(
                            c"src/admin.c".as_ptr(),
                            1959 as ::core::ffi::c_int,
                            c"admin_pause_done".as_ptr(),
                            false,
                            _log_ctx,
                            c"admin_pause_done: bad state".as_ptr(),
                        );
                        exit(1 as ::core::ffi::c_int);
                    }
                }
            }
            if !res {
                disconnect_client(admin, false, c"dead admin".as_ptr());
            } else {
                (*admin).set_wait_for_response(false);
            }
        }
        item = tmp;
        tmp = (*tmp).next;
    }
    if statlist_empty(&raw mut (*admin_pool).active_client_list)
        && cf_pause_mode == P_SUSPEND as ::core::ffi::c_int
    {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_INFO,
            _log_ctx_0,
            c"admin disappeared when suspended, doing RESUME".as_ptr(),
        );
        cf_pause_mode = P_NONE as ::core::ffi::c_int;
        resume_all();
    }
}
#[no_mangle]

pub unsafe extern "C" fn admin_wait_close_done() {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut admin = ::core::ptr::null_mut::<PgSocket>();
    let mut res: bool = false;
    item = (*admin_pool).active_client_list.head.next;
    tmp = (*(*admin_pool).active_client_list.head.next).next;
    while item != &raw mut (*admin_pool).active_client_list.head {
        admin = (item as *mut ::core::ffi::c_char)
            as *mut PgSocket;
        if (*admin).wait_for_response() {
            res = admin_ready(admin, c"WAIT_CLOSE".as_ptr());
            if !res {
                disconnect_client(admin, false, c"dead admin".as_ptr());
            } else {
                (*admin).set_wait_for_response(false);
            }
        }
        item = tmp;
        tmp = (*tmp).next;
    }
}
#[no_mangle]

pub unsafe extern "C" fn admin_handle_cancel(mut admin: *mut PgSocket) {
    if !(*admin).wait_for_response() {
        log_generic(
            LG_WARNING,
            admin as *mut ::core::ffi::c_void,
            c"admin cancel request for non-waiting client?".as_ptr(),
        );
    }
    if cf_shutdown != 0 {
        return;
    }
    if cf_pause_mode != P_NONE as ::core::ffi::c_int {
        full_resume();
    }
}

extern "C" {
    pub fn get_cached_time() -> usec_t;
}
