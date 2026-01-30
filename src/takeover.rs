pub mod sys__types_h {

    pub type __darwin_blkcnt_t = __int64_t;

    pub type __darwin_blksize_t = __int32_t;

    pub type __darwin_dev_t = __int32_t;

    pub type __darwin_gid_t = __uint32_t;

    pub type __darwin_ino64_t = __uint64_t;

    pub type __darwin_mode_t = __uint16_t;

    pub type __darwin_off_t = __int64_t;

    pub type __darwin_pid_t = __int32_t;

    pub type __darwin_suseconds_t = __int32_t;

    pub type __darwin_uid_t = __uint32_t;

    pub type __darwin_useconds_t = __uint32_t;

    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    use crate::types::{__int32_t, __int64_t, __uint16_t, __uint32_t, __uint64_t};
}

pub mod _dev_t_h {

    pub type dev_t = __darwin_dev_t;
    use super::sys__types_h::__darwin_dev_t;
}

pub mod _blkcnt_t_h {

    pub type blkcnt_t = __darwin_blkcnt_t;
    use super::sys__types_h::__darwin_blkcnt_t;
}

pub mod _blksize_t_h {

    pub type blksize_t = __darwin_blksize_t;
    use super::sys__types_h::__darwin_blksize_t;
}

pub mod _gid_t_h {

    pub type gid_t = __darwin_gid_t;
    use super::sys__types_h::__darwin_gid_t;
}

pub mod _mode_t_h {

    pub type mode_t = __darwin_mode_t;
    use super::sys__types_h::__darwin_mode_t;
}

pub mod _nlink_t_h {

    pub type nlink_t = __uint16_t;
    use crate::types::__uint16_t;
}

pub mod _off_t_h {

    pub type off_t = __darwin_off_t;
    use super::sys__types_h::__darwin_off_t;
}

pub mod _useconds_t_h {

    pub type useconds_t = __darwin_useconds_t;
    use super::sys__types_h::__darwin_useconds_t;
}

pub mod _timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct timespec {
        pub tv_sec: __darwin_time_t,
        pub tv_nsec: ::core::ffi::c_long,
    }
    use crate::types::__darwin_time_t;
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
    use super::_blkcnt_t_h::blkcnt_t;
    use super::_blksize_t_h::blksize_t;
    use super::_dev_t_h::dev_t;
    use super::_gid_t_h::gid_t;
    use super::_mode_t_h::mode_t;
    use super::_nlink_t_h::nlink_t;
    use super::_off_t_h::off_t;
    use super::_timespec_h::timespec;
    use super::sys__types_h::__darwin_ino64_t;
    use crate::types::uid_t;
    use crate::types::{__int32_t, __int64_t, __uint32_t};
    extern "C" {

        pub fn stat(_: *const ::core::ffi::c_char, _: *mut stat) -> ::core::ffi::c_int;
    }
}

pub mod tls_h {
    extern "C" {

        pub type tls;
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

pub mod event_h {

    pub type event_callback_fn = Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_short,
            *mut ::core::ffi::c_void,
        ) -> (),
    >;
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
    #[inline]

    pub unsafe extern "C" fn pga_is_unix(mut a: *const PgAddr) -> bool {
        (*a).sa.sa_family as ::core::ffi::c_int == AF_UNIX
    }
    use crate::types::pid_t;

    use super::dnslookup_h::DNSToken;
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

        pub fn pga_set(a: *mut PgAddr, fam: ::core::ffi::c_int, port: ::core::ffi::c_int);

        pub fn pga_pton(
            a: *mut PgAddr,
            s: *const ::core::ffi::c_char,
            port: ::core::ffi::c_int,
        ) -> bool;

        pub static mut cf_listen_port: ::core::ffi::c_int;

        pub static mut cf_pidfile: *mut ::core::ffi::c_char;

        pub static mut cf_reboot: ::core::ffi::c_int;
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
    use super::event_h::event_callback_fn;
    use super::event_struct_h::event;
    use super::iobuf_h::IOBuf;
    use super::tls_h::tls;
    use crate::types::size_t;
    use crate::types::ssize_t;
    use crate::types::uint8_t;
    use crate::types::MBuf;
    extern "C" {

        pub fn sbuf_pause(sbuf: *mut SBuf) -> bool;

        pub fn sbuf_continue_with_callback(sbuf: *mut SBuf, cb: event_callback_fn) -> bool;
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

pub mod unistd_h {
    use super::_useconds_t_h::useconds_t;
    extern "C" {

        pub fn usleep(_: useconds_t) -> ::core::ffi::c_int;
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

        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;

        pub fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;

        pub fn strncmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
            __n: size_t,
        ) -> ::core::ffi::c_int;
    }
}

pub mod objects_h {
    use super::bouncer_h::{PgAddr, PgCredentials, PgDatabase, PgPool, PgSocket};
    use crate::types::uint64_t;

    use crate::types::StatList;
    extern "C" {

        pub static mut pool_list: StatList;

        pub fn find_database(name: *const ::core::ffi::c_char) -> *mut PgDatabase;

        pub fn get_pool(db: *mut PgDatabase, user_credentials: *mut PgCredentials) -> *mut PgPool;

        pub fn disconnect_server(
            server: *mut PgSocket,
            notify: bool,
            reason: *const ::core::ffi::c_char,
            ...
        );

        pub fn launch_new_connection(pool: *mut PgPool, evict_if_needed: bool);

        pub fn use_client_socket(
            fd: ::core::ffi::c_int,
            addr: *mut PgAddr,
            dbname: *const ::core::ffi::c_char,
            username: *const ::core::ffi::c_char,
            ckey: uint64_t,
            oldfd: ::core::ffi::c_int,
            linkfd: ::core::ffi::c_int,
            client_end: *const ::core::ffi::c_char,
            std_string: *const ::core::ffi::c_char,
            datestyle: *const ::core::ffi::c_char,
            timezone: *const ::core::ffi::c_char,
            password: *const ::core::ffi::c_char,
            scram_client_key: *const ::core::ffi::c_char,
            scram_client_key_len: ::core::ffi::c_int,
            scram_server_key: *const ::core::ffi::c_char,
            scram_server_key_len: ::core::ffi::c_int,
        ) -> bool;

        pub fn use_server_socket(
            fd: ::core::ffi::c_int,
            addr: *mut PgAddr,
            dbname: *const ::core::ffi::c_char,
            username: *const ::core::ffi::c_char,
            ckey: uint64_t,
            oldfd: ::core::ffi::c_int,
            linkfd: ::core::ffi::c_int,
            client_end: *const ::core::ffi::c_char,
            std_string: *const ::core::ffi::c_char,
            datestyle: *const ::core::ffi::c_char,
            timezone: *const ::core::ffi::c_char,
            password: *const ::core::ffi::c_char,
            scram_client_key: *const ::core::ffi::c_char,
            scram_client_key_len: ::core::ffi::c_int,
            scram_server_key: *const ::core::ffi::c_char,
            scram_server_key_len: ::core::ffi::c_int,
        ) -> bool;
    }
}

pub mod _param_h {

    pub const __DARWIN_ALIGNBYTES32: usize =
        ::core::mem::size_of::<__uint32_t>().wrapping_sub(1_usize);
    use crate::types::__uint32_t;
}

pub mod safeio_h {
    use super::socket_h::msghdr;
    use crate::types::size_t;
    use crate::types::ssize_t;
    extern "C" {

        pub fn safe_recv(
            fd: ::core::ffi::c_int,
            buf: *mut ::core::ffi::c_void,
            len: size_t,
            flags: ::core::ffi::c_int,
        ) -> ssize_t;

        pub fn safe_recvmsg(
            fd: ::core::ffi::c_int,
            msg: *mut msghdr,
            flags: ::core::ffi::c_int,
        ) -> ssize_t;
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

    pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

    pub const EAGAIN: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
    extern "C" {

        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}

pub mod usual_socket_h {
    extern "C" {

        pub fn socket_set_nonblocking(sock: ::core::ffi::c_int, non_block: bool) -> bool;
    }
}

pub mod protocol_h {

    pub const PqMsg_Query: ::core::ffi::c_int = 'Q' as i32;

    pub const PqMsg_CommandComplete: ::core::ffi::c_uint = 67 as ::core::ffi::c_uint;

    pub const PqMsg_DataRow: ::core::ffi::c_uint = 68 as ::core::ffi::c_uint;

    pub const PqMsg_ErrorResponse: ::core::ffi::c_uint = 69 as ::core::ffi::c_uint;

    pub const PqMsg_RowDescription: ::core::ffi::c_uint = 84 as ::core::ffi::c_uint;

    pub const PqMsg_ReadyForQuery: ::core::ffi::c_uint = 90 as ::core::ffi::c_uint;
}

pub mod pooler_h {
    extern "C" {

        pub fn use_pooler_socket(fd: ::core::ffi::c_int, is_unix: bool) -> bool;
    }
}

pub mod janitor_h {
    extern "C" {

        pub fn resume_all();
    }
}
pub use self::_blkcnt_t_h::blkcnt_t;
pub use self::_blksize_t_h::blksize_t;
pub use self::_dev_t_h::dev_t;
pub use self::_gid_t_h::gid_t;
pub use self::_iovec_t_h::iovec;
use self::_malloc_h::free;
pub use self::_mode_t_h::mode_t;
pub use self::_nlink_t_h::nlink_t;
pub use self::_off_t_h::off_t;
pub use self::_param_h::__DARWIN_ALIGNBYTES32;
pub use self::_socklen_t_h::socklen_t;
use self::_stdlib_h::exit;
use self::_string_h::{memcpy, memset, strcmp, strerror, strncmp};
pub use self::_timespec_h::timespec;
pub use self::_useconds_t_h::useconds_t;
pub use self::bouncer_h::{
    cf_listen_port, cf_pidfile, cf_reboot, pga_is_unix, pga_pton, pga_set, sockaddr_ucreds,
    C2RustUnnamed_9, CallbackState, LoadBalanceHosts, PacketCallbackFlag, PgAddr, PgCredentials,
    PgDatabase, PgGlobalUser, PgPool, PgSocket, PgStats, ReplicationType, ScramState, SocketState,
    CB_HANDLE_COMPLETE_PACKET, CB_NONE, CB_WANT_COMPLETE_PACKET, CL_ACTIVE, CL_ACTIVE_CANCEL,
    CL_FREE, CL_JUSTFREE, CL_LOGIN, CL_WAITING, CL_WAITING_CANCEL, CL_WAITING_LOGIN,
    LOAD_BALANCE_HOSTS_DISABLE, LOAD_BALANCE_HOSTS_ROUND_ROBIN, REPLICATION_LOGICAL,
    REPLICATION_NONE, REPLICATION_PHYSICAL, SV_ACTIVE, SV_ACTIVE_CANCEL, SV_BEING_CANCELED,
    SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN, SV_TESTED, SV_USED,
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
    __darwin_ptrdiff_t, __darwin_size_t, __darwin_socklen_t, __darwin_ssize_t, __darwin_time_t,
    __int32_t, __int64_t, __uint16_t, __uint32_t, __uint64_t, __uint8_t,
};
pub use crate::types::{aatree_cmp_f, aatree_walker_f, AANode, AATree};
pub use crate::types::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};

pub use self::errno_h::{__error, EAGAIN, ENOENT};
pub use self::event_h::{event_base, event_callback_fn};
pub use self::event_struct_h::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
pub use self::in6_h::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use self::in_h::{in_addr, sockaddr_in};
pub use self::iobuf_h::{iobuf, IOBuf};
use self::janitor_h::resume_all;
pub use self::logging_h::{
    cf_verbose, log_fatal, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE,
    LG_STATS, LG_WARNING,
};
use self::objects_h::{
    disconnect_server, find_database, get_pool, launch_new_connection, pool_list,
    use_client_socket, use_server_socket,
};
pub use self::pktbuf_h::{pktbuf_send_immediate, pktbuf_static, pktbuf_write_generic, PktBuf};
use self::pooler_h::use_pooler_socket;
pub use crate::lib::usual::mbuf::{
    mbuf_avail_for_read, mbuf_get_string, mbuf_init_fixed_reader, mbuf_written,
};
pub use crate::types::List;
pub use crate::types::MBuf;
pub use crate::types::{get_header, incomplete_pkt, log_server_error, pkt_desc, PktHdr};
pub use crate::types::{PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement};

// External function declaration (defined in proto.rs)
extern "C" {
    pub fn scan_text_result(
        pkt: *mut crate::types::MBuf,
        tupdesc: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
pub use self::protocol_h::{
    PqMsg_CommandComplete, PqMsg_DataRow, PqMsg_ErrorResponse, PqMsg_Query, PqMsg_ReadyForQuery,
    PqMsg_RowDescription,
};
use self::safeio_h::{safe_recv, safe_recvmsg};
pub use self::sbuf_h::{
    sbuf_cb_t, sbuf_continue_with_callback, sbuf_pause, SBuf, SBufEvent, SBufIO,
    SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK, SBUF_EV_FLUSH, SBUF_EV_PKT_CALLBACK, SBUF_EV_READ,
    SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED, SBUF_EV_TLS_READY,
};
pub use self::socket_h::{cmsghdr, msghdr, sockaddr, AF_UNIX, SCM_RIGHTS, SOL_SOCKET};
pub use self::stat_h::stat;
pub use self::sys__types_h::{
    __darwin_blkcnt_t, __darwin_blksize_t, __darwin_dev_t, __darwin_gid_t, __darwin_ino64_t,
    __darwin_mode_t, __darwin_off_t, __darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t,
    __darwin_useconds_t, __DARWIN_NULL,
};
pub use crate::types::StatList;
pub use crate::types::{false_0, true_0};
pub use crate::types::{usec_t, USEC};
pub use crate::types::{PStr, StrPool};

use self::unistd_h::usleep;
use self::usual_socket_h::socket_set_nonblocking;
pub use crate::types::VarCache;
pub use crate::types::{UT_hash_bucket, UT_hash_handle, UT_hash_table};

static mut old_bouncer: *mut PgSocket = ::core::ptr::null::<PgSocket>() as *mut PgSocket;
#[no_mangle]

pub unsafe extern "C" fn takeover_finish() {
    let mut buf: [uint8_t; 512] = [0; 512];
    let mut fd = (*old_bouncer).sbuf.sock;
    let mut res: bool = false;
    let mut got: ssize_t = 0;
    let mut _log_ctx = NULL;
    log_generic(LG_INFO, _log_ctx, c"sending SHUTDOWN;".as_ptr());
    socket_set_nonblocking(fd, false);
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
        c"SHUTDOWN;".as_ptr(),
    );
    res = pktbuf_send_immediate(&raw mut _buf, old_bouncer);
    if !res {
        let mut _log_ctx_0 = NULL;
        log_generic(LG_FATAL, _log_ctx_0, c"failed to send SHUTDOWN;".as_ptr());
        exit(1 as ::core::ffi::c_int);
    }
    loop {
        got = safe_recv(
            fd,
            &raw mut buf as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 512]>() as size_t,
            0 as ::core::ffi::c_int,
        );
        if got == 0 as ssize_t {
            break;
        }
        if got < 0 as ssize_t {
            let mut _log_ctx_1 = NULL;
            log_generic(
                LG_FATAL,
                _log_ctx_1,
                c"sky is falling - error while waiting result from SHUTDOWN: %s".as_ptr(),
                strerror(*__error()),
            );
            exit(1 as ::core::ffi::c_int);
        }
    }
    disconnect_server(old_bouncer, false, c"disko over".as_ptr());
    old_bouncer = ::core::ptr::null_mut::<PgSocket>();
    if !cf_pidfile.is_null()
        && *cf_pidfile as ::core::ffi::c_int != 0
    {
        let mut _log_ctx_2 = NULL;
        log_generic(
            LG_INFO,
            _log_ctx_2,
            c"waiting for old pidfile to go away".as_ptr(),
        );
        loop {
            let mut st = stat {
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
            if stat(cf_pidfile, &raw mut st) < 0 as ::core::ffi::c_int && *__error() == ENOENT {
                break;
            }
            usleep(USEC.wrapping_div(10 as usec_t) as useconds_t);
        }
    }
    let mut _log_ctx_3 = NULL;
    log_generic(
        LG_INFO,
        _log_ctx_3,
        c"old process killed, resuming work".as_ptr(),
    );
    resume_all();
}

unsafe extern "C" fn takeover_finish_part1(mut bouncer: *mut PgSocket) {
    if !sbuf_pause(&raw mut (*bouncer).sbuf) {
        let mut _log_ctx = NULL;
        log_fatal(
            c"src/takeover.c".as_ptr(),
            85 as ::core::ffi::c_int,
            c"takeover_finish_part1".as_ptr(),
            true,
            _log_ctx,
            c"sbuf_pause failed".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    old_bouncer = bouncer;
    cf_reboot = 0 as ::core::ffi::c_int;
    let mut _log_ctx_0 = NULL;
    log_generic(
        LG_INFO,
        _log_ctx_0,
        c"disko over, going background".as_ptr(),
    );
}

unsafe extern "C" fn takeover_load_fd(mut pkt: *mut MBuf, mut cmsg: *const cmsghdr) {
    let mut fd: ::core::ffi::c_int = 0;
    let mut task = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut saddr = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut user = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut db = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut client_enc = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut std_string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut datestyle = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut timezone = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut password = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut scram_client_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut scram_server_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut scram_client_key_len: ::core::ffi::c_int = 0;
    let mut scram_server_key_len: ::core::ffi::c_int = 0;
    let mut oldfd: ::core::ffi::c_int = 0;
    let mut port: ::core::ffi::c_int = 0;
    let mut linkfd: ::core::ffi::c_int = 0;
    let mut got: ::core::ffi::c_int = 0;
    let mut ckey: uint64_t = 0;
    let mut addr = PgAddr {
        sa: sockaddr {
            sa_len: 0,
            sa_family: 0,
            sa_data: [0; 14],
        },
    };
    let mut res = false;
    memset(
        &raw mut addr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PgAddr>() as size_t,
    );
    if (*cmsg).cmsg_level == SOL_SOCKET
        && (*cmsg).cmsg_type == SCM_RIGHTS
        && (*cmsg).cmsg_len as __darwin_size_t
            >= (::core::mem::size_of::<cmsghdr>().wrapping_add(__DARWIN_ALIGNBYTES32)
                & !__DARWIN_ALIGNBYTES32)
                .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as __darwin_size_t)
    {
        memcpy(
            &raw mut fd as *mut ::core::ffi::c_void,
            (cmsg as *mut ::core::ffi::c_uchar).add(
                ::core::mem::size_of::<cmsghdr>().wrapping_add(__DARWIN_ALIGNBYTES32)
                    & !__DARWIN_ALIGNBYTES32,
            ) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        );
        let mut _log_ctx = NULL;
        if cf_verbose > 0 as ::core::ffi::c_int
        {
            log_generic(LG_DEBUG, _log_ctx, c"got fd: %d".as_ptr(), fd);
        }
    } else {
        let mut _log_ctx_0 = NULL;
        log_fatal(
            c"src/takeover.c".as_ptr(),
            114 as ::core::ffi::c_int,
            c"takeover_load_fd".as_ptr(),
            false,
            _log_ctx_0,
            c"broken fd packet".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    got = scan_text_result(
        pkt,
        c"issssiqisssssbb".as_ptr(),
        &raw mut oldfd,
        &raw mut task,
        &raw mut user,
        &raw mut db,
        &raw mut saddr,
        &raw mut port,
        &raw mut ckey,
        &raw mut linkfd,
        &raw mut client_enc,
        &raw mut std_string,
        &raw mut datestyle,
        &raw mut timezone,
        &raw mut password,
        &raw mut scram_client_key_len,
        &raw mut scram_client_key,
        &raw mut scram_server_key_len,
        &raw mut scram_server_key,
    );
    if got < 0 as ::core::ffi::c_int {
        let mut _log_ctx_1 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_1,
            c"invalid data from old process".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    if task.is_null() || saddr.is_null() {
        let mut _log_ctx_2 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_2,
            c"incomplete data from old process".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    let mut _log_ctx_3 = NULL;
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            _log_ctx_3,
            c"FD row: fd=%d(%d) linkfd=%d task=%s user=%s db=%s enc=%s".as_ptr(),
            oldfd,
            fd,
            linkfd,
            task,
            if !user.is_null() {
                user as *const ::core::ffi::c_char
            } else {
                c"NULL".as_ptr()
            },
            if !db.is_null() {
                db as *const ::core::ffi::c_char
            } else {
                c"NULL".as_ptr()
            },
            if !client_enc.is_null() {
                client_enc as *const ::core::ffi::c_char
            } else {
                c"NULL".as_ptr()
            },
        );
    }
    if password.is_null() {
        password = c"".as_ptr() as *mut ::core::ffi::c_char;
    }
    if strcmp(saddr, c"unix".as_ptr()) == 0 as ::core::ffi::c_int {
        pga_set(&raw mut addr, AF_UNIX, cf_listen_port);
    } else if !pga_pton(&raw mut addr, saddr, port) {
        let mut _log_ctx_4 = NULL;
        log_fatal(
            c"src/takeover.c".as_ptr(),
            144 as ::core::ffi::c_int,
            c"takeover_load_fd".as_ptr(),
            false,
            _log_ctx_4,
            c"failed to convert address: %s".as_ptr(),
            saddr,
        );
        exit(1 as ::core::ffi::c_int);
    }
    if strcmp(task, c"client".as_ptr()) == 0 as ::core::ffi::c_int {
        res = use_client_socket(
            fd,
            &raw mut addr,
            db,
            user,
            ckey,
            oldfd,
            linkfd,
            client_enc,
            std_string,
            datestyle,
            timezone,
            password,
            scram_client_key,
            scram_client_key_len,
            scram_server_key,
            scram_server_key_len,
        );
    } else if strcmp(task, c"server".as_ptr()) == 0 as ::core::ffi::c_int {
        res = use_server_socket(
            fd,
            &raw mut addr,
            db,
            user,
            ckey,
            oldfd,
            linkfd,
            client_enc,
            std_string,
            datestyle,
            timezone,
            password,
            scram_client_key,
            scram_client_key_len,
            scram_server_key,
            scram_server_key_len,
        );
    } else if strcmp(task, c"pooler".as_ptr()) == 0 as ::core::ffi::c_int {
        res = use_pooler_socket(fd, pga_is_unix(&raw mut addr));
    } else {
        let mut _log_ctx_5 = NULL;
        log_fatal(
            c"src/takeover.c".as_ptr(),
            163 as ::core::ffi::c_int,
            c"takeover_load_fd".as_ptr(),
            false,
            _log_ctx_5,
            c"unknown task: %s".as_ptr(),
            task,
        );
        exit(1 as ::core::ffi::c_int);
    }
    free(scram_client_key as *mut ::core::ffi::c_void);
    free(scram_server_key as *mut ::core::ffi::c_void);
    if !res {
        let mut _log_ctx_6 = NULL;
        log_fatal(
            c"src/takeover.c".as_ptr(),
            170 as ::core::ffi::c_int,
            c"takeover_load_fd".as_ptr(),
            false,
            _log_ctx_6,
            c"socket takeover failed".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
}

unsafe extern "C" fn takeover_create_link(mut pool: *mut PgPool, mut client: *mut PgSocket) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut server = ::core::ptr::null_mut::<PgSocket>();
    item = (*pool).active_server_list.head.next;
    while item != &raw mut (*pool).active_server_list.head {
        server = (item as *mut ::core::ffi::c_char)
            as *mut PgSocket;
        if (*server).request_time == (*client).query_start {
            (*server).link = client;
            (*client).link = server;
            return;
        }
        item = (*item).next;
    }
    let mut _log_ctx = NULL;
    log_fatal(
        c"src/takeover.c".as_ptr(),
        186 as ::core::ffi::c_int,
        c"takeover_create_link".as_ptr(),
        false,
        _log_ctx,
        c"takeover_create_link: failed to find pair".as_ptr(),
    );
    exit(1 as ::core::ffi::c_int);
}

unsafe extern "C" fn takeover_clean_socket_list(mut list: *mut StatList) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut sk = ::core::ptr::null_mut::<PgSocket>();
    item = (*list).head.next;
    while item != &raw mut (*list).head {
        sk = (item as *mut ::core::ffi::c_char)
            as *mut PgSocket;
        if (*sk).suspended() {
            (*sk).request_time = get_cached_time();
            (*sk).query_start = get_cached_time();
        }
        item = (*item).next;
    }
}

unsafe extern "C" fn takeover_postprocess_fds() {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut item2 = ::core::ptr::null_mut::<List>();
    let mut client = ::core::ptr::null_mut::<PgSocket>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    item = pool_list.head.next;
    while item != &raw mut pool_list.head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        if !(*(*pool).db).admin {
            item2 = (*pool).active_client_list.head.next;
            while item2 != &raw mut (*pool).active_client_list.head {
                client = (item2 as *mut ::core::ffi::c_char)
                    
                    as *mut PgSocket;
                if (*client).suspended() && (*client).query_start != 0 {
                    takeover_create_link(pool, client);
                }
                item2 = (*item2).next;
            }
        }
        item = (*item).next;
    }
    item = pool_list.head.next;
    while item != &raw mut pool_list.head {
        pool = (item as *mut ::core::ffi::c_char)
            as *mut PgPool;
        takeover_clean_socket_list(&raw mut (*pool).active_client_list);
        takeover_clean_socket_list(&raw mut (*pool).active_server_list);
        takeover_clean_socket_list(&raw mut (*pool).idle_server_list);
        item = (*item).next;
    }
}

unsafe extern "C" fn next_command(mut bouncer: *mut PgSocket, mut pkt: *mut MBuf) {
    let mut res = true;
    let mut cmd = ::core::ptr::null::<::core::ffi::c_char>();
    if !mbuf_get_string(pkt, &raw mut cmd) {
        let mut _log_ctx = NULL;
        log_fatal(
            c"src/takeover.c".as_ptr(),
            234 as ::core::ffi::c_int,
            c"next_command".as_ptr(),
            false,
            _log_ctx,
            c"bad result pkt".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    let mut _log_ctx_0 = NULL;
    if cf_verbose > 0 as ::core::ffi::c_int {
        log_generic(
            LG_DEBUG,
            _log_ctx_0,
            c"takeover_recv_fds: CommandComplete body: %s".as_ptr(),
            cmd,
        );
    }
    if strcmp(cmd, c"SUSPEND".as_ptr()) == 0 as ::core::ffi::c_int {
        let mut _log_ctx_1 = NULL;
        log_generic(
            LG_INFO,
            _log_ctx_1,
            c"SUSPEND finished, sending SHOW FDS".as_ptr(),
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
            PqMsg_Query,
            c"s".as_ptr(),
            c"SHOW FDS;".as_ptr(),
        );
        res = pktbuf_send_immediate(&raw mut _buf, bouncer);
    } else if strncmp(cmd, c"SHOW".as_ptr(), 4 as size_t) == 0 as ::core::ffi::c_int {
        takeover_postprocess_fds();
        let mut _log_ctx_2 = NULL;
        log_generic(LG_INFO, _log_ctx_2, c"SHOW FDS finished".as_ptr());
        takeover_finish_part1(bouncer);
    } else {
        let mut _log_ctx_3 = NULL;
        log_fatal(
            c"src/takeover.c".as_ptr(),
            247 as ::core::ffi::c_int,
            c"next_command".as_ptr(),
            false,
            _log_ctx_3,
            c"got bad CMD from old bouncer: %s".as_ptr(),
            cmd,
        );
        exit(1 as ::core::ffi::c_int);
    }
    if !res {
        let mut _log_ctx_4 = NULL;
        log_fatal(
            c"src/takeover.c".as_ptr(),
            251 as ::core::ffi::c_int,
            c"next_command".as_ptr(),
            false,
            _log_ctx_4,
            c"command send failed".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
}

unsafe extern "C" fn takeover_parse_data(
    mut bouncer: *mut PgSocket,
    mut msg: *mut msghdr,
    mut data: *mut MBuf,
) {
    let mut cmsg = ::core::ptr::null_mut::<cmsghdr>();
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
    cmsg = if (*msg).msg_controllen != 0 {
        if (*msg).msg_controllen as usize >= ::core::mem::size_of::<cmsghdr>() {
            (*msg).msg_control as *mut cmsghdr
        } else {
            ::core::ptr::null_mut::<cmsghdr>()
        }
    } else {
        ::core::ptr::null_mut::<cmsghdr>()
    };
    while mbuf_avail_for_read(data) > 0 as ::core::ffi::c_uint {
        if !get_header(data, &raw mut pkt) {
            let mut _log_ctx = NULL;
            log_fatal(
                c"src/takeover.c".as_ptr(),
                264 as ::core::ffi::c_int,
                c"takeover_parse_data".as_ptr(),
                false,
                _log_ctx,
                c"cannot parse packet".as_ptr(),
            );
            exit(1 as ::core::ffi::c_int);
        }
        if incomplete_pkt(&raw mut pkt) {
            let mut _log_ctx_0 = NULL;
            log_fatal(
                c"src/takeover.c".as_ptr(),
                270 as ::core::ffi::c_int,
                c"takeover_parse_data".as_ptr(),
                false,
                _log_ctx_0,
                c"unexpected partial packet".as_ptr(),
            );
            exit(1 as ::core::ffi::c_int);
        }
        match pkt.type_0 {
            84 => {
                let mut _log_ctx_1 = NULL;
                if cf_verbose > 0 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_DEBUG,
                        _log_ctx_1,
                        c"takeover_parse_data: RowDescription".as_ptr(),
                    );
                }
            }
            68 => {
                let mut _log_ctx_2 = NULL;
                if cf_verbose > 0 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_DEBUG,
                        _log_ctx_2,
                        c"takeover_parse_data: DataRow".as_ptr(),
                    );
                }
                if !cmsg.is_null() {
                    takeover_load_fd(&raw mut pkt.data, cmsg);
                    cmsg = if (cmsg as *mut ::core::ffi::c_char).is_null() {
                        if (*msg).msg_controllen as usize >= ::core::mem::size_of::<cmsghdr>() {
                            (*msg).msg_control as *mut cmsghdr
                        } else {
                            ::core::ptr::null_mut::<cmsghdr>()
                        }
                    } else if (cmsg as *mut ::core::ffi::c_uchar)
                        .add(
                            ((*cmsg).cmsg_len as __darwin_size_t)
                                .wrapping_add(__DARWIN_ALIGNBYTES32)
                                & !__DARWIN_ALIGNBYTES32,
                        )
                        .add(
                            ::core::mem::size_of::<cmsghdr>().wrapping_add(__DARWIN_ALIGNBYTES32)
                                & !__DARWIN_ALIGNBYTES32,
                        )
                        > ((*msg).msg_control as *mut ::core::ffi::c_uchar)
                            .offset((*msg).msg_controllen as isize)
                    {
                        ::core::ptr::null_mut::<cmsghdr>()
                    } else {
                        (cmsg as *mut ::core::ffi::c_uchar).add(
                            ((*cmsg).cmsg_len as __darwin_size_t)
                                .wrapping_add(__DARWIN_ALIGNBYTES32)
                                & !__DARWIN_ALIGNBYTES32,
                        ) as *mut ::core::ffi::c_void as *mut cmsghdr
                    };
                } else {
                    let mut _log_ctx_3 = NULL;
                    log_fatal(
                        c"src/takeover.c".as_ptr(),
                        282 as ::core::ffi::c_int,
                        c"takeover_parse_data".as_ptr(),
                        false,
                        _log_ctx_3,
                        c"got row without fd info".as_ptr(),
                    );
                    exit(1 as ::core::ffi::c_int);
                }
            }
            90 => {
                let mut _log_ctx_4 = NULL;
                if cf_verbose > 0 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_DEBUG,
                        _log_ctx_4,
                        c"takeover_parse_data: ReadyForQuery".as_ptr(),
                    );
                }
            }
            67 => {
                let mut _log_ctx_5 = NULL;
                if cf_verbose > 0 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_DEBUG,
                        _log_ctx_5,
                        c"takeover_parse_data: CommandComplete".as_ptr(),
                    );
                }
                next_command(bouncer, &raw mut pkt.data);
            }
            69 => {
                log_server_error(c"old bouncer sent".as_ptr(), &raw mut pkt);
                let mut _log_ctx_6 = NULL;
                log_fatal(
                    c"src/takeover.c".as_ptr(),
                    294 as ::core::ffi::c_int,
                    c"takeover_parse_data".as_ptr(),
                    false,
                    _log_ctx_6,
                    c"something failed".as_ptr(),
                );
                exit(1 as ::core::ffi::c_int);
            }
            _ => {
                let mut _log_ctx_7 = NULL;
                log_fatal(
                    c"src/takeover.c".as_ptr(),
                    296 as ::core::ffi::c_int,
                    c"takeover_parse_data".as_ptr(),
                    false,
                    _log_ctx_7,
                    c"takeover_parse_data: unexpected pkt: '%c'".as_ptr(),
                    pkt_desc(&raw mut pkt) as ::core::ffi::c_int,
                );
                exit(1 as ::core::ffi::c_int);
            }
        }
    }
}

unsafe extern "C" fn takeover_recv_cb(
    mut sock: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut bouncer = (arg as *mut ::core::ffi::c_char)
        .offset(-(520 as ::core::ffi::c_ulong as isize)) as *mut PgSocket;
    let mut data_buf: [uint8_t; 2048] = [0; 2048];
    let mut cnt_buf: [uint8_t; 128] = [0; 128];
    let mut msg = msghdr {
        msg_name: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_namelen: 0,
        msg_iov: ::core::ptr::null_mut::<iovec>(),
        msg_iovlen: 0,
        msg_control: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut io = iovec {
        iov_base: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        iov_len: 0,
    };
    let mut res: ssize_t = 0;
    let mut data = MBuf {
        data: ::core::ptr::null_mut::<uint8_t>(),
        read_pos: 0,
        write_pos: 0,
        alloc_len: 0,
        reader: false,
        fixed: false,
    };
    memset(
        &raw mut msg as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<msghdr>() as size_t,
    );
    io.iov_base = &raw mut data_buf as *mut uint8_t as *mut ::core::ffi::c_void;
    io.iov_len = ::core::mem::size_of::<[uint8_t; 2048]>() as size_t;
    msg.msg_iov = &raw mut io;
    msg.msg_iovlen = 1 as ::core::ffi::c_int;
    msg.msg_control = &raw mut cnt_buf as *mut uint8_t as *mut ::core::ffi::c_void;
    msg.msg_controllen = ::core::mem::size_of::<[uint8_t; 128]>() as socklen_t;
    res = safe_recvmsg(sock, &raw mut msg, 0 as ::core::ffi::c_int);
    if res > 0 as ssize_t {
        mbuf_init_fixed_reader(
            &raw mut data,
            &raw mut data_buf as *mut uint8_t as *const ::core::ffi::c_void,
            res as ::core::ffi::c_uint,
        );
        takeover_parse_data(bouncer, &raw mut msg, &raw mut data);
    } else if res == 0 as ssize_t {
        let mut _log_ctx = NULL;
        log_fatal(
            c"src/takeover.c".as_ptr(),
            329 as ::core::ffi::c_int,
            c"takeover_recv_cb".as_ptr(),
            false,
            _log_ctx,
            c"unexpected EOF".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    } else {
        if *__error() == EAGAIN {
            return;
        }
        let mut _log_ctx_0 = NULL;
        log_fatal(
            c"src/takeover.c".as_ptr(),
            333 as ::core::ffi::c_int,
            c"takeover_recv_cb".as_ptr(),
            true,
            _log_ctx_0,
            c"safe_recvmsg".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    };
}
#[no_mangle]

pub unsafe extern "C" fn takeover_login(mut bouncer: *mut PgSocket) -> bool {
    let mut res: bool = false;
    log_generic(
        LG_INFO,
        bouncer as *mut ::core::ffi::c_void,
        c"login OK, sending SUSPEND".as_ptr(),
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
        PqMsg_Query,
        c"s".as_ptr(),
        c"SUSPEND;".as_ptr(),
    );
    res = pktbuf_send_immediate(&raw mut _buf, bouncer);
    if res {
        if !sbuf_pause(&raw mut (*bouncer).sbuf) {
            let mut _log_ctx = NULL;
            log_fatal(
                c"src/takeover.c".as_ptr(),
                350 as ::core::ffi::c_int,
                c"takeover_login".as_ptr(),
                false,
                _log_ctx,
                c"sbuf_pause failed".as_ptr(),
            );
            exit(1 as ::core::ffi::c_int);
        }
        res = sbuf_continue_with_callback(
            &raw mut (*bouncer).sbuf,
            Some(
                takeover_recv_cb
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_short,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
        );
        if !res {
            let mut _log_ctx_0 = NULL;
            log_fatal(
                c"src/takeover.c".as_ptr(),
                353 as ::core::ffi::c_int,
                c"takeover_login".as_ptr(),
                false,
                _log_ctx_0,
                c"takeover_login: sbuf_continue_with_callback failed".as_ptr(),
            );
            exit(1 as ::core::ffi::c_int);
        }
    } else {
        let mut _log_ctx_1 = NULL;
        log_fatal(
            c"src/takeover.c".as_ptr(),
            355 as ::core::ffi::c_int,
            c"takeover_login".as_ptr(),
            false,
            _log_ctx_1,
            c"takeover_login: failed to send command".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    res
}
#[no_mangle]

pub unsafe extern "C" fn takeover_init() {
    let mut db = ::core::ptr::null_mut::<PgDatabase>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    db = find_database(c"pgbouncer".as_ptr());
    if !db.is_null() {
        pool = get_pool(db, (*db).forced_user_credentials);
    }
    if pool.is_null() {
        let mut _log_ctx = NULL;
        log_fatal(
            c"src/takeover.c".as_ptr(),
            371 as ::core::ffi::c_int,
            c"takeover_init".as_ptr(),
            false,
            _log_ctx,
            c"no admin pool?".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    let mut _log_ctx_0 = NULL;
    log_generic(
        LG_INFO,
        _log_ctx_0,
        c"takeover_init: launching connection".as_ptr(),
    );
    launch_new_connection(pool, true);
}
#[no_mangle]

pub unsafe extern "C" fn takeover_login_failed() {
    let mut _log_ctx = NULL;
    log_fatal(
        c"src/takeover.c".as_ptr(),
        379 as ::core::ffi::c_int,
        c"takeover_login_failed".as_ptr(),
        false,
        _log_ctx,
        c"login failed".as_ptr(),
    );
    exit(1 as ::core::ffi::c_int);
}

extern "C" {
    pub fn get_cached_time() -> usec_t;
}
