
pub mod _types_h {
    
    pub type __uint8_t = u8;
    
    pub type __int32_t = i32;
    
    pub type __uint32_t = u32;
    
    pub type __darwin_size_t = usize;
    
    pub type __darwin_socklen_t = __uint32_t;
    
    pub type __darwin_ssize_t = isize;
    
    pub type __darwin_time_t = ::core::ffi::c_long;
}

pub mod sys__types_h {
    
    pub type __darwin_suseconds_t = __int32_t;
    
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    use super::_types_h::__int32_t;
}

pub mod _size_t_h {
    
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}

pub mod _ssize_t_h {
    
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
}

pub mod _fd_def_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct fd_set {
        pub fds_bits: [__int32_t; 32],
    }
    use super::_types_h::__int32_t;
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

pub mod _sa_family_t_h {
    
    pub type sa_family_t = __uint8_t;
    use super::_types_h::__uint8_t;
}

pub mod _socklen_t_h {
    
    pub type socklen_t = __darwin_socklen_t;
    use super::_types_h::__darwin_socklen_t;
}

pub mod _iovec_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct iovec {
        pub iov_base: *mut ::core::ffi::c_void,
        pub iov_len: size_t,
    }
    use super::_size_t_h::size_t;
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
    
    pub const AF_UNIX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    use super::_iovec_t_h::iovec;
    use super::_sa_family_t_h::sa_family_t;
    use super::_size_t_h::size_t;
    use super::_socklen_t_h::socklen_t;
    use super::_ssize_t_h::ssize_t;
    use super::_types_h::__uint8_t;
    extern "C" {
        
        pub fn accept(
            _: ::core::ffi::c_int,
            _: *mut sockaddr,
            _: *mut socklen_t,
        ) -> ::core::ffi::c_int;
        
        pub fn connect(
            _: ::core::ffi::c_int,
            _: *const sockaddr,
            _: socklen_t,
        ) -> ::core::ffi::c_int;
        
        pub fn recv(
            _: ::core::ffi::c_int,
            _: *mut ::core::ffi::c_void,
            _: size_t,
            _: ::core::ffi::c_int,
        ) -> ssize_t;
        
        pub fn recvmsg(_: ::core::ffi::c_int, _: *mut msghdr, _: ::core::ffi::c_int) -> ssize_t;
        
        pub fn send(
            _: ::core::ffi::c_int,
            _: *const ::core::ffi::c_void,
            _: size_t,
            _: ::core::ffi::c_int,
        ) -> ssize_t;
        
        pub fn sendmsg(_: ::core::ffi::c_int, _: *const msghdr, _: ::core::ffi::c_int) -> ssize_t;
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

pub mod unistd_h {
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    extern "C" {
        
        pub fn close(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
        
        pub fn read(_: ::core::ffi::c_int, _: *mut ::core::ffi::c_void, __nbyte: size_t)
            -> ssize_t;
        
        pub fn write(
            __fd: ::core::ffi::c_int,
            __buf: *const ::core::ffi::c_void,
            __nbyte: size_t,
        ) -> ssize_t;
    }
}

pub mod _select_h {
    use super::_fd_def_h::fd_set;
    use super::_timeval_h::timeval;
    extern "C" {
        
        pub fn select(
            _: ::core::ffi::c_int,
            _: *mut fd_set,
            _: *mut fd_set,
            _: *mut fd_set,
            _: *mut timeval,
        ) -> ::core::ffi::c_int;
    }
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

pub mod _null_h {
    
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}

pub mod string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        
        pub fn usual_strerror_r(
            e: ::core::ffi::c_int,
            dst: *mut ::core::ffi::c_char,
            dstlen: size_t,
        ) -> *const ::core::ffi::c_char;
    }
}

pub mod errno_h {
    
    pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    pub const EINPROGRESS: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
    
    pub const EMSGSIZE: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
    extern "C" {
        
        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}
pub use self::_fd_def_h::fd_set;
pub use self::_iovec_t_h::iovec;
pub use self::_null_h::NULL;
pub use self::_sa_family_t_h::sa_family_t;
use self::_select_h::select;
pub use self::_size_t_h::size_t;
pub use self::_socklen_t_h::socklen_t;
pub use self::_ssize_t_h::ssize_t;
pub use self::_timeval_h::timeval;
pub use self::_types_h::{
    __darwin_size_t, __darwin_socklen_t, __darwin_ssize_t, __darwin_time_t, __int32_t, __uint32_t,
    __uint8_t,
};
pub use self::errno_h::{__error, EINPROGRESS, EINTR, EMSGSIZE};
pub use self::logging_h::{
    cf_verbose, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE, LG_STATS,
    LG_WARNING,
};
pub use self::socket_h::{
    accept, connect, msghdr, recv, recvmsg, send, sendmsg, sockaddr, AF_UNIX,
};
use self::string_h::usual_strerror_r;
pub use self::sys__types_h::{__darwin_suseconds_t, __DARWIN_NULL};
use self::unistd_h::{close, read, write};
use self::usual_socket_h::sa2str;
#[no_mangle]

pub unsafe extern "C" fn safe_read(
    mut fd: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_void,
    mut len: size_t,
) -> ssize_t {
    let mut res: ssize_t = 0;
    loop {
        res = read(fd, buf, len);
        if !(res < 0 as ssize_t && *__error() == EINTR) {
            break;
        }
    }
    res
}
#[no_mangle]

pub unsafe extern "C" fn safe_write(
    mut fd: ::core::ffi::c_int,
    mut buf: *const ::core::ffi::c_void,
    mut len: size_t,
) -> ssize_t {
    let mut res: ssize_t = 0;
    loop {
        res = write(fd, buf, len);
        if !(res < 0 as ssize_t && *__error() == EINTR) {
            break;
        }
    }
    res
}
#[no_mangle]

pub unsafe extern "C" fn safe_recv(
    mut fd: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_void,
    mut len: size_t,
    mut flags: ::core::ffi::c_int,
) -> ssize_t {
    let mut res: ssize_t = 0;
    let mut ebuf: [::core::ffi::c_char; 128] = [0; 128];
    loop {
        res = recv(fd, buf, len, flags);
        if !(res < 0 as ssize_t && *__error() == EINTR) {
            break;
        }
    }
    if res < 0 as ssize_t {
        let mut _log_ctx = NULL;
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                _log_ctx,
                b"safe_recv(%d, %zu) = %s\0" as *const u8 as *const ::core::ffi::c_char,
                fd,
                len,
                usual_strerror_r(
                    *__error(),
                    &raw mut ebuf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
                ),
            );
        }
    } else if cf_verbose > 2 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                _log_ctx_0,
                b"safe_recv(%d, %zu) = %zd\0" as *const u8 as *const ::core::ffi::c_char,
                fd,
                len,
                res,
            );
        }
    }
    res
}
#[no_mangle]

pub unsafe extern "C" fn safe_send(
    mut fd: ::core::ffi::c_int,
    mut buf: *const ::core::ffi::c_void,
    mut len: size_t,
    mut flags: ::core::ffi::c_int,
) -> ssize_t {
    let mut res: ssize_t = 0;
    let mut ebuf: [::core::ffi::c_char; 128] = [0; 128];
    loop {
        res = send(fd, buf, len, flags);
        if !(res < 0 as ssize_t && *__error() == EINTR) {
            break;
        }
    }
    if res < 0 as ssize_t {
        let mut _log_ctx = NULL;
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                _log_ctx,
                b"safe_send(%d, %zu) = %s\0" as *const u8 as *const ::core::ffi::c_char,
                fd,
                len,
                usual_strerror_r(
                    *__error(),
                    &raw mut ebuf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
                ),
            );
        }
    } else if cf_verbose > 2 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                _log_ctx_0,
                b"safe_send(%d, %zu) = %zd\0" as *const u8 as *const ::core::ffi::c_char,
                fd,
                len,
                res,
            );
        }
    }
    res
}
#[no_mangle]

pub unsafe extern "C" fn safe_close(mut fd: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int = 0;
    res = close(fd);
    if res < 0 as ::core::ffi::c_int {
        let mut ebuf: [::core::ffi::c_char; 128] = [0; 128];
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            b"safe_close(%d) = %s\0" as *const u8 as *const ::core::ffi::c_char,
            fd,
            usual_strerror_r(
                *__error(),
                &raw mut ebuf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
            ),
        );
    } else if cf_verbose > 2 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                _log_ctx_0,
                b"safe_close(%d) = %d\0" as *const u8 as *const ::core::ffi::c_char,
                fd,
                res,
            );
        }
    }
    if res < 0 as ::core::ffi::c_int && *__error() == EINTR {
        return 0 as ::core::ffi::c_int;
    }
    res
}
#[no_mangle]

pub unsafe extern "C" fn safe_recvmsg(
    mut fd: ::core::ffi::c_int,
    mut msg: *mut msghdr,
    mut flags: ::core::ffi::c_int,
) -> ssize_t {
    let mut res: ssize_t = 0;
    let mut ebuf: [::core::ffi::c_char; 128] = [0; 128];
    loop {
        res = recvmsg(fd, msg, flags);
        if !(res < 0 as ssize_t && *__error() == EINTR) {
            break;
        }
    }
    if res < 0 as ssize_t {
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            b"safe_recvmsg(%d, msg, %d) = %s\0" as *const u8 as *const ::core::ffi::c_char,
            fd,
            flags,
            usual_strerror_r(
                *__error(),
                &raw mut ebuf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
            ),
        );
    } else if cf_verbose > 2 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                _log_ctx_0,
                b"safe_recvmsg(%d, msg, %d) = %zd\0" as *const u8 as *const ::core::ffi::c_char,
                fd,
                flags,
                res,
            );
        }
    }
    res
}
#[no_mangle]

pub unsafe extern "C" fn safe_sendmsg(
    mut fd: ::core::ffi::c_int,
    mut msg: *const msghdr,
    mut flags: ::core::ffi::c_int,
) -> ssize_t {
    let mut res: ssize_t = 0;
    let mut msgerr_count = 0 as ::core::ffi::c_int;
    let mut ebuf: [::core::ffi::c_char; 128] = [0; 128];
    loop {
        res = sendmsg(fd, msg, flags);
        if res < 0 as ssize_t && *__error() == EINTR {
            continue;
        }
        if res < 0 as ssize_t {
            let mut _log_ctx = NULL;
            log_generic(
                LG_WARNING,
                _log_ctx,
                b"safe_sendmsg(%d, msg[%d,%d], %d) = %s\0" as *const u8
                    as *const ::core::ffi::c_char,
                fd,
                (*(*msg).msg_iov.offset(0 as ::core::ffi::c_int as isize)).iov_len
                    as ::core::ffi::c_int,
                (*msg).msg_controllen as ::core::ffi::c_int,
                flags,
                usual_strerror_r(
                    *__error(),
                    &raw mut ebuf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
                ),
            );
            if !(*__error() == EMSGSIZE && msgerr_count < 20 as ::core::ffi::c_int) {
                break;
            }
            let mut tv = timeval {
                tv_sec: 1 as __darwin_time_t,
                tv_usec: 0 as __darwin_suseconds_t,
            };
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_WARNING,
                _log_ctx_0,
                b"trying to sleep a bit\0" as *const u8 as *const ::core::ffi::c_char,
            );
            select(
                0 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<fd_set>(),
                ::core::ptr::null_mut::<fd_set>(),
                ::core::ptr::null_mut::<fd_set>(),
                &raw mut tv,
            );
            msgerr_count += 1;
        } else {
            if cf_verbose > 2 as ::core::ffi::c_int {
                let mut _log_ctx_1 = NULL;
                if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0
                {
                    log_generic(
                        LG_NOISE,
                        _log_ctx_1,
                        b"safe_sendmsg(%d, msg, %d) = %zd\0" as *const u8
                            as *const ::core::ffi::c_char,
                        fd,
                        flags,
                        res,
                    );
                }
            }
            break;
        }
    }
    res
}
#[no_mangle]

pub unsafe extern "C" fn safe_connect(
    mut fd: ::core::ffi::c_int,
    mut sa: *const sockaddr,
    mut sa_len: socklen_t,
) -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 128] = [0; 128];
    let mut ebuf: [::core::ffi::c_char; 128] = [0; 128];
    loop {
        res = connect(fd, sa, sa_len);
        if !(res < 0 as ::core::ffi::c_int && *__error() == EINTR) {
            break;
        }
    }
    if res < 0 as ::core::ffi::c_int
        && (*__error() != EINPROGRESS || cf_verbose > 2 as ::core::ffi::c_int)
    {
        let mut _log_ctx = NULL;
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                _log_ctx,
                b"connect(%d, %s) = %s\0" as *const u8 as *const ::core::ffi::c_char,
                fd,
                sa2str(
                    sa,
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
                ),
                usual_strerror_r(
                    *__error(),
                    &raw mut ebuf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
                ),
            );
        }
    } else if cf_verbose > 2 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                _log_ctx_0,
                b"connect(%d, %s) = %d\0" as *const u8 as *const ::core::ffi::c_char,
                fd,
                sa2str(
                    sa,
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
                ),
                res,
            );
        }
    }
    res
}
#[no_mangle]

pub unsafe extern "C" fn safe_accept(
    mut fd: ::core::ffi::c_int,
    mut sa: *mut sockaddr,
    mut sa_len_p: *mut socklen_t,
) -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 128] = [0; 128];
    let mut ebuf: [::core::ffi::c_char; 128] = [0; 128];
    loop {
        res = accept(fd, sa, sa_len_p);
        if !(res < 0 as ::core::ffi::c_int && *__error() == EINTR) {
            break;
        }
    }
    if res < 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_NOISE,
                _log_ctx,
                b"safe_accept(%d) = %s\0" as *const u8 as *const ::core::ffi::c_char,
                fd,
                usual_strerror_r(
                    *__error(),
                    &raw mut ebuf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
                ),
            );
        }
    } else if cf_verbose > 2 as ::core::ffi::c_int {
        if (*sa).sa_family as ::core::ffi::c_int == AF_UNIX {
            let mut _log_ctx_0 = NULL;
            if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_NOISE,
                    _log_ctx_0,
                    b"safe_accept(%d) = %d (unix)\0" as *const u8 as *const ::core::ffi::c_char,
                    fd,
                    res,
                );
            }
        } else {
            let mut _log_ctx_1 = NULL;
            if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_NOISE,
                    _log_ctx_1,
                    b"safe_accept(%d) = %d (%s)\0" as *const u8 as *const ::core::ffi::c_char,
                    fd,
                    res,
                    sa2str(
                        sa,
                        &raw mut buf as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
                    ),
                );
            }
        }
    }
    res
}
