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

    pub const AUTH_TYPE_CERT: auth_type = 4;

    pub type auth_type = ::core::ffi::c_uint;

    pub const AUTH_TYPE_REJECT: auth_type = 10;

    pub const AUTH_TYPE_PEER: auth_type = 9;

    pub const AUTH_TYPE_SCRAM_SHA_256: auth_type = 8;

    pub const AUTH_TYPE_PAM: auth_type = 7;

    pub const AUTH_TYPE_LDAP: auth_type = 6;

    pub const AUTH_TYPE_HBA: auth_type = 5;

    pub const AUTH_TYPE_MD5: auth_type = 3;

    pub const AUTH_TYPE_PLAIN: auth_type = 2;

    pub const AUTH_TYPE_TRUST: auth_type = 1;

    pub const AUTH_TYPE_ANY: auth_type = 0;
    use super::dnslookup_h::DNSToken;
    use crate::types::event_base;
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

        pub static mut pgb_event_base: *mut event_base;

        pub static mut cf_sbuf_len: ::core::ffi::c_int;

        pub static mut cf_auth_type: ::core::ffi::c_int;

        pub static mut cf_pause_mode: ::core::ffi::c_int;

        pub static mut cf_reboot: ::core::ffi::c_int;

        pub static mut cf_sbuf_loopcnt: ::core::ffi::c_int;

        pub static mut cf_tcp_defer_accept: ::core::ffi::c_int;

        pub static mut cf_client_tls_sslmode: ::core::ffi::c_int;

        pub static mut cf_client_tls_protocols: *mut ::core::ffi::c_char;

        pub static mut cf_client_tls_ca_file: *mut ::core::ffi::c_char;

        pub static mut cf_client_tls_cert_file: *mut ::core::ffi::c_char;

        pub static mut cf_client_tls_key_file: *mut ::core::ffi::c_char;

        pub static mut cf_client_tls_ciphers: *mut ::core::ffi::c_char;

        pub static mut cf_client_tls13_ciphers: *mut ::core::ffi::c_char;

        pub static mut cf_client_tls_dheparams: *mut ::core::ffi::c_char;

        pub static mut cf_client_tls_ecdhecurve: *mut ::core::ffi::c_char;

        pub static mut cf_server_tls_sslmode: ::core::ffi::c_int;

        pub static mut cf_server_tls_protocols: *mut ::core::ffi::c_char;

        pub static mut cf_server_tls_ca_file: *mut ::core::ffi::c_char;

        pub static mut cf_server_tls_cert_file: *mut ::core::ffi::c_char;

        pub static mut cf_server_tls_key_file: *mut ::core::ffi::c_char;

        pub static mut cf_server_tls_ciphers: *mut ::core::ffi::c_char;

        pub static mut cf_server_tls13_ciphers: *mut ::core::ffi::c_char;
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

    pub const SBUF_SMALL_PKT: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
    #[inline]

    pub unsafe extern "C" fn sbuf_is_empty(mut sbuf: *mut SBuf) -> bool {
        iobuf_empty((*sbuf).io)
            && (*sbuf).pkt_remain == 0 as ::core::ffi::c_uint
    }
    #[inline]

    pub unsafe extern "C" fn sbuf_op_peek(
        mut sbuf: *mut SBuf,
        mut buf: *mut ::core::ffi::c_void,
        mut len: size_t,
    ) -> ssize_t {
        (*(*sbuf).ops)
            .sbufio_peek
            .expect("non-null function pointer")(sbuf, buf, len)
    }
    #[inline]

    pub unsafe extern "C" fn sbuf_op_recv(
        mut sbuf: *mut SBuf,
        mut buf: *mut ::core::ffi::c_void,
        mut len: size_t,
    ) -> ssize_t {
        (*(*sbuf).ops)
            .sbufio_recv
            .expect("non-null function pointer")(sbuf, buf, len)
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
    #[inline]

    pub unsafe extern "C" fn sbuf_op_close(mut sbuf: *mut SBuf) -> ::core::ffi::c_int {
        (*(*sbuf).ops)
            .sbufio_close
            .expect("non-null function pointer")(sbuf)
    }
    use crate::types::event;
    use super::iobuf_h::{iobuf_empty, IOBuf};
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
    #[inline]

    pub unsafe extern "C" fn iobuf_amount_recv(mut buf: *const IOBuf) -> ::core::ffi::c_uint {
        (cf_sbuf_len as ::core::ffi::c_uint).wrapping_sub((*buf).recv_pos)
    }
    #[inline]

    pub unsafe extern "C" fn iobuf_parse_all(
        mut buf: *const IOBuf,
        mut mbuf: *mut MBuf,
    ) -> ::core::ffi::c_uint {
        let mut avail = iobuf_amount_parse(buf);
        let mut pos = (&raw const (*buf).buf as *const uint8_t).offset((*buf).parse_pos as isize);
        mbuf_init_fixed_reader(mbuf, pos as *const ::core::ffi::c_void, avail);
        avail
    }
    #[inline]

    pub unsafe extern "C" fn iobuf_parse_limit(
        mut buf: *const IOBuf,
        mut mbuf: *mut MBuf,
        mut limit: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_uint {
        let mut avail = iobuf_amount_parse(buf);
        let mut pos = (&raw const (*buf).buf as *const uint8_t).offset((*buf).parse_pos as isize);
        if avail > limit {
            avail = limit;
        }
        mbuf_init_fixed_reader(mbuf, pos as *const ::core::ffi::c_void, avail);
        avail
    }
    #[inline]

    pub unsafe extern "C" fn iobuf_tag_send(mut io: *mut IOBuf, mut len: ::core::ffi::c_uint) {
        (*io).parse_pos += len;
    }
    #[inline]

    pub unsafe extern "C" fn iobuf_tag_skip(mut io: *mut IOBuf, mut len: ::core::ffi::c_uint) {
        (*io).parse_pos += len;
        (*io).done_pos = (*io).parse_pos;
    }
    #[inline]

    pub unsafe extern "C" fn iobuf_try_resync(
        mut io: *mut IOBuf,
        mut small_pkt: ::core::ffi::c_uint,
    ) {
        let mut avail = (*io).recv_pos.wrapping_sub((*io).done_pos);
        if avail == 0 as ::core::ffi::c_uint {
            if (*io).recv_pos > 0 as ::core::ffi::c_uint {
                (*io).done_pos = 0 as ::core::ffi::c_uint;
                (*io).parse_pos = (*io).done_pos;
                (*io).recv_pos = (*io).parse_pos;
            }
        } else if avail <= small_pkt && (*io).done_pos > 0 as ::core::ffi::c_uint {
            memmove(
                &raw mut (*io).buf as *mut uint8_t as *mut ::core::ffi::c_void,
                (&raw mut (*io).buf as *mut uint8_t).offset((*io).done_pos as isize)
                    as *const ::core::ffi::c_void,
                avail as size_t,
            );
            (*io).parse_pos -= (*io).done_pos;
            (*io).recv_pos = avail;
            (*io).done_pos = 0 as ::core::ffi::c_uint;
        }
    }
    #[inline]

    pub unsafe extern "C" fn iobuf_reset(mut io: *mut IOBuf) {
        (*io).done_pos = 0 as ::core::ffi::c_uint;
        (*io).parse_pos = (*io).done_pos;
        (*io).recv_pos = (*io).parse_pos;
    }
    use crate::types::memmove;
    use super::bouncer_h::cf_sbuf_len;
    use crate::lib::usual::mbuf::mbuf_init_fixed_reader;
    use crate::types::size_t;
    use crate::types::uint8_t;
    use crate::types::MBuf;
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
    use crate::types::event;
    use crate::types::uint8_t;
    extern "C" {

        pub fn pktbuf_free(buf: *mut PktBuf);
    }
}

pub mod dnslookup_h {
    extern "C" {

        pub type DNSToken;
    }
}

pub mod objects_h {
    use super::bouncer_h::{PgPool, PgSocket};

    use crate::types::StatList;
    extern "C" {

        pub type Slab;

        pub static mut pool_list: StatList;

        pub static mut iobuf_cache: *mut Slab;

        pub fn disconnect_client(
            client: *mut PgSocket,
            notify: bool,
            reason: *const ::core::ffi::c_char,
            ...
        );

        pub fn tag_pool_dirty(pool: *mut PgPool);
    }
}

pub mod slab_h {
    use super::objects_h::Slab;
    extern "C" {

        pub fn slab_alloc(slab: *mut Slab) -> *mut ::core::ffi::c_void;

        pub fn slab_free(slab: *mut Slab, obj: *mut ::core::ffi::c_void);
    }
}

pub mod errno_h {

    pub const EIO: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

    pub const EAGAIN: ::core::ffi::c_int = 35 as ::core::ffi::c_int;

    pub const EINPROGRESS: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
    extern "C" {

        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}

pub mod util_h {
    extern "C" {

        pub fn tune_socket(sock: ::core::ffi::c_int, is_unix: bool) -> bool;
    }
}
pub use crate::types::in_addr_t;
pub use crate::types::in_port_t;

pub use crate::types::socklen_t;
use crate::types::exit;
use crate::types::{memset, strerror};
pub use crate::types::time_t;
pub use self::bouncer_h::{
    auth_type, cf_auth_type, cf_client_tls13_ciphers, cf_client_tls_ca_file,
    cf_client_tls_cert_file, cf_client_tls_ciphers, cf_client_tls_dheparams,
    cf_client_tls_ecdhecurve, cf_client_tls_key_file, cf_client_tls_protocols,
    cf_client_tls_sslmode, cf_pause_mode, cf_reboot, cf_sbuf_len, cf_sbuf_loopcnt,
    cf_server_tls13_ciphers, cf_server_tls_ca_file, cf_server_tls_cert_file, cf_server_tls_ciphers,
    cf_server_tls_key_file, cf_server_tls_protocols, cf_server_tls_sslmode, cf_tcp_defer_accept,
    pgb_event_base, sockaddr_ucreds, C2RustUnnamed_9, CallbackState, LoadBalanceHosts,
    PacketCallbackFlag, PauseMode, PgAddr, PgCredentials, PgDatabase, PgGlobalUser, PgPool,
    PgSocket, PgStats, ReplicationType, SSLMode, ScramState, SocketState, AUTH_TYPE_ANY,
    AUTH_TYPE_CERT, AUTH_TYPE_HBA, AUTH_TYPE_LDAP, AUTH_TYPE_MD5, AUTH_TYPE_PAM, AUTH_TYPE_PEER,
    AUTH_TYPE_PLAIN, AUTH_TYPE_REJECT, AUTH_TYPE_SCRAM_SHA_256, AUTH_TYPE_TRUST,
    CB_HANDLE_COMPLETE_PACKET, CB_NONE, CB_WANT_COMPLETE_PACKET, CL_ACTIVE, CL_ACTIVE_CANCEL,
    CL_FREE, CL_JUSTFREE, CL_LOGIN, CL_WAITING, CL_WAITING_CANCEL, CL_WAITING_LOGIN,
    LOAD_BALANCE_HOSTS_DISABLE, LOAD_BALANCE_HOSTS_ROUND_ROBIN, P_NONE, P_PAUSE, P_SUSPEND,
    REPLICATION_LOGICAL, REPLICATION_NONE, REPLICATION_PHYSICAL, SSLMODE_ALLOW, SSLMODE_DISABLED,
    SSLMODE_PREFER, SSLMODE_REQUIRE, SSLMODE_VERIFY_CA, SSLMODE_VERIFY_FULL, SV_ACTIVE,
    SV_ACTIVE_CANCEL, SV_BEING_CANCELED, SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN, SV_TESTED,
    SV_USED,
};
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
    __int32_t, __uint16_t, __uint32_t, __uint8_t,
};
pub use crate::types::{aatree_cmp_f, aatree_walker_f, AANode, AATree};
pub use crate::types::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};

pub use self::errno_h::{__error, EAGAIN, EINPROGRESS, EIO};
pub use crate::types::{
    event_add, event_assign, event_base, event_callback_fn, event_del, EV_PERSIST, EV_READ,
    EV_WRITE,
};
pub use crate::types::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
pub use crate::types::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use crate::types::{in_addr, sockaddr_in};
pub use self::iobuf_h::{
    iobuf, iobuf_amount_parse, iobuf_amount_pending, iobuf_amount_recv, iobuf_empty,
    iobuf_parse_all, iobuf_parse_limit, iobuf_reset, iobuf_tag_send, iobuf_tag_skip,
    iobuf_try_resync, IOBuf,
};
pub use crate::types::{
    cf_verbose, log_fatal, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE,
    LG_STATS, LG_WARNING,
};
use self::objects_h::{disconnect_client, iobuf_cache, pool_list, tag_pool_dirty};
pub use self::pktbuf_h::{pktbuf_free, PktBuf};
use crate::types::{safe_close, safe_connect, safe_recv, safe_send};
pub use self::sbuf_h::{
    sbuf_cb_t, sbuf_is_empty, sbuf_op_close, sbuf_op_peek, sbuf_op_recv, sbuf_op_send, SBuf,
    SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK, SBUF_EV_FLUSH,
    SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY, SBUF_SMALL_PKT,
};
use self::slab_h::{slab_alloc, slab_free};
pub use crate::types::{
    getsockopt, sockaddr, socket, AF_UNIX, SOCK_STREAM, SOL_SOCKET, SO_ERROR,
};
pub use crate::types::{__darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t, __DARWIN_NULL};
pub use crate::types::{
    tls, tls_accept_fds, tls_client, tls_close, tls_config, tls_config_equal, tls_config_free,
    tls_config_insecure_noverifycert, tls_config_insecure_noverifyname, tls_config_new,
    tls_config_parse_protocols, tls_config_set_ca_file, tls_config_set_cert_file,
    tls_config_set_ciphers, tls_config_set_ciphers_v13, tls_config_set_dheparams,
    tls_config_set_ecdhecurve, tls_config_set_key_file, tls_config_set_protocols,
    tls_config_verify, tls_config_verify_client, tls_config_verify_client_optional, tls_configure,
    tls_connect_fds, tls_error, tls_handshake, tls_init, tls_read, tls_server, tls_write,
    usual_tls_free, TLS_PROTOCOL_TLSv1, TLS_PROTOCOL_TLSv1_0, TLS_PROTOCOL_TLSv1_1,
    TLS_PROTOCOL_TLSv1_2, TLS_PROTOCOL_TLSv1_3, TLS_PROTOCOLS_ALL, TLS_WANT_POLLIN,
    TLS_WANT_POLLOUT,
};
use crate::types::sa2str;
use self::util_h::tune_socket;
pub use crate::lib::usual::mbuf::{
    mbuf_avail_for_read, mbuf_free, mbuf_init_fixed_reader, mbuf_make_room, mbuf_rewind_writer,
    mbuf_write,
};
pub use crate::types::usec_t;
pub use crate::types::List;
pub use crate::types::MBuf;
pub use crate::types::PktHdr;
pub use crate::types::StatList;
pub use crate::types::VarCache;
pub use crate::types::{false_0, true_0};
pub use crate::types::{PStr, StrPool};
pub use crate::types::{PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement};
pub use crate::types::{UT_hash_bucket, UT_hash_handle, UT_hash_table};

pub const SBUF_TLS_OK: TLSState = 3;

pub const W_NONE: WaitType = 0;

pub const W_ONCE: WaitType = 4;

pub const SBUF_TLS_IN_HANDSHAKE: TLSState = 2;

pub const SBUF_TLS_DO_HANDSHAKE: TLSState = 1;

pub const W_SEND: WaitType = 3;

pub const W_RECV: WaitType = 2;

pub const W_CONNECT: WaitType = 1;

pub type TLSState = ::core::ffi::c_uint;

pub const SBUF_TLS_NONE: TLSState = 0;

pub type WaitType = ::core::ffi::c_uint;

pub const DO_RECV: ::core::ffi::c_int = false_0;

pub const SKIP_RECV: ::core::ffi::c_int = true_0;

pub const ACT_SEND: ::core::ffi::c_int = 1;

pub const ACT_SKIP: ::core::ffi::c_int = 2;

pub const ACT_CALL: ::core::ffi::c_int = 3;

static mut raw_sbufio_ops: SBufIO = unsafe {
    SBufIO {
        sbufio_peek: Some(
            raw_sbufio_peek
                as unsafe extern "C" fn(*mut SBuf, *mut ::core::ffi::c_void, size_t) -> ssize_t,
        ),
        sbufio_recv: Some(
            raw_sbufio_recv
                as unsafe extern "C" fn(*mut SBuf, *mut ::core::ffi::c_void, size_t) -> ssize_t,
        ),
        sbufio_send: Some(
            raw_sbufio_send
                as unsafe extern "C" fn(*mut SBuf, *const ::core::ffi::c_void, size_t) -> ssize_t,
        ),
        sbufio_close: Some(
            raw_sbufio_close as unsafe extern "C" fn(*mut SBuf) -> ::core::ffi::c_int,
        ),
    }
};

static mut tls_sbufio_ops: SBufIO = unsafe {
    SBufIO {
        sbufio_peek: Some(
            tls_sbufio_peek
                as unsafe extern "C" fn(*mut SBuf, *mut ::core::ffi::c_void, size_t) -> ssize_t,
        ),
        sbufio_recv: Some(
            tls_sbufio_recv
                as unsafe extern "C" fn(*mut SBuf, *mut ::core::ffi::c_void, size_t) -> ssize_t,
        ),
        sbufio_send: Some(
            tls_sbufio_send
                as unsafe extern "C" fn(*mut SBuf, *const ::core::ffi::c_void, size_t) -> ssize_t,
        ),
        sbufio_close: Some(
            tls_sbufio_close as unsafe extern "C" fn(*mut SBuf) -> ::core::ffi::c_int,
        ),
    }
};
#[no_mangle]

pub unsafe extern "C" fn sbuf_init(mut sbuf: *mut SBuf, mut proto_fn: sbuf_cb_t) {
    memset(
        sbuf as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<SBuf>() as size_t,
    );
    (*sbuf).proto_cb = proto_fn;
    (*sbuf).ops = &raw const raw_sbufio_ops;
}
#[no_mangle]

pub unsafe extern "C" fn sbuf_accept(
    mut sbuf: *mut SBuf,
    mut sock: ::core::ffi::c_int,
    mut is_unix: bool,
) -> bool {
    let mut current_block: u64;
    let mut res: bool = false;
    (*sbuf).sock = sock;
    if tune_socket(sock, is_unix) {
        if cf_reboot == 0 {
            res = sbuf_wait_for_data(sbuf);
            if !res {
                current_block = 2269420068866141602;
            } else if !handle_possible_direct_tls_startup(sbuf, is_unix) {
                current_block = 2269420068866141602;
            } else {
                if (*sbuf).wait_type as ::core::ffi::c_int == W_RECV as ::core::ffi::c_int
                    && cf_tcp_defer_accept != 0
                    && !is_unix
                {
                    sbuf_main_loop(sbuf, DO_RECV != 0);
                    if (*sbuf).sock == 0 {
                        return false;
                    }
                }
                current_block = 2979737022853876585;
            }
        } else {
            current_block = 2979737022853876585;
        }
        match current_block {
            2269420068866141602 => {}
            _ => return true,
        }
    }
    sbuf_call_proto(sbuf, SBUF_EV_RECV_FAILED as ::core::ffi::c_int);
    false
}
#[no_mangle]

pub unsafe extern "C" fn sbuf_connect(
    mut sbuf: *mut SBuf,
    mut sa: *const sockaddr,
    mut sa_len: socklen_t,
    mut timeout_sec: time_t,
) -> bool {
    let mut res: ::core::ffi::c_int = 0;
    let mut sock: ::core::ffi::c_int = 0;
    let mut timeout = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut buf: [::core::ffi::c_char; 128] = [0; 128];
    let mut is_unix = (*sa).sa_family as ::core::ffi::c_int == AF_UNIX;
    sock = socket(
        (*sa).sa_family as ::core::ffi::c_int,
        SOCK_STREAM,
        0 as ::core::ffi::c_int,
    );
    if (sock >= 0 as ::core::ffi::c_int) && tune_socket(sock, is_unix) {
        (*sbuf).sock = sock;
        timeout.tv_sec = timeout_sec as __darwin_time_t;
        timeout.tv_usec = 0 as ::core::ffi::c_int as __darwin_suseconds_t;
        res = safe_connect(sock, sa, sa_len);
        if res == 0 as ::core::ffi::c_int {
            sbuf_connect_cb(
                sock,
                EV_WRITE as ::core::ffi::c_short,
                sbuf as *mut ::core::ffi::c_void,
            );
            return true;
        } else if *__error() == EINPROGRESS || *__error() == EAGAIN {
            event_assign(
                &raw mut (*sbuf).ev,
                pgb_event_base,
                sock,
                EV_WRITE as ::core::ffi::c_short,
                Some(
                    sbuf_connect_cb
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            ::core::ffi::c_short,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                ),
                sbuf as *mut ::core::ffi::c_void,
            );
            res = event_add(&raw mut (*sbuf).ev, &raw mut timeout);
            if res >= 0 as ::core::ffi::c_int {
                (*sbuf).wait_type = W_CONNECT as ::core::ffi::c_int as uint8_t;
                return true;
            }
        }
    }
    let mut _log_ctx = NULL;
    log_generic(
        LG_WARNING,
        _log_ctx,
        c"sbuf_connect failed to connect to %s: %s".as_ptr(),
        sa2str(
            sa,
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
        ),
        strerror(*__error()),
    );
    if sock >= 0 as ::core::ffi::c_int {
        safe_close(sock);
    }
    (*sbuf).sock = 0 as ::core::ffi::c_int;
    sbuf_call_proto(sbuf, SBUF_EV_CONNECT_FAILED as ::core::ffi::c_int);
    false
}
#[no_mangle]

pub unsafe extern "C" fn sbuf_pause(mut sbuf: *mut SBuf) -> bool {
    if event_del(&raw mut (*sbuf).ev) < 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            c"event_del: %s".as_ptr(),
            strerror(*__error()),
        );
        return false;
    }
    (*sbuf).wait_type = W_NONE as ::core::ffi::c_int as uint8_t;
    true
}
#[no_mangle]

pub unsafe extern "C" fn sbuf_continue(mut sbuf: *mut SBuf) {
    let mut do_recv = DO_RECV != 0;
    let mut res: bool = false;
    res = sbuf_wait_for_data(sbuf);
    if !res {
        sbuf_call_proto(sbuf, SBUF_EV_RECV_FAILED as ::core::ffi::c_int);
        return;
    }
    sbuf_main_loop(sbuf, do_recv);
}
#[no_mangle]

pub unsafe extern "C" fn sbuf_continue_with_callback(
    mut sbuf: *mut SBuf,
    mut user_cb: event_callback_fn,
) -> bool {
    let mut err: ::core::ffi::c_int = 0;
    event_assign(
        &raw mut (*sbuf).ev,
        pgb_event_base,
        (*sbuf).sock,
        (EV_READ | EV_PERSIST) as ::core::ffi::c_short,
        user_cb,
        sbuf as *mut ::core::ffi::c_void,
    );
    err = event_add(&raw mut (*sbuf).ev, ::core::ptr::null::<timeval>());
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            c"sbuf_continue_with_callback: %s".as_ptr(),
            strerror(*__error()),
        );
        return false;
    }
    (*sbuf).wait_type = W_RECV as ::core::ffi::c_int as uint8_t;
    true
}
#[no_mangle]

pub unsafe extern "C" fn sbuf_use_callback_once(
    mut sbuf: *mut SBuf,
    mut ev: ::core::ffi::c_short,
    mut user_cb: event_callback_fn,
) -> bool {
    let mut err: ::core::ffi::c_int = 0;
    if (*sbuf).wait_type as ::core::ffi::c_int != W_NONE as ::core::ffi::c_int {
        err = event_del(&raw mut (*sbuf).ev);
        (*sbuf).wait_type = W_NONE as ::core::ffi::c_int as uint8_t;
        if err < 0 as ::core::ffi::c_int {
            let mut _log_ctx = NULL;
            log_generic(
                LG_WARNING,
                _log_ctx,
                c"sbuf_queue_once: event_del failed: %s".as_ptr(),
                strerror(*__error()),
            );
            return false;
        }
    }
    event_assign(
        &raw mut (*sbuf).ev,
        pgb_event_base,
        (*sbuf).sock,
        ev,
        user_cb,
        sbuf as *mut ::core::ffi::c_void,
    );
    err = event_add(&raw mut (*sbuf).ev, ::core::ptr::null::<timeval>());
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_0,
            c"sbuf_queue_once: event_add failed: %s".as_ptr(),
            strerror(*__error()),
        );
        return false;
    }
    (*sbuf).wait_type = W_ONCE as ::core::ffi::c_int as uint8_t;
    true
}
#[no_mangle]

pub unsafe extern "C" fn sbuf_close(mut sbuf: *mut SBuf) -> bool {
    if (*sbuf).wait_type != 0 {
        *__error() = 0 as ::core::ffi::c_int;
        if event_del(&raw mut (*sbuf).ev) < 0 as ::core::ffi::c_int {
            if *__error() != 0 {
                let mut _log_ctx = NULL;
                log_generic(
                    LG_WARNING,
                    _log_ctx,
                    c"event_del: %s".as_ptr(),
                    strerror(*__error()),
                );
            } else {
                let mut _log_ctx_0 = NULL;
                log_generic(
                    LG_WARNING,
                    _log_ctx_0,
                    c"event_del: libevent error".as_ptr(),
                );
            }
        }
    }
    sbuf_op_close(sbuf);
    (*sbuf).dst = ::core::ptr::null_mut::<SBuf>();
    (*sbuf).sock = 0 as ::core::ffi::c_int;
    (*sbuf).pkt_remain = 0 as ::core::ffi::c_uint;
    (*sbuf).wait_type = 0;
    (*sbuf).pkt_action = (*sbuf).wait_type;
    if !(*sbuf).io.is_null() {
        slab_free(iobuf_cache, (*sbuf).io as *mut ::core::ffi::c_void);
        (*sbuf).io = ::core::ptr::null_mut::<IOBuf>();
    }
    mbuf_free(&raw mut (*sbuf).extra_packets);
    true
}
#[no_mangle]

pub unsafe extern "C" fn sbuf_prepare_send(
    mut sbuf: *mut SBuf,
    mut dst: *mut SBuf,
    mut amount: ::core::ffi::c_uint,
) {
    (*sbuf).pkt_action = ACT_SEND as uint8_t;
    (*sbuf).pkt_remain = amount;
    (*sbuf).dst = dst;
}
#[no_mangle]

pub unsafe extern "C" fn sbuf_prepare_skip(mut sbuf: *mut SBuf, mut amount: ::core::ffi::c_uint) {
    (*sbuf).pkt_action = ACT_SKIP as uint8_t;
    (*sbuf).skip_remain = amount;
    (*sbuf).pkt_remain = amount;
}
#[no_mangle]

pub unsafe extern "C" fn sbuf_prepare_skip_then_send_leftover(
    mut sbuf: *mut SBuf,
    mut dst: *mut SBuf,
    mut skip_amount: ::core::ffi::c_uint,
    mut total_amount: ::core::ffi::c_uint,
) {
    (*sbuf).pkt_action = ACT_SKIP as uint8_t;
    (*sbuf).pkt_remain = total_amount;
    (*sbuf).skip_remain = skip_amount;
    (*sbuf).dst = dst;
}
#[no_mangle]

pub unsafe extern "C" fn sbuf_prepare_fetch(mut sbuf: *mut SBuf, mut amount: ::core::ffi::c_uint) {
    (*sbuf).pkt_action = ACT_CALL as uint8_t;
    (*sbuf).skip_remain = amount;
    (*sbuf).pkt_remain = amount;
}
#[no_mangle]

pub unsafe extern "C" fn sbuf_queue_packet(
    mut src: *mut SBuf,
    mut dst: *mut SBuf,
    mut pkt: *mut PktBuf,
) -> bool {
    let mut res: bool = false;
    if pkt.is_null() || (*pkt).failed() {
        pktbuf_free(pkt);
        return false;
    }
    (*src).dst = dst;
    res = mbuf_write(
        &raw mut (*src).extra_packets,
        (*pkt).buf as *const ::core::ffi::c_void,
        (*pkt).write_pos as ::core::ffi::c_uint,
    );
    pktbuf_free(pkt);
    res
}
#[no_mangle]

pub unsafe extern "C" fn sbuf_queue_full_packet(
    mut src: *mut SBuf,
    mut dst: *mut SBuf,
    mut pkt: *mut PktHdr,
) -> bool {
    let mut res: bool = false;
    (*src).dst = dst;
    res = mbuf_write(
        &raw mut (*src).extra_packets,
        (*pkt).data.data as *const ::core::ffi::c_void,
        (*pkt).data.write_pos,
    );
    res
}

unsafe extern "C" fn sbuf_call_proto(mut sbuf: *mut SBuf, mut event: ::core::ffi::c_int) -> bool {
    let mut mbuf = MBuf {
        data: ::core::ptr::null_mut::<uint8_t>(),
        read_pos: 0,
        write_pos: 0,
        alloc_len: 0,
        reader: false,
        fixed: false,
    };
    let mut io = (*sbuf).io;
    let mut res: bool = false;
    if event == SBUF_EV_PKT_CALLBACK as ::core::ffi::c_int {
        iobuf_parse_limit(io, &raw mut mbuf, (*sbuf).pkt_remain);
    } else if event == SBUF_EV_READ as ::core::ffi::c_int {
        iobuf_parse_all(io, &raw mut mbuf);
    } else {
        memset(
            &raw mut mbuf as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<MBuf>() as size_t,
        );
    }
    res = (*sbuf).proto_cb.expect("non-null function pointer")(
        sbuf,
        event as SBufEvent,
        &raw mut mbuf,
    );
    res
}

unsafe extern "C" fn sbuf_wait_for_data(mut sbuf: *mut SBuf) -> bool {
    let mut err: ::core::ffi::c_int = 0;
    event_assign(
        &raw mut (*sbuf).ev,
        pgb_event_base,
        (*sbuf).sock,
        (EV_READ | EV_PERSIST) as ::core::ffi::c_short,
        Some(
            sbuf_recv_cb
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_short,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sbuf as *mut ::core::ffi::c_void,
    );
    err = event_add(&raw mut (*sbuf).ev, ::core::ptr::null::<timeval>());
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            c"sbuf_wait_for_data: event_add failed: %s".as_ptr(),
            strerror(*__error()),
        );
        return false;
    }
    (*sbuf).wait_type = W_RECV as ::core::ffi::c_int as uint8_t;
    true
}

unsafe extern "C" fn sbuf_recv_forced_cb(
    mut sock: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_short,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut sbuf = arg as *mut SBuf;
    (*sbuf).wait_type = W_NONE as ::core::ffi::c_int as uint8_t;
    if sbuf_wait_for_data(sbuf) {
        sbuf_recv_cb(sock, flags, arg);
    } else {
        sbuf_call_proto(sbuf, SBUF_EV_RECV_FAILED as ::core::ffi::c_int);
    };
}

unsafe extern "C" fn sbuf_wait_for_data_forced(mut sbuf: *mut SBuf) -> bool {
    let mut err: ::core::ffi::c_int = 0;
    let mut tv_min = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    tv_min.tv_sec = 0 as __darwin_time_t;
    tv_min.tv_usec = 1 as ::core::ffi::c_int as __darwin_suseconds_t;
    if (*sbuf).wait_type as ::core::ffi::c_int != W_NONE as ::core::ffi::c_int {
        event_del(&raw mut (*sbuf).ev);
        (*sbuf).wait_type = W_NONE as ::core::ffi::c_int as uint8_t;
    }
    event_assign(
        &raw mut (*sbuf).ev,
        pgb_event_base,
        (*sbuf).sock,
        EV_READ as ::core::ffi::c_short,
        Some(
            sbuf_recv_forced_cb
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_short,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sbuf as *mut ::core::ffi::c_void,
    );
    err = event_add(&raw mut (*sbuf).ev, &raw mut tv_min);
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            c"sbuf_wait_for_data: event_add failed: %s".as_ptr(),
            strerror(*__error()),
        );
        return false;
    }
    (*sbuf).wait_type = W_ONCE as ::core::ffi::c_int as uint8_t;
    true
}

unsafe extern "C" fn sbuf_send_cb(
    mut _sock: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut sbuf = arg as *mut SBuf;
    let mut res: bool = false;
    let mut _log_ctx = NULL;
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(LG_NOISE, _log_ctx, c"Socket is writable again".as_ptr());
    }
    if (*sbuf).sock == 0 {
        return;
    }
    (*sbuf).wait_type = W_NONE as ::core::ffi::c_int as uint8_t;
    res = sbuf_wait_for_data(sbuf);
    if res {
        sbuf_main_loop(sbuf, SKIP_RECV != 0);
    } else {
        sbuf_call_proto(sbuf, SBUF_EV_SEND_FAILED as ::core::ffi::c_int);
    };
}

unsafe extern "C" fn sbuf_queue_send(mut sbuf: *mut SBuf) -> bool {
    let mut err: ::core::ffi::c_int = 0;
    err = event_del(&raw mut (*sbuf).ev);
    (*sbuf).wait_type = W_NONE as ::core::ffi::c_int as uint8_t;
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            c"sbuf_queue_send: event_del failed: %s".as_ptr(),
            strerror(*__error()),
        );
        return false;
    }
    event_assign(
        &raw mut (*sbuf).ev,
        pgb_event_base,
        (*(*sbuf).dst).sock,
        EV_WRITE as ::core::ffi::c_short,
        Some(
            sbuf_send_cb
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_short,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sbuf as *mut ::core::ffi::c_void,
    );
    err = event_add(&raw mut (*sbuf).ev, ::core::ptr::null::<timeval>());
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_0,
            c"sbuf_queue_send: event_add failed: %s".as_ptr(),
            strerror(*__error()),
        );
        return false;
    }
    (*sbuf).wait_type = W_SEND as ::core::ffi::c_int as uint8_t;
    true
}
#[no_mangle]

pub unsafe extern "C" fn sbuf_flush(mut sbuf: *mut SBuf) -> bool {
    if !(*sbuf).io.is_null() {
        let mut _log_ctx = NULL;
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(LG_NOISE, _log_ctx, c"sbuf_flush".as_ptr());
        }
        return sbuf_send_pending_iobuf(sbuf);
    }
    true
}

unsafe extern "C" fn sbuf_send_pending_iobuf(mut sbuf: *mut SBuf) -> bool {
    let mut avail: ::core::ffi::c_int = 0;
    let mut res: ssize_t = 0;
    let mut io = (*sbuf).io;
    let mut _log_ctx = NULL;
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(LG_NOISE, _log_ctx, c"sbuf_send_pending_iobuf".as_ptr());
    }
    loop {
        avail = iobuf_amount_pending(io) as ::core::ffi::c_int;
        if avail == 0 as ::core::ffi::c_int {
            return true;
        }
        if (*(*sbuf).dst).sock == 0 as ::core::ffi::c_int {
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_0,
                c"sbuf_send_pending_iobuf: no dst sock?".as_ptr(),
            );
            sbuf_call_proto(sbuf, SBUF_EV_SEND_FAILED as ::core::ffi::c_int);
            return false;
        }
        res = sbuf_op_send(
            (*sbuf).dst,
            (&raw mut (*io).buf as *mut uint8_t).offset((*io).done_pos as isize)
                as *const ::core::ffi::c_void,
            avail as size_t,
        );
        if res > 0 as ssize_t {
            (*io).done_pos = ((*io).done_pos as ssize_t + res) as ::core::ffi::c_uint;
        } else if res < 0 as ssize_t {
            if *__error() == EAGAIN {
                if !sbuf_queue_send(sbuf) {
                    sbuf_call_proto(sbuf, SBUF_EV_SEND_FAILED as ::core::ffi::c_int);
                }
            } else {
                sbuf_call_proto(sbuf, SBUF_EV_SEND_FAILED as ::core::ffi::c_int);
            }
            return false;
        }
    }
}

unsafe extern "C" fn sbuf_send_pending_extra_packets(mut sbuf: *mut SBuf) -> bool {
    let mut avail: ::core::ffi::c_int = 0;
    let mut res: ssize_t = 0;
    let mut mbuf: *mut MBuf = &raw mut (*sbuf).extra_packets;
    let mut _log_ctx = NULL;
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(
            LG_NOISE,
            _log_ctx,
            c"sbuf_send_pending_extra_packets ".as_ptr(),
        );
    }
    loop {
        avail = mbuf_avail_for_read(mbuf) as ::core::ffi::c_int;
        if avail == 0 as ::core::ffi::c_int {
            return true;
        }
        if (*(*sbuf).dst).sock == 0 as ::core::ffi::c_int {
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_0,
                c"sbuf_send_pending_extra_packets: no dst sock?".as_ptr(),
            );
            sbuf_call_proto(sbuf, SBUF_EV_SEND_FAILED as ::core::ffi::c_int);
            return false;
        }
        res = sbuf_op_send(
            (*sbuf).dst,
            (*mbuf).data.offset((*mbuf).read_pos as isize) as *const ::core::ffi::c_void,
            avail as size_t,
        );
        if res > 0 as ssize_t {
            (*mbuf).read_pos = ((*mbuf).read_pos as ssize_t + res) as ::core::ffi::c_uint;
        } else if res < 0 as ssize_t {
            if *__error() == EAGAIN {
                if !sbuf_queue_send(sbuf) {
                    sbuf_call_proto(sbuf, SBUF_EV_SEND_FAILED as ::core::ffi::c_int);
                }
            } else {
                sbuf_call_proto(sbuf, SBUF_EV_SEND_FAILED as ::core::ffi::c_int);
            }
            return false;
        }
    }
}

unsafe extern "C" fn sbuf_process_pending(mut sbuf: *mut SBuf) -> bool {
    let mut current_block: u64;
    let mut avail: ::core::ffi::c_uint = 0;
    let mut io = (*sbuf).io;
    let mut extra_packets: *mut MBuf = &raw mut (*sbuf).extra_packets;
    let mut full = iobuf_amount_recv(io) <= 0 as ::core::ffi::c_uint;
    let mut loop_number = 0 as ::core::ffi::c_int;
    let mut _log_ctx = NULL;
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(LG_NOISE, _log_ctx, c"sbuf_process_pending: start".as_ptr());
    }
    loop {
        if mbuf_avail_for_read(extra_packets) != 0 {
            if (*sbuf).extra_packet_queue_after && !sbuf_send_pending_iobuf(sbuf) {
                let mut _log_ctx_0 = NULL;
                if cf_verbose > 1 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_NOISE,
                        _log_ctx_0,
                        c"sbuf_process_pending failed to send all pending data".as_ptr(),
                    );
                }
                return false;
            }
            if !sbuf_send_pending_extra_packets(sbuf) {
                let mut _log_ctx_1 = NULL;
                if cf_verbose > 1 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_NOISE,
                        _log_ctx_1,
                        c"sbuf_process_pending ended early because of not being able to send the queued extra packets".as_ptr(),
                    );
                }
                return false;
            }
            if (*extra_packets).alloc_len
                > (cf_sbuf_len as ::core::ffi::c_uint).wrapping_mul(4 as ::core::ffi::c_uint)
            {
                mbuf_free(extra_packets);
            } else {
                mbuf_rewind_writer(extra_packets);
            }
        }
        loop_number += 1;
        let mut _log_ctx_2 = NULL;
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(
                LG_NOISE,
                _log_ctx_2,
                c"sbuf_process_pending: loop %d".as_ptr(),
                loop_number,
            );
        }
        avail = iobuf_amount_parse(io);
        if avail == 0 as ::core::ffi::c_uint
            || full && avail <= SBUF_SMALL_PKT as ::core::ffi::c_uint
        {
            current_block = 2706659501864706830;
            break;
        }
        if (*sbuf).pkt_remain == 0 as ::core::ffi::c_uint
            && !sbuf_call_proto(sbuf, SBUF_EV_READ as ::core::ffi::c_int)
        {
            current_block = 15038619569862261818;
            break;
        }
        if ((*sbuf).pkt_action as ::core::ffi::c_int == ACT_SKIP
            || (*sbuf).pkt_action as ::core::ffi::c_int == ACT_CALL)
            && iobuf_amount_pending(io) > 0 as ::core::ffi::c_uint
            && !sbuf_send_pending_iobuf(sbuf)
        {
            return false;
        }
        if avail > (*sbuf).pkt_remain {
            avail = (*sbuf).pkt_remain;
        }
        match (*sbuf).pkt_action as ::core::ffi::c_int {
            ACT_SEND => {
                iobuf_tag_send(io, avail);
                current_block = 13303144130133872306;
            }
            ACT_CALL => {
                if !sbuf_call_proto(sbuf, SBUF_EV_PKT_CALLBACK as ::core::ffi::c_int) {
                    current_block = 15038619569862261818;
                    break;
                }
                current_block = 6522610172562284459;
            }
            ACT_SKIP => {
                current_block = 6522610172562284459;
            }
            _ => {
                current_block = 13303144130133872306;
            }
        }
        if current_block == 6522610172562284459 {
            if (*sbuf).skip_remain >= avail {
                iobuf_tag_skip(io, avail);
                (*sbuf).skip_remain -= avail;
            } else {
                if (*sbuf).skip_remain != 0 as ::core::ffi::c_uint {
                    iobuf_tag_skip(io, (*sbuf).skip_remain);
                }
                iobuf_tag_send(io, avail.wrapping_sub((*sbuf).skip_remain));
                (*sbuf).skip_remain = 0 as ::core::ffi::c_uint;
            }
        }
        (*sbuf).pkt_remain -= avail;
    }
    match current_block {
        15038619569862261818 => {
            mbuf_rewind_writer(extra_packets);
            if (*sbuf).sock != 0
                && !io.is_null()
                && (*sbuf).wait_type as ::core::ffi::c_int == W_RECV as ::core::ffi::c_int
            {
                if iobuf_amount_pending(io) > 0 as ::core::ffi::c_uint
                    && !sbuf_send_pending_iobuf(sbuf)
                {
                    return false;
                }
                if !io.is_null() && (*io).recv_pos == cf_sbuf_len as ::core::ffi::c_uint {
                    let mut _log_ctx_6 = NULL;
                    if cf_verbose > 1 as ::core::ffi::c_int
                    {
                        log_generic(
                            LG_NOISE,
                            _log_ctx_6,
                            c"resync(%d): done=%u, parse=%u, recv=%u, forced".as_ptr(),
                            (*sbuf).sock,
                            (*io).done_pos,
                            (*io).parse_pos,
                            (*io).recv_pos,
                        );
                    }
                    iobuf_try_resync(io, cf_sbuf_len as ::core::ffi::c_uint);
                }
            }
            false
        }
        _ => {
            let mut _log_ctx_3 = NULL;
            if cf_verbose > 1 as ::core::ffi::c_int
            {
                log_generic(
                    LG_NOISE,
                    _log_ctx_3,
                    c"sbuf_process_pending: done looping".as_ptr(),
                );
            }
            if !sbuf_send_pending_iobuf(sbuf) {
                let mut _log_ctx_4 = NULL;
                if cf_verbose > 1 as ::core::ffi::c_int
                {
                    log_generic(
                        LG_NOISE,
                        _log_ctx_4,
                        c"sbuf_process_pending failed to send all pending data".as_ptr(),
                    );
                }
                return false;
            }
            let mut _log_ctx_5 = NULL;
            if cf_verbose > 1 as ::core::ffi::c_int
            {
                log_generic(LG_NOISE, _log_ctx_5, c"sbuf_process_pending: end".as_ptr());
            }
            true
        }
    }
}

unsafe extern "C" fn sbuf_try_resync(mut sbuf: *mut SBuf, mut release: bool) {
    let mut io = (*sbuf).io;
    if !io.is_null() {
        let mut _log_ctx = NULL;
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(
                LG_NOISE,
                _log_ctx,
                c"resync(%d): done=%u, parse=%u, recv=%u".as_ptr(),
                (*sbuf).sock,
                (*io).done_pos,
                (*io).parse_pos,
                (*io).recv_pos,
            );
        }
    }
    if io.is_null() {
        return;
    }
    if release && iobuf_empty(io) {
        slab_free(iobuf_cache, io as *mut ::core::ffi::c_void);
        (*sbuf).io = ::core::ptr::null_mut::<IOBuf>();
    } else {
        iobuf_try_resync(io, SBUF_SMALL_PKT as ::core::ffi::c_uint);
    };
}

unsafe extern "C" fn sbuf_actual_recv(mut sbuf: *mut SBuf, mut len: size_t) -> bool {
    let mut got: ssize_t = 0;
    let mut io = (*sbuf).io;
    let mut dst = (&raw mut (*io).buf as *mut uint8_t).offset((*io).recv_pos as isize);
    let mut avail = iobuf_amount_recv(io);
    if len > avail as size_t {
        len = avail as size_t;
    }
    got = sbuf_op_recv(sbuf, dst as *mut ::core::ffi::c_void, len);
    if got > 0 as ssize_t {
        (*io).recv_pos = ((*io).recv_pos as ssize_t + got) as ::core::ffi::c_uint;
    } else if got == 0 as ssize_t {
        sbuf_call_proto(sbuf, SBUF_EV_RECV_FAILED as ::core::ffi::c_int);
        return false;
    } else if got < 0 as ssize_t && *__error() != EAGAIN {
        sbuf_call_proto(sbuf, SBUF_EV_RECV_FAILED as ::core::ffi::c_int);
        return false;
    }
    true
}

unsafe extern "C" fn sbuf_recv_cb(
    mut _sock: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut sbuf = arg as *mut SBuf;
    sbuf_main_loop(sbuf, DO_RECV != 0);
}

unsafe extern "C" fn allocate_iobuf(mut sbuf: *mut SBuf) -> bool {
    if (*sbuf).io.is_null() {
        (*sbuf).io = slab_alloc(iobuf_cache) as *mut IOBuf;
        if (*sbuf).io.is_null() {
            sbuf_call_proto(sbuf, SBUF_EV_RECV_FAILED as ::core::ffi::c_int);
            return false;
        }
        iobuf_reset((*sbuf).io);
    }
    true
}

unsafe extern "C" fn sbuf_main_loop(mut sbuf: *mut SBuf, mut skip_recv: bool) {
    let mut current_block: u64;
    let mut free_0: ::core::ffi::c_uint = 0;
    let mut ok: ::core::ffi::c_uint = 0;
    let mut loopcnt = 0 as ::core::ffi::c_int;
    if (*sbuf).sock == 0 {
        return;
    }
    if !allocate_iobuf(sbuf) {
        return;
    }
    if skip_recv {
        current_block = 7691811283911721328;
    } else {
        current_block = 17573018301231930905;
    }
    loop {
        match current_block {
            17573018301231930905 => {
                sbuf_try_resync(sbuf, false);
                if cf_sbuf_loopcnt > 0 as ::core::ffi::c_int && loopcnt >= cf_sbuf_loopcnt {
                    let mut _ignore: bool = false;
                    let mut _log_ctx = NULL;
                    if cf_verbose > 0 as ::core::ffi::c_int
                    {
                        log_generic(LG_DEBUG, _log_ctx, c"loopcnt full".as_ptr());
                    }
                    _ignore = sbuf_process_pending(sbuf);
                    sbuf_wait_for_data_forced(sbuf);
                    return;
                }
                loopcnt += 1;
                free_0 = iobuf_amount_recv((*sbuf).io);
                if free_0 > 0 as ::core::ffi::c_uint {
                    if cf_pause_mode == P_SUSPEND as ::core::ffi::c_int
                        && (*sbuf).pkt_remain > 0 as ::core::ffi::c_uint
                        && (*sbuf).pkt_remain < free_0
                    {
                        free_0 = (*sbuf).pkt_remain;
                    }
                    ok = sbuf_actual_recv(sbuf, free_0 as size_t) as ::core::ffi::c_uint;
                    if ok == 0 {
                        return;
                    }
                }
                current_block = 7691811283911721328;
            }
            _ => {
                ok = sbuf_process_pending(sbuf) as ::core::ffi::c_uint;
                if ok == 0 {
                    return;
                }
                if iobuf_amount_recv((*sbuf).io) <= 0 as ::core::ffi::c_uint {
                    current_block = 17573018301231930905;
                } else {
                    break;
                }
            }
        }
    }
    sbuf_try_resync(sbuf, true);
    if sbuf_is_empty(sbuf) {
        sbuf_call_proto(sbuf, SBUF_EV_FLUSH as ::core::ffi::c_int);
    }
    if (*sbuf).tls_state as ::core::ffi::c_int == SBUF_TLS_DO_HANDSHAKE as ::core::ffi::c_int {
        (*sbuf).pkt_action = SBUF_TLS_IN_HANDSHAKE as ::core::ffi::c_int as uint8_t;
        if !handle_tls_handshake(sbuf) {
            sbuf_call_proto(sbuf, SBUF_EV_RECV_FAILED as ::core::ffi::c_int);
        }
    }
}

unsafe extern "C" fn sbuf_after_connect_check(mut sbuf: *mut SBuf) -> bool {
    let mut optval = 0 as ::core::ffi::c_int;
    let mut err: ::core::ffi::c_int = 0;
    let mut optlen: socklen_t = ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t;
    err = getsockopt(
        (*sbuf).sock,
        SOL_SOCKET,
        SO_ERROR,
        &raw mut optval as *mut ::core::ffi::c_void,
        &raw mut optlen,
    );
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        if cf_verbose > 0 as ::core::ffi::c_int
        {
            log_generic(
                LG_DEBUG,
                _log_ctx,
                c"sbuf_after_connect_check: getsockopt: %s".as_ptr(),
                strerror(*__error()),
            );
        }
        return false;
    }
    if optval != 0 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        if cf_verbose > 0 as ::core::ffi::c_int
        {
            log_generic(
                LG_DEBUG,
                _log_ctx_0,
                c"sbuf_after_connect_check: pending error: %s".as_ptr(),
                strerror(optval),
            );
        }
        return false;
    }
    true
}

unsafe extern "C" fn sbuf_connect_cb(
    mut _sock: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_short,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut sbuf = arg as *mut SBuf;
    (*sbuf).wait_type = W_NONE as ::core::ffi::c_int as uint8_t;
    if flags as ::core::ffi::c_int & EV_WRITE != 0 && sbuf_after_connect_check(sbuf) {
        if !sbuf_call_proto(sbuf, SBUF_EV_CONNECT_OK as ::core::ffi::c_int) {
            return;
        }
        if sbuf_wait_for_data(sbuf) {
            return;
        }
    }
    sbuf_call_proto(sbuf, SBUF_EV_CONNECT_FAILED as ::core::ffi::c_int);
}
#[no_mangle]

pub unsafe extern "C" fn sbuf_answer(
    mut sbuf: *mut SBuf,
    mut buf: *const ::core::ffi::c_void,
    mut len: size_t,
) -> bool {
    let mut res: ssize_t = 0;
    if (*sbuf).sock <= 0 as ::core::ffi::c_int {
        return false;
    }
    res = sbuf_op_send(sbuf, buf, len);
    if res < 0 as ssize_t {
        let mut _log_ctx = NULL;
        if cf_verbose > 0 as ::core::ffi::c_int
        {
            log_generic(
                LG_DEBUG,
                _log_ctx,
                c"sbuf_answer: error sending: %s".as_ptr(),
                strerror(*__error()),
            );
        }
    } else if res as ::core::ffi::c_uint as size_t != len {
        let mut _log_ctx_0 = NULL;
        if cf_verbose > 0 as ::core::ffi::c_int
        {
            log_generic(
                LG_DEBUG,
                _log_ctx_0,
                c"sbuf_answer: partial send: len=%zu sent=%zd".as_ptr(),
                len,
                res,
            );
        }
    }
    res as ::core::ffi::c_uint as size_t == len
}

unsafe extern "C" fn raw_sbufio_peek(
    mut sbuf: *mut SBuf,
    mut buf: *mut ::core::ffi::c_void,
    mut len: size_t,
) -> ssize_t {
    safe_recv((*sbuf).sock, buf, len, 0x2 as ::core::ffi::c_int)
}

unsafe extern "C" fn raw_sbufio_recv(
    mut sbuf: *mut SBuf,
    mut dst: *mut ::core::ffi::c_void,
    mut len: size_t,
) -> ssize_t {
    safe_recv((*sbuf).sock, dst, len, 0 as ::core::ffi::c_int)
}

unsafe extern "C" fn raw_sbufio_send(
    mut sbuf: *mut SBuf,
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
) -> ssize_t {
    safe_send((*sbuf).sock, data, len, 0 as ::core::ffi::c_int)
}

unsafe extern "C" fn raw_sbufio_close(mut sbuf: *mut SBuf) -> ::core::ffi::c_int {
    if (*sbuf).sock > 0 as ::core::ffi::c_int {
        safe_close((*sbuf).sock);
        (*sbuf).sock = 0 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}

static mut client_accept_base: *mut tls = ::core::ptr::null::<tls>() as *mut tls;

static mut client_accept_conf: *mut tls_config =
    ::core::ptr::null::<tls_config>() as *mut tls_config;
#[no_mangle]

pub static mut client_accept_sslmode: ::core::ffi::c_int = 0;

static mut server_connect_conf: *mut tls_config =
    ::core::ptr::null::<tls_config>() as *mut tls_config;
#[no_mangle]

pub static mut server_connect_sslmode: ::core::ffi::c_int = 0;

unsafe extern "C" fn setup_tls(
    mut conf: *mut tls_config,
    mut pfx: *const ::core::ffi::c_char,
    mut sslmode: ::core::ffi::c_int,
    mut protocols: *const ::core::ffi::c_char,
    mut ciphers: *const ::core::ffi::c_char,
    mut ciphers13: *const ::core::ffi::c_char,
    mut keyfile: *const ::core::ffi::c_char,
    mut certfile: *const ::core::ffi::c_char,
    mut cafile: *const ::core::ffi::c_char,
    mut dheparams: *const ::core::ffi::c_char,
    mut ecdhecurve: *const ::core::ffi::c_char,
    mut does_connect: bool,
) -> bool {
    let mut err: ::core::ffi::c_int = 0;
    if *protocols != 0 {
        let mut protos: uint32_t = TLS_PROTOCOLS_ALL as uint32_t;
        err = tls_config_parse_protocols(&raw mut protos, protocols);
        if err != 0 {
            let mut _log_ctx = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx,
                c"invalid %s_protocols: %s".as_ptr(),
                pfx,
                protocols,
            );
            return false;
        }
        tls_config_set_protocols(conf, protos);
    }
    if *ciphers != 0 {
        err = tls_config_set_ciphers(conf, ciphers);
        if err != 0 {
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_0,
                c"invalid %s_ciphers: %s".as_ptr(),
                pfx,
                ciphers,
            );
            return false;
        }
    }
    if !ciphers13.is_null() {
        err = tls_config_set_ciphers_v13(conf, ciphers13);
        if err != 0 {
            let mut _log_ctx_1 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_1,
                c"invalid %s_ciphers: %s".as_ptr(),
                pfx,
                ciphers13,
            );
            return false;
        }
    }
    if *dheparams != 0 {
        err = tls_config_set_dheparams(conf, dheparams);
        if err != 0 {
            let mut _log_ctx_2 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_2,
                c"invalid %s_dheparams: %s".as_ptr(),
                pfx,
                dheparams,
            );
            return false;
        }
    }
    if *ecdhecurve != 0 {
        err = tls_config_set_ecdhecurve(conf, ecdhecurve);
        if err != 0 {
            let mut _log_ctx_3 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_3,
                c"invalid %s_ecdhecurve: %s".as_ptr(),
                pfx,
                ecdhecurve,
            );
            return false;
        }
    }
    if *cafile != 0 {
        err = tls_config_set_ca_file(conf, cafile);
        if err != 0 {
            let mut _log_ctx_4 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_4,
                c"invalid %s_ca_file: %s".as_ptr(),
                pfx,
                cafile,
            );
            return false;
        }
    }
    if *keyfile != 0 {
        err = tls_config_set_key_file(conf, keyfile);
        if err != 0 {
            let mut _log_ctx_5 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_5,
                c"invalid %s_key_file: %s".as_ptr(),
                pfx,
                keyfile,
            );
            return false;
        }
    }
    if *certfile != 0 {
        err = tls_config_set_cert_file(conf, certfile);
        if err != 0 {
            let mut _log_ctx_6 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_6,
                c"invalid %s_cert_file: %s".as_ptr(),
                pfx,
                certfile,
            );
            return false;
        }
    }
    if does_connect {
        if sslmode == SSLMODE_VERIFY_FULL as ::core::ffi::c_int {
            tls_config_verify(conf);
        } else if sslmode == SSLMODE_VERIFY_CA as ::core::ffi::c_int {
            tls_config_verify(conf);
            tls_config_insecure_noverifyname(conf);
        } else {
            tls_config_insecure_noverifycert(conf);
            tls_config_insecure_noverifyname(conf);
        }
    } else if sslmode == SSLMODE_VERIFY_FULL as ::core::ffi::c_int {
        tls_config_verify_client(conf);
    } else if sslmode == SSLMODE_VERIFY_CA as ::core::ffi::c_int {
        tls_config_verify_client(conf);
    } else {
        tls_config_verify_client_optional(conf);
    }
    true
}

unsafe extern "C" fn tls_change_requires_reconnect(
    mut new_server_connect_conf: *mut tls_config,
) -> bool {
    if server_connect_sslmode != cf_server_tls_sslmode {
        let mut _log_ctx = NULL;
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(
                LG_NOISE,
                _log_ctx,
                c"new server_tls_sslmode detected".as_ptr(),
            );
        }
        true
    } else if server_connect_conf.is_null() {
        let mut _log_ctx_0 = NULL;
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(
                LG_NOISE,
                _log_ctx_0,
                c"no existing server tls config detected".as_ptr(),
            );
        }
        true
    } else if tls_config_equal(new_server_connect_conf, server_connect_conf) {
        let mut _log_ctx_1 = NULL;
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(
                LG_NOISE,
                _log_ctx_1,
                c"no server tls config change detected".as_ptr(),
            );
        }
        false
    } else {
        let mut _log_ctx_2 = NULL;
        if cf_verbose > 1 as ::core::ffi::c_int
        {
            log_generic(
                LG_NOISE,
                _log_ctx_2,
                c"server tls config change detected".as_ptr(),
            );
        }
        true
    }
}
#[no_mangle]

pub unsafe extern "C" fn sbuf_tls_setup() -> bool {
    let mut current_block: u64;
    let mut err: ::core::ffi::c_int = 0;
    let mut new_client_accept_conf = ::core::ptr::null_mut::<tls_config>();
    let mut new_server_connect_conf = ::core::ptr::null_mut::<tls_config>();
    let mut new_client_accept_base = ::core::ptr::null_mut::<tls>();
    if cf_client_tls_sslmode != SSLMODE_DISABLED as ::core::ffi::c_int
        && (*cf_client_tls_key_file == 0 || *cf_client_tls_cert_file == 0)
    {
        let mut _log_ctx = NULL;
        log_generic(
                LG_ERROR,
                _log_ctx,
                c"To allow TLS connections from clients, client_tls_key_file and client_tls_cert_file must be set.".as_ptr(),
            );
        return false;
    }
    if cf_auth_type == AUTH_TYPE_CERT as ::core::ffi::c_int {
        if cf_client_tls_sslmode != SSLMODE_VERIFY_FULL as ::core::ffi::c_int {
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_0,
                c"auth_type=cert requires client_tls_sslmode=SSLMODE_VERIFY_FULL".as_ptr(),
            );
            return false;
        }
        if *cf_client_tls_ca_file as ::core::ffi::c_int == '\0' as i32 {
            let mut _log_ctx_1 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_1,
                c"auth_type=cert requires client_tls_ca_file".as_ptr(),
            );
            return false;
        }
    } else if cf_client_tls_sslmode > SSLMODE_VERIFY_CA as ::core::ffi::c_int
        && *cf_client_tls_ca_file as ::core::ffi::c_int == '\0' as i32
    {
        let mut _log_ctx_2 = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx_2,
            c"client_tls_sslmode requires client_tls_ca_file".as_ptr(),
        );
        return false;
    }
    err = tls_init();
    if err != 0 {
        let mut _log_ctx_3 = NULL;
        log_fatal(
            c"src/sbuf.c".as_ptr(),
            1311 as ::core::ffi::c_int,
            c"sbuf_tls_setup".as_ptr(),
            false,
            _log_ctx_3,
            c"tls_init failed".as_ptr(),
        );
        exit(1 as ::core::ffi::c_int);
    }
    if cf_server_tls_sslmode != SSLMODE_DISABLED as ::core::ffi::c_int {
        new_server_connect_conf = tls_config_new();
        if new_server_connect_conf.is_null() {
            let mut _log_ctx_4 = NULL;
            log_generic(LG_ERROR, _log_ctx_4, c"tls_config_new failed 1".as_ptr());
            return false;
        }
        if !setup_tls(
            new_server_connect_conf,
            c"server_tls".as_ptr(),
            cf_server_tls_sslmode,
            cf_server_tls_protocols,
            cf_server_tls_ciphers,
            cf_server_tls13_ciphers,
            cf_server_tls_key_file,
            cf_server_tls_cert_file,
            cf_server_tls_ca_file,
            c"".as_ptr(),
            c"".as_ptr(),
            true,
        ) {
            current_block = 4939747223443714714;
        } else {
            current_block = 14832935472441733737;
        }
    } else {
        current_block = 14832935472441733737;
    }
    if current_block == 14832935472441733737 {
        if cf_client_tls_sslmode != SSLMODE_DISABLED as ::core::ffi::c_int {
            new_client_accept_conf = tls_config_new();
            if new_client_accept_conf.is_null() {
                let mut _log_ctx_5 = NULL;
                log_generic(LG_ERROR, _log_ctx_5, c"tls_config_new failed 2".as_ptr());
                current_block = 4939747223443714714;
            } else if !setup_tls(
                new_client_accept_conf,
                c"client_tls".as_ptr(),
                cf_client_tls_sslmode,
                cf_client_tls_protocols,
                cf_client_tls_ciphers,
                cf_client_tls13_ciphers,
                cf_client_tls_key_file,
                cf_client_tls_cert_file,
                cf_client_tls_ca_file,
                cf_client_tls_dheparams,
                cf_client_tls_ecdhecurve,
                false,
            ) {
                current_block = 4939747223443714714;
            } else {
                new_client_accept_base = tls_server();
                if new_client_accept_base.is_null() {
                    let mut _log_ctx_6 = NULL;
                    log_generic(LG_ERROR, _log_ctx_6, c"server_base failed".as_ptr());
                    current_block = 4939747223443714714;
                } else {
                    err = tls_configure(new_client_accept_base, new_client_accept_conf);
                    if err != 0 {
                        let mut _log_ctx_7 = NULL;
                        log_generic(
                            LG_ERROR,
                            _log_ctx_7,
                            c"TLS setup failed: %s".as_ptr(),
                            tls_error(new_client_accept_base),
                        );
                        current_block = 4939747223443714714;
                    } else {
                        current_block = 17233182392562552756;
                    }
                }
            }
        } else {
            current_block = 17233182392562552756;
        }
        match current_block {
            4939747223443714714 => {}
            _ => {
                if (!server_connect_conf.is_null() || !new_server_connect_conf.is_null())
                    && tls_change_requires_reconnect(new_server_connect_conf) as ::core::ffi::c_int
                        != 0
                {
                    let mut item = ::core::ptr::null_mut::<List>();
                    let mut pool = ::core::ptr::null_mut::<PgPool>();
                    item = pool_list.head.next;
                    while item != &raw mut pool_list.head {
                        pool = (item as *mut ::core::ffi::c_char)
                            
                            as *mut PgPool;
                        tag_pool_dirty(pool);
                        item = (*item).next;
                    }
                }
                usual_tls_free(client_accept_base);
                tls_config_free(client_accept_conf);
                tls_config_free(server_connect_conf);
                client_accept_base = new_client_accept_base;
                client_accept_conf = new_client_accept_conf;
                client_accept_sslmode = cf_client_tls_sslmode;
                server_connect_conf = new_server_connect_conf;
                server_connect_sslmode = cf_server_tls_sslmode;
                return true;
            }
        }
    }
    usual_tls_free(new_client_accept_base);
    tls_config_free(new_client_accept_conf);
    tls_config_free(new_server_connect_conf);
    false
}

unsafe extern "C" fn handle_tls_handshake(mut sbuf: *mut SBuf) -> bool {
    let mut err: ::core::ffi::c_int = 0;
    err = tls_handshake((*sbuf).tls);
    let mut _log_ctx = NULL;
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(LG_NOISE, _log_ctx, c"tls_handshake: err=%d".as_ptr(), err);
    }
    if err == TLS_WANT_POLLIN {
        sbuf_use_callback_once(
            sbuf,
            EV_READ as ::core::ffi::c_short,
            Some(
                sbuf_tls_handshake_cb
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_short,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
        )
    } else if err == TLS_WANT_POLLOUT {
        sbuf_use_callback_once(
            sbuf,
            EV_WRITE as ::core::ffi::c_short,
            Some(
                sbuf_tls_handshake_cb
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_short,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
        )
    } else if err == 0 as ::core::ffi::c_int {
        (*sbuf).tls_state = SBUF_TLS_OK as ::core::ffi::c_int as uint8_t;
        sbuf_call_proto(sbuf, SBUF_EV_TLS_READY as ::core::ffi::c_int);
        true
    } else {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_0,
            c"TLS handshake error: %s".as_ptr(),
            tls_error((*sbuf).tls),
        );
        false
    }
}

unsafe extern "C" fn sbuf_tls_handshake_cb(
    mut _fd: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut _sbuf: *mut ::core::ffi::c_void,
) {
    let mut sbuf = _sbuf as *mut SBuf;
    (*sbuf).wait_type = W_NONE as ::core::ffi::c_int as uint8_t;
    if !handle_tls_handshake(sbuf) {
        sbuf_call_proto(sbuf, SBUF_EV_RECV_FAILED as ::core::ffi::c_int);
    }
}
#[no_mangle]

pub unsafe extern "C" fn sbuf_tls_accept(mut sbuf: *mut SBuf) -> bool {
    let mut err: ::core::ffi::c_int = 0;
    if !sbuf_pause(sbuf) {
        return false;
    }
    (*sbuf).ops = &raw const tls_sbufio_ops;
    err = tls_accept_fds(
        client_accept_base,
        &raw mut (*sbuf).tls,
        (*sbuf).sock,
        (*sbuf).sock,
    );
    let mut _log_ctx = NULL;
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(LG_NOISE, _log_ctx, c"tls_accept_fds: err=%d".as_ptr(), err);
    }
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_0,
            c"TLS accept error: %s".as_ptr(),
            tls_error((*sbuf).tls),
        );
        return false;
    }
    (*sbuf).tls_state = SBUF_TLS_DO_HANDSHAKE as ::core::ffi::c_int as uint8_t;
    true
}
#[no_mangle]

pub unsafe extern "C" fn sbuf_tls_connect(
    mut sbuf: *mut SBuf,
    mut hostname: *const ::core::ffi::c_char,
) -> bool {
    let mut ctls = ::core::ptr::null_mut::<tls>();
    let mut err: ::core::ffi::c_int = 0;
    if !sbuf_pause(sbuf) {
        return false;
    }
    if cf_server_tls_sslmode != SSLMODE_VERIFY_FULL as ::core::ffi::c_int {
        hostname = ::core::ptr::null::<::core::ffi::c_char>();
    }
    ctls = tls_client();
    if ctls.is_null() {
        return false;
    }
    err = tls_configure(ctls, server_connect_conf);
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx,
            c"tls client config failed: %s".as_ptr(),
            tls_error(ctls),
        );
        usual_tls_free(ctls);
        return false;
    }
    (*sbuf).tls = ctls;
    (*sbuf).tls_host = hostname;
    (*sbuf).ops = &raw const tls_sbufio_ops;
    err = tls_connect_fds((*sbuf).tls, (*sbuf).sock, (*sbuf).sock, (*sbuf).tls_host);
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_0,
            c"TLS connect error: %s".as_ptr(),
            tls_error((*sbuf).tls),
        );
        return false;
    }
    (*sbuf).tls_state = SBUF_TLS_DO_HANDSHAKE as ::core::ffi::c_int as uint8_t;
    true
}

unsafe extern "C" fn tls_sbufio_peek(
    mut _sbuf: *mut SBuf,
    mut _buf: *mut ::core::ffi::c_void,
    mut _len: size_t,
) -> ssize_t {
    -(1 as ::core::ffi::c_int) as ssize_t
}

unsafe extern "C" fn tls_sbufio_recv(
    mut sbuf: *mut SBuf,
    mut dst: *mut ::core::ffi::c_void,
    mut len: size_t,
) -> ssize_t {
    let mut out: ssize_t = 0 as ssize_t;
    if (*sbuf).tls_state as ::core::ffi::c_int != SBUF_TLS_OK as ::core::ffi::c_int {
        *__error() = EIO;
        return -(1 as ::core::ffi::c_int) as ssize_t;
    }
    out = tls_read((*sbuf).tls, dst, len);
    let mut _log_ctx = NULL;
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(
            LG_NOISE,
            _log_ctx,
            c"tls_read: req=%zu out=%zd".as_ptr(),
            len,
            out,
        );
    }
    if out >= 0 as ssize_t {
        return out;
    } else if out == TLS_WANT_POLLIN as ssize_t {
        *__error() = EAGAIN;
    } else if out == TLS_WANT_POLLOUT as ssize_t {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_0,
            c"tls_sbufio_recv: got TLS_WANT_POLLOUT".as_ptr(),
        );
        *__error() = EIO;
    } else {
        let mut _log_ctx_1 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_1,
            c"tls_sbufio_recv: %s".as_ptr(),
            tls_error((*sbuf).tls),
        );
        *__error() = EIO;
    }
    -(1 as ::core::ffi::c_int) as ssize_t
}

unsafe extern "C" fn tls_sbufio_send(
    mut sbuf: *mut SBuf,
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
) -> ssize_t {
    let mut out: ssize_t = 0;
    if (*sbuf).tls_state as ::core::ffi::c_int != SBUF_TLS_OK as ::core::ffi::c_int {
        *__error() = EIO;
        return -(1 as ::core::ffi::c_int) as ssize_t;
    }
    out = tls_write((*sbuf).tls, data, len);
    let mut _log_ctx = NULL;
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(
            LG_NOISE,
            _log_ctx,
            c"tls_write: req=%zu out=%zd".as_ptr(),
            len,
            out,
        );
    }
    if out >= 0 as ssize_t {
        return out;
    } else if out == TLS_WANT_POLLOUT as ssize_t {
        *__error() = EAGAIN;
    } else if out == TLS_WANT_POLLIN as ssize_t {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_0,
            c"tls_sbufio_send: got TLS_WANT_POLLIN".as_ptr(),
        );
        *__error() = EIO;
    } else {
        let mut _log_ctx_1 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_1,
            c"tls_sbufio_send: %s".as_ptr(),
            tls_error((*sbuf).tls),
        );
        *__error() = EIO;
    }
    -(1 as ::core::ffi::c_int) as ssize_t
}

unsafe extern "C" fn tls_sbufio_close(mut sbuf: *mut SBuf) -> ::core::ffi::c_int {
    let mut _log_ctx = NULL;
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(LG_NOISE, _log_ctx, c"tls_close".as_ptr());
    }
    if !(*sbuf).tls.is_null() {
        tls_close((*sbuf).tls);
        usual_tls_free((*sbuf).tls);
        (*sbuf).tls = ::core::ptr::null_mut::<tls>();
    }
    if (*sbuf).sock > 0 as ::core::ffi::c_int {
        safe_close((*sbuf).sock);
        (*sbuf).sock = 0 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}
#[no_mangle]

pub unsafe extern "C" fn sbuf_cleanup() {
    usual_tls_free(client_accept_base);
    tls_config_free(client_accept_conf);
    tls_config_free(server_connect_conf);
    client_accept_conf = ::core::ptr::null_mut::<tls_config>();
    server_connect_conf = ::core::ptr::null_mut::<tls_config>();
    client_accept_base = ::core::ptr::null_mut::<tls>();
}

unsafe extern "C" fn handle_possible_direct_tls_startup(
    mut sbuf: *mut SBuf,
    mut is_unix: bool,
) -> bool {
    if client_accept_sslmode == SSLMODE_DISABLED as ::core::ffi::c_int
        || is_unix
    {
        return true;
    }
    sbuf_use_callback_once(
        sbuf,
        EV_READ as ::core::ffi::c_short,
        Some(
            sbuf_possible_direct_tls_startup_cb
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_short,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
    )
}

unsafe extern "C" fn sbuf_possible_direct_tls_startup_cb(
    mut _fd: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut _sbuf: *mut ::core::ffi::c_void,
) {
    let mut peek_byte: [uint8_t; 1] = [0; 1];
    let mut got: ssize_t = 0;
    let mut sbuf = _sbuf as *mut SBuf;
    let mut client = (sbuf as *mut ::core::ffi::c_char)
        .offset(-(520 as ::core::ffi::c_ulong as isize)) as *mut PgSocket;
    (*sbuf).wait_type = W_RECV as ::core::ffi::c_int as uint8_t;
    got = sbuf_op_peek(
        sbuf,
        &raw mut peek_byte as *mut uint8_t as *mut ::core::ffi::c_void,
        1 as size_t,
    );
    if got <= 0 as ssize_t {
        sbuf_call_proto(sbuf, SBUF_EV_RECV_FAILED as ::core::ffi::c_int);
        return;
    }
    if peek_byte[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        != 0x16 as ::core::ffi::c_int
    {
        sbuf_continue(sbuf);
        return;
    }
    let mut _log_ctx = NULL;
    if cf_verbose > 1 as ::core::ffi::c_int {
        log_generic(LG_NOISE, _log_ctx, c"Starting TLS handshake".as_ptr());
    }
    if !sbuf_tls_accept(sbuf) {
        disconnect_client(client, false, c"failed to accept SSL".as_ptr());
        return;
    }
    (*sbuf).pkt_action = SBUF_TLS_IN_HANDSHAKE as ::core::ffi::c_int as uint8_t;
    if !handle_tls_handshake(sbuf) {
        sbuf_call_proto(sbuf, SBUF_EV_RECV_FAILED as ::core::ffi::c_int);
    }
}
