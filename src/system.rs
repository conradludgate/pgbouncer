pub mod pwd_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct passwd {
        pub pw_name: *mut ::core::ffi::c_char,
        pub pw_passwd: *mut ::core::ffi::c_char,
        pub pw_uid: uid_t,
        pub pw_gid: gid_t,
        pub pw_change: __darwin_time_t,
        pub pw_class: *mut ::core::ffi::c_char,
        pub pw_gecos: *mut ::core::ffi::c_char,
        pub pw_dir: *mut ::core::ffi::c_char,
        pub pw_shell: *mut ::core::ffi::c_char,
        pub pw_expire: __darwin_time_t,
    }
    use crate::types::gid_t;
    use crate::types::__darwin_time_t;
    use crate::types::uid_t;
    extern "C" {

        pub fn getpwuid(_: uid_t) -> *mut passwd;

        pub fn getpwnam(_: *const ::core::ffi::c_char) -> *mut passwd;
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

        pub fn log_generic(
            level: LogLevel,
            ctx: *mut ::core::ffi::c_void,
            s: *const ::core::ffi::c_char,
            ...
        );
    }
}

pub mod grp_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct group {
        pub gr_name: *mut ::core::ffi::c_char,
        pub gr_passwd: *mut ::core::ffi::c_char,
        pub gr_gid: gid_t,
        pub gr_mem: *mut *mut ::core::ffi::c_char,
    }
    use crate::types::gid_t;
    extern "C" {

        pub fn getgrnam(_: *const ::core::ffi::c_char) -> *mut group;
    }
}

pub mod unistd_h {
    use crate::types::gid_t;
    use crate::types::uid_t;
    extern "C" {

        pub fn chown(_: *const ::core::ffi::c_char, _: uid_t, _: gid_t) -> ::core::ffi::c_int;

        pub fn geteuid() -> uid_t;

        pub fn getuid() -> uid_t;

        pub fn setgid(_: gid_t) -> ::core::ffi::c_int;

        pub fn setuid(_: uid_t) -> ::core::ffi::c_int;

        pub fn getpeereid(
            _: ::core::ffi::c_int,
            _: *mut uid_t,
            _: *mut gid_t,
        ) -> ::core::ffi::c_int;

        pub fn setgroups(_: ::core::ffi::c_int, _: *const gid_t) -> ::core::ffi::c_int;
    }
}

pub mod stat_h {
    use crate::types::mode_t;
    extern "C" {

        pub fn chmod(_: *const ::core::ffi::c_char, _: mode_t) -> ::core::ffi::c_int;
    }
}

pub mod _stdlib_h {
    extern "C" {

        pub fn exit(_: ::core::ffi::c_int) -> !;

        pub fn strtoul(
            __str: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
            __base: ::core::ffi::c_int,
        ) -> ::core::ffi::c_ulong;
    }
}

pub mod errno_h {
    extern "C" {

        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}

pub mod _string_h {
    extern "C" {

        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;

        pub fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    }
}
pub use crate::types::gid_t;
pub use crate::types::mode_t;
use self::_stdlib_h::{exit, strtoul};
use self::_string_h::{strcmp, strerror};
use self::errno_h::__error;
pub use self::grp_h::{getgrnam, group};
pub use self::logging_h::{
    log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE, LG_STATS, LG_WARNING,
};
pub use self::pwd_h::{getpwnam, getpwuid, passwd};
use self::stat_h::chmod;
pub use crate::types::{__darwin_gid_t, __darwin_mode_t, __darwin_uid_t, __DARWIN_NULL};
use self::unistd_h::{chown, geteuid, getpeereid, getuid, setgid, setgroups, setuid};
pub use crate::types::false_0;
pub use crate::types::uid_t;
pub use crate::types::NULL;
pub use crate::types::{__darwin_time_t, __uint16_t, __uint32_t};
#[no_mangle]

pub unsafe extern "C" fn change_user(mut user: *const ::core::ffi::c_char) {
    let mut pw = ::core::ptr::null::<passwd>();
    let mut gset: [gid_t; 1] = [0; 1];
    pw = getpwnam(user);
    if pw.is_null() {
        let mut _log_ctx = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx,
            c"could not find user '%s' to switch to".as_ptr(),
            user,
        );
        exit(1 as ::core::ffi::c_int);
    }
    gset[0 as ::core::ffi::c_int as usize] = (*pw).pw_gid;
    if getuid() == 0 as uid_t
        && setgroups(1 as ::core::ffi::c_int, &raw mut gset as *mut gid_t) < 0 as ::core::ffi::c_int
    {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_0,
            c"failed to reset groups: %s".as_ptr(),
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    if setgid((*pw).pw_gid) < 0 as ::core::ffi::c_int
        || setuid((*pw).pw_uid) < 0 as ::core::ffi::c_int
    {
        let mut _log_ctx_1 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_1,
            c"failed to assume identity of user '%s': %s".as_ptr(),
            user,
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    if getuid() != (*pw).pw_uid || geteuid() != (*pw).pw_uid {
        let mut _log_ctx_2 = NULL;
        log_generic(LG_FATAL, _log_ctx_2, c"setuid() failed to work".as_ptr());
        exit(1 as ::core::ffi::c_int);
    }
}
#[no_mangle]

pub unsafe extern "C" fn change_file_mode(
    mut fn_0: *const ::core::ffi::c_char,
    mut mode: mode_t,
    mut user_name: *const ::core::ffi::c_char,
    mut group_name: *const ::core::ffi::c_char,
) {
    let mut res: ::core::ffi::c_int = 0;
    let mut uid: uid_t = -(1 as ::core::ffi::c_int) as uid_t;
    let mut gid: gid_t = -(1 as ::core::ffi::c_int) as gid_t;
    let mut val: ::core::ffi::c_ulong = 0;
    let mut end = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !user_name.is_null()
        && *user_name as ::core::ffi::c_int != 0
    {
        let mut pw = ::core::ptr::null::<passwd>();
        val = strtoul(user_name, &raw mut end, 0 as ::core::ffi::c_int);
        if *end as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            uid = val as uid_t;
        } else {
            pw = getpwnam(user_name);
            if pw.is_null() {
                let mut _log_ctx = NULL;
                log_generic(
                    LG_FATAL,
                    _log_ctx,
                    c"could not find user '%s': %s".as_ptr(),
                    user_name,
                    strerror(*__error()),
                );
                exit(1 as ::core::ffi::c_int);
            }
            uid = (*pw).pw_uid;
        }
    }
    if !group_name.is_null()
        && *group_name as ::core::ffi::c_int != 0
    {
        let mut gr = ::core::ptr::null_mut::<group>();
        val = strtoul(group_name, &raw mut end, 0 as ::core::ffi::c_int);
        if *end as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            gid = val as gid_t;
        } else {
            gr = getgrnam(group_name);
            if gr.is_null() {
                let mut _log_ctx_0 = NULL;
                log_generic(
                    LG_FATAL,
                    _log_ctx_0,
                    c"could not find group '%s': %s".as_ptr(),
                    group_name,
                    strerror(*__error()),
                );
                exit(1 as ::core::ffi::c_int);
            }
            gid = (*gr).gr_gid;
        }
    }
    if uid != -(1 as ::core::ffi::c_int) as uid_t || gid != -(1 as ::core::ffi::c_int) as gid_t {
        res = chown(fn_0, uid, gid);
        if res != 0 as ::core::ffi::c_int {
            let mut _log_ctx_1 = NULL;
            log_generic(
                LG_FATAL,
                _log_ctx_1,
                c"chown(%s, %u, %u) failed: %s".as_ptr(),
                fn_0,
                uid,
                gid,
                strerror(*__error()),
            );
            exit(1 as ::core::ffi::c_int);
        }
    }
    res = chmod(fn_0, mode);
    if res != 0 as ::core::ffi::c_int {
        let mut _log_ctx_2 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_2,
            c"failure to chmod(%s, 0%o): %s".as_ptr(),
            fn_0,
            mode as ::core::ffi::c_int,
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
}
#[no_mangle]

pub unsafe extern "C" fn check_unix_peer_name(
    mut fd: ::core::ffi::c_int,
    mut username: *const ::core::ffi::c_char,
) -> bool {
    let mut res: ::core::ffi::c_int = 0;
    let mut peer_uid: uid_t = -(1 as ::core::ffi::c_int) as uid_t;
    let mut peer_gid: gid_t = -(1 as ::core::ffi::c_int) as gid_t;
    let mut pw = ::core::ptr::null_mut::<passwd>();
    res = getpeereid(fd, &raw mut peer_uid, &raw mut peer_gid);
    if res < 0 as ::core::ffi::c_int {
        return false;
    }
    pw = getpwuid(peer_uid);
    if pw.is_null() {
        return false;
    }
    strcmp((*pw).pw_name, username) == 0 as ::core::ffi::c_int
}
