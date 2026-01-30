
pub mod _types_h {
    
    pub type __int32_t = i32;
    
    pub type __darwin_size_t = usize;
    
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

pub mod _time_t_h {
    
    pub type time_t = __darwin_time_t;
    use super::_types_h::__darwin_time_t;
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

pub mod _time_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct tm {
        pub tm_sec: ::core::ffi::c_int,
        pub tm_min: ::core::ffi::c_int,
        pub tm_hour: ::core::ffi::c_int,
        pub tm_mday: ::core::ffi::c_int,
        pub tm_mon: ::core::ffi::c_int,
        pub tm_year: ::core::ffi::c_int,
        pub tm_wday: ::core::ffi::c_int,
        pub tm_yday: ::core::ffi::c_int,
        pub tm_isdst: ::core::ffi::c_int,
        pub tm_gmtoff: ::core::ffi::c_long,
        pub tm_zone: *mut ::core::ffi::c_char,
    }
    use super::_time_t_h::time_t;
    extern "C" {
        
        pub static mut tzname: [*mut ::core::ffi::c_char; 0];
        
        pub fn localtime_r(_: *const time_t, _: *mut tm) -> *mut tm;
    }
}

pub mod time_h {
    
    pub type usec_t = uint64_t;
    
    pub const USEC: usec_t = 1000000 as ::core::ffi::c_int as usec_t;
    use super::_uint64_t_h::uint64_t;
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

pub mod sys_time_h {
    use super::_timeval_h::timeval;
    extern "C" {
        
        pub fn gettimeofday(_: *mut timeval, _: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    }
}

pub mod _null_h {
    
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
use self::_stdio_h::snprintf;
pub use self::_time_h::{localtime_r, tm, tzname};
pub use self::_time_t_h::time_t;
pub use self::_timeval_h::timeval;
pub use self::_types_h::{__darwin_size_t, __darwin_time_t, __int32_t};
pub use self::_uint64_t_h::uint64_t;
pub use self::sys__types_h::{__darwin_suseconds_t, __DARWIN_NULL};
use self::sys_time_h::gettimeofday;
pub use self::time_h::{usec_t, USEC};
#[no_mangle]

pub unsafe extern "C" fn format_time_ms(
    mut time: usec_t,
    mut dest: *mut ::core::ffi::c_char,
    mut destlen: ::core::ffi::c_uint,
) -> *mut ::core::ffi::c_char {
    let mut tm = ::core::ptr::null_mut::<tm>();
    let mut tmbuf = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut tv = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut sec: time_t = 0;
    if time == 0 {
        gettimeofday(&raw mut tv, NULL);
    } else {
        tv.tv_sec = time.wrapping_div(USEC) as __darwin_time_t;
        tv.tv_usec = time.wrapping_rem(USEC) as __darwin_suseconds_t;
    }
    sec = tv.tv_sec as time_t;
    tm = localtime_r(&raw mut sec, &raw mut tmbuf);
    snprintf(
        dest,
        destlen as size_t,
        b"%04d-%02d-%02d %02d:%02d:%02d.%03d %s\0" as *const u8 as *const ::core::ffi::c_char,
        (*tm).tm_year + 1900 as ::core::ffi::c_int,
        (*tm).tm_mon + 1 as ::core::ffi::c_int,
        (*tm).tm_mday,
        (*tm).tm_hour,
        (*tm).tm_min,
        (*tm).tm_sec,
        (tv.tv_usec / 1000 as __darwin_suseconds_t) as ::core::ffi::c_int,
        *(&raw mut tzname as *mut *mut ::core::ffi::c_char).offset(
            (if (*tm).tm_isdst > 0 as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as isize,
        ),
    );
    dest
}
#[no_mangle]

pub unsafe extern "C" fn format_time_s(
    mut time: usec_t,
    mut dest: *mut ::core::ffi::c_char,
    mut destlen: ::core::ffi::c_uint,
) -> *mut ::core::ffi::c_char {
    let mut s: time_t = 0;
    let mut tbuf = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut tm = ::core::ptr::null_mut::<tm>();
    if time == 0 {
        let mut tv = timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        gettimeofday(&raw mut tv, NULL);
        s = tv.tv_sec as time_t;
    } else {
        s = time.wrapping_div(USEC) as time_t;
    }
    tm = localtime_r(&raw mut s, &raw mut tbuf);
    snprintf(
        dest,
        destlen as size_t,
        b"%04d-%02d-%02d %02d:%02d:%02d %s\0" as *const u8 as *const ::core::ffi::c_char,
        (*tm).tm_year + 1900 as ::core::ffi::c_int,
        (*tm).tm_mon + 1 as ::core::ffi::c_int,
        (*tm).tm_mday,
        (*tm).tm_hour,
        (*tm).tm_min,
        (*tm).tm_sec,
        *(&raw mut tzname as *mut *mut ::core::ffi::c_char).offset(
            (if (*tm).tm_isdst > 0 as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as isize,
        ),
    );
    dest
}
#[no_mangle]

pub unsafe extern "C" fn get_time_usec() -> usec_t {
    let mut tv = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    gettimeofday(&raw mut tv, NULL);
    (tv.tv_sec as usec_t)
        .wrapping_mul(USEC)
        .wrapping_add(tv.tv_usec as usec_t)
}

static mut _time_cache: usec_t = 0;
#[no_mangle]

pub unsafe extern "C" fn get_cached_time() -> usec_t {
    if _time_cache == 0 {
        _time_cache = get_time_usec();
    }
    _time_cache
}
#[no_mangle]

pub unsafe extern "C" fn reset_time_cache() {
    _time_cache = 0 as usec_t;
}
