
pub mod _types_h {
    
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
}

pub mod _uintptr_t_h {
    
    pub type uintptr_t = usize;
}

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
    
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    use super::_types_h::{__int32_t, __int64_t, __uint16_t, __uint32_t, __uint64_t};
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

pub mod _in_addr_t_h {
    
    pub type in_addr_t = __uint32_t;
    use super::_types_h::__uint32_t;
}

pub mod _in_port_t_h {
    
    pub type in_port_t = __uint16_t;
    use super::_types_h::__uint16_t;
}

pub mod _mode_t_h {
    
    pub type mode_t = __darwin_mode_t;
    use super::sys__types_h::__darwin_mode_t;
}

pub mod _nlink_t_h {
    
    pub type nlink_t = __uint16_t;
    use super::_types_h::__uint16_t;
}

pub mod _pid_t_h {
    
    pub type pid_t = __darwin_pid_t;
    use super::sys__types_h::__darwin_pid_t;
}

pub mod _off_t_h {
    
    pub type off_t = __darwin_off_t;
    use super::sys__types_h::__darwin_off_t;
}

pub mod _uid_t_h {
    
    pub type uid_t = __darwin_uid_t;
    use super::sys__types_h::__darwin_uid_t;
}

pub mod _size_t_h {
    
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}

pub mod _ssize_t_h {
    
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
}

pub mod _ptrdiff_t_h {
    
    pub type ptrdiff_t = __darwin_ptrdiff_t;
    use super::_types_h::__darwin_ptrdiff_t;
}

pub mod _uint8_t_h {
    
    pub type uint8_t = u8;
}

pub mod _uint16_t_h {
    
    pub type uint16_t = u16;
}

pub mod _uint32_t_h {
    
    pub type uint32_t = u32;
}

pub mod _uint64_t_h {
    
    pub type uint64_t = u64;
}

pub mod _timeval_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct timeval {
        pub tv_sec: __darwin_time_t,
        pub tv_usec: __darwin_suseconds_t,
    }
    use super::_types_h::__darwin_time_t;
    use super::sys__types_h::__darwin_suseconds_t;
}

pub mod _timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct timespec {
        pub tv_sec: __darwin_time_t,
        pub tv_nsec: ::core::ffi::c_long,
    }
    use super::_types_h::__darwin_time_t;
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
    use super::_types_h::{__int32_t, __int64_t, __uint32_t};
    use super::_uid_t_h::uid_t;
    use super::sys__types_h::__darwin_ino64_t;
    extern "C" {
        
        pub fn lstat(_: *const ::core::ffi::c_char, _: *mut stat) -> ::core::ffi::c_int;
    }
}

pub mod tls_h {
    extern "C" {
        
        pub type tls;
    }
}


pub mod list_h {
    pub use super::super::common::types::{List, list_init, list_empty, list_prepend, list_append, list_del, list_pop, list_first, list_last};
}



pub mod _sa_family_t_h {
    
    pub type sa_family_t = __uint8_t;
    use super::_types_h::__uint8_t;
}

pub mod _socklen_t_h {
    
    pub type socklen_t = __darwin_socklen_t;
    use super::_types_h::__darwin_socklen_t;
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
    
    pub struct sockaddr_storage {
        pub ss_len: __uint8_t,
        pub ss_family: sa_family_t,
        pub __ss_pad1: [::core::ffi::c_char; 6],
        pub __ss_align: __int64_t,
        pub __ss_pad2: [::core::ffi::c_char; 112],
    }
    
    pub const SOCK_STREAM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    pub const SO_REUSEADDR: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    pub const SO_REUSEPORT: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
    
    pub const SOL_SOCKET: ::core::ffi::c_int = 0xffff as ::core::ffi::c_int;
    
    pub const AF_UNSPEC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    pub const AF_UNIX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    pub const AF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    pub const AF_INET6: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
    use super::_sa_family_t_h::sa_family_t;
    use super::_socklen_t_h::socklen_t;
    use super::_types_h::{__int64_t, __uint8_t};
    extern "C" {
        
        pub fn bind(_: ::core::ffi::c_int, _: *const sockaddr, _: socklen_t) -> ::core::ffi::c_int;
        
        pub fn getsockname(
            _: ::core::ffi::c_int,
            _: *mut sockaddr,
            _: *mut socklen_t,
        ) -> ::core::ffi::c_int;
        
        pub fn listen(_: ::core::ffi::c_int, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
        
        pub fn setsockopt(
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_int,
            _: *const ::core::ffi::c_void,
            _: socklen_t,
        ) -> ::core::ffi::c_int;
        
        pub fn socket(
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
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
    use super::_sa_family_t_h::sa_family_t;
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
    
    pub const IPPROTO_TCP: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
    
    pub const IPPROTO_IPV6: ::core::ffi::c_int = 41 as ::core::ffi::c_int;
    use super::_in_addr_t_h::in_addr_t;
    use super::_in_port_t_h::in_port_t;
    use super::_sa_family_t_h::sa_family_t;
    use super::_types_h::__uint8_t;
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
    
    pub const IPV6_V6ONLY: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
    use super::_in_port_t_h::in_port_t;
    use super::_sa_family_t_h::sa_family_t;
    use super::_types_h::{__uint16_t, __uint32_t, __uint8_t};
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
    
    pub const AI_PASSIVE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    use super::_socklen_t_h::socklen_t;
    use super::socket_h::sockaddr;
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
}

pub mod event_h {
    
    pub type event_callback_fn = Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_short,
            *mut ::core::ffi::c_void,
        ) -> (),
    >;
    
    pub const EV_READ: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    pub const EV_PERSIST: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    use super::_timeval_h::timeval;
    use super::event_struct_h::event;
    extern "C" {
        
        pub type event_base;
        
        pub fn event_assign(
            _: *mut event,
            _: *mut event_base,
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_short,
            _: event_callback_fn,
            _: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
        
        pub fn event_add(ev: *mut event, timeout: *const timeval) -> ::core::ffi::c_int;
        
        pub fn event_del(_: *mut event) -> ::core::ffi::c_int;
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
    use super::_timeval_h::timeval;
    use super::_uint8_t_h::uint8_t;
    use super::event_h::event_base;
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
    
    pub const SD_LISTEN_FDS_START: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    pub const DEFAULT_UNIX_SOCKET_DIR: [::core::ffi::c_char; 5] =
        unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"/tmp\0") };
    #[inline]
    
    pub unsafe extern "C" fn pga_is_unix(mut a: *const PgAddr) -> bool {
        (*a).sa.sa_family as ::core::ffi::c_int == AF_UNIX
    }
    use super::_pid_t_h::pid_t;

    use super::_uid_t_h::uid_t;
    use super::_uint16_t_h::uint16_t;
    use super::_uint64_t_h::uint64_t;
    use super::_uint8_t_h::uint8_t;
    use crate::types::{AANode, AATree};
    use crate::types::pg_cryptohash_type;
    use super::dnslookup_h::DNSToken;
    use super::event_h::event_base;
    use super::in6_h::sockaddr_in6;
    use super::in_h::sockaddr_in;
    use super::list_h::List;
    use super::pktbuf_h::PktBuf;
    use crate::types::{PgClientPreparedStatement, PgServerPreparedStatement};
    use crate::types::PktHdr;
    use super::sbuf_h::SBuf;
    use super::socket_h::{sockaddr, AF_UNIX};
    use crate::types::StatList;
    use crate::types::usec_t;
    use super::varcache_h::VarCache;
    extern "C" {
        
        pub static mut pgb_event_base: *mut event_base;
        
        pub fn pga_port(a: *const PgAddr) -> ::core::ffi::c_int;
        
        pub fn pga_set(a: *mut PgAddr, fam: ::core::ffi::c_int, port: ::core::ffi::c_int);
        
        pub fn pga_copy(a: *mut PgAddr, sa: *const sockaddr);
        
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
        
        pub static mut cf_unix_socket_dir: *mut ::core::ffi::c_char;
        
        pub static mut cf_unix_socket_mode: ::core::ffi::c_int;
        
        pub static mut cf_unix_socket_group: *mut ::core::ffi::c_char;
        
        pub static mut cf_listen_addr: *mut ::core::ffi::c_char;
        
        pub static mut cf_listen_port: ::core::ffi::c_int;
        
        pub static mut cf_listen_backlog: ::core::ffi::c_int;
        
        pub static mut cf_pause_mode: ::core::ffi::c_int;
        
        pub static mut cf_so_reuseport: ::core::ffi::c_int;
        
        pub static mut cf_tcp_defer_accept: ::core::ffi::c_int;
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
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    use super::_uint8_t_h::uint8_t;
    use super::event_struct_h::event;
    use super::iobuf_h::IOBuf;
    use crate::types::MBuf;
    use super::tls_h::tls;
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
    use super::_uint8_t_h::uint8_t;
}

pub mod varcache_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct VarCache {
        pub var_list: *mut *mut PStr,
    }
    use crate::types::PStr;
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
    use super::_uint8_t_h::uint8_t;
    use super::bouncer_h::PgSocket;
    use super::event_struct_h::event;
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
    }
}

pub mod string_h {
    
    pub type str_cb =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char) -> bool>;
    extern "C" {
        
        pub fn parse_word_list(
            s: *const ::core::ffi::c_char,
            cb_func: str_cb,
            cb_arg: *mut ::core::ffi::c_void,
        ) -> bool;
    }
}

pub mod pooler_h {
    
    pub type pooler_cb = Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int, *const PgAddr) -> bool,
    >;
    use super::bouncer_h::PgAddr;
}

pub mod _stdio_h {
    use super::_size_t_h::size_t;
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
    use super::_size_t_h::size_t;
    extern "C" {
        
        pub fn calloc(__count: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
        
        pub fn free(_: *mut ::core::ffi::c_void);
    }
}

pub mod system_h {
    use super::_mode_t_h::mode_t;
    extern "C" {
        
        pub fn change_file_mode(
            fn_0: *const ::core::ffi::c_char,
            mode: mode_t,
            user: *const ::core::ffi::c_char,
            group: *const ::core::ffi::c_char,
        );
    }
}

pub mod _null_h {
    
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}

pub mod usual_socket_h {
    use super::_size_t_h::size_t;
    use super::socket_h::sockaddr;
    extern "C" {
        
        pub fn sa2str(
            sa: *const sockaddr,
            buf: *mut ::core::ffi::c_char,
            buflen: size_t,
        ) -> *const ::core::ffi::c_char;
    }
}

pub mod _string_h {
    use super::_size_t_h::size_t;
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
        
        pub fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
        
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}

pub mod util_h {
    use super::_timeval_h::timeval;
    use super::event_struct_h::event;
    extern "C" {
        
        pub fn tune_socket(sock: ::core::ffi::c_int, is_unix: bool) -> bool;
        
        pub fn safe_evtimer_add(ev: *mut event, tv: *mut timeval);
    }
}

pub mod safeio_h {
    use super::_socklen_t_h::socklen_t;
    use super::socket_h::sockaddr;
    extern "C" {
        
        pub fn safe_close(fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
        
        pub fn safe_accept(
            fd: ::core::ffi::c_int,
            sa: *mut sockaddr,
            sa_len: *mut socklen_t,
        ) -> ::core::ffi::c_int;
    }
}

pub mod objects_h {
    use super::bouncer_h::PgSocket;
    extern "C" {
        
        pub fn accept_client(sock: ::core::ffi::c_int, is_unix: bool) -> *mut PgSocket;
    }
}

pub mod _stdlib_h {
    extern "C" {
        
        pub fn atexit(_: Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int;
        
        pub fn exit(_: ::core::ffi::c_int) -> !;
    }
}

pub mod errno_h {
    
    pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
    
    pub const EAGAIN: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
    
    pub const ECONNABORTED: ::core::ffi::c_int = 53 as ::core::ffi::c_int;
    extern "C" {
        
        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}

pub mod unistd_h {
    extern "C" {
        
        pub fn unlink(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    }
}

pub mod stdbool_h {
    
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::_blkcnt_t_h::blkcnt_t;
pub use self::_blksize_t_h::blksize_t;
pub use self::_dev_t_h::dev_t;
pub use self::_gid_t_h::gid_t;
pub use self::_in_addr_t_h::in_addr_t;
pub use self::_in_port_t_h::in_port_t;
use self::_malloc_h::{calloc, free};
pub use self::_mode_t_h::mode_t;
pub use self::_nlink_t_h::nlink_t;
pub use self::_null_h::NULL;
pub use self::_off_t_h::off_t;
pub use self::_pid_t_h::pid_t;
pub use self::_ptrdiff_t_h::ptrdiff_t;
pub use self::_sa_family_t_h::sa_family_t;
pub use self::_size_t_h::size_t;
pub use self::_socklen_t_h::socklen_t;
pub use self::_ssize_t_h::ssize_t;
use self::_stdio_h::snprintf;
use self::_stdlib_h::{atexit, exit};
use self::_string_h::{memset, strcmp, strerror, strlen};
pub use self::_timespec_h::timespec;
pub use self::_timeval_h::timeval;
pub use self::_types_h::{
    __darwin_ptrdiff_t, __darwin_size_t, __darwin_socklen_t, __darwin_ssize_t, __darwin_time_t,
    __int32_t, __int64_t, __uint16_t, __uint32_t, __uint64_t, __uint8_t,
};
pub use self::_uid_t_h::uid_t;
pub use self::_uint16_t_h::uint16_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use crate::types::{aatree_cmp_f, aatree_walker_f, AANode, AATree};
pub use self::bouncer_h::{
    cf_listen_addr, cf_listen_backlog, cf_listen_port, cf_pause_mode, cf_so_reuseport,
    cf_tcp_defer_accept, cf_unix_socket_dir, cf_unix_socket_group, cf_unix_socket_mode, pga_copy,
    pga_is_unix, pga_ntop, pga_port, pga_set, pga_str, pgb_event_base, sockaddr_ucreds,
    C2RustUnnamed_9, CallbackState, LoadBalanceHosts, PacketCallbackFlag, PauseMode, PgAddr,
    PgCredentials, PgDatabase, PgGlobalUser, PgPool, PgSocket, PgStats, ReplicationType,
    ScramState, SocketState, CB_HANDLE_COMPLETE_PACKET, CB_NONE, CB_WANT_COMPLETE_PACKET,
    CL_ACTIVE, CL_ACTIVE_CANCEL, CL_FREE, CL_JUSTFREE, CL_LOGIN, CL_WAITING, CL_WAITING_CANCEL,
    CL_WAITING_LOGIN, DEFAULT_UNIX_SOCKET_DIR, LOAD_BALANCE_HOSTS_DISABLE,
    LOAD_BALANCE_HOSTS_ROUND_ROBIN, P_NONE, P_PAUSE, P_SUSPEND, REPLICATION_LOGICAL,
    REPLICATION_NONE, REPLICATION_PHYSICAL, SD_LISTEN_FDS_START, SV_ACTIVE, SV_ACTIVE_CANCEL,
    SV_BEING_CANCELED, SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN, SV_TESTED, SV_USED,
};
pub use crate::types::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};

pub use self::errno_h::{__error, EAGAIN, ECONNABORTED, EINVAL};
pub use self::event_h::{
    event_add, event_assign, event_base, event_callback_fn, event_del, EV_PERSIST, EV_READ,
};
pub use self::event_struct_h::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
pub use self::in6_h::{in6_addr, sockaddr_in6, C2RustUnnamed, IPV6_V6ONLY};
pub use self::in_h::{in_addr, sockaddr_in, IPPROTO_IPV6, IPPROTO_TCP};
pub use self::iobuf_h::{iobuf, IOBuf};
pub use self::list_h::{list_append, list_del, list_init, List};
pub use self::logging_h::{
    cf_verbose, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE, LG_STATS,
    LG_WARNING,
};
pub use crate::types::MBuf;
pub use self::netdb_h::{addrinfo, freeaddrinfo, gai_strerror, getaddrinfo, AI_PASSIVE};
use self::objects_h::accept_client;
pub use self::pktbuf_h::PktBuf;
pub use self::pooler_h::pooler_cb;
pub use crate::types::{
    PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement,
};
pub use crate::types::PktHdr;
use self::safeio_h::{safe_accept, safe_close};
pub use self::sbuf_h::{
    sbuf_cb_t, SBuf, SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK, SBUF_EV_FLUSH,
    SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY,
};
pub use self::socket_h::{
    bind, getsockname, listen, setsockopt, sockaddr, sockaddr_storage, socket, AF_INET, AF_INET6,
    AF_UNIX, AF_UNSPEC, SOCK_STREAM, SOL_SOCKET, SO_REUSEADDR, SO_REUSEPORT,
};
pub use self::stat_h::{lstat, stat};
pub use crate::types::{statlist_append, statlist_count, statlist_remove, StatList};
pub use self::stdbool_h::{false_0, true_0};
pub use self::string_h::{parse_word_list, str_cb};
pub use crate::types::{PStr, StrPool};
pub use self::sys__types_h::{
    __darwin_blkcnt_t, __darwin_blksize_t, __darwin_dev_t, __darwin_gid_t, __darwin_ino64_t,
    __darwin_mode_t, __darwin_off_t, __darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t,
    __DARWIN_NULL,
};
use self::system_h::change_file_mode;
pub use crate::types::usec_t;

pub use self::un_h::sockaddr_un;
use self::unistd_h::unlink;
use self::usual_socket_h::sa2str;
pub use crate::types::{UT_hash_bucket, UT_hash_handle, UT_hash_table};
use self::util_h::{safe_evtimer_add, tune_socket};
pub use self::varcache_h::VarCache;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ListenSocket {
    pub node: List,
    pub fd: ::core::ffi::c_int,
    pub active: bool,
    pub ev: event,
    pub addr: PgAddr,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub union C2RustUnnamed_10 {
    pub in_0: sockaddr_in,
    pub in6: sockaddr_in6,
    pub un: sockaddr_un,
    pub sa: sockaddr,
}

static mut sock_list: StatList = StatList {
    head: List {
        next: ::core::ptr::null::<List>() as *mut List,
        prev: ::core::ptr::null::<List>() as *mut List,
    },
    cur_count: 0,
};

static mut hints: addrinfo = addrinfo {
    ai_flags: AI_PASSIVE,
    ai_family: AF_UNSPEC,
    ai_socktype: SOCK_STREAM,
    ai_protocol: IPPROTO_TCP,
    ai_addrlen: 0,
    ai_canonname: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    ai_addr: ::core::ptr::null::<sockaddr>() as *mut sockaddr,
    ai_next: ::core::ptr::null::<addrinfo>() as *mut addrinfo,
};

static mut need_active: bool = false_0 != 0;

static mut pooler_active: bool = false_0 != 0;

static mut listen_addr_empty: bool = true_0 != 0;

static mut ev_err: event = event {
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

static mut err_timeout: timeval = timeval {
    tv_sec: 5 as __darwin_time_t,
    tv_usec: 0 as __darwin_suseconds_t,
};
#[no_mangle]

pub unsafe extern "C" fn cleanup_tcp_sockets() {
    let mut ls = ::core::ptr::null_mut::<ListenSocket>();
    let mut el = ::core::ptr::null_mut::<List>();
    let mut tmp_l = ::core::ptr::null_mut::<List>();
    if cf_pause_mode == P_SUSPEND as ::core::ffi::c_int {
        return;
    }
    el = sock_list.head.next;
    tmp_l = (*sock_list.head.next).next;
    while el != &raw mut sock_list.head {
        ls = (el as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut ListenSocket;
        if !pga_is_unix(&raw mut (*ls).addr) {
            if event_del(&raw mut (*ls).ev) < 0 as ::core::ffi::c_int {
                let mut _log_ctx = NULL;
                log_generic(
                    LG_WARNING,
                    _log_ctx,
                    b"cleanup_sockets, event_del: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    strerror(*__error()),
                );
            }
            if (*ls).fd > 0 as ::core::ffi::c_int {
                safe_close((*ls).fd);
                (*ls).fd = 0 as ::core::ffi::c_int;
            }
            statlist_remove(&raw mut sock_list, el);
            free(ls as *mut ::core::ffi::c_void);
        }
        el = tmp_l;
        tmp_l = (*tmp_l).next;
    }
}
#[no_mangle]

pub unsafe extern "C" fn cleanup_unix_sockets() {
    let mut ls = ::core::ptr::null_mut::<ListenSocket>();
    let mut el = ::core::ptr::null_mut::<List>();
    let mut tmp_l = ::core::ptr::null_mut::<List>();
    if cf_pause_mode == P_SUSPEND as ::core::ffi::c_int {
        return;
    }
    el = sock_list.head.next;
    tmp_l = (*sock_list.head.next).next;
    while el != &raw mut sock_list.head {
        ls = (el as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut ListenSocket;
        if event_del(&raw mut (*ls).ev) < 0 as ::core::ffi::c_int {
            let mut _log_ctx = NULL;
            log_generic(
                LG_WARNING,
                _log_ctx,
                b"cleanup_sockets, event_del: %s\0" as *const u8 as *const ::core::ffi::c_char,
                strerror(*__error()),
            );
        }
        if (*ls).fd > 0 as ::core::ffi::c_int {
            safe_close((*ls).fd);
            (*ls).fd = 0 as ::core::ffi::c_int;
        }
        if pga_is_unix(&raw mut (*ls).addr) as ::core::ffi::c_int != 0
            && *cf_unix_socket_dir.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '@' as i32
        {
            let mut buf: [::core::ffi::c_char; 126] = [0; 126];
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 126]>() as size_t,
                b"%s/.s.PGSQL.%d\0" as *const u8 as *const ::core::ffi::c_char,
                cf_unix_socket_dir,
                cf_listen_port,
            );
            unlink(&raw mut buf as *mut ::core::ffi::c_char);
        }
        statlist_remove(&raw mut sock_list, el);
        free(ls as *mut ::core::ffi::c_void);
        el = tmp_l;
        tmp_l = (*tmp_l).next;
    }
}

unsafe extern "C" fn add_listen(
    mut af: ::core::ffi::c_int,
    mut sa: *const sockaddr,
    mut salen: ::core::ffi::c_int,
) -> bool {
    let mut current_block: u64;
    let mut ls = ::core::ptr::null_mut::<ListenSocket>();
    let mut sock: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 128] = [0; 128];
    let mut errpos = ::core::ptr::null::<::core::ffi::c_char>();
    let mut _log_ctx = NULL;
    if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_DEBUG,
            _log_ctx,
            b"add_listen: %s\0" as *const u8 as *const ::core::ffi::c_char,
            sa2str(
                sa,
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
            ),
        );
    }
    errpos = b"socket\0" as *const u8 as *const ::core::ffi::c_char;
    sock = socket(af, SOCK_STREAM, 0 as ::core::ffi::c_int);
    if sock >= 0 as ::core::ffi::c_int {
        if af != AF_UNIX {
            let mut val = 1 as ::core::ffi::c_int;
            errpos = b"setsockopt\0" as *const u8 as *const ::core::ffi::c_char;
            res = setsockopt(
                sock,
                SOL_SOCKET,
                SO_REUSEADDR,
                &raw mut val as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
            );
            if res < 0 as ::core::ffi::c_int {
                current_block = 5649001695661142926;
            } else {
                current_block = 2968425633554183086;
            }
        } else {
            current_block = 2968425633554183086;
        }
        match current_block {
            5649001695661142926 => {}
            _ => {
                if af == AF_INET6 {
                    let mut val_0 = 1 as ::core::ffi::c_int;
                    errpos = b"setsockopt/IPV6_V6ONLY\0" as *const u8 as *const ::core::ffi::c_char;
                    res = setsockopt(
                        sock,
                        IPPROTO_IPV6,
                        IPV6_V6ONLY,
                        &raw mut val_0 as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
                    );
                    if res < 0 as ::core::ffi::c_int {
                        current_block = 5649001695661142926;
                    } else {
                        current_block = 17407779659766490442;
                    }
                } else {
                    current_block = 17407779659766490442;
                }
                match current_block {
                    5649001695661142926 => {}
                    _ => {
                        if af != AF_UNIX && cf_so_reuseport != 0 {
                            let mut val_1 = 1 as ::core::ffi::c_int;
                            errpos = b"setsockopt/SO_REUSEPORT\0" as *const u8
                                as *const ::core::ffi::c_char;
                            res = setsockopt(
                                sock,
                                SOL_SOCKET,
                                SO_REUSEPORT,
                                &raw mut val_1 as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
                            );
                            if res < 0 as ::core::ffi::c_int {
                                current_block = 5649001695661142926;
                            } else {
                                current_block = 12124785117276362961;
                            }
                        } else {
                            current_block = 12124785117276362961;
                        }
                        match current_block {
                            5649001695661142926 => {}
                            _ => {
                                errpos = b"bind\0" as *const u8 as *const ::core::ffi::c_char;
                                res = bind(sock, sa, salen as socklen_t);
                                if res >= 0 as ::core::ffi::c_int {
                                    errpos =
                                        b"tune_socket\0" as *const u8 as *const ::core::ffi::c_char;
                                    if tune_socket(sock, af == AF_UNIX) {
                                        errpos =
                                            b"listen\0" as *const u8 as *const ::core::ffi::c_char;
                                        res = listen(sock, cf_listen_backlog);
                                        if res >= 0 as ::core::ffi::c_int {
                                            errpos = b"calloc\0" as *const u8
                                                as *const ::core::ffi::c_char;
                                            ls = calloc(
                                                1 as size_t,
                                                ::core::mem::size_of::<ListenSocket>() as size_t,
                                            )
                                                as *mut ListenSocket;
                                            if !ls.is_null() {
                                                list_init(&raw mut (*ls).node);
                                                (*ls).fd = sock;
                                                if (*sa).sa_family as ::core::ffi::c_int == AF_UNIX
                                                {
                                                    pga_set(
                                                        &raw mut (*ls).addr,
                                                        AF_UNIX,
                                                        cf_listen_port,
                                                    );
                                                } else {
                                                    pga_copy(&raw mut (*ls).addr, sa);
                                                }
                                                if af == AF_UNIX {
                                                    if *cf_unix_socket_dir
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        != '@' as i32
                                                    {
                                                        let mut un = sa as *mut sockaddr_un;
                                                        change_file_mode(
                                                            &raw mut (*un).sun_path
                                                                as *mut ::core::ffi::c_char,
                                                            cf_unix_socket_mode as mode_t,
                                                            ::core::ptr::null::<::core::ffi::c_char>(
                                                            ),
                                                            cf_unix_socket_group,
                                                        );
                                                    }
                                                } else {
                                                    tune_accept(sock, cf_tcp_defer_accept != 0);
                                                }
                                                let mut _log_ctx_0 = NULL;
                                                log_generic(
                                                    LG_INFO,
                                                    _log_ctx_0,
                                                    b"listening on %s\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    sa2str(
                                                        sa,
                                                        &raw mut buf as *mut ::core::ffi::c_char,
                                                        ::core::mem::size_of::<
                                                            [::core::ffi::c_char; 128],
                                                        >(
                                                        )
                                                            as size_t,
                                                    ),
                                                );
                                                statlist_append(
                                                    &raw mut sock_list,
                                                    &raw mut (*ls).node,
                                                );
                                                return true_0 != 0;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let mut _log_ctx_1 = NULL;
    log_generic(
        LG_WARNING,
        _log_ctx_1,
        b"cannot listen on %s: %s(): %s\0" as *const u8 as *const ::core::ffi::c_char,
        sa2str(
            sa,
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
        ),
        errpos,
        strerror(*__error()),
    );
    if sock >= 0 as ::core::ffi::c_int {
        safe_close(sock);
    }
    false_0 != 0
}

unsafe extern "C" fn create_unix_socket(
    mut socket_dir: *const ::core::ffi::c_char,
    mut listen_port: ::core::ffi::c_int,
) {
    let mut un = sockaddr_un {
        sun_len: 0,
        sun_family: 0,
        sun_path: [0; 104],
    };
    let mut addrlen: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 0;
    let mut lockfile: [::core::ffi::c_char; 116] = [0; 116];
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
    memset(
        &raw mut un as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_un>() as size_t,
    );
    un.sun_family = AF_UNIX as sa_family_t;
    snprintf(
        &raw mut un.sun_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 104]>() as size_t,
        b"%s/.s.PGSQL.%d\0" as *const u8 as *const ::core::ffi::c_char,
        socket_dir,
        listen_port,
    );
    if *socket_dir.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '@' as i32 {
        addrlen = (2 as size_t)
            .wrapping_add(strlen(&raw mut un.sun_path as *mut ::core::ffi::c_char))
            as ::core::ffi::c_int;
        un.sun_path[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    } else {
        addrlen = ::core::mem::size_of::<sockaddr_un>() as ::core::ffi::c_int;
    }
    if *socket_dir.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '@' as i32 {
        snprintf(
            &raw mut lockfile as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 116]>() as size_t,
            b"%s.lock\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut un.sun_path as *mut ::core::ffi::c_char,
        );
        res = lstat(&raw mut lockfile as *mut ::core::ffi::c_char, &raw mut st);
        if res == 0 as ::core::ffi::c_int {
            let mut _log_ctx = NULL;
            log_generic(
                LG_FATAL,
                _log_ctx,
                b"unix port %d is in use\0" as *const u8 as *const ::core::ffi::c_char,
                listen_port,
            );
            exit(1 as ::core::ffi::c_int);
        }
        unlink(&raw mut un.sun_path as *mut ::core::ffi::c_char);
    }
    if !add_listen(AF_UNIX, &raw mut un as *const sockaddr, addrlen) {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_0,
            b"failed to create unix socket\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
}

unsafe extern "C" fn tune_accept(mut _sock: ::core::ffi::c_int, mut on: bool) {
    let mut act = if on as ::core::ffi::c_int != 0 {
        b"install\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b"uninstall\0" as *const u8 as *const ::core::ffi::c_char
    };
    let mut res = 0 as ::core::ffi::c_int;
    if on {
        *__error() = EINVAL;
        res = -(1 as ::core::ffi::c_int);
    }
    if res < 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            b"tune_accept: %s TCP_DEFER_ACCEPT: %s\0" as *const u8 as *const ::core::ffi::c_char,
            act,
            strerror(*__error()),
        );
    }
}
#[no_mangle]

pub unsafe extern "C" fn pooler_tune_accept(mut on: bool) {
    let mut el = ::core::ptr::null_mut::<List>();
    let mut ls = ::core::ptr::null_mut::<ListenSocket>();
    el = sock_list.head.next;
    while el != &raw mut sock_list.head {
        ls = (el as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut ListenSocket;
        if !pga_is_unix(&raw mut (*ls).addr) {
            tune_accept((*ls).fd, on);
        }
        el = (*el).next;
    }
}

unsafe extern "C" fn err_wait_func(
    mut _sock: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut _arg: *mut ::core::ffi::c_void,
) {
    if cf_pause_mode != P_SUSPEND as ::core::ffi::c_int {
        resume_pooler();
    }
}

unsafe extern "C" fn addrpair(
    mut src: *const PgAddr,
    mut dst: *const PgAddr,
) -> *const ::core::ffi::c_char {
    static mut ip1buf: [::core::ffi::c_char; 56] = [0; 56];
    static mut ip2buf: [::core::ffi::c_char; 56] = [0; 56];
    static mut buf: [::core::ffi::c_char; 128] = [0; 128];
    let mut ip1 = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ip2 = ::core::ptr::null::<::core::ffi::c_char>();
    if pga_is_unix(src) {
        return b"unix->unix\0" as *const u8 as *const ::core::ffi::c_char;
    }
    ip1 = pga_ntop(
        src,
        &raw mut ip1buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 56]>() as ::core::ffi::c_int,
    );
    ip2 = pga_ntop(
        dst,
        &raw mut ip2buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 56]>() as ::core::ffi::c_int,
    );
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
        b"%s:%d -> %s:%d\0" as *const u8 as *const ::core::ffi::c_char,
        ip1,
        pga_port(src),
        ip2,
        pga_port(dst),
    );
    &raw mut buf as *mut ::core::ffi::c_char
}

unsafe extern "C" fn conninfo(mut sk: *const PgSocket) -> *const ::core::ffi::c_char {
    if (*sk).state() as ::core::ffi::c_int >= SV_FREE as ::core::ffi::c_int {
        addrpair(&raw const (*sk).local_addr, &raw const (*sk).remote_addr)
    } else {
        addrpair(&raw const (*sk).remote_addr, &raw const (*sk).local_addr)
    }
}

unsafe extern "C" fn pool_accept(
    mut sock: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_short,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut ls = arg as *mut ListenSocket;
    let mut fd: ::core::ffi::c_int = 0;
    let mut client = ::core::ptr::null_mut::<PgSocket>();
    let mut raddr = C2RustUnnamed_10 {
        in_0: sockaddr_in {
            sin_len: 0,
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        },
    };
    let mut len: socklen_t = ::core::mem::size_of::<C2RustUnnamed_10>() as socklen_t;
    let mut is_unix = pga_is_unix(&raw mut (*ls).addr);
    if flags as ::core::ffi::c_int & EV_READ == 0 {
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            b"no EV_READ in pool_accept\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    loop {
        fd = safe_accept(sock, &raw mut raddr.sa, &raw mut len);
        if fd < 0 as ::core::ffi::c_int {
            if *__error() == EAGAIN {
                return;
            } else if *__error() == ECONNABORTED {
                return;
            }
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_0,
                b"accept() failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
                strerror(*__error()),
            );
            event_assign(
                &raw mut ev_err,
                pgb_event_base,
                -(1 as ::core::ffi::c_int),
                0 as ::core::ffi::c_short,
                Some(
                    err_wait_func
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            ::core::ffi::c_short,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
            safe_evtimer_add(&raw mut ev_err, &raw mut err_timeout);
            suspend_pooler();
            return;
        }
        let mut _log_ctx_1 = NULL;
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                _log_ctx_1,
                b"new fd from accept=%d\0" as *const u8 as *const ::core::ffi::c_char,
                fd,
            );
        }
        if is_unix {
            client = accept_client(fd, true_0 != 0);
        } else {
            client = accept_client(fd, false_0 != 0);
        }
        if !client.is_null()
            && (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
        {
            log_generic(
                LG_DEBUG,
                client as *mut ::core::ffi::c_void,
                b"P: got connection: %s\0" as *const u8 as *const ::core::ffi::c_char,
                conninfo(client),
            );
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn use_pooler_socket(
    mut sock: ::core::ffi::c_int,
    mut is_unix: bool,
) -> bool {
    let mut ls = ::core::ptr::null_mut::<ListenSocket>();
    let mut res: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 56] = [0; 56];
    if !tune_socket(sock, is_unix) {
        return false_0 != 0;
    }
    ls = calloc(
        1 as size_t,
        ::core::mem::size_of::<ListenSocket>() as size_t,
    ) as *mut ListenSocket;
    if ls.is_null() {
        return false_0 != 0;
    }
    (*ls).fd = sock;
    if is_unix {
        pga_set(&raw mut (*ls).addr, AF_UNIX, cf_listen_port);
    } else {
        let mut ss = sockaddr_storage {
            ss_len: 0,
            ss_family: 0,
            __ss_pad1: [0; 6],
            __ss_align: 0,
            __ss_pad2: [0; 112],
        };
        let mut len: socklen_t = ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
        res = getsockname(sock, &raw mut ss as *mut sockaddr, &raw mut len);
        if res < 0 as ::core::ffi::c_int {
            let mut _log_ctx = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx,
                b"getsockname failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
            free(ls as *mut ::core::ffi::c_void);
            return false_0 != 0;
        }
        pga_copy(&raw mut (*ls).addr, &raw mut ss as *mut sockaddr);
    }
    let mut _log_ctx_0 = NULL;
    log_generic(
        LG_INFO,
        _log_ctx_0,
        b"got pooler socket: %s\0" as *const u8 as *const ::core::ffi::c_char,
        pga_str(
            &raw mut (*ls).addr,
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 56]>() as ::core::ffi::c_int,
        ),
    );
    statlist_append(&raw mut sock_list, &raw mut (*ls).node);
    true_0 != 0
}
#[no_mangle]

pub unsafe extern "C" fn suspend_pooler() {
    let mut el = ::core::ptr::null_mut::<List>();
    let mut ls = ::core::ptr::null_mut::<ListenSocket>();
    need_active = false_0 != 0;
    el = sock_list.head.next;
    while el != &raw mut sock_list.head {
        ls = (el as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut ListenSocket;
        if (*ls).active {
            if event_del(&raw mut (*ls).ev) < 0 as ::core::ffi::c_int {
                let mut _log_ctx = NULL;
                log_generic(
                    LG_WARNING,
                    _log_ctx,
                    b"suspend_pooler, event_del: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    strerror(*__error()),
                );
                return;
            }
            (*ls).active = false_0 != 0;
        }
        el = (*el).next;
    }
    pooler_active = false_0 != 0;
}
#[no_mangle]

pub unsafe extern "C" fn resume_pooler() {
    let mut el = ::core::ptr::null_mut::<List>();
    let mut ls = ::core::ptr::null_mut::<ListenSocket>();
    need_active = true_0 != 0;
    el = sock_list.head.next;
    while el != &raw mut sock_list.head {
        ls = (el as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut ListenSocket;
        if !(*ls).active {
            event_assign(
                &raw mut (*ls).ev,
                pgb_event_base,
                (*ls).fd,
                (EV_READ | EV_PERSIST) as ::core::ffi::c_short,
                Some(
                    pool_accept
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            ::core::ffi::c_short,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                ),
                ls as *mut ::core::ffi::c_void,
            );
            if event_add(&raw mut (*ls).ev, ::core::ptr::null::<timeval>())
                < 0 as ::core::ffi::c_int
            {
                let mut _log_ctx = NULL;
                log_generic(
                    LG_WARNING,
                    _log_ctx,
                    b"event_add failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    strerror(*__error()),
                );
                return;
            }
            (*ls).active = true_0 != 0;
        }
        el = (*el).next;
    }
    pooler_active = true_0 != 0;
}
#[no_mangle]

pub unsafe extern "C" fn per_loop_pooler_maint() {
    if need_active as ::core::ffi::c_int != 0 && !pooler_active {
        resume_pooler();
    } else if !need_active && pooler_active as ::core::ffi::c_int != 0 {
        suspend_pooler();
    }
}

unsafe extern "C" fn parse_addr(
    mut _arg: *mut ::core::ffi::c_void,
    mut addr: *const ::core::ffi::c_char,
) -> bool {
    let mut res: ::core::ffi::c_int = 0;
    let mut service: [::core::ffi::c_char; 64] = [0; 64];
    let mut ai = ::core::ptr::null_mut::<addrinfo>();
    let mut gaires = ::core::ptr::null_mut::<addrinfo>();
    if *addr == 0 {
        return true_0 != 0;
    }
    listen_addr_empty = false_0 != 0;
    if strcmp(addr, b"*\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        addr = ::core::ptr::null::<::core::ffi::c_char>();
    }
    snprintf(
        &raw mut service as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
        b"%d\0" as *const u8 as *const ::core::ffi::c_char,
        cf_listen_port,
    );
    res = getaddrinfo(
        addr,
        &raw mut service as *mut ::core::ffi::c_char,
        &raw const hints,
        &raw mut gaires,
    );
    if res != 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx,
            b"getaddrinfo('%s', '%d') = %s [%d]\0" as *const u8 as *const ::core::ffi::c_char,
            if !addr.is_null() {
                addr
            } else {
                b"*\0" as *const u8 as *const ::core::ffi::c_char
            },
            cf_listen_port,
            gai_strerror(res),
            res,
        );
        exit(1 as ::core::ffi::c_int);
    }
    ai = gaires;
    while !ai.is_null() {
        add_listen(
            (*ai).ai_family,
            (*ai).ai_addr,
            (*ai).ai_addrlen as ::core::ffi::c_int,
        );
        ai = (*ai).ai_next;
    }
    freeaddrinfo(gaires);
    true_0 != 0
}
#[no_mangle]

pub unsafe extern "C" fn pooler_setup() {
    let mut n: ::core::ffi::c_int = 0;
    n = 0 as ::core::ffi::c_int;
    if n > 0 as ::core::ffi::c_int {
        if !cf_listen_addr.is_null() && *cf_listen_addr as ::core::ffi::c_int != 0 {
            let mut _log_ctx = NULL;
            log_generic(
                LG_WARNING,
                _log_ctx,
                b"sockets passed from service manager, cf_listen_addr ignored\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if !cf_unix_socket_dir.is_null()
            && *cf_unix_socket_dir as ::core::ffi::c_int != 0
            && strcmp(cf_unix_socket_dir, DEFAULT_UNIX_SOCKET_DIR.as_ptr())
                != 0 as ::core::ffi::c_int
        {
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_WARNING,
                _log_ctx_0,
                b"sockets passed from service manager, cf_unix_socket_dir ignored\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        let mut i = 0 as ::core::ffi::c_int;
        while i < n {
            let mut fd = SD_LISTEN_FDS_START + i;
            let mut ls = ::core::ptr::null_mut::<ListenSocket>();
            let mut ok = true_0 != 0;
            ls = calloc(
                1 as size_t,
                ::core::mem::size_of::<ListenSocket>() as size_t,
            ) as *mut ListenSocket;
            if ls.is_null() {
                let mut _log_ctx_1 = NULL;
                log_generic(
                    LG_FATAL,
                    _log_ctx_1,
                    b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
                );
                exit(1 as ::core::ffi::c_int);
            }
            list_init(&raw mut (*ls).node);
            (*ls).fd = fd;
            if !ok {
                let mut _log_ctx_2 = NULL;
                log_generic(
                    LG_FATAL,
                    _log_ctx_2,
                    b"failed to set up socket passed from service manager (fd %d)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    fd,
                );
                exit(1 as ::core::ffi::c_int);
            }
            let mut _log_ctx_3 = NULL;
            log_generic(
                LG_INFO,
                _log_ctx_3,
                b"socket passed from service manager (fd %d)\0" as *const u8
                    as *const ::core::ffi::c_char,
                fd,
            );
            statlist_append(&raw mut sock_list, &raw mut (*ls).node);
            i += 1;
        }
    } else {
        let mut ok_0: bool = false;
        static mut init_done: bool = false_0 != 0;
        if !init_done {
            atexit(Some(cleanup_tcp_sockets as unsafe extern "C" fn() -> ()));
            atexit(Some(cleanup_unix_sockets as unsafe extern "C" fn() -> ()));
            init_done = true_0 != 0;
        }
        ok_0 = parse_word_list(
            cf_listen_addr,
            Some(
                parse_addr
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                    ) -> bool,
            ),
            NULL,
        );
        if !ok_0 {
            let mut _log_ctx_4 = NULL;
            log_generic(
                LG_FATAL,
                _log_ctx_4,
                b"failed to parse listen_addr list: %s\0" as *const u8
                    as *const ::core::ffi::c_char,
                cf_listen_addr,
            );
            exit(1 as ::core::ffi::c_int);
        }
        if !listen_addr_empty && statlist_count(&raw mut sock_list) == 0 {
            let mut _log_ctx_5 = NULL;
            log_generic(
                LG_FATAL,
                _log_ctx_5,
                b"failed to listen on any address in listen_addr list: %s\0" as *const u8
                    as *const ::core::ffi::c_char,
                cf_listen_addr,
            );
            exit(1 as ::core::ffi::c_int);
        }
        if !cf_unix_socket_dir.is_null() && *cf_unix_socket_dir as ::core::ffi::c_int != 0 {
            create_unix_socket(cf_unix_socket_dir, cf_listen_port);
        }
    }
    if statlist_count(&raw mut sock_list) == 0 {
        let mut _log_ctx_6 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_6,
            b"nowhere to listen on\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    resume_pooler();
}
#[no_mangle]

pub unsafe extern "C" fn for_each_pooler_fd(
    mut cbfunc: pooler_cb,
    mut arg: *mut ::core::ffi::c_void,
) -> bool {
    let mut el = ::core::ptr::null_mut::<List>();
    let mut ls = ::core::ptr::null_mut::<ListenSocket>();
    let mut ok: bool = false;
    el = sock_list.head.next;
    while el != &raw mut sock_list.head {
        ls = (el as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut ListenSocket;
        ok = cbfunc.expect("non-null function pointer")(arg, (*ls).fd, &raw mut (*ls).addr);
        if !ok {
            return false_0 != 0;
        }
        el = (*el).next;
    }
    true_0 != 0
}
unsafe extern "C" fn run_static_initializers() {
    sock_list = StatList {
        head: List {
            next: &raw mut sock_list.head,
            prev: &raw mut sock_list.head,
        },
        cur_count: 0 as ::core::ffi::c_int,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
