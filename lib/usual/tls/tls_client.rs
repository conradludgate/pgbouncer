
pub mod _types_h {
    
    pub type __uint8_t = u8;
    
    pub type __uint16_t = u16;
    
    pub type __uint32_t = u32;
    
    pub type __darwin_size_t = usize;
    
    pub type __darwin_socklen_t = __uint32_t;
    
    pub type __darwin_time_t = ::core::ffi::c_long;
}

pub mod _in_addr_t_h {
    
    pub type in_addr_t = __uint32_t;
    use super::_types_h::__uint32_t;
}

pub mod _size_t_h {
    
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}

pub mod _time_t_h {
    
    pub type time_t = __darwin_time_t;
    use super::_types_h::__darwin_time_t;
}

pub mod _uint32_t_h {
    
    pub type uint32_t = u32;
}

pub mod tls_internal_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct tls {
        pub config: *mut tls_config,
        pub error: tls_error,
        pub flags: uint32_t,
        pub state: uint32_t,
        pub servername: *mut ::core::ffi::c_char,
        pub socket: ::core::ffi::c_int,
        pub ssl_conn: *mut SSL,
        pub ssl_ctx: *mut SSL_CTX,
        pub ssl_peer_cert: *mut X509,
        pub conninfo: *mut tls_conninfo,
        pub used_dh_bits: ::core::ffi::c_int,
        pub used_ecdh_nid: ::core::ffi::c_int,
        pub ocsp_result: *const ::core::ffi::c_char,
        pub ocsp_info: *mut tls_ocsp_info,
        pub ocsp_query: *mut tls_ocsp_query,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct tls_ocsp_info {
        pub response_status: ::core::ffi::c_int,
        pub cert_status: ::core::ffi::c_int,
        pub crl_reason: ::core::ffi::c_int,
        pub this_update: time_t,
        pub next_update: time_t,
        pub revocation_time: time_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct tls_conninfo {
        pub issuer: *mut ::core::ffi::c_char,
        pub subject: *mut ::core::ffi::c_char,
        pub hash: *mut ::core::ffi::c_char,
        pub serial: *mut ::core::ffi::c_char,
        pub fingerprint: *mut ::core::ffi::c_char,
        pub version: *mut ::core::ffi::c_char,
        pub cipher: *mut ::core::ffi::c_char,
        pub notbefore: time_t,
        pub notafter: time_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct tls_error {
        pub msg: *mut ::core::ffi::c_char,
        pub num: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct tls_config {
        pub error: tls_error,
        pub ca_file: *const ::core::ffi::c_char,
        pub ca_path: *const ::core::ffi::c_char,
        pub ca_mem: *mut ::core::ffi::c_char,
        pub ca_len: size_t,
        pub ciphers: *const ::core::ffi::c_char,
        pub cipher_suites: *const ::core::ffi::c_char,
        pub ciphers_server: ::core::ffi::c_int,
        pub dheparams: ::core::ffi::c_int,
        pub ecdhecurve: ::core::ffi::c_int,
        pub keypair: *mut tls_keypair,
        pub ocsp_file: *const ::core::ffi::c_char,
        pub ocsp_mem: *mut ::core::ffi::c_char,
        pub ocsp_len: size_t,
        pub protocols: uint32_t,
        pub verify_cert: ::core::ffi::c_int,
        pub verify_client: ::core::ffi::c_int,
        pub verify_depth: ::core::ffi::c_int,
        pub verify_name: ::core::ffi::c_int,
        pub verify_time: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct tls_keypair {
        pub next: *mut tls_keypair,
        pub cert_file: *const ::core::ffi::c_char,
        pub cert_mem: *mut ::core::ffi::c_char,
        pub cert_len: size_t,
        pub key_file: *const ::core::ffi::c_char,
        pub key_mem: *mut ::core::ffi::c_char,
        pub key_len: size_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union tls_addr {
        pub ip4: in_addr,
        pub ip6: in6_addr,
    }
    
    pub const TLS_CLIENT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
    
    pub const TLS_HANDSHAKE_COMPLETE: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
    use super::_size_t_h::size_t;
    use super::_time_t_h::time_t;
    use super::_uint32_t_h::uint32_t;
    use super::in6_h::in6_addr;
    use super::in_h::in_addr;
    use super::types_h::{SSL, SSL_CTX, X509};
    extern "C" {
        
        pub type tls_ocsp_query;
        
        pub fn tls_new() -> *mut tls;
        
        pub fn tls_check_name(
            ctx: *mut tls,
            cert: *mut X509,
            servername: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        
        pub fn tls_configure_keypair(
            ctx: *mut tls,
            ssl_ctx: *mut SSL_CTX,
            keypair: *mut tls_keypair,
            required: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        
        pub fn tls_configure_ssl(ctx: *mut tls) -> ::core::ffi::c_int;
        
        pub fn tls_configure_ssl_verify(
            ctx: *mut tls,
            verify: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        
        pub fn tls_host_port(
            hostport: *const ::core::ffi::c_char,
            host: *mut *mut ::core::ffi::c_char,
            port: *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        
        pub fn tls_set_error(
            ctx: *mut tls,
            fmt: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        
        pub fn tls_set_errorx(
            ctx: *mut tls,
            fmt: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        
        pub fn tls_ssl_error(
            ctx: *mut tls,
            ssl_conn: *mut SSL,
            ssl_ret: ::core::ffi::c_int,
            prefix: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        
        pub fn tls_ocsp_verify_callback(
            ssl: *mut SSL,
            arg: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
    }
}

pub mod types_h {
    
    pub type X509 = x509_st;
    
    pub type SSL_CTX = ssl_ctx_st;
    
    pub type SSL = ssl_st;
    extern "C" {
        
        pub type x509_st;
        
        pub type ssl_ctx_st;
        
        pub type ssl_st;
    }
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
    use super::_types_h::{__uint16_t, __uint32_t, __uint8_t};
}

pub mod in_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    use super::_in_addr_t_h::in_addr_t;
}

pub mod ssl_h {
    
    pub type SSL_METHOD = ssl_method_st;
    
    pub const SSL_VERIFY_PEER: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    pub const SSL_CTRL_SET_TLSEXT_HOSTNAME: ::core::ffi::c_int = 55 as ::core::ffi::c_int;
    
    pub const SSL_CTRL_SET_TLSEXT_STATUS_REQ_CB: ::core::ffi::c_int = 63 as ::core::ffi::c_int;
    
    pub const SSL_CTRL_SET_TLSEXT_STATUS_REQ_TYPE: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
    use super::types_h::{SSL, SSL_CTX, X509};
    extern "C" {
        
        pub type ssl_method_st;
        
        pub fn SSL_CTX_new(meth: *const SSL_METHOD) -> *mut SSL_CTX;
        
        pub fn SSL_set_rfd(s: *mut SSL, fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
        
        pub fn SSL_set_wfd(s: *mut SSL, fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
        
        pub fn SSL_get1_peer_certificate(s: *const SSL) -> *mut X509;
        
        pub fn SSL_new(ctx: *mut SSL_CTX) -> *mut SSL;
        
        pub fn SSL_connect(ssl: *mut SSL) -> ::core::ffi::c_int;
        
        pub fn SSL_ctrl(
            ssl: *mut SSL,
            cmd: ::core::ffi::c_int,
            larg: ::core::ffi::c_long,
            parg: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_long;
        
        pub fn SSL_CTX_callback_ctrl(
            _: *mut SSL_CTX,
            _: ::core::ffi::c_int,
            _: Option<unsafe extern "C" fn() -> ()>,
        ) -> ::core::ffi::c_long;
        
        pub fn TLS_client_method() -> *const SSL_METHOD;
        
        pub fn SSL_set_ex_data(
            ssl: *mut SSL,
            idx: ::core::ffi::c_int,
            data: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
    }
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
    
    pub const AI_NUMERICHOST: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    pub const AI_ADDRCONFIG: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
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

pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct sockaddr {
        pub sa_len: __uint8_t,
        pub sa_family: sa_family_t,
        pub sa_data: [::core::ffi::c_char; 14],
    }
    
    pub const SOCK_STREAM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    pub const AF_UNSPEC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    pub const AF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    pub const AF_INET6: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
    use super::_sa_family_t_h::sa_family_t;
    use super::_socklen_t_h::socklen_t;
    use super::_types_h::__uint8_t;
    extern "C" {
        
        pub fn connect(
            _: ::core::ffi::c_int,
            _: *const sockaddr,
            _: socklen_t,
        ) -> ::core::ffi::c_int;
        
        pub fn socket(
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}

pub mod _sa_family_t_h {
    
    pub type sa_family_t = __uint8_t;
    use super::_types_h::__uint8_t;
}

pub mod _socklen_t_h {
    
    pub type socklen_t = __darwin_socklen_t;
    use super::_types_h::__darwin_socklen_t;
}

pub mod _null_h {
    
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}

pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        
        pub fn memset(
            __b: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __len: size_t,
        ) -> *mut ::core::ffi::c_void;
        
        pub fn strdup(__s1: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    }
}

pub mod x509_h {
    use super::types_h::X509;
    extern "C" {
        
        pub fn X509_free(a: *mut X509);
    }
}

pub mod sys__types_h {
    
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}

pub mod _malloc_h {
    extern "C" {
        
        pub fn free(_: *mut ::core::ffi::c_void);
    }
}

pub mod unistd_h {
    extern "C" {
        
        pub fn close(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    }
}

pub mod inet_h {
    extern "C" {
        
        pub fn inet_pton(
            _: ::core::ffi::c_int,
            _: *const ::core::ffi::c_char,
            _: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
    }
}

pub mod tls1_h {
    
    pub const TLSEXT_NAMETYPE_host_name: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}

pub mod err_h {
    extern "C" {
        
        pub fn ERR_clear_error();
    }
}
pub use self::_in_addr_t_h::in_addr_t;
use self::_malloc_h::free;
pub use self::_null_h::NULL;
pub use self::_sa_family_t_h::sa_family_t;
pub use self::_size_t_h::size_t;
pub use self::_socklen_t_h::socklen_t;
use self::_string_h::{memset, strdup};
pub use self::_time_t_h::time_t;
pub use self::_types_h::{
    __darwin_size_t, __darwin_socklen_t, __darwin_time_t, __uint16_t, __uint32_t, __uint8_t,
};
pub use self::_uint32_t_h::uint32_t;
use self::err_h::ERR_clear_error;
pub use self::in6_h::{in6_addr, C2RustUnnamed};
pub use self::in_h::in_addr;
use self::inet_h::inet_pton;
pub use self::netdb_h::{
    addrinfo, freeaddrinfo, gai_strerror, getaddrinfo, AI_ADDRCONFIG, AI_NUMERICHOST,
};
pub use self::socket_h::{connect, sockaddr, socket, AF_INET, AF_INET6, AF_UNSPEC, SOCK_STREAM};
pub use self::ssl_h::{
    ssl_method_st, SSL_CTX_callback_ctrl, SSL_CTX_new, SSL_connect, SSL_ctrl,
    SSL_get1_peer_certificate, SSL_new, SSL_set_ex_data, SSL_set_rfd, SSL_set_wfd,
    TLS_client_method, SSL_CTRL_SET_TLSEXT_HOSTNAME, SSL_CTRL_SET_TLSEXT_STATUS_REQ_CB,
    SSL_CTRL_SET_TLSEXT_STATUS_REQ_TYPE, SSL_METHOD, SSL_VERIFY_PEER,
};
pub use self::sys__types_h::__DARWIN_NULL;
pub use self::tls1_h::TLSEXT_NAMETYPE_host_name;
pub use self::tls_internal_h::{
    tls, tls_addr, tls_check_name, tls_config, tls_configure_keypair, tls_configure_ssl,
    tls_configure_ssl_verify, tls_conninfo, tls_error, tls_host_port, tls_keypair, tls_new,
    tls_ocsp_info, tls_ocsp_query, tls_ocsp_verify_callback, tls_set_error, tls_set_errorx,
    tls_ssl_error, TLS_CLIENT, TLS_HANDSHAKE_COMPLETE,
};
pub use self::types_h::{ssl_ctx_st, ssl_st, x509_st, SSL, SSL_CTX, X509};
use self::unistd_h::close;
use self::x509_h::X509_free;
#[no_mangle]

pub unsafe extern "C" fn tls_client() -> *mut tls {
    let mut ctx = ::core::ptr::null_mut::<tls>();
    ctx = tls_new();
    if ctx.is_null() {
        return ::core::ptr::null_mut::<tls>();
    }
    (*ctx).flags |= TLS_CLIENT as uint32_t;
    ctx
}
#[no_mangle]

pub unsafe extern "C" fn tls_connect(
    mut ctx: *mut tls,
    mut host: *const ::core::ffi::c_char,
    mut port: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    tls_connect_servername(ctx, host, port, ::core::ptr::null::<::core::ffi::c_char>())
}
#[no_mangle]

pub unsafe extern "C" fn tls_connect_servername(
    mut ctx: *mut tls,
    mut host: *const ::core::ffi::c_char,
    mut port: *const ::core::ffi::c_char,
    mut servername: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut hints = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_canonname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ai_addr: ::core::ptr::null_mut::<sockaddr>(),
        ai_next: ::core::ptr::null_mut::<addrinfo>(),
    };
    let mut res = ::core::ptr::null_mut::<addrinfo>();
    let mut res0 = ::core::ptr::null_mut::<addrinfo>();
    let mut h = ::core::ptr::null::<::core::ffi::c_char>();
    let mut p = ::core::ptr::null::<::core::ffi::c_char>();
    let mut hs = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ps = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rv = -(1 as ::core::ffi::c_int);
    let mut s = -(1 as ::core::ffi::c_int);
    let mut ret: ::core::ffi::c_int = 0;
    if (*ctx).flags & TLS_CLIENT as uint32_t == 0 as uint32_t {
        tls_set_errorx(
            ctx,
            b"not a client context\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if host.is_null() {
        tls_set_errorx(
            ctx,
            b"host not specified\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        if port.is_null() {
            ret = tls_host_port(host, &raw mut hs, &raw mut ps);
            if ret == -(1 as ::core::ffi::c_int) {
                tls_set_errorx(
                    ctx,
                    b"memory allocation failure\0" as *const u8 as *const ::core::ffi::c_char,
                );
                current_block = 12048367495949684927;
            } else if ret != 0 as ::core::ffi::c_int {
                tls_set_errorx(
                    ctx,
                    b"no port provided\0" as *const u8 as *const ::core::ffi::c_char,
                );
                current_block = 12048367495949684927;
            } else {
                current_block = 7746791466490516765;
            }
        } else {
            current_block = 7746791466490516765;
        }
        match current_block {
            12048367495949684927 => {}
            _ => {
                h = if !hs.is_null() {
                    hs as *const ::core::ffi::c_char
                } else {
                    host
                };
                p = if !ps.is_null() {
                    ps as *const ::core::ffi::c_char
                } else {
                    port
                };
                memset(
                    &raw mut hints as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<addrinfo>() as size_t,
                );
                hints.ai_socktype = SOCK_STREAM;
                hints.ai_family = AF_INET;
                hints.ai_flags = AI_NUMERICHOST;
                if getaddrinfo(h, p, &raw mut hints, &raw mut res0) != 0 as ::core::ffi::c_int {
                    hints.ai_family = AF_INET6;
                    if getaddrinfo(h, p, &raw mut hints, &raw mut res0) != 0 as ::core::ffi::c_int {
                        hints.ai_family = AF_UNSPEC;
                        hints.ai_flags = AI_ADDRCONFIG;
                        s = getaddrinfo(h, p, &raw mut hints, &raw mut res0);
                        if s != 0 as ::core::ffi::c_int {
                            tls_set_error(
                                ctx,
                                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                gai_strerror(s),
                            );
                            current_block = 12048367495949684927;
                        } else {
                            current_block = 15089075282327824602;
                        }
                    } else {
                        current_block = 15089075282327824602;
                    }
                } else {
                    current_block = 15089075282327824602;
                }
                match current_block {
                    12048367495949684927 => {}
                    _ => {
                        s = -(1 as ::core::ffi::c_int);
                        res = res0;
                        while !res.is_null() {
                            s = socket((*res).ai_family, (*res).ai_socktype, (*res).ai_protocol);
                            if s == -(1 as ::core::ffi::c_int) {
                                tls_set_error(
                                    ctx,
                                    b"socket\0" as *const u8 as *const ::core::ffi::c_char,
                                );
                            } else {
                                if connect(s, (*res).ai_addr, (*res).ai_addrlen)
                                    != -(1 as ::core::ffi::c_int)
                                {
                                    break;
                                }
                                tls_set_error(
                                    ctx,
                                    b"connect\0" as *const u8 as *const ::core::ffi::c_char,
                                );
                                close(s);
                                s = -(1 as ::core::ffi::c_int);
                            }
                            res = (*res).ai_next;
                        }
                        freeaddrinfo(res0);
                        if s != -(1 as ::core::ffi::c_int) {
                            if servername.is_null() {
                                servername = h;
                            }
                            if tls_connect_socket(ctx, s, servername) != 0 as ::core::ffi::c_int {
                                close(s);
                            } else {
                                (*ctx).socket = s;
                                rv = 0 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    free(hs as *mut ::core::ffi::c_void);
    free(ps as *mut ::core::ffi::c_void);
    rv
}
#[no_mangle]

pub unsafe extern "C" fn tls_connect_socket(
    mut ctx: *mut tls,
    mut s: ::core::ffi::c_int,
    mut servername: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    tls_connect_fds(ctx, s, s, servername)
}
#[no_mangle]

pub unsafe extern "C" fn tls_connect_fds(
    mut ctx: *mut tls,
    mut fd_read: ::core::ffi::c_int,
    mut fd_write: ::core::ffi::c_int,
    mut servername: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut addrbuf = tls_addr {
        ip4: in_addr { s_addr: 0 },
    };
    let mut rv = -(1 as ::core::ffi::c_int);
    if (*ctx).flags & TLS_CLIENT as uint32_t == 0 as uint32_t {
        tls_set_errorx(
            ctx,
            b"not a client context\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if fd_read < 0 as ::core::ffi::c_int || fd_write < 0 as ::core::ffi::c_int {
        tls_set_errorx(
            ctx,
            b"invalid file descriptors\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        if !servername.is_null() {
            (*ctx).servername = strdup(servername);
            if (*ctx).servername.is_null() {
                tls_set_errorx(
                    ctx,
                    b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
                );
                current_block = 7102792219047461859;
            } else {
                current_block = 10886091980245723256;
            }
        } else {
            current_block = 10886091980245723256;
        }
        match current_block {
            7102792219047461859 => {}
            _ => {
                (*ctx).ssl_ctx = SSL_CTX_new(TLS_client_method());
                if (*ctx).ssl_ctx.is_null() {
                    tls_set_errorx(
                        ctx,
                        b"ssl context failure\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else if (tls_configure_ssl(ctx) == 0 as ::core::ffi::c_int)
                    && (tls_configure_keypair(
                        ctx,
                        (*ctx).ssl_ctx,
                        (*(*ctx).config).keypair,
                        0 as ::core::ffi::c_int,
                    ) == 0 as ::core::ffi::c_int)
                {
                    if (*(*ctx).config).verify_name != 0 {
                        if servername.is_null() {
                            tls_set_errorx(
                                ctx,
                                b"server name not specified\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            current_block = 7102792219047461859;
                        } else {
                            current_block = 9606288038608642794;
                        }
                    } else {
                        current_block = 9606288038608642794;
                    }
                    match current_block {
                        7102792219047461859 => {}
                        _ => {
                            if !((*(*ctx).config).verify_cert != 0
                                && tls_configure_ssl_verify(ctx, SSL_VERIFY_PEER)
                                    == -(1 as ::core::ffi::c_int))
                            {
                                if SSL_CTX_callback_ctrl(
                                    (*ctx).ssl_ctx,
                                    SSL_CTRL_SET_TLSEXT_STATUS_REQ_CB,
                                    ::core::mem::transmute::<
                                        Option<
                                            unsafe extern "C" fn(
                                                *mut SSL,
                                                *mut ::core::ffi::c_void,
                                            )
                                                -> ::core::ffi::c_int,
                                        >,
                                        Option<unsafe extern "C" fn() -> ()>,
                                    >(Some(
                                        tls_ocsp_verify_callback
                                            as unsafe extern "C" fn(
                                                *mut SSL,
                                                *mut ::core::ffi::c_void,
                                            )
                                                -> ::core::ffi::c_int,
                                    )),
                                ) != 1 as ::core::ffi::c_long
                                {
                                    tls_set_errorx(
                                        ctx,
                                        b"ssl OCSP verification setup failure\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                } else {
                                    (*ctx).ssl_conn = SSL_new((*ctx).ssl_ctx);
                                    if (*ctx).ssl_conn.is_null() {
                                        tls_set_errorx(
                                            ctx,
                                            b"ssl connection failure\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    } else if SSL_set_ex_data(
                                        (*ctx).ssl_conn,
                                        0 as ::core::ffi::c_int,
                                        ctx as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                                    ) != 1 as ::core::ffi::c_int
                                    {
                                        tls_set_errorx(
                                            ctx,
                                            b"ssl application data failure\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    } else if SSL_set_rfd((*ctx).ssl_conn, fd_read)
                                        != 1 as ::core::ffi::c_int
                                        || SSL_set_wfd((*ctx).ssl_conn, fd_write)
                                            != 1 as ::core::ffi::c_int
                                    {
                                        tls_set_errorx(
                                            ctx,
                                            b"ssl file descriptor failure\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    } else if SSL_ctrl(
                                        (*ctx).ssl_conn,
                                        SSL_CTRL_SET_TLSEXT_STATUS_REQ_TYPE,
                                        1 as ::core::ffi::c_long,
                                        NULL,
                                    ) != 1 as ::core::ffi::c_long
                                    {
                                        tls_set_errorx(
                                            ctx,
                                            b"ssl OCSP extension setup failure\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    } else {
                                        if !servername.is_null()
                                            && inet_pton(
                                                AF_INET,
                                                servername,
                                                &raw mut addrbuf as *mut ::core::ffi::c_void,
                                            ) != 1 as ::core::ffi::c_int
                                            && inet_pton(
                                                AF_INET6,
                                                servername,
                                                &raw mut addrbuf as *mut ::core::ffi::c_void,
                                            ) != 1 as ::core::ffi::c_int
                                        {
                                            if SSL_ctrl(
                                                (*ctx).ssl_conn,
                                                SSL_CTRL_SET_TLSEXT_HOSTNAME,
                                                TLSEXT_NAMETYPE_host_name as ::core::ffi::c_long,
                                                servername as *mut ::core::ffi::c_void,
                                            ) == 0 as ::core::ffi::c_long
                                            {
                                                tls_set_errorx(
                                                    ctx,
                                                    b"server name indication failure\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                );
                                                current_block = 7102792219047461859;
                                            } else {
                                                current_block = 5689316957504528238;
                                            }
                                        } else {
                                            current_block = 5689316957504528238;
                                        }
                                        match current_block {
                                            7102792219047461859 => {}
                                            _ => {
                                                rv = 0 as ::core::ffi::c_int;
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
    rv
}
#[no_mangle]

pub unsafe extern "C" fn tls_handshake_client(mut ctx: *mut tls) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut cert = ::core::ptr::null_mut::<X509>();
    let mut ssl_ret: ::core::ffi::c_int = 0;
    let mut rv = -(1 as ::core::ffi::c_int);
    if (*ctx).flags & TLS_CLIENT as uint32_t == 0 as uint32_t {
        tls_set_errorx(
            ctx,
            b"not a client context\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        ERR_clear_error();
        ssl_ret = SSL_connect((*ctx).ssl_conn);
        if ssl_ret != 1 as ::core::ffi::c_int {
            rv = tls_ssl_error(
                ctx,
                (*ctx).ssl_conn,
                ssl_ret,
                b"handshake\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            if (*(*ctx).config).verify_name != 0 {
                cert = SSL_get1_peer_certificate((*ctx).ssl_conn);
                if cert.is_null() {
                    tls_set_errorx(
                        ctx,
                        b"no server certificate\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    current_block = 6257997688677503150;
                } else {
                    rv = tls_check_name(ctx, cert, (*ctx).servername);
                    if rv != 0 as ::core::ffi::c_int {
                        if rv != -(2 as ::core::ffi::c_int) {
                            tls_set_errorx(
                                ctx,
                                b"name `%s' not present in server certificate\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*ctx).servername,
                            );
                        }
                        current_block = 6257997688677503150;
                    } else {
                        current_block = 12800627514080957624;
                    }
                }
            } else {
                current_block = 12800627514080957624;
            }
            match current_block {
                6257997688677503150 => {}
                _ => {
                    (*ctx).state |= TLS_HANDSHAKE_COMPLETE as uint32_t;
                    rv = 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    X509_free(cert);
    rv
}
