#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:19"]
pub mod _types_h {
    #[c2rust::src_loc = "32:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "36:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "117:1"]
    pub type __darwin_socklen_t = __uint32_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_types.h:19"]
pub mod _pthread_types_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:1"]
    pub struct _opaque_pthread_attr_t {
        pub __sig: ::core::ffi::c_long,
        pub __opaque: [::core::ffi::c_char; 56],
    }
    #[c2rust::src_loc = "109:1"]
    pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_attr_t.h:19"]
pub mod _pthread_attr_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pthread_attr_t = __darwin_pthread_attr_t;
    use super::_pthread_types_h::__darwin_pthread_attr_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/signal.h:19"]
pub mod signal_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "158:1"]
    pub union sigval {
        pub sival_int: ::core::ffi::c_int,
        pub sival_ptr: *mut ::core::ffi::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "168:1"]
    pub struct sigevent {
        pub sigev_notify: ::core::ffi::c_int,
        pub sigev_signo: ::core::ffi::c_int,
        pub sigev_value: sigval,
        pub sigev_notify_function: Option<unsafe extern "C" fn(sigval) -> ()>,
        pub sigev_notify_attributes: *mut pthread_attr_t,
    }
    use super::_pthread_attr_t_h::pthread_attr_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_socklen_t.h:19"]
pub mod _socklen_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type socklen_t = __darwin_socklen_t;
    use super::_types_h::__darwin_socklen_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_sa_family_t.h:19"]
pub mod _sa_family_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type sa_family_t = __uint8_t;
    use super::_types_h::__uint8_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/socket.h:19"]
pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "414:1"]
    pub struct sockaddr {
        pub sa_len: __uint8_t,
        pub sa_family: sa_family_t,
        pub sa_data: [::core::ffi::c_char; 14],
    }
    use super::_sa_family_t_h::sa_family_t;
    use super::_types_h::__uint8_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netdb.h:19"]
pub mod netdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:1"]
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
    #[c2rust::src_loc = "198:9"]
    pub const EAI_SYSTEM: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
    use super::_socklen_t_h::socklen_t;
    use super::socket_h::sockaddr;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/netdb.h:19"]
pub mod usual_netdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "54:1"]
    pub struct usual_gaicb {
        pub ar_name: *const ::core::ffi::c_char,
        pub ar_service: *const ::core::ffi::c_char,
        pub ar_request: *const addrinfo,
        pub ar_result: *mut addrinfo,
        pub _state: ::core::ffi::c_int,
    }
    use super::netdb_h::addrinfo;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/errno.h:19"]
pub mod errno_h {
    #[c2rust::src_loc = "209:9"]
    pub const ENOSYS: ::core::ffi::c_int = 78 as ::core::ffi::c_int;
    extern "C" {
        #[c2rust::src_loc = "80:1"]
        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}
pub use self::_pthread_attr_t_h::pthread_attr_t;
pub use self::_pthread_types_h::{__darwin_pthread_attr_t, _opaque_pthread_attr_t};
pub use self::_sa_family_t_h::sa_family_t;
pub use self::_socklen_t_h::socklen_t;
pub use self::_types_h::{__darwin_socklen_t, __uint32_t, __uint8_t};
pub use self::errno_h::{__error, ENOSYS};
pub use self::netdb_h::{addrinfo, EAI_SYSTEM};
pub use self::signal_h::{sigevent, sigval};
pub use self::socket_h::sockaddr;
pub use self::usual_netdb_h::usual_gaicb;
#[no_mangle]
#[c2rust::src_loc = "209:1"]
pub unsafe extern "C" fn usual_getaddrinfo_a(
    mut _mode: ::core::ffi::c_int,
    mut _list: *mut *mut usual_gaicb,
    mut _nitems: ::core::ffi::c_int,
    mut _sevp: *mut sigevent,
) -> ::core::ffi::c_int {
    *__error() = ENOSYS;
    return EAI_SYSTEM;
}
