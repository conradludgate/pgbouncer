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
    use crate::types::{__darwin_rune_t, __darwin_size_t, __uint32_t};
    extern "C" {

        pub static mut _DefaultRuneLocale: _RuneLocale;
    }
}

pub mod stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct stat {
        pub st_dev: dev_t,
        pub st_mode: mode_t,
        pub st_nlink: nlink_t,
        pub st_ino: __darwin_ino64_t,
        pub st_uid: uid_t,
        pub st_gid: gid_t,
        pub st_rdev: dev_t,
        pub st_atimespec: timespec,
        pub st_mtimespec: timespec,
        pub st_ctimespec: timespec,
        pub st_birthtimespec: timespec,
        pub st_size: off_t,
        pub st_blocks: blkcnt_t,
        pub st_blksize: blksize_t,
        pub st_flags: __uint32_t,
        pub st_gen: __uint32_t,
        pub st_lspare: __int32_t,
        pub st_qspare: [__int64_t; 2],
    }
    use crate::types::blkcnt_t;
    use crate::types::blksize_t;
    use crate::types::dev_t;
    use crate::types::gid_t;
    use crate::types::mode_t;
    use crate::types::nlink_t;
    use crate::types::off_t;
    use crate::types::timespec;
    use crate::types::__darwin_ino64_t;
    use crate::types::uid_t;
    use crate::types::{__int32_t, __int64_t, __uint32_t};
    extern "C" {

        pub fn stat(_: *const ::core::ffi::c_char, _: *mut stat) -> ::core::ffi::c_int;
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
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct CfLookup {
        pub name: *const ::core::ffi::c_char,
        pub value: ::core::ffi::c_int,
    }
    extern "C" {

        pub fn cf_set_lookup(cv: *mut CfValue, value: *const ::core::ffi::c_char) -> bool;
    }
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

    pub const MAX_USERNAME: ::core::ffi::c_int = 128 as ::core::ffi::c_int;

    pub const MAX_PASSWORD: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;

    pub const POOL_INHERIT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    #[inline]

    pub unsafe extern "C" fn cstr_skip_ws(
        mut p: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char {
        while *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int == ' ' as i32 {
            p = p.offset(1);
        }
        p
    }
    use super::cfparser_h::CfLookup;
    use super::dnslookup_h::DNSToken;
    use crate::types::sockaddr_in6;
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

        pub static mut cf_autodb_connstr: *mut ::core::ffi::c_char;

        pub static mut cf_auth_file: *mut ::core::ffi::c_char;

        pub static pool_mode_map: [CfLookup; 0];

        pub static load_balance_hosts_map: [CfLookup; 0];
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
    use crate::types::tls;
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

        pub fn pktbuf_dynamic(start_len: ::core::ffi::c_int) -> *mut PktBuf;

        pub fn pktbuf_reset(pkt: *mut PktBuf);

        pub fn pktbuf_put_string(buf: *mut PktBuf, str: *const ::core::ffi::c_char);
    }
}

pub mod dnslookup_h {
    extern "C" {

        pub type DNSToken;
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
    use super::runetype_h::_DefaultRuneLocale;
    use crate::types::__darwin_ct_rune_t;
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

pub mod objects_h {
    use super::bouncer_h::{PgCredentials, PgDatabase, PgGlobalUser};

    use crate::types::StatList;
    extern "C" {

        pub static mut user_list: StatList;

        pub fn add_peer(
            name: *const ::core::ffi::c_char,
            peer_id: ::core::ffi::c_int,
        ) -> *mut PgDatabase;

        pub fn add_database(name: *const ::core::ffi::c_char) -> *mut PgDatabase;

        pub fn force_user_credentials(
            db: *mut PgDatabase,
            username: *const ::core::ffi::c_char,
            passwd: *const ::core::ffi::c_char,
        ) -> *mut PgCredentials;

        pub fn update_global_user_passwd(
            user: *mut PgGlobalUser,
            passwd: *const ::core::ffi::c_char,
        ) -> *mut PgGlobalUser;

        pub fn find_or_add_new_global_user(
            name: *const ::core::ffi::c_char,
            passwd: *const ::core::ffi::c_char,
        ) -> *mut PgGlobalUser;

        pub fn find_or_add_new_global_credentials(
            name: *const ::core::ffi::c_char,
            passwd: *const ::core::ffi::c_char,
        ) -> *mut PgCredentials;

        pub fn tag_database_dirty(db: *mut PgDatabase);

        pub fn tag_autodb_dirty();
    }
}

pub mod fileutil_h {
    use crate::types::size_t;
    extern "C" {

        pub fn load_file(
            fn_0: *const ::core::ffi::c_char,
            len_p: *mut size_t,
        ) -> *mut ::core::ffi::c_void;
    }
}

pub mod errno_h {
    extern "C" {

        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}

pub mod string_h {
    extern "C" {

        pub fn strcmpeq(
            str_left: *const ::core::ffi::c_char,
            str_right: *const ::core::ffi::c_char,
        ) -> bool;
    }
}

pub mod util_h {
    extern "C" {

        pub fn check_reserved_database(value: *const ::core::ffi::c_char) -> bool;
    }
}
pub use crate::types::blkcnt_t;
pub use crate::types::blksize_t;
pub use self::_ctype_h::{__istype, __maskrune, isascii, isspace, _CTYPE_S};
pub use crate::types::dev_t;
pub use crate::types::gid_t;
use crate::types::free;
pub use crate::types::mode_t;
pub use crate::types::nlink_t;
pub use crate::types::off_t;
use crate::types::{atoi, exit, strtonum};
use crate::types::{memset, strcmp, strdup, strerror};
pub use crate::types::timespec;
pub use self::bouncer_h::{
    cf_auth_file, cf_autodb_connstr, cstr_skip_ws, load_balance_hosts_map, pool_mode_map,
    sockaddr_ucreds, C2RustUnnamed_9, CallbackState, LoadBalanceHosts, PacketCallbackFlag, PgAddr,
    PgCredentials, PgDatabase, PgGlobalUser, PgPool, PgSocket, PgStats, ReplicationType,
    ScramState, SocketState, CB_HANDLE_COMPLETE_PACKET, CB_NONE, CB_WANT_COMPLETE_PACKET,
    CL_ACTIVE, CL_ACTIVE_CANCEL, CL_FREE, CL_JUSTFREE, CL_LOGIN, CL_WAITING, CL_WAITING_CANCEL,
    CL_WAITING_LOGIN, LOAD_BALANCE_HOSTS_DISABLE, LOAD_BALANCE_HOSTS_ROUND_ROBIN, MAX_PASSWORD,
    MAX_USERNAME, POOL_INHERIT, REPLICATION_LOGICAL, REPLICATION_NONE, REPLICATION_PHYSICAL,
    SV_ACTIVE, SV_ACTIVE_CANCEL, SV_BEING_CANCELED, SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN,
    SV_TESTED, SV_USED,
};
pub use self::cfparser_h::{cf_set_lookup, CfLookup, CfValue};
pub use self::ctype_h::safe_isspace;
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
    __darwin_ct_rune_t, __darwin_ptrdiff_t, __darwin_rune_t, __darwin_size_t, __darwin_ssize_t,
    __darwin_time_t, __darwin_wchar_t, __int32_t, __int64_t, __uint16_t, __uint32_t, __uint64_t,
    __uint8_t,
};
pub use crate::types::{aatree_cmp_f, aatree_walker_f, AANode, AATree};
pub use crate::types::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};

use self::errno_h::__error;

pub use self::event_struct_h::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
use self::fileutil_h::load_file;
pub use crate::types::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use crate::types::{in_addr, sockaddr_in};
pub use self::iobuf_h::{iobuf, IOBuf};
pub use crate::types::{
    cf_verbose, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE, LG_STATS,
    LG_WARNING,
};
use self::objects_h::{
    add_database, add_peer, find_or_add_new_global_credentials, find_or_add_new_global_user,
    force_user_credentials, tag_autodb_dirty, tag_database_dirty, update_global_user_passwd,
    user_list,
};
pub use self::pktbuf_h::{pktbuf_dynamic, pktbuf_put_string, pktbuf_reset, PktBuf};
pub use self::runetype_h::{
    _DefaultRuneLocale, _RuneCharClass, _RuneEntry, _RuneLocale, _RuneRange,
};
pub use self::sbuf_h::{
    sbuf_cb_t, SBuf, SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK, SBUF_EV_FLUSH,
    SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY,
};
pub use crate::types::sockaddr;
pub use self::stat_h::stat;
use self::string_h::strcmpeq;
pub use crate::types::{
    __darwin_blkcnt_t, __darwin_blksize_t, __darwin_dev_t, __darwin_gid_t, __darwin_ino64_t,
    __darwin_mode_t, __darwin_off_t, __darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t,
    __DARWIN_NULL,
};
pub use crate::types::List;
pub use crate::types::MBuf;
pub use crate::types::PktHdr;
pub use crate::types::StatList;
pub use crate::types::{false_0, true_0};
pub use crate::types::{usec_t, USEC};
pub use crate::types::{PStr, StrPool};
pub use crate::types::{PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement};

use self::util_h::check_reserved_database;
pub use crate::types::VarCache;
pub use crate::types::{UT_hash_bucket, UT_hash_handle, UT_hash_table};
#[no_mangle]

pub static mut any_user_level_timeout_set: bool = false;
#[no_mangle]

pub static mut any_user_level_client_timeout_set: bool = false;

unsafe extern "C" fn cstr_get_key(
    mut p: *mut ::core::ffi::c_char,
    mut dst_p: *mut *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut end = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = cstr_skip_ws(p);
    *dst_p = p;
    while *p as ::core::ffi::c_int != 0
        && *p as ::core::ffi::c_int != '=' as i32
        && *p as ::core::ffi::c_int != ' ' as i32
    {
        p = p.offset(1);
    }
    end = p;
    p = cstr_skip_ws(p);
    if *p as ::core::ffi::c_int != '=' as i32 || *dst_p == end {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    *end = 0 as ::core::ffi::c_char;
    p.offset(1 as ::core::ffi::c_int as isize)
}

unsafe extern "C" fn cstr_unquote_value(
    mut p: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut s = p;
    loop {
        if *p == 0 {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if *p as ::core::ffi::c_int == '\'' as i32 {
            if *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\'' as i32 {
                break;
            }
            p = p.offset(1);
        }
        let fresh0 = p;
        p = p.offset(1);
        let fresh1 = s;
        s = s.offset(1);
        *fresh1 = *fresh0;
    }
    *s = 0 as ::core::ffi::c_char;
    p.offset(1 as ::core::ffi::c_int as isize)
}

unsafe extern "C" fn cstr_get_value(
    mut p: *mut ::core::ffi::c_char,
    mut dst_p: *mut *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    p = cstr_skip_ws(p);
    if *p as ::core::ffi::c_int == '\'' as i32 {
        p = p.offset(1);
        *dst_p = p;
        p = cstr_unquote_value(p);
        if p.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    } else {
        *dst_p = p;
        while *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int != ' ' as i32 {
            p = p.offset(1);
        }
    }
    if *p != 0 {
        *p = 0 as ::core::ffi::c_char;
        p = p.offset(1);
    }
    if **dst_p as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    p
}

unsafe extern "C" fn cstr_get_pair(
    mut p: *mut ::core::ffi::c_char,
    mut key_p: *mut *mut ::core::ffi::c_char,
    mut val_p: *mut *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    p = cstr_skip_ws(p);
    *val_p = p;
    *key_p = *val_p;
    if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return p;
    }
    p = cstr_get_key(p, key_p);
    if p.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    p = cstr_get_value(p, val_p);
    if p.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut _log_ctx = NULL;
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(
            LG_NOISE,
            _log_ctx,
            b"cstr_get_pair: \"%s\"=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            *key_p,
            *val_p,
        );
    }
    cstr_skip_ws(p)
}

unsafe extern "C" fn set_param_value(
    mut old_value: *mut *mut ::core::ffi::c_char,
    mut new_value: *const ::core::ffi::c_char,
) -> bool {
    if strcmpeq(*old_value, new_value) {
        return true;
    }
    if !(*old_value).is_null() {
        free(*old_value as *mut ::core::ffi::c_void);
    }
    if !new_value.is_null() {
        *old_value = strdup(new_value);
        if (*old_value).is_null() {
            let mut _log_ctx = NULL;
            log_generic(LG_ERROR, _log_ctx, c"out of memory".as_ptr());
            return false;
        }
    } else {
        *old_value = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    true
}

unsafe extern "C" fn set_autodb(mut connstr: *const ::core::ffi::c_char) -> bool {
    let mut tmp = strdup(connstr);
    let mut old = cf_autodb_connstr;
    if tmp.is_null() {
        let mut _log_ctx = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx,
            c"no mem to change autodb_connstr".as_ptr(),
        );
        return false;
    }
    cf_autodb_connstr = tmp;
    if !old.is_null() {
        if strcmp(connstr, old) != 0 as ::core::ffi::c_int {
            tag_autodb_dirty();
        }
        free(old as *mut ::core::ffi::c_void);
    }
    true
}
#[no_mangle]

pub unsafe extern "C" fn parse_peer(
    mut _base: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut connstr: *const ::core::ffi::c_char,
) -> bool {
    let mut current_block: u64;
    let mut p = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut peer = ::core::ptr::null_mut::<PgDatabase>();
    let mut tmp_connstr = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut host = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut port = 6432 as ::core::ffi::c_int;
    let mut pool_size = -(1 as ::core::ffi::c_int);
    let mut peer_id = strtonum(
        name,
        1 as ::core::ffi::c_longlong,
        0xffff as ::core::ffi::c_longlong,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) as ::core::ffi::c_int;
    if peer_id == 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx,
            c"ids of peers must be a number larger than 0 and at most 65536".as_ptr(),
        );
        return false;
    }
    tmp_connstr = strdup(connstr);
    if tmp_connstr.is_null() {
        let mut _log_ctx_0 = NULL;
        log_generic(LG_ERROR, _log_ctx_0, c"out of memory".as_ptr());
        return false;
    }
    p = tmp_connstr;
    loop {
        if *p == 0 {
            current_block = 6417057564578538666;
            break;
        }
        p = cstr_get_pair(p, &raw mut key, &raw mut val);
        if p.is_null() {
            let mut _log_ctx_1 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_1,
                c"syntax error in connection string".as_ptr(),
            );
            current_block = 1844502831972581562;
            break;
        } else {
            if *key == 0 {
                current_block = 6417057564578538666;
                break;
            }
            if strcmp(c"host".as_ptr(), key) == 0 as ::core::ffi::c_int {
                if !set_param_value(&raw mut host, val) {
                    current_block = 1844502831972581562;
                    break;
                }
            } else if strcmp(c"port".as_ptr(), key) == 0 as ::core::ffi::c_int {
                port = atoi(val);
                if port != 0 as ::core::ffi::c_int {
                    continue;
                }
                let mut _log_ctx_2 = NULL;
                log_generic(LG_ERROR, _log_ctx_2, c"invalid port: %s".as_ptr(), val);
                current_block = 1844502831972581562;
                break;
            } else if strcmp(c"pool_size".as_ptr(), key) == 0 as ::core::ffi::c_int {
                pool_size = atoi(val);
            } else {
                let mut _log_ctx_3 = NULL;
                log_generic(
                    LG_ERROR,
                    _log_ctx_3,
                    c"unrecognized connection parameter: %s".as_ptr(),
                    key,
                );
                current_block = 1844502831972581562;
                break;
            }
        }
    }
    if current_block == 6417057564578538666 {
        if host.is_null() {
            let mut _log_ctx_4 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_4,
                c"host was not provided for peer %d".as_ptr(),
                peer_id,
            );
        } else {
            peer = add_peer(name, peer_id);
            if peer.is_null() {
                let mut _log_ctx_5 = NULL;
                log_generic(
                    LG_ERROR,
                    _log_ctx_5,
                    c"cannot create peer, no memory?".as_ptr(),
                );
            } else {
                (*peer).db_dead = false;
                free((*peer).host as *mut ::core::ffi::c_void);
                (*peer).host = host;
                (*peer).port = port;
                (*peer).pool_size = pool_size;
                free(tmp_connstr as *mut ::core::ffi::c_void);
                return true;
            }
        }
    }
    free(tmp_connstr as *mut ::core::ffi::c_void);
    free(host as *mut ::core::ffi::c_void);
    false
}
#[no_mangle]

pub unsafe extern "C" fn parse_database(
    mut _base: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut connstr: *const ::core::ffi::c_char,
) -> bool {
    let mut current_block: u64;
    let mut p = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut msg = ::core::ptr::null_mut::<PktBuf>();
    let mut db = ::core::ptr::null_mut::<PgDatabase>();
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
    let mut pool_size = -(1 as ::core::ffi::c_int);
    let mut min_pool_size = -(1 as ::core::ffi::c_int);
    let mut res_pool_size = -(1 as ::core::ffi::c_int);
    let mut max_db_client_connections = -(1 as ::core::ffi::c_int);
    let mut max_db_connections = -(1 as ::core::ffi::c_int);
    let mut server_lifetime: usec_t = 0;
    let mut dbname_ofs: ::core::ffi::c_int = 0;
    let mut pool_mode = POOL_INHERIT;
    let mut load_balance_hosts = LOAD_BALANCE_HOSTS_ROUND_ROBIN;
    let mut tmp_connstr = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut dbname = name;
    let mut host = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut port = 5432 as ::core::ffi::c_int;
    let mut username = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut password = c"".as_ptr() as *mut ::core::ffi::c_char;
    let mut auth_username = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut auth_dbname = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut client_encoding = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut datestyle = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut timezone = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut connect_query = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut appname = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut auth_query = ::core::ptr::null_mut::<::core::ffi::c_char>();
    cv.value_p = &raw mut pool_mode as *mut ::core::ffi::c_void;
    cv.extra = &raw const pool_mode_map as *const CfLookup as *const ::core::ffi::c_void;
    load_balance_hosts_lookup.value_p = &raw mut load_balance_hosts as *mut ::core::ffi::c_void;
    load_balance_hosts_lookup.extra =
        &raw const load_balance_hosts_map as *const CfLookup as *const ::core::ffi::c_void;
    if !check_reserved_database(name) {
        let mut _log_ctx = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx,
            b"database name \"%s\" is reserved\0" as *const u8 as *const ::core::ffi::c_char,
            name,
        );
        return false;
    }
    if strcmp(name, c"*".as_ptr()) == 0 as ::core::ffi::c_int {
        return set_autodb(connstr);
    }
    tmp_connstr = strdup(connstr);
    if tmp_connstr.is_null() {
        let mut _log_ctx_0 = NULL;
        log_generic(LG_ERROR, _log_ctx_0, c"out of memory".as_ptr());
        return false;
    }
    p = tmp_connstr;
    loop {
        if *p == 0 {
            current_block = 2705889988320590074;
            break;
        }
        p = cstr_get_pair(p, &raw mut key, &raw mut val);
        if p.is_null() {
            let mut _log_ctx_1 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_1,
                c"syntax error in connection string".as_ptr(),
            );
            current_block = 17902695515181753043;
            break;
        } else {
            if *key == 0 {
                current_block = 2705889988320590074;
                break;
            }
            if strcmp(c"dbname".as_ptr(), key) == 0 as ::core::ffi::c_int {
                dbname = val;
            } else if strcmp(c"host".as_ptr(), key) == 0 as ::core::ffi::c_int {
                if !set_param_value(&raw mut host, val) {
                    current_block = 17902695515181753043;
                    break;
                }
            } else if strcmp(c"port".as_ptr(), key) == 0 as ::core::ffi::c_int {
                port = atoi(val);
                if port != 0 as ::core::ffi::c_int {
                    continue;
                }
                let mut _log_ctx_2 = NULL;
                log_generic(LG_ERROR, _log_ctx_2, c"invalid port: %s".as_ptr(), val);
                current_block = 17902695515181753043;
                break;
            } else if strcmp(c"user".as_ptr(), key) == 0 as ::core::ffi::c_int {
                username = val;
            } else if strcmp(c"password".as_ptr(), key) == 0 as ::core::ffi::c_int {
                password = val;
            } else if strcmp(c"auth_user".as_ptr(), key) == 0 as ::core::ffi::c_int {
                auth_username = val;
            } else if strcmp(c"auth_dbname".as_ptr(), key) == 0 as ::core::ffi::c_int {
                auth_dbname = val;
            } else if strcmp(c"client_encoding".as_ptr(), key) == 0 as ::core::ffi::c_int {
                client_encoding = val;
            } else if strcmp(c"datestyle".as_ptr(), key) == 0 as ::core::ffi::c_int {
                datestyle = val;
            } else if strcmp(c"timezone".as_ptr(), key) == 0 as ::core::ffi::c_int {
                timezone = val;
            } else if strcmp(c"pool_size".as_ptr(), key) == 0 as ::core::ffi::c_int {
                pool_size = atoi(val);
            } else if strcmp(c"min_pool_size".as_ptr(), key) == 0 as ::core::ffi::c_int {
                min_pool_size = atoi(val);
            } else if strcmp(c"reserve_pool".as_ptr(), key) == 0 as ::core::ffi::c_int {
                res_pool_size = atoi(val);
            } else if strcmp(c"reserve_pool_size".as_ptr(), key) == 0 as ::core::ffi::c_int {
                res_pool_size = atoi(val);
            } else if strcmp(c"max_db_connections".as_ptr(), key) == 0 as ::core::ffi::c_int {
                max_db_connections = atoi(val);
            } else if strcmp(c"max_db_client_connections".as_ptr(), key) == 0 as ::core::ffi::c_int
            {
                max_db_client_connections = atoi(val);
            } else if strcmp(c"server_lifetime".as_ptr(), key) == 0 as ::core::ffi::c_int {
                server_lifetime = (atoi(val) as usec_t).wrapping_mul(USEC);
            } else if strcmp(c"load_balance_hosts".as_ptr(), key) == 0 as ::core::ffi::c_int {
                if cf_set_lookup(&raw mut load_balance_hosts_lookup, val) {
                    continue;
                }
                let mut _log_ctx_3 = NULL;
                log_generic(
                    LG_ERROR,
                    _log_ctx_3,
                    c"invalid load_balance_hosts: %s".as_ptr(),
                    val,
                );
                current_block = 17902695515181753043;
                break;
            } else if strcmp(c"pool_mode".as_ptr(), key) == 0 as ::core::ffi::c_int {
                if cf_set_lookup(&raw mut cv, val) {
                    continue;
                }
                let mut _log_ctx_4 = NULL;
                log_generic(LG_ERROR, _log_ctx_4, c"invalid pool mode: %s".as_ptr(), val);
                current_block = 17902695515181753043;
                break;
            } else if strcmp(c"connect_query".as_ptr(), key) == 0 as ::core::ffi::c_int {
                if !set_param_value(&raw mut connect_query, val) {
                    current_block = 17902695515181753043;
                    break;
                }
            } else if strcmp(c"application_name".as_ptr(), key) == 0 as ::core::ffi::c_int {
                appname = val;
            } else if strcmp(c"auth_query".as_ptr(), key) == 0 as ::core::ffi::c_int {
                auth_query = val;
            } else {
                let mut _log_ctx_5 = NULL;
                log_generic(
                    LG_ERROR,
                    _log_ctx_5,
                    c"unrecognized connection parameter: %s".as_ptr(),
                    key,
                );
                current_block = 17902695515181753043;
                break;
            }
        }
    }
    if current_block == 2705889988320590074 {
        db = add_database(name);
        if db.is_null() {
            let mut _log_ctx_6 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_6,
                c"cannot create database, no memory?".as_ptr(),
            );
        } else {
            (*db).db_dead = false;
            (*db).db_auto = false;
            (*db).inactive_time = 0;
            if !(*db).dbname.is_null() {
                let mut changed = false;
                if strcmp((*db).dbname, dbname) != 0 as ::core::ffi::c_int {
                    changed = true;
                } else if !strcmpeq(host, (*db).host) {
                    changed = true;
                } else if port != (*db).port {
                    changed = true;
                } else if !username.is_null() && (*db).forced_user_credentials.is_null() {
                    changed = true;
                } else if !username.is_null()
                    && strcmp(
                        username,
                        &raw mut (*(*db).forced_user_credentials).name as *mut ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                {
                    changed = true;
                } else if username.is_null() && !(*db).forced_user_credentials.is_null() {
                    changed = true;
                } else if !strcmpeq(connect_query, (*db).connect_query) {
                    changed = true;
                } else if !strcmpeq((*db).auth_dbname, auth_dbname) {
                    changed = true;
                } else if !strcmpeq((*db).auth_query, auth_query) {
                    changed = true;
                } else if load_balance_hosts as ::core::ffi::c_uint
                    != (*db).load_balance_hosts as ::core::ffi::c_uint
                {
                    changed = true;
                }
                if changed {
                    tag_database_dirty(db);
                }
            }
            free((*db).host as *mut ::core::ffi::c_void);
            (*db).host = host;
            host = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*db).port = port;
            (*db).pool_size = pool_size;
            (*db).min_pool_size = min_pool_size;
            (*db).res_pool_size = res_pool_size;
            (*db).pool_mode = pool_mode;
            (*db).max_db_client_connections = max_db_client_connections;
            (*db).max_db_connections = max_db_connections;
            (*db).server_lifetime = server_lifetime;
            (*db).load_balance_hosts = load_balance_hosts;
            free((*db).connect_query as *mut ::core::ffi::c_void);
            (*db).connect_query = connect_query;
            connect_query = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if set_param_value(&raw mut (*db).auth_dbname, auth_dbname)
                && set_param_value(&raw mut (*db).auth_query, auth_query)
            {
                if !(*db).startup_params.is_null() {
                    msg = (*db).startup_params as *mut PktBuf;
                    pktbuf_reset(msg as *mut PktBuf);
                } else {
                    msg = pktbuf_dynamic(128 as ::core::ffi::c_int);
                    if msg.is_null() {
                        let mut _log_ctx_7 = NULL;
                        log_generic(LG_FATAL, _log_ctx_7, c"out of memory".as_ptr());
                        exit(1 as ::core::ffi::c_int);
                    }
                    (*db).startup_params = msg as *mut PktBuf;
                }
                pktbuf_put_string(msg, c"database".as_ptr());
                dbname_ofs = (*msg).write_pos;
                pktbuf_put_string(msg, dbname);
                if !client_encoding.is_null() {
                    pktbuf_put_string(msg, c"client_encoding".as_ptr());
                    pktbuf_put_string(msg, client_encoding);
                }
                if !datestyle.is_null() {
                    pktbuf_put_string(msg, c"datestyle".as_ptr());
                    pktbuf_put_string(msg, datestyle);
                }
                if !timezone.is_null() {
                    pktbuf_put_string(msg, c"timezone".as_ptr());
                    pktbuf_put_string(msg, timezone);
                }
                if !appname.is_null() {
                    pktbuf_put_string(msg, c"application_name".as_ptr());
                    pktbuf_put_string(msg, appname);
                }
                if !auth_username.is_null() {
                    (*db).auth_user_credentials =
                        find_or_add_new_global_credentials(auth_username, c"".as_ptr());
                    if (*db).auth_user_credentials.is_null() {
                        current_block = 17902695515181753043;
                    } else {
                        current_block = 10405920230221879620;
                    }
                } else {
                    if !(*db).auth_user_credentials.is_null() {
                        (*db).auth_user_credentials = ::core::ptr::null_mut::<PgCredentials>();
                    }
                    current_block = 10405920230221879620;
                }
                match current_block {
                    17902695515181753043 => {}
                    _ => {
                        if !username.is_null() {
                            if force_user_credentials(db, username, password).is_null() {
                                let mut _log_ctx_8 = NULL;
                                log_generic(
                                    LG_WARNING,
                                    _log_ctx_8,
                                    c"db setup failed, trying to continue".as_ptr(),
                                );
                            }
                        } else if !(*db).forced_user_credentials.is_null() {
                            let mut _log_ctx_9 = NULL;
                            log_generic(
                                LG_WARNING,
                                _log_ctx_9,
                                c"losing forced user not supported, keeping old setting".as_ptr(),
                            );
                        }
                        (*db).dbname =
                            ((*msg).buf as *mut ::core::ffi::c_char).offset(dbname_ofs as isize);
                        free(tmp_connstr as *mut ::core::ffi::c_void);
                        return true;
                    }
                }
            }
        }
    }
    free(tmp_connstr as *mut ::core::ffi::c_void);
    free(host as *mut ::core::ffi::c_void);
    free(connect_query as *mut ::core::ffi::c_void);
    false
}
#[no_mangle]

pub unsafe extern "C" fn parse_user(
    mut _base: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut connstr: *const ::core::ffi::c_char,
) -> bool {
    let mut current_block: u64;
    let mut p = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut tmp_connstr = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut user = ::core::ptr::null_mut::<PgGlobalUser>();
    let mut cv = CfValue {
        value_p: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        extra: ::core::ptr::null::<::core::ffi::c_void>(),
        key_name: ::core::ptr::null::<::core::ffi::c_char>(),
        buf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        buflen: 0,
    };
    let mut pool_mode = POOL_INHERIT;
    let mut pool_size = -(1 as ::core::ffi::c_int);
    let mut res_pool_size = -(1 as ::core::ffi::c_int);
    let mut max_user_connections = -(1 as ::core::ffi::c_int);
    let mut idle_transaction_timeout: usec_t = 0;
    let mut transaction_timeout: usec_t = 0;
    let mut query_timeout: usec_t = 0;
    let mut client_idle_timeout: usec_t = 0;
    let mut max_user_client_connections = -(1 as ::core::ffi::c_int);
    cv.value_p = &raw mut pool_mode as *mut ::core::ffi::c_void;
    cv.extra = &raw const pool_mode_map as *const CfLookup as *const ::core::ffi::c_void;
    tmp_connstr = strdup(connstr);
    if tmp_connstr.is_null() {
        let mut _log_ctx = NULL;
        log_generic(LG_ERROR, _log_ctx, c"out of memory".as_ptr());
        return false;
    }
    p = tmp_connstr;
    loop {
        if *p == 0 {
            current_block = 7990025728955927862;
            break;
        }
        p = cstr_get_pair(p, &raw mut key, &raw mut val);
        if p.is_null() {
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_0,
                c"syntax error in user settings".as_ptr(),
            );
            current_block = 15018837909795791711;
            break;
        } else {
            if *key == 0 {
                current_block = 7990025728955927862;
                break;
            }
            if strcmp(c"pool_mode".as_ptr(), key) == 0 as ::core::ffi::c_int {
                if cf_set_lookup(&raw mut cv, val) {
                    continue;
                }
                let mut _log_ctx_1 = NULL;
                log_generic(LG_ERROR, _log_ctx_1, c"invalid pool mode: %s".as_ptr(), val);
                current_block = 15018837909795791711;
                break;
            } else if strcmp(c"pool_size".as_ptr(), key) == 0 as ::core::ffi::c_int {
                pool_size = atoi(val);
            } else if strcmp(c"reserve_pool_size".as_ptr(), key) == 0 as ::core::ffi::c_int {
                res_pool_size = atoi(val);
            } else if strcmp(c"max_user_connections".as_ptr(), key) == 0 as ::core::ffi::c_int {
                max_user_connections = atoi(val);
            } else if strcmp(c"transaction_timeout".as_ptr(), key) == 0 as ::core::ffi::c_int {
                any_user_level_timeout_set = true;
                transaction_timeout = (atoi(val) as usec_t).wrapping_mul(USEC);
            } else if strcmp(c"idle_transaction_timeout".as_ptr(), key) == 0 as ::core::ffi::c_int {
                any_user_level_timeout_set = true;
                idle_transaction_timeout = (atoi(val) as usec_t).wrapping_mul(USEC);
            } else if strcmp(c"query_timeout".as_ptr(), key) == 0 as ::core::ffi::c_int {
                any_user_level_timeout_set = true;
                query_timeout = (atoi(val) as usec_t).wrapping_mul(USEC);
            } else if strcmp(c"client_idle_timeout".as_ptr(), key) == 0 as ::core::ffi::c_int {
                any_user_level_client_timeout_set = true;
                client_idle_timeout = (atoi(val) as usec_t).wrapping_mul(USEC);
            } else if strcmp(c"max_user_client_connections".as_ptr(), key)
                == 0 as ::core::ffi::c_int
            {
                max_user_client_connections = atoi(val);
            } else {
                let mut _log_ctx_2 = NULL;
                log_generic(
                    LG_ERROR,
                    _log_ctx_2,
                    c"unrecognized user parameter: %s".as_ptr(),
                    key,
                );
                current_block = 15018837909795791711;
                break;
            }
        }
    }
    if current_block == 7990025728955927862 {
        user = find_or_add_new_global_user(name, c"".as_ptr());
        if user.is_null() {
            let mut _log_ctx_3 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_3,
                c"cannot create user, no memory?".as_ptr(),
            );
        } else {
            (*user).pool_mode = pool_mode;
            (*user).pool_size = pool_size;
            (*user).res_pool_size = res_pool_size;
            (*user).max_user_connections = max_user_connections;
            (*user).idle_transaction_timeout = idle_transaction_timeout;
            (*user).transaction_timeout = transaction_timeout;
            (*user).query_timeout = query_timeout;
            (*user).client_idle_timeout = client_idle_timeout;
            (*user).max_user_client_connections = max_user_client_connections;
            free(tmp_connstr as *mut ::core::ffi::c_void);
            return true;
        }
    }
    free(tmp_connstr as *mut ::core::ffi::c_void);
    false
}

unsafe extern "C" fn find_quote(
    mut p: *mut ::core::ffi::c_char,
    mut start: bool,
) -> *mut ::core::ffi::c_char {
    loop {
        while *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int != '"' as i32 {
            p = p.offset(1);
        }
        if !(*p as ::core::ffi::c_int == '"' as i32
            && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '"' as i32
            && !start)
        {
            break;
        }
        p = p.offset(2 as ::core::ffi::c_int as isize);
    }
    p
}

unsafe extern "C" fn copy_quoted(
    mut dst: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) {
    let mut end = dst
        .offset(len as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    while *src as ::core::ffi::c_int != 0 && dst < end {
        if *src as ::core::ffi::c_int == '"' as i32 {
            src = src.offset(1);
        }
        let fresh4 = src;
        src = src.offset(1);
        let fresh5 = dst;
        dst = dst.offset(1);
        *fresh5 = *fresh4;
    }
    *dst = 0 as ::core::ffi::c_char;
}

unsafe extern "C" fn unquote_add_authfile_user(
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
) {
    let mut real_user: [::core::ffi::c_char; 128] = [0; 128];
    let mut real_passwd: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut user = ::core::ptr::null_mut::<PgGlobalUser>();
    copy_quoted(
        &raw mut real_user as *mut ::core::ffi::c_char,
        username,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as ::core::ffi::c_int,
    );
    copy_quoted(
        &raw mut real_passwd as *mut ::core::ffi::c_char,
        password,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as ::core::ffi::c_int,
    );
    user = find_or_add_new_global_user(
        &raw mut real_user as *mut ::core::ffi::c_char,
        &raw mut real_passwd as *mut ::core::ffi::c_char,
    );
    if user.is_null() {
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            c"cannot create user, no memory".as_ptr(),
        );
        return;
    }
    if strcmp(
        &raw mut (*user).credentials.passwd as *mut ::core::ffi::c_char,
        &raw mut real_passwd as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        user = update_global_user_passwd(user, &raw mut real_passwd as *mut ::core::ffi::c_char);
    }
    (*user).credentials.dynamic_passwd = false;
    if !(*user).credentials.scram_SaltKey.is_null() {
        free((*user).credentials.scram_SaltKey as *mut ::core::ffi::c_void);
        (*user).credentials.scram_SaltKey = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*user).credentials.adhoc_scram_secrets_cached = false;
    }
}

unsafe extern "C" fn auth_loaded(mut fn_0: *const ::core::ffi::c_char) -> bool {
    static mut cache_set: bool = false;
    static mut cache: stat = stat {
        st_dev: 0,
        st_mode: 0,
        st_nlink: 0,
        st_ino: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_atimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_birthtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_size: 0,
        st_blocks: 0,
        st_blksize: 0,
        st_flags: 0,
        st_gen: 0,
        st_lspare: 0,
        st_qspare: [0; 2],
    };
    let mut cur = stat {
        st_dev: 0,
        st_mode: 0,
        st_nlink: 0,
        st_ino: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_atimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_birthtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_size: 0,
        st_blocks: 0,
        st_blksize: 0,
        st_flags: 0,
        st_gen: 0,
        st_lspare: 0,
        st_qspare: [0; 2],
    };
    if fn_0.is_null() {
        memset(
            &raw mut cache as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<stat>() as size_t,
        );
        cache_set = true;
        return false;
    }
    if stat(fn_0, &raw mut cur) < 0 as ::core::ffi::c_int {
        memset(
            &raw mut cur as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<stat>() as size_t,
        );
    }
    if cache_set
        && cache.st_dev == cur.st_dev
        && cache.st_ino == cur.st_ino
        && cache.st_mode as ::core::ffi::c_int == cur.st_mode as ::core::ffi::c_int
        && cache.st_uid == cur.st_uid
        && cache.st_gid == cur.st_gid
        && cache.st_mtimespec.tv_sec == cur.st_mtimespec.tv_sec
        && cache.st_size == cur.st_size
    {
        return true;
    }
    cache = cur;
    cache_set = true;
    false
}
#[no_mangle]

pub unsafe extern "C" fn loader_users_check() -> bool {
    if auth_loaded(cf_auth_file) {
        return true;
    }
    load_auth_file(cf_auth_file)
}

unsafe extern "C" fn disable_users() {
    let mut item = ::core::ptr::null_mut::<List>();
    item = user_list.head.next;
    while item != &raw mut user_list.head {
        let mut user = (item as *mut ::core::ffi::c_char)
            .offset(-(2336 as ::core::ffi::c_ulong as isize))
            as *mut PgGlobalUser;
        (*user).credentials.passwd[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        item = (*item).next;
    }
}
#[no_mangle]

pub unsafe extern "C" fn load_auth_file(mut fn_0: *const ::core::ffi::c_char) -> bool {
    let mut user = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut password = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut buf = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if fn_0.is_null() {
        return false;
    }
    buf = load_file(fn_0, ::core::ptr::null_mut::<size_t>()) as *mut ::core::ffi::c_char;
    if buf.is_null() {
        let mut _log_ctx = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx,
            c"could not open auth_file %s: %s".as_ptr(),
            fn_0,
            strerror(*__error()),
        );
        return false;
    }
    let mut _log_ctx_0 = NULL;
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            _log_ctx_0,
            b"loading auth_file: \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            fn_0,
        );
    }
    disable_users();
    p = buf;
    while *p != 0 {
        while *p as ::core::ffi::c_int != 0 && safe_isspace(*p as ::core::ffi::c_int) != 0 {
            p = p.offset(1);
        }
        if *p == 0 {
            break;
        }
        if *p as ::core::ffi::c_int == ';' as i32 {
            while *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int != '\n' as i32 {
                p = p.offset(1);
            }
        } else if *p as ::core::ffi::c_int != '"' as i32 {
            let mut _log_ctx_1 = NULL;
            log_generic(LG_ERROR, _log_ctx_1, c"broken auth file".as_ptr());
            break;
        } else {
            p = p.offset(1);
            user = p;
            p = find_quote(p, false);
            if *p as ::core::ffi::c_int != '"' as i32 {
                let mut _log_ctx_2 = NULL;
                log_generic(LG_ERROR, _log_ctx_2, c"broken auth file".as_ptr());
                break;
            } else if p.offset_from(user) as ::core::ffi::c_long
                >= MAX_USERNAME as ::core::ffi::c_long
            {
                let mut _log_ctx_3 = NULL;
                log_generic(
                    LG_ERROR,
                    _log_ctx_3,
                    c"username too long in auth file".as_ptr(),
                );
                break;
            } else {
                let fresh2 = p;
                p = p.offset(1);
                *fresh2 = 0 as ::core::ffi::c_char;
                p = find_quote(p, true);
                if *p as ::core::ffi::c_int != '"' as i32 {
                    let mut _log_ctx_4 = NULL;
                    log_generic(LG_ERROR, _log_ctx_4, c"broken auth file".as_ptr());
                    break;
                } else {
                    p = p.offset(1);
                    password = p;
                    p = find_quote(p, false);
                    if *p as ::core::ffi::c_int != '"' as i32 {
                        let mut _log_ctx_5 = NULL;
                        log_generic(LG_ERROR, _log_ctx_5, c"broken auth file".as_ptr());
                        break;
                    } else if p.offset_from(password) as ::core::ffi::c_long
                        >= MAX_PASSWORD as ::core::ffi::c_long
                    {
                        let mut _log_ctx_6 = NULL;
                        log_generic(
                            LG_ERROR,
                            _log_ctx_6,
                            c"password too long in auth file".as_ptr(),
                        );
                        break;
                    } else {
                        let fresh3 = p;
                        p = p.offset(1);
                        *fresh3 = 0 as ::core::ffi::c_char;
                        unquote_add_authfile_user(user, password);
                        while *p as ::core::ffi::c_int != 0
                            && *p as ::core::ffi::c_int != '\n' as i32
                        {
                            p = p.offset(1);
                        }
                    }
                }
            }
        }
    }
    free(buf as *mut ::core::ffi::c_void);
    true
}
