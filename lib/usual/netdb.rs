pub mod _types_h {

    pub type __uint8_t = u8;

    pub type __uint32_t = u32;

    pub type __darwin_socklen_t = __uint32_t;
}

pub mod _pthread_types_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _opaque_pthread_attr_t {
        pub __sig: ::core::ffi::c_long,
        pub __opaque: [::core::ffi::c_char; 56],
    }

    pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
}

pub mod _pthread_attr_t_h {

    pub type pthread_attr_t = __darwin_pthread_attr_t;
    use super::_pthread_types_h::__darwin_pthread_attr_t;
}

pub mod signal_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub union sigval {
        pub sival_int: ::core::ffi::c_int,
        pub sival_ptr: *mut ::core::ffi::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct sigevent {
        pub sigev_notify: ::core::ffi::c_int,
        pub sigev_signo: ::core::ffi::c_int,
        pub sigev_value: sigval,
        pub sigev_notify_function: Option<unsafe extern "C" fn(sigval) -> ()>,
        pub sigev_notify_attributes: *mut pthread_attr_t,
    }
    use super::_pthread_attr_t_h::pthread_attr_t;
}

pub mod _socklen_t_h {

    pub type socklen_t = __darwin_socklen_t;
    use super::_types_h::__darwin_socklen_t;
}

pub mod _sa_family_t_h {

    pub type sa_family_t = __uint8_t;
    use super::_types_h::__uint8_t;
}

pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct sockaddr {
        pub sa_len: __uint8_t,
        pub sa_family: sa_family_t,
        pub sa_data: [::core::ffi::c_char; 14],
    }
    use super::_sa_family_t_h::sa_family_t;
    use super::_types_h::__uint8_t;
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

    pub const EAI_SYSTEM: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
    use super::_socklen_t_h::socklen_t;
    use super::socket_h::sockaddr;
}

pub mod usual_netdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct usual_gaicb {
        pub ar_name: *const ::core::ffi::c_char,
        pub ar_service: *const ::core::ffi::c_char,
        pub ar_request: *const addrinfo,
        pub ar_result: *mut addrinfo,
        pub _state: ::core::ffi::c_int,
    }
    use super::netdb_h::addrinfo;
}

pub mod errno_h {

    pub const ENOSYS: ::core::ffi::c_int = 78 as ::core::ffi::c_int;
    extern "C" {

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

pub unsafe extern "C" fn usual_getaddrinfo_a(
    mut _mode: ::core::ffi::c_int,
    mut _list: *mut *mut usual_gaicb,
    mut _nitems: ::core::ffi::c_int,
    mut _sevp: *mut sigevent,
) -> ::core::ffi::c_int {
    *__error() = ENOSYS;
    EAI_SYSTEM
}
