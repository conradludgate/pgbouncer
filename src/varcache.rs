pub mod sys__types_h {

    pub type __darwin_pid_t = __int32_t;

    pub type __darwin_suseconds_t = __int32_t;

    pub type __darwin_uid_t = __uint32_t;

    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    use crate::types::{__int32_t, __uint32_t};
}

pub mod tls_h {
    extern "C" {

        pub type tls;
    }
}

pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct sockaddr {
        pub sa_len: __uint8_t,
        pub sa_family: sa_family_t,
        pub sa_data: [::core::ffi::c_char; 14],
    }
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
    use super::dnslookup_h::DNSToken;
    use super::in6_h::sockaddr_in6;
    use super::in_h::sockaddr_in;
    use super::pktbuf_h::PktBuf;
    use super::sbuf_h::SBuf;
    use super::socket_h::sockaddr;
    use crate::types::pg_cryptohash_type;
    use crate::types::pid_t;
    use crate::types::uid_t;
    use crate::types::uint16_t;
    use crate::types::uint64_t;
    use crate::types::uint8_t;
    use crate::types::usec_t;
    use crate::types::List;
    use crate::types::PktHdr;
    use crate::types::StatList;
    use crate::types::VarCache;
    use crate::types::{AANode, AATree};
    use crate::types::{PgClientPreparedStatement, PgServerPreparedStatement};
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
    use super::event_struct_h::event;
    use super::iobuf_h::IOBuf;
    use super::tls_h::tls;
    use crate::types::size_t;
    use crate::types::ssize_t;
    use crate::types::uint8_t;
    use crate::types::MBuf;
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

        pub fn pktbuf_temp() -> *mut PktBuf;

        pub fn pktbuf_send_immediate(buf: *mut PktBuf, sk: *mut PgSocket) -> bool;

        pub fn pktbuf_start_packet(buf: *mut PktBuf, type_0: ::core::ffi::c_int);

        pub fn pktbuf_put_char(buf: *mut PktBuf, val: ::core::ffi::c_char);

        pub fn pktbuf_put_string(buf: *mut PktBuf, str: *const ::core::ffi::c_char);

        pub fn pktbuf_put_bytes(
            buf: *mut PktBuf,
            data: *const ::core::ffi::c_void,
            len: ::core::ffi::c_int,
        );

        pub fn pktbuf_finish_packet(buf: *mut PktBuf);

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

pub mod string_h {

    pub type str_cb =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char) -> bool>;
    use super::cxalloc_h::CxMem;
    extern "C" {

        pub type StrList;

        pub fn strlist_new(ca: *const CxMem) -> *mut StrList;

        pub fn strlist_free(slist: *mut StrList);

        pub fn strlist_empty(slist: *mut StrList) -> bool;

        pub fn strlist_append(slist: *mut StrList, str: *const ::core::ffi::c_char) -> bool;

        pub fn strlist_pop(slist: *mut StrList) -> *mut ::core::ffi::c_char;

        pub fn parse_word_list(
            s: *const ::core::ffi::c_char,
            cb_func: str_cb,
            cb_arg: *mut ::core::ffi::c_void,
        ) -> bool;
    }
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
    }
}

pub mod _malloc_h {
    use crate::types::size_t;
    extern "C" {

        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;

        pub fn free(_: *mut ::core::ffi::c_void);
    }
}

pub mod _ctype_h {
    #[inline]

    pub unsafe extern "C" fn tolower(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        __tolower(_c as __darwin_ct_rune_t) as ::core::ffi::c_int
    }
    use crate::types::__darwin_ct_rune_t;
    extern "C" {

        pub fn __tolower(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    }
}

pub mod ctype_h {
    #[inline]

    pub unsafe extern "C" fn safe_tolower(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        tolower(c as ::core::ffi::c_uchar as ::core::ffi::c_int)
    }
    use super::_ctype_h::tolower;
}

pub mod _string_h {
    use crate::types::size_t;
    extern "C" {

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

        pub fn strdup(__s1: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    }
}

pub mod _stdlib_h {
    extern "C" {

        pub fn exit(_: ::core::ffi::c_int) -> !;
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

pub mod protocol_h {

    pub const PqMsg_Query: ::core::ffi::c_int = 'Q' as i32;

    pub const PqMsg_ParameterStatus: ::core::ffi::c_int = 'S' as i32;
}

pub mod pgutil_h {
    extern "C" {

        pub fn pg_quote_literal(
            _dst: *mut ::core::ffi::c_char,
            _src: *const ::core::ffi::c_char,
            dstlen: ::core::ffi::c_int,
        ) -> bool;
    }
}
pub use self::_ctype_h::{__tolower, tolower};
use self::_malloc_h::{free, malloc};
use self::_stdio_h::snprintf;
use self::_stdlib_h::exit;
use self::_string_h::{memset, strcmp, strdup, strlen};
use self::_strings_h::strcasecmp;
pub use self::bouncer_h::{
    sockaddr_ucreds, C2RustUnnamed_9, CallbackState, LoadBalanceHosts, PacketCallbackFlag, PgAddr,
    PgCredentials, PgDatabase, PgGlobalUser, PgPool, PgSocket, PgStats, ReplicationType,
    ScramState, SocketState, CB_HANDLE_COMPLETE_PACKET, CB_NONE, CB_WANT_COMPLETE_PACKET,
    CL_ACTIVE, CL_ACTIVE_CANCEL, CL_FREE, CL_JUSTFREE, CL_LOGIN, CL_WAITING, CL_WAITING_CANCEL,
    CL_WAITING_LOGIN, LOAD_BALANCE_HOSTS_DISABLE, LOAD_BALANCE_HOSTS_ROUND_ROBIN,
    REPLICATION_LOGICAL, REPLICATION_NONE, REPLICATION_PHYSICAL, SV_ACTIVE, SV_ACTIVE_CANCEL,
    SV_BEING_CANCELED, SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN, SV_TESTED, SV_USED,
};
pub use self::ctype_h::safe_tolower;
pub use self::cxalloc_h::{cx_libc_allocator, CxMem, CxOps};
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
    __darwin_ct_rune_t, __darwin_ptrdiff_t, __darwin_size_t, __darwin_ssize_t, __darwin_time_t,
    __int32_t, __uint16_t, __uint32_t, __uint8_t,
};
pub use crate::types::{aatree_cmp_f, aatree_walker_f, AANode, AATree};
pub use crate::types::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};

pub use self::event_struct_h::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
pub use self::in6_h::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use self::in_h::{in_addr, sockaddr_in};
pub use self::iobuf_h::{iobuf, IOBuf};
pub use self::logging_h::{
    cf_verbose, log_fatal, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE,
    LG_STATS, LG_WARNING,
};
use self::pgutil_h::pg_quote_literal;
pub use self::pktbuf_h::{
    pktbuf_finish_packet, pktbuf_put_bytes, pktbuf_put_char, pktbuf_put_string,
    pktbuf_send_immediate, pktbuf_start_packet, pktbuf_temp, pktbuf_write_generic, PktBuf,
};
pub use self::protocol_h::{PqMsg_ParameterStatus, PqMsg_Query};
pub use self::sbuf_h::{
    sbuf_cb_t, SBuf, SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK, SBUF_EV_FLUSH,
    SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY,
};
pub use self::socket_h::sockaddr;
pub use self::string_h::{
    parse_word_list, str_cb, strlist_append, strlist_empty, strlist_free, strlist_new, strlist_pop,
    StrList,
};
pub use self::sys__types_h::{__darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t, __DARWIN_NULL};
pub use crate::types::usec_t;
pub use crate::types::List;
pub use crate::types::MBuf;
pub use crate::types::PktHdr;
pub use crate::types::StatList;
pub use crate::types::{false_0, true_0};
pub use crate::types::{PStr, StrPool};
pub use crate::types::{PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement};

pub use crate::types::VarCache;
pub use crate::types::{
    UT_hash_bucket, UT_hash_handle, UT_hash_table, HASH_BKT_CAPACITY_THRESH,
    HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE,
};
#[derive(Copy, Clone)]
#[repr(C)]

pub struct var_lookup {
    pub name: *const ::core::ffi::c_char,
    pub idx: ::core::ffi::c_int,
    pub hh: UT_hash_handle,
}

static mut num_var_cached: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

static mut lookup_map: *mut var_lookup = ::core::ptr::null::<var_lookup>() as *mut var_lookup;

static mut vpool: *mut StrPool = ::core::ptr::null::<StrPool>() as *mut StrPool;
#[inline]

unsafe extern "C" fn get_value(mut cache: *mut VarCache, mut lk: *const var_lookup) -> *mut PStr {
    *(*cache).var_list.offset((*lk).idx as isize)
}

unsafe extern "C" fn sl_add(
    mut arg: *mut ::core::ffi::c_void,
    mut s: *const ::core::ffi::c_char,
) -> bool {
    strlist_append(arg as *mut StrList, s)
}
#[no_mangle]

pub unsafe extern "C" fn get_num_var_cached() -> ::core::ffi::c_int {
    num_var_cached
}

unsafe extern "C" fn init_var_lookup_from_config(
    mut cf_track_extra_parameters: *const ::core::ffi::c_char,
    mut num_vars: *mut ::core::ffi::c_int,
) {
    let mut var_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut lookup = ::core::ptr::null_mut::<var_lookup>();
    let mut sl = strlist_new(::core::ptr::null::<CxMem>());
    if !parse_word_list(
        cf_track_extra_parameters,
        Some(
            sl_add
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                ) -> bool,
        ),
        sl as *mut ::core::ffi::c_void,
    ) {
        let mut _log_ctx = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx,
            c"failed to parse track_extra_parameters in config %s".as_ptr(),
            cf_track_extra_parameters,
        );
        exit(1 as ::core::ffi::c_int);
    }
    while !strlist_empty(sl) {
        var_name = strlist_pop(sl);
        if var_name.is_null() {
            continue;
        }
        let mut _uthash_hfstr_keylen = strlen(var_name) as ::core::ffi::c_uint;
        lookup = ::core::ptr::null_mut::<var_lookup>();
        if !lookup_map.is_null() {
            let mut _hf_hashv: ::core::ffi::c_uint = 0;
            let mut _hj_i: ::core::ffi::c_uint = 0;
            let mut _hj_j: ::core::ffi::c_uint = 0;
            let mut _hj_k: ::core::ffi::c_uint = 0;
            let mut _hj_key = var_name as *const ::core::ffi::c_uchar;
            _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
            _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
            _hj_i = _hj_j;
            _hj_k = _uthash_hfstr_keylen;
            while _hj_k >= 12 as ::core::ffi::c_uint {
                _hj_i = _hj_i.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (safe_tolower(*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (safe_tolower(*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (safe_tolower(*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_j = _hj_j.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (safe_tolower(*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (safe_tolower(*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (safe_tolower(*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hf_hashv = _hf_hashv.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (safe_tolower(*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (safe_tolower(*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (safe_tolower(*_hj_key.offset(11 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_i = _hj_i.wrapping_sub(_hj_j);
                _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
                _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                _hj_j = _hj_j.wrapping_sub(_hj_i);
                _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
                _hj_i = _hj_i.wrapping_sub(_hj_j);
                _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
                _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                _hj_j = _hj_j.wrapping_sub(_hj_i);
                _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
                _hj_i = _hj_i.wrapping_sub(_hj_j);
                _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
                _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                _hj_j = _hj_j.wrapping_sub(_hj_i);
                _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
                _hj_key = _hj_key.offset(12 as ::core::ffi::c_int as isize);
                _hj_k = _hj_k.wrapping_sub(12 as ::core::ffi::c_uint);
            }
            _hf_hashv = _hf_hashv.wrapping_add(_uthash_hfstr_keylen);
            let mut current_block_58: u64;
            match _hj_k {
                11 => {
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (safe_tolower(*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_58 = 15275504483753031639;
                }
                10 => {
                    current_block_58 = 15275504483753031639;
                }
                9 => {
                    current_block_58 = 14514390918547092561;
                }
                8 => {
                    current_block_58 = 1205942404857220978;
                }
                7 => {
                    current_block_58 = 10244489138959580834;
                }
                6 => {
                    current_block_58 = 7231708513041901785;
                }
                5 => {
                    current_block_58 = 9781868647333494730;
                }
                4 => {
                    current_block_58 = 12285225998683619597;
                }
                3 => {
                    current_block_58 = 4389422820558755489;
                }
                2 => {
                    current_block_58 = 6122795728994222029;
                }
                1 => {
                    current_block_58 = 16419358174521838934;
                }
                _ => {
                    current_block_58 = 7420279277351916581;
                }
            }
            if current_block_58 == 15275504483753031639 {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_58 = 14514390918547092561;
            }
            if current_block_58 == 14514390918547092561 {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_58 = 1205942404857220978;
            }
            if current_block_58 == 1205942404857220978 {
                _hj_j = _hj_j.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_58 = 10244489138959580834;
            }
            if current_block_58 == 10244489138959580834 {
                _hj_j = _hj_j.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_58 = 7231708513041901785;
            }
            if current_block_58 == 7231708513041901785 {
                _hj_j = _hj_j.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_58 = 9781868647333494730;
            }
            if current_block_58 == 9781868647333494730 {
                _hj_j = _hj_j.wrapping_add(safe_tolower(
                    *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                ) as ::core::ffi::c_uint);
                current_block_58 = 12285225998683619597;
            }
            if current_block_58 == 12285225998683619597 {
                _hj_i = _hj_i.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_58 = 4389422820558755489;
            }
            if current_block_58 == 4389422820558755489 {
                _hj_i = _hj_i.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_58 = 6122795728994222029;
            }
            if current_block_58 == 6122795728994222029 {
                _hj_i = _hj_i.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_58 = 16419358174521838934;
            }
            if current_block_58 == 16419358174521838934 {
                _hj_i = _hj_i.wrapping_add(safe_tolower(
                    *_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                ) as ::core::ffi::c_uint);
            }
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
            lookup = ::core::ptr::null_mut::<var_lookup>();
            if !lookup_map.is_null() {
                let mut _hf_bkt: ::core::ffi::c_uint = 0;
                _hf_bkt = _hf_hashv
                    & (*(*lookup_map).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                if !(*(*(*lookup_map).hh.tbl).buckets.offset(_hf_bkt as isize))
                    .hh_head
                    .is_null()
                {
                    lookup = ((*(*(*lookup_map).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                        as *mut ::core::ffi::c_char)
                        .offset(-(*(*lookup_map).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut var_lookup
                        as *mut var_lookup;
                } else {
                    lookup = ::core::ptr::null_mut::<var_lookup>();
                }
                while !lookup.is_null() {
                    if (*lookup).hh.hashv == _hf_hashv
                        && (*lookup).hh.keylen == _uthash_hfstr_keylen
                        && strcasecmp((*lookup).hh.key as *const ::core::ffi::c_char, var_name)
                            == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                    if !(*lookup).hh.hh_next.is_null() {
                        lookup = ((*lookup).hh.hh_next as *mut ::core::ffi::c_char)
                            .offset(-(*(*lookup_map).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut var_lookup as *mut var_lookup;
                    } else {
                        lookup = ::core::ptr::null_mut::<var_lookup>();
                    }
                }
            }
        }
        if !lookup.is_null() {
            continue;
        }
        lookup = malloc(::core::mem::size_of::<var_lookup>() as size_t) as *mut var_lookup;
        (*lookup).name = strdup(var_name);
        let fresh3 = *num_vars;
        *num_vars += 1;
        (*lookup).idx = fresh3;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i_0: ::core::ffi::c_uint = 0;
        let mut _hj_j_0: ::core::ffi::c_uint = 0;
        let mut _hj_k_0: ::core::ffi::c_uint = 0;
        let mut _hj_key_0 = (*lookup).name as *const ::core::ffi::c_uchar;
        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_0 = _hj_j_0;
        _hj_k_0 = strlen((*lookup).name) as ::core::ffi::c_uint;
        while _hj_k_0 >= 12 as ::core::ffi::c_uint {
            _hj_i_0 = _hj_i_0.wrapping_add(
                (safe_tolower(
                    *_hj_key_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (safe_tolower(*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_j_0 = _hj_j_0.wrapping_add(
                (safe_tolower(
                    *_hj_key_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (safe_tolower(*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _ha_hashv = _ha_hashv.wrapping_add(
                (safe_tolower(
                    *_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (safe_tolower(*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(*_hj_key_0.offset(11 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
            _hj_key_0 = _hj_key_0.offset(12 as ::core::ffi::c_int as isize);
            _hj_k_0 = _hj_k_0.wrapping_sub(12 as ::core::ffi::c_uint);
        }
        _ha_hashv = _ha_hashv.wrapping_add(strlen((*lookup).name) as ::core::ffi::c_uint);
        let mut current_block_180: u64;
        match _hj_k_0 {
            11 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (safe_tolower(
                        *_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_180 = 8153286781957891950;
            }
            10 => {
                current_block_180 = 8153286781957891950;
            }
            9 => {
                current_block_180 = 9978657853023849837;
            }
            8 => {
                current_block_180 = 14748375350202154019;
            }
            7 => {
                current_block_180 = 986562491153128823;
            }
            6 => {
                current_block_180 = 2419899655327389259;
            }
            5 => {
                current_block_180 = 18057295174972137893;
            }
            4 => {
                current_block_180 = 428438469909915749;
            }
            3 => {
                current_block_180 = 1152747300973462357;
            }
            2 => {
                current_block_180 = 9357788886370794354;
            }
            1 => {
                current_block_180 = 17622820276845907441;
            }
            _ => {
                current_block_180 = 4534765400774009001;
            }
        }
        if current_block_180 == 8153286781957891950 {
            _ha_hashv = _ha_hashv.wrapping_add(
                (safe_tolower(
                    *_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_180 = 9978657853023849837;
        }
        if current_block_180 == 9978657853023849837 {
            _ha_hashv = _ha_hashv.wrapping_add(
                (safe_tolower(
                    *_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_180 = 14748375350202154019;
        }
        if current_block_180 == 14748375350202154019 {
            _hj_j_0 = _hj_j_0.wrapping_add(
                (safe_tolower(
                    *_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_180 = 986562491153128823;
        }
        if current_block_180 == 986562491153128823 {
            _hj_j_0 = _hj_j_0.wrapping_add(
                (safe_tolower(
                    *_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_180 = 2419899655327389259;
        }
        if current_block_180 == 2419899655327389259 {
            _hj_j_0 = _hj_j_0.wrapping_add(
                (safe_tolower(
                    *_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_180 = 18057295174972137893;
        }
        if current_block_180 == 18057295174972137893 {
            _hj_j_0 = _hj_j_0.wrapping_add(safe_tolower(
                *_hj_key_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint);
            current_block_180 = 428438469909915749;
        }
        if current_block_180 == 428438469909915749 {
            _hj_i_0 = _hj_i_0.wrapping_add(
                (safe_tolower(
                    *_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_180 = 1152747300973462357;
        }
        if current_block_180 == 1152747300973462357 {
            _hj_i_0 = _hj_i_0.wrapping_add(
                (safe_tolower(
                    *_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_180 = 9357788886370794354;
        }
        if current_block_180 == 9357788886370794354 {
            _hj_i_0 = _hj_i_0.wrapping_add(
                (safe_tolower(
                    *_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_180 = 17622820276845907441;
        }
        if current_block_180 == 17622820276845907441 {
            _hj_i_0 = _hj_i_0.wrapping_add(safe_tolower(
                *_hj_key_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint);
        }
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
        _hj_i_0 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
        _ha_hashv ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
        _hj_i_0 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
        _ha_hashv ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
        _hj_i_0 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
        _ha_hashv ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
        let mut _ha_oomed = 0 as ::core::ffi::c_int;
        (*lookup).hh.hashv = _ha_hashv;
        (*lookup).hh.key = (*lookup).name as *const ::core::ffi::c_void;
        (*lookup).hh.keylen = strlen((*lookup).name) as ::core::ffi::c_uint;
        if lookup_map.is_null() {
            (*lookup).hh.next = NULL;
            (*lookup).hh.prev = NULL;
            (*lookup).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as size_t)
                as *mut UT_hash_table as *mut UT_hash_table;
            if (*lookup).hh.tbl.is_null() {
                _ha_oomed = 1 as ::core::ffi::c_int;
            } else {
                memset(
                    (*lookup).hh.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UT_hash_table>() as size_t,
                );
                (*(*lookup).hh.tbl).tail = &raw mut (*lookup).hh as *mut UT_hash_handle;
                (*(*lookup).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*lookup).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*lookup).hh.tbl).hho = (&raw mut (*lookup).hh as *mut ::core::ffi::c_char)
                    .offset_from(lookup as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as ptrdiff_t;
                (*(*lookup).hh.tbl).buckets = malloc(
                    (32 as size_t).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                ) as *mut UT_hash_bucket;
                (*(*lookup).hh.tbl).signature = HASH_SIGNATURE as uint32_t;
                if (*(*lookup).hh.tbl).buckets.is_null() {
                    _ha_oomed = 1 as ::core::ffi::c_int;
                    free((*lookup).hh.tbl as *mut ::core::ffi::c_void);
                } else {
                    memset(
                        (*(*lookup).hh.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as size_t)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                    );
                    if _ha_oomed != 0 {
                        free((*(*lookup).hh.tbl).buckets as *mut ::core::ffi::c_void);
                        free((*lookup).hh.tbl as *mut ::core::ffi::c_void);
                    }
                }
            }
            if _ha_oomed == 0 {
                lookup_map = lookup;
            }
        } else {
            (*lookup).hh.tbl = (*lookup_map).hh.tbl;
            (*lookup).hh.next = NULL;
            (*lookup).hh.prev = ((*(*lookup_map).hh.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*lookup_map).hh.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*lookup_map).hh.tbl).tail).next = lookup as *mut ::core::ffi::c_void;
            (*(*lookup_map).hh.tbl).tail = &raw mut (*lookup).hh as *mut UT_hash_handle;
        }
        if _ha_oomed == 0 {
            let mut _ha_bkt: ::core::ffi::c_uint = 0;
            (*(*lookup_map).hh.tbl).num_items = (*(*lookup_map).hh.tbl).num_items.wrapping_add(1);
            _ha_bkt = _ha_hashv
                & (*(*lookup_map).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _ha_head: *mut UT_hash_bucket =
                (*(*lookup_map).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UT_hash_bucket;
            (*_ha_head).count += 1;
            (*lookup).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
            (*lookup).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
            if !(*_ha_head).hh_head.is_null() {
                (*(*_ha_head).hh_head).hh_prev = &raw mut (*lookup).hh as *mut UT_hash_handle;
            }
            (*_ha_head).hh_head = &raw mut (*lookup).hh as *mut UT_hash_handle;
            if (*_ha_head).count
                >= (*_ha_head)
                    .expand_mult
                    .wrapping_add(1 as ::core::ffi::c_uint)
                    .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                && (*(*lookup).hh.tbl).noexpand == 0
            {
                let mut _he_bkt: ::core::ffi::c_uint = 0;
                let mut _he_bkt_i: ::core::ffi::c_uint = 0;
                let mut _he_thh = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _he_hh_nxt = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _he_new_buckets = ::core::ptr::null_mut::<UT_hash_bucket>();
                let mut _he_newbkt = ::core::ptr::null_mut::<UT_hash_bucket>();
                _he_new_buckets = malloc(
                    (::core::mem::size_of::<UT_hash_bucket>() as size_t)
                        .wrapping_mul((*(*lookup).hh.tbl).num_buckets as size_t)
                        .wrapping_mul(2 as size_t),
                ) as *mut UT_hash_bucket;
                if _he_new_buckets.is_null() {
                    _ha_oomed = 1 as ::core::ffi::c_int;
                } else {
                    memset(
                        _he_new_buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (::core::mem::size_of::<UT_hash_bucket>() as size_t)
                            .wrapping_mul((*(*lookup).hh.tbl).num_buckets as size_t)
                            .wrapping_mul(2 as size_t),
                    );
                    (*(*lookup).hh.tbl).ideal_chain_maxlen = ((*(*lookup).hh.tbl).num_items
                        >> (*(*lookup).hh.tbl)
                            .log2_num_buckets
                            .wrapping_add(1 as ::core::ffi::c_uint))
                    .wrapping_add(
                        if (*(*lookup).hh.tbl).num_items
                            & (*(*lookup).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint)
                            != 0 as ::core::ffi::c_uint
                        {
                            1 as ::core::ffi::c_uint
                        } else {
                            0 as ::core::ffi::c_uint
                        },
                    );
                    (*(*lookup).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                    _he_bkt_i = 0 as ::core::ffi::c_uint;
                    while _he_bkt_i < (*(*lookup).hh.tbl).num_buckets {
                        _he_thh = (*(*(*lookup).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                            as *mut UT_hash_handle;
                        while !_he_thh.is_null() {
                            _he_hh_nxt = (*_he_thh).hh_next;
                            _he_bkt = (*_he_thh).hashv
                                & (*(*lookup).hh.tbl)
                                    .num_buckets
                                    .wrapping_mul(2 as ::core::ffi::c_uint)
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            _he_newbkt =
                                _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                            (*_he_newbkt).count += 1;
                            if (*_he_newbkt).count > (*(*lookup).hh.tbl).ideal_chain_maxlen {
                                (*(*lookup).hh.tbl).nonideal_items =
                                    (*(*lookup).hh.tbl).nonideal_items.wrapping_add(1);
                                if (*_he_newbkt).count
                                    > (*_he_newbkt)
                                        .expand_mult
                                        .wrapping_mul((*(*lookup).hh.tbl).ideal_chain_maxlen)
                                {
                                    (*_he_newbkt).expand_mult += 1;
                                }
                            }
                            (*_he_thh).hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                            (*_he_thh).hh_next = (*_he_newbkt).hh_head as *mut UT_hash_handle;
                            if !(*_he_newbkt).hh_head.is_null() {
                                (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                            }
                            (*_he_newbkt).hh_head = _he_thh as *mut UT_hash_handle;
                            _he_thh = _he_hh_nxt;
                        }
                        _he_bkt_i = _he_bkt_i.wrapping_add(1);
                    }
                    free((*(*lookup).hh.tbl).buckets as *mut ::core::ffi::c_void);
                    (*(*lookup).hh.tbl).num_buckets = (*(*lookup).hh.tbl)
                        .num_buckets
                        .wrapping_mul(2 as ::core::ffi::c_uint);
                    (*(*lookup).hh.tbl).log2_num_buckets =
                        (*(*lookup).hh.tbl).log2_num_buckets.wrapping_add(1);
                    (*(*lookup).hh.tbl).buckets = _he_new_buckets;
                    (*(*lookup).hh.tbl).ineff_expands = if (*(*lookup).hh.tbl).nonideal_items
                        > (*(*lookup).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                    {
                        (*(*lookup).hh.tbl)
                            .ineff_expands
                            .wrapping_add(1 as ::core::ffi::c_uint)
                    } else {
                        0 as ::core::ffi::c_uint
                    };
                    if (*(*lookup).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                        (*(*lookup).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                    }
                }
                if _ha_oomed != 0 {
                    let mut _hd_head: *mut UT_hash_bucket =
                        (*(*lookup_map).hh.tbl).buckets.offset(_ha_bkt as isize)
                            as *mut UT_hash_bucket;
                    (*_hd_head).count -= 1;
                    if (*_hd_head).hh_head == &raw mut (*lookup).hh {
                        (*_hd_head).hh_head = (*lookup).hh.hh_next as *mut UT_hash_handle;
                    }
                    if !(*lookup).hh.hh_prev.is_null() {
                        (*(*lookup).hh.hh_prev).hh_next = (*lookup).hh.hh_next;
                    }
                    if !(*lookup).hh.hh_next.is_null() {
                        (*(*lookup).hh.hh_next).hh_prev = (*lookup).hh.hh_prev;
                    }
                }
            }
            if _ha_oomed != 0 {
                let mut _hd_hh_item: *mut UT_hash_handle = &raw mut (*lookup).hh;
                let mut _hd_bkt: ::core::ffi::c_uint = 0;
                _hd_bkt = (*_hd_hh_item).hashv
                    & (*(*lookup_map).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let fresh4 = &mut (*(*(*lookup_map).hh.tbl).buckets.offset(_hd_bkt as isize)).count;
                *fresh4 = (*fresh4).wrapping_add(1);
                (*_hd_hh_item).hh_next = ::core::ptr::null_mut::<UT_hash_handle>();
                (*_hd_hh_item).hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _hd_hh_del: *const UT_hash_handle = &raw mut (*lookup).hh;
                if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
                    free((*(*lookup_map).hh.tbl).buckets as *mut ::core::ffi::c_void);
                    free((*lookup_map).hh.tbl as *mut ::core::ffi::c_void);
                    lookup_map = ::core::ptr::null_mut::<var_lookup>();
                } else {
                    let mut _hd_bkt_0: ::core::ffi::c_uint = 0;
                    if std::ptr::eq(_hd_hh_del, (*(*lookup_map).hh.tbl).tail) {
                        (*(*lookup_map).hh.tbl).tail = ((*_hd_hh_del).prev
                            as *mut ::core::ffi::c_char)
                            .offset((*(*lookup_map).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut UT_hash_handle
                            as *mut UT_hash_handle;
                    }
                    if !(*_hd_hh_del).prev.is_null() {
                        let fresh5 = &mut (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                            .offset((*(*lookup_map).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut UT_hash_handle))
                            .next;
                        *fresh5 = (*_hd_hh_del).next;
                    } else {
                        lookup_map = (*_hd_hh_del).next as *mut var_lookup as *mut var_lookup;
                    }
                    if !(*_hd_hh_del).next.is_null() {
                        let fresh6 = &mut (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                            .offset((*(*lookup_map).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut UT_hash_handle))
                            .prev;
                        *fresh6 = (*_hd_hh_del).prev;
                    }
                    _hd_bkt_0 = (*_hd_hh_del).hashv
                        & (*(*lookup_map).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    let mut _hd_head_0: *mut UT_hash_bucket =
                        (*(*lookup_map).hh.tbl).buckets.offset(_hd_bkt_0 as isize)
                            as *mut UT_hash_bucket;
                    (*_hd_head_0).count -= 1;
                    if std::ptr::eq((*_hd_head_0).hh_head, _hd_hh_del) {
                        (*_hd_head_0).hh_head = (*_hd_hh_del).hh_next as *mut UT_hash_handle;
                    }
                    if !(*_hd_hh_del).hh_prev.is_null() {
                        (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
                    }
                    if !(*_hd_hh_del).hh_next.is_null() {
                        (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
                    }
                    (*(*lookup_map).hh.tbl).num_items =
                        (*(*lookup_map).hh.tbl).num_items.wrapping_sub(1);
                }
                (*lookup).hh.tbl = ::core::ptr::null_mut::<UT_hash_table>();
                let mut _log_ctx_0 = NULL;
                log_fatal(
                    c"src/varcache.c".as_ptr(),
                    81 as ::core::ffi::c_int,
                    c"init_var_lookup_from_config".as_ptr(),
                    false,
                    _log_ctx_0,
                    c"out of memory".as_ptr(),
                );
                exit(1 as ::core::ffi::c_int);
            }
        } else {
            (*lookup).hh.tbl = ::core::ptr::null_mut::<UT_hash_table>();
            let mut _log_ctx_1 = NULL;
            log_fatal(
                c"src/varcache.c".as_ptr(),
                81 as ::core::ffi::c_int,
                c"init_var_lookup_from_config".as_ptr(),
                false,
                _log_ctx_1,
                c"out of memory".as_ptr(),
            );
            exit(1 as ::core::ffi::c_int);
        }
        free(var_name as *mut ::core::ffi::c_void);
    }
    strlist_free(sl);
}
#[no_mangle]

pub unsafe extern "C" fn init_var_lookup(
    mut cf_track_extra_parameters: *const ::core::ffi::c_char,
) {
    let mut names: [*const ::core::ffi::c_char; 6] = [
        c"DateStyle".as_ptr(),
        c"client_encoding".as_ptr(),
        c"TimeZone".as_ptr(),
        c"standard_conforming_strings".as_ptr(),
        c"application_name".as_ptr(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut idx = 0 as ::core::ffi::c_int;
    let mut lookup = ::core::ptr::null_mut::<var_lookup>();
    while !names[idx as usize].is_null() {
        lookup = malloc(::core::mem::size_of::<var_lookup>() as size_t) as *mut var_lookup;
        (*lookup).name = names[idx as usize];
        (*lookup).idx = idx;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i: ::core::ffi::c_uint = 0;
        let mut _hj_j: ::core::ffi::c_uint = 0;
        let mut _hj_k: ::core::ffi::c_uint = 0;
        let mut _hj_key = (*lookup).name as *const ::core::ffi::c_uchar;
        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i = _hj_j;
        _hj_k = strlen((*lookup).name) as ::core::ffi::c_uint;
        while _hj_k >= 12 as ::core::ffi::c_uint {
            _hj_i = _hj_i.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_j = _hj_j.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _ha_hashv = _ha_hashv.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(*_hj_key.offset(11 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_ha_hashv);
            _hj_i ^= _ha_hashv >> 13 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_ha_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
            _ha_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_ha_hashv);
            _hj_i ^= _ha_hashv >> 12 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_ha_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
            _ha_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_ha_hashv);
            _hj_i ^= _ha_hashv >> 3 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_ha_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
            _ha_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
            _hj_key = _hj_key.offset(12 as ::core::ffi::c_int as isize);
            _hj_k = _hj_k.wrapping_sub(12 as ::core::ffi::c_uint);
        }
        _ha_hashv = _ha_hashv.wrapping_add(strlen((*lookup).name) as ::core::ffi::c_uint);
        let mut current_block_54: u64;
        match _hj_k {
            11 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_54 = 442998152379714613;
            }
            10 => {
                current_block_54 = 442998152379714613;
            }
            9 => {
                current_block_54 = 14916313239819575723;
            }
            8 => {
                current_block_54 = 11772761646808455830;
            }
            7 => {
                current_block_54 = 5536219776913685678;
            }
            6 => {
                current_block_54 = 3967905580732798218;
            }
            5 => {
                current_block_54 = 765793381763078134;
            }
            4 => {
                current_block_54 = 3105948935974009916;
            }
            3 => {
                current_block_54 = 16488506295619998735;
            }
            2 => {
                current_block_54 = 2165477741955893522;
            }
            1 => {
                current_block_54 = 16420434121503669123;
            }
            _ => {
                current_block_54 = 1434579379687443766;
            }
        }
        if current_block_54 == 442998152379714613 {
            _ha_hashv = _ha_hashv.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_54 = 14916313239819575723;
        }
        if current_block_54 == 14916313239819575723 {
            _ha_hashv = _ha_hashv.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_54 = 11772761646808455830;
        }
        if current_block_54 == 11772761646808455830 {
            _hj_j = _hj_j.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_54 = 5536219776913685678;
        }
        if current_block_54 == 5536219776913685678 {
            _hj_j = _hj_j.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_54 = 3967905580732798218;
        }
        if current_block_54 == 3967905580732798218 {
            _hj_j = _hj_j.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_54 = 765793381763078134;
        }
        if current_block_54 == 765793381763078134 {
            _hj_j = _hj_j.wrapping_add(safe_tolower(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint);
            current_block_54 = 3105948935974009916;
        }
        if current_block_54 == 3105948935974009916 {
            _hj_i = _hj_i.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_54 = 16488506295619998735;
        }
        if current_block_54 == 16488506295619998735 {
            _hj_i = _hj_i.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_54 = 2165477741955893522;
        }
        if current_block_54 == 2165477741955893522 {
            _hj_i = _hj_i.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_54 = 16420434121503669123;
        }
        if current_block_54 == 16420434121503669123 {
            _hj_i = _hj_i.wrapping_add(safe_tolower(
                *_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint);
        }
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_ha_hashv);
        _hj_i ^= _ha_hashv >> 13 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_ha_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
        _ha_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_ha_hashv);
        _hj_i ^= _ha_hashv >> 12 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_ha_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
        _ha_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_ha_hashv);
        _hj_i ^= _ha_hashv >> 3 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_ha_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
        _ha_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
        let mut _ha_oomed = 0 as ::core::ffi::c_int;
        (*lookup).hh.hashv = _ha_hashv;
        (*lookup).hh.key = (*lookup).name as *const ::core::ffi::c_void;
        (*lookup).hh.keylen = strlen((*lookup).name) as ::core::ffi::c_uint;
        if lookup_map.is_null() {
            (*lookup).hh.next = NULL;
            (*lookup).hh.prev = NULL;
            (*lookup).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as size_t)
                as *mut UT_hash_table as *mut UT_hash_table;
            if (*lookup).hh.tbl.is_null() {
                _ha_oomed = 1 as ::core::ffi::c_int;
            } else {
                memset(
                    (*lookup).hh.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UT_hash_table>() as size_t,
                );
                (*(*lookup).hh.tbl).tail = &raw mut (*lookup).hh as *mut UT_hash_handle;
                (*(*lookup).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*lookup).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*lookup).hh.tbl).hho = (&raw mut (*lookup).hh as *mut ::core::ffi::c_char)
                    .offset_from(lookup as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as ptrdiff_t;
                (*(*lookup).hh.tbl).buckets = malloc(
                    (32 as size_t).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                ) as *mut UT_hash_bucket;
                (*(*lookup).hh.tbl).signature = HASH_SIGNATURE as uint32_t;
                if (*(*lookup).hh.tbl).buckets.is_null() {
                    _ha_oomed = 1 as ::core::ffi::c_int;
                    free((*lookup).hh.tbl as *mut ::core::ffi::c_void);
                } else {
                    memset(
                        (*(*lookup).hh.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as size_t)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                    );
                    if _ha_oomed != 0 {
                        free((*(*lookup).hh.tbl).buckets as *mut ::core::ffi::c_void);
                        free((*lookup).hh.tbl as *mut ::core::ffi::c_void);
                    }
                }
            }
            if _ha_oomed == 0 {
                lookup_map = lookup;
            }
        } else {
            (*lookup).hh.tbl = (*lookup_map).hh.tbl;
            (*lookup).hh.next = NULL;
            (*lookup).hh.prev = ((*(*lookup_map).hh.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(*lookup_map).hh.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*lookup_map).hh.tbl).tail).next = lookup as *mut ::core::ffi::c_void;
            (*(*lookup_map).hh.tbl).tail = &raw mut (*lookup).hh as *mut UT_hash_handle;
        }
        if _ha_oomed == 0 {
            let mut _ha_bkt: ::core::ffi::c_uint = 0;
            (*(*lookup_map).hh.tbl).num_items = (*(*lookup_map).hh.tbl).num_items.wrapping_add(1);
            _ha_bkt = _ha_hashv
                & (*(*lookup_map).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _ha_head: *mut UT_hash_bucket =
                (*(*lookup_map).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UT_hash_bucket;
            (*_ha_head).count += 1;
            (*lookup).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
            (*lookup).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
            if !(*_ha_head).hh_head.is_null() {
                (*(*_ha_head).hh_head).hh_prev = &raw mut (*lookup).hh as *mut UT_hash_handle;
            }
            (*_ha_head).hh_head = &raw mut (*lookup).hh as *mut UT_hash_handle;
            if (*_ha_head).count
                >= (*_ha_head)
                    .expand_mult
                    .wrapping_add(1 as ::core::ffi::c_uint)
                    .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                && (*(*lookup).hh.tbl).noexpand == 0
            {
                let mut _he_bkt: ::core::ffi::c_uint = 0;
                let mut _he_bkt_i: ::core::ffi::c_uint = 0;
                let mut _he_thh = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _he_hh_nxt = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _he_new_buckets = ::core::ptr::null_mut::<UT_hash_bucket>();
                let mut _he_newbkt = ::core::ptr::null_mut::<UT_hash_bucket>();
                _he_new_buckets = malloc(
                    (::core::mem::size_of::<UT_hash_bucket>() as size_t)
                        .wrapping_mul((*(*lookup).hh.tbl).num_buckets as size_t)
                        .wrapping_mul(2 as size_t),
                ) as *mut UT_hash_bucket;
                if _he_new_buckets.is_null() {
                    _ha_oomed = 1 as ::core::ffi::c_int;
                } else {
                    memset(
                        _he_new_buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (::core::mem::size_of::<UT_hash_bucket>() as size_t)
                            .wrapping_mul((*(*lookup).hh.tbl).num_buckets as size_t)
                            .wrapping_mul(2 as size_t),
                    );
                    (*(*lookup).hh.tbl).ideal_chain_maxlen = ((*(*lookup).hh.tbl).num_items
                        >> (*(*lookup).hh.tbl)
                            .log2_num_buckets
                            .wrapping_add(1 as ::core::ffi::c_uint))
                    .wrapping_add(
                        if (*(*lookup).hh.tbl).num_items
                            & (*(*lookup).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint)
                            != 0 as ::core::ffi::c_uint
                        {
                            1 as ::core::ffi::c_uint
                        } else {
                            0 as ::core::ffi::c_uint
                        },
                    );
                    (*(*lookup).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                    _he_bkt_i = 0 as ::core::ffi::c_uint;
                    while _he_bkt_i < (*(*lookup).hh.tbl).num_buckets {
                        _he_thh = (*(*(*lookup).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                            as *mut UT_hash_handle;
                        while !_he_thh.is_null() {
                            _he_hh_nxt = (*_he_thh).hh_next;
                            _he_bkt = (*_he_thh).hashv
                                & (*(*lookup).hh.tbl)
                                    .num_buckets
                                    .wrapping_mul(2 as ::core::ffi::c_uint)
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            _he_newbkt =
                                _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                            (*_he_newbkt).count += 1;
                            if (*_he_newbkt).count > (*(*lookup).hh.tbl).ideal_chain_maxlen {
                                (*(*lookup).hh.tbl).nonideal_items =
                                    (*(*lookup).hh.tbl).nonideal_items.wrapping_add(1);
                                if (*_he_newbkt).count
                                    > (*_he_newbkt)
                                        .expand_mult
                                        .wrapping_mul((*(*lookup).hh.tbl).ideal_chain_maxlen)
                                {
                                    (*_he_newbkt).expand_mult += 1;
                                }
                            }
                            (*_he_thh).hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                            (*_he_thh).hh_next = (*_he_newbkt).hh_head as *mut UT_hash_handle;
                            if !(*_he_newbkt).hh_head.is_null() {
                                (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                            }
                            (*_he_newbkt).hh_head = _he_thh as *mut UT_hash_handle;
                            _he_thh = _he_hh_nxt;
                        }
                        _he_bkt_i = _he_bkt_i.wrapping_add(1);
                    }
                    free((*(*lookup).hh.tbl).buckets as *mut ::core::ffi::c_void);
                    (*(*lookup).hh.tbl).num_buckets = (*(*lookup).hh.tbl)
                        .num_buckets
                        .wrapping_mul(2 as ::core::ffi::c_uint);
                    (*(*lookup).hh.tbl).log2_num_buckets =
                        (*(*lookup).hh.tbl).log2_num_buckets.wrapping_add(1);
                    (*(*lookup).hh.tbl).buckets = _he_new_buckets;
                    (*(*lookup).hh.tbl).ineff_expands = if (*(*lookup).hh.tbl).nonideal_items
                        > (*(*lookup).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                    {
                        (*(*lookup).hh.tbl)
                            .ineff_expands
                            .wrapping_add(1 as ::core::ffi::c_uint)
                    } else {
                        0 as ::core::ffi::c_uint
                    };
                    if (*(*lookup).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                        (*(*lookup).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                    }
                }
                if _ha_oomed != 0 {
                    let mut _hd_head: *mut UT_hash_bucket =
                        (*(*lookup_map).hh.tbl).buckets.offset(_ha_bkt as isize)
                            as *mut UT_hash_bucket;
                    (*_hd_head).count -= 1;
                    if (*_hd_head).hh_head == &raw mut (*lookup).hh {
                        (*_hd_head).hh_head = (*lookup).hh.hh_next as *mut UT_hash_handle;
                    }
                    if !(*lookup).hh.hh_prev.is_null() {
                        (*(*lookup).hh.hh_prev).hh_next = (*lookup).hh.hh_next;
                    }
                    if !(*lookup).hh.hh_next.is_null() {
                        (*(*lookup).hh.hh_next).hh_prev = (*lookup).hh.hh_prev;
                    }
                }
            }
            if _ha_oomed != 0 {
                let mut _hd_hh_item: *mut UT_hash_handle = &raw mut (*lookup).hh;
                let mut _hd_bkt: ::core::ffi::c_uint = 0;
                _hd_bkt = (*_hd_hh_item).hashv
                    & (*(*lookup_map).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let fresh0 = &mut (*(*(*lookup_map).hh.tbl).buckets.offset(_hd_bkt as isize)).count;
                *fresh0 = (*fresh0).wrapping_add(1);
                (*_hd_hh_item).hh_next = ::core::ptr::null_mut::<UT_hash_handle>();
                (*_hd_hh_item).hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                let mut _hd_hh_del: *const UT_hash_handle = &raw mut (*lookup).hh;
                if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
                    free((*(*lookup_map).hh.tbl).buckets as *mut ::core::ffi::c_void);
                    free((*lookup_map).hh.tbl as *mut ::core::ffi::c_void);
                    lookup_map = ::core::ptr::null_mut::<var_lookup>();
                } else {
                    let mut _hd_bkt_0: ::core::ffi::c_uint = 0;
                    if std::ptr::eq(_hd_hh_del, (*(*lookup_map).hh.tbl).tail) {
                        (*(*lookup_map).hh.tbl).tail = ((*_hd_hh_del).prev
                            as *mut ::core::ffi::c_char)
                            .offset((*(*lookup_map).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut UT_hash_handle
                            as *mut UT_hash_handle;
                    }
                    if !(*_hd_hh_del).prev.is_null() {
                        let fresh1 = &mut (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                            .offset((*(*lookup_map).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut UT_hash_handle))
                            .next;
                        *fresh1 = (*_hd_hh_del).next;
                    } else {
                        lookup_map = (*_hd_hh_del).next as *mut var_lookup as *mut var_lookup;
                    }
                    if !(*_hd_hh_del).next.is_null() {
                        let fresh2 = &mut (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                            .offset((*(*lookup_map).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut UT_hash_handle))
                            .prev;
                        *fresh2 = (*_hd_hh_del).prev;
                    }
                    _hd_bkt_0 = (*_hd_hh_del).hashv
                        & (*(*lookup_map).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    let mut _hd_head_0: *mut UT_hash_bucket =
                        (*(*lookup_map).hh.tbl).buckets.offset(_hd_bkt_0 as isize)
                            as *mut UT_hash_bucket;
                    (*_hd_head_0).count -= 1;
                    if std::ptr::eq((*_hd_head_0).hh_head, _hd_hh_del) {
                        (*_hd_head_0).hh_head = (*_hd_hh_del).hh_next as *mut UT_hash_handle;
                    }
                    if !(*_hd_hh_del).hh_prev.is_null() {
                        (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
                    }
                    if !(*_hd_hh_del).hh_next.is_null() {
                        (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
                    }
                    (*(*lookup_map).hh.tbl).num_items =
                        (*(*lookup_map).hh.tbl).num_items.wrapping_sub(1);
                }
                (*lookup).hh.tbl = ::core::ptr::null_mut::<UT_hash_table>();
                let mut _log_ctx = NULL;
                log_fatal(
                    c"src/varcache.c".as_ptr(),
                    101 as ::core::ffi::c_int,
                    c"init_var_lookup".as_ptr(),
                    false,
                    _log_ctx,
                    c"out of memory".as_ptr(),
                );
                exit(1 as ::core::ffi::c_int);
            }
        } else {
            (*lookup).hh.tbl = ::core::ptr::null_mut::<UT_hash_table>();
            let mut _log_ctx_0 = NULL;
            log_fatal(
                c"src/varcache.c".as_ptr(),
                101 as ::core::ffi::c_int,
                c"init_var_lookup".as_ptr(),
                false,
                _log_ctx_0,
                c"out of memory".as_ptr(),
            );
            exit(1 as ::core::ffi::c_int);
        }
        idx += 1;
    }
    init_var_lookup_from_config(cf_track_extra_parameters, &raw mut idx);
    num_var_cached = idx;
}
#[no_mangle]

pub unsafe extern "C" fn varcache_set(
    mut cache: *mut VarCache,
    mut key: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) -> bool {
    let mut lk = ::core::ptr::null::<var_lookup>();
    let mut pstr = ::core::ptr::null_mut::<PStr>();
    if vpool.is_null() {
        vpool = strpool_create(&raw const cx_libc_allocator);
        if vpool.is_null() {
            return false;
        }
    }
    let mut _uthash_hfstr_keylen = strlen(key) as ::core::ffi::c_uint;
    lk = ::core::ptr::null::<var_lookup>();
    if !lookup_map.is_null() {
        let mut _hf_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i: ::core::ffi::c_uint = 0;
        let mut _hj_j: ::core::ffi::c_uint = 0;
        let mut _hj_k: ::core::ffi::c_uint = 0;
        let mut _hj_key = key as *const ::core::ffi::c_uchar;
        _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i = _hj_j;
        _hj_k = _uthash_hfstr_keylen;
        while _hj_k >= 12 as ::core::ffi::c_uint {
            _hj_i = _hj_i.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_j = _hj_j.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hf_hashv = _hf_hashv.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (safe_tolower(
                            *_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        ) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (safe_tolower(*_hj_key.offset(11 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
            _hj_key = _hj_key.offset(12 as ::core::ffi::c_int as isize);
            _hj_k = _hj_k.wrapping_sub(12 as ::core::ffi::c_uint);
        }
        _hf_hashv = _hf_hashv.wrapping_add(_uthash_hfstr_keylen);
        let mut current_block_57: u64;
        match _hj_k {
            11 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (safe_tolower(
                        *_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    ) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_57 = 9437074751588942005;
            }
            10 => {
                current_block_57 = 9437074751588942005;
            }
            9 => {
                current_block_57 = 4405230302860763820;
            }
            8 => {
                current_block_57 = 9434220525754491920;
            }
            7 => {
                current_block_57 = 1270338533819009205;
            }
            6 => {
                current_block_57 = 3731882003230796202;
            }
            5 => {
                current_block_57 = 8650755990458127310;
            }
            4 => {
                current_block_57 = 4760653373201335839;
            }
            3 => {
                current_block_57 = 11469350245903324201;
            }
            2 => {
                current_block_57 = 11617363695825217706;
            }
            1 => {
                current_block_57 = 14328819089967601331;
            }
            _ => {
                current_block_57 = 13460095289871124136;
            }
        }
        if current_block_57 == 9437074751588942005 {
            _hf_hashv = _hf_hashv.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_57 = 4405230302860763820;
        }
        if current_block_57 == 4405230302860763820 {
            _hf_hashv = _hf_hashv.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_57 = 9434220525754491920;
        }
        if current_block_57 == 9434220525754491920 {
            _hj_j = _hj_j.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_57 = 1270338533819009205;
        }
        if current_block_57 == 1270338533819009205 {
            _hj_j = _hj_j.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_57 = 3731882003230796202;
        }
        if current_block_57 == 3731882003230796202 {
            _hj_j = _hj_j.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_57 = 8650755990458127310;
        }
        if current_block_57 == 8650755990458127310 {
            _hj_j = _hj_j.wrapping_add(safe_tolower(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint);
            current_block_57 = 4760653373201335839;
        }
        if current_block_57 == 4760653373201335839 {
            _hj_i = _hj_i.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_57 = 11469350245903324201;
        }
        if current_block_57 == 11469350245903324201 {
            _hj_i = _hj_i.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_57 = 11617363695825217706;
        }
        if current_block_57 == 11617363695825217706 {
            _hj_i = _hj_i.wrapping_add(
                (safe_tolower(
                    *_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                ) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_57 = 14328819089967601331;
        }
        if current_block_57 == 14328819089967601331 {
            _hj_i = _hj_i.wrapping_add(safe_tolower(
                *_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint);
        }
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
        lk = ::core::ptr::null::<var_lookup>();
        if !lookup_map.is_null() {
            let mut _hf_bkt: ::core::ffi::c_uint = 0;
            _hf_bkt = _hf_hashv
                & (*(*lookup_map).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            if !(*(*(*lookup_map).hh.tbl).buckets.offset(_hf_bkt as isize))
                .hh_head
                .is_null()
            {
                lk = ((*(*(*lookup_map).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                    as *mut ::core::ffi::c_char)
                    .offset(-(*(*lookup_map).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *const var_lookup
                    as *const var_lookup;
            } else {
                lk = ::core::ptr::null::<var_lookup>();
            }
            while !lk.is_null() {
                if (*lk).hh.hashv == _hf_hashv
                    && (*lk).hh.keylen == _uthash_hfstr_keylen
                    && strcasecmp((*lk).hh.key as *const ::core::ffi::c_char, key)
                        == 0 as ::core::ffi::c_int
                {
                    break;
                }
                if !(*lk).hh.hh_next.is_null() {
                    lk = ((*lk).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*lookup_map).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *const var_lookup
                        as *const var_lookup;
                } else {
                    lk = ::core::ptr::null::<var_lookup>();
                }
            }
        }
    }
    if lk.is_null() {
        return false;
    }
    strpool_decref(*(*cache).var_list.offset((*lk).idx as isize));
    let fresh7 = &mut *(*cache).var_list.offset((*lk).idx as isize);
    *fresh7 = ::core::ptr::null_mut::<PStr>();
    if value.is_null() {
        return false;
    }
    pstr = strpool_get(vpool, value, strlen(value) as ssize_t);
    if pstr.is_null() {
        return false;
    }
    let fresh8 = &mut *(*cache).var_list.offset((*lk).idx as isize);
    *fresh8 = pstr;
    true
}

unsafe extern "C" fn variable_is_guc_list_quote(mut key: *const ::core::ffi::c_char) -> bool {
    if strcasecmp(c"search_path".as_ptr(), key) == 0 as ::core::ffi::c_int {
        return true;
    }
    false
}

unsafe extern "C" fn apply_var(
    mut pkt: *mut PktBuf,
    mut key: *const ::core::ffi::c_char,
    mut cval: *const PStr,
    mut sval: *const PStr,
) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_char; 300] = [0; 300];
    let mut qbuf: [::core::ffi::c_char; 128] = [0; 128];
    let mut len: ::core::ffi::c_uint = 0;
    let mut tmp = ::core::ptr::null::<::core::ffi::c_char>();
    if cval.is_null() || sval.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if cval == sval {
        return 0 as ::core::ffi::c_int;
    }
    if strcasecmp(
        &raw const (*cval).str_0 as *const ::core::ffi::c_char,
        &raw const (*sval).str_0 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if variable_is_guc_list_quote(key) {
        if strcmp(
            &raw const (*cval).str_0 as *const ::core::ffi::c_char,
            b"\"\"\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            tmp = c"''".as_ptr();
        } else {
            tmp = &raw const (*cval).str_0 as *const ::core::ffi::c_char;
        }
    } else if pg_quote_literal(
        &raw mut qbuf as *mut ::core::ffi::c_char,
        &raw const (*cval).str_0 as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as ::core::ffi::c_int,
    ) {
        tmp = &raw mut qbuf as *mut ::core::ffi::c_char;
    } else {
        return 0 as ::core::ffi::c_int;
    }
    len = snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 300]>() as size_t,
        c"SET %s=%s;".as_ptr(),
        key,
        tmp,
    ) as ::core::ffi::c_uint;
    if (len as usize) < ::core::mem::size_of::<[::core::ffi::c_char; 300]>() {
        pktbuf_put_bytes(
            pkt,
            &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            len as ::core::ffi::c_int,
        );
    } else {
        let mut buf2 = malloc(
            (::core::mem::size_of::<::core::ffi::c_char>() as size_t).wrapping_mul(len as size_t),
        ) as *mut ::core::ffi::c_char;
        if buf2.is_null() {
            let mut _log_ctx = NULL;
            log_generic(
                LG_FATAL,
                _log_ctx,
                c"failed to allocate memory in apply_var".as_ptr(),
            );
            exit(1 as ::core::ffi::c_int);
        }
        snprintf(buf2, len as size_t, c"SET %s=%s;".as_ptr(), key, tmp);
        pktbuf_put_bytes(
            pkt,
            buf2 as *const ::core::ffi::c_void,
            len as ::core::ffi::c_int,
        );
        free(buf2 as *mut ::core::ffi::c_void);
    }
    1 as ::core::ffi::c_int
}
#[no_mangle]

pub unsafe extern "C" fn varcache_apply(
    mut server: *mut PgSocket,
    mut client: *mut PgSocket,
    mut changes_p: *mut bool,
) -> bool {
    let mut changes = 0 as ::core::ffi::c_int;
    let mut cval = ::core::ptr::null_mut::<PStr>();
    let mut sval = ::core::ptr::null_mut::<PStr>();
    let mut lk = ::core::ptr::null::<var_lookup>();
    let mut tmp = ::core::ptr::null::<var_lookup>();
    let mut sql_ofs: ::core::ffi::c_int = 0;
    let mut pkt = pktbuf_temp();
    pktbuf_start_packet(pkt as *mut PktBuf, PqMsg_Query);
    sql_ofs = (*pkt).write_pos;
    lk = lookup_map;
    tmp = (if !lookup_map.is_null() {
        (*lookup_map).hh.next
    } else {
        NULL
    }) as *const var_lookup as *const var_lookup;
    while !lk.is_null() {
        sval = get_value(&raw mut (*server).vars, lk);
        cval = get_value(&raw mut (*client).vars, lk);
        changes += apply_var(pkt as *mut PktBuf, (*lk).name, cval, sval);
        lk = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *const var_lookup
            as *const var_lookup;
    }
    *changes_p = changes > 0 as ::core::ffi::c_int;
    if changes == 0 {
        return true;
    }
    pktbuf_put_char(pkt as *mut PktBuf, 0 as ::core::ffi::c_char);
    pktbuf_finish_packet(pkt as *mut PktBuf);
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            server as *mut ::core::ffi::c_void,
            c"varcache_apply: %s".as_ptr(),
            (*pkt).buf.offset(sql_ofs as isize),
        );
    }
    pktbuf_send_immediate(pkt as *mut PktBuf, server)
}
#[no_mangle]

pub unsafe extern "C" fn varcache_set_canonical(
    mut server: *mut PgSocket,
    mut client: *mut PgSocket,
) {
    let mut server_val = ::core::ptr::null_mut::<PStr>();
    let mut client_val = ::core::ptr::null_mut::<PStr>();
    let mut lk = ::core::ptr::null::<var_lookup>();
    let mut tmp = ::core::ptr::null::<var_lookup>();
    lk = lookup_map;
    tmp = (if !lookup_map.is_null() {
        (*lookup_map).hh.next
    } else {
        NULL
    }) as *const var_lookup as *const var_lookup;
    while !lk.is_null() {
        server_val = *(*server).vars.var_list.offset((*lk).idx as isize);
        client_val = *(*client).vars.var_list.offset((*lk).idx as isize);
        if !client_val.is_null() && !server_val.is_null() && client_val != server_val {
            if cf_verbose > 0 as ::core::ffi::c_int
            {
                log_generic(
                    LG_DEBUG,
                    client as *mut ::core::ffi::c_void,
                    c"varcache_set_canonical: setting %s to its canonical version %s -> %s"
                        .as_ptr(),
                    (*lk).name,
                    &raw mut (*client_val).str_0 as *mut ::core::ffi::c_char,
                    &raw mut (*server_val).str_0 as *mut ::core::ffi::c_char,
                );
            }
            strpool_incref(server_val);
            strpool_decref(client_val);
            let fresh11 = &mut *(*client).vars.var_list.offset((*lk).idx as isize);
            *fresh11 = server_val;
        }
        lk = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *const var_lookup
            as *const var_lookup;
    }
}
#[no_mangle]

pub unsafe extern "C" fn varcache_apply_startup(mut pkt: *mut PktBuf, mut client: *mut PgSocket) {
    let mut lk = ::core::ptr::null::<var_lookup>();
    let mut tmp = ::core::ptr::null::<var_lookup>();
    lk = lookup_map;
    tmp = (if !lookup_map.is_null() {
        (*lookup_map).hh.next
    } else {
        NULL
    }) as *const var_lookup as *const var_lookup;
    while !lk.is_null() {
        let mut val = get_value(&raw mut (*client).vars, lk);
        if !val.is_null() {
            if cf_verbose > 0 as ::core::ffi::c_int
            {
                log_generic(
                    LG_DEBUG,
                    client as *mut ::core::ffi::c_void,
                    c"varcache_apply_startup: %s=%s".as_ptr(),
                    (*lk).name,
                    &raw mut (*val).str_0 as *mut ::core::ffi::c_char,
                );
            }
            pktbuf_put_string(pkt, (*lk).name);
            pktbuf_put_string(pkt, &raw mut (*val).str_0 as *mut ::core::ffi::c_char);
        }
        lk = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *const var_lookup
            as *const var_lookup;
    }
}
#[no_mangle]

pub unsafe extern "C" fn varcache_fill_unset(mut src: *mut VarCache, mut dst: *mut PgSocket) {
    let mut srcval = ::core::ptr::null_mut::<PStr>();
    let mut dstval = ::core::ptr::null_mut::<PStr>();
    let mut lk = ::core::ptr::null::<var_lookup>();
    let mut tmp = ::core::ptr::null::<var_lookup>();
    lk = lookup_map;
    tmp = (if !lookup_map.is_null() {
        (*lookup_map).hh.next
    } else {
        NULL
    }) as *const var_lookup as *const var_lookup;
    while !lk.is_null() {
        srcval = *(*src).var_list.offset((*lk).idx as isize);
        dstval = *(*dst).vars.var_list.offset((*lk).idx as isize);
        if dstval.is_null() {
            strpool_incref(srcval);
            let fresh9 = &mut *(*dst).vars.var_list.offset((*lk).idx as isize);
            *fresh9 = srcval;
        }
        lk = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *const var_lookup
            as *const var_lookup;
    }
}
#[no_mangle]

pub unsafe extern "C" fn varcache_clean(mut cache: *mut VarCache) {
    let mut i = 0 as ::core::ffi::c_int;
    while i < num_var_cached {
        strpool_decref(*(*cache).var_list.offset(i as isize));
        let fresh10 = &mut *(*cache).var_list.offset(i as isize);
        *fresh10 = ::core::ptr::null_mut::<PStr>();
        i += 1;
    }
}
#[no_mangle]

pub unsafe extern "C" fn varcache_add_params(mut pkt: *mut PktBuf, mut vars: *mut VarCache) {
    let mut val = ::core::ptr::null_mut::<PStr>();
    let mut lk = ::core::ptr::null::<var_lookup>();
    let mut tmp = ::core::ptr::null::<var_lookup>();
    lk = lookup_map;
    tmp = (if !lookup_map.is_null() {
        (*lookup_map).hh.next
    } else {
        NULL
    }) as *const var_lookup as *const var_lookup;
    while !lk.is_null() {
        val = *(*vars).var_list.offset((*lk).idx as isize);
        if !val.is_null() {
            pktbuf_write_generic(
                pkt,
                PqMsg_ParameterStatus,
                c"ss".as_ptr(),
                (*lk).name,
                &raw mut (*val).str_0 as *mut ::core::ffi::c_char,
            );
        }
        lk = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *const var_lookup
            as *const var_lookup;
    }
}
#[no_mangle]

pub unsafe extern "C" fn varcache_deinit() {
    strpool_free(vpool);
    vpool = ::core::ptr::null_mut::<StrPool>();
}

extern "C" {
    pub fn strpool_create(ca: *const CxMem) -> *mut StrPool;
    pub fn strpool_free(sp: *mut StrPool);
    pub fn strpool_get(
        sp: *mut StrPool,
        str: *const ::core::ffi::c_char,
        len: ssize_t,
    ) -> *mut PStr;
    pub fn strpool_incref(str: *mut PStr);
    pub fn strpool_decref(str: *mut PStr);
}
