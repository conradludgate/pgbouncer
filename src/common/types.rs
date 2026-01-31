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

use c2rust_bitfields::BitfieldStruct;

// =============================================================================
// Primitive C types (re-exported from libc)
// =============================================================================

// Internal darwin types (for compatibility with c2rust output)
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;

// Darwin-specific type aliases
pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type __darwin_ptrdiff_t = isize;
pub type __darwin_time_t = ::core::ffi::c_long;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_wchar_t = libc::wchar_t;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_ct_rune_t = ::core::ffi::c_int;
pub type __builtin_va_list = *mut ::core::ffi::c_char;
pub type __darwin_va_list = __builtin_va_list;

// Darwin system types (consolidating sys__types_h modules)
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_sigset_t = __uint32_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
pub type __darwin_useconds_t = __uint32_t;

// Standard C types - use libc types
pub type uintptr_t = libc::uintptr_t;
pub type in_addr_t = libc::in_addr_t;
pub type in_port_t = libc::in_port_t;
pub type pid_t = libc::pid_t;
pub type uid_t = libc::uid_t;
pub type gid_t = libc::gid_t;
pub type size_t = libc::size_t;
pub type ssize_t = libc::ssize_t;
pub type time_t = libc::time_t;
pub type sigset_t = libc::sigset_t;
pub type ptrdiff_t = libc::ptrdiff_t;
pub type socklen_t = libc::socklen_t;
pub type sa_family_t = libc::sa_family_t;
pub type rlim_t = libc::rlim_t;
pub type fpos_t = libc::fpos_t;

// File system types
pub type blkcnt_t = libc::blkcnt_t;
pub type blksize_t = libc::blksize_t;
pub type dev_t = libc::dev_t;
pub type ino_t = libc::ino_t;
pub type ino64_t = __darwin_ino64_t;  // Not in libc on macOS
pub type mode_t = libc::mode_t;
pub type nlink_t = libc::nlink_t;
pub type off_t = libc::off_t;
pub type useconds_t = libc::useconds_t;

// Fixed-width integers - use Rust primitive types directly
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type u_int32_t = ::core::ffi::c_uint;

// Variadic argument type
pub type va_list = __darwin_va_list;

// NULL constant
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut();

// =============================================================================
// libc constants (re-exported from libc crate)
// =============================================================================

pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ENOMEM: ::core::ffi::c_int = libc::ENOMEM;
pub const EINTR: ::core::ffi::c_int = libc::EINTR;
pub const EAGAIN: ::core::ffi::c_int = libc::EAGAIN;
pub const EWOULDBLOCK: ::core::ffi::c_int = libc::EWOULDBLOCK;
pub const EINPROGRESS: ::core::ffi::c_int = libc::EINPROGRESS;

// Socket constants
pub const AF_UNIX: ::core::ffi::c_int = libc::AF_UNIX;
pub const AF_INET: ::core::ffi::c_int = libc::AF_INET;
pub const AF_INET6: ::core::ffi::c_int = libc::AF_INET6;
pub const SOCK_STREAM: ::core::ffi::c_int = libc::SOCK_STREAM;
pub const SOCK_DGRAM: ::core::ffi::c_int = libc::SOCK_DGRAM;

// Resource limits
pub const RLIMIT_NOFILE: ::core::ffi::c_int = libc::RLIMIT_NOFILE as ::core::ffi::c_int;

// Boolean constants (for c2rust compatibility)
pub const true_0: ::core::ffi::c_int = 1;
pub const false_0: ::core::ffi::c_int = 0;

// =============================================================================
// Struct definitions - libc types
// =============================================================================

pub type timeval = libc::timeval;

pub type timespec = libc::timespec;

pub type iovec = libc::iovec;

pub type rlimit = libc::rlimit;

// Socket address structures - use libc types
pub type sockaddr = libc::sockaddr;
pub type sockaddr_un = libc::sockaddr_un;
pub type in_addr = libc::in_addr;
pub type sockaddr_in = libc::sockaddr_in;
pub type in6_addr = libc::in6_addr;
pub type sockaddr_in6 = libc::sockaddr_in6;

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

// Use libc::FILE for compatibility with libc functions (fopen, fclose, etc.)
pub type FILE = libc::FILE;

// =============================================================================
// usual library types
// =============================================================================

pub type usec_t = uint64_t;
pub const USEC: usec_t = 1000000;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct List {
    pub next: *mut List,
    pub prev: *mut List,
}

#[inline]
pub unsafe extern "C" fn list_init(list: *mut List) {
    (*list).prev = list;
    (*list).next = (*list).prev;
}

#[inline]
pub unsafe extern "C" fn list_empty(list: *const List) -> ::core::ffi::c_int {
    std::ptr::eq((*list).next, list) as ::core::ffi::c_int
}

#[inline]
pub unsafe extern "C" fn list_prepend(list: *mut List, item: *mut List) -> *mut List {
    (*item).next = (*list).next;
    (*item).prev = list;
    (*(*list).next).prev = item;
    (*list).next = item;
    item
}

#[inline]
pub unsafe extern "C" fn list_append(list: *mut List, item: *mut List) -> *mut List {
    (*item).next = list;
    (*item).prev = (*list).prev;
    (*(*list).prev).next = item;
    (*list).prev = item;
    item
}

#[inline]
pub unsafe extern "C" fn list_del(item: *mut List) -> *mut List {
    (*(*item).prev).next = (*item).next;
    (*(*item).next).prev = (*item).prev;
    (*item).prev = item;
    (*item).next = (*item).prev;
    item
}

#[inline]
pub unsafe extern "C" fn list_pop(list: *mut List) -> *mut List {
    if list_empty(list) != 0 {
        return ::core::ptr::null_mut::<List>();
    }
    list_del((*list).next)
}

#[inline]
pub unsafe extern "C" fn list_first(list: *const List) -> *mut List {
    if list_empty(list) != 0 {
        return ::core::ptr::null_mut::<List>();
    }
    (*list).next
}

#[inline]
pub unsafe extern "C" fn list_last(list: *const List) -> *mut List {
    if list_empty(list) != 0 {
        return ::core::ptr::null_mut::<List>();
    }
    (*list).prev
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
pub unsafe extern "C" fn statlist_init(list: *mut StatList, _name: *const ::core::ffi::c_char) {
    list_init(&raw mut (*list).head);
    (*list).cur_count = 0;
}

#[inline]
pub unsafe extern "C" fn statlist_count(list: *const StatList) -> ::core::ffi::c_int {
    (*list).cur_count
}

#[inline]
pub unsafe extern "C" fn statlist_empty(list: *const StatList) -> bool {
    list_empty(&raw const (*list).head) != 0
}

#[inline]
pub unsafe extern "C" fn statlist_prepend(list: *mut StatList, item: *mut List) {
    list_prepend(&raw mut (*list).head, item);
    (*list).cur_count += 1;
}

#[inline]
pub unsafe extern "C" fn statlist_append(list: *mut StatList, item: *mut List) {
    list_append(&raw mut (*list).head, item);
    (*list).cur_count += 1;
}

#[inline]
pub unsafe extern "C" fn statlist_remove(list: *mut StatList, item: *mut List) {
    list_del(item);
    (*list).cur_count -= 1;
}

#[inline]
pub unsafe extern "C" fn statlist_pop(list: *mut StatList) -> *mut List {
    let item = list_pop(&raw mut (*list).head);
    if !item.is_null() {
        (*list).cur_count -= 1;
    }
    item
}

#[inline]
pub unsafe extern "C" fn statlist_first(list: *const StatList) -> *mut List {
    list_first(&raw const (*list).head)
}

#[inline]
pub unsafe extern "C" fn statlist_last(list: *const StatList) -> *mut List {
    list_last(&raw const (*list).head)
}

#[inline]
pub unsafe extern "C" fn statlist_put_before(list: *mut StatList, item: *mut List, pos: *mut List) {
    list_append(pos, item);
    (*list).cur_count += 1;
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

pub type AATreeWalkType = ::core::ffi::c_uint;
pub const AA_WALK_IN_ORDER: AATreeWalkType = 0;
pub const AA_WALK_PRE_ORDER: AATreeWalkType = 1;
pub const AA_WALK_POST_ORDER: AATreeWalkType = 2;

extern "C" {
    pub fn aatree_init(tree: *mut AATree, cmpfn: aatree_cmp_f, release_cb: aatree_walker_f);
    pub fn aatree_destroy(tree: *mut AATree);
    pub fn aatree_search(tree: *mut AATree, value: uintptr_t) -> *mut AANode;
    pub fn aatree_insert(tree: *mut AATree, value: uintptr_t, node: *mut AANode);
    pub fn aatree_walk(
        tree: *mut AATree,
        wtype: AATreeWalkType,
        walker: aatree_walker_f,
        arg: *mut ::core::ffi::c_void,
    );
}

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
// C2RustUnnamed type aliases for libevent structs
// These map the anonymous types from c2rust to proper named types
// =============================================================================
pub type C2RustUnnamed_0 = event_union;
pub type C2RustUnnamed_1 = event_signal;
pub type C2RustUnnamed_2 = event_signal_next;
pub type C2RustUnnamed_3 = event_io;
pub type C2RustUnnamed_4 = event_io_next;
pub type C2RustUnnamed_5 = event_timeout_pos;
pub type C2RustUnnamed_6 = event_next_with_common_timeout;
pub type C2RustUnnamed_7 = event_callback_union;
pub type C2RustUnnamed_8 = event_callback_active_next;

// event_callback is an alias for event_callback_struct (c2rust used shorter name)
pub type event_callback = event_callback_struct;

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

// IOBuf inline functions
#[inline]
pub unsafe fn iobuf_empty(io: *const IOBuf) -> bool {
    io.is_null() || (*io).done_pos == (*io).recv_pos
}

#[inline]
pub unsafe fn iobuf_amount_pending(buf: *const IOBuf) -> ::core::ffi::c_uint {
    (*buf).parse_pos.wrapping_sub((*buf).done_pos)
}

#[inline]
pub unsafe fn iobuf_amount_parse(buf: *const IOBuf) -> ::core::ffi::c_uint {
    (*buf).recv_pos.wrapping_sub((*buf).parse_pos)
}

pub type sbuf_cb_t = Option<unsafe extern "C" fn(*mut SBuf, SBufEvent, *mut MBuf) -> bool>;

// Slab allocator function pointer types
pub type slab_init_fn = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type slab_stat_fn = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_char,
        ::core::ffi::c_uint,
        ::core::ffi::c_uint,
        ::core::ffi::c_uint,
    ) -> (),
>;

// Pooler callback type
pub type pooler_cb = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int, *const PgAddr) -> bool,
>;

// String callback type
pub type str_cb =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char) -> bool>;

// SCRAM password types
pub type PasswordType = ::core::ffi::c_uint;
pub const PASSWORD_TYPE_PLAINTEXT: PasswordType = 0;
pub const PASSWORD_TYPE_MD5: PasswordType = 1;
pub const PASSWORD_TYPE_SCRAM_SHA_256: PasswordType = 2;

// Postgres compatibility constants
pub const HIGHBIT: ::core::ffi::c_int = 0x80;
pub const MaxAllocSize: size_t = 0x3fffffff;

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

// PktHdr constants
pub const OLD_HEADER_LEN: ::core::ffi::c_int = 8;
pub const NEW_HEADER_LEN: ::core::ffi::c_int = 5;

// PktHdr inline functions
#[inline]
pub unsafe fn free_header(pkt: *mut PktHdr) {
    crate::lib::usual::mbuf::mbuf_free(&raw mut (*pkt).data);
    (*pkt).type_0 = 0;
    (*pkt).len = 0;
}

#[inline]
pub unsafe fn incomplete_pkt(pkt: *const PktHdr) -> bool {
    crate::lib::usual::mbuf::mbuf_written(&raw const (*pkt).data) != (*pkt).len
}

#[inline]
pub unsafe fn incomplete_header(data: *const MBuf) -> bool {
    let avail = crate::lib::usual::mbuf::mbuf_avail_for_read(data) as u32;
    if avail >= OLD_HEADER_LEN as u32 {
        return false;
    }
    if avail < NEW_HEADER_LEN as u32 {
        return true;
    }
    *(*data).data.offset((*data).read_pos as isize) as ::core::ffi::c_int == 0
}

#[inline]
pub unsafe fn pkt_rewind_v3(pkt: *mut PktHdr) {
    (*pkt).data.read_pos = NEW_HEADER_LEN as u32;
}

#[inline]
pub unsafe fn pkt_rewind_v2(pkt: *mut PktHdr) {
    (*pkt).data.read_pos = OLD_HEADER_LEN as u32;
}

#[inline]
pub unsafe fn pkt_desc(pkt: *const PktHdr) -> ::core::ffi::c_char {
    (if (*pkt).type_0 > 256 {
        '!' as u32
    } else {
        (*pkt).type_0
    }) as ::core::ffi::c_char
}

// External functions that work with PktHdr (declared here, defined in proto.rs)
extern "C" {
    pub fn get_header(data: *mut MBuf, pkt: *mut PktHdr) -> bool;
    pub fn log_server_error(note: *const ::core::ffi::c_char, pkt: *mut PktHdr);
}

// VarCache types and constants
pub type VarCacheIdx = u32;
pub const VDateStyle: VarCacheIdx = 0;
pub const VClientEncoding: VarCacheIdx = 1;
pub const VTimeZone: VarCacheIdx = 2;
pub const VStdStr: VarCacheIdx = 3;
pub const VAppName: VarCacheIdx = 4;
pub const NumVars: VarCacheIdx = 5;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VarCache {
    pub var_list: *mut *mut PStr,
}

// uthash constants
pub const HASH_INITIAL_NUM_BUCKETS: u32 = 32;
pub const HASH_INITIAL_NUM_BUCKETS_LOG2: u32 = 5;
pub const HASH_BKT_CAPACITY_THRESH: u32 = 10;
pub const HASH_SIGNATURE: u32 = 0xa0111fe1;

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
// netdb_h types (DNS/network info)
// =============================================================================

pub const AI_PASSIVE: ::core::ffi::c_int = 0x1;

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

extern "C" {
    pub fn freeaddrinfo(_: *mut addrinfo);
    pub fn gai_strerror(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    pub fn getaddrinfo(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *const addrinfo,
        _: *mut *mut addrinfo,
    ) -> ::core::ffi::c_int;
}

// =============================================================================
// dnslookup_h type aliases
// =============================================================================

pub type adns_callback_f = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const sockaddr, ::core::ffi::c_int) -> (),
>;

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
    pub type Slab;
}

// =============================================================================
// Re-exports from libc crate
// =============================================================================

extern "C" {
}

// errno access (macOS-specific)
pub use libc::__error;

// Memory allocation
pub use libc::calloc;
pub use libc::free;
pub use libc::malloc;
pub use libc::realloc;

// Process control
pub use libc::abort;
pub use libc::exit;

// Memory operations
pub use libc::memchr;
pub use libc::memcmp;
pub use libc::memcpy;
pub use libc::memmove;
pub use libc::memset;

// String operations
pub use libc::strcasecmp;
pub use libc::strcat;
pub use libc::strchr;
pub use libc::strcmp;
pub use libc::strcpy;
pub use libc::strdup;
pub use libc::strerror;
pub use libc::strlen;
pub use libc::strncasecmp;
pub use libc::strncat;
pub use libc::strncmp;
pub use libc::strncpy;
pub use libc::strrchr;
pub use libc::strstr;

// Formatted I/O
pub use libc::fprintf;
pub use libc::printf;
pub use libc::snprintf;
pub use libc::sprintf;
pub use libc::sscanf;

// File operations
pub use libc::fclose;
pub use libc::feof;
pub use libc::ferror;
pub use libc::fflush;
pub use libc::fgets;
pub use libc::fopen;
pub use libc::fread;
pub use libc::fwrite;

// Number parsing
pub use libc::atoi;
pub use libc::atol;
pub use libc::strtol;
pub use libc::strtoul;

// Resource limits
pub use libc::getrlimit;
pub use libc::setrlimit;

// Basic I/O
pub use libc::close;
pub use libc::read;
pub use libc::write;

// Socket operations
pub use libc::accept;
pub use libc::bind;
pub use libc::connect;
pub use libc::getsockopt;
pub use libc::listen;
pub use libc::recv;
pub use libc::send;
pub use libc::setsockopt;
pub use libc::socket;

// Process info
pub use libc::getpid;
pub use libc::getuid;

// macOS-specific peer credentials
extern "C" {
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
// Crypto types - SHA2, cryptohash, HMAC
// =============================================================================

// SHA context structures
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pg_sha256_ctx {
    pub state: [uint32_t; 8],
    pub bitcount: uint64_t,
    pub buffer: [uint8_t; 64],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct pg_sha512_ctx {
    pub state: [uint64_t; 8],
    pub bitcount: [uint64_t; 2],
    pub buffer: [uint8_t; 128],
}

pub type pg_sha224_ctx = pg_sha256_ctx;
pub type pg_sha384_ctx = pg_sha512_ctx;

// SHA constants
pub const PG_SHA224_BLOCK_LENGTH: ::core::ffi::c_int = 64;
pub const PG_SHA224_DIGEST_LENGTH: ::core::ffi::c_int = 28;
pub const PG_SHA256_BLOCK_LENGTH: ::core::ffi::c_int = 64;
pub const PG_SHA256_DIGEST_LENGTH: ::core::ffi::c_int = 32;
pub const PG_SHA384_BLOCK_LENGTH: ::core::ffi::c_int = 128;
pub const PG_SHA384_DIGEST_LENGTH: ::core::ffi::c_int = 48;
pub const PG_SHA512_BLOCK_LENGTH: ::core::ffi::c_int = 128;
pub const PG_SHA512_DIGEST_LENGTH: ::core::ffi::c_int = 64;

// SASL prep return codes
pub type pg_saslprep_rc = ::core::ffi::c_int;
pub const SASLPREP_SUCCESS: pg_saslprep_rc = 0;
pub const SASLPREP_OOM: pg_saslprep_rc = -1;
pub const SASLPREP_INVALID_UTF8: pg_saslprep_rc = -2;
pub const SASLPREP_PROHIBITED: pg_saslprep_rc = -3;

// Wide char type for unicode
pub type pg_wchar = ::core::ffi::c_uint;

// =============================================================================
// Extern functions - Crypto
// =============================================================================

extern "C" {
    pub fn usual_explicit_bzero(buf: *mut ::core::ffi::c_void, len: size_t);
}

// =============================================================================
// pgbouncer-specific types
// =============================================================================

/// Statistics counters for a connection pool
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct PgStats {
    pub server_assignment_count: uint64_t,
    pub xact_count: uint64_t,
    pub query_count: uint64_t,
    pub server_bytes: uint64_t,
    pub client_bytes: uint64_t,
    pub xact_time: usec_t,
    pub query_time: usec_t,
    pub wait_time: usec_t,
    pub ps_server_parse_count: uint64_t,
    pub ps_client_parse_count: uint64_t,
    pub ps_bind_count: uint64_t,
}

// =============================================================================
// tls_h (merged from all files)
// =============================================================================

pub const TLS_PROTOCOLS_ALL: ::core::ffi::c_int = TLS_PROTOCOL_TLSv1;
pub const TLS_PROTOCOL_TLSv1: ::core::ffi::c_int =
        TLS_PROTOCOL_TLSv1_0 | TLS_PROTOCOL_TLSv1_1 | TLS_PROTOCOL_TLSv1_2 | TLS_PROTOCOL_TLSv1_3;
pub const TLS_PROTOCOL_TLSv1_0: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const TLS_PROTOCOL_TLSv1_1: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const TLS_PROTOCOL_TLSv1_2: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const TLS_PROTOCOL_TLSv1_3: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const TLS_WANT_POLLIN: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const TLS_WANT_POLLOUT: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);

extern "C" {
    pub fn tls_accept_fds(
                _ctx: *mut tls,
                _cctx: *mut *mut tls,
                _fd_read: ::core::ffi::c_int,
                _fd_write: ::core::ffi::c_int,
            ) -> ::core::ffi::c_int;
    pub fn tls_backend_version() -> *const ::core::ffi::c_char;
    pub fn tls_client() -> *mut tls;
    pub fn tls_close(_ctx: *mut tls) -> ::core::ffi::c_int;
    pub fn tls_config_equal(
                server_connect_conf_left: *mut tls_config,
                server_connect_conf_right: *mut tls_config,
            ) -> bool;
    pub fn tls_config_free(_config: *mut tls_config);
    pub fn tls_config_insecure_noverifycert(_config: *mut tls_config);
    pub fn tls_config_insecure_noverifyname(_config: *mut tls_config);
    pub fn tls_config_new() -> *mut tls_config;
    pub fn tls_config_parse_protocols(
                _protocols: *mut uint32_t,
                _protostr: *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int;
    pub fn tls_config_set_ca_file(
                _config: *mut tls_config,
                _ca_file: *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int;
    pub fn tls_config_set_cert_file(
                _config: *mut tls_config,
                _cert_file: *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int;
    pub fn tls_config_set_ciphers(
                _config: *mut tls_config,
                _ciphers: *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int;
    pub fn tls_config_set_ciphers_v13(
                _config: *mut tls_config,
                _ciphers: *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int;
    pub fn tls_config_set_dheparams(
                _config: *mut tls_config,
                _params: *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int;
    pub fn tls_config_set_ecdhecurve(
                _config: *mut tls_config,
                _name: *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int;
    pub fn tls_config_set_key_file(
                _config: *mut tls_config,
                _key_file: *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int;
    pub fn tls_config_set_protocols(_config: *mut tls_config, _protocols: uint32_t);
    pub fn tls_config_verify(_config: *mut tls_config);
    pub fn tls_config_verify_client(_config: *mut tls_config);
    pub fn tls_config_verify_client_optional(_config: *mut tls_config);
    pub fn tls_configure(_ctx: *mut tls, _config: *mut tls_config) -> ::core::ffi::c_int;
    pub fn tls_connect_fds(
                _ctx: *mut tls,
                _fd_read: ::core::ffi::c_int,
                _fd_write: ::core::ffi::c_int,
                _servername: *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int;
    pub fn tls_error(_ctx: *mut tls) -> *const ::core::ffi::c_char;
    pub fn tls_get_connection_info(
                ctx: *mut tls,
                buf: *mut ::core::ffi::c_char,
                buflen: size_t,
            ) -> ssize_t;
    pub fn tls_handshake(_ctx: *mut tls) -> ::core::ffi::c_int;
    pub fn tls_init() -> ::core::ffi::c_int;
    pub fn tls_peer_cert_contains_name(
                _ctx: *mut tls,
                _name: *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int;
    pub fn tls_peer_cert_provided(_ctx: *mut tls) -> ::core::ffi::c_int;
    pub fn tls_peer_cert_subject(_ctx: *mut tls) -> *const ::core::ffi::c_char;
    pub fn tls_read(_ctx: *mut tls, _buf: *mut ::core::ffi::c_void, _buflen: size_t)
                -> ssize_t;
    pub fn tls_server() -> *mut tls;
    pub fn tls_write(
                _ctx: *mut tls,
                _buf: *const ::core::ffi::c_void,
                _buflen: size_t,
            ) -> ssize_t;
    pub fn usual_tls_free(_ctx: *mut tls);
}

// =============================================================================
// logging_h (merged from all files)
// =============================================================================

pub type logging_prefix_fn_t = Option<
        unsafe extern "C" fn(
            LogLevel,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int,
    >;

pub const LG_DEBUG: LogLevel = 5;
pub const LG_ERROR: LogLevel = 1;
pub const LG_FATAL: LogLevel = 0;
pub const LG_INFO: LogLevel = 4;
pub const LG_NOISE: LogLevel = 6;
pub const LG_STATS: LogLevel = 3;
pub const LG_WARNING: LogLevel = 2;

extern "C" {
    pub static mut cf_logfile: *const ::core::ffi::c_char;
    pub static mut cf_quiet: ::core::ffi::c_int;
    pub static mut cf_syslog: ::core::ffi::c_int;
    pub static mut cf_syslog_facility: *const ::core::ffi::c_char;
    pub static mut cf_syslog_ident: *const ::core::ffi::c_char;
    pub static mut cf_verbose: ::core::ffi::c_int;
    pub static mut logging_prefix_cb: logging_prefix_fn_t;
    pub fn log_fatal(
                file: *const ::core::ffi::c_char,
                line: ::core::ffi::c_int,
                func: *const ::core::ffi::c_char,
                show_perror: bool,
                ctx: *mut ::core::ffi::c_void,
                s: *const ::core::ffi::c_char,
                ...
            );
    pub fn log_generic(
                level: LogLevel,
                ctx: *mut ::core::ffi::c_void,
                s: *const ::core::ffi::c_char,
                ...
            );
    pub fn reset_logging();
}

// =============================================================================
// socket_h (merged from all files)
// =============================================================================

pub const AF_UNSPEC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SCM_RIGHTS: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SOL_SOCKET: ::core::ffi::c_int = 0xffff as ::core::ffi::c_int;
pub const SO_ERROR: ::core::ffi::c_int = 0x1007 as ::core::ffi::c_int;
pub const SO_RCVBUF: ::core::ffi::c_int = 0x1002 as ::core::ffi::c_int;
pub const SO_REUSEADDR: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SO_REUSEPORT: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const SO_SNDBUF: ::core::ffi::c_int = 0x1001 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
    #[repr(C)]

    pub struct cmsghdr {
        pub cmsg_len: socklen_t,
        pub cmsg_level: ::core::ffi::c_int,
        pub cmsg_type: ::core::ffi::c_int,
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

    pub struct sockaddr_storage {
        pub ss_len: __uint8_t,
        pub ss_family: sa_family_t,
        pub __ss_pad1: [::core::ffi::c_char; 6],
        pub __ss_align: __int64_t,
        pub __ss_pad2: [::core::ffi::c_char; 112],
    }

extern "C" {
    pub fn getpeername(
                _: ::core::ffi::c_int,
                _: *mut sockaddr,
                _: *mut socklen_t,
            ) -> ::core::ffi::c_int;
    pub fn getsockname(
                _: ::core::ffi::c_int,
                _: *mut sockaddr,
                _: *mut socklen_t,
            ) -> ::core::ffi::c_int;
}

// =============================================================================
// in_h (merged from all files)
// =============================================================================

pub const IPPROTO_IPV6: ::core::ffi::c_int = 41 as ::core::ffi::c_int;
pub const IPPROTO_TCP: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

// =============================================================================
// _string_h (merged from all files)
// =============================================================================

extern "C" {
    pub fn strlcat(
                __dst: *mut ::core::ffi::c_char,
                __source: *const ::core::ffi::c_char,
                __size: size_t,
            ) -> ::core::ffi::c_ulong;
    pub fn strlcpy(
                __dst: *mut ::core::ffi::c_char,
                __source: *const ::core::ffi::c_char,
                __size: size_t,
            ) -> ::core::ffi::c_ulong;
    pub fn strnlen(__s1: *const ::core::ffi::c_char, __n: size_t) -> size_t;
    pub fn strspn(
                __s: *const ::core::ffi::c_char,
                __charset: *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_ulong;
    pub fn strtok(
                __str: *mut ::core::ffi::c_char,
                __sep: *const ::core::ffi::c_char,
            ) -> *mut ::core::ffi::c_char;
}

// =============================================================================
// _stdlib_h (merged from all files)
// =============================================================================

extern "C" {
    pub fn atexit(_: Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int;
    pub fn atoll(_: *const ::core::ffi::c_char) -> ::core::ffi::c_longlong;
    pub fn bsearch(
                __key: *const ::core::ffi::c_void,
                __base: *const ::core::ffi::c_void,
                __nel: size_t,
                __width: size_t,
                __compar: Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
                >,
            ) -> *mut ::core::ffi::c_void;
    pub fn getenv(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn setprogname(_: *const ::core::ffi::c_char);
    pub fn srandom(_: ::core::ffi::c_uint);
    pub fn strtonum(
                __numstr: *const ::core::ffi::c_char,
                __minval: ::core::ffi::c_longlong,
                __maxval: ::core::ffi::c_longlong,
                __errstrp: *mut *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_longlong;
}

// =============================================================================
// _malloc_h (merged from all files)
// =============================================================================

// =============================================================================
// safeio_h (merged from all files)
// =============================================================================

extern "C" {
    pub fn safe_accept(
                fd: ::core::ffi::c_int,
                sa: *mut sockaddr,
                sa_len: *mut socklen_t,
            ) -> ::core::ffi::c_int;
    pub fn safe_close(fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn safe_connect(
                fd: ::core::ffi::c_int,
                sa: *const sockaddr,
                sa_len: socklen_t,
            ) -> ::core::ffi::c_int;
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
    pub fn safe_send(
                fd: ::core::ffi::c_int,
                buf: *const ::core::ffi::c_void,
                len: size_t,
                flags: ::core::ffi::c_int,
            ) -> ssize_t;
    pub fn safe_sendmsg(
                fd: ::core::ffi::c_int,
                msg: *const msghdr,
                flags: ::core::ffi::c_int,
            ) -> ssize_t;
    pub fn safe_write(
                fd: ::core::ffi::c_int,
                buf: *const ::core::ffi::c_void,
                len: size_t,
            ) -> ssize_t;
}

// =============================================================================
// usual_socket_h (merged from all files)
// =============================================================================

extern "C" {
    pub fn sa2str(
                sa: *const sockaddr,
                buf: *mut ::core::ffi::c_char,
                buflen: size_t,
            ) -> *const ::core::ffi::c_char;
    pub fn socket_set_keepalive(
                fd: ::core::ffi::c_int,
                onoff: ::core::ffi::c_int,
                keepidle: ::core::ffi::c_int,
                keepintvl: ::core::ffi::c_int,
                keepcnt: ::core::ffi::c_int,
            ) -> bool;
    pub fn socket_set_nonblocking(sock: ::core::ffi::c_int, non_block: bool) -> bool;
    pub fn socket_setup(sock: ::core::ffi::c_int, non_block: bool) -> bool;
    pub fn usual_getpeercreds(
                fd: ::core::ffi::c_int,
                uid_p: *mut uid_t,
                gid_p: *mut gid_t,
                pid_p: *mut pid_t,
            ) -> ::core::ffi::c_int;
}

// =============================================================================
// in6_h (merged from all files)
// =============================================================================

pub const IPV6_V6ONLY: ::core::ffi::c_int = 27 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
    #[repr(C)]

    pub union C2RustUnnamed {
        pub __u6_addr8: [__uint8_t; 16],
        pub __u6_addr16: [__uint16_t; 8],
        pub __u6_addr32: [__uint32_t; 4],
    }

// =============================================================================
// _stdio_h (merged from all files)
// =============================================================================

extern "C" {
    pub fn getline(
                __linep: *mut *mut ::core::ffi::c_char,
                __linecapp: *mut size_t,
                __stream: *mut FILE,
            ) -> ssize_t;
    pub fn vsnprintf(
                __str: *mut ::core::ffi::c_char,
                __size: size_t,
                __format: *const ::core::ffi::c_char,
                _: ::core::ffi::VaList,
            ) -> ::core::ffi::c_int;
}

// =============================================================================
// errno_h (merged from all files)
// =============================================================================

pub const ECONNABORTED: ::core::ffi::c_int = 53 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const EIO: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ENOSYS: ::core::ffi::c_int = 78 as ::core::ffi::c_int;
pub const ESRCH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

// =============================================================================
// bouncer_h types - ResponseAction enum
// =============================================================================

pub type ResponseAction = ::core::ffi::c_uint;
pub const RA_FORWARD: ResponseAction = 0;
pub const RA_SKIP: ResponseAction = 1;
pub const RA_FAKE: ResponseAction = 2;

// =============================================================================
// bouncer_h types - PgAddr union and sockaddr_ucreds
// =============================================================================

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

// =============================================================================
// bouncer_h types - ScramState
// =============================================================================

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

// =============================================================================
// bouncer_h types - PgCredentials and PgGlobalUser
// =============================================================================

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

// =============================================================================
// pktbuf_h types - PktBuf
// =============================================================================

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

// =============================================================================
// bouncer_h types - PgDatabase
// =============================================================================

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

// =============================================================================
// bouncer_h types - PgPool
// =============================================================================

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

// =============================================================================
// bouncer_h types - CallbackState
// =============================================================================

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct CallbackState {
    #[bitfield(name = "flag", ty = "PacketCallbackFlag", bits = "0..=7")]
    pub flag: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub pkt: PktHdr,
}

// =============================================================================
// bouncer_h types - OutstandingRequest
// =============================================================================

#[derive(Copy, Clone)]
#[repr(C)]
pub struct OutstandingRequest {
    pub node: List,
    pub type_0: ::core::ffi::c_char,
    pub action: ResponseAction,
    pub server_ps: *mut PgServerPreparedStatement,
    pub server_ps_query_id: uint64_t,
}

// =============================================================================
// bouncer_h types - C2RustUnnamed_9 (socket-specific union)
// =============================================================================

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub dns_token: *mut DNSToken,
    pub db: *mut PgDatabase,
}

// =============================================================================
// bouncer_h types - PgSocket (main socket struct)
// =============================================================================

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

// =============================================================================
// bouncer_h inline functions
// =============================================================================

#[inline]
pub unsafe fn first_socket(slist: *mut StatList) -> *mut PgSocket {
    if statlist_empty(slist) {
        return ::core::ptr::null_mut::<PgSocket>();
    }
    (*slist).head.next as *mut PgSocket
}

#[inline]
pub unsafe fn last_socket(slist: *mut StatList) -> *mut PgSocket {
    if statlist_empty(slist) {
        return ::core::ptr::null_mut::<PgSocket>();
    }
    (*slist).head.prev as *mut PgSocket
}

#[inline]
pub unsafe fn pga_is_unix(a: *const PgAddr) -> bool {
    (*a).sa.sa_family as ::core::ffi::c_int == AF_UNIX
}

#[inline]
pub unsafe fn pga_family(a: *const PgAddr) -> ::core::ffi::c_uint {
    (*a).sa.sa_family as ::core::ffi::c_uint
}

#[inline]
pub unsafe fn cstr_skip_ws(mut p: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    while *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int == ' ' as i32 {
        p = p.offset(1);
    }
    p
}

// =============================================================================
// bouncer_h constants
// =============================================================================

pub const BACKENDKEY_LEN: ::core::ffi::c_int = 8;
pub const MAX_USERNAME: ::core::ffi::c_int = 128;
pub const MAX_PASSWORD: ::core::ffi::c_int = 2048;
pub const CANCELLATION_TTL_MASK: ::core::ffi::c_int = 0x3;
pub const PKT_STARTUP_V2: ::core::ffi::c_uint = 131072;
pub const PKT_STARTUP_V3: ::core::ffi::c_int = 0x30000;
pub const PKT_STARTUP_V3_UNSUPPORTED: ::core::ffi::c_int = 0x30001;
pub const PKT_STARTUP_V4: ::core::ffi::c_int = 0x40000;
pub const PKT_CANCEL: ::core::ffi::c_int = 80877102;
pub const PKT_SSLREQ: ::core::ffi::c_int = 80877103;
pub const PKT_GSSENCREQ: ::core::ffi::c_int = 80877104;
pub const RAW_IOBUF_SIZE: ::core::ffi::c_ulong = 12;
pub const SD_LISTEN_FDS_START: ::core::ffi::c_int = 3;

// =============================================================================
// bouncer_h extern functions (pga_* functions defined in util.c)
// =============================================================================

extern "C" {
    pub fn pga_port(a: *const PgAddr) -> ::core::ffi::c_int;
    pub fn pga_set(a: *mut PgAddr, fam: ::core::ffi::c_int, port: ::core::ffi::c_int);
    pub fn pga_copy(a: *mut PgAddr, sa: *const sockaddr);
    pub fn pga_cmp_addr(a: *const PgAddr, b: *const PgAddr) -> ::core::ffi::c_int;
    pub fn pga_ntop(
        a: *const PgAddr,
        dst: *mut ::core::ffi::c_char,
        dstlen: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    pub fn pga_str(
        a: *const PgAddr,
        dst: *mut ::core::ffi::c_char,
        dstlen: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    pub fn pga_details(
        a: *const PgAddr,
        dst: *mut ::core::ffi::c_char,
        dstlen: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
}

// =============================================================================
// pktbuf_h extern functions
// =============================================================================

extern "C" {
    pub fn pktbuf_dynamic(start_len: ::core::ffi::c_int) -> *mut PktBuf;
    pub fn pktbuf_free(buf: *mut PktBuf);
    pub fn pktbuf_static(buf: *mut PktBuf, data: *mut uint8_t, len: ::core::ffi::c_int);
    pub fn pktbuf_temp() -> *mut PktBuf;
}

// =============================================================================
// sbuf_h extern functions
// =============================================================================

extern "C" {
    pub fn sbuf_init(sbuf: *mut SBuf, proto_fn: sbuf_cb_t);
    pub fn sbuf_pause(sbuf: *mut SBuf) -> bool;
    pub fn sbuf_prepare_skip(sbuf: *mut SBuf, amount: ::core::ffi::c_uint);
    pub fn sbuf_tls_accept(sbuf: *mut SBuf) -> bool;
}

#[inline]
pub unsafe fn sbuf_is_empty(sbuf: *mut SBuf) -> bool {
    iobuf_empty((*sbuf).io) && (*sbuf).pkt_remain == 0
}

#[inline]
pub unsafe fn sbuf_is_closed(sbuf: *mut SBuf) -> bool {
    (*sbuf).sock == 0
}

#[inline]
pub unsafe fn sbuf_op_send(
    sbuf: *mut SBuf,
    buf: *const ::core::ffi::c_void,
    len: size_t,
) -> ssize_t {
    (*(*sbuf).ops)
        .sbufio_send
        .expect("non-null function pointer")(sbuf, buf, len)
}

// =============================================================================
// sbuf_h constants
// =============================================================================

pub const SBUF_SMALL_PKT: ::core::ffi::c_int = 64;

// =============================================================================
// cfparser_h types
// =============================================================================

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CfLookup {
    pub name: *const ::core::ffi::c_char,
    pub value: ::core::ffi::c_int,
}

// =============================================================================
// hba_h types
// =============================================================================

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HBA {
    pub rules: List,
}

// =============================================================================
// bouncer constants
// =============================================================================

pub const DEFAULT_UNIX_SOCKET_DIR: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"/tmp\0") };
