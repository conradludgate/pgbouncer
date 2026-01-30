pub mod sys__types_h {

    pub type __darwin_pid_t = __int32_t;

    pub type __darwin_suseconds_t = __int32_t;

    pub type __darwin_uid_t = __uint32_t;

    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    use crate::types::{__int32_t, __uint32_t};
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
    use crate::types::{__darwin_rune_t, __darwin_size_t, __uint32_t};
    extern "C" {

        pub static mut _DefaultRuneLocale: _RuneLocale;
    }
}

pub mod tls_h {
    use crate::types::size_t;
    use crate::types::ssize_t;
    extern "C" {

        pub type tls;

        pub fn tls_peer_cert_provided(_ctx: *mut tls) -> ::core::ffi::c_int;

        pub fn tls_peer_cert_contains_name(
            _ctx: *mut tls,
            _name: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;

        pub fn tls_peer_cert_subject(_ctx: *mut tls) -> *const ::core::ffi::c_char;

        pub fn tls_get_connection_info(
            ctx: *mut tls,
            buf: *mut ::core::ffi::c_char,
            buflen: size_t,
        ) -> ssize_t;
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

    pub const AF_UNIX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
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

    pub const AUTH_TYPE_MD5: auth_type = 3;

    pub const AUTH_TYPE_PLAIN: auth_type = 2;

    pub const AUTH_TYPE_PAM: auth_type = 7;

    pub const AUTH_TYPE_LDAP: auth_type = 6;

    pub const AUTH_TYPE_SCRAM_SHA_256: auth_type = 8;

    pub const AUTH_TYPE_PEER: auth_type = 9;

    pub const AUTH_TYPE_CERT: auth_type = 4;

    pub const AUTH_TYPE_TRUST: auth_type = 1;

    pub const AUTH_TYPE_ANY: auth_type = 0;

    pub const AUTH_TYPE_HBA: auth_type = 5;

    pub type auth_type = ::core::ffi::c_uint;

    pub const AUTH_TYPE_REJECT: auth_type = 10;

    pub const MAX_USERNAME: ::core::ffi::c_int = 128 as ::core::ffi::c_int;

    pub const MAX_PASSWORD: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;

    pub const PKT_STARTUP_V2: ::core::ffi::c_uint = 131072 as ::core::ffi::c_uint;

    pub const PKT_STARTUP_V3: ::core::ffi::c_int = 0x30000 as ::core::ffi::c_int;

    pub const PKT_STARTUP_V3_UNSUPPORTED: ::core::ffi::c_int = 0x30001 as ::core::ffi::c_int;

    pub const PKT_CANCEL: ::core::ffi::c_uint = 80877102 as ::core::ffi::c_uint;

    pub const PKT_SSLREQ: ::core::ffi::c_int = 80877103 as ::core::ffi::c_int;

    pub const PKT_GSSENCREQ: ::core::ffi::c_int = 80877104 as ::core::ffi::c_int;

    pub const POOL_SESSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const BACKENDKEY_LEN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    #[inline]

    pub unsafe extern "C" fn pga_is_unix(mut a: *const PgAddr) -> bool {
        (*a).sa.sa_family as ::core::ffi::c_int == AF_UNIX
    }
    #[inline]

    pub unsafe extern "C" fn cstr_skip_ws(
        mut p: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char {
        while *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int == ' ' as i32 {
            p = p.offset(1);
        }
        p
    }
    use crate::types::pid_t;

    use super::dnslookup_h::DNSToken;
    use super::hba_h::HBA;
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
    use crate::types::StatList;
    use crate::types::VarCache;
    use crate::types::{AANode, AATree};
    use crate::types::{PgClientPreparedStatement, PgServerPreparedStatement};
    extern "C" {

        pub static mut cf_sbuf_len: ::core::ffi::c_int;

        pub fn pga_details(
            a: *const PgAddr,
            dst: *mut ::core::ffi::c_char,
            dstlen: ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char;

        pub static mut replication_type_parameters: [*const ::core::ffi::c_char; 3];

        pub static mut cf_max_client_conn: ::core::ffi::c_int;

        pub static mut cf_disable_pqexec: ::core::ffi::c_int;

        pub static mut cf_auth_type: ::core::ffi::c_int;

        pub static mut cf_auth_query: *mut ::core::ffi::c_char;

        pub static mut cf_auth_user: *mut ::core::ffi::c_char;

        pub static mut cf_auth_dbname: *mut ::core::ffi::c_char;

        pub static mut cf_ignore_startup_params: *mut ::core::ffi::c_char;

        pub static mut cf_admin_users: *mut ::core::ffi::c_char;

        pub static mut cf_log_connections: ::core::ffi::c_int;

        pub static mut cf_application_name_add_host: ::core::ffi::c_int;

        pub static mut cf_max_prepared_statements: ::core::ffi::c_int;

        pub static mut parsed_hba: *mut HBA;
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
    use super::tls_h::tls;
    use crate::types::size_t;
    use crate::types::ssize_t;
    use crate::types::uint8_t;
    use crate::types::MBuf;
    use crate::types::PktHdr;
    extern "C" {

        pub static mut client_accept_sslmode: ::core::ffi::c_int;

        pub fn sbuf_tls_accept(sbuf: *mut SBuf) -> bool;

        pub fn sbuf_pause(sbuf: *mut SBuf) -> bool;

        pub fn sbuf_continue(sbuf: *mut SBuf);

        pub fn sbuf_flush(sbuf: *mut SBuf) -> bool;

        pub fn sbuf_prepare_send(sbuf: *mut SBuf, dst: *mut SBuf, amount: ::core::ffi::c_uint);

        pub fn sbuf_prepare_skip(sbuf: *mut SBuf, amount: ::core::ffi::c_uint);

        pub fn sbuf_prepare_fetch(sbuf: *mut SBuf, amount: ::core::ffi::c_uint);

        pub fn sbuf_queue_full_packet(sbuf: *mut SBuf, dst: *mut SBuf, pkt: *mut PktHdr) -> bool;

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

        pub fn pktbuf_free(buf: *mut PktBuf);

        pub fn pktbuf_send_immediate(buf: *mut PktBuf, sk: *mut PgSocket) -> bool;

        pub fn pktbuf_write_generic(
            buf: *mut PktBuf,
            type_0: ::core::ffi::c_int,
            fmt: *const ::core::ffi::c_char,
            ...
        );

        pub fn pktbuf_write_ExtQuery(
            buf: *mut PktBuf,
            query: *const ::core::ffi::c_char,
            nargs: ::core::ffi::c_int,
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

pub mod messages_h {

    pub const PS_IGNORE: PreparedStatementAction = 0;

    pub type PreparedStatementAction = ::core::ffi::c_uint;

    pub const PS_INSPECT_FAILED: PreparedStatementAction = 3;

    pub const PS_HANDLE_FULL_PACKET: PreparedStatementAction = 2;

    pub const PS_HANDLE: PreparedStatementAction = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct PgClosePacket {
        pub type_0: ::core::ffi::c_char,
        pub name: *const ::core::ffi::c_char,
    }
    use super::bouncer_h::PgSocket;
    use crate::types::PktHdr;
    extern "C" {

        pub fn inspect_parse_packet(
            client: *mut PgSocket,
            pkt: *mut PktHdr,
        ) -> PreparedStatementAction;

        pub fn inspect_bind_packet(
            client: *mut PgSocket,
            pkt: *mut PktHdr,
        ) -> PreparedStatementAction;

        pub fn inspect_describe_or_close_packet(
            client: *mut PgSocket,
            pkt: *mut PktHdr,
        ) -> PreparedStatementAction;

        pub fn unmarshall_close_packet(
            client: *mut PgSocket,
            pkt: *mut PktHdr,
            close_packet_p: *mut PgClosePacket,
        ) -> bool;

        pub fn is_close_named_statement_packet(close_packet: *mut PgClosePacket) -> bool;
    }
}

pub mod scram_h {

    pub const PASSWORD_TYPE_PLAINTEXT: PasswordType = 0;

    pub type PasswordType = ::core::ffi::c_uint;

    pub const PASSWORD_TYPE_SCRAM_SHA_256: PasswordType = 2;

    pub const PASSWORD_TYPE_MD5: PasswordType = 1;
    use super::bouncer_h::{PgCredentials, PgSocket, ScramState};
    use crate::types::uint8_t;
    extern "C" {

        pub fn free_scram_state(state: *mut ScramState);

        pub fn get_password_type(shadow_pass: *const ::core::ffi::c_char) -> PasswordType;

        pub fn read_client_first_message(
            client: *mut PgSocket,
            input: *mut ::core::ffi::c_char,
        ) -> bool;

        pub fn read_client_final_message(
            client: *mut PgSocket,
            raw_input: *const uint8_t,
            input: *mut ::core::ffi::c_char,
            client_final_nonce_p: *mut *const ::core::ffi::c_char,
            proof_p: *mut *mut ::core::ffi::c_char,
        ) -> bool;

        pub fn build_server_first_message(
            state: *mut ScramState,
            user: *mut PgCredentials,
            stored_secret: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;

        pub fn build_server_final_message(client: *mut PgSocket) -> *mut ::core::ffi::c_char;

        pub fn verify_final_nonce(
            state: *const ScramState,
            client_final_nonce: *const ::core::ffi::c_char,
        ) -> bool;

        pub fn verify_client_proof(
            client: *mut PgSocket,
            ClientProof: *const ::core::ffi::c_char,
        ) -> bool;

        pub fn scram_verify_plain_password(
            client: *mut PgSocket,
            username: *const ::core::ffi::c_char,
            password: *const ::core::ffi::c_char,
            secret: *const ::core::ffi::c_char,
        ) -> bool;
    }
}

pub mod hba_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct HBARule {
        pub node: List,
        pub rule_type: RuleType,
        pub rule_method: ::core::ffi::c_int,
        pub address: HBAAddress,
        pub db_name: HBAName,
        pub user_name: HBAName,
        pub identmap: *mut IdentMap,
        pub hba_linenr: ::core::ffi::c_int,
        pub auth_options: *mut ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct IdentMap {
        pub node: List,
        pub map_name: *mut ::core::ffi::c_char,
        pub mappings: List,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct HBAName {
        pub flags: ::core::ffi::c_uint,
        pub name_set: *mut StrSet,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct HBAAddress {
        pub flags: ::core::ffi::c_uint,
        pub family: ::core::ffi::c_int,
        pub addr: [uint8_t; 16],
        pub mask: [uint8_t; 16],
    }

    pub type RuleType = ::core::ffi::c_uint;

    pub const RULE_HOSTNOSSL: RuleType = 3;

    pub const RULE_HOSTSSL: RuleType = 2;

    pub const RULE_HOST: RuleType = 1;

    pub const RULE_LOCAL: RuleType = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct Mapping {
        pub node: List,
        pub system_user_name: *mut ::core::ffi::c_char,
        pub postgres_user_name: *mut ::core::ffi::c_char,
        pub name_flags: ::core::ffi::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct HBA {
        pub rules: List,
    }

    pub const NAME_ALL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    use super::bouncer_h::{PgAddr, ReplicationType};
    use crate::types::uint8_t;
    use crate::types::List;
    extern "C" {

        pub type StrSet;

        pub fn hba_eval(
            hba: *mut HBA,
            addr: *mut PgAddr,
            is_tls: bool,
            replication: ReplicationType,
            dbname: *const ::core::ffi::c_char,
            username: *const ::core::ffi::c_char,
        ) -> *mut HBARule;
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

        pub fn calloc(__count: size_t, __size: size_t) -> *mut ::core::ffi::c_void;

        pub fn free(_: *mut ::core::ffi::c_void);
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

        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;

        pub fn strncmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
            __n: size_t,
        ) -> ::core::ffi::c_int;

        pub fn strdup(__s1: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;

        pub fn strlcpy(
            __dst: *mut ::core::ffi::c_char,
            __source: *const ::core::ffi::c_char,
            __size: size_t,
        ) -> ::core::ffi::c_ulong;
    }
}

pub mod util_h {

    pub const MD5_PASSWD_LEN: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
    use crate::types::size_t;
    use crate::types::uint8_t;
    extern "C" {

        pub fn pg_md5_encrypt(
            part1: *const ::core::ffi::c_char,
            part2: *const ::core::ffi::c_char,
            p2len: size_t,
            dest: *mut ::core::ffi::c_char,
        ) -> bool;

        pub fn get_random_bytes(dest: *mut uint8_t, len: ::core::ffi::c_int);

        pub fn bin2hex(
            src: *const uint8_t,
            srclen: ::core::ffi::c_uint,
            dst: *mut ::core::ffi::c_char,
            dstlen: ::core::ffi::c_uint,
        ) -> *const ::core::ffi::c_char;

        pub fn strlist_contains(
            liststr: *const ::core::ffi::c_char,
            str: *const ::core::ffi::c_char,
        ) -> bool;
    }
}

pub mod admin_h {
    use super::bouncer_h::PgSocket;
    use crate::types::PktHdr;
    extern "C" {

        pub fn admin_handle_client(client: *mut PgSocket, pkt: *mut PktHdr) -> bool;

        pub fn admin_pre_login(client: *mut PgSocket, username: *const ::core::ffi::c_char)
            -> bool;

        pub fn admin_post_login(client: *mut PgSocket) -> bool;
    }
}

pub mod objects_h {
    use super::bouncer_h::{
        PgCredentials, PgDatabase, PgGlobalUser, PgPool, PgSocket, ResponseAction,
    };
    extern "C" {

        pub fn find_or_register_database(
            connection: *mut PgSocket,
            name: *const ::core::ffi::c_char,
        ) -> *mut PgDatabase;

        pub fn find_global_user(name: *const ::core::ffi::c_char) -> *mut PgGlobalUser;

        pub fn find_global_credentials(name: *const ::core::ffi::c_char) -> *mut PgCredentials;

        pub fn get_pool(db: *mut PgDatabase, user_credentials: *mut PgCredentials) -> *mut PgPool;

        pub fn find_server(client: *mut PgSocket) -> bool;

        pub fn release_server(server: *mut PgSocket) -> bool;

        pub fn finish_client_login(client: *mut PgSocket) -> bool;

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

        pub fn add_dynamic_credentials(
            db: *mut PgDatabase,
            name: *const ::core::ffi::c_char,
            passwd: *const ::core::ffi::c_char,
        ) -> *mut PgCredentials;

        pub fn add_outstanding_request(
            client: *mut PgSocket,
            type_0: ::core::ffi::c_char,
            action: ResponseAction,
        ) -> bool;

        pub fn find_or_add_new_global_credentials(
            name: *const ::core::ffi::c_char,
            passwd: *const ::core::ffi::c_char,
        ) -> *mut PgCredentials;

        pub fn add_pam_credentials(
            name: *const ::core::ffi::c_char,
            passwd: *const ::core::ffi::c_char,
        ) -> *mut PgCredentials;

        pub fn accept_cancel_request(req: *mut PgSocket);

        pub fn get_active_client_count() -> ::core::ffi::c_int;
    }
}

pub mod server_h {
    use super::bouncer_h::{PgDatabase, PgGlobalUser, PgSocket};
    extern "C" {

        pub fn connection_pool_mode(connection: *mut PgSocket) -> ::core::ffi::c_int;

        pub fn database_max_client_connections(db: *mut PgDatabase) -> ::core::ffi::c_int;

        pub fn user_client_max_connections(user: *mut PgGlobalUser) -> ::core::ffi::c_int;
    }
}

pub mod pam_h {
    use super::bouncer_h::PgSocket;
    extern "C" {

        pub fn pam_auth_begin(client: *mut PgSocket, passwd: *const ::core::ffi::c_char);
    }
}

pub mod ldapauth_h {
    use super::bouncer_h::PgSocket;
    extern "C" {

        pub fn ldap_auth_begin(client: *mut PgSocket, passwd: *const ::core::ffi::c_char);
    }
}

pub mod _stdlib_h {
    extern "C" {

        pub fn exit(_: ::core::ffi::c_int) -> !;
    }
}

pub mod protocol_h {

    pub const PqMsg_Bind: ::core::ffi::c_uint = 66 as ::core::ffi::c_uint;

    pub const PqMsg_Close: ::core::ffi::c_int = 'C' as i32;

    pub const PqMsg_Describe: ::core::ffi::c_uint = 68 as ::core::ffi::c_uint;

    pub const PqMsg_Execute: ::core::ffi::c_uint = 69 as ::core::ffi::c_uint;

    pub const PqMsg_FunctionCall: ::core::ffi::c_uint = 70 as ::core::ffi::c_uint;

    pub const PqMsg_Flush: ::core::ffi::c_uint = 72 as ::core::ffi::c_uint;

    pub const PqMsg_Parse: ::core::ffi::c_uint = 80 as ::core::ffi::c_uint;

    pub const PqMsg_Query: ::core::ffi::c_uint = 81 as ::core::ffi::c_uint;

    pub const PqMsg_Sync: ::core::ffi::c_int = 'S' as i32;

    pub const PqMsg_Terminate: ::core::ffi::c_uint = 88 as ::core::ffi::c_uint;

    pub const PqMsg_CopyFail: ::core::ffi::c_uint = 102 as ::core::ffi::c_uint;

    pub const PqMsg_PasswordMessage: ::core::ffi::c_uint = 112 as ::core::ffi::c_uint;

    pub const PqMsg_ParseComplete: ::core::ffi::c_uint = 49 as ::core::ffi::c_uint;

    pub const PqMsg_BindComplete: ::core::ffi::c_uint = 50 as ::core::ffi::c_uint;

    pub const PqMsg_CommandComplete: ::core::ffi::c_uint = 67 as ::core::ffi::c_uint;

    pub const PqMsg_DataRow: ::core::ffi::c_uint = 68 as ::core::ffi::c_uint;

    pub const PqMsg_ErrorResponse: ::core::ffi::c_uint = 69 as ::core::ffi::c_uint;

    pub const PqMsg_NoticeResponse: ::core::ffi::c_uint = 78 as ::core::ffi::c_uint;

    pub const PqMsg_AuthenticationRequest: ::core::ffi::c_int = 'R' as i32;

    pub const PqMsg_ParameterStatus: ::core::ffi::c_uint = 83 as ::core::ffi::c_uint;

    pub const PqMsg_RowDescription: ::core::ffi::c_uint = 84 as ::core::ffi::c_uint;

    pub const PqMsg_ReadyForQuery: ::core::ffi::c_uint = 90 as ::core::ffi::c_uint;

    pub const PqMsg_NegotiateProtocolVersion: ::core::ffi::c_int = 'v' as i32;

    pub const PqMsg_CopyDone: ::core::ffi::c_uint = 99 as ::core::ffi::c_uint;

    pub const PqMsg_CopyData: ::core::ffi::c_uint = 100 as ::core::ffi::c_uint;

    pub const AUTH_REQ_PASSWORD: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

    pub const AUTH_REQ_MD5: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

    pub const AUTH_REQ_SASL: ::core::ffi::c_int = 10 as ::core::ffi::c_int;

    pub const AUTH_REQ_SASL_CONT: ::core::ffi::c_int = 11 as ::core::ffi::c_int;

    pub const AUTH_REQ_SASL_FIN: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
}

pub mod system_h {
    extern "C" {

        pub fn check_unix_peer_name(
            fd: ::core::ffi::c_int,
            username: *const ::core::ffi::c_char,
        ) -> bool;
    }
}

pub mod builtins_h {
    extern "C" {

        pub fn parse_bool(value: *const ::core::ffi::c_char, result: *mut bool) -> bool;
    }
}
pub use self::_ctype_h::{__istype, __maskrune, isascii, isspace, _CTYPE_S};
use self::_malloc_h::{calloc, free, malloc};
use self::_stdio_h::snprintf;
use self::_stdlib_h::exit;
use self::_string_h::{memcpy, memset, strchr, strcmp, strdup, strlcpy, strlen, strncmp};
use self::admin_h::{admin_handle_client, admin_post_login, admin_pre_login};
pub use self::bouncer_h::{
    auth_type, cf_admin_users, cf_application_name_add_host, cf_auth_dbname, cf_auth_query,
    cf_auth_type, cf_auth_user, cf_disable_pqexec, cf_ignore_startup_params, cf_log_connections,
    cf_max_client_conn, cf_max_prepared_statements, cf_sbuf_len, cstr_skip_ws, parsed_hba,
    pga_details, pga_is_unix, replication_type_parameters, sockaddr_ucreds, C2RustUnnamed_9,
    CallbackState, LoadBalanceHosts, PacketCallbackFlag, PgAddr, PgCredentials, PgDatabase,
    PgGlobalUser, PgPool, PgSocket, PgStats, ReplicationType, ResponseAction, SSLMode, ScramState,
    SocketState, AUTH_TYPE_ANY, AUTH_TYPE_CERT, AUTH_TYPE_HBA, AUTH_TYPE_LDAP, AUTH_TYPE_MD5,
    AUTH_TYPE_PAM, AUTH_TYPE_PEER, AUTH_TYPE_PLAIN, AUTH_TYPE_REJECT, AUTH_TYPE_SCRAM_SHA_256,
    AUTH_TYPE_TRUST, BACKENDKEY_LEN, CB_HANDLE_COMPLETE_PACKET, CB_NONE, CB_WANT_COMPLETE_PACKET,
    CL_ACTIVE, CL_ACTIVE_CANCEL, CL_FREE, CL_JUSTFREE, CL_LOGIN, CL_WAITING, CL_WAITING_CANCEL,
    CL_WAITING_LOGIN, LOAD_BALANCE_HOSTS_DISABLE, LOAD_BALANCE_HOSTS_ROUND_ROBIN, MAX_PASSWORD,
    MAX_USERNAME, PKT_CANCEL, PKT_GSSENCREQ, PKT_SSLREQ, PKT_STARTUP_V2, PKT_STARTUP_V3,
    PKT_STARTUP_V3_UNSUPPORTED, POOL_SESSION, RA_FAKE, RA_FORWARD, RA_SKIP, REPLICATION_LOGICAL,
    REPLICATION_NONE, REPLICATION_PHYSICAL, SSLMODE_ALLOW, SSLMODE_DISABLED, SSLMODE_PREFER,
    SSLMODE_REQUIRE, SSLMODE_VERIFY_CA, SSLMODE_VERIFY_FULL, SV_ACTIVE, SV_ACTIVE_CANCEL,
    SV_BEING_CANCELED, SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN, SV_TESTED, SV_USED,
};
use self::builtins_h::parse_bool;
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
    __darwin_time_t, __darwin_wchar_t, __int32_t, __uint16_t, __uint32_t, __uint8_t,
};
pub use crate::types::{aatree_cmp_f, aatree_walker_f, AANode, AATree};
pub use crate::types::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};

pub use self::event_struct_h::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
pub use self::hba_h::{
    hba_eval, HBAAddress, HBAName, HBARule, IdentMap, Mapping, RuleType, StrSet, HBA, NAME_ALL,
    RULE_HOST, RULE_HOSTNOSSL, RULE_HOSTSSL, RULE_LOCAL,
};
pub use self::in6_h::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use self::in_h::{in_addr, sockaddr_in};
pub use self::iobuf_h::{iobuf, IOBuf};
use self::ldapauth_h::ldap_auth_begin;
pub use self::logging_h::{
    cf_verbose, log_fatal, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE,
    LG_STATS, LG_WARNING,
};
pub use self::messages_h::{
    inspect_bind_packet, inspect_describe_or_close_packet, inspect_parse_packet,
    is_close_named_statement_packet, unmarshall_close_packet, PgClosePacket,
    PreparedStatementAction, PS_HANDLE, PS_HANDLE_FULL_PACKET, PS_IGNORE, PS_INSPECT_FAILED,
};
use self::objects_h::{
    accept_cancel_request, add_dynamic_credentials, add_outstanding_request, add_pam_credentials,
    disconnect_client, disconnect_server, find_global_credentials, find_global_user,
    find_or_add_new_global_credentials, find_or_register_database, find_server,
    finish_client_login, get_active_client_count, get_pool, release_server,
};
use self::pam_h::pam_auth_begin;
pub use self::pktbuf_h::{
    pktbuf_dynamic, pktbuf_free, pktbuf_send_immediate, pktbuf_static, pktbuf_write_ExtQuery,
    pktbuf_write_generic, PktBuf,
};
pub use crate::lib::usual::mbuf::{
    mbuf_avail_for_read, mbuf_free, mbuf_get_bytes, mbuf_get_chars, mbuf_get_string,
    mbuf_get_uint16be, mbuf_get_uint32be, mbuf_init_dynamic, mbuf_init_fixed_writer,
    mbuf_make_room, mbuf_rewind_reader, mbuf_rewind_writer, mbuf_write, mbuf_write_byte,
    mbuf_write_raw_mbuf, mbuf_written,
};
pub use crate::types::List;
pub use crate::types::MBuf;
pub use crate::types::{PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement};

// External function declarations (defined in prepare.rs)
extern "C" {
    pub fn handle_parse_command(
        client: *mut bouncer_h::PgSocket,
        pkt: *mut crate::types::PktHdr,
    ) -> bool;
    pub fn handle_bind_command(
        client: *mut bouncer_h::PgSocket,
        pkt: *mut crate::types::PktHdr,
    ) -> bool;
    pub fn handle_describe_command(
        client: *mut bouncer_h::PgSocket,
        pkt: *mut crate::types::PktHdr,
    ) -> bool;
    pub fn handle_close_statement_command(
        client: *mut bouncer_h::PgSocket,
        pkt: *mut crate::types::PktHdr,
        close_packet: *mut messages_h::PgClosePacket,
    ) -> bool;
}
pub use self::protocol_h::{
    PqMsg_AuthenticationRequest, PqMsg_Bind, PqMsg_BindComplete, PqMsg_Close,
    PqMsg_CommandComplete, PqMsg_CopyData, PqMsg_CopyDone, PqMsg_CopyFail, PqMsg_DataRow,
    PqMsg_Describe, PqMsg_ErrorResponse, PqMsg_Execute, PqMsg_Flush, PqMsg_FunctionCall,
    PqMsg_NegotiateProtocolVersion, PqMsg_NoticeResponse, PqMsg_ParameterStatus, PqMsg_Parse,
    PqMsg_ParseComplete, PqMsg_PasswordMessage, PqMsg_Query, PqMsg_ReadyForQuery,
    PqMsg_RowDescription, PqMsg_Sync, PqMsg_Terminate, AUTH_REQ_MD5, AUTH_REQ_PASSWORD,
    AUTH_REQ_SASL, AUTH_REQ_SASL_CONT, AUTH_REQ_SASL_FIN,
};
pub use self::runetype_h::{
    _DefaultRuneLocale, _RuneCharClass, _RuneEntry, _RuneLocale, _RuneRange,
};
pub use self::sbuf_h::{
    client_accept_sslmode, sbuf_answer, sbuf_cb_t, sbuf_continue, sbuf_flush, sbuf_pause,
    sbuf_prepare_fetch, sbuf_prepare_send, sbuf_prepare_skip, sbuf_queue_full_packet,
    sbuf_tls_accept, SBuf, SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK,
    SBUF_EV_FLUSH, SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY,
};
pub use self::scram_h::{
    build_server_final_message, build_server_first_message, free_scram_state, get_password_type,
    read_client_final_message, read_client_first_message, scram_verify_plain_password,
    verify_client_proof, verify_final_nonce, PasswordType, PASSWORD_TYPE_MD5,
    PASSWORD_TYPE_PLAINTEXT, PASSWORD_TYPE_SCRAM_SHA_256,
};
use self::server_h::{
    connection_pool_mode, database_max_client_connections, user_client_max_connections,
};
pub use self::socket_h::{sockaddr, AF_UNIX};
pub use self::sys__types_h::{__darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t, __DARWIN_NULL};
use self::system_h::check_unix_peer_name;
use self::tls_h::{
    tls_get_connection_info, tls_peer_cert_contains_name, tls_peer_cert_provided,
    tls_peer_cert_subject,
};
pub use self::util_h::{
    bin2hex, get_random_bytes, pg_md5_encrypt, strlist_contains, MD5_PASSWD_LEN,
};
pub use crate::types::usec_t;
pub use crate::types::StatList;
pub use crate::types::VarCache;
pub use crate::types::{false_0, true_0};
pub use crate::types::{
    free_header, get_header, incomplete_header, incomplete_pkt, log_server_error, pkt_desc,
    pkt_rewind_v2, pkt_rewind_v3, PktHdr, NEW_HEADER_LEN, OLD_HEADER_LEN,
};
pub use crate::types::{PStr, StrPool};
pub use crate::types::{UT_hash_bucket, UT_hash_handle, UT_hash_table};

unsafe extern "C" fn hdr2hex(
    mut data: *const MBuf,
    mut buf: *mut ::core::ffi::c_char,
    mut buflen: ::core::ffi::c_uint,
) -> *const ::core::ffi::c_char {
    let mut bin: *const uint8_t = (*data).data.offset((*data).read_pos as isize);
    let mut dlen: ::core::ffi::c_uint = 0;
    dlen = mbuf_avail_for_read(data);
    bin2hex(bin, dlen, buf, buflen)
}
#[no_mangle]

pub unsafe extern "C" fn prepare_auth_database(mut client: *mut PgSocket) -> *mut PgDatabase {
    let mut auth_db = ::core::ptr::null_mut::<PgDatabase>();
    let mut auth_dbname: *const ::core::ffi::c_char =
        if !(*(*client).c2rust_unnamed.db).auth_dbname.is_null() {
            (*(*client).c2rust_unnamed.db).auth_dbname
        } else {
            cf_auth_dbname
        };
    if auth_dbname.is_null() {
        auth_db = (*client).c2rust_unnamed.db;
    } else {
        auth_db = find_or_register_database(client, auth_dbname);
    }
    if auth_db.is_null() {
        log_generic(
            LG_ERROR,
            client as *mut ::core::ffi::c_void,
            b"authentication database \"%s\" is not configured.\0" as *const u8
                as *const ::core::ffi::c_char,
            auth_dbname,
        );
        disconnect_client(client, true, c"bouncer config error".as_ptr());
        return ::core::ptr::null_mut::<PgDatabase>();
    }
    if (*auth_db).db_disabled {
        disconnect_client(
            client,
            true,
            b"authentication database \"%s\" is disabled\0" as *const u8
                as *const ::core::ffi::c_char,
            auth_dbname,
        );
        return ::core::ptr::null_mut::<PgDatabase>();
    }
    if (*auth_db).admin {
        log_generic(
            LG_ERROR,
            client as *mut ::core::ffi::c_void,
            b"cannot use the reserved \"%s\" database as an auth_dbname\0" as *const u8
                as *const ::core::ffi::c_char,
            (*auth_db).dbname,
        );
        disconnect_client(client, true, c"bouncer config error".as_ptr());
        return ::core::ptr::null_mut::<PgDatabase>();
    }
    auth_db
}

unsafe extern "C" fn check_client_passwd(
    mut client: *mut PgSocket,
    mut passwd: *const ::core::ffi::c_char,
) -> bool {
    let mut user = (*client).login_user_credentials;
    let mut auth_type = (*client).client_auth_type;
    if (*user).mock_auth {
        return false;
    }
    if *(&raw mut (*user).passwd as *mut ::core::ffi::c_char) == 0 {
        return false;
    }
    match auth_type {
        2 => {
            match get_password_type(&raw mut (*user).passwd as *mut ::core::ffi::c_char)
                as ::core::ffi::c_uint
            {
                0 => {
                    return strcmp(&raw mut (*user).passwd as *mut ::core::ffi::c_char, passwd)
                        == 0 as ::core::ffi::c_int;
                }
                1 => {
                    let mut md5: [::core::ffi::c_char; 36] = [0; 36];
                    if !pg_md5_encrypt(
                        passwd,
                        &raw mut (*user).name as *mut ::core::ffi::c_char,
                        strlen(&raw mut (*user).name as *mut ::core::ffi::c_char),
                        &raw mut md5 as *mut ::core::ffi::c_char,
                    ) {
                        return false;
                    }
                    return strcmp(
                        &raw mut (*user).passwd as *mut ::core::ffi::c_char,
                        &raw mut md5 as *mut ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int;
                }
                2 => {
                    return scram_verify_plain_password(
                        client,
                        &raw mut (*user).name as *mut ::core::ffi::c_char,
                        passwd,
                        &raw mut (*user).passwd as *mut ::core::ffi::c_char,
                    );
                }
                _ => return false,
            }
        }
        3 => {
            let mut stored_passwd = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut md5_0: [::core::ffi::c_char; 36] = [0; 36];
            if strlen(passwd) != MD5_PASSWD_LEN as size_t {
                return false;
            }
            if get_password_type(&raw mut (*user).passwd as *mut ::core::ffi::c_char)
                as ::core::ffi::c_uint
                == PASSWORD_TYPE_PLAINTEXT as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if !pg_md5_encrypt(
                    &raw mut (*user).passwd as *mut ::core::ffi::c_char,
                    &raw mut (*user).name as *mut ::core::ffi::c_char,
                    strlen(&raw mut (*user).name as *mut ::core::ffi::c_char),
                    &raw mut md5_0 as *mut ::core::ffi::c_char,
                ) {
                    return false;
                }
                stored_passwd = &raw mut md5_0 as *mut ::core::ffi::c_char;
            } else {
                stored_passwd = &raw mut (*user).passwd as *mut ::core::ffi::c_char;
            }
            if !pg_md5_encrypt(
                stored_passwd.offset(3 as ::core::ffi::c_int as isize),
                &raw mut (*client).cancel_key as *mut uint8_t as *mut ::core::ffi::c_char,
                4 as size_t,
                &raw mut md5_0 as *mut ::core::ffi::c_char,
            ) {
                return false;
            }
            return strcmp(&raw mut md5_0 as *mut ::core::ffi::c_char, passwd)
                == 0 as ::core::ffi::c_int;
        }
        _ => {}
    }
    false
}

unsafe extern "C" fn send_client_authreq(mut client: *mut PgSocket) -> bool {
    let mut res: ::core::ffi::c_int = 0;
    let mut auth_type = (*client).client_auth_type;
    if auth_type == AUTH_TYPE_MD5 as ::core::ffi::c_int {
        let mut saltlen: uint8_t = 4 as uint8_t;
        get_random_bytes(
            &raw mut (*client).cancel_key as *mut uint8_t as *mut ::core::ffi::c_void
                as *mut uint8_t,
            saltlen as ::core::ffi::c_int,
        );
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
            PqMsg_AuthenticationRequest,
            c"ib".as_ptr(),
            AUTH_REQ_MD5,
            &raw mut (*client).cancel_key as *mut uint8_t,
            saltlen as ::core::ffi::c_int,
        );
        res = pktbuf_send_immediate(&raw mut _buf, client) as ::core::ffi::c_int;
    } else if auth_type == AUTH_TYPE_PLAIN as ::core::ffi::c_int
        || auth_type == AUTH_TYPE_LDAP as ::core::ffi::c_int
        || auth_type == AUTH_TYPE_PAM as ::core::ffi::c_int
    {
        let mut _data_0: [uint8_t; 512] = [0; 512];
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
            ::core::mem::size_of::<[uint8_t; 512]>() as ::core::ffi::c_int,
        );
        pktbuf_write_generic(
            &raw mut _buf_0,
            PqMsg_AuthenticationRequest,
            c"i".as_ptr(),
            AUTH_REQ_PASSWORD,
        );
        res = pktbuf_send_immediate(&raw mut _buf_0, client) as ::core::ffi::c_int;
    } else if auth_type == AUTH_TYPE_SCRAM_SHA_256 as ::core::ffi::c_int {
        let mut _data_1: [uint8_t; 512] = [0; 512];
        let mut _buf_1 = PktBuf {
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
            &raw mut _buf_1,
            &raw mut _data_1 as *mut uint8_t,
            ::core::mem::size_of::<[uint8_t; 512]>() as ::core::ffi::c_int,
        );
        pktbuf_write_generic(
            &raw mut _buf_1,
            PqMsg_AuthenticationRequest,
            c"iss".as_ptr(),
            AUTH_REQ_SASL,
            c"SCRAM-SHA-256".as_ptr(),
            c"".as_ptr(),
        );
        res = pktbuf_send_immediate(&raw mut _buf_1, client) as ::core::ffi::c_int;
    } else {
        return false;
    }
    if res == 0 {
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(
                LG_NOISE,
                client as *mut ::core::ffi::c_void,
                c"No authentication response received".as_ptr(),
            );
        }
        disconnect_client(client, false, c"failed to send auth req".as_ptr());
    } else if cf_verbose > 1 as ::core::ffi::c_int
    {
        log_generic(
            LG_NOISE,
            client as *mut ::core::ffi::c_void,
            c"Auth request sent successfully".as_ptr(),
        );
    }
    res != 0
}
#[no_mangle]

pub unsafe extern "C" fn sending_auth_query(mut client: *mut PgSocket) -> bool {
    (*client).wait_for_user_conn() || (*client).wait_for_user()
}

unsafe extern "C" fn start_auth_query(
    mut client: *mut PgSocket,
    mut username: *const ::core::ffi::c_char,
) {
    let mut res: ::core::ffi::c_int = 0;
    let mut buf = ::core::ptr::null_mut::<PktBuf>();
    let mut auth_query: *const ::core::ffi::c_char =
        if !(*(*client).c2rust_unnamed.db).auth_query.is_null() {
            (*(*client).c2rust_unnamed.db).auth_query
        } else {
            cf_auth_query
        };
    let mut auth_db = prepare_auth_database(client);
    if auth_db.is_null() {
        return;
    }
    (*client).pool = get_pool(
        auth_db,
        (*(*client).c2rust_unnamed.db).auth_user_credentials,
    );
    if (*client).pool.is_null() {
        disconnect_client(client, true, c"no memory for authentication pool".as_ptr());
        return;
    }
    (*client).set_wait_for_user_conn(true);
    if !find_server(client) {
        return;
    }
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(
            LG_NOISE,
            client as *mut ::core::ffi::c_void,
            c"doing auth_conn query: %s".as_ptr(),
            auth_query,
        );
    }
    (*client).set_wait_for_user_conn(false);
    (*client).set_wait_for_user(true);
    if !sbuf_pause(&raw mut (*client).sbuf) {
        release_server((*client).link);
        disconnect_client(client, true, c"pause failed".as_ptr());
        return;
    }
    (*(*client).link).set_ready(false);
    if !add_outstanding_request(client, PqMsg_Sync as ::core::ffi::c_char, RA_SKIP) {
        disconnect_server((*client).link, true, c"out of memory".as_ptr());
        return;
    }
    res = 0 as ::core::ffi::c_int;
    buf = pktbuf_dynamic(512 as ::core::ffi::c_int);
    if !buf.is_null() {
        pktbuf_write_ExtQuery(buf, auth_query, 1 as ::core::ffi::c_int, username);
        res = pktbuf_send_immediate(buf, (*client).link) as ::core::ffi::c_int;
        pktbuf_free(buf);
    }
    if res == 0 {
        disconnect_server((*client).link, false, c"unable to send auth_query".as_ptr());
    }
}

unsafe extern "C" fn login_via_cert(mut client: *mut PgSocket, mut rule: *mut HBARule) -> bool {
    let mut current_block: u64;
    let mut tls = (*client).sbuf.tls;
    if tls.is_null() {
        log_generic(
            LG_ERROR,
            client as *mut ::core::ffi::c_void,
            c"TLS connection required".as_ptr(),
        );
    } else if tls_peer_cert_provided((*client).sbuf.tls) == 0 {
        log_generic(
            LG_ERROR,
            client as *mut ::core::ffi::c_void,
            c"TLS client certificate required".as_ptr(),
        );
    } else if !(*(*client).login_user_credentials).mock_auth {
        let mut _log_ctx = NULL;
        if cf_verbose > 0 as ::core::ffi::c_int
        {
            log_generic(
                LG_DEBUG,
                _log_ctx,
                c"TLS cert login: %s".as_ptr(),
                tls_peer_cert_subject((*client).sbuf.tls),
            );
        }
        if !rule.is_null() && !(*rule).identmap.is_null() {
            let mut el = ::core::ptr::null_mut::<List>();
            let mut mapping = ::core::ptr::null_mut::<Mapping>();
            let mut mapped = false;
            let mut current_block_12: u64;
            el = (*(*rule).identmap).mappings.next;
            while el != &raw mut (*(*rule).identmap).mappings {
                mapping = (el as *mut ::core::ffi::c_char)
                    
                    as *mut Mapping;
                if tls_peer_cert_contains_name((*client).sbuf.tls, (*mapping).system_user_name) != 0
                {
                    if (*mapping).name_flags & NAME_ALL as ::core::ffi::c_uint == 0 {
                        if strcmp(
                            &raw mut (*(*client).login_user_credentials).name
                                as *mut ::core::ffi::c_char,
                            (*mapping).postgres_user_name,
                        ) != 0
                        {
                            current_block_12 = 7651349459974463963;
                        } else {
                            current_block_12 = 8457315219000651999;
                        }
                    } else {
                        current_block_12 = 8457315219000651999;
                    }
                    match current_block_12 {
                        7651349459974463963 => {}
                        _ => {
                            if cf_verbose > 1 as ::core::ffi::c_int
                            {
                                log_generic(
                                    LG_NOISE,
                                    client as *mut ::core::ffi::c_void,
                                    c"ident map: %s %s %s".as_ptr(),
                                    (*(*rule).identmap).map_name,
                                    (*mapping).system_user_name,
                                    (*mapping).postgres_user_name,
                                );
                            }
                            mapped = true;
                            break;
                        }
                    }
                }
                el = (*el).next;
            }
            if !mapped {
                log_generic(
                    LG_ERROR,
                    client as *mut ::core::ffi::c_void,
                    c"ident map: %s does not have a match".as_ptr(),
                    (*(*rule).identmap).map_name,
                );
                current_block = 14516962830078006934;
            } else {
                current_block = 13550086250199790493;
            }
        } else if tls_peer_cert_contains_name(
            (*client).sbuf.tls,
            &raw mut (*(*client).login_user_credentials).name as *mut ::core::ffi::c_char,
        ) == 0
        {
            log_generic(
                LG_ERROR,
                client as *mut ::core::ffi::c_void,
                c"TLS certificate name mismatch".as_ptr(),
            );
            current_block = 14516962830078006934;
        } else {
            current_block = 13550086250199790493;
        }
        match current_block {
            14516962830078006934 => {}
            _ => return finish_client_login(client),
        }
    }
    disconnect_client(client, true, c"certificate authentication failed".as_ptr());
    false
}

unsafe extern "C" fn login_as_unix_peer(mut client: *mut PgSocket, mut rule: *mut HBARule) -> bool {
    let mut current_block: u64;
    if pga_is_unix(&raw mut (*client).remote_addr) && !(*(*client).login_user_credentials).mock_auth
    {
        if !rule.is_null() && !(*rule).identmap.is_null() {
            let mut el = ::core::ptr::null_mut::<List>();
            let mut mapping = ::core::ptr::null_mut::<Mapping>();
            let mut mapped = false;
            el = (*(*rule).identmap).mappings.next;
            while el != &raw mut (*(*rule).identmap).mappings {
                mapping = (el as *mut ::core::ffi::c_char)
                    
                    as *mut Mapping;
                if check_unix_peer_name((*client).sbuf.sock, (*mapping).system_user_name)
                    && ((*mapping).name_flags & NAME_ALL as ::core::ffi::c_uint != 0
                        || strcmp(
                            (*mapping).postgres_user_name,
                            &raw mut (*(*client).login_user_credentials).name
                                as *mut ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int)
                {
                    if cf_verbose > 1 as ::core::ffi::c_int
                    {
                        log_generic(
                            LG_NOISE,
                            client as *mut ::core::ffi::c_void,
                            c"ident map '%s' is applied".as_ptr(),
                            (*(*rule).identmap).map_name,
                        );
                    }
                    mapped = true;
                    break;
                }
                el = (*el).next;
            }
            if !mapped {
                log_generic(
                    LG_ERROR,
                    client as *mut ::core::ffi::c_void,
                    c"ident map %s cannot be matched".as_ptr(),
                    (*(*rule).identmap).map_name,
                );
                current_block = 4341734365574423248;
            } else {
                current_block = 15904375183555213903;
            }
        } else if !check_unix_peer_name(
            (*client).sbuf.sock,
            &raw mut (*(*client).login_user_credentials).name as *mut ::core::ffi::c_char,
        ) {
            current_block = 4341734365574423248;
        } else {
            current_block = 15904375183555213903;
        }
        match current_block {
            4341734365574423248 => {}
            _ => return finish_client_login(client),
        }
    }
    disconnect_client(client, true, c"unix socket login rejected".as_ptr());
    false
}

unsafe extern "C" fn finish_set_pool(mut client: *mut PgSocket, mut takeover: bool) -> bool {
    let mut ok = false;
    let mut auth: ::core::ffi::c_int = 0;
    let mut rule = ::core::ptr::null_mut::<HBARule>();
    if !(*(*client).login_user_credentials).mock_auth && !(*(*client).c2rust_unnamed.db).fake {
        let mut pool_user_credentials = ::core::ptr::null_mut::<PgCredentials>();
        if !(*(*client).c2rust_unnamed.db)
            .forced_user_credentials
            .is_null()
        {
            pool_user_credentials = (*(*client).c2rust_unnamed.db).forced_user_credentials;
        } else {
            pool_user_credentials = (*client).login_user_credentials;
        }
        (*client).pool = get_pool((*client).c2rust_unnamed.db, pool_user_credentials);
        if (*client).pool.is_null() {
            disconnect_client(client, true, c"no memory for pool".as_ptr());
            return false;
        }
    }
    if cf_log_connections != 0 {
        if !(*client).sbuf.tls.is_null() {
            let mut infobuf: [::core::ffi::c_char; 96] = ::core::mem::transmute::<
                [u8; 96],
                [::core::ffi::c_char; 96],
            >(
                *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            );
            tls_get_connection_info(
                (*client).sbuf.tls,
                &raw mut infobuf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 96]>() as size_t,
            );
            log_generic(
                LG_INFO,
                client as *mut ::core::ffi::c_void,
                c"login attempt: db=%s user=%s tls=%s replication=%s".as_ptr(),
                &raw mut (*(*client).c2rust_unnamed.db).name as *mut ::core::ffi::c_char,
                &raw mut (*(*client).login_user_credentials).name as *mut ::core::ffi::c_char,
                &raw mut infobuf as *mut ::core::ffi::c_char,
                replication_type_parameters[(*client).replication as usize],
            );
        } else {
            log_generic(
                LG_INFO,
                client as *mut ::core::ffi::c_void,
                c"login attempt: db=%s user=%s tls=no replication=%s".as_ptr(),
                &raw mut (*(*client).c2rust_unnamed.db).name as *mut ::core::ffi::c_char,
                &raw mut (*(*client).login_user_credentials).name as *mut ::core::ffi::c_char,
                replication_type_parameters[(*client).replication as usize],
            );
        }
    }
    if takeover {
        return true;
    }
    if !(*client).pool.is_null()
        && (*(*(*client).pool).db).admin
        && !admin_post_login(client)
    {
        return false;
    }
    if (*client).own_user() {
        return finish_client_login(client);
    }
    auth = cf_auth_type;
    if auth == AUTH_TYPE_HBA as ::core::ffi::c_int {
        rule = hba_eval(
            parsed_hba,
            &raw mut (*client).remote_addr,
            !(*client).sbuf.tls.is_null(),
            (*client).replication,
            &raw mut (*(*client).c2rust_unnamed.db).name as *mut ::core::ffi::c_char,
            &raw mut (*(*client).login_user_credentials).name as *mut ::core::ffi::c_char,
        );
        if rule.is_null() {
            disconnect_client(client, true, c"no authentication method is found".as_ptr());
            return false;
        }
        auth = (*rule).rule_method;
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(
                LG_NOISE,
                client as *mut ::core::ffi::c_void,
                c"HBA Line %d is matched".as_ptr(),
                (*rule).hba_linenr,
            );
        }
    }
    if auth == AUTH_TYPE_LDAP as ::core::ffi::c_int {
        disconnect_client(
            client,
            true,
            c"ldap is not supported by this build".as_ptr(),
        );
        return false;
    }
    if auth == AUTH_TYPE_MD5 as ::core::ffi::c_int
        && get_password_type(
            &raw mut (*(*client).login_user_credentials).passwd as *mut ::core::ffi::c_char,
        ) as ::core::ffi::c_uint
            == PASSWORD_TYPE_SCRAM_SHA_256 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        auth = AUTH_TYPE_SCRAM_SHA_256 as ::core::ffi::c_int;
    }
    (*client).client_auth_type = auth;
    match auth {
        0 => {
            ok = finish_client_login(client);
        }
        1 => {
            if (*(*client).login_user_credentials).mock_auth {
                disconnect_client(
                    client,
                    true,
                    b"\"trust\" authentication failed\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
                ok = finish_client_login(client);
            }
        }
        2 | 6 | 3 | 7 | 8 => {
            ok = send_client_authreq(client);
        }
        4 => {
            ok = login_via_cert(client, rule);
        }
        9 => {
            ok = login_as_unix_peer(client, rule);
        }
        _ => {
            disconnect_client(client, true, c"login rejected".as_ptr());
            ok = false;
        }
    }
    ok
}
#[no_mangle]

pub unsafe extern "C" fn check_db_connection_count(mut client: *mut PgSocket) -> bool {
    if !(*client).contributes_db_client_count() {
        (*client).set_contributes_db_client_count(true);
        (*(*client).c2rust_unnamed.db).client_connection_count += 1;
    }
    if database_max_client_connections((*client).c2rust_unnamed.db) <= 0 as ::core::ffi::c_int {
        return true;
    }
    if (*(*client).c2rust_unnamed.db).client_connection_count
        <= database_max_client_connections((*client).c2rust_unnamed.db)
    {
        return true;
    }
    if (*(*client).c2rust_unnamed.db).admin
        && strlist_contains(
            cf_admin_users,
            &raw mut (*(*client).login_user_credentials).name as *mut ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
    {
        return true;
    }
    let mut _log_ctx = NULL;
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            _log_ctx,
            c"set_pool: db '%s' full (%d >= %d)".as_ptr(),
            &raw mut (*(*client).c2rust_unnamed.db).name as *mut ::core::ffi::c_char,
            (*(*client).c2rust_unnamed.db).client_connection_count,
            (*(*client).c2rust_unnamed.db).max_db_client_connections,
        );
    }
    disconnect_client(
        client,
        true,
        c"client connections exceeded (max_db_client_connections)".as_ptr(),
    );
    false
}
#[no_mangle]

pub unsafe extern "C" fn check_user_connection_count(mut client: *mut PgSocket) -> bool {
    let mut client_connection_count: ::core::ffi::c_int = 0;
    let mut max_user_client_connections: ::core::ffi::c_int = 0;
    if (*client).login_user_credentials.is_null() {
        return true;
    }
    if (*(*client).login_user_credentials).global_user.is_null() {
        return true;
    }
    if !(*client).user_connection_counted() {
        (*(*(*client).login_user_credentials).global_user).client_connection_count += 1;
        (*client).set_user_connection_counted(true);
    }
    if (*(*client).c2rust_unnamed.db).admin
        && strlist_contains(
            cf_admin_users,
            &raw mut (*(*client).login_user_credentials).name as *mut ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
    {
        return true;
    }
    max_user_client_connections =
        user_client_max_connections((*(*client).login_user_credentials).global_user);
    if max_user_client_connections == 0 as ::core::ffi::c_int {
        return true;
    }
    client_connection_count =
        (*(*(*client).login_user_credentials).global_user).client_connection_count;
    if client_connection_count <= max_user_client_connections {
        return true;
    }
    let mut _log_ctx = NULL;
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            _log_ctx,
            c"set_pool: user '%s' full (%d >= %d)".as_ptr(),
            &raw mut (*(*client).login_user_credentials).name as *mut ::core::ffi::c_char,
            client_connection_count,
            max_user_client_connections,
        );
    }
    disconnect_client(
        client,
        true,
        c"client connections exceeded (max_user_client_connections)".as_ptr(),
    );
    false
}
#[no_mangle]

pub unsafe extern "C" fn set_pool(
    mut client: *mut PgSocket,
    mut dbname: *const ::core::ffi::c_char,
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut takeover: bool,
) -> bool {
    (*client).c2rust_unnamed.db = find_or_register_database(client, dbname);
    if (*client).c2rust_unnamed.db.is_null() {
        (*client).c2rust_unnamed.db =
            calloc(1 as size_t, ::core::mem::size_of::<PgDatabase>() as size_t) as *mut PgDatabase;
        (*(*client).c2rust_unnamed.db).fake = true;
        strlcpy(
            &raw mut (*(*client).c2rust_unnamed.db).name as *mut ::core::ffi::c_char,
            dbname,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
        );
    }
    if (*(*client).c2rust_unnamed.db).admin && admin_pre_login(client, username) {
        return finish_set_pool(client, takeover);
    }
    if strlen(username) >= MAX_USERNAME as size_t {
        disconnect_client(client, true, c"username too long".as_ptr());
        if cf_log_connections != 0 {
            log_generic(
                LG_INFO,
                client as *mut ::core::ffi::c_void,
                c"login failed: db=%s user=%s".as_ptr(),
                dbname,
                username,
            );
        }
        return false;
    }
    if !password.is_null() && strlen(password) >= MAX_PASSWORD as size_t {
        disconnect_client(client, true, c"password too long".as_ptr());
        if cf_log_connections != 0 {
            log_generic(
                LG_INFO,
                client as *mut ::core::ffi::c_void,
                c"login failed: db=%s user=%s".as_ptr(),
                dbname,
                username,
            );
        }
        return false;
    }
    if cf_auth_type == AUTH_TYPE_ANY as ::core::ffi::c_int {
        if (*(*client).c2rust_unnamed.db)
            .forced_user_credentials
            .is_null()
        {
            log_generic(
                LG_ERROR,
                client as *mut ::core::ffi::c_void,
                c"auth_type=any requires forced user".as_ptr(),
            );
            disconnect_client(client, true, c"bouncer config error".as_ptr());
            return false;
        }
        (*client).login_user_credentials = (*(*client).c2rust_unnamed.db).forced_user_credentials;
        if !check_db_connection_count(client) {
            return false;
        }
        if !check_user_connection_count(client) {
            return false;
        }
    } else if cf_auth_type == AUTH_TYPE_PAM as ::core::ffi::c_int {
        if !(*(*client).c2rust_unnamed.db)
            .auth_user_credentials
            .is_null()
        {
            log_generic(
                LG_ERROR,
                client as *mut ::core::ffi::c_void,
                c"PAM can't be used together with database authentication".as_ptr(),
            );
            disconnect_client(client, true, c"bouncer config error".as_ptr());
            return false;
        }
        (*client).login_user_credentials = add_pam_credentials(username, password);
        if !check_db_connection_count(client) {
            return false;
        }
        if (*client).login_user_credentials.is_null() {
            log_generic(
                LG_ERROR,
                client as *mut ::core::ffi::c_void,
                c"set_pool(): failed to allocate new PAM user".as_ptr(),
            );
            disconnect_client(client, true, c"bouncer resources exhaustion".as_ptr());
            return false;
        }
        if !check_user_connection_count(client) {
            return false;
        }
    } else {
        (*client).login_user_credentials = find_global_credentials(username);
        if !check_db_connection_count(client) {
            return false;
        }
        if !check_user_connection_count(client) {
            return false;
        }
        if (*client).login_user_credentials.is_null()
            || (*(*client).login_user_credentials).dynamic_passwd
        {
            let mut global_user = ::core::ptr::null_mut::<PgGlobalUser>();
            if (*(*client).c2rust_unnamed.db)
                .auth_user_credentials
                .is_null()
                && !cf_auth_user.is_null()
            {
                (*(*client).c2rust_unnamed.db).auth_user_credentials =
                    find_or_add_new_global_credentials(cf_auth_user, c"".as_ptr());
                if (*(*client).c2rust_unnamed.db)
                    .auth_user_credentials
                    .is_null()
                {
                    log_generic(
                        LG_ERROR,
                        client as *mut ::core::ffi::c_void,
                        c"set_pool(): failed to allocate a new global credentials".as_ptr(),
                    );
                    disconnect_client(client, true, c"bouncer resources exhaustion".as_ptr());
                    return false;
                }
            }
            if !(*(*client).c2rust_unnamed.db)
                .auth_user_credentials
                .is_null()
            {
                if (*(*client).c2rust_unnamed.db).fake {
                    if cf_verbose > 0 as ::core::ffi::c_int
                    {
                        log_generic(
                            LG_DEBUG,
                            client as *mut ::core::ffi::c_void,
                            c"not running auth_query because database is fake".as_ptr(),
                        );
                    }
                } else {
                    if takeover {
                        (*client).login_user_credentials = add_dynamic_credentials(
                            (*client).c2rust_unnamed.db,
                            username,
                            password,
                        );
                        if !check_db_connection_count(client) {
                            return false;
                        }
                        if !check_user_connection_count(client) {
                            return false;
                        }
                        return finish_set_pool(client, takeover);
                    }
                    start_auth_query(client, username);
                    return false;
                }
            }
            log_generic(
                LG_INFO,
                client as *mut ::core::ffi::c_void,
                c"no such user: %s".as_ptr(),
                username,
            );
            (*client).login_user_credentials = calloc(
                1 as size_t,
                ::core::mem::size_of::<PgCredentials>() as size_t,
            ) as *mut PgCredentials;
            global_user = find_global_user(username);
            if !global_user.is_null() {
                (*(*client).login_user_credentials).global_user = global_user;
            }
            if !check_db_connection_count(client) {
                return false;
            }
            (*(*client).login_user_credentials).mock_auth = true;
            let mut needed = strlcpy(
                &raw mut (*(*client).login_user_credentials).name as *mut ::core::ffi::c_char,
                username,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
            ) as size_t;
            if (needed >= ::core::mem::size_of::<[::core::ffi::c_char; 128]>())
                as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                let mut _log_ctx = NULL;
                log_generic(
                    LG_WARNING,
                    _log_ctx,
                    c"bug in %s:%d - string truncated".as_ptr(),
                    c"src/client.c".as_ptr(),
                    675 as ::core::ffi::c_int,
                );
            }
            if !check_user_connection_count(client) {
                return false;
            }
        }
    }
    finish_set_pool(client, takeover)
}
#[no_mangle]

pub unsafe extern "C" fn handle_auth_query_response(
    mut client: *mut PgSocket,
    mut pkt: *mut PktHdr,
) -> bool {
    let mut columns: uint16_t = 0;
    let mut length: uint32_t = 0;
    let mut username = ::core::ptr::null::<::core::ffi::c_char>();
    let mut password = ::core::ptr::null::<::core::ffi::c_char>();
    let mut credentials = PgCredentials {
        tree_node: AANode {
            left: ::core::ptr::null_mut::<AANode>(),
            right: ::core::ptr::null_mut::<AANode>(),
            level: 0,
        },
        name: [0; 128],
        passwd: [0; 2048],
        mock_auth: false,
        dynamic_passwd: false,
        global_user: ::core::ptr::null_mut::<PgGlobalUser>(),
        scram_ClientKey: [0; 32],
        scram_ServerKey: [0; 32],
        scram_StoredKey: [0; 32],
        scram_Iiterations: 0,
        scram_SaltKey: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        use_scram_keys: false,
        adhoc_scram_secrets_cached: false,
    };
    let mut server = (*client).link;
    match (*pkt).type_0 {
        84 => {
            if !mbuf_get_uint16be(&raw mut (*pkt).data, &raw mut columns) {
                disconnect_server(server, false, c"bad packet".as_ptr());
                return false;
            }
            if columns as ::core::ffi::c_uint != 2 as ::core::ffi::c_uint {
                disconnect_server(
                    server,
                    false,
                    c"expected 2 columns from auth_query, not %hu".as_ptr(),
                    columns as ::core::ffi::c_int,
                );
                return false;
            }
        }
        68 => {
            memset(
                &raw mut credentials as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<PgCredentials>() as size_t,
            );
            if !mbuf_get_uint16be(&raw mut (*pkt).data, &raw mut columns) {
                disconnect_server(server, false, c"bad packet".as_ptr());
                return false;
            }
            if columns as ::core::ffi::c_uint != 2 as ::core::ffi::c_uint {
                disconnect_server(
                    server,
                    false,
                    c"expected 2 columns from auth_query, not %hu".as_ptr(),
                    columns as ::core::ffi::c_int,
                );
                return false;
            }
            if !mbuf_get_uint32be(&raw mut (*pkt).data, &raw mut length) {
                disconnect_server(server, false, c"bad packet".as_ptr());
                return false;
            }
            if length == -(1 as ::core::ffi::c_int) as uint32_t {
                disconnect_server(
                    server,
                    false,
                    c"auth_query response contained null user name".as_ptr(),
                );
                return false;
            }
            if !mbuf_get_chars(
                &raw mut (*pkt).data,
                length as ::core::ffi::c_uint,
                &raw mut username,
            ) {
                disconnect_server(server, false, c"bad packet".as_ptr());
                return false;
            }
            if ::core::mem::size_of::<[::core::ffi::c_char; 128]>().wrapping_sub(1_usize)
                < length as usize
            {
                length = ::core::mem::size_of::<[::core::ffi::c_char; 128]>().wrapping_sub(1_usize)
                    as uint32_t;
            }
            memcpy(
                &raw mut credentials.name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                username as *const ::core::ffi::c_void,
                length as size_t,
            );
            if !mbuf_get_uint32be(&raw mut (*pkt).data, &raw mut length) {
                disconnect_server(server, false, c"bad packet".as_ptr());
                return false;
            }
            if length == -(1 as ::core::ffi::c_int) as uint32_t {
                password = c"md5".as_ptr();
                length = 3 as uint32_t;
            } else if !mbuf_get_chars(
                &raw mut (*pkt).data,
                length as ::core::ffi::c_uint,
                &raw mut password,
            ) {
                disconnect_server(server, false, c"bad packet".as_ptr());
                return false;
            }
            if ::core::mem::size_of::<[::core::ffi::c_char; 2048]>().wrapping_sub(1_usize)
                < length as usize
            {
                length = ::core::mem::size_of::<[::core::ffi::c_char; 2048]>().wrapping_sub(1_usize)
                    as uint32_t;
            }
            memcpy(
                &raw mut credentials.passwd as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                password as *const ::core::ffi::c_void,
                length as size_t,
            );
            if cf_verbose > 0 as ::core::ffi::c_int
            {
                log_generic(
                    LG_DEBUG,
                    client as *mut ::core::ffi::c_void,
                    c"successfully parsed auth_query response for user %s".as_ptr(),
                    &raw mut credentials.name as *mut ::core::ffi::c_char,
                );
            }
            (*client).login_user_credentials = add_dynamic_credentials(
                (*client).c2rust_unnamed.db,
                &raw mut credentials.name as *mut ::core::ffi::c_char,
                &raw mut credentials.passwd as *mut ::core::ffi::c_char,
            );
            if !check_user_connection_count(client) {
                return false;
            }
            if (*client).login_user_credentials.is_null() {
                disconnect_server(
                    server,
                    false,
                    c"unable to allocate new user for auth".as_ptr(),
                );
                return false;
            }
        }
        78 | 67 | 49 | 50 | 83 => {}
        90 => {
            sbuf_prepare_skip(&raw mut (*(*client).link).sbuf, (*pkt).len);
            if (*client).login_user_credentials.is_null() {
                if cf_log_connections != 0 {
                    log_generic(
                        LG_INFO,
                        client as *mut ::core::ffi::c_void,
                        c"login failed: db=%s".as_ptr(),
                        &raw mut (*(*client).c2rust_unnamed.db).name as *mut ::core::ffi::c_char,
                    );
                }
                disconnect_client(client, true, c"no such user".as_ptr());
            } else {
                if cf_verbose > 1 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_NOISE,
                        client as *mut ::core::ffi::c_void,
                        c"auth query complete".as_ptr(),
                    );
                }
                (*(*client).link).set_resetting(true);
                sbuf_continue(&raw mut (*client).sbuf);
            }
            if (*server).state() as ::core::ffi::c_int == SV_FREE as ::core::ffi::c_int
                || (*server).state() as ::core::ffi::c_int == SV_JUSTFREE as ::core::ffi::c_int
            {
                return false;
            }
            return true;
        }
        69 => {
            log_server_error(c"S: error in auth_query".as_ptr(), pkt);
            disconnect_server(server, false, c"error response from auth_query".as_ptr());
            return false;
        }
        _ => {
            disconnect_server(
                server,
                false,
                c"unexpected response from auth_query".as_ptr(),
            );
            return false;
        }
    }
    sbuf_prepare_skip(&raw mut (*server).sbuf, (*pkt).len);
    true
}

unsafe extern "C" fn read_escaped_token(
    mut escaped_string_ptr: *mut *const ::core::ffi::c_char,
    mut unescaped_token: *mut MBuf,
) -> bool {
    let mut position = *escaped_string_ptr;
    let mut unwritten_start = position;
    while *position != 0 {
        if *position as ::core::ffi::c_int == '\\' as i32 {
            if !mbuf_write(
                unescaped_token,
                unwritten_start as *const ::core::ffi::c_void,
                position.offset_from(unwritten_start) as ::core::ffi::c_long as ::core::ffi::c_uint,
            ) {
                return false;
            }
            position = position.offset(1);
            unwritten_start = position;
            if *position == 0 {
                break;
            }
        } else if safe_isspace(*position as ::core::ffi::c_int) != 0 {
            break;
        }
        position = position.offset(1);
    }
    if !mbuf_write(
        unescaped_token,
        unwritten_start as *const ::core::ffi::c_void,
        position.offset_from(unwritten_start) as ::core::ffi::c_long as ::core::ffi::c_uint,
    ) {
        return false;
    }
    if !mbuf_write_byte(unescaped_token, '\0' as i32 as uint8_t) {
        return false;
    }
    *escaped_string_ptr = position;
    true
}

unsafe extern "C" fn set_startup_options(
    mut client: *mut PgSocket,
    mut options: *const ::core::ffi::c_char,
) -> bool {
    let mut current_block: u64;
    let mut arg_buf: [::core::ffi::c_char; 400] = [0; 400];
    let mut arg = MBuf {
        data: ::core::ptr::null_mut::<uint8_t>(),
        read_pos: 0,
        write_pos: 0,
        alloc_len: 0,
        reader: false,
        fixed: false,
    };
    let mut position = options;
    if (*client).replication as u64 != 0 {
        free((*client).startup_options as *mut ::core::ffi::c_void);
        (*client).startup_options = strdup(options);
        if (*client).startup_options.is_null() {
            disconnect_client(client, true, c"out of memory".as_ptr());
        }
        return true;
    }
    mbuf_init_fixed_writer(
        &raw mut arg,
        &raw mut arg_buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 400]>() as ::core::ffi::c_uint,
    );
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            client as *mut ::core::ffi::c_void,
            c"received options: %s".as_ptr(),
            options,
        );
    }
    loop {
        if *position == 0 {
            current_block = 10758786907990354186;
            break;
        }
        let mut start_position = position;
        let mut key_string = ::core::ptr::null::<::core::ffi::c_char>();
        let mut value_string = ::core::ptr::null::<::core::ffi::c_char>();
        let mut equals = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mbuf_rewind_writer(&raw mut arg);
        position = cstr_skip_ws(position as *mut ::core::ffi::c_char);
        if strncmp(c"-c".as_ptr(), position, 2 as size_t) == 0 as ::core::ffi::c_int {
            position = position.offset(2 as ::core::ffi::c_int as isize);
            position = cstr_skip_ws(position as *mut ::core::ffi::c_char);
        } else {
            if strncmp(c"--".as_ptr(), position, 2 as size_t) != 0 as ::core::ffi::c_int {
                current_block = 5829947976322203174;
                break;
            }
            position = position.offset(2 as ::core::ffi::c_int as isize);
        }
        if !read_escaped_token(&raw mut position, &raw mut arg) {
            if arg.fixed {
                mbuf_init_dynamic(&raw mut arg);
                position = start_position;
            } else {
                disconnect_client(client, true, c"out of memory".as_ptr());
                mbuf_free(&raw mut arg);
                return false;
            }
        } else {
            equals = strchr(arg.data as *mut ::core::ffi::c_char, '=' as i32);
            if equals.is_null() {
                current_block = 5829947976322203174;
                break;
            }
            *equals = '\0' as i32 as ::core::ffi::c_char;
            key_string = arg.data as *const ::core::ffi::c_char;
            value_string =
                (equals as *const ::core::ffi::c_char).offset(1 as ::core::ffi::c_int as isize);
            if varcache_set(&raw mut (*client).vars, key_string, value_string) {
                if cf_verbose > 0 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_DEBUG,
                        client as *mut ::core::ffi::c_void,
                        c"got var from options: %s=%s".as_ptr(),
                        key_string,
                        value_string,
                    );
                }
            } else if strlist_contains(cf_ignore_startup_params, key_string) as ::core::ffi::c_int
                != 0
                || strlist_contains(cf_ignore_startup_params, c"options".as_ptr())
                    as ::core::ffi::c_int
                    != 0
            {
                if cf_verbose > 0 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_DEBUG,
                        client as *mut ::core::ffi::c_void,
                        c"ignoring startup parameter from options: %s=%s".as_ptr(),
                        key_string,
                        value_string,
                    );
                }
            } else {
                log_generic(
                    LG_WARNING,
                    client as *mut ::core::ffi::c_void,
                    c"unsupported startup parameter in options: %s=%s".as_ptr(),
                    key_string,
                    value_string,
                );
                disconnect_client(
                    client,
                    true,
                    c"unsupported startup parameter in options: %s".as_ptr(),
                    key_string,
                );
                mbuf_free(&raw mut arg);
                return false;
            }
        }
    }
    match current_block {
        5829947976322203174 => {
            disconnect_client(
                client,
                true,
                c"unsupported options startup parameter: only '-c config=val' and '--config=val' are allowed".as_ptr(),
            );
            mbuf_free(&raw mut arg);
            false
        }
        _ => {
            mbuf_free(&raw mut arg);
            true
        }
    }
}

unsafe extern "C" fn set_appname(
    mut client: *mut PgSocket,
    mut app_name: *const ::core::ffi::c_char,
) {
    let mut buf: [::core::ffi::c_char; 400] = [0; 400];
    let mut abuf: [::core::ffi::c_char; 300] = [0; 300];
    let mut details = ::core::ptr::null::<::core::ffi::c_char>();
    if cf_application_name_add_host != 0 {
        if app_name.is_null() {
            app_name = c"app".as_ptr();
        }
        details = pga_details(
            &raw mut (*client).remote_addr,
            &raw mut abuf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 300]>() as ::core::ffi::c_int,
        );
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 400]>() as size_t,
            c"%s - %s".as_ptr(),
            app_name,
            details,
        );
        app_name = &raw mut buf as *mut ::core::ffi::c_char;
    }
    if !app_name.is_null() {
        if cf_verbose > 0 as ::core::ffi::c_int
        {
            log_generic(
                LG_DEBUG,
                client as *mut ::core::ffi::c_void,
                c"using application_name: %s".as_ptr(),
                app_name,
            );
        }
        varcache_set(
            &raw mut (*client).vars,
            c"application_name".as_ptr(),
            app_name,
        );
    }
}

unsafe extern "C" fn set_replication(
    mut client: *mut PgSocket,
    mut replicationString: *const ::core::ffi::c_char,
) -> bool {
    let mut replicationBool = false;
    if strcmp(replicationString, c"database".as_ptr()) == 0 as ::core::ffi::c_int {
        (*client).replication = REPLICATION_LOGICAL;
        return true;
    }
    if !parse_bool(replicationString, &raw mut replicationBool) {
        return false;
    }
    (*client).replication = (if replicationBool {
        REPLICATION_PHYSICAL as ::core::ffi::c_int
    } else {
        REPLICATION_NONE as ::core::ffi::c_int
    }) as ReplicationType;
    true
}

unsafe extern "C" fn decide_startup_pool(mut client: *mut PgSocket, mut pkt: *mut PktHdr) -> bool {
    let mut username = ::core::ptr::null::<::core::ffi::c_char>();
    let mut dbname = ::core::ptr::null::<::core::ffi::c_char>();
    let mut key = ::core::ptr::null::<::core::ffi::c_char>();
    let mut val = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ok: bool = false;
    let mut appname_found = false;
    let mut unsupported_protocol_extensions = MBuf {
        data: ::core::ptr::null_mut::<uint8_t>(),
        read_pos: 0,
        write_pos: 0,
        alloc_len: 0,
        reader: false,
        fixed: false,
    };
    let mut unsupported_protocol_extensions_count = 0 as ::core::ffi::c_int;
    let mut original_read_pos = (*pkt).data.read_pos;
    mbuf_init_dynamic(&raw mut unsupported_protocol_extensions);
    loop {
        ok = mbuf_get_string(&raw mut (*pkt).data, &raw mut key);
        if !ok || *key as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            break;
        }
        ok = mbuf_get_string(&raw mut (*pkt).data, &raw mut val);
        if !ok {
            break;
        }
        if strcmp(key, c"replication".as_ptr()) == 0 as ::core::ffi::c_int {
            if cf_verbose > 0 as ::core::ffi::c_int
            {
                log_generic(
                    LG_DEBUG,
                    client as *mut ::core::ffi::c_void,
                    c"got var: %s=%s".as_ptr(),
                    key,
                    val,
                );
            }
            set_replication(client, val);
        }
    }
    (*pkt).data.read_pos = original_read_pos;
    loop {
        ok = mbuf_get_string(&raw mut (*pkt).data, &raw mut key);
        if !ok || *key as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            break;
        }
        ok = mbuf_get_string(&raw mut (*pkt).data, &raw mut val);
        if !ok {
            break;
        }
        if strcmp(key, c"database".as_ptr()) == 0 as ::core::ffi::c_int {
            if cf_verbose > 0 as ::core::ffi::c_int
            {
                log_generic(
                    LG_DEBUG,
                    client as *mut ::core::ffi::c_void,
                    c"got var: %s=%s".as_ptr(),
                    key,
                    val,
                );
            }
            dbname = val;
        } else if strcmp(key, c"user".as_ptr()) == 0 as ::core::ffi::c_int {
            if cf_verbose > 0 as ::core::ffi::c_int
            {
                log_generic(
                    LG_DEBUG,
                    client as *mut ::core::ffi::c_void,
                    c"got var: %s=%s".as_ptr(),
                    key,
                    val,
                );
            }
            username = val;
        } else if strcmp(key, c"options".as_ptr()) == 0 as ::core::ffi::c_int {
            if !set_startup_options(client, val) {
                return false;
            }
        } else if strcmp(key, c"application_name".as_ptr()) == 0 as ::core::ffi::c_int {
            set_appname(client, val);
            appname_found = true;
        } else if strcmp(key, c"replication".as_ptr()) != 0 as ::core::ffi::c_int {
            if strncmp(c"_pq_.".as_ptr(), key, 5 as size_t) == 0 as ::core::ffi::c_int {
                if cf_verbose > 0 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_DEBUG,
                        client as *mut ::core::ffi::c_void,
                        c"ignoring protocol extension parameter: %s=%s".as_ptr(),
                        key,
                        val,
                    );
                }
                unsupported_protocol_extensions_count += 1;
                if !mbuf_write(
                    &raw mut unsupported_protocol_extensions,
                    key as *const ::core::ffi::c_void,
                    strlen(key).wrapping_add(1 as size_t) as ::core::ffi::c_uint,
                ) {
                    return false;
                }
            } else if varcache_set(&raw mut (*client).vars, key, val) {
                if cf_verbose > 0 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_DEBUG,
                        client as *mut ::core::ffi::c_void,
                        c"got var: %s=%s".as_ptr(),
                        key,
                        val,
                    );
                }
            } else if strlist_contains(cf_ignore_startup_params, key) {
                if cf_verbose > 0 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_DEBUG,
                        client as *mut ::core::ffi::c_void,
                        c"ignoring startup parameter: %s=%s".as_ptr(),
                        key,
                        val,
                    );
                }
            } else {
                log_generic(
                    LG_WARNING,
                    client as *mut ::core::ffi::c_void,
                    c"unsupported startup parameter: %s=%s".as_ptr(),
                    key,
                    val,
                );
                disconnect_client(
                    client,
                    true,
                    c"unsupported startup parameter: %s".as_ptr(),
                    key,
                );
                return false;
            }
        }
    }
    if !ok || mbuf_avail_for_read(&raw mut (*pkt).data) != 0 as ::core::ffi::c_uint {
        disconnect_client(
            client,
            true,
            c"invalid startup packet layout: expected terminator as last byte".as_ptr(),
        );
        return false;
    }
    if username.is_null() || *username == 0 {
        disconnect_client(client, true, c"no username supplied".as_ptr());
        return false;
    }
    if dbname.is_null() || *dbname == 0 {
        dbname = username;
    }
    if !appname_found {
        set_appname(client, ::core::ptr::null::<::core::ffi::c_char>());
    }
    if get_active_client_count() > cf_max_client_conn
        && strcmp(dbname, c"pgbouncer".as_ptr()) != 0 as ::core::ffi::c_int
    {
        disconnect_client(
            client,
            true,
            c"no more connections allowed (max_client_conn)".as_ptr(),
        );
        return false;
    }
    if (*pkt).type_0 == PKT_STARTUP_V3_UNSUPPORTED as ::core::ffi::c_uint
        || unsupported_protocol_extensions_count > 0 as ::core::ffi::c_int
    {
        let mut buf = pktbuf_dynamic(512 as ::core::ffi::c_int);
        let mut res: ::core::ffi::c_int = 0;
        pktbuf_write_generic(
            buf,
            PqMsg_NegotiateProtocolVersion,
            c"iib".as_ptr(),
            PKT_STARTUP_V3,
            unsupported_protocol_extensions_count,
            unsupported_protocol_extensions.data,
            unsupported_protocol_extensions.write_pos,
        );
        res = pktbuf_send_immediate(buf, client) as ::core::ffi::c_int;
        if res == 0 {
            pktbuf_free(buf);
            disconnect_client(
                client,
                false,
                c"unable to send protocol negotiation packet".as_ptr(),
            );
            return false;
        }
    }
    set_pool(
        client,
        dbname,
        username,
        ::core::ptr::null::<::core::ffi::c_char>(),
        false,
    )
}

unsafe extern "C" fn scram_client_first(
    mut client: *mut PgSocket,
    mut datalen: uint32_t,
    mut data: *const uint8_t,
) -> bool {
    let mut current_block: u64;
    let mut ibuf = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut input = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut res: ::core::ffi::c_int = 0;
    let mut user = (*client).login_user_credentials;
    ibuf = malloc(datalen.wrapping_add(1 as uint32_t) as size_t) as *mut ::core::ffi::c_char;
    if ibuf.is_null() {
        return false;
    }
    memcpy(
        ibuf as *mut ::core::ffi::c_void,
        data as *const ::core::ffi::c_void,
        datalen as size_t,
    );
    *ibuf.offset(datalen as isize) = '\0' as i32 as ::core::ffi::c_char;
    input = ibuf;
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            client as *mut ::core::ffi::c_void,
            b"SCRAM client-first-message = \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            input,
        );
    }
    if read_client_first_message(client, input) {
        if !(*user).mock_auth {
            if cf_verbose > 0 as ::core::ffi::c_int
            {
                log_generic(
                    LG_DEBUG,
                    client as *mut ::core::ffi::c_void,
                    b"stored secret = \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut (*user).passwd as *mut ::core::ffi::c_char,
                );
            }
            match get_password_type(&raw mut (*user).passwd as *mut ::core::ffi::c_char)
                as ::core::ffi::c_uint
            {
                1 => {
                    log_generic(
                        LG_ERROR,
                        client as *mut ::core::ffi::c_void,
                        c"SCRAM authentication failed: user has MD5 secret".as_ptr(),
                    );
                    current_block = 7404489773572977653;
                }
                0 | 2 | _ => {
                    current_block = 6009453772311597924;
                }
            }
        } else {
            current_block = 6009453772311597924;
        }
        match current_block {
            7404489773572977653 => {}
            _ => {
                if !build_server_first_message(
                    &raw mut (*client).scram_state,
                    user,
                    if (*user).mock_auth {
                        ::core::ptr::null_mut::<::core::ffi::c_char>()
                    } else {
                        &raw mut (*user).passwd as *mut ::core::ffi::c_char
                    },
                )
                .is_null()
                {
                    if cf_verbose > 0 as ::core::ffi::c_int
                    {
                        log_generic(
                            LG_DEBUG,
                            client as *mut ::core::ffi::c_void,
                            b"SCRAM server-first-message = \"%s\"\0" as *const u8
                                as *const ::core::ffi::c_char,
                            (*client).scram_state.server_first_message,
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
                        PqMsg_AuthenticationRequest,
                        c"ib".as_ptr(),
                        AUTH_REQ_SASL_CONT,
                        (*client).scram_state.server_first_message,
                        strlen((*client).scram_state.server_first_message),
                    );
                    res = pktbuf_send_immediate(&raw mut _buf, client) as ::core::ffi::c_int;
                    free(ibuf as *mut ::core::ffi::c_void);
                    return res != 0;
                }
            }
        }
    }
    free(ibuf as *mut ::core::ffi::c_void);
    false
}

unsafe extern "C" fn scram_client_final(
    mut client: *mut PgSocket,
    mut datalen: uint32_t,
    mut data: *const uint8_t,
) -> bool {
    let mut ibuf = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut input = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut client_final_nonce = ::core::ptr::null::<::core::ffi::c_char>();
    let mut proof = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut server_final_message = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut res: ::core::ffi::c_int = 0;
    ibuf = malloc(datalen.wrapping_add(1 as uint32_t) as size_t) as *mut ::core::ffi::c_char;
    if ibuf.is_null() {
        return false;
    }
    memcpy(
        ibuf as *mut ::core::ffi::c_void,
        data as *const ::core::ffi::c_void,
        datalen as size_t,
    );
    *ibuf.offset(datalen as isize) = '\0' as i32 as ::core::ffi::c_char;
    input = ibuf;
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            client as *mut ::core::ffi::c_void,
            b"SCRAM client-final-message = \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            input,
        );
    }
    if read_client_final_message(
        client,
        data,
        input,
        &raw mut client_final_nonce,
        &raw mut proof,
    ) {
        if cf_verbose > 0 as ::core::ffi::c_int
        {
            log_generic(
                LG_DEBUG,
                client as *mut ::core::ffi::c_void,
                b"SCRAM client-final-message-without-proof = \"%s\"\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*client).scram_state.client_final_message_without_proof,
            );
        }
        if !verify_final_nonce(&raw mut (*client).scram_state, client_final_nonce) {
            log_generic(
                LG_ERROR,
                client as *mut ::core::ffi::c_void,
                c"invalid SCRAM response (nonce does not match)".as_ptr(),
            );
        } else if !verify_client_proof(client, proof) || (*client).login_user_credentials.is_null()
        {
            log_generic(
                LG_ERROR,
                client as *mut ::core::ffi::c_void,
                c"password authentication failed".as_ptr(),
            );
        } else {
            server_final_message = build_server_final_message(client);
            if !server_final_message.is_null() {
                if cf_verbose > 0 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_DEBUG,
                        client as *mut ::core::ffi::c_void,
                        b"SCRAM server-final-message = \"%s\"\0" as *const u8
                            as *const ::core::ffi::c_char,
                        server_final_message,
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
                    PqMsg_AuthenticationRequest,
                    c"ib".as_ptr(),
                    AUTH_REQ_SASL_FIN,
                    server_final_message,
                    strlen(server_final_message),
                );
                res = pktbuf_send_immediate(&raw mut _buf, client) as ::core::ffi::c_int;
                free(server_final_message as *mut ::core::ffi::c_void);
                free(proof as *mut ::core::ffi::c_void);
                free(ibuf as *mut ::core::ffi::c_void);
                return res != 0;
            }
        }
    }
    free(proof as *mut ::core::ffi::c_void);
    free(ibuf as *mut ::core::ffi::c_void);
    false
}

unsafe extern "C" fn handle_client_startup(
    mut client: *mut PgSocket,
    mut pkt: *mut PktHdr,
) -> bool {
    let mut passwd = ::core::ptr::null::<::core::ffi::c_char>();
    let mut key = ::core::ptr::null::<uint8_t>();
    let mut ok: bool = false;
    let mut is_unix = pga_is_unix(&raw mut (*client).remote_addr);
    let mut sbuf: *mut SBuf = &raw mut (*client).sbuf;
    if incomplete_pkt(pkt) {
        if (*pkt).len > cf_sbuf_len as ::core::ffi::c_uint {
            (*client)
                .packet_cb_state
                .set_flag(CB_WANT_COMPLETE_PACKET as PacketCallbackFlag);
            sbuf_prepare_fetch(sbuf, (*pkt).len);
            return true;
        } else {
            return false;
        }
    }
    if (*client).wait_for_welcome()
        || (*client).wait_for_auth()
    {
        if finish_client_login(client) {
            if (*client).packet_cb_state.flag() as ::core::ffi::c_int
                != CB_HANDLE_COMPLETE_PACKET as ::core::ffi::c_int
            {
                sbuf_prepare_skip(sbuf, (*pkt).len);
            }
            return true;
        } else {
            return false;
        }
    }
    match (*pkt).type_0 {
        80877103 => {
            if cf_verbose > 1 as ::core::ffi::c_int
            {
                log_generic(
                    LG_NOISE,
                    client as *mut ::core::ffi::c_void,
                    c"C: req SSL".as_ptr(),
                );
            }
            if !(*client).sbuf.tls.is_null() {
                disconnect_client(client, false, c"SSL req inside SSL".as_ptr());
                return false;
            }
            if client_accept_sslmode != SSLMODE_DISABLED as ::core::ffi::c_int && !is_unix {
                if cf_verbose > 1 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_NOISE,
                        client as *mut ::core::ffi::c_void,
                        c"P: SSL ack".as_ptr(),
                    );
                }
                if !sbuf_answer(
                    &raw mut (*client).sbuf,
                    c"S".as_ptr() as *const ::core::ffi::c_void,
                    1 as size_t,
                ) {
                    disconnect_client(client, false, c"failed to ack SSL".as_ptr());
                    return false;
                }
                if !sbuf_tls_accept(&raw mut (*client).sbuf) {
                    disconnect_client(client, false, c"failed to accept SSL".as_ptr());
                    return false;
                }
            } else {
                if cf_verbose > 1 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_NOISE,
                        client as *mut ::core::ffi::c_void,
                        c"P: nak".as_ptr(),
                    );
                }
                if !sbuf_answer(
                    &raw mut (*client).sbuf,
                    c"N".as_ptr() as *const ::core::ffi::c_void,
                    1 as size_t,
                ) {
                    disconnect_client(client, false, c"failed to nak SSL".as_ptr());
                    return false;
                }
            }
        }
        80877104 => {
            if cf_verbose > 1 as ::core::ffi::c_int
            {
                log_generic(
                    LG_NOISE,
                    client as *mut ::core::ffi::c_void,
                    c"C: req GSS enc".as_ptr(),
                );
            }
            if !sbuf_answer(
                &raw mut (*client).sbuf,
                c"N".as_ptr() as *const ::core::ffi::c_void,
                1 as size_t,
            ) {
                disconnect_client(client, false, c"failed to nak GSS enc".as_ptr());
                return false;
            }
        }
        131072 => {
            disconnect_client(client, true, c"old V2 protocol not supported".as_ptr());
            return false;
        }
        196609 | 196608 => {
            if client_accept_sslmode >= SSLMODE_REQUIRE as ::core::ffi::c_int
                && (*client).sbuf.tls.is_null()
                && !is_unix
            {
                disconnect_client(client, true, c"SSL required".as_ptr());
                return false;
            }
            if !(*client).pool.is_null() && !sending_auth_query(client) {
                disconnect_client(client, true, c"client re-sent startup pkt".as_ptr());
                return false;
            }
            if (*client).wait_for_user() {
                (*client).set_wait_for_user(false);
                if !finish_set_pool(client, false) {
                    return false;
                }
            } else if !decide_startup_pool(client, pkt) {
                return false;
            }
        }
        112 => {
            if (*client).login_user_credentials.is_null() {
                disconnect_client(
                    client,
                    true,
                    c"client password pkt before startup packet".as_ptr(),
                );
                return false;
            }
            if (*client).client_auth_type == AUTH_TYPE_SCRAM_SHA_256 as ::core::ffi::c_int {
                let mut mech = ::core::ptr::null::<::core::ffi::c_char>();
                let mut length: uint32_t = 0;
                let mut data = ::core::ptr::null::<uint8_t>();
                if (*client).scram_state.server_nonce.is_null() {
                    if !mbuf_get_string(&raw mut (*pkt).data, &raw mut mech) {
                        return false;
                    }
                    if cf_verbose > 0 as ::core::ffi::c_int
                    {
                        log_generic(
                            LG_DEBUG,
                            client as *mut ::core::ffi::c_void,
                            c"C: selected SASL mechanism: %s".as_ptr(),
                            mech,
                        );
                    }
                    if strcmp(mech, c"SCRAM-SHA-256".as_ptr()) != 0 as ::core::ffi::c_int {
                        disconnect_client(
                            client,
                            true,
                            c"client selected an invalid SASL authentication mechanism".as_ptr(),
                        );
                        return false;
                    }
                    if !mbuf_get_uint32be(&raw mut (*pkt).data, &raw mut length) {
                        return false;
                    }
                    if !mbuf_get_bytes(
                        &raw mut (*pkt).data,
                        length as ::core::ffi::c_uint,
                        &raw mut data,
                    ) {
                        return false;
                    }
                    if !scram_client_first(client, length, data) {
                        disconnect_client(client, true, c"SASL authentication failed".as_ptr());
                        return false;
                    }
                } else {
                    length = mbuf_avail_for_read(&raw mut (*pkt).data) as uint32_t;
                    if !mbuf_get_bytes(
                        &raw mut (*pkt).data,
                        length as ::core::ffi::c_uint,
                        &raw mut data,
                    ) {
                        return false;
                    }
                    if scram_client_final(client, length, data) {
                        if !(*client).scram_state.adhoc && !(*(*client).c2rust_unnamed.db).fake {
                            memcpy(
                                &raw mut (*(*(*client).pool).user_credentials).scram_ClientKey
                                    as *mut uint8_t
                                    as *mut ::core::ffi::c_void,
                                &raw mut (*client).scram_state.ClientKey as *mut uint8_t
                                    as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
                            );
                            memcpy(
                                &raw mut (*(*(*client).pool).user_credentials).scram_ServerKey
                                    as *mut uint8_t
                                    as *mut ::core::ffi::c_void,
                                &raw mut (*client).scram_state.ServerKey as *mut uint8_t
                                    as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
                            );
                            (*(*(*client).pool).user_credentials).use_scram_keys = true;
                        }
                        free_scram_state(&raw mut (*client).scram_state);
                        if !finish_client_login(client) {
                            return false;
                        }
                    } else {
                        disconnect_client(client, true, c"SASL authentication failed".as_ptr());
                        return false;
                    }
                }
            } else {
                ok = mbuf_get_string(&raw mut (*pkt).data, &raw mut passwd);
                if ok {
                    if *passwd == 0 {
                        disconnect_client(
                            client,
                            true,
                            c"empty password returned by client".as_ptr(),
                        );
                        return false;
                    }
                    if (*client).client_auth_type == AUTH_TYPE_LDAP as ::core::ffi::c_int {
                        if !sbuf_pause(&raw mut (*client).sbuf) {
                            disconnect_client(client, true, c"pause failed".as_ptr());
                            return false;
                        }
                        ldap_auth_begin(client, passwd);
                        return false;
                    }
                    if (*client).client_auth_type == AUTH_TYPE_PAM as ::core::ffi::c_int {
                        if !sbuf_pause(&raw mut (*client).sbuf) {
                            disconnect_client(client, true, c"pause failed".as_ptr());
                            return false;
                        }
                        pam_auth_begin(client, passwd);
                        return false;
                    }
                    if check_client_passwd(client, passwd) {
                        if !finish_client_login(client) {
                            return false;
                        }
                    } else {
                        disconnect_client(client, true, c"password authentication failed".as_ptr());
                        return false;
                    }
                }
            }
        }
        80877102 => {
            if mbuf_avail_for_read(&raw mut (*pkt).data) == BACKENDKEY_LEN as ::core::ffi::c_uint
                && mbuf_get_bytes(
                    &raw mut (*pkt).data,
                    BACKENDKEY_LEN as ::core::ffi::c_uint,
                    &raw mut key,
                ) as ::core::ffi::c_int
                    != 0
            {
                memcpy(
                    &raw mut (*client).cancel_key as *mut uint8_t as *mut ::core::ffi::c_void,
                    key as *const ::core::ffi::c_void,
                    BACKENDKEY_LEN as size_t,
                );
                accept_cancel_request(client);
            } else {
                disconnect_client(client, false, c"bad cancel request".as_ptr());
            }
            return false;
        }
        _ => {
            disconnect_client(client, false, c"bad packet".as_ptr());
            return false;
        }
    }
    if (*client).packet_cb_state.flag() as ::core::ffi::c_int
        != CB_HANDLE_COMPLETE_PACKET as ::core::ffi::c_int
    {
        sbuf_prepare_skip(sbuf, (*pkt).len);
    }
    (*client).request_time = get_cached_time();
    true
}

unsafe extern "C" fn handle_client_work(mut client: *mut PgSocket, mut pkt: *mut PktHdr) -> bool {
    let mut sbuf: *mut SBuf = &raw mut (*client).sbuf;
    let mut track_outstanding = false_0;
    let mut ps_action = PS_IGNORE;
    let mut close_packet = PgClosePacket {
        type_0: 0,
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    match (*pkt).type_0 {
        81 => {
            if cf_disable_pqexec != 0 {
                log_generic(
                    LG_ERROR,
                    client as *mut ::core::ffi::c_void,
                    b"client used \"Query\" packet type\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                disconnect_client(client, true, c"PQexec disallowed".as_ptr());
                return false;
            }
            track_outstanding = true_0;
        }
        70 => {
            track_outstanding = true_0;
        }
        83 => {
            track_outstanding = true_0;
        }
        99 | 102 => {
            track_outstanding = true_0;
        }
        80 => {
            track_outstanding = true_0;
            if connection_pool_mode(client) != POOL_SESSION
                && cf_max_prepared_statements != 0 as ::core::ffi::c_int
            {
                ps_action = inspect_parse_packet(client, pkt);
                pkt_rewind_v3(pkt);
            }
        }
        69 => {
            track_outstanding = true_0;
        }
        67 => {
            track_outstanding = true_0;
            if connection_pool_mode(client) != POOL_SESSION
                && cf_max_prepared_statements != 0 as ::core::ffi::c_int
            {
                ps_action = inspect_describe_or_close_packet(client, pkt);
                pkt_rewind_v3(pkt);
            }
        }
        66 => {
            track_outstanding = true_0;
            if connection_pool_mode(client) != POOL_SESSION
                && cf_max_prepared_statements != 0 as ::core::ffi::c_int
            {
                ps_action = inspect_bind_packet(client, pkt);
                pkt_rewind_v3(pkt);
            }
        }
        68 => {
            track_outstanding = true_0;
            if connection_pool_mode(client) != POOL_SESSION
                && cf_max_prepared_statements != 0 as ::core::ffi::c_int
            {
                ps_action = inspect_describe_or_close_packet(client, pkt);
                pkt_rewind_v3(pkt);
            }
        }
        72 | 100 => {}
        88 => {
            disconnect_client(client, false, c"client close request".as_ptr());
            return false;
        }
        _ => {
            log_generic(
                LG_ERROR,
                client as *mut ::core::ffi::c_void,
                c"unknown pkt from client: %u/0x%x".as_ptr(),
                (*pkt).type_0,
                (*pkt).type_0,
            );
            disconnect_client(client, true, c"unknown pkt".as_ptr());
            return false;
        }
    }
    if ps_action as ::core::ffi::c_uint
        == PS_HANDLE_FULL_PACKET as ::core::ffi::c_int as ::core::ffi::c_uint
        && incomplete_pkt(pkt)
    {
        if (*pkt).len > cf_sbuf_len as ::core::ffi::c_uint {
            (*client)
                .packet_cb_state
                .set_flag(CB_WANT_COMPLETE_PACKET as PacketCallbackFlag);
            sbuf_prepare_fetch(sbuf, (*pkt).len);
            return true;
        } else {
            return false;
        }
    }
    if ps_action as ::core::ffi::c_uint
        == PS_INSPECT_FAILED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if !incomplete_pkt(pkt) {
            log_generic(
                LG_ERROR,
                client as *mut ::core::ffi::c_void,
                c"failed to parse prepared statement packet type '%c'".as_ptr(),
                (*pkt).type_0,
            );
            disconnect_client(client, true, c"failed to parse packet".as_ptr());
            return false;
        }
        if (*pkt).data.write_pos >= cf_sbuf_len as ::core::ffi::c_uint {
            (*client)
                .packet_cb_state
                .set_flag(CB_WANT_COMPLETE_PACKET as PacketCallbackFlag);
            sbuf_prepare_fetch(sbuf, (*pkt).len);
            return true;
        }
        return false;
    }
    if ps_action as ::core::ffi::c_uint != PS_IGNORE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*pkt).type_0 == PqMsg_Close as ::core::ffi::c_uint
    {
        if !unmarshall_close_packet(client, pkt, &raw mut close_packet) {
            return false;
        }
        if is_close_named_statement_packet(&raw mut close_packet) {
            if !handle_close_statement_command(client, pkt, &raw mut close_packet) {
                return false;
            }
            (*(*client).pool).stats.client_bytes = (*(*client).pool)
                .stats
                .client_bytes
                .wrapping_add((*pkt).len as uint64_t);
            return true;
        }
    }
    if (*client).query_start == 0 {
        (*(*client).pool).stats.query_count = (*(*client).pool).stats.query_count.wrapping_add(1);
        (*client).query_start = get_cached_time();
    }
    if (*client).xact_start == 0 {
        (*(*client).pool).stats.xact_count = (*(*client).pool).stats.xact_count.wrapping_add(1);
        (*client).xact_start = (*client).query_start;
    }
    if (*(*(*client).pool).db).admin {
        return admin_handle_client(client, pkt);
    }
    if !find_server(client) {
        return false;
    }
    (*(*client).pool).stats.client_bytes = (*(*client).pool)
        .stats
        .client_bytes
        .wrapping_add((*pkt).len as uint64_t);
    (*(*client).link).set_ready(false);
    (*(*client).link).set_idle_tx(false);
    if ps_action as ::core::ffi::c_uint != PS_IGNORE as ::core::ffi::c_int as ::core::ffi::c_uint {
        if !sbuf_flush(sbuf) {
            return false;
        }
        match (*pkt).type_0 {
            80 => return handle_parse_command(client, pkt),
            66 => return handle_bind_command(client, pkt),
            68 => return handle_describe_command(client, pkt),
            _ => {}
        }
        return true;
    }
    if track_outstanding != 0
        && !add_outstanding_request(client, (*pkt).type_0 as ::core::ffi::c_char, RA_FORWARD)
    {
        return false;
    }
    if (*client).packet_cb_state.flag() as ::core::ffi::c_int
        == CB_HANDLE_COMPLETE_PACKET as ::core::ffi::c_int
    {
        if !sbuf_flush(sbuf) {
            return false;
        }
        if !sbuf_queue_full_packet(
            &raw mut (*client).sbuf,
            &raw mut (*(*client).link).sbuf,
            pkt,
        ) {
            disconnect_client(client, true, c"out of memory".as_ptr());
            disconnect_server((*client).link, true, c"out of memory".as_ptr());
            return false;
        }
        return true;
    }
    sbuf_prepare_send(sbuf, &raw mut (*(*client).link).sbuf, (*pkt).len);
    true
}

unsafe extern "C" fn expect_startup_packet(mut client: *mut PgSocket) -> bool {
    match (*client).state() as ::core::ffi::c_int {
        2 => true,
        5 => {
            if (*client).wait_for_welcome() {
                true
            } else {
                false
            }
        }
        3 => {
            let mut _log_ctx = NULL;
            log_fatal(
                c"src/client.c".as_ptr(),
                1695 as ::core::ffi::c_int,
                c"expect_startup_packet".as_ptr(),
                false,
                _log_ctx,
                c"why waiting client in client_proto()".as_ptr(),
            );
            exit(1 as ::core::ffi::c_int);
        }
        6 | 7 => {
            let mut _log_ctx_0 = NULL;
            log_fatal(
                c"src/client.c".as_ptr(),
                1698 as ::core::ffi::c_int,
                c"expect_startup_packet".as_ptr(),
                false,
                _log_ctx_0,
                c"why canceling client in client_proto()".as_ptr(),
            );
            exit(1 as ::core::ffi::c_int);
        }
        _ => {
            let mut _log_ctx_1 = NULL;
            log_fatal(
                c"src/client.c".as_ptr(),
                1700 as ::core::ffi::c_int,
                c"expect_startup_packet".as_ptr(),
                false,
                _log_ctx_1,
                c"bad client state: %d".as_ptr(),
                (*client).state() as ::core::ffi::c_int,
            );
            exit(1 as ::core::ffi::c_int);
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn client_proto(
    mut sbuf: *mut SBuf,
    mut evtype: SBufEvent,
    mut data: *mut MBuf,
) -> bool {
    let mut res = false;
    let mut client = (sbuf as *mut ::core::ffi::c_char)
        .offset(-(520 as ::core::ffi::c_ulong as isize)) as *mut PgSocket;
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
    if (*client).state() as ::core::ffi::c_int == CL_JUSTFREE as ::core::ffi::c_int {
        return false;
    }
    match evtype as ::core::ffi::c_uint {
        4 | 3 | 1 => {
            if (*client).state() as ::core::ffi::c_int == CL_LOGIN as ::core::ffi::c_int
                && mbuf_avail_for_read(data) == 0 as ::core::ffi::c_uint
            {
                disconnect_client(client, false, ::core::ptr::null::<::core::ffi::c_char>());
            } else {
                disconnect_client(client, false, c"client unexpected eof".as_ptr());
            }
        }
        2 => {
            disconnect_server((*client).link, false, c"server connection closed".as_ptr());
        }
        0 => {
            if incomplete_header(data) {
                if cf_verbose > 1 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_NOISE,
                        client as *mut ::core::ffi::c_void,
                        c"C: got partial header, trying to wait a bit".as_ptr(),
                    );
                }
                return false;
            }
            if !get_header(data, &raw mut pkt) {
                let mut hex: [::core::ffi::c_char; 17] = [0; 17];
                disconnect_client(
                    client,
                    true,
                    c"bad packet header: '%s'".as_ptr(),
                    hdr2hex(
                        data,
                        &raw mut hex as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 17]>() as ::core::ffi::c_uint,
                    ),
                );
                return false;
            }
            if cf_verbose > 1 as ::core::ffi::c_int
            {
                log_generic(
                    LG_NOISE,
                    client as *mut ::core::ffi::c_void,
                    c"read pkt='%c' len=%u".as_ptr(),
                    pkt_desc(&raw mut pkt) as ::core::ffi::c_int,
                    pkt.len,
                );
            }
            if pkt.type_0 == PKT_SSLREQ as ::core::ffi::c_uint
                && mbuf_avail_for_read(data) > 0 as ::core::ffi::c_uint
            {
                disconnect_client(
                    client,
                    true,
                    c"received unencrypted data after SSL request".as_ptr(),
                );
                return false;
            }
            if pkt.type_0 == PKT_GSSENCREQ as ::core::ffi::c_uint
                && mbuf_avail_for_read(data) > 0 as ::core::ffi::c_uint
            {
                disconnect_client(
                    client,
                    true,
                    c"received unencrypted data after GSSAPI encryption request".as_ptr(),
                );
                return false;
            }
            (*client).request_time = get_cached_time();
            if expect_startup_packet(client) {
                res = handle_client_startup(client, &raw mut pkt);
            } else {
                res = handle_client_work(client, &raw mut pkt);
            }
        }
        6 => {
            let mut first = false;
            if (*client).packet_cb_state.pkt.type_0 == 0 as ::core::ffi::c_uint {
                first = true;
                if !get_header(data, &raw mut (*client).packet_cb_state.pkt) {
                    let mut hex_0: [::core::ffi::c_char; 17] = [0; 17];
                    disconnect_client(
                        client,
                        true,
                        c"bad packet header: '%s'".as_ptr(),
                        hdr2hex(
                            data,
                            &raw mut hex_0 as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 17]>()
                                as ::core::ffi::c_uint,
                        ),
                    );
                    return false;
                }
                mbuf_rewind_reader(data);
            }
            let mut current_block_71: u64;
            match (*client).packet_cb_state.flag() as ::core::ffi::c_int {
                1 => {
                    if first {
                        if cf_verbose > 0 as ::core::ffi::c_int
                        {
                            log_generic(
                                LG_DEBUG,
                                client as *mut ::core::ffi::c_void,
                                c"buffering complete packet, pkt='%c' len=%d incomplete=%s available=%d".as_ptr(),
                                pkt_desc(&raw mut (*client).packet_cb_state.pkt)
                                    as ::core::ffi::c_int,
                                (*client).packet_cb_state.pkt.len,
                                if incomplete_pkt(&raw mut (*client).packet_cb_state.pkt)
                                {
                                    c"true".as_ptr()
                                } else {
                                    c"false".as_ptr()
                                },
                                mbuf_avail_for_read(data),
                            );
                        }
                        mbuf_init_dynamic(&raw mut (*client).packet_cb_state.pkt.data);
                        if !mbuf_make_room(
                            &raw mut (*client).packet_cb_state.pkt.data,
                            (*client).packet_cb_state.pkt.len,
                        ) {
                            return false;
                        }
                    }
                    if !mbuf_write_raw_mbuf(&raw mut (*client).packet_cb_state.pkt.data, data) {
                        return false;
                    }
                    if (*sbuf).pkt_remain != mbuf_avail_for_read(data) {
                        res = true;
                        current_block_71 = 14001958660280927786;
                    } else {
                        (*client)
                            .packet_cb_state
                            .set_flag(CB_HANDLE_COMPLETE_PACKET as PacketCallbackFlag);
                        current_block_71 = 1560471608298357361;
                    }
                }
                2 => {
                    current_block_71 = 1560471608298357361;
                }
                _ => {
                    disconnect_client(client, true, c"BUG: unknown packet callback flag".as_ptr());
                    current_block_71 = 14001958660280927786;
                }
            }
            if current_block_71 == 1560471608298357361 {
                if expect_startup_packet(client) {
                    pkt_rewind_v2(&raw mut (*client).packet_cb_state.pkt);
                    res = handle_client_startup(client, &raw mut (*client).packet_cb_state.pkt);
                } else {
                    pkt_rewind_v3(&raw mut (*client).packet_cb_state.pkt);
                    res = handle_client_work(client, &raw mut (*client).packet_cb_state.pkt);
                }
                if !res {
                    return false;
                }
                (*client)
                    .packet_cb_state
                    .set_flag(CB_NONE as PacketCallbackFlag);
                free_header(&raw mut (*client).packet_cb_state.pkt);
            }
        }
        7 => {
            sbuf_continue(&raw mut (*client).sbuf);
            res = true;
        }
        5 | _ => {}
    }
    res
}

extern "C" {
    pub fn get_cached_time() -> usec_t;
}

extern "C" {
    pub fn varcache_set(
        cache: *mut VarCache,
        key: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
    ) -> bool;
}
