#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = *mut ::core::ffi::c_char;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:19"]
pub mod _types_h {
    #[c2rust::src_loc = "35:1"]
    pub type __int32_t = i32;
    #[c2rust::src_loc = "36:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "37:1"]
    pub type __int64_t = i64;
    #[c2rust::src_loc = "61:1"]
    pub type __darwin_ct_rune_t = ::core::ffi::c_int;
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
    #[c2rust::src_loc = "95:1"]
    pub type __darwin_va_list = __builtin_va_list;
    #[c2rust::src_loc = "103:1"]
    pub type __darwin_wchar_t = ::libc::wchar_t;
    #[c2rust::src_loc = "108:1"]
    pub type __darwin_rune_t = __darwin_wchar_t;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:19"]
pub mod sys__types_h {
    #[c2rust::src_loc = "83:1"]
    pub type __darwin_off_t = __int64_t;
    #[c2rust::src_loc = "84:1"]
    pub type __darwin_pid_t = __int32_t;
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    use super::_types_h::{__int32_t, __int64_t};
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_pid_t.h:19"]
pub mod _pid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pid_t = __darwin_pid_t;
    use super::sys__types_h::__darwin_pid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:19"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint64_t.h:19"]
pub mod _uint64_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint64_t = u64;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_va_list.h:19"]
pub mod _va_list_h {
    #[c2rust::src_loc = "44:1"]
    pub type va_list = __darwin_va_list;
    use super::_types_h::__darwin_va_list;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:19"]
pub mod _stdio_h {
    #[c2rust::src_loc = "86:1"]
    pub type fpos_t = __darwin_off_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "97:1"]
    pub struct __sbuf {
        pub _base: *mut ::core::ffi::c_uchar,
        pub _size: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "131:9"]
    pub struct __sFILE {
        pub _p: *mut ::core::ffi::c_uchar,
        pub _r: ::core::ffi::c_int,
        pub _w: ::core::ffi::c_int,
        pub _flags: ::core::ffi::c_short,
        pub _file: ::core::ffi::c_short,
        pub _bf: __sbuf,
        pub _lbfsize: ::core::ffi::c_int,
        pub _cookie: *mut ::core::ffi::c_void,
        pub _close: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
        pub _read: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub _seek: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void, fpos_t, ::core::ffi::c_int) -> fpos_t,
        >,
        pub _write: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub _ub: __sbuf,
        pub _extra: *mut __sFILEX,
        pub _ur: ::core::ffi::c_int,
        pub _ubuf: [::core::ffi::c_uchar; 3],
        pub _nbuf: [::core::ffi::c_uchar; 1],
        pub _lb: __sbuf,
        pub _blksize: ::core::ffi::c_int,
        pub _offset: fpos_t,
    }
    #[c2rust::src_loc = "131:1"]
    pub type FILE = __sFILE;
    #[c2rust::src_loc = "201:9"]
    pub const _IONBF: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    use super::_size_t_h::size_t;

    use super::sys__types_h::__darwin_off_t;
    extern "C" {
        #[c2rust::src_loc = "103:1"]
        pub type __sFILEX;
        #[c2rust::src_loc = "169:1"]
        pub static mut __stderrp: *mut FILE;
        #[c2rust::src_loc = "233:1"]
        pub fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "243:1"]
        pub fn fopen(
            __filename: *const ::core::ffi::c_char,
            __mode: *const ::core::ffi::c_char,
        ) -> *mut FILE;
        #[c2rust::src_loc = "245:1"]
        pub fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "273:1"]
        pub fn setvbuf(
            _: *mut FILE,
            _: *mut ::core::ffi::c_char,
            _: ::core::ffi::c_int,
            __size: size_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "438:1"]
        pub fn vsnprintf(
            __str: *mut ::core::ffi::c_char,
            __size: size_t,
            __format: *const ::core::ffi::c_char,
            _: ::core::ffi::VaList,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/logging.h:19"]
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
    #[c2rust::src_loc = "69:1"]
    pub type logging_prefix_fn_t = Option<
        unsafe extern "C" fn(
            LogLevel,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int,
    >;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/time.h:25"]
pub mod time_h {
    #[c2rust::src_loc = "40:1"]
    pub type usec_t = uint64_t;
    use super::_uint64_t_h::uint64_t;
    extern "C" {
        #[c2rust::src_loc = "46:1"]
        pub fn format_time_ms(
            time: usec_t,
            dest: *mut ::core::ffi::c_char,
            destlen: ::core::ffi::c_uint,
        ) -> *mut ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/runetype.h:23"]
pub mod runetype_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "80:9"]
    pub struct _RuneLocale {
        pub __magic: [::core::ffi::c_char; 8],
        pub __encoding: [::core::ffi::c_char; 32],
        pub __sgetrune: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                __darwin_size_t,
                *mut *const ::core::ffi::c_char,
            ) -> __darwin_rune_t,
        >,
        pub __sputrune: Option<
            unsafe extern "C" fn(
                __darwin_rune_t,
                *mut ::core::ffi::c_char,
                __darwin_size_t,
                *mut *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub __invalid_rune: __darwin_rune_t,
        pub __runetype: [__uint32_t; 256],
        pub __maplower: [__darwin_rune_t; 256],
        pub __mapupper: [__darwin_rune_t; 256],
        pub __runetype_ext: _RuneRange,
        pub __maplower_ext: _RuneRange,
        pub __mapupper_ext: _RuneRange,
        pub __variable: *mut ::core::ffi::c_void,
        pub __variable_len: ::core::ffi::c_int,
        pub __ncharclasses: ::core::ffi::c_int,
        pub __charclasses: *mut _RuneCharClass,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "75:9"]
    pub struct _RuneCharClass {
        pub __name: [::core::ffi::c_char; 14],
        pub __mask: __uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "70:9"]
    pub struct _RuneRange {
        pub __nranges: ::core::ffi::c_int,
        pub __ranges: *mut _RuneEntry,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:9"]
    pub struct _RuneEntry {
        pub __min: __darwin_rune_t,
        pub __max: __darwin_rune_t,
        pub __map: __darwin_rune_t,
        pub __types: *mut __uint32_t,
    }
    use super::_types_h::{__darwin_rune_t, __darwin_size_t, __uint32_t};
    extern "C" {
        #[c2rust::src_loc = "114:1"]
        pub static mut _DefaultRuneLocale: _RuneLocale;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/unistd.h:19"]
pub mod unistd_h {
    use super::_pid_t_h::pid_t;
    extern "C" {
        #[c2rust::src_loc = "464:1"]
        pub fn getpid() -> pid_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:19"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/string.h:24"]
pub mod string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "166:1"]
        pub fn usual_strerror_r(
            e: ::core::ffi::c_int,
            dst: *mut ::core::ffi::c_char,
            dstlen: size_t,
        ) -> *const ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/ctype.h:23"]
pub mod ctype_h {
    #[inline]
    #[c2rust::src_loc = "105:1"]
    pub unsafe extern "C" fn safe_isspace(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        return isspace(c as ::core::ffi::c_uchar as ::core::ffi::c_int);
    }
    use super::_ctype_h::isspace;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_ctype.h:23"]
pub mod _ctype_h {
    #[c2rust::src_loc = "82:9"]
    pub const _CTYPE_S: ::core::ffi::c_long = 0x4000 as ::core::ffi::c_long;
    #[inline]
    #[c2rust::src_loc = "139:1"]
    pub unsafe extern "C" fn isascii(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        return (_c & !(0x7f as ::core::ffi::c_int) == 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "157:1"]
    pub unsafe extern "C" fn __istype(
        mut _c: __darwin_ct_rune_t,
        mut _f: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int {
        return if isascii(_c as ::core::ffi::c_int) != 0 {
            (_DefaultRuneLocale.__runetype[_c as usize] as ::core::ffi::c_ulong & _f != 0)
                as ::core::ffi::c_int
        } else {
            (__maskrune(_c, _f) != 0) as ::core::ffi::c_int
        };
    }
    #[inline]
    #[c2rust::src_loc = "271:1"]
    pub unsafe extern "C" fn isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        return __istype(_c as __darwin_ct_rune_t, _CTYPE_S as ::core::ffi::c_ulong);
    }
    use super::_types_h::__darwin_ct_rune_t;
    use super::runetype_h::_DefaultRuneLocale;
    extern "C" {
        #[c2rust::src_loc = "153:1"]
        pub fn __maskrune(_: __darwin_ct_rune_t, _: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdlib.h:19"]
pub mod _stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "160:1"]
        pub fn exit(_: ::core::ffi::c_int) -> !;
        #[c2rust::src_loc = "324:1"]
        pub fn getprogname() -> *const ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/errno.h:19"]
pub mod errno_h {
    extern "C" {
        #[c2rust::src_loc = "80:1"]
        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/llvm/21.1.8/lib/clang/21/include/stdbool.h:19"]
pub mod stdbool_h {
    #[c2rust::src_loc = "26:9"]
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:24"]
pub mod _string_h {
    extern "C" {
        #[c2rust::src_loc = "88:1"]
        pub fn strchr(
            __s: *const ::core::ffi::c_char,
            __c: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "89:1"]
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/syslog.h:29"]
pub mod syslog_h {
    #[c2rust::src_loc = "80:9"]
    pub const LOG_CRIT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "81:9"]
    pub const LOG_ERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "82:9"]
    pub const LOG_WARNING: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    #[c2rust::src_loc = "84:9"]
    pub const LOG_INFO: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
    #[c2rust::src_loc = "85:9"]
    pub const LOG_DEBUG: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
    #[c2rust::src_loc = "120:9"]
    pub const LOG_USER: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "122:9"]
    pub const LOG_DAEMON: ::core::ffi::c_int = (3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "123:9"]
    pub const LOG_AUTH: ::core::ffi::c_int = (4 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "129:9"]
    pub const LOG_AUTHPRIV: ::core::ffi::c_int =
        (10 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "143:9"]
    pub const LOG_LOCAL0: ::core::ffi::c_int =
        (16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "144:9"]
    pub const LOG_LOCAL1: ::core::ffi::c_int =
        (17 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "145:9"]
    pub const LOG_LOCAL2: ::core::ffi::c_int =
        (18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "146:9"]
    pub const LOG_LOCAL3: ::core::ffi::c_int =
        (19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "147:9"]
    pub const LOG_LOCAL4: ::core::ffi::c_int =
        (20 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "148:9"]
    pub const LOG_LOCAL5: ::core::ffi::c_int =
        (21 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "149:9"]
    pub const LOG_LOCAL6: ::core::ffi::c_int =
        (22 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "150:9"]
    pub const LOG_LOCAL7: ::core::ffi::c_int =
        (23 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "205:9"]
    pub const LOG_PID: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    extern "C" {
        #[c2rust::src_loc = "223:1"]
        pub fn closelog();
        #[c2rust::src_loc = "224:1"]
        pub fn openlog(_: *const ::core::ffi::c_char, _: ::core::ffi::c_int, _: ::core::ffi::c_int);
        #[c2rust::src_loc = "227:1"]
        pub fn syslog(_: ::core::ffi::c_int, _: *const ::core::ffi::c_char, ...);
    }
}
pub use self::_ctype_h::{__istype, __maskrune, isascii, isspace, _CTYPE_S};
pub use self::_null_h::NULL;
pub use self::_pid_t_h::pid_t;
pub use self::_size_t_h::size_t;
pub use self::_stdio_h::{
    __sFILE, __sFILEX, __sbuf, __stderrp, fclose, fopen, fpos_t, fprintf, setvbuf, vsnprintf, FILE,
    _IONBF,
};
use self::_stdlib_h::{exit, getprogname};
use self::_string_h::{strchr, strcmp};
pub use self::_types_h::{
    __darwin_ct_rune_t, __darwin_rune_t, __darwin_size_t, __darwin_va_list, __darwin_wchar_t,
    __int32_t, __int64_t, __uint32_t,
};
pub use self::_uint64_t_h::uint64_t;
pub use self::_va_list_h::va_list;
pub use self::ctype_h::safe_isspace;
use self::errno_h::__error;
pub use self::internal::__builtin_va_list;
pub use self::logging_h::{
    logging_prefix_fn_t, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE, LG_STATS,
    LG_WARNING,
};
pub use self::runetype_h::{
    _DefaultRuneLocale, _RuneCharClass, _RuneEntry, _RuneLocale, _RuneRange,
};
pub use self::stdbool_h::false_0;
use self::string_h::usual_strerror_r;
pub use self::sys__types_h::{__darwin_off_t, __darwin_pid_t, __DARWIN_NULL};
pub use self::syslog_h::{
    closelog, openlog, syslog, LOG_AUTH, LOG_AUTHPRIV, LOG_CRIT, LOG_DAEMON, LOG_DEBUG, LOG_ERR,
    LOG_INFO, LOG_LOCAL0, LOG_LOCAL1, LOG_LOCAL2, LOG_LOCAL3, LOG_LOCAL4, LOG_LOCAL5, LOG_LOCAL6,
    LOG_LOCAL7, LOG_PID, LOG_USER, LOG_WARNING,
};
pub use self::time_h::{format_time_ms, usec_t};
use self::unistd_h::getpid;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "76:1"]
pub struct LevelInfo {
    pub tag: *const ::core::ffi::c_char,
    pub syslog_prio: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "91:1"]
pub struct FacName {
    pub name: *const ::core::ffi::c_char,
    pub code: ::core::ffi::c_int,
}
#[no_mangle]
#[c2rust::src_loc = "58:1"]
pub static mut cf_quiet: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
#[c2rust::src_loc = "59:1"]
pub static mut cf_verbose: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
#[c2rust::src_loc = "60:1"]
pub static mut cf_logfile: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
#[no_mangle]
#[c2rust::src_loc = "62:1"]
pub static mut cf_syslog: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
#[c2rust::src_loc = "63:1"]
pub static mut cf_syslog_ident: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
#[no_mangle]
#[c2rust::src_loc = "64:1"]
pub static mut cf_syslog_facility: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
#[no_mangle]
#[c2rust::src_loc = "66:1"]
pub static mut cf_syslog_level: LogLevel = LG_INFO;
#[no_mangle]
#[c2rust::src_loc = "67:1"]
pub static mut cf_logfile_level: LogLevel = LG_NOISE;
#[no_mangle]
#[c2rust::src_loc = "68:1"]
pub static mut cf_stderr_level: LogLevel = LG_NOISE;
#[no_mangle]
#[c2rust::src_loc = "71:1"]
pub static mut logging_prefix_cb: logging_prefix_fn_t = None;
#[c2rust::src_loc = "73:1"]
static mut log_file: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
#[c2rust::src_loc = "74:1"]
static mut syslog_started: bool = false_0 != 0;
#[c2rust::src_loc = "81:1"]
static mut log_level_list: [LevelInfo; 7] = [
    LevelInfo {
        tag: b"FATAL\0" as *const u8 as *const ::core::ffi::c_char,
        syslog_prio: LOG_CRIT,
    },
    LevelInfo {
        tag: b"ERROR\0" as *const u8 as *const ::core::ffi::c_char,
        syslog_prio: LOG_ERR,
    },
    LevelInfo {
        tag: b"WARNING\0" as *const u8 as *const ::core::ffi::c_char,
        syslog_prio: LOG_WARNING,
    },
    LevelInfo {
        tag: b"LOG\0" as *const u8 as *const ::core::ffi::c_char,
        syslog_prio: LOG_INFO,
    },
    LevelInfo {
        tag: b"LOG\0" as *const u8 as *const ::core::ffi::c_char,
        syslog_prio: LOG_INFO,
    },
    LevelInfo {
        tag: b"DEBUG\0" as *const u8 as *const ::core::ffi::c_char,
        syslog_prio: LOG_DEBUG,
    },
    LevelInfo {
        tag: b"NOISE\0" as *const u8 as *const ::core::ffi::c_char,
        syslog_prio: LOG_DEBUG,
    },
];
#[c2rust::src_loc = "92:1"]
static mut facility_names: [FacName; 13] = [
    FacName {
        name: b"auth\0" as *const u8 as *const ::core::ffi::c_char,
        code: LOG_AUTH,
    },
    FacName {
        name: b"authpriv\0" as *const u8 as *const ::core::ffi::c_char,
        code: LOG_AUTHPRIV,
    },
    FacName {
        name: b"daemon\0" as *const u8 as *const ::core::ffi::c_char,
        code: LOG_DAEMON,
    },
    FacName {
        name: b"user\0" as *const u8 as *const ::core::ffi::c_char,
        code: LOG_USER,
    },
    FacName {
        name: b"local0\0" as *const u8 as *const ::core::ffi::c_char,
        code: LOG_LOCAL0,
    },
    FacName {
        name: b"local1\0" as *const u8 as *const ::core::ffi::c_char,
        code: LOG_LOCAL1,
    },
    FacName {
        name: b"local2\0" as *const u8 as *const ::core::ffi::c_char,
        code: LOG_LOCAL2,
    },
    FacName {
        name: b"local3\0" as *const u8 as *const ::core::ffi::c_char,
        code: LOG_LOCAL3,
    },
    FacName {
        name: b"local4\0" as *const u8 as *const ::core::ffi::c_char,
        code: LOG_LOCAL4,
    },
    FacName {
        name: b"local5\0" as *const u8 as *const ::core::ffi::c_char,
        code: LOG_LOCAL5,
    },
    FacName {
        name: b"local6\0" as *const u8 as *const ::core::ffi::c_char,
        code: LOG_LOCAL6,
    },
    FacName {
        name: b"local7\0" as *const u8 as *const ::core::ffi::c_char,
        code: LOG_LOCAL7,
    },
    FacName {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        code: 0,
    },
];
#[no_mangle]
#[c2rust::src_loc = "112:1"]
pub unsafe extern "C" fn reset_logging() {
    if !log_file.is_null() {
        fclose(log_file);
        log_file = ::core::ptr::null_mut::<FILE>();
    }
    if syslog_started {
        closelog();
        syslog_started = 0 as ::core::ffi::c_int != 0;
    }
}
#[c2rust::src_loc = "125:1"]
unsafe extern "C" fn start_syslog() {
    let mut f = ::core::ptr::null::<FacName>();
    let mut fac = LOG_DAEMON;
    let mut ident = cf_syslog_ident;
    if cf_syslog == 0 {
        return;
    }
    if !cf_syslog_facility.is_null() {
        f = &raw const facility_names as *const FacName;
        while !(*f).name.is_null() {
            if strcmp((*f).name, cf_syslog_facility) == 0 as ::core::ffi::c_int {
                fac = (*f).code;
                break;
            } else {
                f = f.offset(1);
            }
        }
    }
    if ident.is_null() {
        ident = getprogname();
        if ident.is_null() {
            ident = b"unnamed\0" as *const u8 as *const ::core::ffi::c_char;
        }
    }
    openlog(ident, LOG_PID, fac);
    syslog_started = 1 as ::core::ffi::c_int != 0;
}
#[no_mangle]
#[c2rust::src_loc = "154:1"]
pub unsafe extern "C" fn log_generic(
    mut level: LogLevel,
    mut ctx: *mut ::core::ffi::c_void,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut current_block: u64;
    let mut buf: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut buf2: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut ebuf: [::core::ffi::c_char; 256] = [0; 256];
    let mut timebuf: [::core::ffi::c_char; 64] = [0; 64];
    let mut lev: *const LevelInfo =
        (&raw const log_level_list as *const LevelInfo).offset(level as isize) as *const LevelInfo;
    let mut pid = getpid() as ::core::ffi::c_uint;
    let mut ap: ::core::ffi::VaListImpl;
    let mut pfxlen = 0 as ::core::ffi::c_int;
    let mut old_errno = *__error();
    let mut msg = &raw mut buf as *mut ::core::ffi::c_char;
    if logging_prefix_cb.is_some() {
        pfxlen = logging_prefix_cb.expect("non-null function pointer")(
            level,
            ctx,
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as ::core::ffi::c_uint,
        );
        if pfxlen < 0 as ::core::ffi::c_int {
            current_block = 9841131127325725570;
        } else {
            if pfxlen >= ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as ::core::ffi::c_int
            {
                pfxlen = (::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as usize)
                    .wrapping_sub(1 as usize) as ::core::ffi::c_int;
            }
            current_block = 10886091980245723256;
        }
    } else {
        current_block = 10886091980245723256;
    }
    match current_block {
        10886091980245723256 => {
            ap = args.clone();
            vsnprintf(
                (&raw mut buf as *mut ::core::ffi::c_char).offset(pfxlen as isize),
                (::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as size_t)
                    .wrapping_sub(pfxlen as size_t),
                fmt,
                ap.as_va_list(),
            );
            if !strchr(msg, '\n' as i32).is_null() {
                let mut dst = &raw mut buf2 as *mut ::core::ffi::c_char;
                while *msg as ::core::ffi::c_int != 0
                    && (dst.offset_from(&raw mut buf2 as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long)
                        < (::core::mem::size_of::<[::core::ffi::c_char; 2048]>()
                            as ::core::ffi::c_int
                            - 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long
                {
                    let fresh0 = dst;
                    dst = dst.offset(1);
                    *fresh0 = *msg;
                    if *msg as ::core::ffi::c_int == '\n' as i32 {
                        let fresh1 = dst;
                        dst = dst.offset(1);
                        *fresh1 = '\t' as i32 as ::core::ffi::c_char;
                    }
                    msg = msg.offset(1);
                }
                while dst > &raw mut buf2 as *mut ::core::ffi::c_char
                    && safe_isspace(
                        *dst.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    ) != 0
                {
                    dst = dst.offset(-1);
                }
                *dst = 0 as ::core::ffi::c_char;
                msg = &raw mut buf2 as *mut ::core::ffi::c_char;
            }
            format_time_ms(
                0 as usec_t,
                &raw mut timebuf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as ::core::ffi::c_uint,
            );
            if log_file.is_null()
                && !cf_logfile.is_null()
                && *cf_logfile.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
            {
                log_file = fopen(
                    cf_logfile,
                    b"a\0" as *const u8 as *const ::core::ffi::c_char,
                ) as *mut FILE;
                if !log_file.is_null() {
                    setvbuf(
                        log_file,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        _IONBF,
                        0 as size_t,
                    );
                } else {
                    fprintf(
                        __stderrp,
                        b"%s %u %s Cannot open logfile: '%s': %s\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &raw mut timebuf as *mut ::core::ffi::c_char,
                        pid,
                        log_level_list[0 as ::core::ffi::c_int as usize].tag,
                        cf_logfile,
                        usual_strerror_r(
                            *__error(),
                            &raw mut ebuf as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
                        ),
                    );
                    exit(1 as ::core::ffi::c_int);
                }
            }
            if cf_quiet == 0
                && level as ::core::ffi::c_uint <= cf_stderr_level as ::core::ffi::c_uint
            {
                fprintf(
                    __stderrp,
                    b"%s [%u] %s %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut timebuf as *mut ::core::ffi::c_char,
                    pid,
                    (*lev).tag,
                    msg,
                );
            }
            if !log_file.is_null()
                && level as ::core::ffi::c_uint <= cf_logfile_level as ::core::ffi::c_uint
            {
                fprintf(
                    log_file,
                    b"%s [%u] %s %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut timebuf as *mut ::core::ffi::c_char,
                    pid,
                    (*lev).tag,
                    msg,
                );
            }
            if cf_syslog != 0
                && level as ::core::ffi::c_uint <= cf_syslog_level as ::core::ffi::c_uint
            {
                if !syslog_started {
                    start_syslog();
                }
                syslog(
                    (*lev).syslog_prio,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    msg,
                );
            }
        }
        _ => {}
    }
    if old_errno != *__error() {
        *__error() = old_errno;
    }
}
#[no_mangle]
#[c2rust::src_loc = "249:1"]
pub unsafe extern "C" fn log_fatal(
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
    mut func: *const ::core::ffi::c_char,
    mut show_perror: bool,
    mut ctx: *mut ::core::ffi::c_void,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut buf: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut ebuf: [::core::ffi::c_char; 256] = [0; 256];
    let mut estr = ::core::ptr::null::<::core::ffi::c_char>();
    let mut old_errno = 0 as ::core::ffi::c_int;
    let mut ap: ::core::ffi::VaListImpl;
    if show_perror {
        old_errno = *__error();
        estr = usual_strerror_r(
            *__error(),
            &raw mut ebuf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
        );
    }
    ap = args.clone();
    vsnprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as size_t,
        fmt,
        ap.as_va_list(),
    );
    if show_perror {
        log_generic(
            LG_FATAL,
            ctx,
            b"@%s:%d in function %s(): %s: %s [%d]\0" as *const u8 as *const ::core::ffi::c_char,
            file,
            line,
            func,
            &raw mut buf as *mut ::core::ffi::c_char,
            estr,
            old_errno,
        );
    } else {
        log_generic(
            LG_FATAL,
            ctx,
            b"@%s:%d in function %s(): %s\0" as *const u8 as *const ::core::ffi::c_char,
            file,
            line,
            func,
            &raw mut buf as *mut ::core::ffi::c_char,
        );
    };
}
