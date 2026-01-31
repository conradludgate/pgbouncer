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
    use super::dnslookup_h::DNSToken;
    use super::in6_h::sockaddr_in6;
    use crate::types::sockaddr_in;
    use super::pktbuf_h::PktBuf;
    use super::sbuf_h::SBuf;
    use crate::types::sockaddr;
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
    extern "C" {

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
    use super::event_struct_h::event;
    use super::iobuf_h::IOBuf;
    use super::pktbuf_h::PktBuf;
    use crate::types::tls;
    use crate::types::size_t;
    use crate::types::ssize_t;
    use crate::types::uint8_t;
    use crate::types::MBuf;
    extern "C" {

        pub fn sbuf_prepare_skip(sbuf: *mut SBuf, amount: ::core::ffi::c_uint);

        pub fn sbuf_prepare_skip_then_send_leftover(
            sbuf: *mut SBuf,
            dst: *mut SBuf,
            skip_amount: ::core::ffi::c_uint,
            total_amount: ::core::ffi::c_uint,
        );

        pub fn sbuf_queue_packet(sbuf: *mut SBuf, dst: *mut SBuf, pkt: *mut PktBuf) -> bool;
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
    use crate::types::uint32_t;
    use crate::types::uint8_t;
    extern "C" {

        pub fn pktbuf_static(buf: *mut PktBuf, data: *mut uint8_t, len: ::core::ffi::c_int);

        pub fn pktbuf_temp() -> *mut PktBuf;

        pub fn pktbuf_send_immediate(buf: *mut PktBuf, sk: *mut PgSocket) -> bool;

        pub fn pktbuf_put_char(buf: *mut PktBuf, val: ::core::ffi::c_char);

        pub fn pktbuf_put_uint32(buf: *mut PktBuf, val: uint32_t);

        pub fn pktbuf_put_string(buf: *mut PktBuf, str: *const ::core::ffi::c_char);

        pub fn pktbuf_put_bytes(
            buf: *mut PktBuf,
            data: *const ::core::ffi::c_void,
            len: ::core::ffi::c_int,
        );

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

pub mod objects_h {
    use super::bouncer_h::{PgSocket, ResponseAction};
    use crate::types::PgPreparedStatement;
    extern "C" {

        pub type Slab;

        pub static mut server_prepared_statement_cache: *mut Slab;

        pub static mut prepared_statements: *mut PgPreparedStatement;

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

        pub fn add_outstanding_request(
            client: *mut PgSocket,
            type_0: ::core::ffi::c_char,
            action: ResponseAction,
        ) -> bool;
    }
}

pub mod messages_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct PgParsePacket {
        pub len: ::core::ffi::c_uint,
        pub name: *const ::core::ffi::c_char,
        pub query_and_parameters_len: size_t,
        pub query_and_parameters: *const ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct PgBindPacket {
        pub len: ::core::ffi::c_uint,
        pub portal: *const ::core::ffi::c_char,
        pub name: *const ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct PgDescribePacket {
        pub type_0: ::core::ffi::c_char,
        pub name: *const ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct PgClosePacket {
        pub type_0: ::core::ffi::c_char,
        pub name: *const ::core::ffi::c_char,
    }
    use super::bouncer_h::PgSocket;
    use crate::types::size_t;
    use crate::types::PktHdr;
    extern "C" {

        pub fn unmarshall_parse_packet(
            client: *mut PgSocket,
            pkt: *mut PktHdr,
            parse_packet_p: *mut PgParsePacket,
        ) -> bool;

        pub fn unmarshall_bind_packet(
            client: *mut PgSocket,
            pkt: *mut PktHdr,
            bind_packet_p: *mut PgBindPacket,
        ) -> bool;

        pub fn unmarshall_describe_packet(
            client: *mut PgSocket,
            pkt: *mut PktHdr,
            describe_packet_p: *mut PgDescribePacket,
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

pub mod slab_h {
    use super::objects_h::Slab;
    extern "C" {

        pub fn slab_alloc(slab: *mut Slab) -> *mut ::core::ffi::c_void;

        pub fn slab_free(slab: *mut Slab, obj: *mut ::core::ffi::c_void);
    }
}

pub mod protocol_h {

    pub const PqMsg_Bind: ::core::ffi::c_int = 'B' as i32;

    pub const PqMsg_Close: ::core::ffi::c_int = 'C' as i32;

    pub const PqMsg_Describe: ::core::ffi::c_int = 'D' as i32;

    pub const PqMsg_Parse: ::core::ffi::c_int = 'P' as i32;

    pub const PqMsg_CloseComplete: ::core::ffi::c_int = '3' as i32;
}
use self::_malloc_h::{free, malloc};
use self::_stdio_h::snprintf;
use crate::types::{memcmp, memcpy, memset, strlen};
pub use self::bouncer_h::{
    cf_max_prepared_statements, sockaddr_ucreds, C2RustUnnamed_9, CallbackState, LoadBalanceHosts,
    OutstandingRequest, PacketCallbackFlag, PgAddr, PgCredentials, PgDatabase, PgGlobalUser,
    PgPool, PgSocket, PgStats, ReplicationType, ResponseAction, ScramState, SocketState,
    CB_HANDLE_COMPLETE_PACKET, CB_NONE, CB_WANT_COMPLETE_PACKET, CL_ACTIVE, CL_ACTIVE_CANCEL,
    CL_FREE, CL_JUSTFREE, CL_LOGIN, CL_WAITING, CL_WAITING_CANCEL, CL_WAITING_LOGIN,
    LOAD_BALANCE_HOSTS_DISABLE, LOAD_BALANCE_HOSTS_ROUND_ROBIN, RA_FAKE, RA_FORWARD, RA_SKIP,
    REPLICATION_LOGICAL, REPLICATION_NONE, REPLICATION_PHYSICAL, SV_ACTIVE, SV_ACTIVE_CANCEL,
    SV_BEING_CANCELED, SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN, SV_TESTED, SV_USED,
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
pub use crate::types::NULL;
pub use crate::types::{
    __darwin_ptrdiff_t, __darwin_size_t, __darwin_ssize_t, __darwin_time_t, __int32_t, __uint16_t,
    __uint32_t, __uint8_t,
};
pub use crate::types::{aatree_cmp_f, aatree_walker_f, AANode, AATree};
pub use crate::types::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};

pub use self::event_struct_h::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
pub use self::in6_h::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use crate::types::{in_addr, sockaddr_in};
pub use self::iobuf_h::{iobuf, IOBuf};
pub use crate::types::{
    cf_verbose, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE, LG_STATS,
    LG_WARNING,
};
pub use self::messages_h::{
    unmarshall_bind_packet, unmarshall_describe_packet, unmarshall_parse_packet, PgBindPacket,
    PgClosePacket, PgDescribePacket, PgParsePacket,
};
use self::objects_h::{
    add_outstanding_request, disconnect_client, disconnect_server, prepared_statements,
    server_prepared_statement_cache,
};
pub use self::pktbuf_h::{
    pktbuf_put_bytes, pktbuf_put_char, pktbuf_put_string, pktbuf_put_uint32, pktbuf_send_immediate,
    pktbuf_static, pktbuf_temp, pktbuf_write_generic, PktBuf,
};
pub use self::protocol_h::{
    PqMsg_Bind, PqMsg_Close, PqMsg_CloseComplete, PqMsg_Describe, PqMsg_Parse,
};
pub use self::sbuf_h::{
    sbuf_cb_t, sbuf_prepare_skip, sbuf_prepare_skip_then_send_leftover, sbuf_queue_packet, SBuf,
    SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK, SBUF_EV_FLUSH,
    SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY,
};
use self::slab_h::{slab_alloc, slab_free};
pub use crate::types::sockaddr;
pub use crate::types::{__darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t, __DARWIN_NULL};
pub use crate::types::usec_t;
pub use crate::types::MBuf;
pub use crate::types::PktHdr;
pub use crate::types::{false_0, true_0};
pub use crate::types::{list_empty, list_last, List};
pub use crate::types::{statlist_count, statlist_last, StatList};
pub use crate::types::{PStr, StrPool};
pub use crate::types::{PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement};

pub use crate::types::VarCache;
pub use crate::types::{UT_hash_bucket, UT_hash_handle, UT_hash_table};
pub use crate::types::{
    HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2,
    HASH_SIGNATURE,
};

static mut next_unique_query_id: uint64_t = 0;

static mut uthash_alloc_failed: bool = false;

unsafe extern "C" fn create_prepared_statement(
    mut pkt: *mut PgParsePacket,
) -> *mut PgPreparedStatement {
    let mut ps = malloc(
        (::core::mem::size_of::<PgPreparedStatement>() as size_t)
            .wrapping_add((*pkt).query_and_parameters_len),
    ) as *mut PgPreparedStatement;
    if ps.is_null() {
        return ::core::ptr::null_mut::<PgPreparedStatement>();
    }
    next_unique_query_id = next_unique_query_id.wrapping_add(1 as uint64_t);
    (*ps).query_id = next_unique_query_id;
    (*ps).use_count = 0;
    (*ps).query_and_parameters_len = (*pkt).query_and_parameters_len;
    memcpy(
        &raw mut (*ps).query_and_parameters as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (*pkt).query_and_parameters as *const ::core::ffi::c_void,
        (*pkt).query_and_parameters_len,
    );
    ps
}

unsafe extern "C" fn create_client_prepared_statement(
    mut name: *const ::core::ffi::c_char,
    mut ps: *mut PgPreparedStatement,
) -> *mut PgClientPreparedStatement {
    let mut name_len: size_t = strlen(name).wrapping_add(1 as size_t);
    let mut client_ps = malloc(
        (::core::mem::size_of::<PgClientPreparedStatement>() as size_t).wrapping_add(name_len),
    ) as *mut PgClientPreparedStatement;
    if client_ps.is_null() {
        return ::core::ptr::null_mut::<PgClientPreparedStatement>();
    }
    memcpy(
        &raw mut (*client_ps).stmt_name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        name as *const ::core::ffi::c_void,
        name_len,
    );
    (*client_ps).ps = ps;
    (*ps).use_count += 1 as uint32_t;
    client_ps
}

unsafe extern "C" fn create_server_prepared_statement(
    mut ps: *mut PgPreparedStatement,
) -> *mut PgServerPreparedStatement {
    let mut server_ps =
        slab_alloc(server_prepared_statement_cache) as *mut PgServerPreparedStatement;
    if server_ps.is_null() {
        return ::core::ptr::null_mut::<PgServerPreparedStatement>();
    }
    (*server_ps).ps = ps;
    (*server_ps).query_id = (*ps).query_id;
    (*ps).use_count += 1 as uint32_t;
    server_ps
}

unsafe extern "C" fn get_prepared_statement(
    mut pkt: *mut PgParsePacket,
    mut found: *mut bool,
) -> *mut PgPreparedStatement {
    let mut ps = ::core::ptr::null_mut::<PgPreparedStatement>();
    ps = ::core::ptr::null_mut::<PgPreparedStatement>();
    if !prepared_statements.is_null() {
        let mut _hf_hashv: ::core::ffi::c_uint = 0;
        let mut _hb_keylen = (*pkt).query_and_parameters_len as ::core::ffi::c_uint;
        let mut _hb_key = (*pkt).query_and_parameters as *const ::core::ffi::c_uchar;
        _hf_hashv = 0 as ::core::ffi::c_uint;
        loop {
            let fresh18 = _hb_keylen;
            _hb_keylen = _hb_keylen.wrapping_sub(1);
            if fresh18 == 0 as ::core::ffi::c_uint {
                break;
            }
            let fresh19 = _hb_key;
            _hb_key = _hb_key.offset(1);
            _hf_hashv = (_hf_hashv << 5 as ::core::ffi::c_int)
                .wrapping_add(_hf_hashv)
                .wrapping_add(*fresh19 as ::core::ffi::c_uint);
        }
        ps = ::core::ptr::null_mut::<PgPreparedStatement>();
        if !prepared_statements.is_null() {
            let mut _hf_bkt: ::core::ffi::c_uint = 0;
            _hf_bkt = _hf_hashv
                & (*(*prepared_statements).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            if !(*(*(*prepared_statements).hh.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                ps = ((*(*(*prepared_statements).hh.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*prepared_statements).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut PgPreparedStatement
                    as *mut PgPreparedStatement;
            } else {
                ps = ::core::ptr::null_mut::<PgPreparedStatement>();
            }
            while !ps.is_null() {
                if (*ps).hh.hashv == _hf_hashv
                    && (*ps).hh.keylen as size_t == (*pkt).query_and_parameters_len
                    && memcmp(
                        (*ps).hh.key,
                        (*pkt).query_and_parameters as *const ::core::ffi::c_void,
                        (*pkt).query_and_parameters_len,
                    ) == 0 as ::core::ffi::c_int
                {
                    break;
                }
                if !(*ps).hh.hh_next.is_null() {
                    ps = ((*ps).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*prepared_statements).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut PgPreparedStatement
                        as *mut PgPreparedStatement;
                } else {
                    ps = ::core::ptr::null_mut::<PgPreparedStatement>();
                }
            }
        }
    }
    if !ps.is_null() {
        *found = true;
        return ps;
    }
    ps = create_prepared_statement(pkt);
    if ps.is_null() {
        return ::core::ptr::null_mut::<PgPreparedStatement>();
    }
    let mut _ha_hashv: ::core::ffi::c_uint = 0;
    let mut _hb_keylen_0 = (*ps).query_and_parameters_len as ::core::ffi::c_uint;
    let mut _hb_key_0 = &raw mut (*ps).query_and_parameters as *const ::core::ffi::c_uchar;
    _ha_hashv = 0 as ::core::ffi::c_uint;
    loop {
        let fresh20 = _hb_keylen_0;
        _hb_keylen_0 = _hb_keylen_0.wrapping_sub(1);
        if fresh20 == 0 as ::core::ffi::c_uint {
            break;
        }
        let fresh21 = _hb_key_0;
        _hb_key_0 = _hb_key_0.offset(1);
        _ha_hashv = (_ha_hashv << 5 as ::core::ffi::c_int)
            .wrapping_add(_ha_hashv)
            .wrapping_add(*fresh21 as ::core::ffi::c_uint);
    }
    let mut _ha_oomed = 0 as ::core::ffi::c_int;
    (*ps).hh.hashv = _ha_hashv;
    (*ps).hh.key = &raw mut (*ps).query_and_parameters as *const ::core::ffi::c_void;
    (*ps).hh.keylen = (*ps).query_and_parameters_len as ::core::ffi::c_uint;
    if prepared_statements.is_null() {
        (*ps).hh.next = NULL;
        (*ps).hh.prev = NULL;
        (*ps).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as size_t)
            as *mut UT_hash_table as *mut UT_hash_table;
        if (*ps).hh.tbl.is_null() {
            _ha_oomed = 1 as ::core::ffi::c_int;
        } else {
            memset(
                (*ps).hh.tbl as *mut ::core::ffi::c_void,
                '\0' as i32,
                ::core::mem::size_of::<UT_hash_table>() as size_t,
            );
            (*(*ps).hh.tbl).tail = &raw mut (*ps).hh as *mut UT_hash_handle;
            (*(*ps).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
            (*(*ps).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
            (*(*ps).hh.tbl).hho = (&raw mut (*ps).hh as *mut ::core::ffi::c_char)
                .offset_from(ps as *mut ::core::ffi::c_char)
                as ::core::ffi::c_long as ptrdiff_t;
            (*(*ps).hh.tbl).buckets = malloc(
                (32 as size_t).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
            ) as *mut UT_hash_bucket;
            (*(*ps).hh.tbl).signature = HASH_SIGNATURE as uint32_t;
            if (*(*ps).hh.tbl).buckets.is_null() {
                _ha_oomed = 1 as ::core::ffi::c_int;
                free((*ps).hh.tbl as *mut ::core::ffi::c_void);
            } else {
                memset(
                    (*(*ps).hh.tbl).buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (32 as size_t).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                );
                if _ha_oomed != 0 {
                    free((*(*ps).hh.tbl).buckets as *mut ::core::ffi::c_void);
                    free((*ps).hh.tbl as *mut ::core::ffi::c_void);
                }
            }
        }
        if _ha_oomed == 0 {
            prepared_statements = ps as *mut PgPreparedStatement;
        }
    } else {
        (*ps).hh.tbl = (*prepared_statements).hh.tbl;
        (*ps).hh.next = NULL;
        (*ps).hh.prev = ((*(*prepared_statements).hh.tbl).tail as *mut ::core::ffi::c_char)
            .offset(-(*(*prepared_statements).hh.tbl).hho)
            as *mut ::core::ffi::c_void;
        (*(*(*prepared_statements).hh.tbl).tail).next = ps as *mut ::core::ffi::c_void;
        (*(*prepared_statements).hh.tbl).tail = &raw mut (*ps).hh as *mut UT_hash_handle;
    }
    if _ha_oomed == 0 {
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*prepared_statements).hh.tbl).num_items =
            (*(*prepared_statements).hh.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*prepared_statements).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UT_hash_bucket = (*(*prepared_statements).hh.tbl)
            .buckets
            .offset(_ha_bkt as isize)
            as *mut UT_hash_bucket;
        (*_ha_head).count += 1;
        (*ps).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
        (*ps).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*ps).hh as *mut UT_hash_handle;
        }
        (*_ha_head).hh_head = &raw mut (*ps).hh as *mut UT_hash_handle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*ps).hh.tbl).noexpand == 0
        {
            let mut _he_bkt: ::core::ffi::c_uint = 0;
            let mut _he_bkt_i: ::core::ffi::c_uint = 0;
            let mut _he_thh = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _he_hh_nxt = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _he_new_buckets = ::core::ptr::null_mut::<UT_hash_bucket>();
            let mut _he_newbkt = ::core::ptr::null_mut::<UT_hash_bucket>();
            _he_new_buckets = malloc(
                (::core::mem::size_of::<UT_hash_bucket>() as size_t)
                    .wrapping_mul((*(*ps).hh.tbl).num_buckets as size_t)
                    .wrapping_mul(2 as size_t),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                _ha_oomed = 1 as ::core::ffi::c_int;
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (::core::mem::size_of::<UT_hash_bucket>() as size_t)
                        .wrapping_mul((*(*ps).hh.tbl).num_buckets as size_t)
                        .wrapping_mul(2 as size_t),
                );
                (*(*ps).hh.tbl).ideal_chain_maxlen = ((*(*ps).hh.tbl).num_items
                    >> (*(*ps).hh.tbl)
                        .log2_num_buckets
                        .wrapping_add(1 as ::core::ffi::c_uint))
                .wrapping_add(
                    if (*(*ps).hh.tbl).num_items
                        & (*(*ps).hh.tbl)
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
                (*(*ps).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                _he_bkt_i = 0 as ::core::ffi::c_uint;
                while _he_bkt_i < (*(*ps).hh.tbl).num_buckets {
                    _he_thh = (*(*(*ps).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                        as *mut UT_hash_handle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*ps).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                        (*_he_newbkt).count += 1;
                        if (*_he_newbkt).count > (*(*ps).hh.tbl).ideal_chain_maxlen {
                            (*(*ps).hh.tbl).nonideal_items =
                                (*(*ps).hh.tbl).nonideal_items.wrapping_add(1);
                            if (*_he_newbkt).count
                                > (*_he_newbkt)
                                    .expand_mult
                                    .wrapping_mul((*(*ps).hh.tbl).ideal_chain_maxlen)
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
                free((*(*ps).hh.tbl).buckets as *mut ::core::ffi::c_void);
                (*(*ps).hh.tbl).num_buckets = (*(*ps).hh.tbl)
                    .num_buckets
                    .wrapping_mul(2 as ::core::ffi::c_uint);
                (*(*ps).hh.tbl).log2_num_buckets = (*(*ps).hh.tbl).log2_num_buckets.wrapping_add(1);
                (*(*ps).hh.tbl).buckets = _he_new_buckets;
                (*(*ps).hh.tbl).ineff_expands = if (*(*ps).hh.tbl).nonideal_items
                    > (*(*ps).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                {
                    (*(*ps).hh.tbl)
                        .ineff_expands
                        .wrapping_add(1 as ::core::ffi::c_uint)
                } else {
                    0 as ::core::ffi::c_uint
                };
                if (*(*ps).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                    (*(*ps).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                }
            }
            if _ha_oomed != 0 {
                let mut _hd_head: *mut UT_hash_bucket = (*(*prepared_statements).hh.tbl)
                    .buckets
                    .offset(_ha_bkt as isize)
                    as *mut UT_hash_bucket;
                (*_hd_head).count -= 1;
                if (*_hd_head).hh_head == &raw mut (*ps).hh {
                    (*_hd_head).hh_head = (*ps).hh.hh_next as *mut UT_hash_handle;
                }
                if !(*ps).hh.hh_prev.is_null() {
                    (*(*ps).hh.hh_prev).hh_next = (*ps).hh.hh_next;
                }
                if !(*ps).hh.hh_next.is_null() {
                    (*(*ps).hh.hh_next).hh_prev = (*ps).hh.hh_prev;
                }
            }
        }
        if _ha_oomed != 0 {
            let mut _hd_hh_item: *mut UT_hash_handle = &raw mut (*ps).hh;
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            _hd_bkt = (*_hd_hh_item).hashv
                & (*(*prepared_statements).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let fresh22 = &mut (*(*(*prepared_statements).hh.tbl)
                .buckets
                .offset(_hd_bkt as isize))
            .count;
            *fresh22 = (*fresh22).wrapping_add(1);
            (*_hd_hh_item).hh_next = ::core::ptr::null_mut::<UT_hash_handle>();
            (*_hd_hh_item).hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _hd_hh_del: *const UT_hash_handle = &raw mut (*ps).hh;
            if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
                free((*(*prepared_statements).hh.tbl).buckets as *mut ::core::ffi::c_void);
                free((*prepared_statements).hh.tbl as *mut ::core::ffi::c_void);
                prepared_statements = ::core::ptr::null_mut::<PgPreparedStatement>();
            } else {
                let mut _hd_bkt_0: ::core::ffi::c_uint = 0;
                if std::ptr::eq(_hd_hh_del, (*(*prepared_statements).hh.tbl).tail) {
                    (*(*prepared_statements).hh.tbl).tail =
                        ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                            .offset((*(*prepared_statements).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut UT_hash_handle as *mut UT_hash_handle;
                }
                if !(*_hd_hh_del).prev.is_null() {
                    let fresh23 = &mut (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*prepared_statements).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut UT_hash_handle))
                        .next;
                    *fresh23 = (*_hd_hh_del).next;
                } else {
                    prepared_statements =
                        (*_hd_hh_del).next as *mut PgPreparedStatement as *mut PgPreparedStatement;
                }
                if !(*_hd_hh_del).next.is_null() {
                    let fresh24 = &mut (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                        .offset((*(*prepared_statements).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut UT_hash_handle))
                        .prev;
                    *fresh24 = (*_hd_hh_del).prev;
                }
                _hd_bkt_0 = (*_hd_hh_del).hashv
                    & (*(*prepared_statements).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let mut _hd_head_0: *mut UT_hash_bucket = (*(*prepared_statements).hh.tbl)
                    .buckets
                    .offset(_hd_bkt_0 as isize)
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
                (*(*prepared_statements).hh.tbl).num_items =
                    (*(*prepared_statements).hh.tbl).num_items.wrapping_sub(1);
            }
            (*ps).hh.tbl = ::core::ptr::null_mut::<UT_hash_table>();
            uthash_alloc_failed = true;
        }
    } else {
        (*ps).hh.tbl = ::core::ptr::null_mut::<UT_hash_table>();
        uthash_alloc_failed = true;
    }
    if uthash_alloc_failed {
        uthash_alloc_failed = false;
        free(ps as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<PgPreparedStatement>();
    }
    (*ps).stmt_name_len = snprintf(
        &raw mut (*ps).stmt_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 31]>() as size_t,
        c"PGBOUNCER_%llu".as_ptr(),
        (*ps).query_id,
    ) as uint8_t;
    *found = false;
    ps
}

unsafe extern "C" fn skip_possibly_completely_buffered_packet(
    mut client: *mut PgSocket,
    mut pkt: *mut PktHdr,
) {
    if (*client).packet_cb_state.flag() as ::core::ffi::c_int
        == CB_HANDLE_COMPLETE_PACKET as ::core::ffi::c_int
    {
        return;
    }
    sbuf_prepare_skip(&raw mut (*client).sbuf, (*pkt).len);
}
#[no_mangle]

pub unsafe extern "C" fn free_server_prepared_statement(
    mut server_ps: *mut PgServerPreparedStatement,
) {
    if server_ps.is_null() {
        return;
    }
    (*(*server_ps).ps).use_count = (*(*server_ps).ps).use_count.wrapping_sub(1);
    if (*(*server_ps).ps).use_count == 0 {
        let mut _hd_hh_del: *const UT_hash_handle = &raw mut (*(*server_ps).ps).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*prepared_statements).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*prepared_statements).hh.tbl as *mut ::core::ffi::c_void);
            prepared_statements = ::core::ptr::null_mut::<PgPreparedStatement>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if std::ptr::eq(_hd_hh_del, (*(*prepared_statements).hh.tbl).tail) {
                (*(*prepared_statements).hh.tbl).tail =
                    ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*prepared_statements).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut UT_hash_handle
                        as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let fresh9 = &mut (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*prepared_statements).hh.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh9 = (*_hd_hh_del).next;
            } else {
                prepared_statements =
                    (*_hd_hh_del).next as *mut PgPreparedStatement as *mut PgPreparedStatement;
            }
            if !(*_hd_hh_del).next.is_null() {
                let fresh10 = &mut (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*prepared_statements).hh.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut UT_hash_handle))
                    .prev;
                *fresh10 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*prepared_statements).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UT_hash_bucket = (*(*prepared_statements).hh.tbl)
                .buckets
                .offset(_hd_bkt as isize)
                as *mut UT_hash_bucket;
            (*_hd_head).count -= 1;
            if std::ptr::eq((*_hd_head).hh_head, _hd_hh_del) {
                (*_hd_head).hh_head = (*_hd_hh_del).hh_next as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).hh_prev.is_null() {
                (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
            }
            if !(*_hd_hh_del).hh_next.is_null() {
                (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
            }
            (*(*prepared_statements).hh.tbl).num_items =
                (*(*prepared_statements).hh.tbl).num_items.wrapping_sub(1);
        }
        free((*server_ps).ps as *mut ::core::ffi::c_void);
    }
    slab_free(
        server_prepared_statement_cache,
        server_ps as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]

pub unsafe extern "C" fn unregister_prepared_statement(
    mut server: *mut PgSocket,
    mut query_id: uint64_t,
) {
    let mut server_ps = ::core::ptr::null_mut::<PgServerPreparedStatement>();
    server_ps = ::core::ptr::null_mut::<PgServerPreparedStatement>();
    if !(*server).server_prepared_statements.is_null() {
        let mut _hf_hashv: ::core::ffi::c_uint = 0;
        let mut _hb_keylen = ::core::mem::size_of::<uint64_t>() as ::core::ffi::c_uint;
        let mut _hb_key = &raw mut query_id as *const ::core::ffi::c_uchar;
        _hf_hashv = 0 as ::core::ffi::c_uint;
        loop {
            let fresh37 = _hb_keylen;
            _hb_keylen = _hb_keylen.wrapping_sub(1);
            if fresh37 == 0 as ::core::ffi::c_uint {
                break;
            }
            let fresh38 = _hb_key;
            _hb_key = _hb_key.offset(1);
            _hf_hashv = (_hf_hashv << 5 as ::core::ffi::c_int)
                .wrapping_add(_hf_hashv)
                .wrapping_add(*fresh38 as ::core::ffi::c_uint);
        }
        server_ps = ::core::ptr::null_mut::<PgServerPreparedStatement>();
        if !(*server).server_prepared_statements.is_null() {
            let mut _hf_bkt: ::core::ffi::c_uint = 0;
            _hf_bkt = _hf_hashv
                & (*(*(*server).server_prepared_statements).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            if !(*(*(*(*server).server_prepared_statements).hh.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                server_ps = ((*(*(*(*server).server_prepared_statements).hh.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*server).server_prepared_statements).hh.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut PgServerPreparedStatement
                    as *mut PgServerPreparedStatement;
            } else {
                server_ps = ::core::ptr::null_mut::<PgServerPreparedStatement>();
            }
            while !server_ps.is_null() {
                if (*server_ps).hh.hashv == _hf_hashv
                    && (*server_ps).hh.keylen as usize == ::core::mem::size_of::<uint64_t>()
                    && memcmp(
                        (*server_ps).hh.key,
                        &raw mut query_id as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<uint64_t>() as size_t,
                    ) == 0 as ::core::ffi::c_int
                {
                    break;
                }
                if !(*server_ps).hh.hh_next.is_null() {
                    server_ps = ((*server_ps).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*server).server_prepared_statements).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut PgServerPreparedStatement
                        as *mut PgServerPreparedStatement;
                } else {
                    server_ps = ::core::ptr::null_mut::<PgServerPreparedStatement>();
                }
            }
        }
    }
    if !server_ps.is_null() {
        let mut _hd_hh_del: *const UT_hash_handle = &raw mut (*server_ps).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free(
                (*(*(*server).server_prepared_statements).hh.tbl).buckets
                    as *mut ::core::ffi::c_void,
            );
            free((*(*server).server_prepared_statements).hh.tbl as *mut ::core::ffi::c_void);
            (*server).server_prepared_statements =
                ::core::ptr::null_mut::<PgServerPreparedStatement>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if std::ptr::eq(
                _hd_hh_del,
                (*(*(*server).server_prepared_statements).hh.tbl).tail,
            ) {
                (*(*(*server).server_prepared_statements).hh.tbl).tail =
                    ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*(*server).server_prepared_statements).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut UT_hash_handle
                        as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let fresh39 = &mut (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*server).server_prepared_statements).hh.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh39 = (*_hd_hh_del).next;
            } else {
                (*server).server_prepared_statements = (*_hd_hh_del).next
                    as *mut PgServerPreparedStatement
                    as *mut PgServerPreparedStatement;
            }
            if !(*_hd_hh_del).next.is_null() {
                let fresh40 = &mut (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*(*server).server_prepared_statements).hh.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut UT_hash_handle))
                    .prev;
                *fresh40 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*(*server).server_prepared_statements).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UT_hash_bucket =
                (*(*(*server).server_prepared_statements).hh.tbl)
                    .buckets
                    .offset(_hd_bkt as isize) as *mut UT_hash_bucket;
            (*_hd_head).count -= 1;
            if std::ptr::eq((*_hd_head).hh_head, _hd_hh_del) {
                (*_hd_head).hh_head = (*_hd_hh_del).hh_next as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).hh_prev.is_null() {
                (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
            }
            if !(*_hd_hh_del).hh_next.is_null() {
                (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
            }
            (*(*(*server).server_prepared_statements).hh.tbl).num_items =
                (*(*(*server).server_prepared_statements).hh.tbl)
                    .num_items
                    .wrapping_sub(1);
        }
        free_server_prepared_statement(server_ps);
    }
}
#[no_mangle]

pub unsafe extern "C" fn add_prepared_statement(
    mut server: *mut PgSocket,
    mut server_ps: *mut PgServerPreparedStatement,
) -> bool {
    let mut _ha_hashv: ::core::ffi::c_uint = 0;
    let mut _hb_keylen = ::core::mem::size_of::<uint64_t>() as ::core::ffi::c_uint;
    let mut _hb_key = &raw mut (*server_ps).query_id as *const ::core::ffi::c_uchar;
    _ha_hashv = 0 as ::core::ffi::c_uint;
    loop {
        let fresh13 = _hb_keylen;
        _hb_keylen = _hb_keylen.wrapping_sub(1);
        if fresh13 == 0 as ::core::ffi::c_uint {
            break;
        }
        let fresh14 = _hb_key;
        _hb_key = _hb_key.offset(1);
        _ha_hashv = (_ha_hashv << 5 as ::core::ffi::c_int)
            .wrapping_add(_ha_hashv)
            .wrapping_add(*fresh14 as ::core::ffi::c_uint);
    }
    let mut _ha_oomed = 0 as ::core::ffi::c_int;
    (*server_ps).hh.hashv = _ha_hashv;
    (*server_ps).hh.key = &raw mut (*server_ps).query_id as *const ::core::ffi::c_void;
    (*server_ps).hh.keylen = ::core::mem::size_of::<uint64_t>() as ::core::ffi::c_uint;
    if (*server).server_prepared_statements.is_null() {
        (*server_ps).hh.next = NULL;
        (*server_ps).hh.prev = NULL;
        (*server_ps).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as size_t)
            as *mut UT_hash_table as *mut UT_hash_table;
        if (*server_ps).hh.tbl.is_null() {
            _ha_oomed = 1 as ::core::ffi::c_int;
        } else {
            memset(
                (*server_ps).hh.tbl as *mut ::core::ffi::c_void,
                '\0' as i32,
                ::core::mem::size_of::<UT_hash_table>() as size_t,
            );
            (*(*server_ps).hh.tbl).tail = &raw mut (*server_ps).hh as *mut UT_hash_handle;
            (*(*server_ps).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
            (*(*server_ps).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
            (*(*server_ps).hh.tbl).hho = (&raw mut (*server_ps).hh as *mut ::core::ffi::c_char)
                .offset_from(server_ps as *mut ::core::ffi::c_char)
                as ::core::ffi::c_long as ptrdiff_t;
            (*(*server_ps).hh.tbl).buckets = malloc(
                (32 as size_t).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
            ) as *mut UT_hash_bucket;
            (*(*server_ps).hh.tbl).signature = HASH_SIGNATURE as uint32_t;
            if (*(*server_ps).hh.tbl).buckets.is_null() {
                _ha_oomed = 1 as ::core::ffi::c_int;
                free((*server_ps).hh.tbl as *mut ::core::ffi::c_void);
            } else {
                memset(
                    (*(*server_ps).hh.tbl).buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (32 as size_t).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                );
                if _ha_oomed != 0 {
                    free((*(*server_ps).hh.tbl).buckets as *mut ::core::ffi::c_void);
                    free((*server_ps).hh.tbl as *mut ::core::ffi::c_void);
                }
            }
        }
        if _ha_oomed == 0 {
            (*server).server_prepared_statements = server_ps;
        }
    } else {
        (*server_ps).hh.tbl = (*(*server).server_prepared_statements).hh.tbl;
        (*server_ps).hh.next = NULL;
        (*server_ps).hh.prev = ((*(*(*server).server_prepared_statements).hh.tbl).tail
            as *mut ::core::ffi::c_char)
            .offset(-(*(*(*server).server_prepared_statements).hh.tbl).hho)
            as *mut ::core::ffi::c_void;
        (*(*(*(*server).server_prepared_statements).hh.tbl).tail).next =
            server_ps as *mut ::core::ffi::c_void;
        (*(*(*server).server_prepared_statements).hh.tbl).tail =
            &raw mut (*server_ps).hh as *mut UT_hash_handle;
    }
    if _ha_oomed == 0 {
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(*(*server).server_prepared_statements).hh.tbl).num_items =
            (*(*(*server).server_prepared_statements).hh.tbl)
                .num_items
                .wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(*(*server).server_prepared_statements).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UT_hash_bucket = (*(*(*server).server_prepared_statements).hh.tbl)
            .buckets
            .offset(_ha_bkt as isize)
            as *mut UT_hash_bucket;
        (*_ha_head).count += 1;
        (*server_ps).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
        (*server_ps).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*server_ps).hh as *mut UT_hash_handle;
        }
        (*_ha_head).hh_head = &raw mut (*server_ps).hh as *mut UT_hash_handle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*server_ps).hh.tbl).noexpand == 0
        {
            let mut _he_bkt: ::core::ffi::c_uint = 0;
            let mut _he_bkt_i: ::core::ffi::c_uint = 0;
            let mut _he_thh = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _he_hh_nxt = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _he_new_buckets = ::core::ptr::null_mut::<UT_hash_bucket>();
            let mut _he_newbkt = ::core::ptr::null_mut::<UT_hash_bucket>();
            _he_new_buckets = malloc(
                (::core::mem::size_of::<UT_hash_bucket>() as size_t)
                    .wrapping_mul((*(*server_ps).hh.tbl).num_buckets as size_t)
                    .wrapping_mul(2 as size_t),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                _ha_oomed = 1 as ::core::ffi::c_int;
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (::core::mem::size_of::<UT_hash_bucket>() as size_t)
                        .wrapping_mul((*(*server_ps).hh.tbl).num_buckets as size_t)
                        .wrapping_mul(2 as size_t),
                );
                (*(*server_ps).hh.tbl).ideal_chain_maxlen = ((*(*server_ps).hh.tbl).num_items
                    >> (*(*server_ps).hh.tbl)
                        .log2_num_buckets
                        .wrapping_add(1 as ::core::ffi::c_uint))
                .wrapping_add(
                    if (*(*server_ps).hh.tbl).num_items
                        & (*(*server_ps).hh.tbl)
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
                (*(*server_ps).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                _he_bkt_i = 0 as ::core::ffi::c_uint;
                while _he_bkt_i < (*(*server_ps).hh.tbl).num_buckets {
                    _he_thh = (*(*(*server_ps).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                        as *mut UT_hash_handle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*server_ps).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                        (*_he_newbkt).count += 1;
                        if (*_he_newbkt).count > (*(*server_ps).hh.tbl).ideal_chain_maxlen {
                            (*(*server_ps).hh.tbl).nonideal_items =
                                (*(*server_ps).hh.tbl).nonideal_items.wrapping_add(1);
                            if (*_he_newbkt).count
                                > (*_he_newbkt)
                                    .expand_mult
                                    .wrapping_mul((*(*server_ps).hh.tbl).ideal_chain_maxlen)
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
                free((*(*server_ps).hh.tbl).buckets as *mut ::core::ffi::c_void);
                (*(*server_ps).hh.tbl).num_buckets = (*(*server_ps).hh.tbl)
                    .num_buckets
                    .wrapping_mul(2 as ::core::ffi::c_uint);
                (*(*server_ps).hh.tbl).log2_num_buckets =
                    (*(*server_ps).hh.tbl).log2_num_buckets.wrapping_add(1);
                (*(*server_ps).hh.tbl).buckets = _he_new_buckets;
                (*(*server_ps).hh.tbl).ineff_expands = if (*(*server_ps).hh.tbl).nonideal_items
                    > (*(*server_ps).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                {
                    (*(*server_ps).hh.tbl)
                        .ineff_expands
                        .wrapping_add(1 as ::core::ffi::c_uint)
                } else {
                    0 as ::core::ffi::c_uint
                };
                if (*(*server_ps).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                    (*(*server_ps).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                }
            }
            if _ha_oomed != 0 {
                let mut _hd_head: *mut UT_hash_bucket =
                    (*(*(*server).server_prepared_statements).hh.tbl)
                        .buckets
                        .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
                (*_hd_head).count -= 1;
                if (*_hd_head).hh_head == &raw mut (*server_ps).hh {
                    (*_hd_head).hh_head = (*server_ps).hh.hh_next as *mut UT_hash_handle;
                }
                if !(*server_ps).hh.hh_prev.is_null() {
                    (*(*server_ps).hh.hh_prev).hh_next = (*server_ps).hh.hh_next;
                }
                if !(*server_ps).hh.hh_next.is_null() {
                    (*(*server_ps).hh.hh_next).hh_prev = (*server_ps).hh.hh_prev;
                }
            }
        }
        if _ha_oomed != 0 {
            let mut _hd_hh_item: *mut UT_hash_handle = &raw mut (*server_ps).hh;
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            _hd_bkt = (*_hd_hh_item).hashv
                & (*(*(*server).server_prepared_statements).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let fresh15 = &mut (*(*(*(*server).server_prepared_statements).hh.tbl)
                .buckets
                .offset(_hd_bkt as isize))
            .count;
            *fresh15 = (*fresh15).wrapping_add(1);
            (*_hd_hh_item).hh_next = ::core::ptr::null_mut::<UT_hash_handle>();
            (*_hd_hh_item).hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _hd_hh_del: *const UT_hash_handle = &raw mut (*server_ps).hh;
            if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
                free(
                    (*(*(*server).server_prepared_statements).hh.tbl).buckets
                        as *mut ::core::ffi::c_void,
                );
                free((*(*server).server_prepared_statements).hh.tbl as *mut ::core::ffi::c_void);
                (*server).server_prepared_statements =
                    ::core::ptr::null_mut::<PgServerPreparedStatement>();
            } else {
                let mut _hd_bkt_0: ::core::ffi::c_uint = 0;
                if std::ptr::eq(
                    _hd_hh_del,
                    (*(*(*server).server_prepared_statements).hh.tbl).tail,
                ) {
                    (*(*(*server).server_prepared_statements).hh.tbl).tail =
                        ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                            .offset((*(*(*server).server_prepared_statements).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut UT_hash_handle as *mut UT_hash_handle;
                }
                if !(*_hd_hh_del).prev.is_null() {
                    let fresh16 = &mut (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*(*server).server_prepared_statements).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut UT_hash_handle))
                        .next;
                    *fresh16 = (*_hd_hh_del).next;
                } else {
                    (*server).server_prepared_statements = (*_hd_hh_del).next
                        as *mut PgServerPreparedStatement
                        as *mut PgServerPreparedStatement;
                }
                if !(*_hd_hh_del).next.is_null() {
                    let fresh17 = &mut (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                        .offset((*(*(*server).server_prepared_statements).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut UT_hash_handle))
                        .prev;
                    *fresh17 = (*_hd_hh_del).prev;
                }
                _hd_bkt_0 = (*_hd_hh_del).hashv
                    & (*(*(*server).server_prepared_statements).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let mut _hd_head_0: *mut UT_hash_bucket =
                    (*(*(*server).server_prepared_statements).hh.tbl)
                        .buckets
                        .offset(_hd_bkt_0 as isize) as *mut UT_hash_bucket;
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
                (*(*(*server).server_prepared_statements).hh.tbl).num_items =
                    (*(*(*server).server_prepared_statements).hh.tbl)
                        .num_items
                        .wrapping_sub(1);
            }
            (*server_ps).hh.tbl = ::core::ptr::null_mut::<UT_hash_table>();
            uthash_alloc_failed = true;
        }
    } else {
        (*server_ps).hh.tbl = ::core::ptr::null_mut::<UT_hash_table>();
        uthash_alloc_failed = true;
    }
    if uthash_alloc_failed {
        uthash_alloc_failed = false;
        return false;
    }
    true
}

unsafe extern "C" fn register_prepared_statement(
    mut client: *mut PgSocket,
    mut server: *mut PgSocket,
    mut server_ps: *mut PgServerPreparedStatement,
) -> bool {
    let mut current = ::core::ptr::null_mut::<PgServerPreparedStatement>();
    let mut tmp = ::core::ptr::null_mut::<PgServerPreparedStatement>();
    let mut outstanding_request = ::core::ptr::null_mut::<OutstandingRequest>();
    let mut el = ::core::ptr::null_mut::<List>();
    let mut res: ::core::ffi::c_int = 0;
    el = statlist_last(&raw mut (*server).outstanding_requests);
    outstanding_request = (el as *mut ::core::ffi::c_char)
        
        as *mut OutstandingRequest;
    (*outstanding_request).server_ps_query_id = (*(*server_ps).ps).query_id;
    if !add_prepared_statement(server, server_ps) {
        return false;
    }
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(
            LG_NOISE,
            server as *mut ::core::ffi::c_void,
            c"prepared statement PGBOUNCER_%llu added to server cache, %d cached items".as_ptr(),
            (*(*server_ps).ps).query_id,
            if !(*server).server_prepared_statements.is_null() {
                (*(*(*server).server_prepared_statements).hh.tbl).num_items
            } else {
                0 as ::core::ffi::c_uint
            },
        );
    }
    current = (*server).server_prepared_statements as *mut PgServerPreparedStatement;
    tmp = (if !(*server).server_prepared_statements.is_null() {
        (*(*server).server_prepared_statements).hh.next
    } else {
        NULL
    }) as *mut PgServerPreparedStatement as *mut PgServerPreparedStatement;
    while !current.is_null() {
        if (if !(*server).server_prepared_statements.is_null() {
            (*(*(*server).server_prepared_statements).hh.tbl).num_items
        } else {
            0 as ::core::ffi::c_uint
        }) <= cf_max_prepared_statements as ::core::ffi::c_uint
        {
            break;
        }
        let mut _data: [uint8_t; 37] = [0; 37];
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
            ::core::mem::size_of::<[uint8_t; 37]>() as ::core::ffi::c_int,
        );
        pktbuf_write_generic(
            &raw mut _buf,
            PqMsg_Close,
            c"cs".as_ptr(),
            'S' as i32,
            &raw mut (*(*current).ps).stmt_name as *mut ::core::ffi::c_char,
        );
        res = sbuf_queue_packet(
            &raw mut (*client).sbuf,
            &raw mut (*server).sbuf,
            &raw mut _buf,
        ) as ::core::ffi::c_int;
        if res == 0 {
            return false;
        }
        if !add_outstanding_request(client, PqMsg_Close as ::core::ffi::c_char, RA_SKIP) {
            return false;
        }
        el = statlist_last(&raw mut (*server).outstanding_requests);
        outstanding_request = (el as *mut ::core::ffi::c_char)
            
            as *mut OutstandingRequest;
        (*outstanding_request).server_ps = current as *mut PgServerPreparedStatement;
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(
                LG_NOISE,
                server as *mut ::core::ffi::c_void,
                c"prepared statement '%s' deleted from server cache".as_ptr(),
                &raw mut (*(*current).ps).stmt_name as *mut ::core::ffi::c_char,
            );
        }
        let mut _hd_hh_del: *const UT_hash_handle = &raw mut (*current).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free(
                (*(*(*server).server_prepared_statements).hh.tbl).buckets
                    as *mut ::core::ffi::c_void,
            );
            free((*(*server).server_prepared_statements).hh.tbl as *mut ::core::ffi::c_void);
            (*server).server_prepared_statements =
                ::core::ptr::null_mut::<PgServerPreparedStatement>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if std::ptr::eq(
                _hd_hh_del,
                (*(*(*server).server_prepared_statements).hh.tbl).tail,
            ) {
                (*(*(*server).server_prepared_statements).hh.tbl).tail =
                    ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*(*server).server_prepared_statements).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut UT_hash_handle
                        as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let fresh11 = &mut (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*server).server_prepared_statements).hh.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh11 = (*_hd_hh_del).next;
            } else {
                (*server).server_prepared_statements = (*_hd_hh_del).next
                    as *mut PgServerPreparedStatement
                    as *mut PgServerPreparedStatement;
            }
            if !(*_hd_hh_del).next.is_null() {
                let fresh12 = &mut (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*(*server).server_prepared_statements).hh.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut UT_hash_handle))
                    .prev;
                *fresh12 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*(*server).server_prepared_statements).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UT_hash_bucket =
                (*(*(*server).server_prepared_statements).hh.tbl)
                    .buckets
                    .offset(_hd_bkt as isize) as *mut UT_hash_bucket;
            (*_hd_head).count -= 1;
            if std::ptr::eq((*_hd_head).hh_head, _hd_hh_del) {
                (*_hd_head).hh_head = (*_hd_hh_del).hh_next as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).hh_prev.is_null() {
                (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
            }
            if !(*_hd_hh_del).hh_next.is_null() {
                (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
            }
            (*(*(*server).server_prepared_statements).hh.tbl).num_items =
                (*(*(*server).server_prepared_statements).hh.tbl)
                    .num_items
                    .wrapping_sub(1);
        }
        current = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut PgServerPreparedStatement
            as *mut PgServerPreparedStatement;
    }
    true
}
#[no_mangle]

pub unsafe extern "C" fn handle_parse_command(
    mut client: *mut PgSocket,
    mut pkt: *mut PktHdr,
) -> bool {
    let mut current_block: u64;
    let mut server = (*client).link;
    let mut pp = PgParsePacket {
        len: 0,
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        query_and_parameters_len: 0,
        query_and_parameters: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut server_ps = ::core::ptr::null_mut::<PgServerPreparedStatement>();
    let mut client_ps = ::core::ptr::null_mut::<PgClientPreparedStatement>();
    let mut ps = ::core::ptr::null_mut::<PgPreparedStatement>();
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    let mut found = false;
    if !unmarshall_parse_packet(client, pkt, &raw mut pp) {
        return false;
    }
    let mut _uthash_hfstr_keylen = strlen(pp.name) as ::core::ffi::c_uint;
    client_ps = ::core::ptr::null_mut::<PgClientPreparedStatement>();
    if !(*client).client_prepared_statements.is_null() {
        let mut _hf_hashv: ::core::ffi::c_uint = 0;
        let mut _hb_keylen = _uthash_hfstr_keylen;
        let mut _hb_key = pp.name as *const ::core::ffi::c_uchar;
        _hf_hashv = 0 as ::core::ffi::c_uint;
        loop {
            let fresh0 = _hb_keylen;
            _hb_keylen = _hb_keylen.wrapping_sub(1);
            if fresh0 == 0 as ::core::ffi::c_uint {
                break;
            }
            let fresh1 = _hb_key;
            _hb_key = _hb_key.offset(1);
            _hf_hashv = (_hf_hashv << 5 as ::core::ffi::c_int)
                .wrapping_add(_hf_hashv)
                .wrapping_add(*fresh1 as ::core::ffi::c_uint);
        }
        client_ps = ::core::ptr::null_mut::<PgClientPreparedStatement>();
        if !(*client).client_prepared_statements.is_null() {
            let mut _hf_bkt: ::core::ffi::c_uint = 0;
            _hf_bkt = _hf_hashv
                & (*(*(*client).client_prepared_statements).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            if !(*(*(*(*client).client_prepared_statements).hh.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                client_ps = ((*(*(*(*client).client_prepared_statements).hh.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*client).client_prepared_statements).hh.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut PgClientPreparedStatement
                    as *mut PgClientPreparedStatement;
            } else {
                client_ps = ::core::ptr::null_mut::<PgClientPreparedStatement>();
            }
            while !client_ps.is_null() {
                if (*client_ps).hh.hashv == _hf_hashv
                    && (*client_ps).hh.keylen == _uthash_hfstr_keylen
                    && memcmp(
                        (*client_ps).hh.key,
                        pp.name as *const ::core::ffi::c_void,
                        _uthash_hfstr_keylen as size_t,
                    ) == 0 as ::core::ffi::c_int
                {
                    break;
                }
                if !(*client_ps).hh.hh_next.is_null() {
                    client_ps = ((*client_ps).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*client).client_prepared_statements).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut PgClientPreparedStatement
                        as *mut PgClientPreparedStatement;
                } else {
                    client_ps = ::core::ptr::null_mut::<PgClientPreparedStatement>();
                }
            }
        }
    }
    if !client_ps.is_null() {
        log_generic(
            LG_ERROR,
            client as *mut ::core::ffi::c_void,
            c"prepared statement '%s' was already prepared".as_ptr(),
            pp.name,
        );
        disconnect_client(
            client,
            true,
            c"prepared statement name is already in use".as_ptr(),
        );
        return false;
    }
    (*(*client).pool).stats.ps_client_parse_count = (*(*client).pool)
        .stats
        .ps_client_parse_count
        .wrapping_add(1);
    ps = get_prepared_statement(&raw mut pp, &raw mut found);
    if !ps.is_null() {
        client_ps = create_client_prepared_statement(pp.name, ps);
        if !client_ps.is_null() {
            let mut _uthash_hastr_keylen =
                strlen(&raw mut (*client_ps).stmt_name as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_uint;
            let mut _ha_hashv: ::core::ffi::c_uint = 0;
            let mut _hb_keylen_0 = _uthash_hastr_keylen;
            let mut _hb_key_0 = (&raw mut (*client_ps).stmt_name as *mut ::core::ffi::c_char)
                
                as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_uchar;
            _ha_hashv = 0 as ::core::ffi::c_uint;
            loop {
                let fresh2 = _hb_keylen_0;
                _hb_keylen_0 = _hb_keylen_0.wrapping_sub(1);
                if fresh2 == 0 as ::core::ffi::c_uint {
                    break;
                }
                let fresh3 = _hb_key_0;
                _hb_key_0 = _hb_key_0.offset(1);
                _ha_hashv = (_ha_hashv << 5 as ::core::ffi::c_int)
                    .wrapping_add(_ha_hashv)
                    .wrapping_add(*fresh3 as ::core::ffi::c_uint);
            }
            let mut _ha_oomed = 0 as ::core::ffi::c_int;
            (*client_ps).hh.hashv = _ha_hashv;
            (*client_ps).hh.key = (&raw mut (*client_ps).stmt_name as *mut ::core::ffi::c_char)
                
                as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_void;
            (*client_ps).hh.keylen = _uthash_hastr_keylen;
            if (*client).client_prepared_statements.is_null() {
                (*client_ps).hh.next = NULL;
                (*client_ps).hh.prev = NULL;
                (*client_ps).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as size_t)
                    as *mut UT_hash_table
                    as *mut UT_hash_table;
                if (*client_ps).hh.tbl.is_null() {
                    _ha_oomed = 1 as ::core::ffi::c_int;
                } else {
                    memset(
                        (*client_ps).hh.tbl as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        ::core::mem::size_of::<UT_hash_table>() as size_t,
                    );
                    (*(*client_ps).hh.tbl).tail = &raw mut (*client_ps).hh as *mut UT_hash_handle;
                    (*(*client_ps).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                    (*(*client_ps).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                    (*(*client_ps).hh.tbl).hho =
                        (&raw mut (*client_ps).hh as *mut ::core::ffi::c_char)
                            .offset_from(client_ps as *mut ::core::ffi::c_char)
                            as ::core::ffi::c_long as ptrdiff_t;
                    (*(*client_ps).hh.tbl).buckets = malloc(
                        (32 as size_t)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                    ) as *mut UT_hash_bucket;
                    (*(*client_ps).hh.tbl).signature = HASH_SIGNATURE as uint32_t;
                    if (*(*client_ps).hh.tbl).buckets.is_null() {
                        _ha_oomed = 1 as ::core::ffi::c_int;
                        free((*client_ps).hh.tbl as *mut ::core::ffi::c_void);
                    } else {
                        memset(
                            (*(*client_ps).hh.tbl).buckets as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            (32 as size_t)
                                .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as size_t),
                        );
                        if _ha_oomed != 0 {
                            free((*(*client_ps).hh.tbl).buckets as *mut ::core::ffi::c_void);
                            free((*client_ps).hh.tbl as *mut ::core::ffi::c_void);
                        }
                    }
                }
                if _ha_oomed == 0 {
                    (*client).client_prepared_statements = client_ps;
                }
            } else {
                (*client_ps).hh.tbl = (*(*client).client_prepared_statements).hh.tbl;
                (*client_ps).hh.next = NULL;
                (*client_ps).hh.prev = ((*(*(*client).client_prepared_statements).hh.tbl).tail
                    as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*client).client_prepared_statements).hh.tbl).hho)
                    as *mut ::core::ffi::c_void;
                (*(*(*(*client).client_prepared_statements).hh.tbl).tail).next =
                    client_ps as *mut ::core::ffi::c_void;
                (*(*(*client).client_prepared_statements).hh.tbl).tail =
                    &raw mut (*client_ps).hh as *mut UT_hash_handle;
            }
            if _ha_oomed == 0 {
                let mut _ha_bkt: ::core::ffi::c_uint = 0;
                (*(*(*client).client_prepared_statements).hh.tbl).num_items =
                    (*(*(*client).client_prepared_statements).hh.tbl)
                        .num_items
                        .wrapping_add(1);
                _ha_bkt = _ha_hashv
                    & (*(*(*client).client_prepared_statements).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let mut _ha_head: *mut UT_hash_bucket =
                    (*(*(*client).client_prepared_statements).hh.tbl)
                        .buckets
                        .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
                (*_ha_head).count += 1;
                (*client_ps).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
                (*client_ps).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                if !(*_ha_head).hh_head.is_null() {
                    (*(*_ha_head).hh_head).hh_prev =
                        &raw mut (*client_ps).hh as *mut UT_hash_handle;
                }
                (*_ha_head).hh_head = &raw mut (*client_ps).hh as *mut UT_hash_handle;
                if (*_ha_head).count
                    >= (*_ha_head)
                        .expand_mult
                        .wrapping_add(1 as ::core::ffi::c_uint)
                        .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                    && (*(*client_ps).hh.tbl).noexpand == 0
                {
                    let mut _he_bkt: ::core::ffi::c_uint = 0;
                    let mut _he_bkt_i: ::core::ffi::c_uint = 0;
                    let mut _he_thh = ::core::ptr::null_mut::<UT_hash_handle>();
                    let mut _he_hh_nxt = ::core::ptr::null_mut::<UT_hash_handle>();
                    let mut _he_new_buckets = ::core::ptr::null_mut::<UT_hash_bucket>();
                    let mut _he_newbkt = ::core::ptr::null_mut::<UT_hash_bucket>();
                    _he_new_buckets = malloc(
                        (::core::mem::size_of::<UT_hash_bucket>() as size_t)
                            .wrapping_mul((*(*client_ps).hh.tbl).num_buckets as size_t)
                            .wrapping_mul(2 as size_t),
                    ) as *mut UT_hash_bucket;
                    if _he_new_buckets.is_null() {
                        _ha_oomed = 1 as ::core::ffi::c_int;
                    } else {
                        memset(
                            _he_new_buckets as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            (::core::mem::size_of::<UT_hash_bucket>() as size_t)
                                .wrapping_mul((*(*client_ps).hh.tbl).num_buckets as size_t)
                                .wrapping_mul(2 as size_t),
                        );
                        (*(*client_ps).hh.tbl).ideal_chain_maxlen = ((*(*client_ps).hh.tbl)
                            .num_items
                            >> (*(*client_ps).hh.tbl)
                                .log2_num_buckets
                                .wrapping_add(1 as ::core::ffi::c_uint))
                        .wrapping_add(
                            if (*(*client_ps).hh.tbl).num_items
                                & (*(*client_ps).hh.tbl)
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
                        (*(*client_ps).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                        _he_bkt_i = 0 as ::core::ffi::c_uint;
                        while _he_bkt_i < (*(*client_ps).hh.tbl).num_buckets {
                            _he_thh = (*(*(*client_ps).hh.tbl).buckets.offset(_he_bkt_i as isize))
                                .hh_head
                                as *mut UT_hash_handle;
                            while !_he_thh.is_null() {
                                _he_hh_nxt = (*_he_thh).hh_next;
                                _he_bkt = (*_he_thh).hashv
                                    & (*(*client_ps).hh.tbl)
                                        .num_buckets
                                        .wrapping_mul(2 as ::core::ffi::c_uint)
                                        .wrapping_sub(1 as ::core::ffi::c_uint);
                                _he_newbkt =
                                    _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                                (*_he_newbkt).count += 1;
                                if (*_he_newbkt).count > (*(*client_ps).hh.tbl).ideal_chain_maxlen {
                                    (*(*client_ps).hh.tbl).nonideal_items =
                                        (*(*client_ps).hh.tbl).nonideal_items.wrapping_add(1);
                                    if (*_he_newbkt).count
                                        > (*_he_newbkt)
                                            .expand_mult
                                            .wrapping_mul((*(*client_ps).hh.tbl).ideal_chain_maxlen)
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
                        free((*(*client_ps).hh.tbl).buckets as *mut ::core::ffi::c_void);
                        (*(*client_ps).hh.tbl).num_buckets = (*(*client_ps).hh.tbl)
                            .num_buckets
                            .wrapping_mul(2 as ::core::ffi::c_uint);
                        (*(*client_ps).hh.tbl).log2_num_buckets =
                            (*(*client_ps).hh.tbl).log2_num_buckets.wrapping_add(1);
                        (*(*client_ps).hh.tbl).buckets = _he_new_buckets;
                        (*(*client_ps).hh.tbl).ineff_expands = if (*(*client_ps).hh.tbl)
                            .nonideal_items
                            > (*(*client_ps).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                        {
                            (*(*client_ps).hh.tbl)
                                .ineff_expands
                                .wrapping_add(1 as ::core::ffi::c_uint)
                        } else {
                            0 as ::core::ffi::c_uint
                        };
                        if (*(*client_ps).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                            (*(*client_ps).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                        }
                    }
                    if _ha_oomed != 0 {
                        let mut _hd_head: *mut UT_hash_bucket =
                            (*(*(*client).client_prepared_statements).hh.tbl)
                                .buckets
                                .offset(_ha_bkt as isize)
                                as *mut UT_hash_bucket;
                        (*_hd_head).count -= 1;
                        if (*_hd_head).hh_head == &raw mut (*client_ps).hh {
                            (*_hd_head).hh_head = (*client_ps).hh.hh_next as *mut UT_hash_handle;
                        }
                        if !(*client_ps).hh.hh_prev.is_null() {
                            (*(*client_ps).hh.hh_prev).hh_next = (*client_ps).hh.hh_next;
                        }
                        if !(*client_ps).hh.hh_next.is_null() {
                            (*(*client_ps).hh.hh_next).hh_prev = (*client_ps).hh.hh_prev;
                        }
                    }
                }
                if _ha_oomed != 0 {
                    let mut _hd_hh_item: *mut UT_hash_handle = &raw mut (*client_ps).hh;
                    let mut _hd_bkt: ::core::ffi::c_uint = 0;
                    _hd_bkt = (*_hd_hh_item).hashv
                        & (*(*(*client).client_prepared_statements).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    let fresh4 = &mut (*(*(*(*client).client_prepared_statements).hh.tbl)
                        .buckets
                        .offset(_hd_bkt as isize))
                    .count;
                    *fresh4 = (*fresh4).wrapping_add(1);
                    (*_hd_hh_item).hh_next = ::core::ptr::null_mut::<UT_hash_handle>();
                    (*_hd_hh_item).hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                    let mut _hd_hh_del: *const UT_hash_handle = &raw mut (*client_ps).hh;
                    if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
                        free(
                            (*(*(*client).client_prepared_statements).hh.tbl).buckets
                                as *mut ::core::ffi::c_void,
                        );
                        free(
                            (*(*client).client_prepared_statements).hh.tbl
                                as *mut ::core::ffi::c_void,
                        );
                        (*client).client_prepared_statements =
                            ::core::ptr::null_mut::<PgClientPreparedStatement>();
                    } else {
                        let mut _hd_bkt_0: ::core::ffi::c_uint = 0;
                        if std::ptr::eq(
                            _hd_hh_del,
                            (*(*(*client).client_prepared_statements).hh.tbl).tail,
                        ) {
                            (*(*(*client).client_prepared_statements).hh.tbl).tail =
                                ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                                    .offset((*(*(*client).client_prepared_statements).hh.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut UT_hash_handle
                                    as *mut UT_hash_handle;
                        }
                        if !(*_hd_hh_del).prev.is_null() {
                            let fresh5 = &mut (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                                .offset((*(*(*client).client_prepared_statements).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut UT_hash_handle))
                                .next;
                            *fresh5 = (*_hd_hh_del).next;
                        } else {
                            (*client).client_prepared_statements = (*_hd_hh_del).next
                                as *mut PgClientPreparedStatement
                                as *mut PgClientPreparedStatement;
                        }
                        if !(*_hd_hh_del).next.is_null() {
                            let fresh6 = &mut (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                                .offset((*(*(*client).client_prepared_statements).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut UT_hash_handle))
                                .prev;
                            *fresh6 = (*_hd_hh_del).prev;
                        }
                        _hd_bkt_0 = (*_hd_hh_del).hashv
                            & (*(*(*client).client_prepared_statements).hh.tbl)
                                .num_buckets
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        let mut _hd_head_0: *mut UT_hash_bucket =
                            (*(*(*client).client_prepared_statements).hh.tbl)
                                .buckets
                                .offset(_hd_bkt_0 as isize)
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
                        (*(*(*client).client_prepared_statements).hh.tbl).num_items =
                            (*(*(*client).client_prepared_statements).hh.tbl)
                                .num_items
                                .wrapping_sub(1);
                    }
                    (*client_ps).hh.tbl = ::core::ptr::null_mut::<UT_hash_table>();
                    uthash_alloc_failed = true;
                }
            } else {
                (*client_ps).hh.tbl = ::core::ptr::null_mut::<UT_hash_table>();
                uthash_alloc_failed = true;
            }
            if uthash_alloc_failed {
                uthash_alloc_failed = false;
            } else {
                if found {
                    server_ps = ::core::ptr::null_mut::<PgServerPreparedStatement>();
                    if !(*server).server_prepared_statements.is_null() {
                        let mut _hf_hashv_0: ::core::ffi::c_uint = 0;
                        let mut _hb_keylen_1 =
                            ::core::mem::size_of::<uint64_t>() as ::core::ffi::c_uint;
                        let mut _hb_key_1 = &raw mut (*ps).query_id as *const ::core::ffi::c_uchar;
                        _hf_hashv_0 = 0 as ::core::ffi::c_uint;
                        loop {
                            let fresh7 = _hb_keylen_1;
                            _hb_keylen_1 = _hb_keylen_1.wrapping_sub(1);
                            if fresh7 == 0 as ::core::ffi::c_uint {
                                break;
                            }
                            let fresh8 = _hb_key_1;
                            _hb_key_1 = _hb_key_1.offset(1);
                            _hf_hashv_0 = (_hf_hashv_0 << 5 as ::core::ffi::c_int)
                                .wrapping_add(_hf_hashv_0)
                                .wrapping_add(*fresh8 as ::core::ffi::c_uint);
                        }
                        server_ps = ::core::ptr::null_mut::<PgServerPreparedStatement>();
                        if !(*server).server_prepared_statements.is_null() {
                            let mut _hf_bkt_0: ::core::ffi::c_uint = 0;
                            _hf_bkt_0 = _hf_hashv_0
                                & (*(*(*server).server_prepared_statements).hh.tbl)
                                    .num_buckets
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            if !(*(*(*(*server).server_prepared_statements).hh.tbl)
                                .buckets
                                .offset(_hf_bkt_0 as isize))
                            .hh_head
                            .is_null()
                            {
                                server_ps = ((*(*(*(*server).server_prepared_statements).hh.tbl)
                                    .buckets
                                    .offset(_hf_bkt_0 as isize))
                                .hh_head
                                    as *mut ::core::ffi::c_char)
                                    .offset(-(*(*(*server).server_prepared_statements).hh.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut PgServerPreparedStatement
                                    as *mut PgServerPreparedStatement;
                            } else {
                                server_ps = ::core::ptr::null_mut::<PgServerPreparedStatement>();
                            }
                            while !server_ps.is_null() {
                                if (*server_ps).hh.hashv == _hf_hashv_0
                                    && (*server_ps).hh.keylen as usize
                                        == ::core::mem::size_of::<uint64_t>()
                                    && memcmp(
                                        (*server_ps).hh.key,
                                        &raw mut (*ps).query_id as *const ::core::ffi::c_void,
                                        ::core::mem::size_of::<uint64_t>() as size_t,
                                    ) == 0 as ::core::ffi::c_int
                                {
                                    break;
                                }
                                if !(*server_ps).hh.hh_next.is_null() {
                                    server_ps = ((*server_ps).hh.hh_next
                                        as *mut ::core::ffi::c_char)
                                        .offset(
                                            -(*(*(*server).server_prepared_statements).hh.tbl).hho,
                                        )
                                        as *mut ::core::ffi::c_void
                                        as *mut PgServerPreparedStatement
                                        as *mut PgServerPreparedStatement;
                                } else {
                                    server_ps =
                                        ::core::ptr::null_mut::<PgServerPreparedStatement>();
                                }
                            }
                        }
                    }
                    if !server_ps.is_null() {
                        if cf_verbose > 0 as ::core::ffi::c_int
                        {
                            log_generic(
                                LG_DEBUG,
                                client as *mut ::core::ffi::c_void,
                                c"handle_parse_command: mapping statement '%s' to '%s' (query '%s')".as_ptr(),
                                &raw mut (*client_ps).stmt_name as *mut ::core::ffi::c_char,
                                &raw mut (*ps).stmt_name as *mut ::core::ffi::c_char,
                                &raw mut (*ps).query_and_parameters
                                    as *mut ::core::ffi::c_char,
                            );
                        }
                        if !add_outstanding_request(
                            client,
                            PqMsg_Parse as ::core::ffi::c_char,
                            RA_FAKE,
                        ) {
                            current_block = 5810280148545133055;
                        } else {
                            current_block = 15360222575584860582;
                        }
                    } else {
                        current_block = 3760002206039831082;
                    }
                } else {
                    current_block = 3760002206039831082;
                }
                match current_block {
                    5810280148545133055 => {}
                    _ => {
                        if current_block == 3760002206039831082 {
                            if cf_verbose > 0 as ::core::ffi::c_int
                            {
                                log_generic(
                                    LG_DEBUG,
                                    client as *mut ::core::ffi::c_void,
                                    c"handle_parse_command: creating mapping for statement '%s' to '%s' (query '%s')".as_ptr(),
                                    &raw mut (*client_ps).stmt_name as *mut ::core::ffi::c_char,
                                    &raw mut (*ps).stmt_name as *mut ::core::ffi::c_char,
                                    &raw mut (*ps).query_and_parameters
                                        as *mut ::core::ffi::c_char,
                                );
                            }
                            buf = pktbuf_temp() as *mut PktBuf;
                            pktbuf_write_generic(
                                buf,
                                PqMsg_Parse,
                                c"sb".as_ptr(),
                                &raw mut (*ps).stmt_name as *mut ::core::ffi::c_char,
                                &raw mut (*ps).query_and_parameters as *mut ::core::ffi::c_char,
                                (*ps).query_and_parameters_len,
                            );
                            if !sbuf_queue_packet(
                                &raw mut (*client).sbuf,
                                &raw mut (*server).sbuf,
                                buf as *mut PktBuf,
                            ) {
                                current_block = 5810280148545133055;
                            } else {
                                (*(*client).pool).stats.ps_server_parse_count = (*(*client).pool)
                                    .stats
                                    .ps_server_parse_count
                                    .wrapping_add(1);
                                if !add_outstanding_request(
                                    client,
                                    PqMsg_Parse as ::core::ffi::c_char,
                                    RA_FORWARD,
                                ) {
                                    current_block = 5810280148545133055;
                                } else {
                                    server_ps = create_server_prepared_statement(ps);
                                    if server_ps.is_null() {
                                        current_block = 5810280148545133055;
                                    } else if !register_prepared_statement(
                                        client, server, server_ps,
                                    ) {
                                        current_block = 5810280148545133055;
                                    } else {
                                        current_block = 15360222575584860582;
                                    }
                                }
                            }
                        }
                        match current_block {
                            5810280148545133055 => {}
                            _ => {
                                skip_possibly_completely_buffered_packet(client, pkt);
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    free(client_ps as *mut ::core::ffi::c_void);
    free_server_prepared_statement(server_ps);
    disconnect_client(client, true, c"out of memory".as_ptr());
    disconnect_server((*client).link, true, c"out of memory".as_ptr());
    false
}

unsafe extern "C" fn get_client_prepared_statement(
    mut client: *mut PgSocket,
    mut name: *const ::core::ffi::c_char,
) -> *mut PgClientPreparedStatement {
    let mut client_ps = ::core::ptr::null_mut::<PgClientPreparedStatement>();
    let mut _uthash_hfstr_keylen = strlen(name) as ::core::ffi::c_uint;
    client_ps = ::core::ptr::null_mut::<PgClientPreparedStatement>();
    if !(*client).client_prepared_statements.is_null() {
        let mut _hf_hashv: ::core::ffi::c_uint = 0;
        let mut _hb_keylen = _uthash_hfstr_keylen;
        let mut _hb_key = name as *const ::core::ffi::c_uchar;
        _hf_hashv = 0 as ::core::ffi::c_uint;
        loop {
            let fresh29 = _hb_keylen;
            _hb_keylen = _hb_keylen.wrapping_sub(1);
            if fresh29 == 0 as ::core::ffi::c_uint {
                break;
            }
            let fresh30 = _hb_key;
            _hb_key = _hb_key.offset(1);
            _hf_hashv = (_hf_hashv << 5 as ::core::ffi::c_int)
                .wrapping_add(_hf_hashv)
                .wrapping_add(*fresh30 as ::core::ffi::c_uint);
        }
        client_ps = ::core::ptr::null_mut::<PgClientPreparedStatement>();
        if !(*client).client_prepared_statements.is_null() {
            let mut _hf_bkt: ::core::ffi::c_uint = 0;
            _hf_bkt = _hf_hashv
                & (*(*(*client).client_prepared_statements).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            if !(*(*(*(*client).client_prepared_statements).hh.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                client_ps = ((*(*(*(*client).client_prepared_statements).hh.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*client).client_prepared_statements).hh.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut PgClientPreparedStatement
                    as *mut PgClientPreparedStatement;
            } else {
                client_ps = ::core::ptr::null_mut::<PgClientPreparedStatement>();
            }
            while !client_ps.is_null() {
                if (*client_ps).hh.hashv == _hf_hashv
                    && (*client_ps).hh.keylen == _uthash_hfstr_keylen
                    && memcmp(
                        (*client_ps).hh.key,
                        name as *const ::core::ffi::c_void,
                        _uthash_hfstr_keylen as size_t,
                    ) == 0 as ::core::ffi::c_int
                {
                    break;
                }
                if !(*client_ps).hh.hh_next.is_null() {
                    client_ps = ((*client_ps).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*client).client_prepared_statements).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut PgClientPreparedStatement
                        as *mut PgClientPreparedStatement;
                } else {
                    client_ps = ::core::ptr::null_mut::<PgClientPreparedStatement>();
                }
            }
        }
    }
    if client_ps.is_null() {
        log_generic(
            LG_ERROR,
            client as *mut ::core::ffi::c_void,
            c"prepared statement '%s' not found".as_ptr(),
            name,
        );
        disconnect_client(client, true, c"prepared statement did not exist".as_ptr());
    }
    client_ps
}

unsafe extern "C" fn ensure_statement_is_prepared_on_server(
    mut server: *mut PgSocket,
    mut ps: *mut PgPreparedStatement,
) -> bool {
    let mut client = (*server).link;
    let mut server_ps = ::core::ptr::null_mut::<PgServerPreparedStatement>();
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    server_ps = ::core::ptr::null_mut::<PgServerPreparedStatement>();
    if !(*server).server_prepared_statements.is_null() {
        let mut _hf_hashv: ::core::ffi::c_uint = 0;
        let mut _hb_keylen = ::core::mem::size_of::<uint64_t>() as ::core::ffi::c_uint;
        let mut _hb_key = &raw mut (*ps).query_id as *const ::core::ffi::c_uchar;
        _hf_hashv = 0 as ::core::ffi::c_uint;
        loop {
            let fresh25 = _hb_keylen;
            _hb_keylen = _hb_keylen.wrapping_sub(1);
            if fresh25 == 0 as ::core::ffi::c_uint {
                break;
            }
            let fresh26 = _hb_key;
            _hb_key = _hb_key.offset(1);
            _hf_hashv = (_hf_hashv << 5 as ::core::ffi::c_int)
                .wrapping_add(_hf_hashv)
                .wrapping_add(*fresh26 as ::core::ffi::c_uint);
        }
        server_ps = ::core::ptr::null_mut::<PgServerPreparedStatement>();
        if !(*server).server_prepared_statements.is_null() {
            let mut _hf_bkt: ::core::ffi::c_uint = 0;
            _hf_bkt = _hf_hashv
                & (*(*(*server).server_prepared_statements).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            if !(*(*(*(*server).server_prepared_statements).hh.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                server_ps = ((*(*(*(*server).server_prepared_statements).hh.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*server).server_prepared_statements).hh.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut PgServerPreparedStatement
                    as *mut PgServerPreparedStatement;
            } else {
                server_ps = ::core::ptr::null_mut::<PgServerPreparedStatement>();
            }
            while !server_ps.is_null() {
                if (*server_ps).hh.hashv == _hf_hashv
                    && (*server_ps).hh.keylen as usize == ::core::mem::size_of::<uint64_t>()
                    && memcmp(
                        (*server_ps).hh.key,
                        &raw mut (*ps).query_id as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<uint64_t>() as size_t,
                    ) == 0 as ::core::ffi::c_int
                {
                    break;
                }
                if !(*server_ps).hh.hh_next.is_null() {
                    server_ps = ((*server_ps).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*server).server_prepared_statements).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut PgServerPreparedStatement
                        as *mut PgServerPreparedStatement;
                } else {
                    server_ps = ::core::ptr::null_mut::<PgServerPreparedStatement>();
                }
            }
        }
    }
    if !server_ps.is_null() {
        if (if !(*server).server_prepared_statements.is_null() {
            (*(*(*server).server_prepared_statements).hh.tbl).num_items
        } else {
            0 as ::core::ffi::c_uint
        }) != 1 as ::core::ffi::c_uint
        {
            let mut _hd_hh_del: *mut UT_hash_handle = &raw mut (*server_ps).hh;
            if !(*_hd_hh_del).prev.is_null() {
                let fresh27 = &mut (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*server).server_prepared_statements).hh.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh27 = (*_hd_hh_del).next;
            } else {
                (*server).server_prepared_statements = (*_hd_hh_del).next
                    as *mut PgServerPreparedStatement
                    as *mut PgServerPreparedStatement;
            }
            if !(*_hd_hh_del).next.is_null() {
                let fresh28 = &mut (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*(*server).server_prepared_statements).hh.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut UT_hash_handle))
                    .prev;
                *fresh28 = (*_hd_hh_del).prev;
            } else {
                (*(*_hd_hh_del).tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*_hd_hh_del).tbl).hho as isize)
                    as *mut ::core::ffi::c_void
                    as *mut UT_hash_handle
                    as *mut UT_hash_handle;
            }
            (*server_ps).hh.next = NULL;
            (*server_ps).hh.prev = ((*(*(*server).server_prepared_statements).hh.tbl).tail
                as *mut ::core::ffi::c_char)
                .offset(-(*(*(*server).server_prepared_statements).hh.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(*(*server).server_prepared_statements).hh.tbl).tail).next =
                server_ps as *mut ::core::ffi::c_void;
            (*(*(*server).server_prepared_statements).hh.tbl).tail =
                &raw mut (*server_ps).hh as *mut UT_hash_handle;
        }
        return true;
    }
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            server as *mut ::core::ffi::c_void,
            c"handle_bind_command: prepared statement '%s' (query '%s') not available on server, preparing '%s' before bind".as_ptr(),
            &raw mut (*ps).stmt_name as *mut ::core::ffi::c_char,
            &raw mut (*ps).query_and_parameters as *mut ::core::ffi::c_char,
            &raw mut (*ps).stmt_name as *mut ::core::ffi::c_char,
        );
    }
    (*(*client).pool).stats.ps_server_parse_count = (*(*client).pool)
        .stats
        .ps_server_parse_count
        .wrapping_add(1);
    if !add_outstanding_request(client, PqMsg_Parse as ::core::ffi::c_char, RA_SKIP) {
        return false;
    }
    buf = pktbuf_temp() as *mut PktBuf;
    pktbuf_write_generic(
        buf,
        PqMsg_Parse,
        c"sb".as_ptr(),
        &raw mut (*ps).stmt_name as *mut ::core::ffi::c_char,
        &raw mut (*ps).query_and_parameters as *mut ::core::ffi::c_char,
        (*ps).query_and_parameters_len,
    );
    if !sbuf_queue_packet(
        &raw mut (*client).sbuf,
        &raw mut (*server).sbuf,
        buf as *mut PktBuf,
    ) {
        return false;
    }
    server_ps = create_server_prepared_statement(ps);
    if server_ps.is_null() {
        return false;
    }
    if !register_prepared_statement(client, server, server_ps) {
        free_server_prepared_statement(server_ps);
        return false;
    }
    true
}
#[no_mangle]

pub unsafe extern "C" fn handle_bind_command(
    mut client: *mut PgSocket,
    mut pkt: *mut PktHdr,
) -> bool {
    let mut server = (*client).link;
    let mut bp = PgBindPacket {
        len: 0,
        portal: ::core::ptr::null::<::core::ffi::c_char>(),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut client_ps = ::core::ptr::null_mut::<PgClientPreparedStatement>();
    let mut ps = ::core::ptr::null_mut::<PgPreparedStatement>();
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    let mut diff: ::core::ffi::c_int = 0;
    if !unmarshall_bind_packet(client, pkt, &raw mut bp) {
        return false;
    }
    (*(*client).pool).stats.ps_bind_count = (*(*client).pool).stats.ps_bind_count.wrapping_add(1);
    client_ps = get_client_prepared_statement(client, bp.name);
    if client_ps.is_null() {
        return false;
    }
    ps = (*client_ps).ps;
    if ensure_statement_is_prepared_on_server(server, ps) {
        if cf_verbose > 0 as ::core::ffi::c_int
        {
            log_generic(
                LG_DEBUG,
                client as *mut ::core::ffi::c_void,
                c"handle_bind_command: mapped statement '%s' (query '%s') to '%s'".as_ptr(),
                bp.name,
                &raw mut (*ps).query_and_parameters as *mut ::core::ffi::c_char,
                &raw mut (*ps).stmt_name as *mut ::core::ffi::c_char,
            );
        }
        if add_outstanding_request(client, PqMsg_Bind as ::core::ffi::c_char, RA_FORWARD) {
            diff =
                strlen(bp.name).wrapping_sub((*ps).stmt_name_len as size_t) as ::core::ffi::c_int;
            buf = pktbuf_temp() as *mut PktBuf;
            if !buf.is_null() {
                pktbuf_put_char(buf, (*pkt).type_0 as ::core::ffi::c_char);
                pktbuf_put_uint32(
                    buf,
                    ((*pkt).len as uint32_t)
                        .wrapping_sub(diff as uint32_t)
                        .wrapping_sub(1 as uint32_t),
                );
                pktbuf_put_string(buf, bp.portal);
                pktbuf_put_string(buf, &raw mut (*ps).stmt_name as *mut ::core::ffi::c_char);
                if (*client).packet_cb_state.flag() as ::core::ffi::c_int
                    == CB_HANDLE_COMPLETE_PACKET as ::core::ffi::c_int
                {
                    pktbuf_put_bytes(
                        buf,
                        (*pkt).data.data.offset((*pkt).data.read_pos as isize)
                            as *const ::core::ffi::c_void,
                        (*pkt).data.write_pos.wrapping_sub((*pkt).data.read_pos)
                            as ::core::ffi::c_int,
                    );
                    if sbuf_queue_packet(
                        &raw mut (*client).sbuf,
                        &raw mut (*server).sbuf,
                        buf as *mut PktBuf,
                    ) {
                        return true;
                    }
                } else if sbuf_queue_packet(
                    &raw mut (*client).sbuf,
                    &raw mut (*server).sbuf,
                    buf as *mut PktBuf,
                ) {
                    sbuf_prepare_skip_then_send_leftover(
                        &raw mut (*client).sbuf,
                        &raw mut (*server).sbuf,
                        (*pkt).data.read_pos,
                        (*pkt).len,
                    );
                    return true;
                }
            }
        }
    }
    disconnect_client(client, true, c"out of memory".as_ptr());
    disconnect_server((*client).link, true, c"out of memory".as_ptr());
    false
}
#[no_mangle]

pub unsafe extern "C" fn handle_describe_command(
    mut client: *mut PgSocket,
    mut pkt: *mut PktHdr,
) -> bool {
    let mut server = (*client).link;
    let mut dp = PgDescribePacket {
        type_0: 0,
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut client_ps = ::core::ptr::null_mut::<PgClientPreparedStatement>();
    let mut ps = ::core::ptr::null_mut::<PgPreparedStatement>();
    let mut res: bool = false;
    if !unmarshall_describe_packet(client, pkt, &raw mut dp)
        || dp.type_0 as ::core::ffi::c_int != 'S' as i32
    {
        return false;
    }
    client_ps = get_client_prepared_statement(client, dp.name);
    if client_ps.is_null() {
        return false;
    }
    ps = (*client_ps).ps;
    if ensure_statement_is_prepared_on_server(server, ps) {
        if cf_verbose > 0 as ::core::ffi::c_int
        {
            log_generic(
                LG_DEBUG,
                client as *mut ::core::ffi::c_void,
                c"handle_describe_command: mapped statement '%s' (query '%s') to '%s'".as_ptr(),
                dp.name,
                &raw mut (*ps).query_and_parameters as *mut ::core::ffi::c_char,
                &raw mut (*ps).stmt_name as *mut ::core::ffi::c_char,
            );
        }
        if add_outstanding_request(client, PqMsg_Describe as ::core::ffi::c_char, RA_FORWARD) {
            skip_possibly_completely_buffered_packet(client, pkt);
            let mut _data: [uint8_t; 37] = [0; 37];
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
                ::core::mem::size_of::<[uint8_t; 37]>() as ::core::ffi::c_int,
            );
            pktbuf_write_generic(
                &raw mut _buf,
                PqMsg_Describe,
                c"cs".as_ptr(),
                'S' as i32,
                &raw mut (*ps).stmt_name as *mut ::core::ffi::c_char,
            );
            res = sbuf_queue_packet(
                &raw mut (*client).sbuf,
                &raw mut (*server).sbuf,
                &raw mut _buf,
            );
            return res;
        }
    }
    disconnect_client(client, true, c"out of memory".as_ptr());
    disconnect_server((*client).link, true, c"out of memory".as_ptr());
    false
}
#[no_mangle]

pub unsafe extern "C" fn handle_close_statement_command(
    mut client: *mut PgSocket,
    mut pkt: *mut PktHdr,
    mut close_packet: *mut PgClosePacket,
) -> bool {
    let mut client_ps = ::core::ptr::null_mut::<PgClientPreparedStatement>();
    let mut res = true;
    let mut _uthash_hfstr_keylen = strlen((*close_packet).name) as ::core::ffi::c_uint;
    client_ps = ::core::ptr::null_mut::<PgClientPreparedStatement>();
    if !(*client).client_prepared_statements.is_null() {
        let mut _hf_hashv: ::core::ffi::c_uint = 0;
        let mut _hb_keylen = _uthash_hfstr_keylen;
        let mut _hb_key = (*close_packet).name as *const ::core::ffi::c_uchar;
        _hf_hashv = 0 as ::core::ffi::c_uint;
        loop {
            let fresh31 = _hb_keylen;
            _hb_keylen = _hb_keylen.wrapping_sub(1);
            if fresh31 == 0 as ::core::ffi::c_uint {
                break;
            }
            let fresh32 = _hb_key;
            _hb_key = _hb_key.offset(1);
            _hf_hashv = (_hf_hashv << 5 as ::core::ffi::c_int)
                .wrapping_add(_hf_hashv)
                .wrapping_add(*fresh32 as ::core::ffi::c_uint);
        }
        client_ps = ::core::ptr::null_mut::<PgClientPreparedStatement>();
        if !(*client).client_prepared_statements.is_null() {
            let mut _hf_bkt: ::core::ffi::c_uint = 0;
            _hf_bkt = _hf_hashv
                & (*(*(*client).client_prepared_statements).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            if !(*(*(*(*client).client_prepared_statements).hh.tbl)
                .buckets
                .offset(_hf_bkt as isize))
            .hh_head
            .is_null()
            {
                client_ps = ((*(*(*(*client).client_prepared_statements).hh.tbl)
                    .buckets
                    .offset(_hf_bkt as isize))
                .hh_head as *mut ::core::ffi::c_char)
                    .offset(-(*(*(*client).client_prepared_statements).hh.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut PgClientPreparedStatement
                    as *mut PgClientPreparedStatement;
            } else {
                client_ps = ::core::ptr::null_mut::<PgClientPreparedStatement>();
            }
            while !client_ps.is_null() {
                if (*client_ps).hh.hashv == _hf_hashv
                    && (*client_ps).hh.keylen == _uthash_hfstr_keylen
                    && memcmp(
                        (*client_ps).hh.key,
                        (*close_packet).name as *const ::core::ffi::c_void,
                        _uthash_hfstr_keylen as size_t,
                    ) == 0 as ::core::ffi::c_int
                {
                    break;
                }
                if !(*client_ps).hh.hh_next.is_null() {
                    client_ps = ((*client_ps).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(*(*client).client_prepared_statements).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut PgClientPreparedStatement
                        as *mut PgClientPreparedStatement;
                } else {
                    client_ps = ::core::ptr::null_mut::<PgClientPreparedStatement>();
                }
            }
        }
    }
    if !client_ps.is_null() {
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(
                LG_NOISE,
                client as *mut ::core::ffi::c_void,
                c"handle_close_command: removed '%s' from cached prepared statements, items remaining %u".as_ptr(),
                (*close_packet).name,
                if !(*client).client_prepared_statements.is_null() {
                    (*(*(*client).client_prepared_statements).hh.tbl).num_items
                } else {
                    0 as ::core::ffi::c_uint
                },
            );
        }
        let mut _hd_hh_del: *const UT_hash_handle = &raw mut (*client_ps).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free(
                (*(*(*client).client_prepared_statements).hh.tbl).buckets
                    as *mut ::core::ffi::c_void,
            );
            free((*(*client).client_prepared_statements).hh.tbl as *mut ::core::ffi::c_void);
            (*client).client_prepared_statements =
                ::core::ptr::null_mut::<PgClientPreparedStatement>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if std::ptr::eq(
                _hd_hh_del,
                (*(*(*client).client_prepared_statements).hh.tbl).tail,
            ) {
                (*(*(*client).client_prepared_statements).hh.tbl).tail =
                    ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*(*client).client_prepared_statements).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut UT_hash_handle
                        as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let fresh33 = &mut (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*client).client_prepared_statements).hh.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh33 = (*_hd_hh_del).next;
            } else {
                (*client).client_prepared_statements = (*_hd_hh_del).next
                    as *mut PgClientPreparedStatement
                    as *mut PgClientPreparedStatement;
            }
            if !(*_hd_hh_del).next.is_null() {
                let fresh34 = &mut (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*(*client).client_prepared_statements).hh.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut UT_hash_handle))
                    .prev;
                *fresh34 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*(*client).client_prepared_statements).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UT_hash_bucket =
                (*(*(*client).client_prepared_statements).hh.tbl)
                    .buckets
                    .offset(_hd_bkt as isize) as *mut UT_hash_bucket;
            (*_hd_head).count -= 1;
            if std::ptr::eq((*_hd_head).hh_head, _hd_hh_del) {
                (*_hd_head).hh_head = (*_hd_hh_del).hh_next as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).hh_prev.is_null() {
                (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
            }
            if !(*_hd_hh_del).hh_next.is_null() {
                (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
            }
            (*(*(*client).client_prepared_statements).hh.tbl).num_items =
                (*(*(*client).client_prepared_statements).hh.tbl)
                    .num_items
                    .wrapping_sub(1);
        }
        (*(*client_ps).ps).use_count = (*(*client_ps).ps).use_count.wrapping_sub(1);
        if (*(*client_ps).ps).use_count == 0 {
            let mut _hd_hh_del_0: *const UT_hash_handle = &raw mut (*(*client_ps).ps).hh;
            if (*_hd_hh_del_0).prev.is_null() && (*_hd_hh_del_0).next.is_null() {
                free((*(*prepared_statements).hh.tbl).buckets as *mut ::core::ffi::c_void);
                free((*prepared_statements).hh.tbl as *mut ::core::ffi::c_void);
                prepared_statements = ::core::ptr::null_mut::<PgPreparedStatement>();
            } else {
                let mut _hd_bkt_0: ::core::ffi::c_uint = 0;
                if std::ptr::eq(_hd_hh_del_0, (*(*prepared_statements).hh.tbl).tail) {
                    (*(*prepared_statements).hh.tbl).tail =
                        ((*_hd_hh_del_0).prev as *mut ::core::ffi::c_char)
                            .offset((*(*prepared_statements).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut UT_hash_handle as *mut UT_hash_handle;
                }
                if !(*_hd_hh_del_0).prev.is_null() {
                    let fresh35 = &mut (*(((*_hd_hh_del_0).prev as *mut ::core::ffi::c_char)
                        .offset((*(*prepared_statements).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut UT_hash_handle))
                        .next;
                    *fresh35 = (*_hd_hh_del_0).next;
                } else {
                    prepared_statements = (*_hd_hh_del_0).next as *mut PgPreparedStatement
                        as *mut PgPreparedStatement;
                }
                if !(*_hd_hh_del_0).next.is_null() {
                    let fresh36 = &mut (*(((*_hd_hh_del_0).next as *mut ::core::ffi::c_char)
                        .offset((*(*prepared_statements).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut UT_hash_handle))
                        .prev;
                    *fresh36 = (*_hd_hh_del_0).prev;
                }
                _hd_bkt_0 = (*_hd_hh_del_0).hashv
                    & (*(*prepared_statements).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let mut _hd_head_0: *mut UT_hash_bucket = (*(*prepared_statements).hh.tbl)
                    .buckets
                    .offset(_hd_bkt_0 as isize)
                    as *mut UT_hash_bucket;
                (*_hd_head_0).count -= 1;
                if std::ptr::eq((*_hd_head_0).hh_head, _hd_hh_del_0) {
                    (*_hd_head_0).hh_head = (*_hd_hh_del_0).hh_next as *mut UT_hash_handle;
                }
                if !(*_hd_hh_del_0).hh_prev.is_null() {
                    (*(*_hd_hh_del_0).hh_prev).hh_next = (*_hd_hh_del_0).hh_next;
                }
                if !(*_hd_hh_del_0).hh_next.is_null() {
                    (*(*_hd_hh_del_0).hh_next).hh_prev = (*_hd_hh_del_0).hh_prev;
                }
                (*(*prepared_statements).hh.tbl).num_items =
                    (*(*prepared_statements).hh.tbl).num_items.wrapping_sub(1);
            }
            free((*client_ps).ps as *mut ::core::ffi::c_void);
        }
        free(client_ps as *mut ::core::ffi::c_void);
    }
    skip_possibly_completely_buffered_packet(client, pkt);
    if (*client).link.is_null()
        || statlist_count(&raw mut (*(*client).link).outstanding_requests)
            == 0 as ::core::ffi::c_int
    {
        if cf_verbose > 0 as ::core::ffi::c_int
        {
            log_generic(
                LG_DEBUG,
                client as *mut ::core::ffi::c_void,
                c"handle_close_statement_command: no outstanding requests so instantly answering client".as_ptr(),
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
        pktbuf_write_generic(&raw mut _buf, PqMsg_CloseComplete, c"".as_ptr());
        res = pktbuf_send_immediate(&raw mut _buf, client);
        return res;
    }
    if !add_outstanding_request(client, PqMsg_Close as ::core::ffi::c_char, RA_FAKE) {
        return false;
    }
    true
}
#[no_mangle]

pub unsafe extern "C" fn free_client_prepared_statements(mut client: *mut PgSocket) {
    let mut client_ps = ::core::ptr::null_mut::<PgClientPreparedStatement>();
    let mut tmp = ::core::ptr::null_mut::<PgClientPreparedStatement>();
    client_ps = (*client).client_prepared_statements;
    tmp = (if !(*client).client_prepared_statements.is_null() {
        (*(*client).client_prepared_statements).hh.next
    } else {
        NULL
    }) as *mut PgClientPreparedStatement as *mut PgClientPreparedStatement;
    while !client_ps.is_null() {
        let mut _hd_hh_del: *const UT_hash_handle = &raw mut (*client_ps).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free(
                (*(*(*client).client_prepared_statements).hh.tbl).buckets
                    as *mut ::core::ffi::c_void,
            );
            free((*(*client).client_prepared_statements).hh.tbl as *mut ::core::ffi::c_void);
            (*client).client_prepared_statements =
                ::core::ptr::null_mut::<PgClientPreparedStatement>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if std::ptr::eq(
                _hd_hh_del,
                (*(*(*client).client_prepared_statements).hh.tbl).tail,
            ) {
                (*(*(*client).client_prepared_statements).hh.tbl).tail =
                    ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*(*client).client_prepared_statements).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut UT_hash_handle
                        as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let fresh41 = &mut (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*client).client_prepared_statements).hh.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh41 = (*_hd_hh_del).next;
            } else {
                (*client).client_prepared_statements = (*_hd_hh_del).next
                    as *mut PgClientPreparedStatement
                    as *mut PgClientPreparedStatement;
            }
            if !(*_hd_hh_del).next.is_null() {
                let fresh42 = &mut (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*(*client).client_prepared_statements).hh.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut UT_hash_handle))
                    .prev;
                *fresh42 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*(*client).client_prepared_statements).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UT_hash_bucket =
                (*(*(*client).client_prepared_statements).hh.tbl)
                    .buckets
                    .offset(_hd_bkt as isize) as *mut UT_hash_bucket;
            (*_hd_head).count -= 1;
            if std::ptr::eq((*_hd_head).hh_head, _hd_hh_del) {
                (*_hd_head).hh_head = (*_hd_hh_del).hh_next as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).hh_prev.is_null() {
                (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
            }
            if !(*_hd_hh_del).hh_next.is_null() {
                (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
            }
            (*(*(*client).client_prepared_statements).hh.tbl).num_items =
                (*(*(*client).client_prepared_statements).hh.tbl)
                    .num_items
                    .wrapping_sub(1);
        }
        (*(*client_ps).ps).use_count = (*(*client_ps).ps).use_count.wrapping_sub(1);
        if (*(*client_ps).ps).use_count == 0 {
            let mut _hd_hh_del_0: *const UT_hash_handle = &raw mut (*(*client_ps).ps).hh;
            if (*_hd_hh_del_0).prev.is_null() && (*_hd_hh_del_0).next.is_null() {
                free((*(*prepared_statements).hh.tbl).buckets as *mut ::core::ffi::c_void);
                free((*prepared_statements).hh.tbl as *mut ::core::ffi::c_void);
                prepared_statements = ::core::ptr::null_mut::<PgPreparedStatement>();
            } else {
                let mut _hd_bkt_0: ::core::ffi::c_uint = 0;
                if std::ptr::eq(_hd_hh_del_0, (*(*prepared_statements).hh.tbl).tail) {
                    (*(*prepared_statements).hh.tbl).tail =
                        ((*_hd_hh_del_0).prev as *mut ::core::ffi::c_char)
                            .offset((*(*prepared_statements).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut UT_hash_handle as *mut UT_hash_handle;
                }
                if !(*_hd_hh_del_0).prev.is_null() {
                    let fresh43 = &mut (*(((*_hd_hh_del_0).prev as *mut ::core::ffi::c_char)
                        .offset((*(*prepared_statements).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut UT_hash_handle))
                        .next;
                    *fresh43 = (*_hd_hh_del_0).next;
                } else {
                    prepared_statements = (*_hd_hh_del_0).next as *mut PgPreparedStatement
                        as *mut PgPreparedStatement;
                }
                if !(*_hd_hh_del_0).next.is_null() {
                    let fresh44 = &mut (*(((*_hd_hh_del_0).next as *mut ::core::ffi::c_char)
                        .offset((*(*prepared_statements).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut UT_hash_handle))
                        .prev;
                    *fresh44 = (*_hd_hh_del_0).prev;
                }
                _hd_bkt_0 = (*_hd_hh_del_0).hashv
                    & (*(*prepared_statements).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                let mut _hd_head_0: *mut UT_hash_bucket = (*(*prepared_statements).hh.tbl)
                    .buckets
                    .offset(_hd_bkt_0 as isize)
                    as *mut UT_hash_bucket;
                (*_hd_head_0).count -= 1;
                if std::ptr::eq((*_hd_head_0).hh_head, _hd_hh_del_0) {
                    (*_hd_head_0).hh_head = (*_hd_hh_del_0).hh_next as *mut UT_hash_handle;
                }
                if !(*_hd_hh_del_0).hh_prev.is_null() {
                    (*(*_hd_hh_del_0).hh_prev).hh_next = (*_hd_hh_del_0).hh_next;
                }
                if !(*_hd_hh_del_0).hh_next.is_null() {
                    (*(*_hd_hh_del_0).hh_next).hh_prev = (*_hd_hh_del_0).hh_prev;
                }
                (*(*prepared_statements).hh.tbl).num_items =
                    (*(*prepared_statements).hh.tbl).num_items.wrapping_sub(1);
            }
            free((*client_ps).ps as *mut ::core::ffi::c_void);
        }
        free(client_ps as *mut ::core::ffi::c_void);
        client_ps = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut PgClientPreparedStatement
            as *mut PgClientPreparedStatement;
    }
    free((*client).client_prepared_statements as *mut ::core::ffi::c_void);
    (*client).client_prepared_statements = ::core::ptr::null_mut::<PgClientPreparedStatement>();
}
#[no_mangle]

pub unsafe extern "C" fn free_server_prepared_statements(mut server: *mut PgSocket) {
    let mut current = ::core::ptr::null_mut::<PgServerPreparedStatement>();
    let mut tmp_s = ::core::ptr::null_mut::<PgServerPreparedStatement>();
    current = (*server).server_prepared_statements as *mut PgServerPreparedStatement;
    tmp_s = (if !(*server).server_prepared_statements.is_null() {
        (*(*server).server_prepared_statements).hh.next
    } else {
        NULL
    }) as *mut PgServerPreparedStatement as *mut PgServerPreparedStatement;
    while !current.is_null() {
        let mut _hd_hh_del: *const UT_hash_handle = &raw mut (*current).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free(
                (*(*(*server).server_prepared_statements).hh.tbl).buckets
                    as *mut ::core::ffi::c_void,
            );
            free((*(*server).server_prepared_statements).hh.tbl as *mut ::core::ffi::c_void);
            (*server).server_prepared_statements =
                ::core::ptr::null_mut::<PgServerPreparedStatement>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if std::ptr::eq(
                _hd_hh_del,
                (*(*(*server).server_prepared_statements).hh.tbl).tail,
            ) {
                (*(*(*server).server_prepared_statements).hh.tbl).tail =
                    ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                        .offset((*(*(*server).server_prepared_statements).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut UT_hash_handle
                        as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let fresh45 = &mut (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*(*server).server_prepared_statements).hh.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut UT_hash_handle))
                    .next;
                *fresh45 = (*_hd_hh_del).next;
            } else {
                (*server).server_prepared_statements = (*_hd_hh_del).next
                    as *mut PgServerPreparedStatement
                    as *mut PgServerPreparedStatement;
            }
            if !(*_hd_hh_del).next.is_null() {
                let fresh46 = &mut (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*(*server).server_prepared_statements).hh.tbl).hho)
                    as *mut ::core::ffi::c_void
                    as *mut UT_hash_handle))
                    .prev;
                *fresh46 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*(*server).server_prepared_statements).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UT_hash_bucket =
                (*(*(*server).server_prepared_statements).hh.tbl)
                    .buckets
                    .offset(_hd_bkt as isize) as *mut UT_hash_bucket;
            (*_hd_head).count -= 1;
            if std::ptr::eq((*_hd_head).hh_head, _hd_hh_del) {
                (*_hd_head).hh_head = (*_hd_hh_del).hh_next as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).hh_prev.is_null() {
                (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
            }
            if !(*_hd_hh_del).hh_next.is_null() {
                (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
            }
            (*(*(*server).server_prepared_statements).hh.tbl).num_items =
                (*(*(*server).server_prepared_statements).hh.tbl)
                    .num_items
                    .wrapping_sub(1);
        }
        free_server_prepared_statement(current as *mut PgServerPreparedStatement);
        current = tmp_s;
        tmp_s = (if !tmp_s.is_null() {
            (*tmp_s).hh.next
        } else {
            NULL
        }) as *mut PgServerPreparedStatement as *mut PgServerPreparedStatement;
    }
    free((*server).server_prepared_statements as *mut ::core::ffi::c_void);
    (*server).server_prepared_statements = ::core::ptr::null_mut::<PgServerPreparedStatement>();
}
