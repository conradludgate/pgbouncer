#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:19"]
pub mod _types_h {
    #[c2rust::src_loc = "36:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "61:1"]
    pub type __darwin_ct_rune_t = ::core::ffi::c_int;
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
    #[c2rust::src_loc = "103:1"]
    pub type __darwin_wchar_t = ::libc::wchar_t;
    #[c2rust::src_loc = "108:1"]
    pub type __darwin_rune_t = __darwin_wchar_t;
    #[c2rust::src_loc = "119:1"]
    pub type __darwin_time_t = ::core::ffi::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:19"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:19"]
pub mod sys__types_h {
    #[c2rust::src_loc = "72:1"]
    pub type __darwin_gid_t = __uint32_t;
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_uid_t = __uint32_t;
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    use super::_types_h::__uint32_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_gid_t.h:19"]
pub mod _gid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type gid_t = __darwin_gid_t;
    use super::sys__types_h::__darwin_gid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uid_t.h:19"]
pub mod _uid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uid_t = __darwin_uid_t;
    use super::sys__types_h::__darwin_uid_t;
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
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/cfparser.h:19"]
pub mod cfparser_h {
    #[c2rust::src_loc = "31:1"]
    pub type cf_handler_f = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            bool,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> bool,
    >;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "66:1"]
    pub struct CfValue {
        pub value_p: *mut ::core::ffi::c_void,
        pub extra: *const ::core::ffi::c_void,
        pub key_name: *const ::core::ffi::c_char,
        pub buf: *mut ::core::ffi::c_char,
        pub buflen: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "80:1"]
    pub struct CfOps {
        pub setter: Option<unsafe extern "C" fn(*mut CfValue, *const ::core::ffi::c_char) -> bool>,
        pub getter: Option<unsafe extern "C" fn(*mut CfValue) -> *const ::core::ffi::c_char>,
        pub op_extra: *const ::core::ffi::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "89:1"]
    pub struct CfKey {
        pub key_name: *const ::core::ffi::c_char,
        pub op: CfOps,
        pub flags: ::core::ffi::c_int,
        pub key_ofs: uintptr_t,
        pub def_value: *const ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "105:1"]
    pub struct CfSect {
        pub sect_name: *const ::core::ffi::c_char,
        pub key_list: *const CfKey,
        pub base_lookup: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub set_key: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> bool,
        >,
        pub get_key: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                *mut ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> *const ::core::ffi::c_char,
        >,
        pub section_start: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char) -> bool,
        >,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "128:1"]
    pub struct CfContext {
        pub sect_list: *const CfSect,
        pub base: *mut ::core::ffi::c_void,
        pub loaded: bool,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "199:1"]
    pub struct CfLookup {
        pub name: *const ::core::ffi::c_char,
        pub value: ::core::ffi::c_int,
    }
    #[c2rust::src_loc = "53:9"]
    pub const CF_VAL_REL: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "56:9"]
    pub const CF_NO_RELOAD: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "59:9"]
    pub const CF_READONLY: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    use super::_uintptr_t_h::uintptr_t;
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
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/runetype.h:25"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/pwd.h:22"]
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
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/time.h:28"]
pub mod time_h {
    #[c2rust::src_loc = "40:1"]
    pub type usec_t = uint64_t;
    #[c2rust::src_loc = "43:9"]
    pub const USEC: usec_t = 1000000 as ::core::ffi::c_int as usec_t;
    use super::_uint64_t_h::uint64_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:19"]
pub mod _stdio_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "435:1"]
        pub fn snprintf(
            __str: *mut ::core::ffi::c_char,
            __size: size_t,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:19"]
pub mod _malloc_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "54:1"]
        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "56:1"]
        pub fn free(_: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/unistd.h:19"]
pub mod unistd_h {
    use super::_uid_t_h::uid_t;
    extern "C" {
        #[c2rust::src_loc = "466:1"]
        pub fn getuid() -> uid_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:19"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/ctype.h:25"]
pub mod ctype_h {
    #[inline]
    #[c2rust::src_loc = "43:1"]
    pub unsafe extern "C" fn usual_isblank(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        return (c == ' ' as i32 || c == '\t' as i32) as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "55:1"]
    pub unsafe extern "C" fn safe_isalnum(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        return isalnum(c as ::core::ffi::c_uchar as ::core::ffi::c_int);
    }
    #[inline]
    #[c2rust::src_loc = "70:1"]
    pub unsafe extern "C" fn safe_isblank(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        return usual_isblank(c as ::core::ffi::c_uchar as ::core::ffi::c_int);
    }
    #[inline]
    #[c2rust::src_loc = "105:1"]
    pub unsafe extern "C" fn safe_isspace(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        return isspace(c as ::core::ffi::c_uchar as ::core::ffi::c_int);
    }
    use super::_ctype_h::{isalnum, isspace};
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_ctype.h:25"]
pub mod _ctype_h {
    #[c2rust::src_loc = "76:9"]
    pub const _CTYPE_A: ::core::ffi::c_long = 0x100 as ::core::ffi::c_long;
    #[c2rust::src_loc = "78:9"]
    pub const _CTYPE_D: ::core::ffi::c_long = 0x400 as ::core::ffi::c_long;
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
    #[no_mangle]
    #[inline]
    #[linkage = "external"]
    #[c2rust::src_loc = "216:1"]
    pub unsafe extern "C" fn isalnum(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        return __istype(
            _c as __darwin_ct_rune_t,
            (_CTYPE_A | _CTYPE_D) as ::core::ffi::c_ulong,
        );
    }
    #[inline]
    #[c2rust::src_loc = "271:1"]
    pub unsafe extern "C" fn isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        return __istype(_c as __darwin_ct_rune_t, _CTYPE_S as ::core::ffi::c_ulong);
    }
    use super::_types_h::{__darwin_ct_rune_t, __uint32_t};
    use super::runetype_h::_DefaultRuneLocale;
    extern "C" {
        #[c2rust::src_loc = "153:1"]
        pub fn __maskrune(_: __darwin_ct_rune_t, _: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:29"]
pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn memchr(
            __s: *const ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "77:1"]
        pub fn memcpy(
            __dst: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "83:1"]
        pub fn memset(
            __b: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __len: size_t,
        ) -> *mut ::core::ffi::c_void;
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
        #[c2rust::src_loc = "95:1"]
        pub fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "96:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
        #[c2rust::src_loc = "101:1"]
        pub fn strncmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
            __n: size_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "141:1"]
        pub fn strdup(__s1: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/fileutil.h:26"]
pub mod fileutil_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "40:1"]
        pub fn load_file(
            fn_0: *const ::core::ffi::c_char,
            len_p: *mut size_t,
        ) -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdlib.h:19"]
pub mod _stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "162:1"]
        pub fn getenv(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "186:1"]
        pub fn strtol(
            __str: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
            __base: ::core::ffi::c_int,
        ) -> ::core::ffi::c_long;
        #[c2rust::src_loc = "193:1"]
        pub fn strtoul(
            __str: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
            __base: ::core::ffi::c_int,
        ) -> ::core::ffi::c_ulong;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/errno.h:19"]
pub mod errno_h {
    #[c2rust::src_loc = "112:9"]
    pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
    extern "C" {
        #[c2rust::src_loc = "80:1"]
        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/llvm/21.1.8/lib/clang/21/include/stdbool.h:19"]
pub mod stdbool_h {
    #[c2rust::src_loc = "25:9"]
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_strings.h:29"]
pub mod _strings_h {
    extern "C" {
        #[c2rust::src_loc = "81:1"]
        pub fn strcasecmp(
            _: *const ::core::ffi::c_char,
            _: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/string.h:29"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "171:1"]
        pub fn strtod_dot(
            s: *const ::core::ffi::c_char,
            tokend: *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_double;
    }
}
pub use self::_ctype_h::{
    __istype, __maskrune, isalnum, isascii, isspace, _CTYPE_A, _CTYPE_D, _CTYPE_S,
};
pub use self::_gid_t_h::gid_t;
use self::_malloc_h::{free, malloc};
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
use self::_stdio_h::snprintf;
use self::_stdlib_h::{getenv, strtol, strtoul};
use self::_string_h::{memchr, memcpy, memset, strchr, strcmp, strdup, strerror, strlen, strncmp};
use self::_strings_h::strcasecmp;
pub use self::_types_h::{
    __darwin_ct_rune_t, __darwin_rune_t, __darwin_size_t, __darwin_time_t, __darwin_wchar_t,
    __uint32_t,
};
pub use self::_uid_t_h::uid_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::cfparser_h::{
    cf_handler_f, CfContext, CfKey, CfLookup, CfOps, CfSect, CfValue, CF_NO_RELOAD, CF_READONLY,
    CF_VAL_REL,
};
pub use self::ctype_h::{safe_isalnum, safe_isblank, safe_isspace, usual_isblank};
pub use self::errno_h::{__error, EINVAL};
use self::fileutil_h::load_file;
pub use self::logging_h::{
    cf_verbose, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE, LG_STATS,
    LG_WARNING,
};
pub use self::pwd_h::{getpwnam, getpwuid, passwd};
pub use self::runetype_h::{
    _DefaultRuneLocale, _RuneCharClass, _RuneEntry, _RuneLocale, _RuneRange,
};
pub use self::stdbool_h::{false_0, true_0};
use self::string_h::strtod_dot;
pub use self::sys__types_h::{__darwin_gid_t, __darwin_uid_t, __DARWIN_NULL};
pub use self::time_h::{usec_t, USEC};
use self::unistd_h::getuid;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "342:1"]
pub struct LoaderCtx {
    pub cf: *const CfContext,
    pub cur_sect: *mut ::core::ffi::c_char,
    pub top_base: *mut ::core::ffi::c_void,
    pub got_main_sect: bool,
}
#[c2rust::src_loc = "31:9"]
pub const MAX_INCLUDE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
#[c2rust::src_loc = "37:1"]
unsafe extern "C" fn count_lines(
    mut s: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut lineno = 1 as ::core::ffi::c_int;
    while s < end {
        if *s as ::core::ffi::c_int == '\n' as i32 {
            lineno += 1;
        }
        s = s.offset(1);
    }
    return lineno;
}
#[c2rust::src_loc = "47:1"]
unsafe extern "C" fn parse_ini_file_internal(
    mut fn_0: *const ::core::ffi::c_char,
    mut user_handler: cf_handler_f,
    mut arg: *mut ::core::ffi::c_void,
    mut inclevel: ::core::ffi::c_int,
) -> bool {
    let mut current_block: u64;
    let mut buf = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut klen: ::core::ffi::c_int = 0;
    let mut vlen: ::core::ffi::c_int = 0;
    let mut o1: ::core::ffi::c_char = 0;
    let mut o2: ::core::ffi::c_char = 0;
    let mut ok: bool = false;
    buf = load_file(fn_0, ::core::ptr::null_mut::<size_t>()) as *mut ::core::ffi::c_char;
    if buf.is_null() {
        let mut _log_ctx = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx,
            b"could not load file \"%s\": %s\0" as *const u8 as *const ::core::ffi::c_char,
            fn_0,
            strerror(*__error()),
        );
        return false_0 != 0;
    }
    p = buf;
    loop {
        if !(*p != 0) {
            current_block = 5706507068631705000;
            break;
        }
        while *p as ::core::ffi::c_int != 0 && safe_isspace(*p as ::core::ffi::c_int) != 0 {
            p = p.offset(1);
        }
        if strncmp(
            p,
            b"%include\0" as *const u8 as *const ::core::ffi::c_char,
            8 as size_t,
        ) == 0 as ::core::ffi::c_int
            && *p.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            && safe_isblank(*p.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int) != 0
        {
            if inclevel >= MAX_INCLUDE {
                let mut _log_ctx_0 = NULL;
                log_generic(
                    LG_ERROR,
                    _log_ctx_0,
                    b"include nesting level too deep (%s:%d), stopping loading\0" as *const u8
                        as *const ::core::ffi::c_char,
                    fn_0,
                    count_lines(buf, p),
                );
                current_block = 2366758333218785846;
                break;
            } else {
                p = p.offset(8 as ::core::ffi::c_int as isize);
                while *p as ::core::ffi::c_int != 0 && safe_isblank(*p as ::core::ffi::c_int) != 0 {
                    p = p.offset(1);
                }
                val = p;
                while *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int != '\n' as i32 {
                    p = p.offset(1);
                }
                vlen = p.offset_from(val) as ::core::ffi::c_long as ::core::ffi::c_int;
                while vlen > 0 as ::core::ffi::c_int
                    && safe_isspace(*val.offset((vlen - 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int)
                        != 0
                {
                    vlen -= 1;
                }
                o1 = *val.offset(vlen as isize);
                *val.offset(vlen as isize) = 0 as ::core::ffi::c_char;
                let mut _log_ctx_1 = NULL;
                if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0
                {
                    log_generic(
                        LG_DEBUG,
                        _log_ctx_1,
                        b"processing include: %s\0" as *const u8 as *const ::core::ffi::c_char,
                        val,
                    );
                }
                ok = parse_ini_file_internal(
                    val,
                    user_handler,
                    arg,
                    inclevel + 1 as ::core::ffi::c_int,
                );
                *val.offset(vlen as isize) = o1;
                if !ok {
                    let mut _log_ctx_2 = NULL;
                    log_generic(
                        LG_ERROR,
                        _log_ctx_2,
                        b"error processing include file in configuration (%s:%d), stopping loading\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        fn_0,
                        count_lines(buf, p),
                    );
                    current_block = 2366758333218785846;
                    break;
                } else {
                    let mut _log_ctx_3 = NULL;
                    if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        as ::core::ffi::c_long
                        != 0
                    {
                        log_generic(
                            LG_DEBUG,
                            _log_ctx_3,
                            b"returned to processing file %s\0" as *const u8
                                as *const ::core::ffi::c_char,
                            fn_0,
                        );
                    }
                }
            }
        } else if *p as ::core::ffi::c_int == '#' as i32 || *p as ::core::ffi::c_int == ';' as i32 {
            while *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int != '\n' as i32 {
                p = p.offset(1);
            }
        } else if *p as ::core::ffi::c_int == '[' as i32 {
            p = p.offset(1);
            key = p;
            while *p as ::core::ffi::c_int != 0
                && *p as ::core::ffi::c_int != ']' as i32
                && *p as ::core::ffi::c_int != '\n' as i32
            {
                p = p.offset(1);
            }
            if *p as ::core::ffi::c_int != ']' as i32 {
                current_block = 1271458265109284100;
                break;
            }
            o1 = *p;
            *p = 0 as ::core::ffi::c_char;
            let mut _log_ctx_4 = NULL;
            if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_DEBUG,
                    _log_ctx_4,
                    b"parse_ini_file: [%s]\0" as *const u8 as *const ::core::ffi::c_char,
                    key,
                );
            }
            ok = user_handler.expect("non-null function pointer")(
                arg,
                true_0 != 0,
                key,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            if !ok {
                let mut _log_ctx_5 = NULL;
                log_generic(
                    LG_ERROR,
                    _log_ctx_5,
                    b"invalid section \"%s\" in configuration (%s:%d)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    key,
                    fn_0,
                    count_lines(buf, p),
                );
                current_block = 2366758333218785846;
                break;
            } else {
                let fresh0 = p;
                p = p.offset(1);
                *fresh0 = o1;
            }
        } else {
            if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                current_block = 5706507068631705000;
                break;
            }
            if *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int == '\'' as i32 {
                p = p.offset(1);
                key = p;
                while *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int != '\'' as i32 {
                    p = p.offset(1);
                }
                if *p as ::core::ffi::c_int != '\'' as i32 {
                    current_block = 1271458265109284100;
                    break;
                }
                klen = p.offset_from(key) as ::core::ffi::c_long as ::core::ffi::c_int;
                if klen <= 0 as ::core::ffi::c_int {
                    current_block = 1271458265109284100;
                    break;
                }
                p = p.offset(1);
            } else {
                key = p;
                while *p as ::core::ffi::c_int != 0
                    && (safe_isalnum(*p as ::core::ffi::c_int) != 0
                        || !strchr(
                            b"_.-*\0" as *const u8 as *const ::core::ffi::c_char,
                            *p as ::core::ffi::c_int,
                        )
                        .is_null())
                {
                    p = p.offset(1);
                }
                klen = p.offset_from(key) as ::core::ffi::c_long as ::core::ffi::c_int;
            }
            while *p as ::core::ffi::c_int != 0
                && (*p as ::core::ffi::c_int == ' ' as i32
                    || *p as ::core::ffi::c_int == '\t' as i32)
            {
                p = p.offset(1);
            }
            if *p as ::core::ffi::c_int != '=' as i32 {
                current_block = 1271458265109284100;
                break;
            }
            p = p.offset(1);
            while *p as ::core::ffi::c_int != 0
                && (*p as ::core::ffi::c_int == ' ' as i32
                    || *p as ::core::ffi::c_int == '\t' as i32)
            {
                p = p.offset(1);
            }
            val = p;
            while *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int != '\n' as i32 {
                p = p.offset(1);
            }
            vlen = p.offset_from(val) as ::core::ffi::c_long as ::core::ffi::c_int;
            while vlen > 0 as ::core::ffi::c_int
                && safe_isspace(
                    *val.offset((vlen - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                ) != 0
            {
                vlen -= 1;
            }
            while *p as ::core::ffi::c_int != 0 && safe_isspace(*p as ::core::ffi::c_int) != 0 {
                p = p.offset(1);
            }
            o1 = *key.offset(klen as isize);
            o2 = *val.offset(vlen as isize);
            *key.offset(klen as isize) = 0 as ::core::ffi::c_char;
            *val.offset(vlen as isize) = 0 as ::core::ffi::c_char;
            let mut _log_ctx_6 = NULL;
            if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_DEBUG,
                    _log_ctx_6,
                    b"parse_ini_file: '%s' = '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                    key,
                    val,
                );
            }
            ok = user_handler.expect("non-null function pointer")(arg, false_0 != 0, key, val);
            let mut _log_ctx_7 = NULL;
            if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                log_generic(
                    LG_DEBUG,
                    _log_ctx_7,
                    b"parse_ini_file: '%s' = '%s' ok:%d\0" as *const u8
                        as *const ::core::ffi::c_char,
                    key,
                    val,
                    ok as ::core::ffi::c_int,
                );
            }
            if !ok {
                let mut _log_ctx_8 = NULL;
                log_generic(
                    LG_ERROR,
                    _log_ctx_8,
                    b"invalid value \"%s\" for parameter %s in configuration (%s:%d)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    val,
                    key,
                    fn_0,
                    count_lines(buf, p),
                );
            }
            *key.offset(klen as isize) = o1;
            *val.offset(vlen as isize) = o2;
            if !ok {
                current_block = 2366758333218785846;
                break;
            }
        }
    }
    match current_block {
        5706507068631705000 => {
            free(buf as *mut ::core::ffi::c_void);
            return true_0 != 0;
        }
        1271458265109284100 => {
            let mut _log_ctx_9 = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx_9,
                b"syntax error in configuration (%s:%d), stopping loading\0" as *const u8
                    as *const ::core::ffi::c_char,
                fn_0,
                count_lines(buf, p),
            );
        }
        _ => {}
    }
    free(buf as *mut ::core::ffi::c_void);
    return false_0 != 0;
}
#[no_mangle]
#[c2rust::src_loc = "203:1"]
pub unsafe extern "C" fn parse_ini_file(
    mut fn_0: *const ::core::ffi::c_char,
    mut user_handler: cf_handler_f,
    mut arg: *mut ::core::ffi::c_void,
) -> bool {
    return parse_ini_file_internal(fn_0, user_handler, arg, 0 as ::core::ffi::c_int);
}
#[c2rust::src_loc = "212:1"]
unsafe extern "C" fn get_dest(
    mut base: *mut ::core::ffi::c_void,
    mut k: *const CfKey,
) -> *mut ::core::ffi::c_void {
    let mut dst = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*k).flags & CF_VAL_REL != 0 {
        if base.is_null() {
            return NULL;
        }
        dst = (base as *mut ::core::ffi::c_char).offset((*k).key_ofs as isize);
    } else {
        dst = (*k).key_ofs as *mut ::core::ffi::c_char;
    }
    return dst as *mut ::core::ffi::c_void;
}
#[c2rust::src_loc = "226:1"]
unsafe extern "C" fn find_sect(
    mut cf: *const CfContext,
    mut name: *const ::core::ffi::c_char,
) -> *const CfSect {
    let mut s = ::core::ptr::null::<CfSect>();
    s = (*cf).sect_list;
    while !(*s).sect_name.is_null() {
        if strcmp((*s).sect_name, name) == 0 as ::core::ffi::c_int {
            return s;
        }
        if strcmp(
            (*s).sect_name,
            b"*\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return s;
        }
        s = s.offset(1);
    }
    return ::core::ptr::null::<CfSect>();
}
#[c2rust::src_loc = "238:1"]
unsafe extern "C" fn find_key(
    mut s: *const CfSect,
    mut key: *const ::core::ffi::c_char,
) -> *const CfKey {
    let mut k = ::core::ptr::null::<CfKey>();
    k = (*s).key_list;
    while !(*k).key_name.is_null() {
        if strcmp((*k).key_name, key) == 0 as ::core::ffi::c_int {
            return k;
        }
        k = k.offset(1);
    }
    return ::core::ptr::null::<CfKey>();
}
#[no_mangle]
#[c2rust::src_loc = "248:1"]
pub unsafe extern "C" fn cf_get(
    mut cf: *const CfContext,
    mut sect: *const ::core::ffi::c_char,
    mut key: *const ::core::ffi::c_char,
    mut buf: *mut ::core::ffi::c_char,
    mut buflen: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut s = ::core::ptr::null::<CfSect>();
    let mut k = ::core::ptr::null::<CfKey>();
    let mut base = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut p = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut cv = CfValue {
        value_p: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        extra: ::core::ptr::null::<::core::ffi::c_void>(),
        key_name: ::core::ptr::null::<::core::ffi::c_char>(),
        buf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        buflen: 0,
    };
    s = find_sect(cf, sect);
    if s.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    base = (*cf).base;
    if (*s).base_lookup.is_some() {
        base = (*s).base_lookup.expect("non-null function pointer")(base, sect);
    }
    if (*s).set_key.is_some() {
        if (*s).get_key.is_none() {
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        return (*s).get_key.expect("non-null function pointer")(base, key, buf, buflen);
    }
    k = find_key(s, key);
    if k.is_null() || (*k).op.getter.is_none() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    p = get_dest(base, k);
    if p.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    cv.key_name = (*k).key_name;
    cv.extra = (*k).op.op_extra;
    cv.value_p = p;
    cv.buf = buf;
    cv.buflen = buflen;
    return (*k).op.getter.expect("non-null function pointer")(&raw mut cv);
}
#[no_mangle]
#[c2rust::src_loc = "288:1"]
pub unsafe extern "C" fn cf_set(
    mut cf: *const CfContext,
    mut sect: *const ::core::ffi::c_char,
    mut key: *const ::core::ffi::c_char,
    mut val: *const ::core::ffi::c_char,
) -> bool {
    let mut s = ::core::ptr::null::<CfSect>();
    let mut k = ::core::ptr::null::<CfKey>();
    let mut base = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut p = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut cv = CfValue {
        value_p: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        extra: ::core::ptr::null::<::core::ffi::c_void>(),
        key_name: ::core::ptr::null::<::core::ffi::c_char>(),
        buf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        buflen: 0,
    };
    s = find_sect(cf, sect);
    if s.is_null() {
        let mut _log_ctx = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx,
            b"unknown section: %s\0" as *const u8 as *const ::core::ffi::c_char,
            sect,
        );
        return false_0 != 0;
    }
    base = (*cf).base;
    if (*s).base_lookup.is_some() {
        base = (*s).base_lookup.expect("non-null function pointer")(base, sect);
    }
    if (*s).set_key.is_some() {
        return (*s).set_key.expect("non-null function pointer")(base, key, val);
    }
    k = find_key(s, key);
    if k.is_null() {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx_0,
            b"unknown parameter: %s/%s\0" as *const u8 as *const ::core::ffi::c_char,
            sect,
            key,
        );
        return false_0 != 0;
    }
    if (*k).op.setter.is_none() || (*k).flags & CF_READONLY != 0 {
        return true_0 != 0;
    }
    if (*k).flags & CF_NO_RELOAD != 0 && (*cf).loaded as ::core::ffi::c_int != 0 {
        return true_0 != 0;
    }
    p = get_dest(base, k);
    if p.is_null() {
        let mut _log_ctx_1 = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx_1,
            b"bug - no base for relative key: %s/%s\0" as *const u8 as *const ::core::ffi::c_char,
            sect,
            key,
        );
        return false_0 != 0;
    }
    cv.key_name = (*k).key_name;
    cv.extra = (*k).op.op_extra;
    cv.value_p = p;
    cv.buf = ::core::ptr::null_mut::<::core::ffi::c_char>();
    cv.buflen = 0 as ::core::ffi::c_int;
    return (*k).op.setter.expect("non-null function pointer")(&raw mut cv, val);
}
#[c2rust::src_loc = "349:1"]
unsafe extern "C" fn fill_defaults(mut ctx: *mut LoaderCtx) -> bool {
    let mut current_block: u64;
    let mut k = ::core::ptr::null::<CfKey>();
    let mut s = ::core::ptr::null::<CfSect>();
    s = find_sect((*ctx).cf, (*ctx).cur_sect);
    if !s.is_null() {
        if s == (*(*ctx).cf).sect_list {
            (*ctx).got_main_sect = true_0 != 0;
        }
        if (*s).section_start.is_some() {
            if !(*s).section_start.expect("non-null function pointer")(
                (*ctx).top_base,
                (*ctx).cur_sect,
            ) {
                return false_0 != 0;
            }
        }
        if (*s).set_key.is_some() {
            return true_0 != 0;
        }
        k = (*s).key_list;
        loop {
            if (*k).key_name.is_null() {
                current_block = 3512920355445576850;
                break;
            }
            if !((*k).def_value.is_null() || (*k).flags & CF_READONLY != 0) {
                if !((*k).flags & CF_NO_RELOAD != 0
                    && (*(*ctx).cf).loaded as ::core::ffi::c_int != 0)
                {
                    if !cf_set((*ctx).cf, (*ctx).cur_sect, (*k).key_name, (*k).def_value) {
                        current_block = 15255303563108543441;
                        break;
                    }
                }
            }
            k = k.offset(1);
        }
        match current_block {
            15255303563108543441 => {}
            _ => return true_0 != 0,
        }
    }
    let mut _log_ctx = NULL;
    log_generic(
        LG_ERROR,
        _log_ctx,
        b"fill_defaults fail\0" as *const u8 as *const ::core::ffi::c_char,
    );
    return false_0 != 0;
}
#[c2rust::src_loc = "383:1"]
unsafe extern "C" fn load_handler(
    mut arg: *mut ::core::ffi::c_void,
    mut is_sect: bool,
    mut key: *const ::core::ffi::c_char,
    mut val: *const ::core::ffi::c_char,
) -> bool {
    let mut ctx = arg as *mut LoaderCtx;
    if is_sect {
        free((*ctx).cur_sect as *mut ::core::ffi::c_void);
        (*ctx).cur_sect = strdup(key);
        if (*ctx).cur_sect.is_null() {
            return false_0 != 0;
        }
        return fill_defaults(ctx);
    } else if (*ctx).cur_sect.is_null() {
        let mut _log_ctx = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx,
            b"load_init_file: value without section: %s\0" as *const u8
                as *const ::core::ffi::c_char,
            key,
        );
        return false_0 != 0;
    } else {
        return cf_set((*ctx).cf, (*ctx).cur_sect, key, val);
    };
}
#[no_mangle]
#[c2rust::src_loc = "401:1"]
pub unsafe extern "C" fn cf_load_file(
    mut cf: *const CfContext,
    mut fn_0: *const ::core::ffi::c_char,
) -> bool {
    let mut ctx = LoaderCtx {
        cf: ::core::ptr::null::<CfContext>(),
        cur_sect: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        top_base: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        got_main_sect: false,
    };
    let mut ok: bool = false;
    memset(
        &raw mut ctx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<LoaderCtx>() as size_t,
    );
    ctx.cf = cf;
    ok = parse_ini_file(
        fn_0,
        Some(
            load_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    bool,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> bool,
        ),
        &raw mut ctx as *mut ::core::ffi::c_void,
    );
    free(ctx.cur_sect as *mut ::core::ffi::c_void);
    if ok as ::core::ffi::c_int != 0 && !ctx.got_main_sect {
        let mut _log_ctx = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx,
            b"load_init_file: main section missing from config file\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    return ok;
}
#[no_mangle]
#[c2rust::src_loc = "421:1"]
pub unsafe extern "C" fn cf_set_int(
    mut cv: *mut CfValue,
    mut value: *const ::core::ffi::c_char,
) -> bool {
    let mut ptr = (*cv).value_p as *mut ::core::ffi::c_int;
    let mut end = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val: ::core::ffi::c_long = 0;
    *__error() = 0 as ::core::ffi::c_int;
    val = strtol(value, &raw mut end, 0 as ::core::ffi::c_int);
    if end == value as *mut ::core::ffi::c_char
        || *end as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        if *__error() == 0 {
            *__error() = EINVAL;
        }
        return false_0 != 0;
    }
    *ptr = val as ::core::ffi::c_int;
    return true_0 != 0;
}
#[no_mangle]
#[c2rust::src_loc = "439:1"]
pub unsafe extern "C" fn cf_set_uint(
    mut cv: *mut CfValue,
    mut value: *const ::core::ffi::c_char,
) -> bool {
    let mut ptr = (*cv).value_p as *mut ::core::ffi::c_uint;
    let mut end = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val: ::core::ffi::c_ulong = 0;
    *__error() = 0 as ::core::ffi::c_int;
    val = strtoul(value, &raw mut end, 0 as ::core::ffi::c_int);
    if end == value as *mut ::core::ffi::c_char
        || *end as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        if *__error() == 0 {
            *__error() = EINVAL;
        }
        return false_0 != 0;
    }
    *ptr = val as ::core::ffi::c_uint;
    return true_0 != 0;
}
#[no_mangle]
#[c2rust::src_loc = "457:1"]
pub unsafe extern "C" fn cf_set_str(
    mut cv: *mut CfValue,
    mut value: *const ::core::ffi::c_char,
) -> bool {
    let mut dst_p = (*cv).value_p as *mut *mut ::core::ffi::c_char;
    let mut tmp = strdup(value);
    if tmp.is_null() {
        let mut _log_ctx = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx,
            b"cf_set_str: no mem\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    free(*dst_p as *mut ::core::ffi::c_void);
    *dst_p = tmp;
    return true_0 != 0;
}
#[no_mangle]
#[c2rust::src_loc = "471:1"]
pub unsafe extern "C" fn cf_set_filename(
    mut cv: *mut CfValue,
    mut value: *const ::core::ffi::c_char,
) -> bool {
    let mut current_block: u64;
    let mut dst_p = (*cv).value_p as *mut *mut ::core::ffi::c_char;
    let mut tmp = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut home = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut v_len: ::core::ffi::c_int = 0;
    let mut usr_len: ::core::ffi::c_int = 0;
    let mut home_len: ::core::ffi::c_int = 0;
    let mut pw = ::core::ptr::null_mut::<passwd>();
    if *value.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '~' as i32 {
        return cf_set_str(cv, value);
    }
    v_len = strlen(value) as ::core::ffi::c_int;
    p = memchr(
        value as *const ::core::ffi::c_void,
        '/' as i32,
        v_len as size_t,
    ) as *mut ::core::ffi::c_char;
    if p.is_null() {
        usr_len = v_len - 1 as ::core::ffi::c_int;
    } else {
        usr_len = (p.offset_from(value) as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
            as ::core::ffi::c_int;
    }
    if usr_len != 0 {
        p = malloc((usr_len + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
        if p.is_null() {
            return false_0 != 0;
        }
        memcpy(
            p as *mut ::core::ffi::c_void,
            value.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            usr_len as size_t,
        );
        *p.offset(usr_len as isize) = 0 as ::core::ffi::c_char;
        pw = getpwnam(p);
        free(p as *mut ::core::ffi::c_void);
        if pw.is_null() {
            current_block = 2191271295494071663;
        } else {
            home = (*pw).pw_dir;
            current_block = 5689001924483802034;
        }
    } else {
        home = getenv(b"HOME\0" as *const u8 as *const ::core::ffi::c_char);
        if home.is_null() {
            pw = getpwuid(getuid());
            if pw.is_null() {
                current_block = 2191271295494071663;
            } else {
                home = (*pw).pw_dir;
                current_block = 5689001924483802034;
            }
        } else {
            current_block = 5689001924483802034;
        }
    }
    match current_block {
        5689001924483802034 => {
            if !home.is_null() {
                home_len = strlen(home) as ::core::ffi::c_int;
                tmp = malloc((v_len - usr_len + home_len) as size_t) as *mut ::core::ffi::c_char;
                if tmp.is_null() {
                    return false_0 != 0;
                }
                memcpy(
                    tmp as *mut ::core::ffi::c_void,
                    home as *const ::core::ffi::c_void,
                    home_len as size_t,
                );
                memcpy(
                    tmp.offset(home_len as isize) as *mut ::core::ffi::c_void,
                    value
                        .offset(usr_len as isize)
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_void,
                    (v_len - usr_len - 1 as ::core::ffi::c_int) as size_t,
                );
                *tmp.offset((v_len - 1 as ::core::ffi::c_int - usr_len + home_len) as isize) =
                    0 as ::core::ffi::c_char;
                let mut _log_ctx = NULL;
                if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0
                {
                    log_generic(
                        LG_DEBUG,
                        _log_ctx,
                        b"expanded '%s' -> '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                        value,
                        tmp,
                    );
                }
                free(*dst_p as *mut ::core::ffi::c_void);
                *dst_p = tmp;
                return true_0 != 0;
            }
        }
        _ => {}
    }
    let mut _log_ctx_0 = NULL;
    log_generic(
        LG_ERROR,
        _log_ctx_0,
        b"cannot to expand filename: %s\0" as *const u8 as *const ::core::ffi::c_char,
        value,
    );
    return false_0 != 0;
}
#[c2rust::src_loc = "531:1"]
unsafe extern "C" fn parse_time(mut value: *const ::core::ffi::c_char) -> ::core::ffi::c_double {
    let mut v: ::core::ffi::c_double = 0.;
    let mut endp = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *__error() = 0 as ::core::ffi::c_int;
    v = strtod_dot(value, &raw mut endp);
    if *__error() != 0 {
        return -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
    }
    if *endp as ::core::ffi::c_int != 0
        || endp == value as *mut ::core::ffi::c_char
        || v < 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        *__error() = EINVAL;
        return -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
    }
    return v;
}
#[no_mangle]
#[c2rust::src_loc = "547:1"]
pub unsafe extern "C" fn cf_set_time_usec(
    mut cv: *mut CfValue,
    mut value: *const ::core::ffi::c_char,
) -> bool {
    let mut ptr = (*cv).value_p as *mut usec_t;
    let mut v = parse_time(value);
    if v < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        return false_0 != 0;
    }
    *ptr = (USEC as ::core::ffi::c_double * v) as usec_t;
    return true_0 != 0;
}
#[no_mangle]
#[c2rust::src_loc = "557:1"]
pub unsafe extern "C" fn cf_set_time_double(
    mut cv: *mut CfValue,
    mut value: *const ::core::ffi::c_char,
) -> bool {
    let mut ptr = (*cv).value_p as *mut ::core::ffi::c_double;
    let mut v = parse_time(value);
    if v < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        return false_0 != 0;
    }
    *ptr = v;
    return true_0 != 0;
}
#[no_mangle]
#[c2rust::src_loc = "571:1"]
pub unsafe extern "C" fn cf_get_str(mut cv: *mut CfValue) -> *const ::core::ffi::c_char {
    let mut p = (*cv).value_p as *mut *mut ::core::ffi::c_char;
    return *p;
}
#[no_mangle]
#[c2rust::src_loc = "577:1"]
pub unsafe extern "C" fn cf_get_int(mut cv: *mut CfValue) -> *const ::core::ffi::c_char {
    let mut p = (*cv).value_p as *mut ::core::ffi::c_int;
    snprintf(
        (*cv).buf,
        (*cv).buflen as size_t,
        b"%d\0" as *const u8 as *const ::core::ffi::c_char,
        *p,
    );
    return (*cv).buf;
}
#[no_mangle]
#[c2rust::src_loc = "584:1"]
pub unsafe extern "C" fn cf_get_uint(mut cv: *mut CfValue) -> *const ::core::ffi::c_char {
    let mut p = (*cv).value_p as *mut ::core::ffi::c_uint;
    snprintf(
        (*cv).buf,
        (*cv).buflen as size_t,
        b"%u\0" as *const u8 as *const ::core::ffi::c_char,
        *p,
    );
    return (*cv).buf;
}
#[no_mangle]
#[c2rust::src_loc = "591:1"]
pub unsafe extern "C" fn cf_get_time_double(mut cv: *mut CfValue) -> *const ::core::ffi::c_char {
    let mut p = (*cv).value_p as *mut ::core::ffi::c_double;
    snprintf(
        (*cv).buf,
        (*cv).buflen as size_t,
        b"%g\0" as *const u8 as *const ::core::ffi::c_char,
        *p,
    );
    return (*cv).buf;
}
#[no_mangle]
#[c2rust::src_loc = "598:1"]
pub unsafe extern "C" fn cf_get_time_usec(mut cv: *mut CfValue) -> *const ::core::ffi::c_char {
    let mut tmp = *cv;
    let mut p = (*cv).value_p as *mut usec_t;
    let mut d = *p as ::core::ffi::c_double / USEC as ::core::ffi::c_double;
    tmp.value_p = &raw mut d as *mut ::core::ffi::c_void;
    return cf_get_time_double(&raw mut tmp);
}
#[no_mangle]
#[c2rust::src_loc = "611:1"]
pub unsafe extern "C" fn cf_get_lookup(mut cv: *mut CfValue) -> *const ::core::ffi::c_char {
    let mut p = (*cv).value_p as *mut ::core::ffi::c_int;
    let mut lk = (*cv).extra as *const CfLookup;
    while !(*lk).name.is_null() {
        if (*lk).value == *p {
            return (*lk).name;
        }
        lk = lk.offset(1);
    }
    return b"INVALID\0" as *const u8 as *const ::core::ffi::c_char;
}
#[no_mangle]
#[c2rust::src_loc = "622:1"]
pub unsafe extern "C" fn cf_set_lookup(
    mut cv: *mut CfValue,
    mut value: *const ::core::ffi::c_char,
) -> bool {
    let mut p = (*cv).value_p as *mut ::core::ffi::c_int;
    let mut lk = (*cv).extra as *const CfLookup;
    while !(*lk).name.is_null() {
        if strcasecmp((*lk).name, value) == 0 as ::core::ffi::c_int {
            *p = (*lk).value;
            return true_0 != 0;
        }
        lk = lk.offset(1);
    }
    return false_0 != 0;
}
