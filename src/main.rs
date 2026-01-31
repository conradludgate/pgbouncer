#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(extern_types, raw_ref_op, register_tool)]
#[macro_use]
extern crate c2rust_bitfields;
#[allow(unused_imports)]
use ::pgbouncer;

pub mod _stdio_h {

    pub type fpos_t = __darwin_off_t;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct __sbuf {
        pub _base: *mut ::core::ffi::c_uchar,
        pub _size: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct __sFILE {
        pub _p: *mut ::core::ffi::c_uchar,
        pub _r: ::core::ffi::c_int,
        pub _w: ::core::ffi::c_int,
        pub _flags: ::core::ffi::c_short,
        pub _file: ::core::ffi::c_short,
        pub _bf: __sbuf,
        pub _lbfsize: ::core::ffi::c_int,
        pub _cookie: *mut ::core::ffi::c_void,
        pub _close: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
        pub _read: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub _seek: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void, fpos_t, ::core::ffi::c_int) -> fpos_t,
        >,
        pub _write: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub _ub: __sbuf,
        pub _extra: *mut __sFILEX,
        pub _ur: ::core::ffi::c_int,
        pub _ubuf: [::core::ffi::c_uchar; 3],
        pub _nbuf: [::core::ffi::c_uchar; 1],
        pub _lb: __sbuf,
        pub _blksize: ::core::ffi::c_int,
        pub _offset: fpos_t,
    }

    pub type FILE = __sFILE;
    use pgbouncer::types::__darwin_off_t;
    use pgbouncer::types::size_t;
    extern "C" {

        pub type __sFILEX;

        pub static mut __stderrp: *mut FILE;

        pub fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;

        pub fn snprintf(
            __str: *mut ::core::ffi::c_char,
            __size: size_t,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}

pub mod resource_h {

    pub type rlim_t = __uint64_t;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }

    pub const RLIMIT_NOFILE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    use pgbouncer::types::__uint64_t;
    extern "C" {

        pub fn getrlimit(_: ::core::ffi::c_int, _: *mut rlimit) -> ::core::ffi::c_int;
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

    pub struct CfOps {
        pub setter: Option<unsafe extern "C" fn(*mut CfValue, *const ::core::ffi::c_char) -> bool>,
        pub getter: Option<unsafe extern "C" fn(*mut CfValue) -> *const ::core::ffi::c_char>,
        pub op_extra: *const ::core::ffi::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct CfKey {
        pub key_name: *const ::core::ffi::c_char,
        pub op: CfOps,
        pub flags: ::core::ffi::c_int,
        pub key_ofs: uintptr_t,
        pub def_value: *const ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct CfSect {
        pub sect_name: *const ::core::ffi::c_char,
        pub key_list: *const CfKey,
        pub base_lookup: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub set_key: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> bool,
        >,
        pub get_key: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                *mut ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> *const ::core::ffi::c_char,
        >,
        pub section_start: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char) -> bool,
        >,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct CfContext {
        pub sect_list: *const CfSect,
        pub base: *mut ::core::ffi::c_void,
        pub loaded: bool,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct CfLookup {
        pub name: *const ::core::ffi::c_char,
        pub value: ::core::ffi::c_int,
    }

    pub const CF_VAL_ABS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const CF_NO_RELOAD: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;

    pub const CF_READONLY: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    use pgbouncer::types::uintptr_t;
    extern "C" {

        pub fn cf_set_str(cv: *mut CfValue, value: *const ::core::ffi::c_char) -> bool;

        pub fn cf_set_int(cv: *mut CfValue, value: *const ::core::ffi::c_char) -> bool;

        pub fn cf_set_uint(cv: *mut CfValue, value: *const ::core::ffi::c_char) -> bool;

        pub fn cf_set_time_usec(cv: *mut CfValue, value: *const ::core::ffi::c_char) -> bool;

        pub fn cf_set_lookup(cv: *mut CfValue, value: *const ::core::ffi::c_char) -> bool;

        pub fn cf_get_str(cv: *mut CfValue) -> *const ::core::ffi::c_char;

        pub fn cf_get_int(cv: *mut CfValue) -> *const ::core::ffi::c_char;

        pub fn cf_get_uint(cv: *mut CfValue) -> *const ::core::ffi::c_char;

        pub fn cf_get_time_usec(cv: *mut CfValue) -> *const ::core::ffi::c_char;

        pub fn cf_get_lookup(cv: *mut CfValue) -> *const ::core::ffi::c_char;

        pub fn cf_load_file(cf: *const CfContext, fn_0: *const ::core::ffi::c_char) -> bool;

        pub fn cf_get(
            cf: *const CfContext,
            sect: *const ::core::ffi::c_char,
            var: *const ::core::ffi::c_char,
            buf: *mut ::core::ffi::c_char,
            buflen: ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char;

        pub fn cf_set(
            cf: *const CfContext,
            sect: *const ::core::ffi::c_char,
            var: *const ::core::ffi::c_char,
            val: *const ::core::ffi::c_char,
        ) -> bool;
    }
}

pub mod un_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct sockaddr_un {
        pub sun_len: ::core::ffi::c_uchar,
        pub sun_family: sa_family_t,
        pub sun_path: [::core::ffi::c_char; 104],
    }
    use pgbouncer::types::sa_family_t;
}

pub mod event_h {

    pub type event_callback_fn = Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_short,
            *mut ::core::ffi::c_void,
        ) -> (),
    >;

    pub const EVLOOP_ONCE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

    pub const EV_SIGNAL: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

    pub const EV_PERSIST: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    use super::event_struct_h::event;
    use pgbouncer::types::timeval;
    extern "C" {

        pub type event_base;

        pub fn event_base_new() -> *mut event_base;

        pub fn event_base_get_method(_: *const event_base) -> *const ::core::ffi::c_char;

        pub fn event_base_free(_: *mut event_base);

        pub fn event_base_loop(_: *mut event_base, _: ::core::ffi::c_int) -> ::core::ffi::c_int;

        pub fn event_assign(
            _: *mut event,
            _: *mut event_base,
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_short,
            _: event_callback_fn,
            _: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;

        pub fn event_add(ev: *mut event, timeout: *const timeval) -> ::core::ffi::c_int;

        pub fn event_get_version() -> *const ::core::ffi::c_char;
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
    use pgbouncer::types::timeval;
    use pgbouncer::types::uint8_t;
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
    // PgStats moved to pgbouncer::src::common::types
    pub use ::pgbouncer::src::common::types::PgStats;
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

    pub type auth_type = ::core::ffi::c_uint;

    pub const AUTH_TYPE_REJECT: auth_type = 10;

    pub const AUTH_TYPE_PEER: auth_type = 9;

    pub const AUTH_TYPE_SCRAM_SHA_256: auth_type = 8;

    pub const AUTH_TYPE_PAM: auth_type = 7;

    pub const AUTH_TYPE_LDAP: auth_type = 6;

    pub const AUTH_TYPE_HBA: auth_type = 5;

    pub const AUTH_TYPE_CERT: auth_type = 4;

    pub const AUTH_TYPE_MD5: auth_type = 3;

    pub const AUTH_TYPE_PLAIN: auth_type = 2;

    pub const AUTH_TYPE_TRUST: auth_type = 1;

    pub const AUTH_TYPE_ANY: auth_type = 0;

    pub const POOL_SESSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const POOL_TX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    pub const POOL_STMT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    use super::dnslookup_h::DNSToken;
    use pgbouncer::types::sockaddr_in6;
    use pgbouncer::types::sockaddr_in;
    use super::pktbuf_h::PktBuf;
    use super::sbuf_h::SBuf;
    use pgbouncer::types::sockaddr;
    use pgbouncer::types::pg_cryptohash_type;
    use pgbouncer::types::pid_t;
    use pgbouncer::types::uid_t;
    use pgbouncer::types::uint16_t;
    use pgbouncer::types::uint64_t;
    use pgbouncer::types::uint8_t;
    use pgbouncer::types::usec_t;
    use pgbouncer::types::List;
    use pgbouncer::types::PktHdr;
    use pgbouncer::types::StatList;
    use pgbouncer::types::VarCache;
    use pgbouncer::types::{AANode, AATree};
    use pgbouncer::types::{PgClientPreparedStatement, PgServerPreparedStatement};
    extern "C" {

        pub static mut any_user_level_timeout_set: bool;

        pub static mut any_user_level_client_timeout_set: bool;
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
    use pgbouncer::types::tls;
    use pgbouncer::types::size_t;
    use pgbouncer::types::ssize_t;
    use pgbouncer::types::uint8_t;
    use pgbouncer::types::MBuf;
    extern "C" {

        pub fn sbuf_tls_setup() -> bool;
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
    use pgbouncer::types::uint8_t;
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
    use pgbouncer::types::uint8_t;
}

pub mod dnslookup_h {
    extern "C" {

        pub type DNSToken;

        pub type DNSContext;

        pub fn adns_create_context() -> *mut DNSContext;

        pub fn adns_get_backend() -> *const ::core::ffi::c_char;

        pub fn adns_per_loop(ctx: *mut DNSContext);
    }
}

pub mod hba_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct HBA {
        pub rules: List,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct Ident {
        pub maps: List,
    }
    use pgbouncer::types::List;
    extern "C" {

        pub fn ident_load_map(fn_0: *const ::core::ffi::c_char) -> *mut Ident;

        pub fn ident_free(ident: *mut Ident);

        pub fn hba_load_rules(fn_0: *const ::core::ffi::c_char, ident: *mut Ident) -> *mut HBA;

        pub fn hba_free(hba: *mut HBA);
    }
}

pub mod getopt_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct option {
        pub name: *const ::core::ffi::c_char,
        pub has_arg: ::core::ffi::c_int,
        pub flag: *mut ::core::ffi::c_int,
        pub val: ::core::ffi::c_int,
    }

    pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    extern "C" {

        pub fn getopt_long(
            __argc: ::core::ffi::c_int,
            _: *const *mut ::core::ffi::c_char,
            _: *const ::core::ffi::c_char,
            _: *const option,
            _: *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}

pub mod unistd_h {
    use pgbouncer::types::pid_t;
    use pgbouncer::types::size_t;
    use pgbouncer::types::ssize_t;
    use pgbouncer::types::uid_t;
    extern "C" {

        pub fn _exit(_: ::core::ffi::c_int) -> !;

        pub fn close(_: ::core::ffi::c_int) -> ::core::ffi::c_int;

        pub fn dup2(_: ::core::ffi::c_int, _: ::core::ffi::c_int) -> ::core::ffi::c_int;

        pub fn fork() -> pid_t;

        pub fn getpid() -> pid_t;

        pub fn getuid() -> uid_t;

        pub fn read(_: ::core::ffi::c_int, _: *mut ::core::ffi::c_void, __nbyte: size_t)
            -> ssize_t;

        pub fn setsid() -> pid_t;

        pub fn unlink(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;

        pub static mut optarg: *mut ::core::ffi::c_char;

        pub static mut optind: ::core::ffi::c_int;
    }
}

pub mod _time_h {
    use pgbouncer::types::time_t;
    extern "C" {

        pub fn time(_: *mut time_t) -> time_t;
    }
}

pub mod util_h {
    use super::cfparser_h::CfValue;
    use pgbouncer::types::LogLevel;
    extern "C" {

        pub fn log_socket_prefix(
            lev: LogLevel,
            ctx: *mut ::core::ffi::c_void,
            dst: *mut ::core::ffi::c_char,
            dstlen: ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int;

        pub fn rescue_timers();

        pub fn cf_set_authdb(cv: *mut CfValue, value: *const ::core::ffi::c_char) -> bool;
    }
}

pub mod objects_h {

    use pgbouncer::types::StatList;
    extern "C" {

        pub static mut user_list: StatList;

        pub static mut database_list: StatList;

        pub static mut peer_list: StatList;

        pub fn reuse_just_freed_objects();

        pub fn init_objects();

        pub fn init_caches();
    }
}

pub mod include_signal_h {
    #[inline(always)]

    pub unsafe extern "C" fn __sigbits(mut __signo: ::core::ffi::c_int) -> ::core::ffi::c_int {
        if __signo > __DARWIN_NSIG {
            0 as ::core::ffi::c_int
        } else {
            (1 as ::core::ffi::c_int) << (__signo - 1 as ::core::ffi::c_int)
        }
    }
    use pgbouncer::types::sigset_t;
    use super::signal_h::__DARWIN_NSIG;
    use pgbouncer::types::pid_t;
    extern "C" {

        pub fn kill(_: pid_t, _: ::core::ffi::c_int) -> ::core::ffi::c_int;

        pub fn sigprocmask(
            _: ::core::ffi::c_int,
            _: *const sigset_t,
            _: *mut sigset_t,
        ) -> ::core::ffi::c_int;
    }
}

pub mod config_h {

    pub const PACKAGE_BUGREPORT: [::core::ffi::c_char; 46] = unsafe {
        ::core::mem::transmute::<[u8; 46], [::core::ffi::c_char; 46]>(
            *b"https://github.com/pgbouncer/pgbouncer/issues\0",
        )
    };

    pub const PACKAGE_NAME: [::core::ffi::c_char; 10] =
        unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"PgBouncer\0") };

    pub const PACKAGE_STRING: [::core::ffi::c_char; 17] = unsafe {
        ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"PgBouncer 1.25.1\0")
    };

    pub const PACKAGE_URL: [::core::ffi::c_char; 27] = unsafe {
        ::core::mem::transmute::<[u8; 27], [::core::ffi::c_char; 27]>(
            *b"https://www.pgbouncer.org/\0",
        )
    };
}

pub mod signal_h {

    pub const __DARWIN_NSIG: ::core::ffi::c_int = 32 as ::core::ffi::c_int;

    pub const SIG_BLOCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
}

pub mod _printf_h {
    extern "C" {

        pub fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    }
}

pub mod errno_h {

    pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

    pub const ESRCH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

    pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    extern "C" {

        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}

pub mod system_h {
    extern "C" {

        pub fn change_user(user: *const ::core::ffi::c_char);
    }
}

pub mod fcntl_h {

    pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const O_WRONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

    pub const O_RDWR: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

    pub const O_CREAT: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;

    pub const O_EXCL: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
    extern "C" {

        pub fn open(
            _: *const ::core::ffi::c_char,
            _: ::core::ffi::c_int,
            ...
        ) -> ::core::ffi::c_int;
    }
}

pub mod string_h {
    extern "C" {

        pub fn usual_basename(path: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;

        pub fn strcmpeq(
            str_left: *const ::core::ffi::c_char,
            str_right: *const ::core::ffi::c_char,
        ) -> bool;
    }
}

pub mod admin_h {
    extern "C" {

        pub fn admin_setup();
    }
}

pub mod loader_h {
    extern "C" {

        pub fn parse_database(
            base: *mut ::core::ffi::c_void,
            name: *const ::core::ffi::c_char,
            connstr: *const ::core::ffi::c_char,
        ) -> bool;

        pub fn parse_peer(
            base: *mut ::core::ffi::c_void,
            name: *const ::core::ffi::c_char,
            connstr: *const ::core::ffi::c_char,
        ) -> bool;

        pub fn parse_user(
            base: *mut ::core::ffi::c_void,
            name: *const ::core::ffi::c_char,
            params: *const ::core::ffi::c_char,
        ) -> bool;

        pub fn loader_users_check() -> bool;
    }
}

pub mod pooler_h {
    extern "C" {

        pub fn pooler_setup();

        pub fn per_loop_pooler_maint();

        pub fn pooler_tune_accept(on: bool);

        pub fn cleanup_tcp_sockets();
    }
}

pub mod stats_h {
    extern "C" {

        pub fn stats_setup();
    }
}

pub mod takeover_h {
    extern "C" {

        pub fn takeover_init();

        pub fn takeover_finish();
    }
}

pub mod janitor_h {
    extern "C" {

        pub fn janitor_setup();

        pub fn config_postprocess();

        pub fn resume_all();

        pub fn per_loop_maint();
    }
}

pub mod ldapauth_h {
    extern "C" {

        pub fn auth_ldap_init();

        pub fn ldap_poll() -> ::core::ffi::c_int;
    }
}

pub mod pam_h {
    extern "C" {

        pub fn pam_init();

        pub fn pam_poll() -> ::core::ffi::c_int;
    }
}

pub mod err_h {
    extern "C" {

        pub fn xstrdup(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    }
}
use pgbouncer::types::free;
use self::_printf_h::printf;
pub use pgbouncer::types::sigset_t;
pub use pgbouncer::types::socklen_t;
pub use self::_stdio_h::{__sFILE, __sFILEX, __sbuf, __stderrp, fpos_t, fprintf, snprintf, FILE};
use pgbouncer::types::{atexit, atol, exit, getenv, setprogname, srandom};
use pgbouncer::types::{memset, strerror, strlen};
use self::_time_h::time;
pub use pgbouncer::types::time_t;
use self::admin_h::admin_setup;
pub use self::bouncer_h::{
    any_user_level_client_timeout_set, any_user_level_timeout_set, auth_type, sockaddr_ucreds,
    C2RustUnnamed_9, CallbackState, LoadBalanceHosts, PacketCallbackFlag, PauseMode, PgAddr,
    PgCredentials, PgDatabase, PgGlobalUser, PgPool, PgSocket, PgStats, ReplicationType, SSLMode,
    ScramState, ShutDownMode, SocketState, AUTH_TYPE_ANY, AUTH_TYPE_CERT, AUTH_TYPE_HBA,
    AUTH_TYPE_LDAP, AUTH_TYPE_MD5, AUTH_TYPE_PAM, AUTH_TYPE_PEER, AUTH_TYPE_PLAIN,
    AUTH_TYPE_REJECT, AUTH_TYPE_SCRAM_SHA_256, AUTH_TYPE_TRUST, CB_HANDLE_COMPLETE_PACKET, CB_NONE,
    CB_WANT_COMPLETE_PACKET, CL_ACTIVE, CL_ACTIVE_CANCEL, CL_FREE, CL_JUSTFREE, CL_LOGIN,
    CL_WAITING, CL_WAITING_CANCEL, CL_WAITING_LOGIN, LOAD_BALANCE_HOSTS_DISABLE,
    LOAD_BALANCE_HOSTS_ROUND_ROBIN, POOL_SESSION, POOL_STMT, POOL_TX, P_NONE, P_PAUSE, P_SUSPEND,
    REPLICATION_LOGICAL, REPLICATION_NONE, REPLICATION_PHYSICAL, SHUTDOWN_IMMEDIATE, SHUTDOWN_NONE,
    SHUTDOWN_WAIT_FOR_CLIENTS, SHUTDOWN_WAIT_FOR_SERVERS, SSLMODE_ALLOW, SSLMODE_DISABLED,
    SSLMODE_PREFER, SSLMODE_REQUIRE, SSLMODE_VERIFY_CA, SSLMODE_VERIFY_FULL, SV_ACTIVE,
    SV_ACTIVE_CANCEL, SV_BEING_CANCELED, SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN, SV_TESTED,
    SV_USED,
};
pub use self::cfparser_h::{
    cf_get, cf_get_int, cf_get_lookup, cf_get_str, cf_get_time_usec, cf_get_uint, cf_load_file,
    cf_set, cf_set_int, cf_set_lookup, cf_set_str, cf_set_time_usec, cf_set_uint, CfContext, CfKey,
    CfLookup, CfOps, CfSect, CfValue, CF_NO_RELOAD, CF_READONLY, CF_VAL_ABS,
};
pub use self::config_h::{PACKAGE_BUGREPORT, PACKAGE_NAME, PACKAGE_STRING, PACKAGE_URL};
use self::dnslookup_h::{adns_create_context, adns_get_backend, adns_per_loop, DNSContext};
use self::err_h::xstrdup;
pub use self::errno_h::{__error, EINTR, ENOENT, ESRCH};
pub use self::event_h::{
    event_add, event_assign, event_base, event_base_free, event_base_get_method, event_base_loop,
    event_base_new, event_callback_fn, event_get_version, EVLOOP_ONCE, EV_PERSIST, EV_SIGNAL,
};
pub use self::event_struct_h::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
pub use self::fcntl_h::{open, O_CREAT, O_EXCL, O_RDONLY, O_RDWR, O_WRONLY};
pub use self::getopt_h::{getopt_long, no_argument, option, required_argument};
pub use self::hba_h::{hba_free, hba_load_rules, ident_free, ident_load_map, Ident, HBA};
pub use pgbouncer::types::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use pgbouncer::types::{in_addr, sockaddr_in};
pub use self::include_signal_h::{__sigbits, kill, sigprocmask};
pub use self::iobuf_h::{iobuf, IOBuf};
use self::janitor_h::{config_postprocess, janitor_setup, per_loop_maint, resume_all};
use self::ldapauth_h::{auth_ldap_init, ldap_poll};
use self::loader_h::{loader_users_check, parse_database, parse_peer, parse_user};
pub use pgbouncer::types::{
    cf_logfile, cf_quiet, cf_syslog, cf_syslog_facility, cf_syslog_ident, cf_verbose, log_fatal,
    log_generic, logging_prefix_cb, logging_prefix_fn_t, reset_logging, LogLevel, LG_DEBUG,
    LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE, LG_STATS, LG_WARNING,
};
use self::objects_h::{
    database_list, init_caches, init_objects, peer_list, reuse_just_freed_objects, user_list,
};
use self::pam_h::{pam_init, pam_poll};
pub use self::pktbuf_h::PktBuf;
use self::pooler_h::{
    cleanup_tcp_sockets, per_loop_pooler_maint, pooler_setup, pooler_tune_accept,
};
pub use self::resource_h::{getrlimit, rlim_t, rlimit, RLIMIT_NOFILE};
use pgbouncer::types::{safe_close, safe_connect, safe_write};
pub use self::sbuf_h::{
    sbuf_cb_t, sbuf_tls_setup, SBuf, SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK,
    SBUF_EV_FLUSH, SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY,
};
pub use self::signal_h::{SIG_BLOCK, __DARWIN_NSIG};
pub use pgbouncer::types::{sockaddr, socket, AF_UNIX, SOCK_STREAM};
use self::stats_h::stats_setup;
use self::string_h::{strcmpeq, usual_basename};
pub use pgbouncer::types::{
    __darwin_off_t, __darwin_pid_t, __darwin_sigset_t, __darwin_suseconds_t, __darwin_uid_t,
    __DARWIN_NULL,
};
use self::system_h::change_user;
use self::takeover_h::{takeover_finish, takeover_init};
use pgbouncer::types::tls_backend_version;
pub use self::un_h::sockaddr_un;
use self::unistd_h::{
    _exit, close, dup2, fork, getpid, getuid, optarg, optind, read, setsid, unlink,
};
use self::util_h::{cf_set_authdb, log_socket_prefix, rescue_timers};
pub use pgbouncer::types::in_addr_t;
pub use pgbouncer::types::in_port_t;
pub use pgbouncer::types::pid_t;
pub use pgbouncer::types::ptrdiff_t;
pub use pgbouncer::types::sa_family_t;
pub use pgbouncer::types::size_t;
pub use pgbouncer::types::ssize_t;
pub use pgbouncer::types::timeval;
pub use pgbouncer::types::uid_t;
pub use pgbouncer::types::uint16_t;
pub use pgbouncer::types::uint32_t;
pub use pgbouncer::types::uint64_t;
pub use pgbouncer::types::uint8_t;
pub use pgbouncer::types::uintptr_t;
pub use pgbouncer::types::usec_t;
pub use pgbouncer::types::List;
pub use pgbouncer::types::MBuf;
pub use pgbouncer::types::PktHdr;
pub use pgbouncer::types::VarCache;
pub use pgbouncer::types::NULL;
pub use pgbouncer::types::{aatree_cmp_f, aatree_walker_f, AANode, AATree};
pub use pgbouncer::types::{false_0, true_0};
pub use pgbouncer::types::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};
pub use pgbouncer::types::{statlist_count, StatList};
pub use pgbouncer::types::{PStr, StrPool};
pub use pgbouncer::types::{
    PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement,
};
pub use pgbouncer::types::{UT_hash_bucket, UT_hash_handle, UT_hash_table};
pub use pgbouncer::types::{
    __darwin_ptrdiff_t, __darwin_size_t, __darwin_socklen_t, __darwin_ssize_t, __darwin_time_t,
    __int32_t, __int64_t, __uint16_t, __uint32_t, __uint64_t, __uint8_t,
};

unsafe extern "C" fn usage(mut exe: *const ::core::ffi::c_char) {
    printf(
        c"%s is a connection pooler for PostgreSQL.\n\n".as_ptr(),
        exe,
    );
    printf(c"Usage:\n".as_ptr());
    printf(c"  %s [OPTION]... CONFIG_FILE\n".as_ptr(), exe);
    printf(c"\nOptions:\n".as_ptr());
    printf(c"  -d, --daemon         run in background (as a daemon)\n".as_ptr());
    printf(c"  -q, --quiet          run quietly\n".as_ptr());
    printf(c"  -R, --reboot         do an online reboot\n".as_ptr());
    printf(c"  -u, --user=USERNAME  assume identity of USERNAME\n".as_ptr());
    printf(c"  -v, --verbose        increase verbosity\n".as_ptr());
    printf(c"  -V, --version        show version, then exit\n".as_ptr());
    printf(c"  -h, --help           show this help, then exit\n".as_ptr());
    printf(c"\n".as_ptr());
    printf(
        c"Report bugs to <%s>.\n".as_ptr(),
        PACKAGE_BUGREPORT.as_ptr(),
    );
    printf(
        c"%s home page: <%s>\n".as_ptr(),
        PACKAGE_NAME.as_ptr(),
        PACKAGE_URL.as_ptr(),
    );
    exit(0 as ::core::ffi::c_int);
}
#[no_mangle]

pub static mut pgb_event_base: *mut event_base =
    ::core::ptr::null::<event_base>() as *mut event_base;
#[no_mangle]

pub static mut adns: *mut DNSContext = ::core::ptr::null::<DNSContext>() as *mut DNSContext;
#[no_mangle]

pub static mut parsed_hba: *mut HBA = ::core::ptr::null::<HBA>() as *mut HBA;
#[no_mangle]

pub static mut parsed_ident: *mut Ident = ::core::ptr::null::<Ident>() as *mut Ident;
#[no_mangle]

pub static mut cf_query_wait_notify: ::core::ffi::c_ulong = 0;
#[no_mangle]

pub static mut cf_daemon: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_pause_mode: ::core::ffi::c_int = P_NONE as ::core::ffi::c_int;
#[no_mangle]

pub static mut cf_shutdown: ::core::ffi::c_int = SHUTDOWN_NONE as ::core::ffi::c_int;
#[no_mangle]

pub static mut cf_reboot: ::core::ffi::c_int = 0;

static mut global_username: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_config_file: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_listen_addr: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_listen_port: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_listen_backlog: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_unix_socket_dir: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_unix_socket_mode: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_unix_socket_group: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_peer_id: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_pool_mode: ::core::ffi::c_int = POOL_SESSION;
#[no_mangle]

pub static mut cf_sbuf_len: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_sbuf_loopcnt: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_so_reuseport: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_tcp_socket_buffer: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_tcp_defer_accept: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_tcp_keepalive: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_tcp_keepcnt: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_tcp_keepidle: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_tcp_keepintvl: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_tcp_user_timeout: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_auth_type: ::core::ffi::c_int = AUTH_TYPE_MD5 as ::core::ffi::c_int;
#[no_mangle]

pub static mut cf_auth_file: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_auth_hba_file: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_auth_ident_file: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_auth_ldap_options: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_auth_user: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_auth_query: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_auth_dbname: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_track_extra_parameters: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_max_client_conn: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_default_pool_size: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_min_pool_size: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_res_pool_size: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_res_pool_timeout: usec_t = 0;
#[no_mangle]

pub static mut cf_max_db_connections: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_max_db_client_connections: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_max_user_connections: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_max_user_client_connections: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_server_reset_query: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_server_reset_query_always: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_server_check_query: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut empty_server_check_query: bool = false;
#[no_mangle]

pub static mut cf_server_check_delay: usec_t = 0;
#[no_mangle]

pub static mut cf_server_fast_close: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_server_round_robin: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_disable_pqexec: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_dns_max_ttl: usec_t = 0;
#[no_mangle]

pub static mut cf_dns_nxdomain_ttl: usec_t = 0;
#[no_mangle]

pub static mut cf_dns_zone_check_period: usec_t = 0;
#[no_mangle]

pub static mut cf_resolv_conf: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_max_packet_size: ::core::ffi::c_uint = 0;
#[no_mangle]

pub static mut cf_ignore_startup_params: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_autodb_connstr: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_autodb_idle_timeout: usec_t = 0;
#[no_mangle]

pub static mut cf_server_lifetime: usec_t = 0;
#[no_mangle]

pub static mut cf_server_idle_timeout: usec_t = 0;
#[no_mangle]

pub static mut cf_server_connect_timeout: usec_t = 0;
#[no_mangle]

pub static mut cf_server_login_retry: usec_t = 0;
#[no_mangle]

pub static mut cf_query_timeout: usec_t = 0;
#[no_mangle]

pub static mut cf_query_wait_timeout: usec_t = 0;
#[no_mangle]

pub static mut cf_cancel_wait_timeout: usec_t = 0;
#[no_mangle]

pub static mut cf_client_idle_timeout: usec_t = 0;
#[no_mangle]

pub static mut cf_client_login_timeout: usec_t = 0;
#[no_mangle]

pub static mut cf_idle_transaction_timeout: usec_t = 0;
#[no_mangle]

pub static mut cf_transaction_timeout: usec_t = 0;
#[no_mangle]

pub static mut cf_suspend_timeout: usec_t = 0;
#[no_mangle]

pub static mut g_suspend_start: usec_t = 0;
#[no_mangle]

pub static mut cf_pidfile: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_jobname: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_admin_users: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_stats_users: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_stats_period: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_log_stats: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_log_connections: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_log_disconnections: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_log_pooler_errors: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_application_name_add_host: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_client_tls_sslmode: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_client_tls_protocols: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_client_tls_ca_file: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_client_tls_cert_file: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_client_tls_key_file: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_client_tls_ciphers: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_client_tls13_ciphers: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_client_tls_dheparams: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_client_tls_ecdhecurve: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_server_tls_sslmode: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_server_tls_protocols: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_server_tls_ca_file: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_server_tls_cert_file: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_server_tls_key_file: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_server_tls_ciphers: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_server_tls13_ciphers: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]

pub static mut cf_max_prepared_statements: ::core::ffi::c_int = 0;
#[no_mangle]

pub static mut cf_scram_iterations: ::core::ffi::c_int = 0;

static mut auth_type_map: [CfLookup; 8] = [
    CfLookup {
        name: c"any".as_ptr(),
        value: AUTH_TYPE_ANY as ::core::ffi::c_int,
    },
    CfLookup {
        name: c"trust".as_ptr(),
        value: AUTH_TYPE_TRUST as ::core::ffi::c_int,
    },
    CfLookup {
        name: c"plain".as_ptr(),
        value: AUTH_TYPE_PLAIN as ::core::ffi::c_int,
    },
    CfLookup {
        name: c"md5".as_ptr(),
        value: AUTH_TYPE_MD5 as ::core::ffi::c_int,
    },
    CfLookup {
        name: c"cert".as_ptr(),
        value: AUTH_TYPE_CERT as ::core::ffi::c_int,
    },
    CfLookup {
        name: c"hba".as_ptr(),
        value: AUTH_TYPE_HBA as ::core::ffi::c_int,
    },
    CfLookup {
        name: c"scram-sha-256".as_ptr(),
        value: AUTH_TYPE_SCRAM_SHA_256 as ::core::ffi::c_int,
    },
    CfLookup {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0,
    },
];
#[no_mangle]

pub static mut pool_mode_map: [CfLookup; 4] = [
    CfLookup {
        name: c"session".as_ptr(),
        value: POOL_SESSION,
    },
    CfLookup {
        name: c"transaction".as_ptr(),
        value: POOL_TX,
    },
    CfLookup {
        name: c"statement".as_ptr(),
        value: POOL_STMT,
    },
    CfLookup {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0,
    },
];
#[no_mangle]

pub static mut sslmode_map: [CfLookup; 7] = [
    CfLookup {
        name: c"disable".as_ptr(),
        value: SSLMODE_DISABLED as ::core::ffi::c_int,
    },
    CfLookup {
        name: c"allow".as_ptr(),
        value: SSLMODE_ALLOW as ::core::ffi::c_int,
    },
    CfLookup {
        name: c"prefer".as_ptr(),
        value: SSLMODE_PREFER as ::core::ffi::c_int,
    },
    CfLookup {
        name: c"require".as_ptr(),
        value: SSLMODE_REQUIRE as ::core::ffi::c_int,
    },
    CfLookup {
        name: c"verify-ca".as_ptr(),
        value: SSLMODE_VERIFY_CA as ::core::ffi::c_int,
    },
    CfLookup {
        name: c"verify-full".as_ptr(),
        value: SSLMODE_VERIFY_FULL as ::core::ffi::c_int,
    },
    CfLookup {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0,
    },
];
#[no_mangle]

pub static mut load_balance_hosts_map: [CfLookup; 3] = [
    CfLookup {
        name: c"disable".as_ptr(),
        value: LOAD_BALANCE_HOSTS_DISABLE as ::core::ffi::c_int,
    },
    CfLookup {
        name: c"round-robin".as_ptr(),
        value: LOAD_BALANCE_HOSTS_ROUND_ROBIN as ::core::ffi::c_int,
    },
    CfLookup {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0,
    },
];

static mut bouncer_params: [CfKey; 99] = [CfKey {
    key_name: ::core::ptr::null::<::core::ffi::c_char>(),
    op: CfOps {
        setter: None,
        getter: None,
        op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
    },
    flags: 0,
    key_ofs: 0,
    def_value: ::core::ptr::null::<::core::ffi::c_char>(),
}; 99];

static mut config_sects: [CfSect; 5] = unsafe {
    [
        CfSect {
            sect_name: c"pgbouncer".as_ptr(),
            key_list: &raw const bouncer_params as *const CfKey,
            base_lookup: None,
            set_key: None,
            get_key: None,
            section_start: None,
        },
        CfSect {
            sect_name: c"databases".as_ptr(),
            key_list: ::core::ptr::null::<CfKey>(),
            base_lookup: None,
            set_key: Some(
                parse_database
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> bool,
            ),
            get_key: None,
            section_start: None,
        },
        CfSect {
            sect_name: c"users".as_ptr(),
            key_list: ::core::ptr::null::<CfKey>(),
            base_lookup: None,
            set_key: Some(
                parse_user
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> bool,
            ),
            get_key: None,
            section_start: None,
        },
        CfSect {
            sect_name: c"peers".as_ptr(),
            key_list: ::core::ptr::null::<CfKey>(),
            base_lookup: None,
            set_key: Some(
                parse_peer
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> bool,
            ),
            get_key: None,
            section_start: None,
        },
        CfSect {
            sect_name: ::core::ptr::null::<::core::ffi::c_char>(),
            key_list: ::core::ptr::null::<CfKey>(),
            base_lookup: None,
            set_key: None,
            get_key: None,
            section_start: None,
        },
    ]
};

static mut main_config: CfContext = unsafe {
    CfContext {
        sect_list: &raw const config_sects as *const CfSect,
        base: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        loaded: false,
    }
};
#[no_mangle]

pub unsafe extern "C" fn set_config_param(
    mut key: *const ::core::ffi::c_char,
    mut val: *const ::core::ffi::c_char,
) -> bool {
    cf_set(&raw mut main_config, c"pgbouncer".as_ptr(), key, val)
}
#[no_mangle]

pub unsafe extern "C" fn config_for_each(
    mut param_cb: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            bool,
        ) -> (),
    >,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut k = &raw const bouncer_params as *const CfKey;
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    let mut reloadable: bool = false;
    let mut val = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ro = CF_NO_RELOAD | CF_READONLY;
    while !(*k).key_name.is_null() {
        val = cf_get(
            &raw mut main_config,
            c"pgbouncer".as_ptr(),
            (*k).key_name,
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int,
        );
        reloadable = (*k).flags & ro == 0 as ::core::ffi::c_int;
        param_cb.expect("non-null function pointer")(
            arg,
            (*k).key_name,
            val,
            (*k).def_value,
            reloadable,
        );
        k = k.offset(1);
    }
}

unsafe extern "C" fn set_defer_accept(
    mut cv: *mut CfValue,
    mut val: *const ::core::ffi::c_char,
) -> bool {
    let mut p = (*cv).value_p as *mut ::core::ffi::c_int;
    let mut ok: bool = false;
    let mut oldval = *p;
    ok = cf_set_int(cv, val);
    if ok
        && (oldval != 0) as ::core::ffi::c_int != (*p != 0) as ::core::ffi::c_int
    {
        pooler_tune_accept(*p != 0);
    }
    ok
}

unsafe extern "C" fn set_dbs_dead(mut flag: bool) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut db = ::core::ptr::null_mut::<PgDatabase>();
    item = database_list.head.next;
    while item != &raw mut database_list.head {
        db = (item as *mut ::core::ffi::c_char)
            as *mut PgDatabase;
        if !(*db).admin && !(*db).db_auto {
            (*db).db_dead = flag;
        }
        item = (*item).next;
    }
}

unsafe extern "C" fn set_peers_dead(mut flag: bool) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut db = ::core::ptr::null_mut::<PgDatabase>();
    item = peer_list.head.next;
    while item != &raw mut peer_list.head {
        db = (item as *mut ::core::ffi::c_char)
            as *mut PgDatabase;
        (*db).db_dead = flag;
        item = (*item).next;
    }
}

unsafe extern "C" fn requires_auth_file(mut auth_type: ::core::ffi::c_int) -> bool {
    if auth_type == AUTH_TYPE_PAM as ::core::ffi::c_int {
        return false;
    }
    auth_type >= AUTH_TYPE_TRUST as ::core::ffi::c_int
}
#[no_mangle]

pub unsafe extern "C" fn load_config() -> bool {
    static mut loaded: bool = false;
    let mut load_file_ok: bool = false;
    let mut ok: bool = false;
    let mut q = ::core::ptr::null::<::core::ffi::c_char>();
    any_user_level_timeout_set = false;
    empty_server_check_query = false;
    any_user_level_client_timeout_set = false;
    set_dbs_dead(true);
    set_peers_dead(true);
    load_file_ok = cf_load_file(&raw mut main_config, cf_config_file);
    if load_file_ok {
        if requires_auth_file(cf_auth_type) {
            loader_users_check();
        }
        loaded = true;
        ok = true;
    } else if !loaded {
        let mut _log_ctx = NULL;
        log_generic(LG_FATAL, _log_ctx, c"cannot load config file".as_ptr());
        exit(1 as ::core::ffi::c_int);
    } else {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_0,
            c"config file loading failed".as_ptr(),
        );
        set_dbs_dead(false);
        ok = false;
    }
    q = cf_server_check_query;
    if strcmpeq(q, c"<empty>".as_ptr()) {
        empty_server_check_query = true;
    }
    if cf_auth_type == AUTH_TYPE_HBA as ::core::ffi::c_int {
        let mut ident = ::core::ptr::null_mut::<Ident>();
        let mut hba = ::core::ptr::null_mut::<HBA>();
        ident = ident_load_map(cf_auth_ident_file);
        if !ident.is_null() {
            ident_free(parsed_ident);
            parsed_ident = ident;
        }
        hba = hba_load_rules(cf_auth_hba_file, parsed_ident);
        if !hba.is_null() {
            hba_free(parsed_hba);
            parsed_hba = hba;
        }
    } else {
        hba_free(parsed_hba);
        parsed_hba = ::core::ptr::null_mut::<HBA>();
    }
    config_postprocess();
    if main_config.loaded {
        reset_logging();
    }
    ok
}

static mut ev_sigterm: event = event {
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

static mut ev_sigint: event = event {
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

unsafe extern "C" fn handle_sigterm(
    mut _sock: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut _arg: *mut ::core::ffi::c_void,
) {
    if cf_shutdown != 0 {
        let mut _log_ctx = NULL;
        log_generic(
            LG_INFO,
            _log_ctx,
            c"got SIGTERM while shutting down, fast exit".as_ptr(),
        );
        exit(0 as ::core::ffi::c_int);
    }
    let mut _log_ctx_0 = NULL;
    log_generic(
        LG_INFO,
        _log_ctx_0,
        c"got SIGTERM, shutting down, waiting for all clients disconnect".as_ptr(),
    );
    if cf_reboot != 0 {
        let mut _log_ctx_1 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_1,
            c"takeover was in progress, going down immediately".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    if cf_pause_mode == P_SUSPEND as ::core::ffi::c_int {
        let mut _log_ctx_2 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_2,
            c"suspend was in progress, going down immediately".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    cf_shutdown = SHUTDOWN_WAIT_FOR_CLIENTS as ::core::ffi::c_int;
    cleanup_tcp_sockets();
}

unsafe extern "C" fn handle_sigint(
    mut _sock: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut _arg: *mut ::core::ffi::c_void,
) {
    if cf_shutdown != 0 {
        let mut _log_ctx = NULL;
        log_generic(
            LG_INFO,
            _log_ctx,
            c"got SIGINT while shutting down, fast exit".as_ptr(),
        );
        exit(0 as ::core::ffi::c_int);
    }
    let mut _log_ctx_0 = NULL;
    log_generic(
        LG_INFO,
        _log_ctx_0,
        c"got SIGINT, shutting down, waiting for all servers connections to be released".as_ptr(),
    );
    if cf_reboot != 0 {
        let mut _log_ctx_1 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_1,
            c"takeover was in progress, going down immediately".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    if cf_pause_mode == P_SUSPEND as ::core::ffi::c_int {
        let mut _log_ctx_2 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_2,
            c"suspend was in progress, going down immediately".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    cf_pause_mode = P_PAUSE as ::core::ffi::c_int;
    cf_shutdown = SHUTDOWN_WAIT_FOR_SERVERS as ::core::ffi::c_int;
    cleanup_tcp_sockets();
}

static mut ev_sigquit: event = event {
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

static mut ev_sigusr1: event = event {
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

static mut ev_sigusr2: event = event {
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

static mut ev_sighup: event = event {
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

unsafe extern "C" fn handle_sigquit(
    mut _sock: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut _arg: *mut ::core::ffi::c_void,
) {
    let mut _log_ctx = NULL;
    log_generic(LG_INFO, _log_ctx, c"got SIGQUIT, fast exit".as_ptr());
    exit(0 as ::core::ffi::c_int);
}

unsafe extern "C" fn handle_sigusr1(
    mut _sock: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut _arg: *mut ::core::ffi::c_void,
) {
    if cf_pause_mode == P_NONE as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_generic(
            LG_INFO,
            _log_ctx,
            c"got SIGUSR1, pausing all activity".as_ptr(),
        );
        cf_pause_mode = P_PAUSE as ::core::ffi::c_int;
    } else {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_INFO,
            _log_ctx_0,
            c"got SIGUSR1, but already paused/suspended".as_ptr(),
        );
    };
}

unsafe extern "C" fn handle_sigusr2(
    mut _sock: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut _arg: *mut ::core::ffi::c_void,
) {
    if cf_shutdown != 0 {
        let mut _log_ctx = NULL;
        log_generic(
            LG_INFO,
            _log_ctx,
            c"got SIGUSR2 while shutting down, ignoring".as_ptr(),
        );
        return;
    }
    match cf_pause_mode {
        2 => {
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_INFO,
                _log_ctx_0,
                c"got SIGUSR2, continuing from SUSPEND".as_ptr(),
            );
            resume_all();
            cf_pause_mode = P_NONE as ::core::ffi::c_int;
        }
        1 => {
            let mut _log_ctx_1 = NULL;
            log_generic(
                LG_INFO,
                _log_ctx_1,
                c"got SIGUSR2, continuing from PAUSE".as_ptr(),
            );
            cf_pause_mode = P_NONE as ::core::ffi::c_int;
        }
        0 => {
            let mut _log_ctx_2 = NULL;
            log_generic(
                LG_INFO,
                _log_ctx_2,
                c"got SIGUSR2, but not paused/suspended".as_ptr(),
            );
        }
        _ => {}
    };
}

unsafe extern "C" fn notify_reloading() {}

unsafe extern "C" fn handle_sighup(
    mut _sock: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut _arg: *mut ::core::ffi::c_void,
) {
    let mut _log_ctx = NULL;
    log_generic(LG_INFO, _log_ctx, c"got SIGHUP, re-reading config".as_ptr());
    notify_reloading();
    load_config();
    if !sbuf_tls_setup() {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx_0,
            c"TLS configuration could not be reloaded, keeping old configuration".as_ptr(),
        );
    }
}

unsafe extern "C" fn signal_setup() {
    let mut err: ::core::ffi::c_int = 0;
    let mut set: sigset_t = 0;
    set = 0 as sigset_t;
    set |= __sigbits(13 as ::core::ffi::c_int) as sigset_t;
    err = sigprocmask(SIG_BLOCK, &raw mut set, ::core::ptr::null_mut::<sigset_t>());
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_fatal(
            c"src/main.c".as_ptr(),
            655 as ::core::ffi::c_int,
            c"signal_setup".as_ptr(),
            true,
            _log_ctx,
            c"sigprocmask".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    event_assign(
        &raw mut ev_sigusr1,
        pgb_event_base,
        30 as ::core::ffi::c_int,
        (EV_SIGNAL | EV_PERSIST) as ::core::ffi::c_short,
        Some(
            handle_sigusr1
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_short,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    err = event_add(&raw mut ev_sigusr1, ::core::ptr::null::<timeval>());
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        log_fatal(
            c"src/main.c".as_ptr(),
            662 as ::core::ffi::c_int,
            c"signal_setup".as_ptr(),
            true,
            _log_ctx_0,
            c"evsignal_add".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    event_assign(
        &raw mut ev_sigusr2,
        pgb_event_base,
        31 as ::core::ffi::c_int,
        (EV_SIGNAL | EV_PERSIST) as ::core::ffi::c_short,
        Some(
            handle_sigusr2
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_short,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    err = event_add(&raw mut ev_sigusr2, ::core::ptr::null::<timeval>());
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx_1 = NULL;
        log_fatal(
            c"src/main.c".as_ptr(),
            667 as ::core::ffi::c_int,
            c"signal_setup".as_ptr(),
            true,
            _log_ctx_1,
            c"evsignal_add".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    event_assign(
        &raw mut ev_sighup,
        pgb_event_base,
        1 as ::core::ffi::c_int,
        (EV_SIGNAL | EV_PERSIST) as ::core::ffi::c_short,
        Some(
            handle_sighup
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_short,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    err = event_add(&raw mut ev_sighup, ::core::ptr::null::<timeval>());
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx_2 = NULL;
        log_fatal(
            c"src/main.c".as_ptr(),
            672 as ::core::ffi::c_int,
            c"signal_setup".as_ptr(),
            true,
            _log_ctx_2,
            c"evsignal_add".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    event_assign(
        &raw mut ev_sigquit,
        pgb_event_base,
        3 as ::core::ffi::c_int,
        (EV_SIGNAL | EV_PERSIST) as ::core::ffi::c_short,
        Some(
            handle_sigquit
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_short,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    err = event_add(&raw mut ev_sigquit, ::core::ptr::null::<timeval>());
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx_3 = NULL;
        log_fatal(
            c"src/main.c".as_ptr(),
            677 as ::core::ffi::c_int,
            c"signal_setup".as_ptr(),
            true,
            _log_ctx_3,
            c"evsignal_add".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    event_assign(
        &raw mut ev_sigterm,
        pgb_event_base,
        15 as ::core::ffi::c_int,
        (EV_SIGNAL | EV_PERSIST) as ::core::ffi::c_short,
        Some(
            handle_sigterm
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_short,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    err = event_add(&raw mut ev_sigterm, ::core::ptr::null::<timeval>());
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx_4 = NULL;
        log_fatal(
            c"src/main.c".as_ptr(),
            682 as ::core::ffi::c_int,
            c"signal_setup".as_ptr(),
            true,
            _log_ctx_4,
            c"evsignal_add".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    event_assign(
        &raw mut ev_sigint,
        pgb_event_base,
        2 as ::core::ffi::c_int,
        (EV_SIGNAL | EV_PERSIST) as ::core::ffi::c_short,
        Some(
            handle_sigint
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_short,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    err = event_add(&raw mut ev_sigint, ::core::ptr::null::<timeval>());
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx_5 = NULL;
        log_fatal(
            c"src/main.c".as_ptr(),
            687 as ::core::ffi::c_int,
            c"signal_setup".as_ptr(),
            true,
            _log_ctx_5,
            c"evsignal_add".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
}

unsafe extern "C" fn go_daemon() {
    let mut pid: ::core::ffi::c_int = 0;
    let mut fd: ::core::ffi::c_int = 0;
    if cf_pidfile.is_null() || *cf_pidfile == 0 {
        let mut _log_ctx = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx,
            c"daemon needs pidfile configured".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    cf_quiet = 1 as ::core::ffi::c_int;
    fd = open(c"/dev/null".as_ptr(), O_RDWR);
    if fd < 0 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_0,
            c"could not open /dev/null: %s".as_ptr(),
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    dup2(fd, 0 as ::core::ffi::c_int);
    dup2(fd, 1 as ::core::ffi::c_int);
    dup2(fd, 2 as ::core::ffi::c_int);
    if fd > 2 as ::core::ffi::c_int {
        close(fd);
    }
    pid = fork() as ::core::ffi::c_int;
    if pid < 0 as ::core::ffi::c_int {
        let mut _log_ctx_1 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_1,
            c"fork failed: %s".as_ptr(),
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    if pid > 0 as ::core::ffi::c_int {
        _exit(0 as ::core::ffi::c_int);
    }
    pid = setsid() as ::core::ffi::c_int;
    if pid < 0 as ::core::ffi::c_int {
        let mut _log_ctx_2 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_2,
            c"setsid failed: %s".as_ptr(),
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    pid = fork() as ::core::ffi::c_int;
    if pid < 0 as ::core::ffi::c_int {
        let mut _log_ctx_3 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_3,
            c"fork failed: %s".as_ptr(),
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    if pid > 0 as ::core::ffi::c_int {
        _exit(0 as ::core::ffi::c_int);
    }
}

unsafe extern "C" fn remove_pidfile() {
    if !cf_pidfile.is_null() {
        if *cf_pidfile != 0 {
            unlink(cf_pidfile);
        }
        free(cf_pidfile as *mut ::core::ffi::c_void);
        cf_pidfile = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
}

unsafe extern "C" fn check_pidfile() {
    let mut buf: [::core::ffi::c_char; 129] = [0; 129];
    let mut pid: pid_t = 0 as pid_t;
    let mut fd: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    if cf_pidfile.is_null() || *cf_pidfile == 0 {
        return;
    }
    fd = open(cf_pidfile, O_RDONLY);
    if fd < 0 as ::core::ffi::c_int {
        if *__error() == ENOENT {
            return;
        }
        let mut _log_ctx = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx,
            c"could not open pidfile '%s': %s".as_ptr(),
            cf_pidfile,
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    res = read(
        fd,
        &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 129]>() as size_t).wrapping_sub(1 as size_t),
    ) as ::core::ffi::c_int;
    close(fd);
    if res < 0 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_0,
            c"could not read pidfile '%s': %s".as_ptr(),
            cf_pidfile,
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    if res != 0 as ::core::ffi::c_int {
        buf[res as usize] = 0 as ::core::ffi::c_char;
        pid = atol(&raw mut buf as *mut ::core::ffi::c_char) as pid_t;
        if (pid > 0 as pid_t)
            && (kill(pid, 0 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int)
            && (*__error() == ESRCH)
        {
            let mut _log_ctx_1 = NULL;
            log_generic(LG_INFO, _log_ctx_1, c"stale pidfile, removing".as_ptr());
            err = unlink(cf_pidfile);
            if err != 0 as ::core::ffi::c_int {
                let mut _log_ctx_2 = NULL;
                log_generic(
                    LG_FATAL,
                    _log_ctx_2,
                    c"could not remove stale pidfile: %s".as_ptr(),
                    strerror(*__error()),
                );
                exit(1 as ::core::ffi::c_int);
            }
            return;
        }
    }
    let mut _log_ctx_3 = NULL;
    log_generic(
        LG_FATAL,
        _log_ctx_3,
        c"pidfile '%s' exists, another instance running?".as_ptr(),
        cf_pidfile,
    );
    exit(1 as ::core::ffi::c_int);
}

unsafe extern "C" fn write_pidfile() {
    let mut buf: [::core::ffi::c_char; 64] = [0; 64];
    let mut pid: pid_t = 0;
    let mut res: ::core::ffi::c_int = 0;
    let mut fd: ::core::ffi::c_int = 0;
    if cf_pidfile.is_null() || *cf_pidfile == 0 {
        return;
    }
    pid = getpid();
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
        c"%u\n".as_ptr(),
        pid as ::core::ffi::c_uint,
    );
    fd = open(
        cf_pidfile,
        O_WRONLY | O_CREAT | O_EXCL,
        0o644 as ::core::ffi::c_int,
    );
    if fd < 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx,
            c"could not open pidfile '%s': %s".as_ptr(),
            cf_pidfile,
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    res = safe_write(
        fd,
        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(&raw mut buf as *mut ::core::ffi::c_char),
    ) as ::core::ffi::c_int;
    if res < 0 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_0,
            c"could not write pidfile '%s': %s".as_ptr(),
            cf_pidfile,
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    close(fd);
    atexit(Some(remove_pidfile as unsafe extern "C" fn() -> ()));
}

unsafe extern "C" fn check_limits() {
    let mut lim = rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };
    let mut total_users = statlist_count(&raw mut user_list);
    let mut fd_count: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    let mut item = ::core::ptr::null_mut::<List>();
    let mut db = ::core::ptr::null_mut::<PgDatabase>();
    let mut _log_ctx = NULL;
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(
            LG_NOISE,
            _log_ctx,
            c"event: %d, SBuf: %d, PgSocket: %d, IOBuf: %d".as_ptr(),
            ::core::mem::size_of::<event>() as ::core::ffi::c_int,
            ::core::mem::size_of::<SBuf>() as ::core::ffi::c_int,
            ::core::mem::size_of::<PgSocket>() as ::core::ffi::c_int,
            (12 as ::core::ffi::c_ulong).wrapping_add(cf_sbuf_len as ::core::ffi::c_ulong)
                as ::core::ffi::c_int,
        );
    }
    err = getrlimit(RLIMIT_NOFILE, &raw mut lim);
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx_0,
            c"could not get RLIMIT_NOFILE: %s".as_ptr(),
            strerror(*__error()),
        );
        return;
    }
    fd_count = cf_max_client_conn + 10 as ::core::ffi::c_int;
    item = database_list.head.next;
    while item != &raw mut database_list.head {
        db = (item as *mut ::core::ffi::c_char)
            as *mut PgDatabase;
        if !(*db).forced_user_credentials.is_null() {
            fd_count += if (*db).pool_size >= 0 as ::core::ffi::c_int {
                (*db).pool_size
            } else {
                cf_default_pool_size
            };
        } else {
            fd_count += (if (*db).pool_size >= 0 as ::core::ffi::c_int {
                (*db).pool_size
            } else {
                cf_default_pool_size
            }) * total_users;
        }
        item = (*item).next;
    }
    let mut _log_ctx_1 = NULL;
    log_generic(
        LG_INFO,
        _log_ctx_1,
        c"kernel file descriptor limit: %d (hard: %d); max_client_conn: %d, max expected fd use: %d".as_ptr(),
        lim.rlim_cur as ::core::ffi::c_int,
        lim.rlim_max as ::core::ffi::c_int,
        cf_max_client_conn,
        fd_count,
    );
}

unsafe extern "C" fn check_old_process_unix() -> bool {
    let mut sa_un = sockaddr_un {
        sun_len: 0,
        sun_family: 0,
        sun_path: [0; 104],
    };
    let mut len: socklen_t = ::core::mem::size_of::<sockaddr_un>() as socklen_t;
    let mut domain = AF_UNIX;
    let mut res: ::core::ffi::c_int = 0;
    let mut fd: ::core::ffi::c_int = 0;
    if cf_unix_socket_dir.is_null()
        || *cf_unix_socket_dir == 0
        || 0 as ::core::ffi::c_int > 0 as ::core::ffi::c_int
    {
        return false;
    }
    memset(
        &raw mut sa_un as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        len as size_t,
    );
    sa_un.sun_family = domain as sa_family_t;
    snprintf(
        &raw mut sa_un.sun_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 104]>() as size_t,
        c"%s/.s.PGSQL.%d".as_ptr(),
        cf_unix_socket_dir,
        cf_listen_port,
    );
    fd = socket(domain, SOCK_STREAM, 0 as ::core::ffi::c_int);
    if fd < 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx,
            c"could not create socket: %s".as_ptr(),
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    res = safe_connect(fd, &raw mut sa_un as *mut sockaddr, len);
    safe_close(fd);
    if res < 0 as ::core::ffi::c_int {
        return false;
    }
    true
}

unsafe extern "C" fn main_loop_once() {
    let mut err: ::core::ffi::c_int = 0;
    reset_time_cache();
    err = event_base_loop(pgb_event_base, EVLOOP_ONCE);
    if err < 0 as ::core::ffi::c_int && *__error() != EINTR {
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            c"event_loop failed: %s".as_ptr(),
            strerror(*__error()),
        );
    }
    ldap_poll();
    pam_poll();
    per_loop_maint();
    reuse_just_freed_objects();
    rescue_timers();
    per_loop_pooler_maint();
    if !adns.is_null() {
        adns_per_loop(adns);
    }
}

unsafe extern "C" fn takeover_part1() {
    let mut evtmp = ::core::ptr::null_mut::<event_base>();
    evtmp = pgb_event_base;
    pgb_event_base = event_base_new();
    if cf_unix_socket_dir.is_null() || *cf_unix_socket_dir == 0 {
        let mut _log_ctx = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx,
            c"cannot reboot if unix dir not configured".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    if *cf_unix_socket_dir as ::core::ffi::c_int
        == '@' as i32
    {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_0,
            c"cannot reboot with abstract Unix socket".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    if 0 as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        let mut _log_ctx_1 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_1,
            c"cannot reboot under service manager".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    takeover_init();
    while cf_reboot != 0 {
        main_loop_once();
    }
    event_base_free(pgb_event_base);
    pgb_event_base = evtmp;
}

unsafe extern "C" fn dns_setup() {
    if !adns.is_null() {
        return;
    }
    adns = adns_create_context();
    if adns.is_null() {
        let mut _log_ctx = NULL;
        log_generic(LG_FATAL, _log_ctx, c"dns setup failed".as_ptr());
        exit(1 as ::core::ffi::c_int);
    }
}

unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    let mut did_takeover = false;
    let mut arg_username = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut long_idx: ::core::ffi::c_int = 0;
    static mut long_options: [option; 8] = [
        option {
            name: c"quiet".as_ptr(),
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'q' as i32,
        },
        option {
            name: c"verbose".as_ptr(),
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'v' as i32,
        },
        option {
            name: c"help".as_ptr(),
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'h' as i32,
        },
        option {
            name: c"daemon".as_ptr(),
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'd' as i32,
        },
        option {
            name: c"version".as_ptr(),
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'V' as i32,
        },
        option {
            name: c"reboot".as_ptr(),
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'R' as i32,
        },
        option {
            name: c"user".as_ptr(),
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'u' as i32,
        },
        option {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
    ];
    setprogname(usual_basename(
        *argv,
    ));
    loop {
        c = getopt_long(
            argc,
            argv as *const *mut ::core::ffi::c_char,
            c"qvhdVRu:".as_ptr(),
            &raw const long_options as *const option,
            &raw mut long_idx,
        );
        if c == -(1 as ::core::ffi::c_int) {
            break;
        }
        match c {
            82 => {
                cf_reboot = 1 as ::core::ffi::c_int;
            }
            118 => {
                cf_verbose += 1;
            }
            86 => {
                printf(c"%s\n".as_ptr(), PACKAGE_STRING.as_ptr());
                printf(
                    c"libevent %s\nadns: %s\ntls: %s\n".as_ptr(),
                    event_get_version(),
                    adns_get_backend(),
                    tls_backend_version(),
                );
                return 0 as ::core::ffi::c_int;
            }
            100 => {
                cf_daemon = 1 as ::core::ffi::c_int;
            }
            113 => {
                cf_quiet = 1 as ::core::ffi::c_int;
            }
            117 => {
                arg_username = optarg;
            }
            104 => {
                usage(*argv);
            }
            _ => {
                fprintf(
                    __stderrp,
                    b"Try \"%s --help\" for more information.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    *argv,
                );
                exit(1 as ::core::ffi::c_int);
            }
        }
    }
    if optind + 1 as ::core::ffi::c_int != argc {
        fprintf(
            __stderrp,
            c"%s: no configuration file specified\n".as_ptr(),
            *argv,
        );
        fprintf(
            __stderrp,
            b"Try \"%s --help\" for more information.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            *argv,
        );
        exit(1 as ::core::ffi::c_int);
    }
    cf_config_file = xstrdup(*argv.offset(optind as isize));
    init_objects();
    load_config();
    main_config.loaded = true;
    init_var_lookup(cf_track_extra_parameters);
    init_caches();
    logging_prefix_cb = Some(
        log_socket_prefix
            as unsafe extern "C" fn(
                LogLevel,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_char,
                ::core::ffi::c_uint,
            ) -> ::core::ffi::c_int,
    ) as logging_prefix_fn_t;
    if !sbuf_tls_setup() {
        let mut _log_ctx = NULL;
        log_generic(LG_FATAL, _log_ctx, c"TLS setup failed".as_ptr());
        exit(1 as ::core::ffi::c_int);
    }
    if !arg_username.is_null() {
        free(global_username as *mut ::core::ffi::c_void);
        global_username = xstrdup(arg_username);
    }
    if !global_username.is_null() && *global_username as ::core::ffi::c_int != 0 {
        change_user(global_username);
    }
    if getuid() == 0 as uid_t {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_0,
            c"PgBouncer should not run as root".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    admin_setup();
    if cf_reboot != 0 {
        let mut _log_ctx_1 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_1,
            c"Online restart is deprecated, use so_reuseport instead".as_ptr(),
        );
        if check_old_process_unix() {
            takeover_part1();
            did_takeover = true;
        } else {
            let mut _log_ctx_2 = NULL;
            log_generic(
                LG_INFO,
                _log_ctx_2,
                c"old process not found, try to continue normally".as_ptr(),
            );
            cf_reboot = 0 as ::core::ffi::c_int;
            check_pidfile();
        }
    } else {
        if check_old_process_unix() {
            let mut _log_ctx_3 = NULL;
            log_generic(
                LG_FATAL,
                _log_ctx_3,
                c"unix socket is in use, cannot continue".as_ptr(),
            );
            exit(1 as ::core::ffi::c_int);
        }
        check_pidfile();
    }
    if cf_daemon != 0 {
        go_daemon();
    }
    if !getenv(c"NOTIFY_SOCKET".as_ptr()).is_null() {
        let mut _log_ctx_4 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_4,
            c"apparently running under systemd with notify socket, but systemd support was not built".as_ptr(),
        );
    }
    check_limits();
    srandom((time(::core::ptr::null_mut::<time_t>()) ^ getpid() as time_t) as ::core::ffi::c_uint);
    pgb_event_base = event_base_new();
    if pgb_event_base.is_null() {
        let mut _log_ctx_5 = NULL;
        log_generic(LG_FATAL, _log_ctx_5, c"event_base_new() failed".as_ptr());
        exit(1 as ::core::ffi::c_int);
    }
    dns_setup();
    signal_setup();
    janitor_setup();
    stats_setup();
    auth_ldap_init();
    pam_init();
    if did_takeover {
        takeover_finish();
    } else {
        pooler_setup();
    }
    write_pidfile();
    let mut _log_ctx_6 = NULL;
    log_generic(
        LG_INFO,
        _log_ctx_6,
        c"process up: %s, libevent %s (%s), adns: %s, tls: %s".as_ptr(),
        c"PgBouncer 1.25.1".as_ptr(),
        event_get_version(),
        event_base_get_method(pgb_event_base),
        adns_get_backend(),
        tls_backend_version(),
    );
    while cf_shutdown != SHUTDOWN_IMMEDIATE as ::core::ffi::c_int {
        main_loop_once();
    }
    0 as ::core::ffi::c_int
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}
unsafe extern "C" fn run_static_initializers() {
    bouncer_params = [
        CfKey {
            key_name: c"admin_users".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_admin_users as uintptr_t,
            def_value: c"".as_ptr(),
        },
        CfKey {
            key_name: c"application_name_add_host".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_application_name_add_host as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"auth_dbname".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_authdb
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_auth_dbname as uintptr_t,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        CfKey {
            key_name: c"auth_file".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_auth_file as uintptr_t,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        CfKey {
            key_name: c"auth_hba_file".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_auth_hba_file as uintptr_t,
            def_value: c"".as_ptr(),
        },
        CfKey {
            key_name: c"auth_ident_file".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_auth_ident_file as uintptr_t,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        CfKey {
            key_name: c"auth_ldap_options".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_auth_ldap_options as uintptr_t,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        CfKey {
            key_name: c"auth_query".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_auth_query as uintptr_t,
            def_value: c"SELECT rolname, CASE WHEN rolvaliduntil < now() THEN NULL ELSE rolpassword END FROM pg_authid WHERE rolname=$1 AND rolcanlogin".as_ptr(),
        },
        CfKey {
            key_name: c"auth_type".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_lookup
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_lookup
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: &raw const auth_type_map as *const CfLookup
                    as *const ::core::ffi::c_void,
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_auth_type as uintptr_t,
            def_value: c"md5".as_ptr(),
        },
        CfKey {
            key_name: c"auth_user".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_auth_user as uintptr_t,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        CfKey {
            key_name: c"autodb_idle_timeout".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_autodb_idle_timeout as uintptr_t,
            def_value: c"3600".as_ptr(),
        },
        CfKey {
            key_name: c"cancel_wait_timeout".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_cancel_wait_timeout as uintptr_t,
            def_value: c"10".as_ptr(),
        },
        CfKey {
            key_name: c"client_idle_timeout".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_idle_timeout as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"client_login_timeout".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_login_timeout as uintptr_t,
            def_value: c"60".as_ptr(),
        },
        CfKey {
            key_name: c"client_tls13_ciphers".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_tls13_ciphers as uintptr_t,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        CfKey {
            key_name: c"client_tls_ca_file".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_tls_ca_file as uintptr_t,
            def_value: c"".as_ptr(),
        },
        CfKey {
            key_name: c"client_tls_cert_file".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_tls_cert_file as uintptr_t,
            def_value: c"".as_ptr(),
        },
        CfKey {
            key_name: c"client_tls_ciphers".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_tls_ciphers as uintptr_t,
            def_value: c"default".as_ptr(),
        },
        CfKey {
            key_name: c"client_tls_dheparams".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_tls_dheparams as uintptr_t,
            def_value: c"auto".as_ptr(),
        },
        CfKey {
            key_name: c"client_tls_ecdhcurve".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_tls_ecdhecurve as uintptr_t,
            def_value: c"auto".as_ptr(),
        },
        CfKey {
            key_name: c"client_tls_key_file".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_tls_key_file as uintptr_t,
            def_value: c"".as_ptr(),
        },
        CfKey {
            key_name: c"client_tls_protocols".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_tls_protocols as uintptr_t,
            def_value: c"secure".as_ptr(),
        },
        CfKey {
            key_name: c"client_tls_sslmode".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_lookup
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_lookup
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: &raw const sslmode_map as *const CfLookup
                    as *const ::core::ffi::c_void,
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_tls_sslmode as uintptr_t,
            def_value: c"disable".as_ptr(),
        },
        CfKey {
            key_name: c"conffile".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_config_file as uintptr_t,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        CfKey {
            key_name: c"default_pool_size".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_default_pool_size as uintptr_t,
            def_value: c"20".as_ptr(),
        },
        CfKey {
            key_name: c"disable_pqexec".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_disable_pqexec as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"dns_max_ttl".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_dns_max_ttl as uintptr_t,
            def_value: c"15".as_ptr(),
        },
        CfKey {
            key_name: c"dns_nxdomain_ttl".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_dns_nxdomain_ttl as uintptr_t,
            def_value: c"15".as_ptr(),
        },
        CfKey {
            key_name: c"dns_zone_check_period".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_dns_zone_check_period as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"idle_transaction_timeout".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_idle_transaction_timeout as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"ignore_startup_parameters".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_ignore_startup_params as uintptr_t,
            def_value: c"".as_ptr(),
        },
        CfKey {
            key_name: c"job_name".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_jobname as uintptr_t,
            def_value: c"pgbouncer".as_ptr(),
        },
        CfKey {
            key_name: c"listen_addr".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_listen_addr as uintptr_t,
            def_value: c"".as_ptr(),
        },
        CfKey {
            key_name: c"listen_backlog".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_listen_backlog as uintptr_t,
            def_value: c"128".as_ptr(),
        },
        CfKey {
            key_name: c"listen_port".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_listen_port as uintptr_t,
            def_value: c"6432".as_ptr(),
        },
        CfKey {
            key_name: c"log_connections".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_log_connections as uintptr_t,
            def_value: c"1".as_ptr(),
        },
        CfKey {
            key_name: c"log_disconnections".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_log_disconnections as uintptr_t,
            def_value: c"1".as_ptr(),
        },
        CfKey {
            key_name: c"log_pooler_errors".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_log_pooler_errors as uintptr_t,
            def_value: c"1".as_ptr(),
        },
        CfKey {
            key_name: c"log_stats".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_log_stats as uintptr_t,
            def_value: c"1".as_ptr(),
        },
        CfKey {
            key_name: c"logfile".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_logfile as uintptr_t,
            def_value: c"".as_ptr(),
        },
        CfKey {
            key_name: c"max_client_conn".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_max_client_conn as uintptr_t,
            def_value: c"100".as_ptr(),
        },
        CfKey {
            key_name: c"max_db_client_connections".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_max_db_client_connections as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"max_db_connections".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_max_db_connections as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"max_packet_size".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_uint
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_uint
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_max_packet_size as uintptr_t,
            def_value: c"2147483647".as_ptr(),
        },
        CfKey {
            key_name: c"max_prepared_statements".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_max_prepared_statements as uintptr_t,
            def_value: c"200".as_ptr(),
        },
        CfKey {
            key_name: c"max_user_client_connections".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_max_user_client_connections as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"max_user_connections".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_max_user_connections as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"min_pool_size".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_min_pool_size as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"peer_id".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_peer_id as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"pidfile".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_pidfile as uintptr_t,
            def_value: c"".as_ptr(),
        },
        CfKey {
            key_name: c"pkt_buf".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_sbuf_len as uintptr_t,
            def_value: c"4096".as_ptr(),
        },
        CfKey {
            key_name: c"pool_mode".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_lookup
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_lookup
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: &raw const pool_mode_map as *const CfLookup
                    as *const ::core::ffi::c_void,
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_pool_mode as uintptr_t,
            def_value: c"session".as_ptr(),
        },
        CfKey {
            key_name: c"query_timeout".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_query_timeout as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"query_wait_notify".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_query_wait_notify as uintptr_t,
            def_value: c"5".as_ptr(),
        },
        CfKey {
            key_name: c"query_wait_timeout".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_query_wait_timeout as uintptr_t,
            def_value: c"120".as_ptr(),
        },
        CfKey {
            key_name: c"reserve_pool_size".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_res_pool_size as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"reserve_pool_timeout".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_res_pool_timeout as uintptr_t,
            def_value: c"5".as_ptr(),
        },
        CfKey {
            key_name: c"resolv_conf".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_resolv_conf as uintptr_t,
            def_value: c"".as_ptr(),
        },
        CfKey {
            key_name: c"sbuf_loopcnt".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_sbuf_loopcnt as uintptr_t,
            def_value: c"5".as_ptr(),
        },
        CfKey {
            key_name: c"scram_iterations".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_scram_iterations as uintptr_t,
            def_value: c"4096".as_ptr(),
        },
        CfKey {
            key_name: c"server_check_delay".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_check_delay as uintptr_t,
            def_value: c"30".as_ptr(),
        },
        CfKey {
            key_name: c"server_check_query".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_check_query as uintptr_t,
            def_value: c"<empty>".as_ptr(),
        },
        CfKey {
            key_name: c"server_connect_timeout".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_connect_timeout as uintptr_t,
            def_value: c"15".as_ptr(),
        },
        CfKey {
            key_name: c"server_fast_close".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_fast_close as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"server_idle_timeout".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_idle_timeout as uintptr_t,
            def_value: c"600".as_ptr(),
        },
        CfKey {
            key_name: c"server_lifetime".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_lifetime as uintptr_t,
            def_value: c"3600".as_ptr(),
        },
        CfKey {
            key_name: c"server_login_retry".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_login_retry as uintptr_t,
            def_value: c"15".as_ptr(),
        },
        CfKey {
            key_name: c"server_reset_query".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_reset_query as uintptr_t,
            def_value: c"DISCARD ALL".as_ptr(),
        },
        CfKey {
            key_name: c"server_reset_query_always".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_reset_query_always as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"server_round_robin".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_round_robin as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"server_tls13_ciphers".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_tls13_ciphers as uintptr_t,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        CfKey {
            key_name: c"server_tls_ca_file".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_tls_ca_file as uintptr_t,
            def_value: c"".as_ptr(),
        },
        CfKey {
            key_name: c"server_tls_cert_file".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_tls_cert_file as uintptr_t,
            def_value: c"".as_ptr(),
        },
        CfKey {
            key_name: c"server_tls_ciphers".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_tls_ciphers as uintptr_t,
            def_value: c"default".as_ptr(),
        },
        CfKey {
            key_name: c"server_tls_key_file".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_tls_key_file as uintptr_t,
            def_value: c"".as_ptr(),
        },
        CfKey {
            key_name: c"server_tls_protocols".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_tls_protocols as uintptr_t,
            def_value: c"secure".as_ptr(),
        },
        CfKey {
            key_name: c"server_tls_sslmode".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_lookup
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_lookup
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: &raw const sslmode_map as *const CfLookup
                    as *const ::core::ffi::c_void,
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_tls_sslmode as uintptr_t,
            def_value: c"prefer".as_ptr(),
        },
        CfKey {
            key_name: c"so_reuseport".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_so_reuseport as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"stats_period".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_stats_period as uintptr_t,
            def_value: c"60".as_ptr(),
        },
        CfKey {
            key_name: c"stats_users".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_stats_users as uintptr_t,
            def_value: c"".as_ptr(),
        },
        CfKey {
            key_name: c"suspend_timeout".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_suspend_timeout as uintptr_t,
            def_value: c"10".as_ptr(),
        },
        CfKey {
            key_name: c"syslog".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_syslog as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"syslog_facility".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_syslog_facility as uintptr_t,
            def_value: c"daemon".as_ptr(),
        },
        CfKey {
            key_name: c"syslog_ident".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_syslog_ident as uintptr_t,
            def_value: c"pgbouncer".as_ptr(),
        },
        CfKey {
            key_name: c"tcp_defer_accept".as_ptr(),
            op: CfOps {
                setter: Some(
                    set_defer_accept
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_tcp_defer_accept as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"tcp_keepalive".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_tcp_keepalive as uintptr_t,
            def_value: c"1".as_ptr(),
        },
        CfKey {
            key_name: c"tcp_keepcnt".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_tcp_keepcnt as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"tcp_keepidle".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_tcp_keepidle as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"tcp_keepintvl".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_tcp_keepintvl as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"tcp_socket_buffer".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_tcp_socket_buffer as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"tcp_user_timeout".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_tcp_user_timeout as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"track_extra_parameters".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_track_extra_parameters as uintptr_t,
            def_value: c"IntervalStyle".as_ptr(),
        },
        CfKey {
            key_name: c"transaction_timeout".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_transaction_timeout as uintptr_t,
            def_value: c"0".as_ptr(),
        },
        CfKey {
            key_name: c"unix_socket_dir".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_unix_socket_dir as uintptr_t,
            def_value: c"/tmp".as_ptr(),
        },
        CfKey {
            key_name: c"unix_socket_group".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_unix_socket_group as uintptr_t,
            def_value: c"".as_ptr(),
        },
        CfKey {
            key_name: c"unix_socket_mode".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_unix_socket_mode as uintptr_t,
            def_value: c"0777".as_ptr(),
        },
        CfKey {
            key_name: c"user".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut global_username as uintptr_t,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        CfKey {
            key_name: c"verbose".as_ptr(),
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_verbose as uintptr_t,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        CfKey {
            key_name: ::core::ptr::null::<::core::ffi::c_char>(),
            op: CfOps {
                setter: None,
                getter: None,
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0,
            key_ofs: 0,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

extern "C" {
    pub fn reset_time_cache();
}

extern "C" {
    pub fn init_var_lookup(cf_track_extra_parameters_0: *const ::core::ffi::c_char);
}
