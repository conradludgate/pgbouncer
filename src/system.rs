#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:23"]
pub mod _types_h {
    #[c2rust::src_loc = "34:1"]
    pub type __uint16_t = u16;
    #[c2rust::src_loc = "36:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "119:1"]
    pub type __darwin_time_t = ::core::ffi::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:23"]
pub mod sys__types_h {
    #[c2rust::src_loc = "72:1"]
    pub type __darwin_gid_t = __uint32_t;
    #[c2rust::src_loc = "82:1"]
    pub type __darwin_mode_t = __uint16_t;
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_uid_t = __uint32_t;
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    use super::_types_h::{__uint16_t, __uint32_t};
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_gid_t.h:23"]
pub mod _gid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type gid_t = __darwin_gid_t;
    use super::sys__types_h::__darwin_gid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_mode_t.h:23"]
pub mod _mode_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type mode_t = __darwin_mode_t;
    use super::sys__types_h::__darwin_mode_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uid_t.h:23"]
pub mod _uid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uid_t = __darwin_uid_t;
    use super::sys__types_h::__darwin_uid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/pwd.h:37"]
pub mod pwd_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "84:1"]
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
    use super::_gid_t_h::gid_t;
    use super::_types_h::__darwin_time_t;
    use super::_uid_t_h::uid_t;
    extern "C" {
        #[c2rust::src_loc = "100:1"]
        pub fn getpwuid(_: uid_t) -> *mut passwd;
        #[c2rust::src_loc = "101:1"]
        pub fn getpwnam(_: *const ::core::ffi::c_char) -> *mut passwd;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/logging.h:23"]
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
        #[c2rust::src_loc = "117:1"]
        pub fn log_generic(
            level: LogLevel,
            ctx: *mut ::core::ffi::c_void,
            s: *const ::core::ffi::c_char,
            ...
        );
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/grp.h:40"]
pub mod grp_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "53:1"]
    pub struct group {
        pub gr_name: *mut ::core::ffi::c_char,
        pub gr_passwd: *mut ::core::ffi::c_char,
        pub gr_gid: gid_t,
        pub gr_mem: *mut *mut ::core::ffi::c_char,
    }
    use super::_gid_t_h::gid_t;
    extern "C" {
        #[c2rust::src_loc = "65:1"]
        pub fn getgrnam(_: *const ::core::ffi::c_char) -> *mut group;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/unistd.h:23"]
pub mod unistd_h {
    use super::_gid_t_h::gid_t;
    use super::_uid_t_h::uid_t;
    extern "C" {
        #[c2rust::src_loc = "439:1"]
        pub fn chown(_: *const ::core::ffi::c_char, _: uid_t, _: gid_t) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "455:1"]
        pub fn geteuid() -> uid_t;
        #[c2rust::src_loc = "466:1"]
        pub fn getuid() -> uid_t;
        #[c2rust::src_loc = "479:1"]
        pub fn setgid(_: gid_t) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "482:1"]
        pub fn setuid(_: uid_t) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "663:1"]
        pub fn getpeereid(
            _: ::core::ffi::c_int,
            _: *mut uid_t,
            _: *mut gid_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "717:1"]
        pub fn setgroups(_: ::core::ffi::c_int, _: *const gid_t) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/stat.h:23"]
pub mod stat_h {
    use super::_mode_t_h::mode_t;
    extern "C" {
        #[c2rust::src_loc = "381:1"]
        pub fn chmod(_: *const ::core::ffi::c_char, _: mode_t) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:23"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdlib.h:23"]
pub mod _stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "160:1"]
        pub fn exit(_: ::core::ffi::c_int) -> !;
        #[c2rust::src_loc = "193:1"]
        pub fn strtoul(
            __str: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
            __base: ::core::ffi::c_int,
        ) -> ::core::ffi::c_ulong;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/errno.h:23"]
pub mod errno_h {
    extern "C" {
        #[c2rust::src_loc = "80:1"]
        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/llvm/21.1.8/lib/clang/21/include/stdbool.h:23"]
pub mod stdbool_h {
    #[c2rust::src_loc = "26:9"]
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:23"]
pub mod _string_h {
    extern "C" {
        #[c2rust::src_loc = "89:1"]
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "95:1"]
        pub fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    }
}
pub use self::_gid_t_h::gid_t;
pub use self::_mode_t_h::mode_t;
pub use self::_null_h::NULL;
use self::_stdlib_h::{exit, strtoul};
use self::_string_h::{strcmp, strerror};
pub use self::_types_h::{__darwin_time_t, __uint16_t, __uint32_t};
pub use self::_uid_t_h::uid_t;
use self::errno_h::__error;
pub use self::grp_h::{getgrnam, group};
pub use self::logging_h::{
    log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE, LG_STATS, LG_WARNING,
};
pub use self::pwd_h::{getpwnam, getpwuid, passwd};
use self::stat_h::chmod;
pub use self::stdbool_h::false_0;
pub use self::sys__types_h::{__darwin_gid_t, __darwin_mode_t, __darwin_uid_t, __DARWIN_NULL};
use self::unistd_h::{chown, geteuid, getpeereid, getuid, setgid, setgroups, setuid};
#[no_mangle]
#[c2rust::src_loc = "43:1"]
pub unsafe extern "C" fn change_user(mut user: *const ::core::ffi::c_char) {
    let mut pw = ::core::ptr::null::<passwd>();
    let mut gset: [gid_t; 1] = [0; 1];
    pw = getpwnam(user);
    if pw.is_null() {
        let mut _log_ctx = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx,
            b"could not find user '%s' to switch to\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"failed to reset groups: %s\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"failed to assume identity of user '%s': %s\0" as *const u8
                as *const ::core::ffi::c_char,
            user,
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    if getuid() != (*pw).pw_uid || geteuid() != (*pw).pw_uid {
        let mut _log_ctx_2 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_2,
            b"setuid() failed to work\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
}
#[no_mangle]
#[c2rust::src_loc = "71:1"]
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
        && *user_name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
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
                    b"could not find user '%s': %s\0" as *const u8 as *const ::core::ffi::c_char,
                    user_name,
                    strerror(*__error()),
                );
                exit(1 as ::core::ffi::c_int);
            }
            uid = (*pw).pw_uid;
        }
    }
    if !group_name.is_null()
        && *group_name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
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
                    b"could not find group '%s': %s\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"chown(%s, %u, %u) failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
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
            b"failure to chmod(%s, 0%o): %s\0" as *const u8 as *const ::core::ffi::c_char,
            fn_0,
            mode as ::core::ffi::c_int,
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
}
#[no_mangle]
#[c2rust::src_loc = "137:1"]
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
        return false_0 != 0;
    }
    pw = getpwuid(peer_uid);
    if pw.is_null() {
        return false_0 != 0;
    }
    strcmp((*pw).pw_name, username) == 0 as ::core::ffi::c_int
}
