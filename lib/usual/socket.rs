pub mod _types_h {

    pub type __uint8_t = u8;

    pub type __uint16_t = u16;

    pub type __int32_t = i32;

    pub type __uint32_t = u32;

    pub type __darwin_size_t = usize;

    pub type __darwin_socklen_t = __uint32_t;
}

pub mod sys__types_h {

    pub type __darwin_gid_t = __uint32_t;

    pub type __darwin_pid_t = __int32_t;

    pub type __darwin_uid_t = __uint32_t;

    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    use super::_types_h::{__int32_t, __uint32_t};
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

pub mod _pid_t_h {

    pub type pid_t = __darwin_pid_t;
    use super::sys__types_h::__darwin_pid_t;
}

pub mod _uid_t_h {

    pub type uid_t = __darwin_uid_t;
    use super::sys__types_h::__darwin_uid_t;
}

pub mod _size_t_h {

    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
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

    pub const SO_KEEPALIVE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

    pub const SO_NOSIGPIPE: ::core::ffi::c_int = 0x1022 as ::core::ffi::c_int;

    pub const SOL_SOCKET: ::core::ffi::c_int = 0xffff as ::core::ffi::c_int;

    pub const AF_UNIX: ::core::ffi::c_int = 1;

    pub const AF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

    pub const AF_INET6: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
    use super::_sa_family_t_h::sa_family_t;
    use super::_socklen_t_h::socklen_t;
    use super::_types_h::__uint8_t;
    extern "C" {

        pub fn setsockopt(
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_int,
            _: *const ::core::ffi::c_void,
            _: socklen_t,
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
    use super::_in_port_t_h::in_port_t;
    use super::_sa_family_t_h::sa_family_t;
    use super::_types_h::{__uint16_t, __uint32_t, __uint8_t};
}

pub mod _OSByteOrder_h {
    #[inline]

    pub unsafe extern "C" fn _OSSwapInt16(mut _data: __uint16_t) -> __uint16_t {
        ((_data as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | _data as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as __uint16_t
    }
    use super::_types_h::__uint16_t;
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

pub mod unistd_h {
    use super::_gid_t_h::gid_t;
    use super::_uid_t_h::uid_t;
    extern "C" {

        pub fn getpeereid(
            _: ::core::ffi::c_int,
            _: *mut uid_t,
            _: *mut gid_t,
        ) -> ::core::ffi::c_int;
    }
}

pub mod inet_h {
    use super::_socklen_t_h::socklen_t;
    extern "C" {

        pub fn inet_ntop(
            _: ::core::ffi::c_int,
            _: *const ::core::ffi::c_void,
            _: *mut ::core::ffi::c_char,
            __size: socklen_t,
        ) -> *const ::core::ffi::c_char;
    }
}

pub mod _null_h {

    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}

pub mod errno_h {

    pub const ENOPROTOOPT: ::core::ffi::c_int = 42 as ::core::ffi::c_int;
    extern "C" {

        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}

pub mod stdbool_h {

    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}

pub mod fcntl_h {

    pub const O_NONBLOCK: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

    pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

    pub const F_GETFL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

    pub const F_SETFL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

    pub const FD_CLOEXEC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    extern "C" {

        pub fn fcntl(_: ::core::ffi::c_int, _: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    }
}

pub mod tcp_h {

    pub const TCP_KEEPALIVE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

    pub const TCP_KEEPINTVL: ::core::ffi::c_int = 0x101 as ::core::ffi::c_int;

    pub const TCP_KEEPCNT: ::core::ffi::c_int = 0x102 as ::core::ffi::c_int;
}
pub use self::_OSByteOrder_h::_OSSwapInt16;
pub use self::_gid_t_h::gid_t;
pub use self::_in_addr_t_h::in_addr_t;
pub use self::_in_port_t_h::in_port_t;
pub use self::_null_h::NULL;
pub use self::_pid_t_h::pid_t;
pub use self::_sa_family_t_h::sa_family_t;
pub use self::_size_t_h::size_t;
pub use self::_socklen_t_h::socklen_t;
use self::_stdio_h::snprintf;
pub use self::_types_h::{
    __darwin_size_t, __darwin_socklen_t, __int32_t, __uint16_t, __uint32_t, __uint8_t,
};
pub use self::_uid_t_h::uid_t;
pub use self::errno_h::{__error, ENOPROTOOPT};
pub use self::fcntl_h::{fcntl, FD_CLOEXEC, F_GETFL, F_SETFD, F_SETFL, O_NONBLOCK};
pub use self::in6_h::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use self::in_h::{in_addr, sockaddr_in, IPPROTO_TCP};
use self::inet_h::inet_ntop;
pub use self::socket_h::{
    setsockopt, sockaddr, AF_INET, AF_INET6, AF_UNIX, SOL_SOCKET, SO_KEEPALIVE, SO_NOSIGPIPE,
};
pub use self::stdbool_h::{false_0, true_0};
pub use self::sys__types_h::{__darwin_gid_t, __darwin_pid_t, __darwin_uid_t, __DARWIN_NULL};
pub use self::tcp_h::{TCP_KEEPALIVE, TCP_KEEPCNT, TCP_KEEPINTVL};
pub use self::un_h::sockaddr_un;
use self::unistd_h::getpeereid;
#[no_mangle]

pub unsafe extern "C" fn socket_set_nonblocking(
    mut fd: ::core::ffi::c_int,
    mut non_block: bool,
) -> bool {
    let mut flags: ::core::ffi::c_int = 0;
    flags = fcntl(fd, F_GETFL, 0 as ::core::ffi::c_int);
    if flags < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if non_block {
        flags |= O_NONBLOCK;
    } else {
        flags &= !O_NONBLOCK;
    }
    if fcntl(fd, F_SETFL, flags) < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    true_0 != 0
}
#[no_mangle]

pub unsafe extern "C" fn socket_setup(mut sock: ::core::ffi::c_int, mut non_block: bool) -> bool {
    let mut res: ::core::ffi::c_int = 0;
    let mut val = 1 as ::core::ffi::c_int;
    res = setsockopt(
        sock,
        SOL_SOCKET,
        SO_NOSIGPIPE,
        &raw mut val as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    );
    if res < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    res = fcntl(sock, F_SETFD, FD_CLOEXEC);
    if res < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if !socket_set_nonblocking(sock, non_block) {
        return false_0 != 0;
    }
    true_0 != 0
}
#[no_mangle]

pub unsafe extern "C" fn socket_set_keepalive(
    mut fd: ::core::ffi::c_int,
    mut onoff: ::core::ffi::c_int,
    mut keepidle: ::core::ffi::c_int,
    mut keepintvl: ::core::ffi::c_int,
    mut keepcnt: ::core::ffi::c_int,
) -> bool {
    let mut val: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 0;
    if onoff == 0 {
        val = 0 as ::core::ffi::c_int;
        res = setsockopt(
            fd,
            SOL_SOCKET,
            SO_KEEPALIVE,
            &raw mut val as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        );
        return res == 0 as ::core::ffi::c_int;
    }
    val = 1 as ::core::ffi::c_int;
    res = setsockopt(
        fd,
        SOL_SOCKET,
        SO_KEEPALIVE,
        &raw mut val as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    );
    if res < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if keepidle != 0 {
        val = keepidle;
        res = setsockopt(
            fd,
            IPPROTO_TCP,
            TCP_KEEPALIVE,
            &raw mut val as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        );
        if res < 0 as ::core::ffi::c_int && *__error() != ENOPROTOOPT {
            return false_0 != 0;
        }
    }
    if keepintvl != 0 {
        val = keepintvl;
        res = setsockopt(
            fd,
            IPPROTO_TCP,
            TCP_KEEPINTVL,
            &raw mut val as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        );
        if res < 0 as ::core::ffi::c_int && *__error() != ENOPROTOOPT {
            return false_0 != 0;
        }
    }
    if keepcnt > 0 as ::core::ffi::c_int {
        val = keepcnt;
        res = setsockopt(
            fd,
            IPPROTO_TCP,
            TCP_KEEPCNT,
            &raw mut val as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        );
        if res < 0 as ::core::ffi::c_int && *__error() != ENOPROTOOPT {
            return false_0 != 0;
        }
    }
    true_0 != 0
}
#[no_mangle]

pub unsafe extern "C" fn sa2str(
    mut sa: *const sockaddr,
    mut dst: *mut ::core::ffi::c_char,
    mut dstlen: size_t,
) -> *const ::core::ffi::c_char {
    let mut in_0 = ::core::ptr::null::<sockaddr_in>();
    let mut in6 = ::core::ptr::null::<sockaddr_in6>();
    let mut un = ::core::ptr::null::<sockaddr_un>();
    let mut tmp = ::core::ptr::null::<::core::ffi::c_char>();
    let mut buf: [::core::ffi::c_char; 128] = [0; 128];
    match (*sa).sa_family as ::core::ffi::c_int {
        AF_INET => {
            in_0 = sa as *mut sockaddr_in;
            tmp = inet_ntop(
                AF_INET,
                &raw const (*in_0).sin_addr as *const ::core::ffi::c_void,
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as socklen_t,
            );
            if tmp.is_null() {
                return ::core::ptr::null::<::core::ffi::c_char>();
            }
            snprintf(
                dst,
                dstlen,
                b"%s:%d\0" as *const u8 as *const ::core::ffi::c_char,
                tmp,
                (if 0 != 0 {
                    (((*in_0).sin_port as ::core::ffi::c_uint & 0xff00 as ::core::ffi::c_uint)
                        >> 8 as ::core::ffi::c_int
                        | ((*in_0).sin_port as ::core::ffi::c_uint & 0xff as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int) as __uint16_t
                        as ::core::ffi::c_int
                } else {
                    _OSSwapInt16((*in_0).sin_port as __uint16_t) as ::core::ffi::c_int
                }) as __uint16_t as ::core::ffi::c_int,
            );
        }
        AF_INET6 => {
            in6 = sa as *mut sockaddr_in6;
            tmp = inet_ntop(
                AF_INET6,
                &raw const (*in6).sin6_addr as *const ::core::ffi::c_void,
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as socklen_t,
            );
            if tmp.is_null() {
                return ::core::ptr::null::<::core::ffi::c_char>();
            }
            snprintf(
                dst,
                dstlen,
                b"[%s]:%d\0" as *const u8 as *const ::core::ffi::c_char,
                tmp,
                (if 0 != 0 {
                    (((*in6).sin6_port as ::core::ffi::c_uint & 0xff00 as ::core::ffi::c_uint)
                        >> 8 as ::core::ffi::c_int
                        | ((*in6).sin6_port as ::core::ffi::c_uint & 0xff as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int) as __uint16_t
                        as ::core::ffi::c_int
                } else {
                    _OSSwapInt16((*in6).sin6_port as __uint16_t) as ::core::ffi::c_int
                }) as __uint16_t as ::core::ffi::c_int,
            );
        }
        AF_UNIX => {
            un = sa as *mut sockaddr_un;
            if (*un).sun_path[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '\0' as i32
                && (*un).sun_path[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    != '\0' as i32
            {
                snprintf(
                    dst,
                    dstlen,
                    b"unix:@%s\0" as *const u8 as *const ::core::ffi::c_char,
                    (&raw const (*un).sun_path as *const ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize),
                );
            } else {
                snprintf(
                    dst,
                    dstlen,
                    b"unix:%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw const (*un).sun_path as *const ::core::ffi::c_char,
                );
            }
        }
        _ => {
            snprintf(
                dst,
                dstlen,
                b"sa2str(%d): unknown proto\0" as *const u8 as *const ::core::ffi::c_char,
                (*sa).sa_family as ::core::ffi::c_int,
            );
        }
    }
    dst
}
#[no_mangle]

pub unsafe extern "C" fn usual_getpeercreds(
    mut fd: ::core::ffi::c_int,
    mut uid_p: *mut uid_t,
    mut gid_p: *mut gid_t,
    mut pid_p: *mut pid_t,
) -> ::core::ffi::c_int {
    *pid_p = 0 as ::core::ffi::c_int as pid_t;
    if getpeereid(fd, uid_p, gid_p) == 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -(1 as ::core::ffi::c_int)
    }
}
