#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:27"]
pub mod _types_h {
    #[c2rust::src_loc = "32:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "34:1"]
    pub type __uint16_t = u16;
    #[c2rust::src_loc = "35:1"]
    pub type __int32_t = i32;
    #[c2rust::src_loc = "36:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "77:1"]
    pub type __darwin_ptrdiff_t = isize;
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
    #[c2rust::src_loc = "117:1"]
    pub type __darwin_socklen_t = __uint32_t;
    #[c2rust::src_loc = "118:1"]
    pub type __darwin_ssize_t = isize;
    #[c2rust::src_loc = "119:1"]
    pub type __darwin_time_t = ::core::ffi::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:27"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:27"]
pub mod sys__types_h {
    #[c2rust::src_loc = "84:1"]
    pub type __darwin_pid_t = __int32_t;
    #[c2rust::src_loc = "86:1"]
    pub type __darwin_suseconds_t = __int32_t;
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_uid_t = __uint32_t;
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    use super::_types_h::{__int32_t, __uint32_t};
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_in_addr_t.h:27"]
pub mod _in_addr_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type in_addr_t = __uint32_t;
    use super::_types_h::__uint32_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_in_port_t.h:27"]
pub mod _in_port_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type in_port_t = __uint16_t;
    use super::_types_h::__uint16_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_pid_t.h:27"]
pub mod _pid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pid_t = __darwin_pid_t;
    use super::sys__types_h::__darwin_pid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uid_t.h:27"]
pub mod _uid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uid_t = __darwin_uid_t;
    use super::sys__types_h::__darwin_uid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:27"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ssize_t.h:27"]
pub mod _ssize_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_time_t.h:27"]
pub mod _time_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type time_t = __darwin_time_t;
    use super::_types_h::__darwin_time_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ptrdiff_t.h:27"]
pub mod _ptrdiff_t_h {
    #[c2rust::src_loc = "51:1"]
    pub type ptrdiff_t = __darwin_ptrdiff_t;
    use super::_types_h::__darwin_ptrdiff_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:27"]
pub mod _uint8_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint8_t = u8;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint16_t.h:27"]
pub mod _uint16_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint16_t = u16;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:27"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = u32;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint64_t.h:27"]
pub mod _uint64_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint64_t = u64;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_timeval.h:27"]
pub mod _timeval_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "34:1"]
    pub struct timeval {
        pub tv_sec: __darwin_time_t,
        pub tv_usec: __darwin_suseconds_t,
    }
    use super::_types_h::__darwin_time_t;
    use super::sys__types_h::__darwin_suseconds_t;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/tls/tls.h:27"]
pub mod tls_h {
    #[c2rust::src_loc = "29:9"]
    pub const TLS_PROTOCOL_TLSv1_0: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "30:9"]
    pub const TLS_PROTOCOL_TLSv1_1: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "31:9"]
    pub const TLS_PROTOCOL_TLSv1_2: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "32:9"]
    pub const TLS_PROTOCOL_TLSv1_3: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
    #[c2rust::src_loc = "33:9"]
    pub const TLS_PROTOCOL_TLSv1: ::core::ffi::c_int =
        TLS_PROTOCOL_TLSv1_0 | TLS_PROTOCOL_TLSv1_1 | TLS_PROTOCOL_TLSv1_2 | TLS_PROTOCOL_TLSv1_3;
    #[c2rust::src_loc = "36:9"]
    pub const TLS_PROTOCOLS_ALL: ::core::ffi::c_int = TLS_PROTOCOL_TLSv1;
    #[c2rust::src_loc = "39:9"]
    pub const TLS_WANT_POLLIN: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
    #[c2rust::src_loc = "40:9"]
    pub const TLS_WANT_POLLOUT: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    use super::_uint32_t_h::uint32_t;
    extern "C" {
        #[c2rust::src_loc = "66:1"]
        pub type tls;
        #[c2rust::src_loc = "67:1"]
        pub type tls_config;
        #[c2rust::src_loc = "69:1"]
        pub fn tls_init() -> ::core::ffi::c_int;
        #[c2rust::src_loc = "75:1"]
        pub fn tls_error(_ctx: *mut tls) -> *const ::core::ffi::c_char;
        #[c2rust::src_loc = "77:1"]
        pub fn tls_config_new() -> *mut tls_config;
        #[c2rust::src_loc = "78:1"]
        pub fn tls_config_free(_config: *mut tls_config);
        #[c2rust::src_loc = "80:1"]
        pub fn tls_config_set_ca_file(
            _config: *mut tls_config,
            _ca_file: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "84:1"]
        pub fn tls_config_set_cert_file(
            _config: *mut tls_config,
            _cert_file: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "88:1"]
        pub fn tls_config_set_ciphers(
            _config: *mut tls_config,
            _ciphers: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "89:1"]
        pub fn tls_config_set_ciphers_v13(
            _config: *mut tls_config,
            _ciphers: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "90:1"]
        pub fn tls_config_set_dheparams(
            _config: *mut tls_config,
            _params: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "91:1"]
        pub fn tls_config_set_ecdhecurve(
            _config: *mut tls_config,
            _name: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "92:1"]
        pub fn tls_config_set_key_file(
            _config: *mut tls_config,
            _key_file: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "102:1"]
        pub fn tls_config_set_protocols(_config: *mut tls_config, _protocols: uint32_t);
        #[c2rust::src_loc = "108:1"]
        pub fn tls_config_insecure_noverifycert(_config: *mut tls_config);
        #[c2rust::src_loc = "109:1"]
        pub fn tls_config_insecure_noverifyname(_config: *mut tls_config);
        #[c2rust::src_loc = "111:1"]
        pub fn tls_config_verify(_config: *mut tls_config);
        #[c2rust::src_loc = "113:1"]
        pub fn tls_config_verify_client(_config: *mut tls_config);
        #[c2rust::src_loc = "114:1"]
        pub fn tls_config_verify_client_optional(_config: *mut tls_config);
        #[c2rust::src_loc = "117:1"]
        pub fn tls_config_parse_protocols(
            _protocols: *mut uint32_t,
            _protostr: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "119:1"]
        pub fn tls_client() -> *mut tls;
        #[c2rust::src_loc = "120:1"]
        pub fn tls_server() -> *mut tls;
        #[c2rust::src_loc = "121:1"]
        pub fn tls_configure(_ctx: *mut tls, _config: *mut tls_config) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "123:1"]
        pub fn usual_tls_free(_ctx: *mut tls);
        #[c2rust::src_loc = "125:1"]
        pub fn tls_accept_fds(
            _ctx: *mut tls,
            _cctx: *mut *mut tls,
            _fd_read: ::core::ffi::c_int,
            _fd_write: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "129:1"]
        pub fn tls_connect_fds(
            _ctx: *mut tls,
            _fd_read: ::core::ffi::c_int,
            _fd_write: ::core::ffi::c_int,
            _servername: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "134:1"]
        pub fn tls_handshake(_ctx: *mut tls) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "135:1"]
        pub fn tls_read(_ctx: *mut tls, _buf: *mut ::core::ffi::c_void, _buflen: size_t)
            -> ssize_t;
        #[c2rust::src_loc = "136:1"]
        pub fn tls_write(
            _ctx: *mut tls,
            _buf: *const ::core::ffi::c_void,
            _buflen: size_t,
        ) -> ssize_t;
        #[c2rust::src_loc = "137:1"]
        pub fn tls_close(_ctx: *mut tls) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "168:1"]
        pub fn tls_config_equal(
            server_connect_conf_left: *mut tls_config,
            server_connect_conf_right: *mut tls_config,
        ) -> bool;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/time.h:27"]
pub mod time_h {
    #[c2rust::src_loc = "40:1"]
    pub type usec_t = uint64_t;
    use super::_uint64_t_h::uint64_t;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/list.h:27"]
pub mod list_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "35:1"]
    pub struct List {
        pub next: *mut List,
        pub prev: *mut List,
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/statlist.h:27"]
pub mod statlist_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:1"]
    pub struct StatList {
        pub head: List,
        pub cur_count: ::core::ffi::c_int,
    }
    use super::list_h::List;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/aatree.h:27"]
pub mod aatree_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "41:1"]
    pub struct AATree {
        pub root: *mut AANode,
        pub count: ::core::ffi::c_int,
        pub node_cmp: aatree_cmp_f,
        pub release_cb: aatree_walker_f,
    }
    #[c2rust::src_loc = "36:1"]
    pub type aatree_walker_f =
        Option<unsafe extern "C" fn(*mut AANode, *mut ::core::ffi::c_void) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "57:1"]
    pub struct AANode {
        pub left: *mut AANode,
        pub right: *mut AANode,
        pub level: ::core::ffi::c_int,
    }
    #[c2rust::src_loc = "33:1"]
    pub type aatree_cmp_f =
        Option<unsafe extern "C" fn(uintptr_t, *mut AANode) -> ::core::ffi::c_int>;
    use super::_uintptr_t_h::uintptr_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_sa_family_t.h:27"]
pub mod _sa_family_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type sa_family_t = __uint8_t;
    use super::_types_h::__uint8_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_socklen_t.h:27"]
pub mod _socklen_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type socklen_t = __darwin_socklen_t;
    use super::_types_h::__darwin_socklen_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/socket.h:27"]
pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "414:1"]
    pub struct sockaddr {
        pub sa_len: __uint8_t,
        pub sa_family: sa_family_t,
        pub sa_data: [::core::ffi::c_char; 14],
    }
    #[c2rust::src_loc = "111:9"]
    pub const SOCK_STREAM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "161:9"]
    pub const SO_ERROR: ::core::ffi::c_int = 0x1007 as ::core::ffi::c_int;
    #[c2rust::src_loc = "354:9"]
    pub const SOL_SOCKET: ::core::ffi::c_int = 0xffff as ::core::ffi::c_int;
    #[c2rust::src_loc = "361:9"]
    pub const AF_UNIX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    use super::_sa_family_t_h::sa_family_t;
    use super::_socklen_t_h::socklen_t;
    use super::_types_h::__uint8_t;
    extern "C" {
        #[c2rust::src_loc = "716:1"]
        pub fn getsockopt(
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_int,
            _: *mut ::core::ffi::c_void,
            _: *mut socklen_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "729:1"]
        pub fn socket(
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet/in.h:27"]
pub mod in_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "301:1"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "374:1"]
    pub struct sockaddr_in {
        pub sin_len: __uint8_t,
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [::core::ffi::c_char; 8],
    }
    use super::_in_addr_t_h::in_addr_t;
    use super::_in_port_t_h::in_port_t;
    use super::_sa_family_t_h::sa_family_t;
    use super::_types_h::__uint8_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet6/in6.h:27"]
pub mod in6_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "152:9"]
    pub struct in6_addr {
        pub __u6_addr: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "153:2"]
    pub union C2RustUnnamed {
        pub __u6_addr8: [__uint8_t; 16],
        pub __u6_addr16: [__uint16_t; 8],
        pub __u6_addr32: [__uint32_t; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "170:1"]
    pub struct sockaddr_in6 {
        pub sin6_len: __uint8_t,
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: __uint32_t,
        pub sin6_addr: in6_addr,
        pub sin6_scope_id: __uint32_t,
    }
    use super::_in_port_t_h::in_port_t;
    use super::_sa_family_t_h::sa_family_t;
    use super::_types_h::{__uint16_t, __uint32_t, __uint8_t};
}
#[c2rust::header_src = "/opt/homebrew/Cellar/libevent/2.1.12_1/include/event2/event.h:27"]
pub mod event_h {
    #[c2rust::src_loc = "1014:1"]
    pub type event_callback_fn = Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_short,
            *mut ::core::ffi::c_void,
        ) -> (),
    >;
    #[c2rust::src_loc = "923:9"]
    pub const EV_READ: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "925:9"]
    pub const EV_WRITE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    #[c2rust::src_loc = "934:9"]
    pub const EV_PERSIST: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    use super::_timeval_h::timeval;
    use super::event_struct_h::event;
    extern "C" {
        #[c2rust::src_loc = "217:1"]
        pub type event_base;
        #[c2rust::src_loc = "1132:1"]
        pub fn event_assign(
            _: *mut event,
            _: *mut event_base,
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_short,
            _: event_callback_fn,
            _: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1233:1"]
        pub fn event_add(ev: *mut event, timeout: *const timeval) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1259:1"]
        pub fn event_del(_: *mut event) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/libevent/2.1.12_1/include/event2/event_struct.h:27"]
pub mod event_struct_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "123:1"]
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
    #[c2rust::src_loc = "135:2"]
    pub union C2RustUnnamed_0 {
        pub ev_io: C2RustUnnamed_3,
        pub ev_signal: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "143:3"]
    pub struct C2RustUnnamed_1 {
        pub ev_signal_next: C2RustUnnamed_2,
        pub ev_ncalls: ::core::ffi::c_short,
        pub ev_pncalls: *mut ::core::ffi::c_short,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "144:4"]
    pub struct C2RustUnnamed_2 {
        pub le_next: *mut event,
        pub le_prev: *mut *mut event,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "137:3"]
    pub struct C2RustUnnamed_3 {
        pub ev_io_next: C2RustUnnamed_4,
        pub ev_timeout: timeval,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "138:4"]
    pub struct C2RustUnnamed_4 {
        pub le_next: *mut event,
        pub le_prev: *mut *mut event,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "127:2"]
    pub union C2RustUnnamed_5 {
        pub ev_next_with_common_timeout: C2RustUnnamed_6,
        pub min_heap_idx: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "128:3"]
    pub struct C2RustUnnamed_6 {
        pub tqe_next: *mut event,
        pub tqe_prev: *mut *mut event,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "107:1"]
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
    #[c2rust::src_loc = "113:9"]
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
    #[c2rust::src_loc = "108:2"]
    pub struct C2RustUnnamed_8 {
        pub tqe_next: *mut event_callback,
        pub tqe_prev: *mut *mut event_callback,
    }
    use super::_timeval_h::timeval;
    use super::_uint8_t_h::uint8_t;
    use super::event_h::event_base;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/cryptohash.h:27"]
pub mod cryptohash_h {
    #[c2rust::src_loc = "19:9"]
    pub type pg_cryptohash_type = ::core::ffi::c_uint;
    #[c2rust::src_loc = "24:2"]
    pub const PG_SHA512: pg_cryptohash_type = 3;
    #[c2rust::src_loc = "23:2"]
    pub const PG_SHA384: pg_cryptohash_type = 2;
    #[c2rust::src_loc = "22:2"]
    pub const PG_SHA256: pg_cryptohash_type = 1;
    #[c2rust::src_loc = "21:2"]
    pub const PG_SHA224: pg_cryptohash_type = 0;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/uthash.h:27"]
pub mod uthash_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1072:9"]
    pub struct UT_hash_bucket {
        pub hh_head: *mut UT_hash_handle,
        pub count: ::core::ffi::c_uint,
        pub expand_mult: ::core::ffi::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1129:9"]
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
    #[c2rust::src_loc = "1096:9"]
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
    use super::_ptrdiff_t_h::ptrdiff_t;
    use super::_uint32_t_h::uint32_t;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/bouncer.h:27"]
pub mod bouncer_h {
    #[c2rust::src_loc = "70:1"]
    pub type SocketState = ::core::ffi::c_uint;
    #[c2rust::src_loc = "88:2"]
    pub const SV_TESTED: SocketState = 16;
    #[c2rust::src_loc = "87:2"]
    pub const SV_USED: SocketState = 15;
    #[c2rust::src_loc = "86:2"]
    pub const SV_ACTIVE_CANCEL: SocketState = 14;
    #[c2rust::src_loc = "85:2"]
    pub const SV_ACTIVE: SocketState = 13;
    #[c2rust::src_loc = "84:2"]
    pub const SV_IDLE: SocketState = 12;
    #[c2rust::src_loc = "83:2"]
    pub const SV_BEING_CANCELED: SocketState = 11;
    #[c2rust::src_loc = "82:2"]
    pub const SV_LOGIN: SocketState = 10;
    #[c2rust::src_loc = "81:2"]
    pub const SV_JUSTFREE: SocketState = 9;
    #[c2rust::src_loc = "80:2"]
    pub const SV_FREE: SocketState = 8;
    #[c2rust::src_loc = "78:2"]
    pub const CL_ACTIVE_CANCEL: SocketState = 7;
    #[c2rust::src_loc = "77:2"]
    pub const CL_WAITING_CANCEL: SocketState = 6;
    #[c2rust::src_loc = "76:2"]
    pub const CL_ACTIVE: SocketState = 5;
    #[c2rust::src_loc = "75:2"]
    pub const CL_WAITING_LOGIN: SocketState = 4;
    #[c2rust::src_loc = "74:2"]
    pub const CL_WAITING: SocketState = 3;
    #[c2rust::src_loc = "73:2"]
    pub const CL_LOGIN: SocketState = 2;
    #[c2rust::src_loc = "72:2"]
    pub const CL_JUSTFREE: SocketState = 1;
    #[c2rust::src_loc = "71:2"]
    pub const CL_FREE: SocketState = 0;
    #[c2rust::src_loc = "91:1"]
    pub type PauseMode = ::core::ffi::c_uint;
    #[c2rust::src_loc = "94:2"]
    pub const P_SUSPEND: PauseMode = 2;
    #[c2rust::src_loc = "93:2"]
    pub const P_PAUSE: PauseMode = 1;
    #[c2rust::src_loc = "92:2"]
    pub const P_NONE: PauseMode = 0;
    #[c2rust::src_loc = "122:1"]
    pub type SSLMode = ::core::ffi::c_uint;
    #[c2rust::src_loc = "128:2"]
    pub const SSLMODE_VERIFY_FULL: SSLMode = 5;
    #[c2rust::src_loc = "127:2"]
    pub const SSLMODE_VERIFY_CA: SSLMode = 4;
    #[c2rust::src_loc = "126:2"]
    pub const SSLMODE_REQUIRE: SSLMode = 3;
    #[c2rust::src_loc = "125:2"]
    pub const SSLMODE_PREFER: SSLMode = 2;
    #[c2rust::src_loc = "124:2"]
    pub const SSLMODE_ALLOW: SSLMode = 1;
    #[c2rust::src_loc = "123:2"]
    pub const SSLMODE_DISABLED: SSLMode = 0;
    #[c2rust::src_loc = "131:1"]
    pub type PacketCallbackFlag = ::core::ffi::c_uint;
    #[c2rust::src_loc = "144:2"]
    pub const CB_HANDLE_COMPLETE_PACKET: PacketCallbackFlag = 2;
    #[c2rust::src_loc = "139:2"]
    pub const CB_WANT_COMPLETE_PACKET: PacketCallbackFlag = 1;
    #[c2rust::src_loc = "133:2"]
    pub const CB_NONE: PacketCallbackFlag = 0;
    #[c2rust::src_loc = "147:1"]
    pub type LoadBalanceHosts = ::core::ffi::c_uint;
    #[c2rust::src_loc = "149:2"]
    pub const LOAD_BALANCE_HOSTS_ROUND_ROBIN: LoadBalanceHosts = 1;
    #[c2rust::src_loc = "148:2"]
    pub const LOAD_BALANCE_HOSTS_DISABLE: LoadBalanceHosts = 0;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "663:1"]
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
    #[c2rust::src_loc = "768:2"]
    pub struct CallbackState {
        #[bitfield(name = "flag", ty = "PacketCallbackFlag", bits = "0..=7")]
        pub flag: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
        pub pkt: PktHdr,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "732:2"]
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
    #[c2rust::src_loc = "727:2"]
    pub union C2RustUnnamed_9 {
        pub dns_token: *mut DNSToken,
        pub db: *mut PgDatabase,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "570:1"]
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
    #[c2rust::src_loc = "509:1"]
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
    #[c2rust::src_loc = "549:1"]
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
    #[c2rust::src_loc = "345:1"]
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "320:1"]
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "292:1"]
    pub union PgAddr {
        pub sa: sockaddr,
        pub sin: sockaddr_in,
        pub sin6: sockaddr_in6,
        pub scred: sockaddr_ucreds,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "282:1"]
    pub struct sockaddr_ucreds {
        pub sin: sockaddr_in,
        pub uid: uid_t,
        pub pid: pid_t,
    }
    #[c2rust::src_loc = "650:1"]
    pub type ReplicationType = ::core::ffi::c_uint;
    #[c2rust::src_loc = "653:2"]
    pub const REPLICATION_PHYSICAL: ReplicationType = 2;
    #[c2rust::src_loc = "652:2"]
    pub const REPLICATION_LOGICAL: ReplicationType = 1;
    #[c2rust::src_loc = "651:2"]
    pub const REPLICATION_NONE: ReplicationType = 0;
    #[c2rust::src_loc = "237:2"]
    pub const AUTH_TYPE_CERT: auth_type = 4;
    #[c2rust::src_loc = "232:1"]
    pub type auth_type = ::core::ffi::c_uint;
    #[c2rust::src_loc = "243:2"]
    pub const AUTH_TYPE_REJECT: auth_type = 10;
    #[c2rust::src_loc = "242:2"]
    pub const AUTH_TYPE_PEER: auth_type = 9;
    #[c2rust::src_loc = "241:2"]
    pub const AUTH_TYPE_SCRAM_SHA_256: auth_type = 8;
    #[c2rust::src_loc = "240:2"]
    pub const AUTH_TYPE_PAM: auth_type = 7;
    #[c2rust::src_loc = "239:2"]
    pub const AUTH_TYPE_LDAP: auth_type = 6;
    #[c2rust::src_loc = "238:2"]
    pub const AUTH_TYPE_HBA: auth_type = 5;
    #[c2rust::src_loc = "236:2"]
    pub const AUTH_TYPE_MD5: auth_type = 3;
    #[c2rust::src_loc = "235:2"]
    pub const AUTH_TYPE_PLAIN: auth_type = 2;
    #[c2rust::src_loc = "234:2"]
    pub const AUTH_TYPE_TRUST: auth_type = 1;
    #[c2rust::src_loc = "233:2"]
    pub const AUTH_TYPE_ANY: auth_type = 0;
    use super::_pid_t_h::pid_t;
    use super::_uid_t_h::uid_t;
    use super::_uint16_t_h::uint16_t;
    use super::_uint64_t_h::uint64_t;
    use super::_uint8_t_h::uint8_t;
    use super::aatree_h::{AANode, AATree};
    use super::cryptohash_h::pg_cryptohash_type;
    use super::dnslookup_h::DNSToken;
    use super::event_h::event_base;
    use super::in6_h::sockaddr_in6;
    use super::in_h::sockaddr_in;
    use super::list_h::List;
    use super::pktbuf_h::PktBuf;
    use super::prepare_h::{PgClientPreparedStatement, PgServerPreparedStatement};
    use super::proto_h::PktHdr;
    use super::sbuf_h::SBuf;
    use super::socket_h::sockaddr;
    use super::statlist_h::StatList;
    use super::time_h::usec_t;
    use super::varcache_h::VarCache;
    extern "C" {
        #[c2rust::src_loc = "66:1"]
        pub static mut pgb_event_base: *mut event_base;
        #[c2rust::src_loc = "171:1"]
        pub static mut cf_sbuf_len: ::core::ffi::c_int;
        #[c2rust::src_loc = "851:1"]
        pub static mut cf_auth_type: ::core::ffi::c_int;
        #[c2rust::src_loc = "868:1"]
        pub static mut cf_pause_mode: ::core::ffi::c_int;
        #[c2rust::src_loc = "870:1"]
        pub static mut cf_reboot: ::core::ffi::c_int;
        #[c2rust::src_loc = "874:1"]
        pub static mut cf_sbuf_loopcnt: ::core::ffi::c_int;
        #[c2rust::src_loc = "881:1"]
        pub static mut cf_tcp_defer_accept: ::core::ffi::c_int;
        #[c2rust::src_loc = "889:1"]
        pub static mut cf_client_tls_sslmode: ::core::ffi::c_int;
        #[c2rust::src_loc = "890:1"]
        pub static mut cf_client_tls_protocols: *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "891:1"]
        pub static mut cf_client_tls_ca_file: *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "892:1"]
        pub static mut cf_client_tls_cert_file: *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "893:1"]
        pub static mut cf_client_tls_key_file: *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "894:1"]
        pub static mut cf_client_tls_ciphers: *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "895:1"]
        pub static mut cf_client_tls13_ciphers: *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "896:1"]
        pub static mut cf_client_tls_dheparams: *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "897:1"]
        pub static mut cf_client_tls_ecdhecurve: *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "899:1"]
        pub static mut cf_server_tls_sslmode: ::core::ffi::c_int;
        #[c2rust::src_loc = "900:1"]
        pub static mut cf_server_tls_protocols: *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "901:1"]
        pub static mut cf_server_tls_ca_file: *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "902:1"]
        pub static mut cf_server_tls_cert_file: *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "903:1"]
        pub static mut cf_server_tls_key_file: *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "904:1"]
        pub static mut cf_server_tls_ciphers: *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "905:1"]
        pub static mut cf_server_tls13_ciphers: *mut ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/sbuf.h:27"]
pub mod sbuf_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "71:1"]
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
    #[c2rust::src_loc = "58:1"]
    pub struct SBufIO {
        pub sbufio_peek:
            Option<unsafe extern "C" fn(*mut SBuf, *mut ::core::ffi::c_void, size_t) -> ssize_t>,
        pub sbufio_recv:
            Option<unsafe extern "C" fn(*mut SBuf, *mut ::core::ffi::c_void, size_t) -> ssize_t>,
        pub sbufio_send:
            Option<unsafe extern "C" fn(*mut SBuf, *const ::core::ffi::c_void, size_t) -> ssize_t>,
        pub sbufio_close: Option<unsafe extern "C" fn(*mut SBuf) -> ::core::ffi::c_int>,
    }
    #[c2rust::src_loc = "54:1"]
    pub type sbuf_cb_t = Option<unsafe extern "C" fn(*mut SBuf, SBufEvent, *mut MBuf) -> bool>;
    #[c2rust::src_loc = "24:9"]
    pub type SBufEvent = ::core::ffi::c_uint;
    #[c2rust::src_loc = "32:2"]
    pub const SBUF_EV_TLS_READY: SBufEvent = 7;
    #[c2rust::src_loc = "31:2"]
    pub const SBUF_EV_PKT_CALLBACK: SBufEvent = 6;
    #[c2rust::src_loc = "30:2"]
    pub const SBUF_EV_FLUSH: SBufEvent = 5;
    #[c2rust::src_loc = "29:2"]
    pub const SBUF_EV_CONNECT_OK: SBufEvent = 4;
    #[c2rust::src_loc = "28:2"]
    pub const SBUF_EV_CONNECT_FAILED: SBufEvent = 3;
    #[c2rust::src_loc = "27:2"]
    pub const SBUF_EV_SEND_FAILED: SBufEvent = 2;
    #[c2rust::src_loc = "26:2"]
    pub const SBUF_EV_RECV_FAILED: SBufEvent = 1;
    #[c2rust::src_loc = "25:2"]
    pub const SBUF_EV_READ: SBufEvent = 0;
    #[c2rust::src_loc = "43:9"]
    pub const SBUF_SMALL_PKT: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
    #[inline]
    #[c2rust::src_loc = "144:1"]
    pub unsafe extern "C" fn sbuf_is_empty(mut sbuf: *mut SBuf) -> bool {
        iobuf_empty((*sbuf).io) as ::core::ffi::c_int != 0
            && (*sbuf).pkt_remain == 0 as ::core::ffi::c_uint
    }
    #[inline]
    #[c2rust::src_loc = "158:1"]
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
    #[c2rust::src_loc = "163:1"]
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
    #[c2rust::src_loc = "168:1"]
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
    #[c2rust::src_loc = "173:1"]
    pub unsafe extern "C" fn sbuf_op_close(mut sbuf: *mut SBuf) -> ::core::ffi::c_int {
        (*(*sbuf).ops)
            .sbufio_close
            .expect("non-null function pointer")(sbuf)
    }
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    use super::_uint8_t_h::uint8_t;
    use super::event_struct_h::event;
    use super::iobuf_h::{iobuf_empty, IOBuf};
    use super::mbuf_h::MBuf;
    use super::tls_h::tls;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/iobuf.h:27"]
pub mod iobuf_h {
    #[c2rust::src_loc = "55:1"]
    pub type IOBuf = iobuf;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:1"]
    pub struct iobuf {
        pub done_pos: ::core::ffi::c_uint,
        pub parse_pos: ::core::ffi::c_uint,
        pub recv_pos: ::core::ffi::c_uint,
        pub buf: [uint8_t; 0],
    }
    #[inline]
    #[c2rust::src_loc = "65:1"]
    pub unsafe extern "C" fn iobuf_empty(mut io: *const IOBuf) -> bool {
        io.is_null() || (*io).done_pos == (*io).recv_pos
    }
    #[inline]
    #[c2rust::src_loc = "71:1"]
    pub unsafe extern "C" fn iobuf_amount_pending(mut buf: *const IOBuf) -> ::core::ffi::c_uint {
        (*buf).parse_pos.wrapping_sub((*buf).done_pos)
    }
    #[inline]
    #[c2rust::src_loc = "77:1"]
    pub unsafe extern "C" fn iobuf_amount_parse(mut buf: *const IOBuf) -> ::core::ffi::c_uint {
        (*buf).recv_pos.wrapping_sub((*buf).parse_pos)
    }
    #[inline]
    #[c2rust::src_loc = "83:1"]
    pub unsafe extern "C" fn iobuf_amount_recv(mut buf: *const IOBuf) -> ::core::ffi::c_uint {
        (cf_sbuf_len as ::core::ffi::c_uint).wrapping_sub((*buf).recv_pos)
    }
    #[inline]
    #[c2rust::src_loc = "89:1"]
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
    #[c2rust::src_loc = "98:1"]
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
    #[c2rust::src_loc = "108:1"]
    pub unsafe extern "C" fn iobuf_tag_send(mut io: *mut IOBuf, mut len: ::core::ffi::c_uint) {
        (*io).parse_pos = (*io).parse_pos.wrapping_add(len);
    }
    #[inline]
    #[c2rust::src_loc = "115:1"]
    pub unsafe extern "C" fn iobuf_tag_skip(mut io: *mut IOBuf, mut len: ::core::ffi::c_uint) {
        (*io).parse_pos = (*io).parse_pos.wrapping_add(len);
        (*io).done_pos = (*io).parse_pos;
    }
    #[inline]
    #[c2rust::src_loc = "124:1"]
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
            (*io).parse_pos = (*io).parse_pos.wrapping_sub((*io).done_pos);
            (*io).recv_pos = avail;
            (*io).done_pos = 0 as ::core::ffi::c_uint;
        }
    }
    #[inline]
    #[c2rust::src_loc = "138:1"]
    pub unsafe extern "C" fn iobuf_reset(mut io: *mut IOBuf) {
        (*io).done_pos = 0 as ::core::ffi::c_uint;
        (*io).parse_pos = (*io).done_pos;
        (*io).recv_pos = (*io).parse_pos;
    }
    use super::_size_t_h::size_t;
    use super::_string_h::memmove;
    use super::_uint8_t_h::uint8_t;
    use super::bouncer_h::cf_sbuf_len;
    use super::mbuf_h::{mbuf_init_fixed_reader, MBuf};
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/mbuf.h:27"]
pub mod mbuf_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "14:1"]
    pub struct MBuf {
        pub data: *mut uint8_t,
        pub read_pos: ::core::ffi::c_uint,
        pub write_pos: ::core::ffi::c_uint,
        pub alloc_len: ::core::ffi::c_uint,
        pub reader: bool,
        pub fixed: bool,
    }
    #[inline]
    #[c2rust::src_loc = "33:1"]
    pub unsafe extern "C" fn mbuf_init_fixed_reader(
        mut buf: *mut MBuf,
        mut ptr: *const ::core::ffi::c_void,
        mut len: ::core::ffi::c_uint,
    ) {
        (*buf).data = ptr as *mut uint8_t;
        (*buf).read_pos = 0 as ::core::ffi::c_uint;
        (*buf).write_pos = len;
        (*buf).alloc_len = len;
        (*buf).reader = true_0 != 0;
        (*buf).fixed = true_0 != 0;
    }
    #[inline]
    #[c2rust::src_loc = "66:1"]
    pub unsafe extern "C" fn mbuf_free(mut buf: *mut MBuf) {
        if !(*buf).data.is_null() {
            if !(*buf).fixed {
                free((*buf).data as *mut ::core::ffi::c_void);
            }
            memset(
                buf as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<MBuf>() as size_t,
            );
        }
    }
    #[inline]
    #[c2rust::src_loc = "86:1"]
    pub unsafe extern "C" fn mbuf_rewind_writer(mut buf: *mut MBuf) {
        if !(*buf).reader {
            (*buf).read_pos = 0 as ::core::ffi::c_uint;
            (*buf).write_pos = 0 as ::core::ffi::c_uint;
        }
    }
    #[inline]
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn mbuf_avail_for_read(mut buf: *const MBuf) -> ::core::ffi::c_uint {
        (*buf).write_pos.wrapping_sub((*buf).read_pos)
    }
    #[inline]
    #[c2rust::src_loc = "263:1"]
    pub unsafe extern "C" fn mbuf_write(
        mut buf: *mut MBuf,
        mut ptr: *const ::core::ffi::c_void,
        mut len: ::core::ffi::c_uint,
    ) -> bool {
        if (*buf).write_pos.wrapping_add(len) > (*buf).alloc_len && !mbuf_make_room(buf, len) {
            return false_0 != 0;
        }
        if len > 0 as ::core::ffi::c_uint {
            memcpy(
                (*buf).data.offset((*buf).write_pos as isize) as *mut ::core::ffi::c_void,
                ptr,
                len as size_t,
            );
        }
        (*buf).write_pos = (*buf).write_pos.wrapping_add(len);
        true_0 != 0
    }
    use super::_malloc_h::free;
    use super::_size_t_h::size_t;
    use super::_string_h::{memcpy, memset};
    use super::_uint8_t_h::uint8_t;
    use super::stdbool_h::{false_0, true_0};
    extern "C" {
        #[c2rust::src_loc = "248:1"]
        pub fn mbuf_make_room(buf: *mut MBuf, len: ::core::ffi::c_uint) -> bool;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/proto.h:27"]
pub mod proto_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "34:1"]
    pub struct PktHdr {
        pub type_0: ::core::ffi::c_uint,
        pub len: ::core::ffi::c_uint,
        pub data: MBuf,
    }
    use super::mbuf_h::MBuf;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/prepare.h:27"]
pub mod prepare_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:9"]
    pub struct PgServerPreparedStatement {
        pub query_id: uint64_t,
        pub hh: UT_hash_handle,
        pub ps: *mut PgPreparedStatement,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:9"]
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
    #[c2rust::src_loc = "21:9"]
    pub struct PgClientPreparedStatement {
        pub hh: UT_hash_handle,
        pub ps: *mut PgPreparedStatement,
        pub stmt_name: [::core::ffi::c_char; 0],
    }
    use super::_size_t_h::size_t;
    use super::_uint32_t_h::uint32_t;
    use super::_uint64_t_h::uint64_t;
    use super::_uint8_t_h::uint8_t;
    use super::uthash_h::UT_hash_handle;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/varcache.h:27"]
pub mod varcache_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "14:1"]
    pub struct VarCache {
        pub var_list: *mut *mut PStr,
    }
    use super::strpool_h::PStr;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/strpool.h:27"]
pub mod strpool_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:1"]
    pub struct PStr {
        pub pool: *mut StrPool,
        pub len: size_t,
        pub refcnt: ::core::ffi::c_int,
        pub str_0: [::core::ffi::c_char; 0],
    }
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "34:1"]
        pub type StrPool;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/pktbuf.h:27"]
pub mod pktbuf_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "26:1"]
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
    extern "C" {
        #[c2rust::src_loc = "47:1"]
        pub fn pktbuf_free(buf: *mut PktBuf);
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/dnslookup.h:27"]
pub mod dnslookup_h {
    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub type DNSToken;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/logging.h:27"]
pub mod logging_h {
    #[c2rust::src_loc = "47:1"]
    pub type LogLevel = ::core::ffi::c_uint;
    #[c2rust::src_loc = "54:2"]
    pub const LG_NOISE: LogLevel = 6;
    #[c2rust::src_loc = "53:2"]
    pub const LG_DEBUG: LogLevel = 5;
    #[c2rust::src_loc = "52:2"]
    pub const LG_INFO: LogLevel = 4;
    #[c2rust::src_loc = "51:2"]
    pub const LG_STATS: LogLevel = 3;
    #[c2rust::src_loc = "50:2"]
    pub const LG_WARNING: LogLevel = 2;
    #[c2rust::src_loc = "49:2"]
    pub const LG_ERROR: LogLevel = 1;
    #[c2rust::src_loc = "48:2"]
    pub const LG_FATAL: LogLevel = 0;
    extern "C" {
        #[c2rust::src_loc = "85:1"]
        pub static mut cf_verbose: ::core::ffi::c_int;
        #[c2rust::src_loc = "117:1"]
        pub fn log_generic(
            level: LogLevel,
            ctx: *mut ::core::ffi::c_void,
            s: *const ::core::ffi::c_char,
            ...
        );
        #[c2rust::src_loc = "120:1"]
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
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/objects.h:27"]
pub mod objects_h {
    use super::bouncer_h::{PgPool, PgSocket};

    use super::statlist_h::StatList;
    extern "C" {
        #[c2rust::src_loc = "27:8"]
        pub type Slab;
        #[c2rust::src_loc = "21:1"]
        pub static mut pool_list: StatList;
        #[c2rust::src_loc = "35:1"]
        pub static mut iobuf_cache: *mut Slab;
        #[c2rust::src_loc = "62:1"]
        pub fn disconnect_client(
            client: *mut PgSocket,
            notify: bool,
            reason: *const ::core::ffi::c_char,
            ...
        );
        #[c2rust::src_loc = "105:1"]
        pub fn tag_pool_dirty(pool: *mut PgPool);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:27"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/socket.h:27"]
pub mod usual_socket_h {
    use super::_size_t_h::size_t;
    use super::socket_h::sockaddr;
    extern "C" {
        #[c2rust::src_loc = "103:1"]
        pub fn sa2str(
            sa: *const sockaddr,
            buf: *mut ::core::ffi::c_char,
            buflen: size_t,
        ) -> *const ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:27"]
pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "77:1"]
        pub fn memcpy(
            __dst: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "80:1"]
        pub fn memmove(
            __dst: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __len: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "83:1"]
        pub fn memset(
            __b: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __len: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "95:1"]
        pub fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/safeio.h:29"]
pub mod safeio_h {
    use super::_size_t_h::size_t;
    use super::_socklen_t_h::socklen_t;
    use super::_ssize_t_h::ssize_t;
    use super::socket_h::sockaddr;
    extern "C" {
        #[c2rust::src_loc = "31:1"]
        pub fn safe_recv(
            fd: ::core::ffi::c_int,
            buf: *mut ::core::ffi::c_void,
            len: size_t,
            flags: ::core::ffi::c_int,
        ) -> ssize_t;
        #[c2rust::src_loc = "33:1"]
        pub fn safe_send(
            fd: ::core::ffi::c_int,
            buf: *const ::core::ffi::c_void,
            len: size_t,
            flags: ::core::ffi::c_int,
        ) -> ssize_t;
        #[c2rust::src_loc = "35:1"]
        pub fn safe_close(fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "41:1"]
        pub fn safe_connect(
            fd: ::core::ffi::c_int,
            sa: *const sockaddr,
            sa_len: socklen_t,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/slab.h:30"]
pub mod slab_h {
    use super::objects_h::Slab;
    extern "C" {
        #[c2rust::src_loc = "52:1"]
        pub fn slab_alloc(slab: *mut Slab) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "55:1"]
        pub fn slab_free(slab: *mut Slab, obj: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:27"]
pub mod _malloc_h {
    extern "C" {
        #[c2rust::src_loc = "56:1"]
        pub fn free(_: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdlib.h:27"]
pub mod _stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "160:1"]
        pub fn exit(_: ::core::ffi::c_int) -> !;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/errno.h:27"]
pub mod errno_h {
    #[c2rust::src_loc = "92:9"]
    pub const EIO: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    #[c2rust::src_loc = "129:9"]
    pub const EAGAIN: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
    #[c2rust::src_loc = "131:9"]
    pub const EINPROGRESS: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
    extern "C" {
        #[c2rust::src_loc = "80:1"]
        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/llvm/21.1.8/lib/clang/21/include/stdbool.h:27"]
pub mod stdbool_h {
    #[c2rust::src_loc = "25:9"]
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/util.h:27"]
pub mod util_h {
    extern "C" {
        #[c2rust::src_loc = "49:1"]
        pub fn tune_socket(sock: ::core::ffi::c_int, is_unix: bool) -> bool;
    }
}
pub use self::_in_addr_t_h::in_addr_t;
pub use self::_in_port_t_h::in_port_t;

pub use self::_null_h::NULL;
pub use self::_pid_t_h::pid_t;
pub use self::_ptrdiff_t_h::ptrdiff_t;
pub use self::_sa_family_t_h::sa_family_t;
pub use self::_size_t_h::size_t;
pub use self::_socklen_t_h::socklen_t;
pub use self::_ssize_t_h::ssize_t;
use self::_stdlib_h::exit;
use self::_string_h::{memset, strerror};
pub use self::_time_t_h::time_t;
pub use self::_timeval_h::timeval;
pub use self::_types_h::{
    __darwin_ptrdiff_t, __darwin_size_t, __darwin_socklen_t, __darwin_ssize_t, __darwin_time_t,
    __int32_t, __uint16_t, __uint32_t, __uint8_t,
};
pub use self::_uid_t_h::uid_t;
pub use self::_uint16_t_h::uint16_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::aatree_h::{aatree_cmp_f, aatree_walker_f, AANode, AATree};
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
pub use self::cryptohash_h::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};

pub use self::errno_h::{__error, EAGAIN, EINPROGRESS, EIO};
pub use self::event_h::{
    event_add, event_assign, event_base, event_callback_fn, event_del, EV_PERSIST, EV_READ,
    EV_WRITE,
};
pub use self::event_struct_h::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
pub use self::in6_h::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use self::in_h::{in_addr, sockaddr_in};
pub use self::iobuf_h::{
    iobuf, iobuf_amount_parse, iobuf_amount_pending, iobuf_amount_recv, iobuf_empty,
    iobuf_parse_all, iobuf_parse_limit, iobuf_reset, iobuf_tag_send, iobuf_tag_skip,
    iobuf_try_resync, IOBuf,
};
pub use self::list_h::List;
pub use self::logging_h::{
    cf_verbose, log_fatal, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE,
    LG_STATS, LG_WARNING,
};
pub use self::mbuf_h::{
    mbuf_avail_for_read, mbuf_free, mbuf_init_fixed_reader, mbuf_make_room, mbuf_rewind_writer,
    mbuf_write, MBuf,
};
use self::objects_h::{disconnect_client, iobuf_cache, pool_list, tag_pool_dirty};
pub use self::pktbuf_h::{pktbuf_free, PktBuf};
pub use self::prepare_h::{
    PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement,
};
pub use self::proto_h::PktHdr;
use self::safeio_h::{safe_close, safe_connect, safe_recv, safe_send};
pub use self::sbuf_h::{
    sbuf_cb_t, sbuf_is_empty, sbuf_op_close, sbuf_op_peek, sbuf_op_recv, sbuf_op_send, SBuf,
    SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK, SBUF_EV_FLUSH,
    SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY, SBUF_SMALL_PKT,
};
use self::slab_h::{slab_alloc, slab_free};
pub use self::socket_h::{
    getsockopt, sockaddr, socket, AF_UNIX, SOCK_STREAM, SOL_SOCKET, SO_ERROR,
};
pub use self::statlist_h::StatList;
pub use self::stdbool_h::{false_0, true_0};
pub use self::strpool_h::{PStr, StrPool};
pub use self::sys__types_h::{__darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t, __DARWIN_NULL};
pub use self::time_h::usec_t;
pub use self::tls_h::{
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
use self::usual_socket_h::sa2str;
pub use self::uthash_h::{UT_hash_bucket, UT_hash_handle, UT_hash_table};
use self::util_h::tune_socket;
pub use self::varcache_h::VarCache;
#[c2rust::src_loc = "51:2"]
pub const SBUF_TLS_OK: TLSState = 3;
#[c2rust::src_loc = "55:2"]
pub const W_NONE: WaitType = 0;
#[c2rust::src_loc = "59:2"]
pub const W_ONCE: WaitType = 4;
#[c2rust::src_loc = "50:2"]
pub const SBUF_TLS_IN_HANDSHAKE: TLSState = 2;
#[c2rust::src_loc = "49:2"]
pub const SBUF_TLS_DO_HANDSHAKE: TLSState = 1;
#[c2rust::src_loc = "58:2"]
pub const W_SEND: WaitType = 3;
#[c2rust::src_loc = "57:2"]
pub const W_RECV: WaitType = 2;
#[c2rust::src_loc = "56:2"]
pub const W_CONNECT: WaitType = 1;
#[c2rust::src_loc = "47:1"]
pub type TLSState = ::core::ffi::c_uint;
#[c2rust::src_loc = "48:2"]
pub const SBUF_TLS_NONE: TLSState = 0;
#[c2rust::src_loc = "54:1"]
pub type WaitType = ::core::ffi::c_uint;
#[c2rust::src_loc = "39:9"]
pub const DO_RECV: ::core::ffi::c_int = false_0;
#[c2rust::src_loc = "40:9"]
pub const SKIP_RECV: ::core::ffi::c_int = true_0;
#[c2rust::src_loc = "43:9"]
pub const ACT_SEND: ::core::ffi::c_int = 1;
#[c2rust::src_loc = "44:9"]
pub const ACT_SKIP: ::core::ffi::c_int = 2;
#[c2rust::src_loc = "45:9"]
pub const ACT_CALL: ::core::ffi::c_int = 3;
#[c2rust::src_loc = "92:1"]
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
#[c2rust::src_loc = "105:1"]
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
#[c2rust::src_loc = "122:1"]
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
#[c2rust::src_loc = "130:1"]
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
                        return false_0 != 0;
                    }
                }
                current_block = 2979737022853876585;
            }
        } else {
            current_block = 2979737022853876585;
        }
        match current_block {
            2269420068866141602 => {}
            _ => return true_0 != 0,
        }
    }
    sbuf_call_proto(sbuf, SBUF_EV_RECV_FAILED as ::core::ffi::c_int);
    false_0 != 0
}
#[no_mangle]
#[c2rust::src_loc = "161:1"]
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
            return true_0 != 0;
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
                return true_0 != 0;
            }
        }
    }
    let mut _log_ctx = NULL;
    log_generic(
        LG_WARNING,
        _log_ctx,
        b"sbuf_connect failed to connect to %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
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
    false_0 != 0
}
#[no_mangle]
#[c2rust::src_loc = "216:1"]
pub unsafe extern "C" fn sbuf_pause(mut sbuf: *mut SBuf) -> bool {
    if event_del(&raw mut (*sbuf).ev) < 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            b"event_del: %s\0" as *const u8 as *const ::core::ffi::c_char,
            strerror(*__error()),
        );
        return false_0 != 0;
    }
    (*sbuf).wait_type = W_NONE as ::core::ffi::c_int as uint8_t;
    true_0 != 0
}
#[no_mangle]
#[c2rust::src_loc = "230:1"]
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
#[c2rust::src_loc = "263:1"]
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
            b"sbuf_continue_with_callback: %s\0" as *const u8 as *const ::core::ffi::c_char,
            strerror(*__error()),
        );
        return false_0 != 0;
    }
    (*sbuf).wait_type = W_RECV as ::core::ffi::c_int as uint8_t;
    true_0 != 0
}
#[no_mangle]
#[c2rust::src_loc = "281:1"]
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
                b"sbuf_queue_once: event_del failed: %s\0" as *const u8
                    as *const ::core::ffi::c_char,
                strerror(*__error()),
            );
            return false_0 != 0;
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
            b"sbuf_queue_once: event_add failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
            strerror(*__error()),
        );
        return false_0 != 0;
    }
    (*sbuf).wait_type = W_ONCE as ::core::ffi::c_int as uint8_t;
    true_0 != 0
}
#[no_mangle]
#[c2rust::src_loc = "307:1"]
pub unsafe extern "C" fn sbuf_close(mut sbuf: *mut SBuf) -> bool {
    if (*sbuf).wait_type != 0 {
        *__error() = 0 as ::core::ffi::c_int;
        if event_del(&raw mut (*sbuf).ev) < 0 as ::core::ffi::c_int {
            if *__error() != 0 {
                let mut _log_ctx = NULL;
                log_generic(
                    LG_WARNING,
                    _log_ctx,
                    b"event_del: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    strerror(*__error()),
                );
            } else {
                let mut _log_ctx_0 = NULL;
                log_generic(
                    LG_WARNING,
                    _log_ctx_0,
                    b"event_del: libevent error\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
    }
    sbuf_op_close(sbuf);
    (*sbuf).dst = ::core::ptr::null_mut::<SBuf>();
    (*sbuf).sock = 0 as ::core::ffi::c_int;
    (*sbuf).pkt_remain = 0 as ::core::ffi::c_uint;
    (*sbuf).wait_type = 0 as uint8_t;
    (*sbuf).pkt_action = (*sbuf).wait_type;
    if !(*sbuf).io.is_null() {
        slab_free(iobuf_cache, (*sbuf).io as *mut ::core::ffi::c_void);
        (*sbuf).io = ::core::ptr::null_mut::<IOBuf>();
    }
    mbuf_free(&raw mut (*sbuf).extra_packets);
    true_0 != 0
}
#[no_mangle]
#[c2rust::src_loc = "337:1"]
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
#[c2rust::src_loc = "350:1"]
pub unsafe extern "C" fn sbuf_prepare_skip(mut sbuf: *mut SBuf, mut amount: ::core::ffi::c_uint) {
    (*sbuf).pkt_action = ACT_SKIP as uint8_t;
    (*sbuf).skip_remain = amount;
    (*sbuf).pkt_remain = amount;
}
#[no_mangle]
#[c2rust::src_loc = "366:1"]
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
#[c2rust::src_loc = "384:1"]
pub unsafe extern "C" fn sbuf_prepare_fetch(mut sbuf: *mut SBuf, mut amount: ::core::ffi::c_uint) {
    (*sbuf).pkt_action = ACT_CALL as uint8_t;
    (*sbuf).skip_remain = amount;
    (*sbuf).pkt_remain = amount;
}
#[no_mangle]
#[c2rust::src_loc = "406:1"]
pub unsafe extern "C" fn sbuf_queue_packet(
    mut src: *mut SBuf,
    mut dst: *mut SBuf,
    mut pkt: *mut PktBuf,
) -> bool {
    let mut res: bool = false;
    if pkt.is_null() || (*pkt).failed() as ::core::ffi::c_int != 0 {
        pktbuf_free(pkt);
        return false_0 != 0;
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
#[c2rust::src_loc = "445:1"]
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
#[c2rust::src_loc = "490:1"]
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
#[c2rust::src_loc = "516:1"]
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
            b"sbuf_wait_for_data: event_add failed: %s\0" as *const u8
                as *const ::core::ffi::c_char,
            strerror(*__error()),
        );
        return false_0 != 0;
    }
    (*sbuf).wait_type = W_RECV as ::core::ffi::c_int as uint8_t;
    true_0 != 0
}
#[c2rust::src_loc = "530:1"]
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
#[c2rust::src_loc = "543:1"]
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
            b"sbuf_wait_for_data: event_add failed: %s\0" as *const u8
                as *const ::core::ffi::c_char,
            strerror(*__error()),
        );
        return false_0 != 0;
    }
    (*sbuf).wait_type = W_ONCE as ::core::ffi::c_int as uint8_t;
    true_0 != 0
}
#[c2rust::src_loc = "567:1"]
unsafe extern "C" fn sbuf_send_cb(
    mut _sock: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut sbuf = arg as *mut SBuf;
    let mut res: bool = false;
    let mut _log_ctx = NULL;
    if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_NOISE,
            _log_ctx,
            b"Socket is writable again\0" as *const u8 as *const ::core::ffi::c_char,
        );
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
#[c2rust::src_loc = "594:1"]
unsafe extern "C" fn sbuf_queue_send(mut sbuf: *mut SBuf) -> bool {
    let mut err: ::core::ffi::c_int = 0;
    err = event_del(&raw mut (*sbuf).ev);
    (*sbuf).wait_type = W_NONE as ::core::ffi::c_int as uint8_t;
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            b"sbuf_queue_send: event_del failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
            strerror(*__error()),
        );
        return false_0 != 0;
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
            b"sbuf_queue_send: event_add failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
            strerror(*__error()),
        );
        return false_0 != 0;
    }
    (*sbuf).wait_type = W_SEND as ::core::ffi::c_int as uint8_t;
    true_0 != 0
}
#[no_mangle]
#[c2rust::src_loc = "626:1"]
pub unsafe extern "C" fn sbuf_flush(mut sbuf: *mut SBuf) -> bool {
    if !(*sbuf).io.is_null() {
        let mut _log_ctx = NULL;
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                _log_ctx,
                b"sbuf_flush\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        return sbuf_send_pending_iobuf(sbuf);
    }
    true_0 != 0
}
#[c2rust::src_loc = "640:1"]
unsafe extern "C" fn sbuf_send_pending_iobuf(mut sbuf: *mut SBuf) -> bool {
    let mut avail: ::core::ffi::c_int = 0;
    let mut res: ssize_t = 0;
    let mut io = (*sbuf).io;
    let mut _log_ctx = NULL;
    if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_NOISE,
            _log_ctx,
            b"sbuf_send_pending_iobuf\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    loop {
        avail = iobuf_amount_pending(io) as ::core::ffi::c_int;
        if avail == 0 as ::core::ffi::c_int {
            return true_0 != 0;
        }
        if (*(*sbuf).dst).sock == 0 as ::core::ffi::c_int {
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_0,
                b"sbuf_send_pending_iobuf: no dst sock?\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            sbuf_call_proto(sbuf, SBUF_EV_SEND_FAILED as ::core::ffi::c_int);
            return false_0 != 0;
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
            return false_0 != 0;
        }
    }
}
#[c2rust::src_loc = "693:1"]
unsafe extern "C" fn sbuf_send_pending_extra_packets(mut sbuf: *mut SBuf) -> bool {
    let mut avail: ::core::ffi::c_int = 0;
    let mut res: ssize_t = 0;
    let mut mbuf: *mut MBuf = &raw mut (*sbuf).extra_packets;
    let mut _log_ctx = NULL;
    if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_NOISE,
            _log_ctx,
            b"sbuf_send_pending_extra_packets \0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    loop {
        avail = mbuf_avail_for_read(mbuf) as ::core::ffi::c_int;
        if avail == 0 as ::core::ffi::c_int {
            return true_0 != 0;
        }
        if (*(*sbuf).dst).sock == 0 as ::core::ffi::c_int {
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_0,
                b"sbuf_send_pending_extra_packets: no dst sock?\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            sbuf_call_proto(sbuf, SBUF_EV_SEND_FAILED as ::core::ffi::c_int);
            return false_0 != 0;
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
            return false_0 != 0;
        }
    }
}
#[c2rust::src_loc = "744:1"]
unsafe extern "C" fn sbuf_process_pending(mut sbuf: *mut SBuf) -> bool {
    let mut current_block: u64;
    let mut avail: ::core::ffi::c_uint = 0;
    let mut io = (*sbuf).io;
    let mut extra_packets: *mut MBuf = &raw mut (*sbuf).extra_packets;
    let mut full = iobuf_amount_recv(io) <= 0 as ::core::ffi::c_uint;
    let mut loop_number = 0 as ::core::ffi::c_int;
    let mut _log_ctx = NULL;
    if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_NOISE,
            _log_ctx,
            b"sbuf_process_pending: start\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    loop {
        if mbuf_avail_for_read(extra_packets) != 0 {
            if (*sbuf).extra_packet_queue_after && !sbuf_send_pending_iobuf(sbuf) {
                let mut _log_ctx_0 = NULL;
                if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0
                {
                    log_generic(
                        LG_NOISE,
                        _log_ctx_0,
                        b"sbuf_process_pending failed to send all pending data\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                return false_0 != 0;
            }
            if !sbuf_send_pending_extra_packets(sbuf) {
                let mut _log_ctx_1 = NULL;
                if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0
                {
                    log_generic(
                        LG_NOISE,
                        _log_ctx_1,
                        b"sbuf_process_pending ended early because of not being able to send the queued extra packets\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                return false_0 != 0;
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
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                _log_ctx_2,
                b"sbuf_process_pending: loop %d\0" as *const u8 as *const ::core::ffi::c_char,
                loop_number,
            );
        }
        avail = iobuf_amount_parse(io);
        if avail == 0 as ::core::ffi::c_uint
            || full as ::core::ffi::c_int != 0 && avail <= SBUF_SMALL_PKT as ::core::ffi::c_uint
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
            return false_0 != 0;
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
                (*sbuf).skip_remain = (*sbuf).skip_remain.wrapping_sub(avail);
            } else {
                if (*sbuf).skip_remain != 0 as ::core::ffi::c_uint {
                    iobuf_tag_skip(io, (*sbuf).skip_remain);
                }
                iobuf_tag_send(io, avail.wrapping_sub((*sbuf).skip_remain));
                (*sbuf).skip_remain = 0 as ::core::ffi::c_uint;
            }
        }
        (*sbuf).pkt_remain = (*sbuf).pkt_remain.wrapping_sub(avail);
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
                    return false_0 != 0;
                }
                if !io.is_null() && (*io).recv_pos == cf_sbuf_len as ::core::ffi::c_uint {
                    let mut _log_ctx_6 = NULL;
                    if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                        as ::core::ffi::c_long
                        != 0
                    {
                        log_generic(
                            LG_NOISE,
                            _log_ctx_6,
                            b"resync(%d): done=%u, parse=%u, recv=%u, forced\0" as *const u8
                                as *const ::core::ffi::c_char,
                            (*sbuf).sock,
                            (*io).done_pos,
                            (*io).parse_pos,
                            (*io).recv_pos,
                        );
                    }
                    iobuf_try_resync(io, cf_sbuf_len as ::core::ffi::c_uint);
                }
            }
            false_0 != 0
        }
        _ => {
            let mut _log_ctx_3 = NULL;
            if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_NOISE,
                    _log_ctx_3,
                    b"sbuf_process_pending: done looping\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            if !sbuf_send_pending_iobuf(sbuf) {
                let mut _log_ctx_4 = NULL;
                if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0
                {
                    log_generic(
                        LG_NOISE,
                        _log_ctx_4,
                        b"sbuf_process_pending failed to send all pending data\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                return false_0 != 0;
            }
            let mut _log_ctx_5 = NULL;
            if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_NOISE,
                    _log_ctx_5,
                    b"sbuf_process_pending: end\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            true_0 != 0
        }
    }
}
#[c2rust::src_loc = "900:1"]
unsafe extern "C" fn sbuf_try_resync(mut sbuf: *mut SBuf, mut release: bool) {
    let mut io = (*sbuf).io;
    if !io.is_null() {
        let mut _log_ctx = NULL;
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                _log_ctx,
                b"resync(%d): done=%u, parse=%u, recv=%u\0" as *const u8
                    as *const ::core::ffi::c_char,
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
    if release as ::core::ffi::c_int != 0 && iobuf_empty(io) as ::core::ffi::c_int != 0 {
        slab_free(iobuf_cache, io as *mut ::core::ffi::c_void);
        (*sbuf).io = ::core::ptr::null_mut::<IOBuf>();
    } else {
        iobuf_try_resync(io, SBUF_SMALL_PKT as ::core::ffi::c_uint);
    };
}
#[c2rust::src_loc = "923:1"]
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
        return false_0 != 0;
    } else if got < 0 as ssize_t && *__error() != EAGAIN {
        sbuf_call_proto(sbuf, SBUF_EV_RECV_FAILED as ::core::ffi::c_int);
        return false_0 != 0;
    }
    true_0 != 0
}
#[c2rust::src_loc = "947:1"]
unsafe extern "C" fn sbuf_recv_cb(
    mut _sock: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut sbuf = arg as *mut SBuf;
    sbuf_main_loop(sbuf, DO_RECV != 0);
}
#[c2rust::src_loc = "953:1"]
unsafe extern "C" fn allocate_iobuf(mut sbuf: *mut SBuf) -> bool {
    if (*sbuf).io.is_null() {
        (*sbuf).io = slab_alloc(iobuf_cache) as *mut IOBuf;
        if (*sbuf).io.is_null() {
            sbuf_call_proto(sbuf, SBUF_EV_RECV_FAILED as ::core::ffi::c_int);
            return false_0 != 0;
        }
        iobuf_reset((*sbuf).io);
    }
    true_0 != 0
}
#[c2rust::src_loc = "975:1"]
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
                sbuf_try_resync(sbuf, false_0 != 0);
                if cf_sbuf_loopcnt > 0 as ::core::ffi::c_int && loopcnt >= cf_sbuf_loopcnt {
                    let mut _ignore: bool = false;
                    let mut _log_ctx = NULL;
                    if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        as ::core::ffi::c_long
                        != 0
                    {
                        log_generic(
                            LG_DEBUG,
                            _log_ctx,
                            b"loopcnt full\0" as *const u8 as *const ::core::ffi::c_char,
                        );
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
    sbuf_try_resync(sbuf, true_0 != 0);
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
#[c2rust::src_loc = "1063:1"]
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
        if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_DEBUG,
                _log_ctx,
                b"sbuf_after_connect_check: getsockopt: %s\0" as *const u8
                    as *const ::core::ffi::c_char,
                strerror(*__error()),
            );
        }
        return false_0 != 0;
    }
    if optval != 0 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_DEBUG,
                _log_ctx_0,
                b"sbuf_after_connect_check: pending error: %s\0" as *const u8
                    as *const ::core::ffi::c_char,
                strerror(optval),
            );
        }
        return false_0 != 0;
    }
    true_0 != 0
}
#[c2rust::src_loc = "1083:1"]
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
#[c2rust::src_loc = "1104:1"]
pub unsafe extern "C" fn sbuf_answer(
    mut sbuf: *mut SBuf,
    mut buf: *const ::core::ffi::c_void,
    mut len: size_t,
) -> bool {
    let mut res: ssize_t = 0;
    if (*sbuf).sock <= 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    res = sbuf_op_send(sbuf, buf, len);
    if res < 0 as ssize_t {
        let mut _log_ctx = NULL;
        if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_DEBUG,
                _log_ctx,
                b"sbuf_answer: error sending: %s\0" as *const u8 as *const ::core::ffi::c_char,
                strerror(*__error()),
            );
        }
    } else if res as ::core::ffi::c_uint as size_t != len {
        let mut _log_ctx_0 = NULL;
        if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_DEBUG,
                _log_ctx_0,
                b"sbuf_answer: partial send: len=%zu sent=%zd\0" as *const u8
                    as *const ::core::ffi::c_char,
                len,
                res,
            );
        }
    }
    res as ::core::ffi::c_uint as size_t == len
}
#[c2rust::src_loc = "1122:1"]
unsafe extern "C" fn raw_sbufio_peek(
    mut sbuf: *mut SBuf,
    mut buf: *mut ::core::ffi::c_void,
    mut len: size_t,
) -> ssize_t {
    safe_recv((*sbuf).sock, buf, len, 0x2 as ::core::ffi::c_int)
}
#[c2rust::src_loc = "1127:1"]
unsafe extern "C" fn raw_sbufio_recv(
    mut sbuf: *mut SBuf,
    mut dst: *mut ::core::ffi::c_void,
    mut len: size_t,
) -> ssize_t {
    safe_recv((*sbuf).sock, dst, len, 0 as ::core::ffi::c_int)
}
#[c2rust::src_loc = "1132:1"]
unsafe extern "C" fn raw_sbufio_send(
    mut sbuf: *mut SBuf,
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
) -> ssize_t {
    safe_send((*sbuf).sock, data, len, 0 as ::core::ffi::c_int)
}
#[c2rust::src_loc = "1137:1"]
unsafe extern "C" fn raw_sbufio_close(mut sbuf: *mut SBuf) -> ::core::ffi::c_int {
    if (*sbuf).sock > 0 as ::core::ffi::c_int {
        safe_close((*sbuf).sock);
        (*sbuf).sock = 0 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}
#[c2rust::src_loc = "1157:1"]
static mut client_accept_base: *mut tls = ::core::ptr::null::<tls>() as *mut tls;
#[c2rust::src_loc = "1158:1"]
static mut client_accept_conf: *mut tls_config =
    ::core::ptr::null::<tls_config>() as *mut tls_config;
#[no_mangle]
#[c2rust::src_loc = "1159:1"]
pub static mut client_accept_sslmode: ::core::ffi::c_int = 0;
#[c2rust::src_loc = "1160:1"]
static mut server_connect_conf: *mut tls_config =
    ::core::ptr::null::<tls_config>() as *mut tls_config;
#[no_mangle]
#[c2rust::src_loc = "1161:1"]
pub static mut server_connect_sslmode: ::core::ffi::c_int = 0;
#[c2rust::src_loc = "1167:1"]
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
                b"invalid %s_protocols: %s\0" as *const u8 as *const ::core::ffi::c_char,
                pfx,
                protocols,
            );
            return false_0 != 0;
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
                b"invalid %s_ciphers: %s\0" as *const u8 as *const ::core::ffi::c_char,
                pfx,
                ciphers,
            );
            return false_0 != 0;
        }
    }
    if !ciphers13.is_null() {
        err = tls_config_set_ciphers_v13(conf, ciphers13);
        if err != 0 {
            let mut _log_ctx_1 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_1,
                b"invalid %s_ciphers: %s\0" as *const u8 as *const ::core::ffi::c_char,
                pfx,
                ciphers13,
            );
            return false_0 != 0;
        }
    }
    if *dheparams != 0 {
        err = tls_config_set_dheparams(conf, dheparams);
        if err != 0 {
            let mut _log_ctx_2 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_2,
                b"invalid %s_dheparams: %s\0" as *const u8 as *const ::core::ffi::c_char,
                pfx,
                dheparams,
            );
            return false_0 != 0;
        }
    }
    if *ecdhecurve != 0 {
        err = tls_config_set_ecdhecurve(conf, ecdhecurve);
        if err != 0 {
            let mut _log_ctx_3 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_3,
                b"invalid %s_ecdhecurve: %s\0" as *const u8 as *const ::core::ffi::c_char,
                pfx,
                ecdhecurve,
            );
            return false_0 != 0;
        }
    }
    if *cafile != 0 {
        err = tls_config_set_ca_file(conf, cafile);
        if err != 0 {
            let mut _log_ctx_4 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_4,
                b"invalid %s_ca_file: %s\0" as *const u8 as *const ::core::ffi::c_char,
                pfx,
                cafile,
            );
            return false_0 != 0;
        }
    }
    if *keyfile != 0 {
        err = tls_config_set_key_file(conf, keyfile);
        if err != 0 {
            let mut _log_ctx_5 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_5,
                b"invalid %s_key_file: %s\0" as *const u8 as *const ::core::ffi::c_char,
                pfx,
                keyfile,
            );
            return false_0 != 0;
        }
    }
    if *certfile != 0 {
        err = tls_config_set_cert_file(conf, certfile);
        if err != 0 {
            let mut _log_ctx_6 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_6,
                b"invalid %s_cert_file: %s\0" as *const u8 as *const ::core::ffi::c_char,
                pfx,
                certfile,
            );
            return false_0 != 0;
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
    true_0 != 0
}
#[c2rust::src_loc = "1258:1"]
unsafe extern "C" fn tls_change_requires_reconnect(
    mut new_server_connect_conf: *mut tls_config,
) -> bool {
    if server_connect_sslmode != cf_server_tls_sslmode {
        let mut _log_ctx = NULL;
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                _log_ctx,
                b"new server_tls_sslmode detected\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        true_0 != 0
    } else if server_connect_conf.is_null() {
        let mut _log_ctx_0 = NULL;
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                _log_ctx_0,
                b"no existing server tls config detected\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        true_0 != 0
    } else if tls_config_equal(new_server_connect_conf, server_connect_conf) {
        let mut _log_ctx_1 = NULL;
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                _log_ctx_1,
                b"no server tls config change detected\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        false_0 != 0
    } else {
        let mut _log_ctx_2 = NULL;
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                _log_ctx_2,
                b"server tls config change detected\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        true_0 != 0
    }
}
#[no_mangle]
#[c2rust::src_loc = "1275:1"]
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
                b"To allow TLS connections from clients, client_tls_key_file and client_tls_cert_file must be set.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        return false_0 != 0;
    }
    if cf_auth_type == AUTH_TYPE_CERT as ::core::ffi::c_int {
        if cf_client_tls_sslmode != SSLMODE_VERIFY_FULL as ::core::ffi::c_int {
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_0,
                b"auth_type=cert requires client_tls_sslmode=SSLMODE_VERIFY_FULL\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
        if *cf_client_tls_ca_file as ::core::ffi::c_int == '\0' as i32 {
            let mut _log_ctx_1 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_1,
                b"auth_type=cert requires client_tls_ca_file\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
    } else if cf_client_tls_sslmode > SSLMODE_VERIFY_CA as ::core::ffi::c_int
        && *cf_client_tls_ca_file as ::core::ffi::c_int == '\0' as i32
    {
        let mut _log_ctx_2 = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx_2,
            b"client_tls_sslmode requires client_tls_ca_file\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    err = tls_init();
    if err != 0 {
        let mut _log_ctx_3 = NULL;
        log_fatal(
            b"src/sbuf.c\0" as *const u8 as *const ::core::ffi::c_char,
            1311 as ::core::ffi::c_int,
            b"sbuf_tls_setup\0" as *const u8 as *const ::core::ffi::c_char,
            false_0 != 0,
            _log_ctx_3,
            b"tls_init failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    if cf_server_tls_sslmode != SSLMODE_DISABLED as ::core::ffi::c_int {
        new_server_connect_conf = tls_config_new();
        if new_server_connect_conf.is_null() {
            let mut _log_ctx_4 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_4,
                b"tls_config_new failed 1\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return false_0 != 0;
        }
        if !setup_tls(
            new_server_connect_conf,
            b"server_tls\0" as *const u8 as *const ::core::ffi::c_char,
            cf_server_tls_sslmode,
            cf_server_tls_protocols,
            cf_server_tls_ciphers,
            cf_server_tls13_ciphers,
            cf_server_tls_key_file,
            cf_server_tls_cert_file,
            cf_server_tls_ca_file,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            true_0 != 0,
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
                log_generic(
                    LG_ERROR,
                    _log_ctx_5,
                    b"tls_config_new failed 2\0" as *const u8 as *const ::core::ffi::c_char,
                );
                current_block = 4939747223443714714;
            } else if !setup_tls(
                new_client_accept_conf,
                b"client_tls\0" as *const u8 as *const ::core::ffi::c_char,
                cf_client_tls_sslmode,
                cf_client_tls_protocols,
                cf_client_tls_ciphers,
                cf_client_tls13_ciphers,
                cf_client_tls_key_file,
                cf_client_tls_cert_file,
                cf_client_tls_ca_file,
                cf_client_tls_dheparams,
                cf_client_tls_ecdhecurve,
                false_0 != 0,
            ) {
                current_block = 4939747223443714714;
            } else {
                new_client_accept_base = tls_server();
                if new_client_accept_base.is_null() {
                    let mut _log_ctx_6 = NULL;
                    log_generic(
                        LG_ERROR,
                        _log_ctx_6,
                        b"server_base failed\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    current_block = 4939747223443714714;
                } else {
                    err = tls_configure(new_client_accept_base, new_client_accept_conf);
                    if err != 0 {
                        let mut _log_ctx_7 = NULL;
                        log_generic(
                            LG_ERROR,
                            _log_ctx_7,
                            b"TLS setup failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
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
                            .offset(-(0 as ::core::ffi::c_ulong as isize))
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
                return true_0 != 0;
            }
        }
    }
    usual_tls_free(new_client_accept_base);
    tls_config_free(new_client_accept_conf);
    tls_config_free(new_server_connect_conf);
    false_0 != 0
}
#[c2rust::src_loc = "1389:1"]
unsafe extern "C" fn handle_tls_handshake(mut sbuf: *mut SBuf) -> bool {
    let mut err: ::core::ffi::c_int = 0;
    err = tls_handshake((*sbuf).tls);
    let mut _log_ctx = NULL;
    if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_NOISE,
            _log_ctx,
            b"tls_handshake: err=%d\0" as *const u8 as *const ::core::ffi::c_char,
            err,
        );
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
        true_0 != 0
    } else {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_0,
            b"TLS handshake error: %s\0" as *const u8 as *const ::core::ffi::c_char,
            tls_error((*sbuf).tls),
        );
        false_0 != 0
    }
}
#[c2rust::src_loc = "1409:1"]
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
#[c2rust::src_loc = "1421:1"]
pub unsafe extern "C" fn sbuf_tls_accept(mut sbuf: *mut SBuf) -> bool {
    let mut err: ::core::ffi::c_int = 0;
    if !sbuf_pause(sbuf) {
        return false_0 != 0;
    }
    (*sbuf).ops = &raw const tls_sbufio_ops;
    err = tls_accept_fds(
        client_accept_base,
        &raw mut (*sbuf).tls,
        (*sbuf).sock,
        (*sbuf).sock,
    );
    let mut _log_ctx = NULL;
    if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_NOISE,
            _log_ctx,
            b"tls_accept_fds: err=%d\0" as *const u8 as *const ::core::ffi::c_char,
            err,
        );
    }
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_0,
            b"TLS accept error: %s\0" as *const u8 as *const ::core::ffi::c_char,
            tls_error((*sbuf).tls),
        );
        return false_0 != 0;
    }
    (*sbuf).tls_state = SBUF_TLS_DO_HANDSHAKE as ::core::ffi::c_int as uint8_t;
    true_0 != 0
}
#[no_mangle]
#[c2rust::src_loc = "1445:1"]
pub unsafe extern "C" fn sbuf_tls_connect(
    mut sbuf: *mut SBuf,
    mut hostname: *const ::core::ffi::c_char,
) -> bool {
    let mut ctls = ::core::ptr::null_mut::<tls>();
    let mut err: ::core::ffi::c_int = 0;
    if !sbuf_pause(sbuf) {
        return false_0 != 0;
    }
    if cf_server_tls_sslmode != SSLMODE_VERIFY_FULL as ::core::ffi::c_int {
        hostname = ::core::ptr::null::<::core::ffi::c_char>();
    }
    ctls = tls_client();
    if ctls.is_null() {
        return false_0 != 0;
    }
    err = tls_configure(ctls, server_connect_conf);
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx,
            b"tls client config failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
            tls_error(ctls),
        );
        usual_tls_free(ctls);
        return false_0 != 0;
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
            b"TLS connect error: %s\0" as *const u8 as *const ::core::ffi::c_char,
            tls_error((*sbuf).tls),
        );
        return false_0 != 0;
    }
    (*sbuf).tls_state = SBUF_TLS_DO_HANDSHAKE as ::core::ffi::c_int as uint8_t;
    true_0 != 0
}
#[c2rust::src_loc = "1484:1"]
unsafe extern "C" fn tls_sbufio_peek(
    mut _sbuf: *mut SBuf,
    mut _buf: *mut ::core::ffi::c_void,
    mut _len: size_t,
) -> ssize_t {
    -(1 as ::core::ffi::c_int) as ssize_t
}
#[c2rust::src_loc = "1490:1"]
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
    if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_NOISE,
            _log_ctx,
            b"tls_read: req=%zu out=%zd\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"tls_sbufio_recv: got TLS_WANT_POLLOUT\0" as *const u8 as *const ::core::ffi::c_char,
        );
        *__error() = EIO;
    } else {
        let mut _log_ctx_1 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_1,
            b"tls_sbufio_recv: %s\0" as *const u8 as *const ::core::ffi::c_char,
            tls_error((*sbuf).tls),
        );
        *__error() = EIO;
    }
    -(1 as ::core::ffi::c_int) as ssize_t
}
#[c2rust::src_loc = "1515:1"]
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
    if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_NOISE,
            _log_ctx,
            b"tls_write: req=%zu out=%zd\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"tls_sbufio_send: got TLS_WANT_POLLIN\0" as *const u8 as *const ::core::ffi::c_char,
        );
        *__error() = EIO;
    } else {
        let mut _log_ctx_1 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_1,
            b"tls_sbufio_send: %s\0" as *const u8 as *const ::core::ffi::c_char,
            tls_error((*sbuf).tls),
        );
        *__error() = EIO;
    }
    -(1 as ::core::ffi::c_int) as ssize_t
}
#[c2rust::src_loc = "1540:1"]
unsafe extern "C" fn tls_sbufio_close(mut sbuf: *mut SBuf) -> ::core::ffi::c_int {
    let mut _log_ctx = NULL;
    if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_NOISE,
            _log_ctx,
            b"tls_close\0" as *const u8 as *const ::core::ffi::c_char,
        );
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
#[c2rust::src_loc = "1555:1"]
pub unsafe extern "C" fn sbuf_cleanup() {
    usual_tls_free(client_accept_base);
    tls_config_free(client_accept_conf);
    tls_config_free(server_connect_conf);
    client_accept_conf = ::core::ptr::null_mut::<tls_config>();
    server_connect_conf = ::core::ptr::null_mut::<tls_config>();
    client_accept_base = ::core::ptr::null_mut::<tls>();
}
#[c2rust::src_loc = "1565:1"]
unsafe extern "C" fn handle_possible_direct_tls_startup(
    mut sbuf: *mut SBuf,
    mut is_unix: bool,
) -> bool {
    if client_accept_sslmode == SSLMODE_DISABLED as ::core::ffi::c_int
        || is_unix as ::core::ffi::c_int != 0
    {
        return true_0 != 0;
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
#[c2rust::src_loc = "1573:1"]
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
    if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_NOISE,
            _log_ctx,
            b"Starting TLS handshake\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !sbuf_tls_accept(sbuf) {
        disconnect_client(
            client,
            false_0 != 0,
            b"failed to accept SSL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*sbuf).pkt_action = SBUF_TLS_IN_HANDSHAKE as ::core::ffi::c_int as uint8_t;
    if !handle_tls_handshake(sbuf) {
        sbuf_call_proto(sbuf, SBUF_EV_RECV_FAILED as ::core::ffi::c_int);
    }
}
