//! Shared type definitions for pgbouncer c2rust codebase.
//!
//! This module consolidates all commonly duplicated type definitions
//! to reduce code duplication and ensure type consistency across modules.

#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]

// =============================================================================
// Primitive C types (libc equivalents)
// =============================================================================

// Internal darwin types
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __darwin_ptrdiff_t = isize;
pub type __darwin_size_t = usize;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_ssize_t = isize;
pub type __darwin_time_t = ::core::ffi::c_long;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_sigset_t = __uint32_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;

// Standard C types
pub type uintptr_t = usize;
pub type in_addr_t = __uint32_t;
pub type in_port_t = __uint16_t;
pub type pid_t = __darwin_pid_t;
pub type uid_t = __darwin_uid_t;
pub type size_t = __darwin_size_t;
pub type ssize_t = __darwin_ssize_t;
pub type time_t = __darwin_time_t;
pub type sigset_t = __darwin_sigset_t;
pub type ptrdiff_t = __darwin_ptrdiff_t;
pub type socklen_t = __darwin_socklen_t;
pub type sa_family_t = __uint8_t;
pub type rlim_t = __uint64_t;
pub type fpos_t = __darwin_off_t;

// Fixed-width integers
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

// =============================================================================
// libc constants
// =============================================================================

pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ENOMEM: ::core::ffi::c_int = 12;
pub const EINTR: ::core::ffi::c_int = 4;
pub const EAGAIN: ::core::ffi::c_int = 35;
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const EINPROGRESS: ::core::ffi::c_int = 36;

// Socket constants
pub const AF_UNIX: ::core::ffi::c_int = 1;
pub const AF_INET: ::core::ffi::c_int = 2;
pub const AF_INET6: ::core::ffi::c_int = 30;
pub const SOCK_STREAM: ::core::ffi::c_int = 1;
pub const SOCK_DGRAM: ::core::ffi::c_int = 2;

// Resource limits
pub const RLIMIT_NOFILE: ::core::ffi::c_int = 8;

// Boolean constants (for c2rust compatibility)
pub const true_0: ::core::ffi::c_int = 1;
pub const false_0: ::core::ffi::c_int = 0;

// =============================================================================
// Struct definitions - libc types
// =============================================================================

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __darwin_time_t,
    pub tv_usec: __darwin_suseconds_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: ::core::ffi::c_long,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}

// Socket address structures
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_len: __uint8_t,
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_len: ::core::ffi::c_uchar,
    pub sun_family: sa_family_t,
    pub sun_path: [::core::ffi::c_char; 104],
}

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __u6_addr: in6_addr_union,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union in6_addr_union {
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

// FILE structure
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

// =============================================================================
// usual library types
// =============================================================================

pub type usec_t = uint64_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct List {
    pub next: *mut List,
    pub prev: *mut List,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct StatList {
    pub head: List,
    pub cur_count: ::core::ffi::c_int,
}

impl StatList {
    #[inline]
    pub unsafe fn count(&self) -> ::core::ffi::c_int {
        self.cur_count
    }
}

#[inline]
pub unsafe extern "C" fn statlist_count(list: *const StatList) -> ::core::ffi::c_int {
    (*list).cur_count
}

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

pub type aatree_cmp_f = Option<unsafe extern "C" fn(uintptr_t, *mut AANode) -> ::core::ffi::c_int>;

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CxMem {
    pub ops: *const CxOps,
    pub parent: *const CxMem,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CxOps {
    pub c_alloc:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>,
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
pub struct StrList {
    pub ca: *const CxMem,
    pub items: StatList,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct StrItem {
    pub node: List,
    pub str_0: [::core::ffi::c_char; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PStr {
    pub pool: *mut StrPool,
    pub len: size_t,
    pub refcnt: ::core::ffi::c_int,
    pub str_0: [::core::ffi::c_char; 0],
}

// =============================================================================
// libevent types
// =============================================================================

pub type event_callback_fn = Option<
    unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_short, *mut ::core::ffi::c_void) -> (),
>;

pub const EVLOOP_ONCE: ::core::ffi::c_int = 0x1;
pub const EVLOOP_NONBLOCK: ::core::ffi::c_int = 0x2;
pub const EV_READ: ::core::ffi::c_int = 0x2;
pub const EV_WRITE: ::core::ffi::c_int = 0x4;
pub const EV_SIGNAL: ::core::ffi::c_int = 0x8;
pub const EV_PERSIST: ::core::ffi::c_int = 0x10;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub ev_evcallback: event_callback_struct,
    pub ev_timeout_pos: event_timeout_pos,
    pub ev_fd: ::core::ffi::c_int,
    pub ev_base: *mut event_base,
    pub ev_: event_union,
    pub ev_events: ::core::ffi::c_short,
    pub ev_res: ::core::ffi::c_short,
    pub ev_timeout: timeval,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union event_union {
    pub ev_io: event_io,
    pub ev_signal: event_signal,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_signal {
    pub ev_signal_next: event_signal_next,
    pub ev_ncalls: ::core::ffi::c_short,
    pub ev_pncalls: *mut ::core::ffi::c_short,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_signal_next {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_io {
    pub ev_io_next: event_io_next,
    pub ev_timeout: timeval,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_io_next {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union event_timeout_pos {
    pub ev_next_with_common_timeout: event_next_with_common_timeout,
    pub min_heap_idx: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_next_with_common_timeout {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback_struct {
    pub evcb_active_next: event_callback_active_next,
    pub evcb_flags: ::core::ffi::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: event_callback_union,
    pub evcb_arg: *mut ::core::ffi::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union event_callback_union {
    pub evcb_callback: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_short,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub evcb_selfcb:
        Option<unsafe extern "C" fn(*mut event_callback_struct, *mut ::core::ffi::c_void) -> ()>,
    pub evcb_evfinalize: Option<unsafe extern "C" fn(*mut event, *mut ::core::ffi::c_void) -> ()>,
    pub evcb_cbfinalize:
        Option<unsafe extern "C" fn(*mut event_callback_struct, *mut ::core::ffi::c_void) -> ()>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback_active_next {
    pub tqe_next: *mut event_callback_struct,
    pub tqe_prev: *mut *mut event_callback_struct,
}

// =============================================================================
// pgbouncer core types - enums
// =============================================================================

pub type SocketState = ::core::ffi::c_uint;
pub const CL_FREE: SocketState = 0;
pub const CL_JUSTFREE: SocketState = 1;
pub const CL_LOGIN: SocketState = 2;
pub const CL_WAITING: SocketState = 3;
pub const CL_WAITING_LOGIN: SocketState = 4;
pub const CL_ACTIVE: SocketState = 5;
pub const CL_WAITING_CANCEL: SocketState = 6;
pub const CL_ACTIVE_CANCEL: SocketState = 7;
pub const SV_FREE: SocketState = 8;
pub const SV_JUSTFREE: SocketState = 9;
pub const SV_LOGIN: SocketState = 10;
pub const SV_BEING_CANCELED: SocketState = 11;
pub const SV_IDLE: SocketState = 12;
pub const SV_ACTIVE: SocketState = 13;
pub const SV_ACTIVE_CANCEL: SocketState = 14;
pub const SV_USED: SocketState = 15;
pub const SV_TESTED: SocketState = 16;

pub type PauseMode = ::core::ffi::c_uint;
pub const P_NONE: PauseMode = 0;
pub const P_PAUSE: PauseMode = 1;
pub const P_SUSPEND: PauseMode = 2;

pub type ShutDownMode = ::core::ffi::c_uint;
pub const SHUTDOWN_NONE: ShutDownMode = 0;
pub const SHUTDOWN_WAIT_FOR_SERVERS: ShutDownMode = 1;
pub const SHUTDOWN_WAIT_FOR_CLIENTS: ShutDownMode = 2;
pub const SHUTDOWN_IMMEDIATE: ShutDownMode = 3;

pub type SSLMode = ::core::ffi::c_uint;
pub const SSLMODE_DISABLED: SSLMode = 0;
pub const SSLMODE_ALLOW: SSLMode = 1;
pub const SSLMODE_PREFER: SSLMode = 2;
pub const SSLMODE_REQUIRE: SSLMode = 3;
pub const SSLMODE_VERIFY_CA: SSLMode = 4;
pub const SSLMODE_VERIFY_FULL: SSLMode = 5;

pub type PacketCallbackFlag = ::core::ffi::c_uint;
pub const CB_NONE: PacketCallbackFlag = 0;
pub const CB_WANT_COMPLETE_PACKET: PacketCallbackFlag = 1;
pub const CB_HANDLE_COMPLETE_PACKET: PacketCallbackFlag = 2;

pub type LoadBalanceHosts = ::core::ffi::c_uint;
pub const LOAD_BALANCE_HOSTS_DISABLE: LoadBalanceHosts = 0;
pub const LOAD_BALANCE_HOSTS_ROUND_ROBIN: LoadBalanceHosts = 1;

pub type ReplicationType = ::core::ffi::c_uint;
pub const REPLICATION_NONE: ReplicationType = 0;
pub const REPLICATION_LOGICAL: ReplicationType = 1;
pub const REPLICATION_PHYSICAL: ReplicationType = 2;

pub type auth_type = ::core::ffi::c_uint;
pub const AUTH_TYPE_ANY: auth_type = 0;
pub const AUTH_TYPE_TRUST: auth_type = 1;
pub const AUTH_TYPE_PLAIN: auth_type = 2;
pub const AUTH_TYPE_MD5: auth_type = 3;
pub const AUTH_TYPE_CERT: auth_type = 4;
pub const AUTH_TYPE_HBA: auth_type = 5;
pub const AUTH_TYPE_LDAP: auth_type = 6;
pub const AUTH_TYPE_PAM: auth_type = 7;
pub const AUTH_TYPE_SCRAM_SHA_256: auth_type = 8;
pub const AUTH_TYPE_PEER: auth_type = 9;
pub const AUTH_TYPE_REJECT: auth_type = 10;

pub type SBufEvent = ::core::ffi::c_uint;
pub const SBUF_EV_READ: SBufEvent = 0;
pub const SBUF_EV_RECV_FAILED: SBufEvent = 1;
pub const SBUF_EV_SEND_FAILED: SBufEvent = 2;
pub const SBUF_EV_CONNECT_FAILED: SBufEvent = 3;
pub const SBUF_EV_CONNECT_OK: SBufEvent = 4;
pub const SBUF_EV_FLUSH: SBufEvent = 5;
pub const SBUF_EV_PKT_CALLBACK: SBufEvent = 6;
pub const SBUF_EV_TLS_READY: SBufEvent = 7;

pub type TLSState = ::core::ffi::c_uint;
pub const SBUF_TLS_NONE: TLSState = 0;
pub const SBUF_TLS_DO_HANDSHAKE: TLSState = 1;
pub const SBUF_TLS_OK: TLSState = 2;

pub type WaitType = ::core::ffi::c_uint;
pub const W_NONE: WaitType = 0;
pub const W_CONNECT: WaitType = 1;
pub const W_RECV: WaitType = 2;
pub const W_SEND: WaitType = 3;

pub type pg_cryptohash_type = ::core::ffi::c_uint;
pub const PG_SHA224: pg_cryptohash_type = 0;
pub const PG_SHA256: pg_cryptohash_type = 1;
pub const PG_SHA384: pg_cryptohash_type = 2;
pub const PG_SHA512: pg_cryptohash_type = 3;

pub type LogLevel = ::core::ffi::c_uint;
pub const LOG_DEBUG: LogLevel = 0;
pub const LOG_INFO: LogLevel = 1;
pub const LOG_LOG: LogLevel = 2;
pub const LOG_WARNING: LogLevel = 3;
pub const LOG_ERROR: LogLevel = 4;
pub const LOG_FATAL: LogLevel = 5;
pub const LOG_NOISE: LogLevel = 6;

// Pool mode constants
pub const POOL_SESSION: ::core::ffi::c_int = 0;
pub const POOL_TX: ::core::ffi::c_int = 1;
pub const POOL_STMT: ::core::ffi::c_int = 2;
pub const POOL_INHERIT: ::core::ffi::c_int = -1;

// =============================================================================
// pgbouncer core structs
// =============================================================================

#[derive(Copy, Clone)]
#[repr(C)]
pub struct iobuf {
    pub done_pos: ::core::ffi::c_uint,
    pub parse_pos: ::core::ffi::c_uint,
    pub recv_pos: ::core::ffi::c_uint,
    pub buf: [uint8_t; 0],
}

pub type IOBuf = iobuf;

pub type sbuf_cb_t = Option<unsafe extern "C" fn(*mut SBuf, SBufEvent, *mut MBuf) -> bool>;

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
pub struct PktHdr {
    pub type_0: ::core::ffi::c_uint,
    pub len: ::core::ffi::c_uint,
    pub data: MBuf,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VarCache {
    pub var_list: *mut *mut PStr,
}

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PgServerPreparedStatement {
    pub query_id: uint64_t,
    pub hh: UT_hash_handle,
    pub ps: *mut PgPreparedStatement,
}

// =============================================================================
// Extern type declarations (opaque types)
// =============================================================================

extern "C" {
    pub type __sFILEX;
    pub type tls;
    pub type tls_config;
    pub type event_base;
    pub type StrPool;
    pub type DNSToken;
    pub type DNSContext;
}

// =============================================================================
// Extern functions - libc
// =============================================================================

extern "C" {
    pub static mut __stderrp: *mut FILE;
    pub static mut __stdoutp: *mut FILE;
    pub static mut __stdinp: *mut FILE;

    pub fn __error() -> *mut ::core::ffi::c_int;

    pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    pub fn calloc(__count: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    pub fn free(_: *mut ::core::ffi::c_void);
    pub fn exit(_: ::core::ffi::c_int) -> !;
    pub fn abort() -> !;

    pub fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    pub fn memmove(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    pub fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    pub fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    pub fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;

    pub fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    pub fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn strncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    pub fn strcasecmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn strncasecmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    pub fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    pub fn strncpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    pub fn strcat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    pub fn strncat(
        __s1: *mut ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    pub fn strchr(_: *const ::core::ffi::c_char, _: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    pub fn strrchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    pub fn strstr(
        __big: *const ::core::ffi::c_char,
        __little: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    pub fn strdup(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn strerror(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;

    pub fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn snprintf(
        __str: *mut ::core::ffi::c_char,
        __size: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    pub fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;

    pub fn fopen(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char) -> *mut FILE;
    pub fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    pub fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __nitems: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    pub fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __nitems: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    pub fn fgets(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    pub fn feof(_: *mut FILE) -> ::core::ffi::c_int;
    pub fn ferror(_: *mut FILE) -> ::core::ffi::c_int;
    pub fn fflush(_: *mut FILE) -> ::core::ffi::c_int;

    pub fn atoi(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn atol(_: *const ::core::ffi::c_char) -> ::core::ffi::c_long;
    pub fn strtol(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    pub fn strtoul(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;

    pub fn getrlimit(_: ::core::ffi::c_int, _: *mut rlimit) -> ::core::ffi::c_int;
    pub fn setrlimit(_: ::core::ffi::c_int, _: *const rlimit) -> ::core::ffi::c_int;

    pub fn close(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn read(_: ::core::ffi::c_int, _: *mut ::core::ffi::c_void, _: size_t) -> ssize_t;
    pub fn write(_: ::core::ffi::c_int, _: *const ::core::ffi::c_void, _: size_t) -> ssize_t;

    pub fn socket(
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn connect(_: ::core::ffi::c_int, _: *const sockaddr, _: socklen_t) -> ::core::ffi::c_int;
    pub fn bind(_: ::core::ffi::c_int, _: *const sockaddr, _: socklen_t) -> ::core::ffi::c_int;
    pub fn listen(_: ::core::ffi::c_int, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn accept(_: ::core::ffi::c_int, _: *mut sockaddr, _: *mut socklen_t)
        -> ::core::ffi::c_int;
    pub fn send(
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_void,
        _: size_t,
        _: ::core::ffi::c_int,
    ) -> ssize_t;
    pub fn recv(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
        _: size_t,
        _: ::core::ffi::c_int,
    ) -> ssize_t;
    pub fn getsockopt(
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
        _: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    pub fn setsockopt(
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_void,
        _: socklen_t,
    ) -> ::core::ffi::c_int;

    pub fn getpid() -> pid_t;
    pub fn getuid() -> uid_t;
    pub fn getpeereid(
        _: ::core::ffi::c_int,
        _: *mut uid_t,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

// =============================================================================
// Extern functions - libevent
// =============================================================================

extern "C" {
    pub fn event_base_new() -> *mut event_base;
    pub fn event_base_free(_: *mut event_base);
    pub fn event_base_get_method(_: *const event_base) -> *const ::core::ffi::c_char;
    pub fn event_base_loop(_: *mut event_base, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn event_base_loopbreak(_: *mut event_base) -> ::core::ffi::c_int;
    pub fn event_base_loopexit(_: *mut event_base, _: *const timeval) -> ::core::ffi::c_int;

    pub fn event_assign(
        _: *mut event,
        _: *mut event_base,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_short,
        _: event_callback_fn,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn event_add(ev: *mut event, timeout: *const timeval) -> ::core::ffi::c_int;
    pub fn event_del(ev: *mut event) -> ::core::ffi::c_int;
    pub fn event_get_version() -> *const ::core::ffi::c_char;
}

// =============================================================================
// pgbouncer-specific forward declarations and types
// These are declared but not fully defined here to break circular dependencies
// =============================================================================

// Forward declarations for complex types that have bitfields
// (PgSocket, PgPool, etc. need c2rust_bitfields and are defined in their modules)
