#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(extern_types, raw_ref_op, register_tool)]
#[macro_use]
extern crate c2rust_bitfields;
#[allow(unused_imports)]
use ::pgbouncer;
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:23"]
pub mod _types_h {
    #[c2rust::src_loc = "32:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "34:1"]
    pub type __uint16_t = u16;
    #[c2rust::src_loc = "35:1"]
    pub type __int32_t = i32;
    #[c2rust::src_loc = "36:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "37:1"]
    pub type __int64_t = i64;
    #[c2rust::src_loc = "38:1"]
    pub type __uint64_t = u64;
    #[c2rust::src_loc = "77:1"]
    pub type __darwin_ptrdiff_t = isize;
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
    #[c2rust::src_loc = "117:1"]
    pub type __darwin_socklen_t = __uint32_t;
    #[c2rust::src_loc = "118:1"]
    pub type __darwin_ssize_t = isize;
    #[c2rust::src_loc = "119:1"]
    pub type __darwin_time_t = ::core::ffi::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:23"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:23"]
pub mod sys__types_h {
    #[c2rust::src_loc = "83:1"]
    pub type __darwin_off_t = __int64_t;
    #[c2rust::src_loc = "84:1"]
    pub type __darwin_pid_t = __int32_t;
    #[c2rust::src_loc = "85:1"]
    pub type __darwin_sigset_t = __uint32_t;
    #[c2rust::src_loc = "86:1"]
    pub type __darwin_suseconds_t = __int32_t;
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_uid_t = __uint32_t;
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    use super::_types_h::{__int32_t, __int64_t, __uint32_t};
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_in_addr_t.h:23"]
pub mod _in_addr_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type in_addr_t = __uint32_t;
    use super::_types_h::__uint32_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_in_port_t.h:23"]
pub mod _in_port_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type in_port_t = __uint16_t;
    use super::_types_h::__uint16_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_pid_t.h:23"]
pub mod _pid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pid_t = __darwin_pid_t;
    use super::sys__types_h::__darwin_pid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uid_t.h:23"]
pub mod _uid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uid_t = __darwin_uid_t;
    use super::sys__types_h::__darwin_uid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:23"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ssize_t.h:23"]
pub mod _ssize_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_time_t.h:23"]
pub mod _time_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type time_t = __darwin_time_t;
    use super::_types_h::__darwin_time_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_sigset_t.h:23"]
pub mod _sigset_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type sigset_t = __darwin_sigset_t;
    use super::sys__types_h::__darwin_sigset_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ptrdiff_t.h:23"]
pub mod _ptrdiff_t_h {
    #[c2rust::src_loc = "51:1"]
    pub type ptrdiff_t = __darwin_ptrdiff_t;
    use super::_types_h::__darwin_ptrdiff_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:23"]
pub mod _uint8_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint8_t = u8;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint16_t.h:23"]
pub mod _uint16_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint16_t = u16;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:23"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = u32;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint64_t.h:23"]
pub mod _uint64_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint64_t = u64;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:23"]
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
    use super::_size_t_h::size_t;
    use super::sys__types_h::__darwin_off_t;
    extern "C" {
        #[c2rust::src_loc = "103:1"]
        pub type __sFILEX;
        #[c2rust::src_loc = "169:1"]
        pub static mut __stderrp: *mut FILE;
        #[c2rust::src_loc = "245:1"]
        pub fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "435:1"]
        pub fn snprintf(
            __str: *mut ::core::ffi::c_char,
            __size: size_t,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_timeval.h:23"]
pub mod _timeval_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "34:1"]
    pub struct timeval {
        pub tv_sec: __darwin_time_t,
        pub tv_usec: __darwin_suseconds_t,
    }
    use super::_types_h::__darwin_time_t;
    use super::sys__types_h::__darwin_suseconds_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/resource.h:23"]
pub mod resource_h {
    #[c2rust::src_loc = "89:1"]
    pub type rlim_t = __uint64_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "464:1"]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    #[c2rust::src_loc = "454:9"]
    pub const RLIMIT_NOFILE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    use super::_types_h::__uint64_t;
    extern "C" {
        #[c2rust::src_loc = "575:1"]
        pub fn getrlimit(_: ::core::ffi::c_int, _: *mut rlimit) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/tls/tls.h:23"]
pub mod tls_h {
    extern "C" {
        #[c2rust::src_loc = "66:1"]
        pub type tls;
        #[c2rust::src_loc = "72:1"]
        pub fn tls_backend_version() -> *const ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/cfparser.h:23"]
pub mod cfparser_h {
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
    #[c2rust::src_loc = "50:9"]
    pub const CF_VAL_ABS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "56:9"]
    pub const CF_NO_RELOAD: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "59:9"]
    pub const CF_READONLY: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    use super::_uintptr_t_h::uintptr_t;
    extern "C" {
        #[c2rust::src_loc = "143:1"]
        pub fn cf_set_str(cv: *mut CfValue, value: *const ::core::ffi::c_char) -> bool;
        #[c2rust::src_loc = "147:1"]
        pub fn cf_set_int(cv: *mut CfValue, value: *const ::core::ffi::c_char) -> bool;
        #[c2rust::src_loc = "149:1"]
        pub fn cf_set_uint(cv: *mut CfValue, value: *const ::core::ffi::c_char) -> bool;
        #[c2rust::src_loc = "151:1"]
        pub fn cf_set_time_usec(cv: *mut CfValue, value: *const ::core::ffi::c_char) -> bool;
        #[c2rust::src_loc = "155:1"]
        pub fn cf_set_lookup(cv: *mut CfValue, value: *const ::core::ffi::c_char) -> bool;
        #[c2rust::src_loc = "158:1"]
        pub fn cf_get_str(cv: *mut CfValue) -> *const ::core::ffi::c_char;
        #[c2rust::src_loc = "160:1"]
        pub fn cf_get_int(cv: *mut CfValue) -> *const ::core::ffi::c_char;
        #[c2rust::src_loc = "162:1"]
        pub fn cf_get_uint(cv: *mut CfValue) -> *const ::core::ffi::c_char;
        #[c2rust::src_loc = "164:1"]
        pub fn cf_get_time_usec(cv: *mut CfValue) -> *const ::core::ffi::c_char;
        #[c2rust::src_loc = "168:1"]
        pub fn cf_get_lookup(cv: *mut CfValue) -> *const ::core::ffi::c_char;
        #[c2rust::src_loc = "234:1"]
        pub fn cf_load_file(cf: *const CfContext, fn_0: *const ::core::ffi::c_char) -> bool;
        #[c2rust::src_loc = "239:1"]
        pub fn cf_get(
            cf: *const CfContext,
            sect: *const ::core::ffi::c_char,
            var: *const ::core::ffi::c_char,
            buf: *mut ::core::ffi::c_char,
            buflen: ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_char;
        #[c2rust::src_loc = "244:1"]
        pub fn cf_set(
            cf: *const CfContext,
            sect: *const ::core::ffi::c_char,
            var: *const ::core::ffi::c_char,
            val: *const ::core::ffi::c_char,
        ) -> bool;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/time.h:23"]
pub mod time_h {
    #[c2rust::src_loc = "40:1"]
    pub type usec_t = uint64_t;
    use super::_uint64_t_h::uint64_t;
    extern "C" {
        #[c2rust::src_loc = "56:1"]
        pub fn reset_time_cache();
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/list.h:23"]
pub mod list_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "35:1"]
    pub struct List {
        pub next: *mut List,
        pub prev: *mut List,
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/statlist.h:23"]
pub mod statlist_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:1"]
    pub struct StatList {
        pub head: List,
        pub cur_count: ::core::ffi::c_int,
    }
    #[inline]
    #[c2rust::src_loc = "88:1"]
    pub unsafe extern "C" fn statlist_count(mut list: *const StatList) -> ::core::ffi::c_int {
        return (*list).cur_count;
    }
    use super::list_h::List;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/aatree.h:23"]
pub mod aatree_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "41:1"]
    pub struct AATree {
        pub root: *mut AANode,
        pub count: ::core::ffi::c_int,
        pub node_cmp: aatree_cmp_f,
        pub release_cb: aatree_walker_f,
    }
    #[c2rust::src_loc = "36:1"]
    pub type aatree_walker_f =
        Option<unsafe extern "C" fn(*mut AANode, *mut ::core::ffi::c_void) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "57:1"]
    pub struct AANode {
        pub left: *mut AANode,
        pub right: *mut AANode,
        pub level: ::core::ffi::c_int,
    }
    #[c2rust::src_loc = "33:1"]
    pub type aatree_cmp_f =
        Option<unsafe extern "C" fn(uintptr_t, *mut AANode) -> ::core::ffi::c_int>;
    use super::_uintptr_t_h::uintptr_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_sa_family_t.h:23"]
pub mod _sa_family_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type sa_family_t = __uint8_t;
    use super::_types_h::__uint8_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_socklen_t.h:23"]
pub mod _socklen_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type socklen_t = __darwin_socklen_t;
    use super::_types_h::__darwin_socklen_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/socket.h:23"]
pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "414:1"]
    pub struct sockaddr {
        pub sa_len: __uint8_t,
        pub sa_family: sa_family_t,
        pub sa_data: [::core::ffi::c_char; 14],
    }
    #[c2rust::src_loc = "111:9"]
    pub const SOCK_STREAM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "361:9"]
    pub const AF_UNIX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    use super::_sa_family_t_h::sa_family_t;
    use super::_types_h::__uint8_t;
    extern "C" {
        #[c2rust::src_loc = "729:1"]
        pub fn socket(
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/un.h:23"]
pub mod un_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "76:1"]
    pub struct sockaddr_un {
        pub sun_len: ::core::ffi::c_uchar,
        pub sun_family: sa_family_t,
        pub sun_path: [::core::ffi::c_char; 104],
    }
    use super::_sa_family_t_h::sa_family_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet/in.h:23"]
pub mod in_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "301:1"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "374:1"]
    pub struct sockaddr_in {
        pub sin_len: __uint8_t,
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [::core::ffi::c_char; 8],
    }
    use super::_in_addr_t_h::in_addr_t;
    use super::_in_port_t_h::in_port_t;
    use super::_sa_family_t_h::sa_family_t;
    use super::_types_h::__uint8_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet6/in6.h:23"]
pub mod in6_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "152:9"]
    pub struct in6_addr {
        pub __u6_addr: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "153:2"]
    pub union C2RustUnnamed {
        pub __u6_addr8: [__uint8_t; 16],
        pub __u6_addr16: [__uint16_t; 8],
        pub __u6_addr32: [__uint32_t; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "170:1"]
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
#[c2rust::header_src = "/opt/homebrew/Cellar/libevent/2.1.12_1/include/event2/event.h:23"]
pub mod event_h {
    #[c2rust::src_loc = "1014:1"]
    pub type event_callback_fn = Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_short,
            *mut ::core::ffi::c_void,
        ) -> (),
    >;
    #[c2rust::src_loc = "796:9"]
    pub const EVLOOP_ONCE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "927:9"]
    pub const EV_SIGNAL: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    #[c2rust::src_loc = "934:9"]
    pub const EV_PERSIST: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    use super::_timeval_h::timeval;
    use super::event_struct_h::event;
    extern "C" {
        #[c2rust::src_loc = "217:1"]
        pub type event_base;
        #[c2rust::src_loc = "344:1"]
        pub fn event_base_new() -> *mut event_base;
        #[c2rust::src_loc = "382:1"]
        pub fn event_base_get_method(_: *const event_base) -> *const ::core::ffi::c_char;
        #[c2rust::src_loc = "692:1"]
        pub fn event_base_free(_: *mut event_base);
        #[c2rust::src_loc = "826:1"]
        pub fn event_base_loop(_: *mut event_base, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1132:1"]
        pub fn event_assign(
            _: *mut event,
            _: *mut event_base,
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_short,
            _: event_callback_fn,
            _: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1233:1"]
        pub fn event_add(ev: *mut event, timeout: *const timeval) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1417:1"]
        pub fn event_get_version() -> *const ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/libevent/2.1.12_1/include/event2/event_struct.h:23"]
pub mod event_struct_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "123:1"]
    pub struct event {
        pub ev_evcallback: event_callback,
        pub ev_timeout_pos: C2RustUnnamed_5,
        pub ev_fd: ::core::ffi::c_int,
        pub ev_base: *mut event_base,
        pub ev_: C2RustUnnamed_0,
        pub ev_events: ::core::ffi::c_short,
        pub ev_res: ::core::ffi::c_short,
        pub ev_timeout: timeval,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "135:2"]
    pub union C2RustUnnamed_0 {
        pub ev_io: C2RustUnnamed_3,
        pub ev_signal: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "143:3"]
    pub struct C2RustUnnamed_1 {
        pub ev_signal_next: C2RustUnnamed_2,
        pub ev_ncalls: ::core::ffi::c_short,
        pub ev_pncalls: *mut ::core::ffi::c_short,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "144:4"]
    pub struct C2RustUnnamed_2 {
        pub le_next: *mut event,
        pub le_prev: *mut *mut event,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "137:3"]
    pub struct C2RustUnnamed_3 {
        pub ev_io_next: C2RustUnnamed_4,
        pub ev_timeout: timeval,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "138:4"]
    pub struct C2RustUnnamed_4 {
        pub le_next: *mut event,
        pub le_prev: *mut *mut event,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "127:2"]
    pub union C2RustUnnamed_5 {
        pub ev_next_with_common_timeout: C2RustUnnamed_6,
        pub min_heap_idx: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "128:3"]
    pub struct C2RustUnnamed_6 {
        pub tqe_next: *mut event,
        pub tqe_prev: *mut *mut event,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "107:1"]
    pub struct event_callback {
        pub evcb_active_next: C2RustUnnamed_8,
        pub evcb_flags: ::core::ffi::c_short,
        pub evcb_pri: uint8_t,
        pub evcb_closure: uint8_t,
        pub evcb_cb_union: C2RustUnnamed_7,
        pub evcb_arg: *mut ::core::ffi::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:9"]
    pub union C2RustUnnamed_7 {
        pub evcb_callback: Option<
            unsafe extern "C" fn(
                ::core::ffi::c_int,
                ::core::ffi::c_short,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
        pub evcb_selfcb:
            Option<unsafe extern "C" fn(*mut event_callback, *mut ::core::ffi::c_void) -> ()>,
        pub evcb_evfinalize:
            Option<unsafe extern "C" fn(*mut event, *mut ::core::ffi::c_void) -> ()>,
        pub evcb_cbfinalize:
            Option<unsafe extern "C" fn(*mut event_callback, *mut ::core::ffi::c_void) -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "108:2"]
    pub struct C2RustUnnamed_8 {
        pub tqe_next: *mut event_callback,
        pub tqe_prev: *mut *mut event_callback,
    }
    use super::_timeval_h::timeval;
    use super::_uint8_t_h::uint8_t;
    use super::event_h::event_base;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/cryptohash.h:23"]
pub mod cryptohash_h {
    #[c2rust::src_loc = "19:9"]
    pub type pg_cryptohash_type = ::core::ffi::c_uint;
    #[c2rust::src_loc = "24:2"]
    pub const PG_SHA512: pg_cryptohash_type = 3;
    #[c2rust::src_loc = "23:2"]
    pub const PG_SHA384: pg_cryptohash_type = 2;
    #[c2rust::src_loc = "22:2"]
    pub const PG_SHA256: pg_cryptohash_type = 1;
    #[c2rust::src_loc = "21:2"]
    pub const PG_SHA224: pg_cryptohash_type = 0;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/uthash.h:23"]
pub mod uthash_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1072:9"]
    pub struct UT_hash_bucket {
        pub hh_head: *mut UT_hash_handle,
        pub count: ::core::ffi::c_uint,
        pub expand_mult: ::core::ffi::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1129:9"]
    pub struct UT_hash_handle {
        pub tbl: *mut UT_hash_table,
        pub prev: *mut ::core::ffi::c_void,
        pub next: *mut ::core::ffi::c_void,
        pub hh_prev: *mut UT_hash_handle,
        pub hh_next: *mut UT_hash_handle,
        pub key: *const ::core::ffi::c_void,
        pub keylen: ::core::ffi::c_uint,
        pub hashv: ::core::ffi::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1096:9"]
    pub struct UT_hash_table {
        pub buckets: *mut UT_hash_bucket,
        pub num_buckets: ::core::ffi::c_uint,
        pub log2_num_buckets: ::core::ffi::c_uint,
        pub num_items: ::core::ffi::c_uint,
        pub tail: *mut UT_hash_handle,
        pub hho: ptrdiff_t,
        pub ideal_chain_maxlen: ::core::ffi::c_uint,
        pub nonideal_items: ::core::ffi::c_uint,
        pub ineff_expands: ::core::ffi::c_uint,
        pub noexpand: ::core::ffi::c_uint,
        pub signature: uint32_t,
    }
    use super::_ptrdiff_t_h::ptrdiff_t;
    use super::_uint32_t_h::uint32_t;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/bouncer.h:23"]
pub mod bouncer_h {
    #[c2rust::src_loc = "70:1"]
    pub type SocketState = ::core::ffi::c_uint;
    #[c2rust::src_loc = "88:2"]
    pub const SV_TESTED: SocketState = 16;
    #[c2rust::src_loc = "87:2"]
    pub const SV_USED: SocketState = 15;
    #[c2rust::src_loc = "86:2"]
    pub const SV_ACTIVE_CANCEL: SocketState = 14;
    #[c2rust::src_loc = "85:2"]
    pub const SV_ACTIVE: SocketState = 13;
    #[c2rust::src_loc = "84:2"]
    pub const SV_IDLE: SocketState = 12;
    #[c2rust::src_loc = "83:2"]
    pub const SV_BEING_CANCELED: SocketState = 11;
    #[c2rust::src_loc = "82:2"]
    pub const SV_LOGIN: SocketState = 10;
    #[c2rust::src_loc = "81:2"]
    pub const SV_JUSTFREE: SocketState = 9;
    #[c2rust::src_loc = "80:2"]
    pub const SV_FREE: SocketState = 8;
    #[c2rust::src_loc = "78:2"]
    pub const CL_ACTIVE_CANCEL: SocketState = 7;
    #[c2rust::src_loc = "77:2"]
    pub const CL_WAITING_CANCEL: SocketState = 6;
    #[c2rust::src_loc = "76:2"]
    pub const CL_ACTIVE: SocketState = 5;
    #[c2rust::src_loc = "75:2"]
    pub const CL_WAITING_LOGIN: SocketState = 4;
    #[c2rust::src_loc = "74:2"]
    pub const CL_WAITING: SocketState = 3;
    #[c2rust::src_loc = "73:2"]
    pub const CL_LOGIN: SocketState = 2;
    #[c2rust::src_loc = "72:2"]
    pub const CL_JUSTFREE: SocketState = 1;
    #[c2rust::src_loc = "71:2"]
    pub const CL_FREE: SocketState = 0;
    #[c2rust::src_loc = "91:1"]
    pub type PauseMode = ::core::ffi::c_uint;
    #[c2rust::src_loc = "94:2"]
    pub const P_SUSPEND: PauseMode = 2;
    #[c2rust::src_loc = "93:2"]
    pub const P_PAUSE: PauseMode = 1;
    #[c2rust::src_loc = "92:2"]
    pub const P_NONE: PauseMode = 0;
    #[c2rust::src_loc = "97:1"]
    pub type ShutDownMode = ::core::ffi::c_uint;
    #[c2rust::src_loc = "119:2"]
    pub const SHUTDOWN_IMMEDIATE: ShutDownMode = 3;
    #[c2rust::src_loc = "117:2"]
    pub const SHUTDOWN_WAIT_FOR_CLIENTS: ShutDownMode = 2;
    #[c2rust::src_loc = "106:2"]
    pub const SHUTDOWN_WAIT_FOR_SERVERS: ShutDownMode = 1;
    #[c2rust::src_loc = "99:2"]
    pub const SHUTDOWN_NONE: ShutDownMode = 0;
    #[c2rust::src_loc = "122:1"]
    pub type SSLMode = ::core::ffi::c_uint;
    #[c2rust::src_loc = "128:2"]
    pub const SSLMODE_VERIFY_FULL: SSLMode = 5;
    #[c2rust::src_loc = "127:2"]
    pub const SSLMODE_VERIFY_CA: SSLMode = 4;
    #[c2rust::src_loc = "126:2"]
    pub const SSLMODE_REQUIRE: SSLMode = 3;
    #[c2rust::src_loc = "125:2"]
    pub const SSLMODE_PREFER: SSLMode = 2;
    #[c2rust::src_loc = "124:2"]
    pub const SSLMODE_ALLOW: SSLMode = 1;
    #[c2rust::src_loc = "123:2"]
    pub const SSLMODE_DISABLED: SSLMode = 0;
    #[c2rust::src_loc = "131:1"]
    pub type PacketCallbackFlag = ::core::ffi::c_uint;
    #[c2rust::src_loc = "144:2"]
    pub const CB_HANDLE_COMPLETE_PACKET: PacketCallbackFlag = 2;
    #[c2rust::src_loc = "139:2"]
    pub const CB_WANT_COMPLETE_PACKET: PacketCallbackFlag = 1;
    #[c2rust::src_loc = "133:2"]
    pub const CB_NONE: PacketCallbackFlag = 0;
    #[c2rust::src_loc = "147:1"]
    pub type LoadBalanceHosts = ::core::ffi::c_uint;
    #[c2rust::src_loc = "149:2"]
    pub const LOAD_BALANCE_HOSTS_ROUND_ROBIN: LoadBalanceHosts = 1;
    #[c2rust::src_loc = "148:2"]
    pub const LOAD_BALANCE_HOSTS_DISABLE: LoadBalanceHosts = 0;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "663:1"]
    pub struct PgSocket {
        pub head: List,
        pub cancel_head: List,
        pub link: *mut PgSocket,
        pub pool: *mut PgPool,
        pub login_user_credentials: *mut PgCredentials,
        pub id: ::core::ffi::c_ulonglong,
        pub client_auth_type: ::core::ffi::c_int,
        pub outstanding_requests: StatList,
        #[bitfield(name = "state", ty = "SocketState", bits = "0..=7")]
        #[bitfield(name = "contributes_db_client_count", ty = "bool", bits = "8..=8")]
        #[bitfield(name = "user_connection_counted", ty = "bool", bits = "9..=9")]
        #[bitfield(name = "ready", ty = "bool", bits = "10..=10")]
        #[bitfield(name = "idle_tx", ty = "bool", bits = "11..=11")]
        #[bitfield(name = "close_needed", ty = "bool", bits = "12..=12")]
        #[bitfield(name = "setting_vars", ty = "bool", bits = "13..=13")]
        #[bitfield(name = "exec_on_connect", ty = "bool", bits = "14..=14")]
        #[bitfield(name = "resetting", ty = "bool", bits = "15..=15")]
        #[bitfield(name = "copy_mode", ty = "bool", bits = "16..=16")]
        #[bitfield(name = "wait_for_welcome", ty = "bool", bits = "17..=17")]
        #[bitfield(name = "welcome_sent", ty = "bool", bits = "18..=18")]
        #[bitfield(name = "wait_for_user_conn", ty = "bool", bits = "19..=19")]
        #[bitfield(name = "wait_for_user", ty = "bool", bits = "20..=20")]
        #[bitfield(name = "wait_for_auth", ty = "bool", bits = "21..=21")]
        #[bitfield(name = "suspended", ty = "bool", bits = "22..=22")]
        #[bitfield(name = "sent_wait_notification", ty = "bool", bits = "23..=23")]
        #[bitfield(name = "admin_user", ty = "bool", bits = "24..=24")]
        #[bitfield(name = "own_user", ty = "bool", bits = "25..=25")]
        #[bitfield(name = "wait_for_response", ty = "bool", bits = "26..=26")]
        #[bitfield(name = "wait_sslchar", ty = "bool", bits = "27..=27")]
        #[bitfield(name = "query_failed", ty = "bool", bits = "28..=28")]
        pub state_contributes_db_client_count_user_connection_counted_ready_idle_tx_close_needed_setting_vars_exec_on_connect_resetting_copy_mode_wait_for_welcome_welcome_sent_wait_for_user_conn_wait_for_user_wait_for_auth_suspended_sent_wait_notification_admin_user_own_user_wait_for_response_wait_sslchar_query_failed:
            [u8; 4],
        pub replication: ReplicationType,
        pub startup_options: *mut ::core::ffi::c_char,
        pub connect_time: usec_t,
        pub request_time: usec_t,
        pub query_start: usec_t,
        pub xact_start: usec_t,
        pub wait_start: usec_t,
        pub cancel_key: [uint8_t; 8],
        pub canceling_clients: StatList,
        pub canceled_server: *mut PgSocket,
        pub remote_addr: PgAddr,
        pub local_addr: PgAddr,
        pub host: *mut ::core::ffi::c_char,
        pub c2rust_unnamed: C2RustUnnamed_9,
        pub scram_state: ScramState,
        pub vars: VarCache,
        pub client_prepared_statements: *mut PgClientPreparedStatement,
        pub server_prepared_statements: *mut PgServerPreparedStatement,
        pub packet_cb_state: CallbackState,
        pub sbuf: SBuf,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "768:2"]
    pub struct CallbackState {
        #[bitfield(name = "flag", ty = "PacketCallbackFlag", bits = "0..=7")]
        pub flag: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
        pub pkt: PktHdr,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "732:2"]
    pub struct ScramState {
        pub client_nonce: *mut ::core::ffi::c_char,
        pub client_first_message_bare: *mut ::core::ffi::c_char,
        pub client_final_message_without_proof: *mut ::core::ffi::c_char,
        pub server_nonce: *mut ::core::ffi::c_char,
        pub server_first_message: *mut ::core::ffi::c_char,
        pub iterations: ::core::ffi::c_int,
        pub hash_type: pg_cryptohash_type,
        pub key_length: ::core::ffi::c_int,
        pub salt: *mut uint8_t,
        pub saltlen: ::core::ffi::c_int,
        pub SaltedPassword: *mut uint8_t,
        pub cbind_flag: ::core::ffi::c_char,
        pub adhoc: bool,
        pub encoded_salt: *mut ::core::ffi::c_char,
        pub ClientKey: [uint8_t; 32],
        pub StoredKey: [uint8_t; 32],
        pub ServerKey: [uint8_t; 32],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "727:2"]
    pub union C2RustUnnamed_9 {
        pub dns_token: *mut DNSToken,
        pub db: *mut PgDatabase,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "570:1"]
    pub struct PgDatabase {
        pub head: List,
        pub name: [::core::ffi::c_char; 64],
        pub peer_id: ::core::ffi::c_int,
        pub pool: *mut PgPool,
        pub host: *mut ::core::ffi::c_char,
        pub port: ::core::ffi::c_int,
        pub pool_size: ::core::ffi::c_int,
        pub min_pool_size: ::core::ffi::c_int,
        pub res_pool_size: ::core::ffi::c_int,
        pub pool_mode: ::core::ffi::c_int,
        pub max_db_client_connections: ::core::ffi::c_int,
        pub max_db_connections: ::core::ffi::c_int,
        pub server_lifetime: usec_t,
        pub connect_query: *mut ::core::ffi::c_char,
        pub load_balance_hosts: LoadBalanceHosts,
        pub startup_params: *mut PktBuf,
        pub dbname: *const ::core::ffi::c_char,
        pub auth_dbname: *mut ::core::ffi::c_char,
        pub forced_user_credentials: *mut PgCredentials,
        pub auth_user_credentials: *mut PgCredentials,
        pub auth_query: *mut ::core::ffi::c_char,
        pub db_paused: bool,
        pub db_wait_close: bool,
        pub db_dead: bool,
        pub db_auto: bool,
        pub db_disabled: bool,
        pub admin: bool,
        pub fake: bool,
        pub inactive_time: usec_t,
        pub active_stamp: ::core::ffi::c_uint,
        pub connection_count: ::core::ffi::c_int,
        pub client_connection_count: ::core::ffi::c_int,
        pub user_tree: AATree,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "509:1"]
    pub struct PgCredentials {
        pub tree_node: AANode,
        pub name: [::core::ffi::c_char; 128],
        pub passwd: [::core::ffi::c_char; 2048],
        pub mock_auth: bool,
        pub dynamic_passwd: bool,
        pub global_user: *mut PgGlobalUser,
        pub scram_ClientKey: [uint8_t; 32],
        pub scram_ServerKey: [uint8_t; 32],
        pub scram_StoredKey: [uint8_t; 32],
        pub scram_Iiterations: ::core::ffi::c_int,
        pub scram_SaltKey: *mut ::core::ffi::c_char,
        pub use_scram_keys: bool,
        pub adhoc_scram_secrets_cached: bool,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "549:1"]
    pub struct PgGlobalUser {
        pub credentials: PgCredentials,
        pub head: List,
        pub pool_list: List,
        pub pool_mode: ::core::ffi::c_int,
        pub pool_size: ::core::ffi::c_int,
        pub res_pool_size: ::core::ffi::c_int,
        pub transaction_timeout: usec_t,
        pub idle_transaction_timeout: usec_t,
        pub query_timeout: usec_t,
        pub client_idle_timeout: usec_t,
        pub max_user_connections: ::core::ffi::c_int,
        pub max_user_client_connections: ::core::ffi::c_int,
        pub connection_count: ::core::ffi::c_int,
        pub client_connection_count: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "345:1"]
    pub struct PgPool {
        pub head: List,
        pub map_head: List,
        pub db: *mut PgDatabase,
        pub user_credentials: *mut PgCredentials,
        pub active_client_list: StatList,
        pub waiting_client_list: StatList,
        pub waiting_cancel_req_list: StatList,
        pub active_cancel_req_list: StatList,
        pub active_server_list: StatList,
        pub active_cancel_server_list: StatList,
        pub being_canceled_server_list: StatList,
        pub idle_server_list: StatList,
        pub used_server_list: StatList,
        pub tested_server_list: StatList,
        pub new_server_list: StatList,
        pub stats: PgStats,
        pub newer_stats: PgStats,
        pub older_stats: PgStats,
        pub welcome_msg: *mut PktBuf,
        pub orig_vars: VarCache,
        pub last_lifetime_disconnect: usec_t,
        pub last_connect_time: usec_t,
        #[bitfield(name = "last_connect_failed", ty = "bool", bits = "0..=0")]
        pub last_connect_failed: [u8; 1],
        pub last_connect_failed_message: [::core::ffi::c_char; 100],
        #[bitfield(name = "last_login_failed", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "welcome_msg_ready", ty = "bool", bits = "1..=1")]
        pub last_login_failed_welcome_msg_ready: [u8; 1],
        pub rrcounter: uint16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "320:1"]
    pub struct PgStats {
        pub server_assignment_count: uint64_t,
        pub xact_count: uint64_t,
        pub query_count: uint64_t,
        pub server_bytes: uint64_t,
        pub client_bytes: uint64_t,
        pub xact_time: usec_t,
        pub query_time: usec_t,
        pub wait_time: usec_t,
        pub ps_server_parse_count: uint64_t,
        pub ps_client_parse_count: uint64_t,
        pub ps_bind_count: uint64_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "292:1"]
    pub union PgAddr {
        pub sa: sockaddr,
        pub sin: sockaddr_in,
        pub sin6: sockaddr_in6,
        pub scred: sockaddr_ucreds,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "282:1"]
    pub struct sockaddr_ucreds {
        pub sin: sockaddr_in,
        pub uid: uid_t,
        pub pid: pid_t,
    }
    #[c2rust::src_loc = "650:1"]
    pub type ReplicationType = ::core::ffi::c_uint;
    #[c2rust::src_loc = "653:2"]
    pub const REPLICATION_PHYSICAL: ReplicationType = 2;
    #[c2rust::src_loc = "652:2"]
    pub const REPLICATION_LOGICAL: ReplicationType = 1;
    #[c2rust::src_loc = "651:2"]
    pub const REPLICATION_NONE: ReplicationType = 0;
    #[c2rust::src_loc = "232:1"]
    pub type auth_type = ::core::ffi::c_uint;
    #[c2rust::src_loc = "243:2"]
    pub const AUTH_TYPE_REJECT: auth_type = 10;
    #[c2rust::src_loc = "242:2"]
    pub const AUTH_TYPE_PEER: auth_type = 9;
    #[c2rust::src_loc = "241:2"]
    pub const AUTH_TYPE_SCRAM_SHA_256: auth_type = 8;
    #[c2rust::src_loc = "240:2"]
    pub const AUTH_TYPE_PAM: auth_type = 7;
    #[c2rust::src_loc = "239:2"]
    pub const AUTH_TYPE_LDAP: auth_type = 6;
    #[c2rust::src_loc = "238:2"]
    pub const AUTH_TYPE_HBA: auth_type = 5;
    #[c2rust::src_loc = "237:2"]
    pub const AUTH_TYPE_CERT: auth_type = 4;
    #[c2rust::src_loc = "236:2"]
    pub const AUTH_TYPE_MD5: auth_type = 3;
    #[c2rust::src_loc = "235:2"]
    pub const AUTH_TYPE_PLAIN: auth_type = 2;
    #[c2rust::src_loc = "234:2"]
    pub const AUTH_TYPE_TRUST: auth_type = 1;
    #[c2rust::src_loc = "233:2"]
    pub const AUTH_TYPE_ANY: auth_type = 0;
    #[c2rust::src_loc = "255:9"]
    pub const POOL_SESSION: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "256:9"]
    pub const POOL_TX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "257:9"]
    pub const POOL_STMT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    use super::_pid_t_h::pid_t;
    use super::_uid_t_h::uid_t;
    use super::_uint16_t_h::uint16_t;
    use super::_uint64_t_h::uint64_t;
    use super::_uint8_t_h::uint8_t;
    use super::aatree_h::{AANode, AATree};
    use super::cryptohash_h::pg_cryptohash_type;
    use super::dnslookup_h::DNSToken;
    use super::in6_h::sockaddr_in6;
    use super::in_h::sockaddr_in;
    use super::list_h::List;
    use super::pktbuf_h::PktBuf;
    use super::prepare_h::{PgClientPreparedStatement, PgServerPreparedStatement};
    use super::proto_h::PktHdr;
    use super::sbuf_h::SBuf;
    use super::socket_h::sockaddr;
    use super::statlist_h::StatList;
    use super::time_h::usec_t;
    use super::varcache_h::VarCache;
    extern "C" {
        #[c2rust::src_loc = "842:1"]
        pub static mut any_user_level_timeout_set: bool;
        #[c2rust::src_loc = "843:1"]
        pub static mut any_user_level_client_timeout_set: bool;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/sbuf.h:23"]
pub mod sbuf_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "71:1"]
    pub struct SBuf {
        pub ev: event,
        pub wait_type: uint8_t,
        pub pkt_action: uint8_t,
        pub tls_state: uint8_t,
        pub sock: ::core::ffi::c_int,
        pub pkt_remain: ::core::ffi::c_uint,
        pub skip_remain: ::core::ffi::c_uint,
        pub extra_packets: MBuf,
        pub extra_packet_queue_after: bool,
        pub proto_cb: sbuf_cb_t,
        pub dst: *mut SBuf,
        pub io: *mut IOBuf,
        pub ops: *const SBufIO,
        pub tls: *mut tls,
        pub tls_host: *const ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "58:1"]
    pub struct SBufIO {
        pub sbufio_peek:
            Option<unsafe extern "C" fn(*mut SBuf, *mut ::core::ffi::c_void, size_t) -> ssize_t>,
        pub sbufio_recv:
            Option<unsafe extern "C" fn(*mut SBuf, *mut ::core::ffi::c_void, size_t) -> ssize_t>,
        pub sbufio_send:
            Option<unsafe extern "C" fn(*mut SBuf, *const ::core::ffi::c_void, size_t) -> ssize_t>,
        pub sbufio_close: Option<unsafe extern "C" fn(*mut SBuf) -> ::core::ffi::c_int>,
    }
    #[c2rust::src_loc = "54:1"]
    pub type sbuf_cb_t = Option<unsafe extern "C" fn(*mut SBuf, SBufEvent, *mut MBuf) -> bool>;
    #[c2rust::src_loc = "24:9"]
    pub type SBufEvent = ::core::ffi::c_uint;
    #[c2rust::src_loc = "32:2"]
    pub const SBUF_EV_TLS_READY: SBufEvent = 7;
    #[c2rust::src_loc = "31:2"]
    pub const SBUF_EV_PKT_CALLBACK: SBufEvent = 6;
    #[c2rust::src_loc = "30:2"]
    pub const SBUF_EV_FLUSH: SBufEvent = 5;
    #[c2rust::src_loc = "29:2"]
    pub const SBUF_EV_CONNECT_OK: SBufEvent = 4;
    #[c2rust::src_loc = "28:2"]
    pub const SBUF_EV_CONNECT_FAILED: SBufEvent = 3;
    #[c2rust::src_loc = "27:2"]
    pub const SBUF_EV_SEND_FAILED: SBufEvent = 2;
    #[c2rust::src_loc = "26:2"]
    pub const SBUF_EV_RECV_FAILED: SBufEvent = 1;
    #[c2rust::src_loc = "25:2"]
    pub const SBUF_EV_READ: SBufEvent = 0;
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    use super::_uint8_t_h::uint8_t;
    use super::event_struct_h::event;
    use super::iobuf_h::IOBuf;
    use super::mbuf_h::MBuf;
    use super::tls_h::tls;
    extern "C" {
        #[c2rust::src_loc = "117:1"]
        pub fn sbuf_tls_setup() -> bool;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/iobuf.h:23"]
pub mod iobuf_h {
    #[c2rust::src_loc = "55:1"]
    pub type IOBuf = iobuf;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:1"]
    pub struct iobuf {
        pub done_pos: ::core::ffi::c_uint,
        pub parse_pos: ::core::ffi::c_uint,
        pub recv_pos: ::core::ffi::c_uint,
        pub buf: [uint8_t; 0],
    }
    use super::_uint8_t_h::uint8_t;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/mbuf.h:23"]
pub mod mbuf_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "14:1"]
    pub struct MBuf {
        pub data: *mut uint8_t,
        pub read_pos: ::core::ffi::c_uint,
        pub write_pos: ::core::ffi::c_uint,
        pub alloc_len: ::core::ffi::c_uint,
        pub reader: bool,
        pub fixed: bool,
    }
    use super::_uint8_t_h::uint8_t;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/proto.h:23"]
pub mod proto_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "34:1"]
    pub struct PktHdr {
        pub type_0: ::core::ffi::c_uint,
        pub len: ::core::ffi::c_uint,
        pub data: MBuf,
    }
    use super::mbuf_h::MBuf;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/prepare.h:23"]
pub mod prepare_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:9"]
    pub struct PgServerPreparedStatement {
        pub query_id: uint64_t,
        pub hh: UT_hash_handle,
        pub ps: *mut PgPreparedStatement,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:9"]
    pub struct PgPreparedStatement {
        pub hh: UT_hash_handle,
        pub query_id: uint64_t,
        pub use_count: uint32_t,
        pub query_and_parameters_len: size_t,
        pub stmt_name_len: uint8_t,
        pub stmt_name: [::core::ffi::c_char; 31],
        pub query_and_parameters: [::core::ffi::c_char; 0],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "21:9"]
    pub struct PgClientPreparedStatement {
        pub hh: UT_hash_handle,
        pub ps: *mut PgPreparedStatement,
        pub stmt_name: [::core::ffi::c_char; 0],
    }
    use super::_size_t_h::size_t;
    use super::_uint32_t_h::uint32_t;
    use super::_uint64_t_h::uint64_t;
    use super::_uint8_t_h::uint8_t;
    use super::uthash_h::UT_hash_handle;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/varcache.h:23"]
pub mod varcache_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "14:1"]
    pub struct VarCache {
        pub var_list: *mut *mut PStr,
    }
    use super::strpool_h::PStr;
    extern "C" {
        #[c2rust::src_loc = "18:1"]
        pub fn init_var_lookup(cf_track_extra_parameters_0: *const ::core::ffi::c_char);
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/strpool.h:23"]
pub mod strpool_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:1"]
    pub struct PStr {
        pub pool: *mut StrPool,
        pub len: size_t,
        pub refcnt: ::core::ffi::c_int,
        pub str_0: [::core::ffi::c_char; 0],
    }
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "34:1"]
        pub type StrPool;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/pktbuf.h:23"]
pub mod pktbuf_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "26:1"]
    pub struct PktBuf {
        pub buf: *mut uint8_t,
        pub buf_len: ::core::ffi::c_int,
        pub write_pos: ::core::ffi::c_int,
        pub pktlen_pos: ::core::ffi::c_int,
        pub send_pos: ::core::ffi::c_int,
        pub ev: *mut event,
        pub queued_dst: *mut PgSocket,
        #[bitfield(name = "failed", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "sending", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "fixed_buf", ty = "bool", bits = "2..=2")]
        pub failed_sending_fixed_buf: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
    }
    use super::_uint8_t_h::uint8_t;
    use super::bouncer_h::PgSocket;
    use super::event_struct_h::event;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/dnslookup.h:23"]
pub mod dnslookup_h {
    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub type DNSToken;
        #[c2rust::src_loc = "19:1"]
        pub type DNSContext;
        #[c2rust::src_loc = "25:1"]
        pub fn adns_create_context() -> *mut DNSContext;
        #[c2rust::src_loc = "33:1"]
        pub fn adns_get_backend() -> *const ::core::ffi::c_char;
        #[c2rust::src_loc = "45:1"]
        pub fn adns_per_loop(ctx: *mut DNSContext);
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
    #[c2rust::src_loc = "69:1"]
    pub type logging_prefix_fn_t = Option<
        unsafe extern "C" fn(
            LogLevel,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int,
    >;
    extern "C" {
        #[c2rust::src_loc = "76:1"]
        pub static mut logging_prefix_cb: logging_prefix_fn_t;
        #[c2rust::src_loc = "85:1"]
        pub static mut cf_verbose: ::core::ffi::c_int;
        #[c2rust::src_loc = "91:1"]
        pub static mut cf_quiet: ::core::ffi::c_int;
        #[c2rust::src_loc = "96:1"]
        pub static mut cf_logfile: *const ::core::ffi::c_char;
        #[c2rust::src_loc = "99:1"]
        pub static mut cf_syslog: ::core::ffi::c_int;
        #[c2rust::src_loc = "101:1"]
        pub static mut cf_syslog_ident: *const ::core::ffi::c_char;
        #[c2rust::src_loc = "103:1"]
        pub static mut cf_syslog_facility: *const ::core::ffi::c_char;
        #[c2rust::src_loc = "117:1"]
        pub fn log_generic(
            level: LogLevel,
            ctx: *mut ::core::ffi::c_void,
            s: *const ::core::ffi::c_char,
            ...
        );
        #[c2rust::src_loc = "120:1"]
        pub fn log_fatal(
            file: *const ::core::ffi::c_char,
            line: ::core::ffi::c_int,
            func: *const ::core::ffi::c_char,
            show_perror: bool,
            ctx: *mut ::core::ffi::c_void,
            s: *const ::core::ffi::c_char,
            ...
        );
        #[c2rust::src_loc = "179:1"]
        pub fn reset_logging();
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/hba.h:23"]
pub mod hba_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "56:1"]
    pub struct HBA {
        pub rules: List,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "73:1"]
    pub struct Ident {
        pub maps: List,
    }
    use super::list_h::List;
    extern "C" {
        #[c2rust::src_loc = "77:1"]
        pub fn ident_load_map(fn_0: *const ::core::ffi::c_char) -> *mut Ident;
        #[c2rust::src_loc = "78:1"]
        pub fn ident_free(ident: *mut Ident);
        #[c2rust::src_loc = "79:1"]
        pub fn hba_load_rules(fn_0: *const ::core::ffi::c_char, ident: *mut Ident) -> *mut HBA;
        #[c2rust::src_loc = "80:1"]
        pub fn hba_free(hba: *mut HBA);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/getopt.h:28"]
pub mod getopt_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "57:1"]
    pub struct option {
        pub name: *const ::core::ffi::c_char,
        pub has_arg: ::core::ffi::c_int,
        pub flag: *mut ::core::ffi::c_int,
        pub val: ::core::ffi::c_int,
    }
    #[c2rust::src_loc = "53:9"]
    pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "54:9"]
    pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    extern "C" {
        #[c2rust::src_loc = "72:1"]
        pub fn getopt_long(
            __argc: ::core::ffi::c_int,
            _: *const *mut ::core::ffi::c_char,
            _: *const ::core::ffi::c_char,
            _: *const option,
            _: *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/unistd.h:23"]
pub mod unistd_h {
    use super::_pid_t_h::pid_t;
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    use super::_uid_t_h::uid_t;
    extern "C" {
        #[c2rust::src_loc = "434:1"]
        pub fn _exit(_: ::core::ffi::c_int) -> !;
        #[c2rust::src_loc = "441:1"]
        pub fn close(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "444:1"]
        pub fn dup2(_: ::core::ffi::c_int, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "451:1"]
        pub fn fork() -> pid_t;
        #[c2rust::src_loc = "464:1"]
        pub fn getpid() -> pid_t;
        #[c2rust::src_loc = "466:1"]
        pub fn getuid() -> uid_t;
        #[c2rust::src_loc = "476:1"]
        pub fn read(_: ::core::ffi::c_int, _: *mut ::core::ffi::c_void, __nbyte: size_t)
            -> ssize_t;
        #[c2rust::src_loc = "481:1"]
        pub fn setsid() -> pid_t;
        #[c2rust::src_loc = "498:1"]
        pub fn unlink(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "515:1"]
        pub static mut optarg: *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "516:1"]
        pub static mut optind: ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_time.h:23"]
pub mod _time_h {
    use super::_time_t_h::time_t;
    extern "C" {
        #[c2rust::src_loc = "121:1"]
        pub fn time(_: *mut time_t) -> time_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:23"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:23"]
pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "83:1"]
        pub fn memset(
            __b: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __len: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "95:1"]
        pub fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "96:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/util.h:23"]
pub mod util_h {
    use super::cfparser_h::CfValue;
    use super::logging_h::LogLevel;
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn log_socket_prefix(
            lev: LogLevel,
            ctx: *mut ::core::ffi::c_void,
            dst: *mut ::core::ffi::c_char,
            dstlen: ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "57:1"]
        pub fn rescue_timers();
        #[c2rust::src_loc = "69:1"]
        pub fn cf_set_authdb(cv: *mut CfValue, value: *const ::core::ffi::c_char) -> bool;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/objects.h:23"]
pub mod objects_h {

    use super::statlist_h::StatList;
    extern "C" {
        #[c2rust::src_loc = "19:1"]
        pub static mut user_list: StatList;
        #[c2rust::src_loc = "23:1"]
        pub static mut database_list: StatList;
        #[c2rust::src_loc = "24:1"]
        pub static mut peer_list: StatList;
        #[c2rust::src_loc = "111:1"]
        pub fn reuse_just_freed_objects();
        #[c2rust::src_loc = "113:1"]
        pub fn init_objects();
        #[c2rust::src_loc = "115:1"]
        pub fn init_caches();
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/signal.h:25"]
pub mod include_signal_h {
    #[inline(always)]
    #[c2rust::src_loc = "116:1"]
    pub unsafe extern "C" fn __sigbits(mut __signo: ::core::ffi::c_int) -> ::core::ffi::c_int {
        return if __signo > __DARWIN_NSIG {
            0 as ::core::ffi::c_int
        } else {
            (1 as ::core::ffi::c_int) << __signo - 1 as ::core::ffi::c_int
        };
    }
    use super::_pid_t_h::pid_t;
    use super::_sigset_t_h::sigset_t;
    use super::signal_h::__DARWIN_NSIG;
    extern "C" {
        #[c2rust::src_loc = "83:1"]
        pub fn kill(_: pid_t, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "100:1"]
        pub fn sigprocmask(
            _: ::core::ffi::c_int,
            _: *const sigset_t,
            _: *mut sigset_t,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/safeio.h:29"]
pub mod safeio_h {
    use super::_size_t_h::size_t;
    use super::_socklen_t_h::socklen_t;
    use super::_ssize_t_h::ssize_t;
    use super::socket_h::sockaddr;
    extern "C" {
        #[c2rust::src_loc = "29:1"]
        pub fn safe_write(
            fd: ::core::ffi::c_int,
            buf: *const ::core::ffi::c_void,
            len: size_t,
        ) -> ssize_t;
        #[c2rust::src_loc = "35:1"]
        pub fn safe_close(fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "41:1"]
        pub fn safe_connect(
            fd: ::core::ffi::c_int,
            sa: *const sockaddr,
            sa_len: socklen_t,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/config.h:23"]
pub mod config_h {
    #[c2rust::src_loc = "350:9"]
    pub const PACKAGE_BUGREPORT: [::core::ffi::c_char; 46] = unsafe {
        ::core::mem::transmute::<[u8; 46], [::core::ffi::c_char; 46]>(
            *b"https://github.com/pgbouncer/pgbouncer/issues\0",
        )
    };
    #[c2rust::src_loc = "353:9"]
    pub const PACKAGE_NAME: [::core::ffi::c_char; 10] =
        unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"PgBouncer\0") };
    #[c2rust::src_loc = "356:9"]
    pub const PACKAGE_STRING: [::core::ffi::c_char; 17] = unsafe {
        ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"PgBouncer 1.25.1\0")
    };
    #[c2rust::src_loc = "362:9"]
    pub const PACKAGE_URL: [::core::ffi::c_char; 27] = unsafe {
        ::core::mem::transmute::<[u8; 27], [::core::ffi::c_char; 27]>(
            *b"https://www.pgbouncer.org/\0",
        )
    };
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/signal.h:23"]
pub mod signal_h {
    #[c2rust::src_loc = "76:9"]
    pub const __DARWIN_NSIG: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
    #[c2rust::src_loc = "319:9"]
    pub const SIG_BLOCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_printf.h:23"]
pub mod _printf_h {
    extern "C" {
        #[c2rust::src_loc = "34:1"]
        pub fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:23"]
pub mod _malloc_h {
    extern "C" {
        #[c2rust::src_loc = "56:1"]
        pub fn free(_: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdlib.h:23"]
pub mod _stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "144:1"]
        pub fn atexit(_: Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "151:1"]
        pub fn atol(_: *const ::core::ffi::c_char) -> ::core::ffi::c_long;
        #[c2rust::src_loc = "160:1"]
        pub fn exit(_: ::core::ffi::c_int) -> !;
        #[c2rust::src_loc = "162:1"]
        pub fn getenv(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "262:1"]
        pub fn srandom(_: ::core::ffi::c_uint);
        #[c2rust::src_loc = "326:1"]
        pub fn setprogname(_: *const ::core::ffi::c_char);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/errno.h:23"]
pub mod errno_h {
    #[c2rust::src_loc = "89:9"]
    pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "90:9"]
    pub const ESRCH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "91:9"]
    pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    extern "C" {
        #[c2rust::src_loc = "80:1"]
        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/llvm/21.1.8/lib/clang/21/include/stdbool.h:23"]
pub mod stdbool_h {
    #[c2rust::src_loc = "25:9"]
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/system.h:23"]
pub mod system_h {
    extern "C" {
        #[c2rust::src_loc = "53:1"]
        pub fn change_user(user: *const ::core::ffi::c_char);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/fcntl.h:23"]
pub mod fcntl_h {
    #[c2rust::src_loc = "96:9"]
    pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "97:9"]
    pub const O_WRONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "98:9"]
    pub const O_RDWR: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "125:9"]
    pub const O_CREAT: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
    #[c2rust::src_loc = "127:9"]
    pub const O_EXCL: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
    extern "C" {
        #[c2rust::src_loc = "602:1"]
        pub fn open(
            _: *const ::core::ffi::c_char,
            _: ::core::ffi::c_int,
            ...
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/string.h:23"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "138:1"]
        pub fn usual_basename(path: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
        #[c2rust::src_loc = "217:1"]
        pub fn strcmpeq(
            str_left: *const ::core::ffi::c_char,
            str_right: *const ::core::ffi::c_char,
        ) -> bool;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/admin.h:23"]
pub mod admin_h {
    extern "C" {
        #[c2rust::src_loc = "21:1"]
        pub fn admin_setup();
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/loader.h:23"]
pub mod loader_h {
    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub fn parse_database(
            base: *mut ::core::ffi::c_void,
            name: *const ::core::ffi::c_char,
            connstr: *const ::core::ffi::c_char,
        ) -> bool;
        #[c2rust::src_loc = "21:1"]
        pub fn parse_peer(
            base: *mut ::core::ffi::c_void,
            name: *const ::core::ffi::c_char,
            connstr: *const ::core::ffi::c_char,
        ) -> bool;
        #[c2rust::src_loc = "23:1"]
        pub fn parse_user(
            base: *mut ::core::ffi::c_void,
            name: *const ::core::ffi::c_char,
            params: *const ::core::ffi::c_char,
        ) -> bool;
        #[c2rust::src_loc = "27:1"]
        pub fn loader_users_check() -> bool;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/pooler.h:23"]
pub mod pooler_h {
    extern "C" {
        #[c2rust::src_loc = "19:1"]
        pub fn pooler_setup();
        #[c2rust::src_loc = "23:1"]
        pub fn per_loop_pooler_maint();
        #[c2rust::src_loc = "24:1"]
        pub fn pooler_tune_accept(on: bool);
        #[c2rust::src_loc = "25:1"]
        pub fn cleanup_tcp_sockets();
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/stats.h:23"]
pub mod stats_h {
    extern "C" {
        #[c2rust::src_loc = "19:1"]
        pub fn stats_setup();
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/takeover.h:23"]
pub mod takeover_h {
    extern "C" {
        #[c2rust::src_loc = "19:1"]
        pub fn takeover_init();
        #[c2rust::src_loc = "22:1"]
        pub fn takeover_finish();
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/janitor.h:23"]
pub mod janitor_h {
    extern "C" {
        #[c2rust::src_loc = "19:1"]
        pub fn janitor_setup();
        #[c2rust::src_loc = "20:1"]
        pub fn config_postprocess();
        #[c2rust::src_loc = "21:1"]
        pub fn resume_all();
        #[c2rust::src_loc = "22:1"]
        pub fn per_loop_maint();
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/ldapauth.h:23"]
pub mod ldapauth_h {
    extern "C" {
        #[c2rust::src_loc = "30:1"]
        pub fn auth_ldap_init();
        #[c2rust::src_loc = "32:1"]
        pub fn ldap_poll() -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/pam.h:23"]
pub mod pam_h {
    extern "C" {
        #[c2rust::src_loc = "33:1"]
        pub fn pam_init();
        #[c2rust::src_loc = "35:1"]
        pub fn pam_poll() -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/err.h:26"]
pub mod err_h {
    extern "C" {
        #[c2rust::src_loc = "65:1"]
        pub fn xstrdup(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    }
}
pub use self::_in_addr_t_h::in_addr_t;
pub use self::_in_port_t_h::in_port_t;
use self::_malloc_h::free;
pub use self::_null_h::NULL;
pub use self::_pid_t_h::pid_t;
use self::_printf_h::printf;
pub use self::_ptrdiff_t_h::ptrdiff_t;
pub use self::_sa_family_t_h::sa_family_t;
pub use self::_sigset_t_h::sigset_t;
pub use self::_size_t_h::size_t;
pub use self::_socklen_t_h::socklen_t;
pub use self::_ssize_t_h::ssize_t;
pub use self::_stdio_h::{__sFILE, __sFILEX, __sbuf, __stderrp, fpos_t, fprintf, snprintf, FILE};
use self::_stdlib_h::{atexit, atol, exit, getenv, setprogname, srandom};
use self::_string_h::{memset, strerror, strlen};
use self::_time_h::time;
pub use self::_time_t_h::time_t;
pub use self::_timeval_h::timeval;
pub use self::_types_h::{
    __darwin_ptrdiff_t, __darwin_size_t, __darwin_socklen_t, __darwin_ssize_t, __darwin_time_t,
    __int32_t, __int64_t, __uint16_t, __uint32_t, __uint64_t, __uint8_t,
};
pub use self::_uid_t_h::uid_t;
pub use self::_uint16_t_h::uint16_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::aatree_h::{aatree_cmp_f, aatree_walker_f, AANode, AATree};
use self::admin_h::admin_setup;
pub use self::bouncer_h::{
    any_user_level_client_timeout_set, any_user_level_timeout_set, auth_type, sockaddr_ucreds,
    C2RustUnnamed_9, CallbackState, LoadBalanceHosts, PacketCallbackFlag, PauseMode, PgAddr,
    PgCredentials, PgDatabase, PgGlobalUser, PgPool, PgSocket, PgStats, ReplicationType, SSLMode,
    ScramState, ShutDownMode, SocketState, AUTH_TYPE_ANY, AUTH_TYPE_CERT, AUTH_TYPE_HBA,
    AUTH_TYPE_LDAP, AUTH_TYPE_MD5, AUTH_TYPE_PAM, AUTH_TYPE_PEER, AUTH_TYPE_PLAIN,
    AUTH_TYPE_REJECT, AUTH_TYPE_SCRAM_SHA_256, AUTH_TYPE_TRUST, CB_HANDLE_COMPLETE_PACKET, CB_NONE,
    CB_WANT_COMPLETE_PACKET, CL_ACTIVE, CL_ACTIVE_CANCEL, CL_FREE, CL_JUSTFREE, CL_LOGIN,
    CL_WAITING, CL_WAITING_CANCEL, CL_WAITING_LOGIN, LOAD_BALANCE_HOSTS_DISABLE,
    LOAD_BALANCE_HOSTS_ROUND_ROBIN, POOL_SESSION, POOL_STMT, POOL_TX, P_NONE, P_PAUSE, P_SUSPEND,
    REPLICATION_LOGICAL, REPLICATION_NONE, REPLICATION_PHYSICAL, SHUTDOWN_IMMEDIATE, SHUTDOWN_NONE,
    SHUTDOWN_WAIT_FOR_CLIENTS, SHUTDOWN_WAIT_FOR_SERVERS, SSLMODE_ALLOW, SSLMODE_DISABLED,
    SSLMODE_PREFER, SSLMODE_REQUIRE, SSLMODE_VERIFY_CA, SSLMODE_VERIFY_FULL, SV_ACTIVE,
    SV_ACTIVE_CANCEL, SV_BEING_CANCELED, SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN, SV_TESTED,
    SV_USED,
};
pub use self::cfparser_h::{
    cf_get, cf_get_int, cf_get_lookup, cf_get_str, cf_get_time_usec, cf_get_uint, cf_load_file,
    cf_set, cf_set_int, cf_set_lookup, cf_set_str, cf_set_time_usec, cf_set_uint, CfContext, CfKey,
    CfLookup, CfOps, CfSect, CfValue, CF_NO_RELOAD, CF_READONLY, CF_VAL_ABS,
};
pub use self::config_h::{PACKAGE_BUGREPORT, PACKAGE_NAME, PACKAGE_STRING, PACKAGE_URL};
pub use self::cryptohash_h::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};
use self::dnslookup_h::{adns_create_context, adns_get_backend, adns_per_loop, DNSContext};
use self::err_h::xstrdup;
pub use self::errno_h::{__error, EINTR, ENOENT, ESRCH};
pub use self::event_h::{
    event_add, event_assign, event_base, event_base_free, event_base_get_method, event_base_loop,
    event_base_new, event_callback_fn, event_get_version, EVLOOP_ONCE, EV_PERSIST, EV_SIGNAL,
};
pub use self::event_struct_h::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
pub use self::fcntl_h::{open, O_CREAT, O_EXCL, O_RDONLY, O_RDWR, O_WRONLY};
pub use self::getopt_h::{getopt_long, no_argument, option, required_argument};
pub use self::hba_h::{hba_free, hba_load_rules, ident_free, ident_load_map, Ident, HBA};
pub use self::in6_h::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use self::in_h::{in_addr, sockaddr_in};
pub use self::include_signal_h::{__sigbits, kill, sigprocmask};
pub use self::iobuf_h::{iobuf, IOBuf};
use self::janitor_h::{config_postprocess, janitor_setup, per_loop_maint, resume_all};
use self::ldapauth_h::{auth_ldap_init, ldap_poll};
pub use self::list_h::List;
use self::loader_h::{loader_users_check, parse_database, parse_peer, parse_user};
pub use self::logging_h::{
    cf_logfile, cf_quiet, cf_syslog, cf_syslog_facility, cf_syslog_ident, cf_verbose, log_fatal,
    log_generic, logging_prefix_cb, logging_prefix_fn_t, reset_logging, LogLevel, LG_DEBUG,
    LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE, LG_STATS, LG_WARNING,
};
pub use self::mbuf_h::MBuf;
use self::objects_h::{
    database_list, init_caches, init_objects, peer_list, reuse_just_freed_objects, user_list,
};
use self::pam_h::{pam_init, pam_poll};
pub use self::pktbuf_h::PktBuf;
use self::pooler_h::{
    cleanup_tcp_sockets, per_loop_pooler_maint, pooler_setup, pooler_tune_accept,
};
pub use self::prepare_h::{
    PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement,
};
pub use self::proto_h::PktHdr;
pub use self::resource_h::{getrlimit, rlim_t, rlimit, RLIMIT_NOFILE};
use self::safeio_h::{safe_close, safe_connect, safe_write};
pub use self::sbuf_h::{
    sbuf_cb_t, sbuf_tls_setup, SBuf, SBufEvent, SBufIO, SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK,
    SBUF_EV_FLUSH, SBUF_EV_PKT_CALLBACK, SBUF_EV_READ, SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED,
    SBUF_EV_TLS_READY,
};
pub use self::signal_h::{SIG_BLOCK, __DARWIN_NSIG};
pub use self::socket_h::{sockaddr, socket, AF_UNIX, SOCK_STREAM};
pub use self::statlist_h::{statlist_count, StatList};
use self::stats_h::stats_setup;
pub use self::stdbool_h::{false_0, true_0};
use self::string_h::{strcmpeq, usual_basename};
pub use self::strpool_h::{PStr, StrPool};
pub use self::sys__types_h::{
    __darwin_off_t, __darwin_pid_t, __darwin_sigset_t, __darwin_suseconds_t, __darwin_uid_t,
    __DARWIN_NULL,
};
use self::system_h::change_user;
use self::takeover_h::{takeover_finish, takeover_init};
pub use self::time_h::{reset_time_cache, usec_t};
use self::tls_h::tls_backend_version;
pub use self::un_h::sockaddr_un;
use self::unistd_h::{
    _exit, close, dup2, fork, getpid, getuid, optarg, optind, read, setsid, unlink,
};
pub use self::uthash_h::{UT_hash_bucket, UT_hash_handle, UT_hash_table};
use self::util_h::{cf_set_authdb, log_socket_prefix, rescue_timers};
pub use self::varcache_h::{init_var_lookup, VarCache};
#[c2rust::src_loc = "48:1"]
unsafe extern "C" fn usage(mut exe: *const ::core::ffi::c_char) {
    printf(
        b"%s is a connection pooler for PostgreSQL.\n\n\0" as *const u8
            as *const ::core::ffi::c_char,
        exe,
    );
    printf(b"Usage:\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"  %s [OPTION]... CONFIG_FILE\n\0" as *const u8 as *const ::core::ffi::c_char,
        exe,
    );
    printf(b"\nOptions:\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"  -d, --daemon         run in background (as a daemon)\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(b"  -q, --quiet          run quietly\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"  -R, --reboot         do an online reboot\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"  -u, --user=USERNAME  assume identity of USERNAME\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"  -v, --verbose        increase verbosity\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    printf(
        b"  -V, --version        show version, then exit\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(
        b"  -h, --help           show this help, then exit\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    printf(
        b"Report bugs to <%s>.\n\0" as *const u8 as *const ::core::ffi::c_char,
        PACKAGE_BUGREPORT.as_ptr(),
    );
    printf(
        b"%s home page: <%s>\n\0" as *const u8 as *const ::core::ffi::c_char,
        PACKAGE_NAME.as_ptr(),
        PACKAGE_URL.as_ptr(),
    );
    exit(0 as ::core::ffi::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "74:1"]
pub static mut pgb_event_base: *mut event_base =
    ::core::ptr::null::<event_base>() as *mut event_base;
#[no_mangle]
#[c2rust::src_loc = "77:1"]
pub static mut adns: *mut DNSContext = ::core::ptr::null::<DNSContext>() as *mut DNSContext;
#[no_mangle]
#[c2rust::src_loc = "79:1"]
pub static mut parsed_hba: *mut HBA = ::core::ptr::null::<HBA>() as *mut HBA;
#[no_mangle]
#[c2rust::src_loc = "80:1"]
pub static mut parsed_ident: *mut Ident = ::core::ptr::null::<Ident>() as *mut Ident;
#[no_mangle]
#[c2rust::src_loc = "86:1"]
pub static mut cf_query_wait_notify: ::core::ffi::c_ulong = 0;
#[no_mangle]
#[c2rust::src_loc = "87:1"]
pub static mut cf_daemon: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "88:1"]
pub static mut cf_pause_mode: ::core::ffi::c_int = P_NONE as ::core::ffi::c_int;
#[no_mangle]
#[c2rust::src_loc = "89:1"]
pub static mut cf_shutdown: ::core::ffi::c_int = SHUTDOWN_NONE as ::core::ffi::c_int;
#[no_mangle]
#[c2rust::src_loc = "90:1"]
pub static mut cf_reboot: ::core::ffi::c_int = 0;
#[c2rust::src_loc = "91:1"]
static mut global_username: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub static mut cf_config_file: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "94:1"]
pub static mut cf_listen_addr: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "95:1"]
pub static mut cf_listen_port: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "96:1"]
pub static mut cf_listen_backlog: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "97:1"]
pub static mut cf_unix_socket_dir: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "98:1"]
pub static mut cf_unix_socket_mode: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "99:1"]
pub static mut cf_unix_socket_group: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "100:1"]
pub static mut cf_peer_id: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "102:1"]
pub static mut cf_pool_mode: ::core::ffi::c_int = POOL_SESSION;
#[no_mangle]
#[c2rust::src_loc = "105:1"]
pub static mut cf_sbuf_len: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "106:1"]
pub static mut cf_sbuf_loopcnt: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "107:1"]
pub static mut cf_so_reuseport: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "108:1"]
pub static mut cf_tcp_socket_buffer: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "109:1"]
pub static mut cf_tcp_defer_accept: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "115:1"]
pub static mut cf_tcp_keepalive: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "116:1"]
pub static mut cf_tcp_keepcnt: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "117:1"]
pub static mut cf_tcp_keepidle: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "118:1"]
pub static mut cf_tcp_keepintvl: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "119:1"]
pub static mut cf_tcp_user_timeout: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "121:1"]
pub static mut cf_auth_type: ::core::ffi::c_int = AUTH_TYPE_MD5 as ::core::ffi::c_int;
#[no_mangle]
#[c2rust::src_loc = "122:1"]
pub static mut cf_auth_file: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "123:1"]
pub static mut cf_auth_hba_file: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "124:1"]
pub static mut cf_auth_ident_file: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "125:1"]
pub static mut cf_auth_ldap_options: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "126:1"]
pub static mut cf_auth_user: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "127:1"]
pub static mut cf_auth_query: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "128:1"]
pub static mut cf_auth_dbname: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "129:1"]
pub static mut cf_track_extra_parameters: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "131:1"]
pub static mut cf_max_client_conn: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "132:1"]
pub static mut cf_default_pool_size: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "133:1"]
pub static mut cf_min_pool_size: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "134:1"]
pub static mut cf_res_pool_size: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "135:1"]
pub static mut cf_res_pool_timeout: usec_t = 0;
#[no_mangle]
#[c2rust::src_loc = "136:1"]
pub static mut cf_max_db_connections: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "137:1"]
pub static mut cf_max_db_client_connections: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "138:1"]
pub static mut cf_max_user_connections: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "139:1"]
pub static mut cf_max_user_client_connections: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "141:1"]
pub static mut cf_server_reset_query: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "142:1"]
pub static mut cf_server_reset_query_always: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "143:1"]
pub static mut cf_server_check_query: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "144:1"]
pub static mut empty_server_check_query: bool = false;
#[no_mangle]
#[c2rust::src_loc = "145:1"]
pub static mut cf_server_check_delay: usec_t = 0;
#[no_mangle]
#[c2rust::src_loc = "146:1"]
pub static mut cf_server_fast_close: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "147:1"]
pub static mut cf_server_round_robin: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "148:1"]
pub static mut cf_disable_pqexec: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "149:1"]
pub static mut cf_dns_max_ttl: usec_t = 0;
#[no_mangle]
#[c2rust::src_loc = "150:1"]
pub static mut cf_dns_nxdomain_ttl: usec_t = 0;
#[no_mangle]
#[c2rust::src_loc = "151:1"]
pub static mut cf_dns_zone_check_period: usec_t = 0;
#[no_mangle]
#[c2rust::src_loc = "152:1"]
pub static mut cf_resolv_conf: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "153:1"]
pub static mut cf_max_packet_size: ::core::ffi::c_uint = 0;
#[no_mangle]
#[c2rust::src_loc = "155:1"]
pub static mut cf_ignore_startup_params: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "157:1"]
pub static mut cf_autodb_connstr: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "159:1"]
pub static mut cf_autodb_idle_timeout: usec_t = 0;
#[no_mangle]
#[c2rust::src_loc = "161:1"]
pub static mut cf_server_lifetime: usec_t = 0;
#[no_mangle]
#[c2rust::src_loc = "162:1"]
pub static mut cf_server_idle_timeout: usec_t = 0;
#[no_mangle]
#[c2rust::src_loc = "163:1"]
pub static mut cf_server_connect_timeout: usec_t = 0;
#[no_mangle]
#[c2rust::src_loc = "164:1"]
pub static mut cf_server_login_retry: usec_t = 0;
#[no_mangle]
#[c2rust::src_loc = "165:1"]
pub static mut cf_query_timeout: usec_t = 0;
#[no_mangle]
#[c2rust::src_loc = "166:1"]
pub static mut cf_query_wait_timeout: usec_t = 0;
#[no_mangle]
#[c2rust::src_loc = "167:1"]
pub static mut cf_cancel_wait_timeout: usec_t = 0;
#[no_mangle]
#[c2rust::src_loc = "168:1"]
pub static mut cf_client_idle_timeout: usec_t = 0;
#[no_mangle]
#[c2rust::src_loc = "169:1"]
pub static mut cf_client_login_timeout: usec_t = 0;
#[no_mangle]
#[c2rust::src_loc = "170:1"]
pub static mut cf_idle_transaction_timeout: usec_t = 0;
#[no_mangle]
#[c2rust::src_loc = "171:1"]
pub static mut cf_transaction_timeout: usec_t = 0;
#[no_mangle]
#[c2rust::src_loc = "172:1"]
pub static mut cf_suspend_timeout: usec_t = 0;
#[no_mangle]
#[c2rust::src_loc = "174:1"]
pub static mut g_suspend_start: usec_t = 0;
#[no_mangle]
#[c2rust::src_loc = "176:1"]
pub static mut cf_pidfile: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "177:1"]
pub static mut cf_jobname: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "179:1"]
pub static mut cf_admin_users: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "180:1"]
pub static mut cf_stats_users: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "181:1"]
pub static mut cf_stats_period: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "182:1"]
pub static mut cf_log_stats: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "184:1"]
pub static mut cf_log_connections: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "185:1"]
pub static mut cf_log_disconnections: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "186:1"]
pub static mut cf_log_pooler_errors: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "187:1"]
pub static mut cf_application_name_add_host: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "189:1"]
pub static mut cf_client_tls_sslmode: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "190:1"]
pub static mut cf_client_tls_protocols: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "191:1"]
pub static mut cf_client_tls_ca_file: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "192:1"]
pub static mut cf_client_tls_cert_file: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "193:1"]
pub static mut cf_client_tls_key_file: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "194:1"]
pub static mut cf_client_tls_ciphers: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "195:1"]
pub static mut cf_client_tls13_ciphers: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "196:1"]
pub static mut cf_client_tls_dheparams: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "197:1"]
pub static mut cf_client_tls_ecdhecurve: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "199:1"]
pub static mut cf_server_tls_sslmode: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "200:1"]
pub static mut cf_server_tls_protocols: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "201:1"]
pub static mut cf_server_tls_ca_file: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "202:1"]
pub static mut cf_server_tls_cert_file: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "203:1"]
pub static mut cf_server_tls_key_file: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "204:1"]
pub static mut cf_server_tls_ciphers: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "205:1"]
pub static mut cf_server_tls13_ciphers: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
#[c2rust::src_loc = "207:1"]
pub static mut cf_max_prepared_statements: ::core::ffi::c_int = 0;
#[no_mangle]
#[c2rust::src_loc = "209:1"]
pub static mut cf_scram_iterations: ::core::ffi::c_int = 0;
#[c2rust::src_loc = "218:1"]
static mut auth_type_map: [CfLookup; 8] = [
    CfLookup {
        name: b"any\0" as *const u8 as *const ::core::ffi::c_char,
        value: AUTH_TYPE_ANY as ::core::ffi::c_int,
    },
    CfLookup {
        name: b"trust\0" as *const u8 as *const ::core::ffi::c_char,
        value: AUTH_TYPE_TRUST as ::core::ffi::c_int,
    },
    CfLookup {
        name: b"plain\0" as *const u8 as *const ::core::ffi::c_char,
        value: AUTH_TYPE_PLAIN as ::core::ffi::c_int,
    },
    CfLookup {
        name: b"md5\0" as *const u8 as *const ::core::ffi::c_char,
        value: AUTH_TYPE_MD5 as ::core::ffi::c_int,
    },
    CfLookup {
        name: b"cert\0" as *const u8 as *const ::core::ffi::c_char,
        value: AUTH_TYPE_CERT as ::core::ffi::c_int,
    },
    CfLookup {
        name: b"hba\0" as *const u8 as *const ::core::ffi::c_char,
        value: AUTH_TYPE_HBA as ::core::ffi::c_int,
    },
    CfLookup {
        name: b"scram-sha-256\0" as *const u8 as *const ::core::ffi::c_char,
        value: AUTH_TYPE_SCRAM_SHA_256 as ::core::ffi::c_int,
    },
    CfLookup {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0,
    },
];
#[no_mangle]
#[c2rust::src_loc = "235:1"]
pub static mut pool_mode_map: [CfLookup; 4] = [
    CfLookup {
        name: b"session\0" as *const u8 as *const ::core::ffi::c_char,
        value: POOL_SESSION,
    },
    CfLookup {
        name: b"transaction\0" as *const u8 as *const ::core::ffi::c_char,
        value: POOL_TX,
    },
    CfLookup {
        name: b"statement\0" as *const u8 as *const ::core::ffi::c_char,
        value: POOL_STMT,
    },
    CfLookup {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0,
    },
];
#[no_mangle]
#[c2rust::src_loc = "242:1"]
pub static mut sslmode_map: [CfLookup; 7] = [
    CfLookup {
        name: b"disable\0" as *const u8 as *const ::core::ffi::c_char,
        value: SSLMODE_DISABLED as ::core::ffi::c_int,
    },
    CfLookup {
        name: b"allow\0" as *const u8 as *const ::core::ffi::c_char,
        value: SSLMODE_ALLOW as ::core::ffi::c_int,
    },
    CfLookup {
        name: b"prefer\0" as *const u8 as *const ::core::ffi::c_char,
        value: SSLMODE_PREFER as ::core::ffi::c_int,
    },
    CfLookup {
        name: b"require\0" as *const u8 as *const ::core::ffi::c_char,
        value: SSLMODE_REQUIRE as ::core::ffi::c_int,
    },
    CfLookup {
        name: b"verify-ca\0" as *const u8 as *const ::core::ffi::c_char,
        value: SSLMODE_VERIFY_CA as ::core::ffi::c_int,
    },
    CfLookup {
        name: b"verify-full\0" as *const u8 as *const ::core::ffi::c_char,
        value: SSLMODE_VERIFY_FULL as ::core::ffi::c_int,
    },
    CfLookup {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0,
    },
];
#[no_mangle]
#[c2rust::src_loc = "252:1"]
pub static mut load_balance_hosts_map: [CfLookup; 3] = [
    CfLookup {
        name: b"disable\0" as *const u8 as *const ::core::ffi::c_char,
        value: LOAD_BALANCE_HOSTS_DISABLE as ::core::ffi::c_int,
    },
    CfLookup {
        name: b"round-robin\0" as *const u8 as *const ::core::ffi::c_char,
        value: LOAD_BALANCE_HOSTS_ROUND_ROBIN as ::core::ffi::c_int,
    },
    CfLookup {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        value: 0,
    },
];
#[c2rust::src_loc = "261:1"]
static mut bouncer_params: [CfKey; 99] = [CfKey {
    key_name: ::core::ptr::null::<::core::ffi::c_char>(),
    op: CfOps {
        setter: None,
        getter: None,
        op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
    },
    flags: 0,
    key_ofs: 0,
    def_value: ::core::ptr::null::<::core::ffi::c_char>(),
}; 99];
#[c2rust::src_loc = "371:1"]
static mut config_sects: [CfSect; 5] = unsafe {
    [
        CfSect {
            sect_name: b"pgbouncer\0" as *const u8 as *const ::core::ffi::c_char,
            key_list: &raw const bouncer_params as *const CfKey,
            base_lookup: None,
            set_key: None,
            get_key: None,
            section_start: None,
        },
        CfSect {
            sect_name: b"databases\0" as *const u8 as *const ::core::ffi::c_char,
            key_list: ::core::ptr::null::<CfKey>(),
            base_lookup: None,
            set_key: Some(
                parse_database
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> bool,
            ),
            get_key: None,
            section_start: None,
        },
        CfSect {
            sect_name: b"users\0" as *const u8 as *const ::core::ffi::c_char,
            key_list: ::core::ptr::null::<CfKey>(),
            base_lookup: None,
            set_key: Some(
                parse_user
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> bool,
            ),
            get_key: None,
            section_start: None,
        },
        CfSect {
            sect_name: b"peers\0" as *const u8 as *const ::core::ffi::c_char,
            key_list: ::core::ptr::null::<CfKey>(),
            base_lookup: None,
            set_key: Some(
                parse_peer
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> bool,
            ),
            get_key: None,
            section_start: None,
        },
        CfSect {
            sect_name: ::core::ptr::null::<::core::ffi::c_char>(),
            key_list: ::core::ptr::null::<CfKey>(),
            base_lookup: None,
            set_key: None,
            get_key: None,
            section_start: None,
        },
    ]
};
#[c2rust::src_loc = "389:1"]
static mut main_config: CfContext = unsafe {
    CfContext {
        sect_list: &raw const config_sects as *const CfSect,
        base: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        loaded: false,
    }
};
#[no_mangle]
#[c2rust::src_loc = "391:1"]
pub unsafe extern "C" fn set_config_param(
    mut key: *const ::core::ffi::c_char,
    mut val: *const ::core::ffi::c_char,
) -> bool {
    return cf_set(
        &raw mut main_config,
        b"pgbouncer\0" as *const u8 as *const ::core::ffi::c_char,
        key,
        val,
    );
}
#[no_mangle]
#[c2rust::src_loc = "396:1"]
pub unsafe extern "C" fn config_for_each(
    mut param_cb: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            bool,
        ) -> (),
    >,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut k = &raw const bouncer_params as *const CfKey;
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    let mut reloadable: bool = false;
    let mut val = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ro = CF_NO_RELOAD | CF_READONLY;
    while !(*k).key_name.is_null() {
        val = cf_get(
            &raw mut main_config,
            b"pgbouncer\0" as *const u8 as *const ::core::ffi::c_char,
            (*k).key_name,
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int,
        );
        reloadable = (*k).flags & ro == 0 as ::core::ffi::c_int;
        param_cb.expect("non-null function pointer")(
            arg,
            (*k).key_name,
            val,
            (*k).def_value,
            reloadable,
        );
        k = k.offset(1);
    }
}
#[c2rust::src_loc = "412:1"]
unsafe extern "C" fn set_defer_accept(
    mut cv: *mut CfValue,
    mut val: *const ::core::ffi::c_char,
) -> bool {
    let mut p = (*cv).value_p as *mut ::core::ffi::c_int;
    let mut ok: bool = false;
    let mut oldval = *p;
    ok = cf_set_int(cv, val);
    if ok as ::core::ffi::c_int != 0
        && (oldval != 0) as ::core::ffi::c_int != (*p != 0) as ::core::ffi::c_int
    {
        pooler_tune_accept(*p != 0);
    }
    return ok;
}
#[c2rust::src_loc = "423:1"]
unsafe extern "C" fn set_dbs_dead(mut flag: bool) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut db = ::core::ptr::null_mut::<PgDatabase>();
    item = database_list.head.next;
    while item != &raw mut database_list.head {
        db = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgDatabase;
        if !(*db).admin {
            if !(*db).db_auto {
                (*db).db_dead = flag;
            }
        }
        item = (*item).next;
    }
}
#[c2rust::src_loc = "438:1"]
unsafe extern "C" fn set_peers_dead(mut flag: bool) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut db = ::core::ptr::null_mut::<PgDatabase>();
    item = peer_list.head.next;
    while item != &raw mut peer_list.head {
        db = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgDatabase;
        (*db).db_dead = flag;
        item = (*item).next;
    }
}
#[c2rust::src_loc = "450:1"]
unsafe extern "C" fn requires_auth_file(mut auth_type: ::core::ffi::c_int) -> bool {
    if auth_type == AUTH_TYPE_PAM as ::core::ffi::c_int {
        return false_0 != 0;
    }
    return auth_type >= AUTH_TYPE_TRUST as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "459:1"]
pub unsafe extern "C" fn load_config() -> bool {
    static mut loaded: bool = false_0 != 0;
    let mut load_file_ok: bool = false;
    let mut ok: bool = false;
    let mut q = ::core::ptr::null::<::core::ffi::c_char>();
    any_user_level_timeout_set = false_0 != 0;
    empty_server_check_query = false_0 != 0;
    any_user_level_client_timeout_set = false_0 != 0;
    set_dbs_dead(true_0 != 0);
    set_peers_dead(true_0 != 0);
    load_file_ok = cf_load_file(&raw mut main_config, cf_config_file);
    if load_file_ok {
        if requires_auth_file(cf_auth_type) {
            loader_users_check();
        }
        loaded = true_0 != 0;
        ok = true_0 != 0;
    } else if !loaded {
        let mut _log_ctx = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx,
            b"cannot load config file\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    } else {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_0,
            b"config file loading failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        set_dbs_dead(false_0 != 0);
        ok = false_0 != 0;
    }
    q = cf_server_check_query;
    if strcmpeq(q, b"<empty>\0" as *const u8 as *const ::core::ffi::c_char) {
        empty_server_check_query = true_0 != 0;
    }
    if cf_auth_type == AUTH_TYPE_HBA as ::core::ffi::c_int {
        let mut ident = ::core::ptr::null_mut::<Ident>();
        let mut hba = ::core::ptr::null_mut::<HBA>();
        ident = ident_load_map(cf_auth_ident_file);
        if !ident.is_null() {
            ident_free(parsed_ident);
            parsed_ident = ident;
        }
        hba = hba_load_rules(cf_auth_hba_file, parsed_ident);
        if !hba.is_null() {
            hba_free(parsed_hba);
            parsed_hba = hba;
        }
    } else {
        hba_free(parsed_hba);
        parsed_hba = ::core::ptr::null_mut::<HBA>();
    }
    config_postprocess();
    if main_config.loaded {
        reset_logging();
    }
    return ok;
}
#[c2rust::src_loc = "532:1"]
static mut ev_sigterm: event = event {
    ev_evcallback: event_callback {
        evcb_active_next: C2RustUnnamed_8 {
            tqe_next: ::core::ptr::null::<event_callback>() as *mut event_callback,
            tqe_prev: ::core::ptr::null::<*mut event_callback>() as *mut *mut event_callback,
        },
        evcb_flags: 0,
        evcb_pri: 0,
        evcb_closure: 0,
        evcb_cb_union: C2RustUnnamed_7 {
            evcb_callback: None,
        },
        evcb_arg: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    },
    ev_timeout_pos: C2RustUnnamed_5 {
        ev_next_with_common_timeout: C2RustUnnamed_6 {
            tqe_next: ::core::ptr::null::<event>() as *mut event,
            tqe_prev: ::core::ptr::null::<*mut event>() as *mut *mut event,
        },
    },
    ev_fd: 0,
    ev_base: ::core::ptr::null::<event_base>() as *mut event_base,
    ev_: C2RustUnnamed_0 {
        ev_io: C2RustUnnamed_3 {
            ev_io_next: C2RustUnnamed_4 {
                le_next: ::core::ptr::null::<event>() as *mut event,
                le_prev: ::core::ptr::null::<*mut event>() as *mut *mut event,
            },
            ev_timeout: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        },
    },
    ev_events: 0,
    ev_res: 0,
    ev_timeout: timeval {
        tv_sec: 0,
        tv_usec: 0,
    },
};
#[c2rust::src_loc = "533:1"]
static mut ev_sigint: event = event {
    ev_evcallback: event_callback {
        evcb_active_next: C2RustUnnamed_8 {
            tqe_next: ::core::ptr::null::<event_callback>() as *mut event_callback,
            tqe_prev: ::core::ptr::null::<*mut event_callback>() as *mut *mut event_callback,
        },
        evcb_flags: 0,
        evcb_pri: 0,
        evcb_closure: 0,
        evcb_cb_union: C2RustUnnamed_7 {
            evcb_callback: None,
        },
        evcb_arg: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    },
    ev_timeout_pos: C2RustUnnamed_5 {
        ev_next_with_common_timeout: C2RustUnnamed_6 {
            tqe_next: ::core::ptr::null::<event>() as *mut event,
            tqe_prev: ::core::ptr::null::<*mut event>() as *mut *mut event,
        },
    },
    ev_fd: 0,
    ev_base: ::core::ptr::null::<event_base>() as *mut event_base,
    ev_: C2RustUnnamed_0 {
        ev_io: C2RustUnnamed_3 {
            ev_io_next: C2RustUnnamed_4 {
                le_next: ::core::ptr::null::<event>() as *mut event,
                le_prev: ::core::ptr::null::<*mut event>() as *mut *mut event,
            },
            ev_timeout: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        },
    },
    ev_events: 0,
    ev_res: 0,
    ev_timeout: timeval {
        tv_sec: 0,
        tv_usec: 0,
    },
};
#[c2rust::src_loc = "535:1"]
unsafe extern "C" fn handle_sigterm(
    mut _sock: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut _arg: *mut ::core::ffi::c_void,
) {
    if cf_shutdown != 0 {
        let mut _log_ctx = NULL;
        log_generic(
            LG_INFO,
            _log_ctx,
            b"got SIGTERM while shutting down, fast exit\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        exit(0 as ::core::ffi::c_int);
    }
    let mut _log_ctx_0 = NULL;
    log_generic(
        LG_INFO,
        _log_ctx_0,
        b"got SIGTERM, shutting down, waiting for all clients disconnect\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    if cf_reboot != 0 {
        let mut _log_ctx_1 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_1,
            b"takeover was in progress, going down immediately\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    if cf_pause_mode == P_SUSPEND as ::core::ffi::c_int {
        let mut _log_ctx_2 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_2,
            b"suspend was in progress, going down immediately\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    cf_shutdown = SHUTDOWN_WAIT_FOR_CLIENTS as ::core::ffi::c_int;
    cleanup_tcp_sockets();
}
#[c2rust::src_loc = "552:1"]
unsafe extern "C" fn handle_sigint(
    mut _sock: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut _arg: *mut ::core::ffi::c_void,
) {
    if cf_shutdown != 0 {
        let mut _log_ctx = NULL;
        log_generic(
            LG_INFO,
            _log_ctx,
            b"got SIGINT while shutting down, fast exit\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        exit(0 as ::core::ffi::c_int);
    }
    let mut _log_ctx_0 = NULL;
    log_generic(
        LG_INFO,
        _log_ctx_0,
        b"got SIGINT, shutting down, waiting for all servers connections to be released\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    if cf_reboot != 0 {
        let mut _log_ctx_1 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_1,
            b"takeover was in progress, going down immediately\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    if cf_pause_mode == P_SUSPEND as ::core::ffi::c_int {
        let mut _log_ctx_2 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_2,
            b"suspend was in progress, going down immediately\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    cf_pause_mode = P_PAUSE as ::core::ffi::c_int;
    cf_shutdown = SHUTDOWN_WAIT_FOR_SERVERS as ::core::ffi::c_int;
    cleanup_tcp_sockets();
}
#[c2rust::src_loc = "572:1"]
static mut ev_sigquit: event = event {
    ev_evcallback: event_callback {
        evcb_active_next: C2RustUnnamed_8 {
            tqe_next: ::core::ptr::null::<event_callback>() as *mut event_callback,
            tqe_prev: ::core::ptr::null::<*mut event_callback>() as *mut *mut event_callback,
        },
        evcb_flags: 0,
        evcb_pri: 0,
        evcb_closure: 0,
        evcb_cb_union: C2RustUnnamed_7 {
            evcb_callback: None,
        },
        evcb_arg: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    },
    ev_timeout_pos: C2RustUnnamed_5 {
        ev_next_with_common_timeout: C2RustUnnamed_6 {
            tqe_next: ::core::ptr::null::<event>() as *mut event,
            tqe_prev: ::core::ptr::null::<*mut event>() as *mut *mut event,
        },
    },
    ev_fd: 0,
    ev_base: ::core::ptr::null::<event_base>() as *mut event_base,
    ev_: C2RustUnnamed_0 {
        ev_io: C2RustUnnamed_3 {
            ev_io_next: C2RustUnnamed_4 {
                le_next: ::core::ptr::null::<event>() as *mut event,
                le_prev: ::core::ptr::null::<*mut event>() as *mut *mut event,
            },
            ev_timeout: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        },
    },
    ev_events: 0,
    ev_res: 0,
    ev_timeout: timeval {
        tv_sec: 0,
        tv_usec: 0,
    },
};
#[c2rust::src_loc = "573:1"]
static mut ev_sigusr1: event = event {
    ev_evcallback: event_callback {
        evcb_active_next: C2RustUnnamed_8 {
            tqe_next: ::core::ptr::null::<event_callback>() as *mut event_callback,
            tqe_prev: ::core::ptr::null::<*mut event_callback>() as *mut *mut event_callback,
        },
        evcb_flags: 0,
        evcb_pri: 0,
        evcb_closure: 0,
        evcb_cb_union: C2RustUnnamed_7 {
            evcb_callback: None,
        },
        evcb_arg: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    },
    ev_timeout_pos: C2RustUnnamed_5 {
        ev_next_with_common_timeout: C2RustUnnamed_6 {
            tqe_next: ::core::ptr::null::<event>() as *mut event,
            tqe_prev: ::core::ptr::null::<*mut event>() as *mut *mut event,
        },
    },
    ev_fd: 0,
    ev_base: ::core::ptr::null::<event_base>() as *mut event_base,
    ev_: C2RustUnnamed_0 {
        ev_io: C2RustUnnamed_3 {
            ev_io_next: C2RustUnnamed_4 {
                le_next: ::core::ptr::null::<event>() as *mut event,
                le_prev: ::core::ptr::null::<*mut event>() as *mut *mut event,
            },
            ev_timeout: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        },
    },
    ev_events: 0,
    ev_res: 0,
    ev_timeout: timeval {
        tv_sec: 0,
        tv_usec: 0,
    },
};
#[c2rust::src_loc = "574:1"]
static mut ev_sigusr2: event = event {
    ev_evcallback: event_callback {
        evcb_active_next: C2RustUnnamed_8 {
            tqe_next: ::core::ptr::null::<event_callback>() as *mut event_callback,
            tqe_prev: ::core::ptr::null::<*mut event_callback>() as *mut *mut event_callback,
        },
        evcb_flags: 0,
        evcb_pri: 0,
        evcb_closure: 0,
        evcb_cb_union: C2RustUnnamed_7 {
            evcb_callback: None,
        },
        evcb_arg: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    },
    ev_timeout_pos: C2RustUnnamed_5 {
        ev_next_with_common_timeout: C2RustUnnamed_6 {
            tqe_next: ::core::ptr::null::<event>() as *mut event,
            tqe_prev: ::core::ptr::null::<*mut event>() as *mut *mut event,
        },
    },
    ev_fd: 0,
    ev_base: ::core::ptr::null::<event_base>() as *mut event_base,
    ev_: C2RustUnnamed_0 {
        ev_io: C2RustUnnamed_3 {
            ev_io_next: C2RustUnnamed_4 {
                le_next: ::core::ptr::null::<event>() as *mut event,
                le_prev: ::core::ptr::null::<*mut event>() as *mut *mut event,
            },
            ev_timeout: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        },
    },
    ev_events: 0,
    ev_res: 0,
    ev_timeout: timeval {
        tv_sec: 0,
        tv_usec: 0,
    },
};
#[c2rust::src_loc = "575:1"]
static mut ev_sighup: event = event {
    ev_evcallback: event_callback {
        evcb_active_next: C2RustUnnamed_8 {
            tqe_next: ::core::ptr::null::<event_callback>() as *mut event_callback,
            tqe_prev: ::core::ptr::null::<*mut event_callback>() as *mut *mut event_callback,
        },
        evcb_flags: 0,
        evcb_pri: 0,
        evcb_closure: 0,
        evcb_cb_union: C2RustUnnamed_7 {
            evcb_callback: None,
        },
        evcb_arg: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    },
    ev_timeout_pos: C2RustUnnamed_5 {
        ev_next_with_common_timeout: C2RustUnnamed_6 {
            tqe_next: ::core::ptr::null::<event>() as *mut event,
            tqe_prev: ::core::ptr::null::<*mut event>() as *mut *mut event,
        },
    },
    ev_fd: 0,
    ev_base: ::core::ptr::null::<event_base>() as *mut event_base,
    ev_: C2RustUnnamed_0 {
        ev_io: C2RustUnnamed_3 {
            ev_io_next: C2RustUnnamed_4 {
                le_next: ::core::ptr::null::<event>() as *mut event,
                le_prev: ::core::ptr::null::<*mut event>() as *mut *mut event,
            },
            ev_timeout: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        },
    },
    ev_events: 0,
    ev_res: 0,
    ev_timeout: timeval {
        tv_sec: 0,
        tv_usec: 0,
    },
};
#[c2rust::src_loc = "577:1"]
unsafe extern "C" fn handle_sigquit(
    mut _sock: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut _arg: *mut ::core::ffi::c_void,
) {
    let mut _log_ctx = NULL;
    log_generic(
        LG_INFO,
        _log_ctx,
        b"got SIGQUIT, fast exit\0" as *const u8 as *const ::core::ffi::c_char,
    );
    exit(0 as ::core::ffi::c_int);
}
#[c2rust::src_loc = "584:1"]
unsafe extern "C" fn handle_sigusr1(
    mut _sock: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut _arg: *mut ::core::ffi::c_void,
) {
    if cf_pause_mode == P_NONE as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_generic(
            LG_INFO,
            _log_ctx,
            b"got SIGUSR1, pausing all activity\0" as *const u8 as *const ::core::ffi::c_char,
        );
        cf_pause_mode = P_PAUSE as ::core::ffi::c_int;
    } else {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_INFO,
            _log_ctx_0,
            b"got SIGUSR1, but already paused/suspended\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
}
#[c2rust::src_loc = "594:1"]
unsafe extern "C" fn handle_sigusr2(
    mut _sock: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut _arg: *mut ::core::ffi::c_void,
) {
    if cf_shutdown != 0 {
        let mut _log_ctx = NULL;
        log_generic(
            LG_INFO,
            _log_ctx,
            b"got SIGUSR2 while shutting down, ignoring\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    match cf_pause_mode {
        2 => {
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_INFO,
                _log_ctx_0,
                b"got SIGUSR2, continuing from SUSPEND\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            resume_all();
            cf_pause_mode = P_NONE as ::core::ffi::c_int;
        }
        1 => {
            let mut _log_ctx_1 = NULL;
            log_generic(
                LG_INFO,
                _log_ctx_1,
                b"got SIGUSR2, continuing from PAUSE\0" as *const u8 as *const ::core::ffi::c_char,
            );
            cf_pause_mode = P_NONE as ::core::ffi::c_int;
        }
        0 => {
            let mut _log_ctx_2 = NULL;
            log_generic(
                LG_INFO,
                _log_ctx_2,
                b"got SIGUSR2, but not paused/suspended\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        _ => {}
    };
}
#[c2rust::src_loc = "621:1"]
unsafe extern "C" fn notify_reloading() {}
#[c2rust::src_loc = "632:1"]
unsafe extern "C" fn handle_sighup(
    mut _sock: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut _arg: *mut ::core::ffi::c_void,
) {
    let mut _log_ctx = NULL;
    log_generic(
        LG_INFO,
        _log_ctx,
        b"got SIGHUP, re-reading config\0" as *const u8 as *const ::core::ffi::c_char,
    );
    notify_reloading();
    load_config();
    if !sbuf_tls_setup() {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx_0,
            b"TLS configuration could not be reloaded, keeping old configuration\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
}
#[c2rust::src_loc = "643:1"]
unsafe extern "C" fn signal_setup() {
    let mut err: ::core::ffi::c_int = 0;
    let mut set: sigset_t = 0;
    set = 0 as sigset_t;
    set |= __sigbits(13 as ::core::ffi::c_int) as sigset_t;
    err = sigprocmask(SIG_BLOCK, &raw mut set, ::core::ptr::null_mut::<sigset_t>());
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_fatal(
            b"src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
            655 as ::core::ffi::c_int,
            b"signal_setup\0" as *const u8 as *const ::core::ffi::c_char,
            true_0 != 0,
            _log_ctx,
            b"sigprocmask\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    event_assign(
        &raw mut ev_sigusr1,
        pgb_event_base,
        30 as ::core::ffi::c_int,
        (EV_SIGNAL | EV_PERSIST) as ::core::ffi::c_short,
        Some(
            handle_sigusr1
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_short,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    err = event_add(&raw mut ev_sigusr1, ::core::ptr::null::<timeval>());
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        log_fatal(
            b"src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
            662 as ::core::ffi::c_int,
            b"signal_setup\0" as *const u8 as *const ::core::ffi::c_char,
            true_0 != 0,
            _log_ctx_0,
            b"evsignal_add\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    event_assign(
        &raw mut ev_sigusr2,
        pgb_event_base,
        31 as ::core::ffi::c_int,
        (EV_SIGNAL | EV_PERSIST) as ::core::ffi::c_short,
        Some(
            handle_sigusr2
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_short,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    err = event_add(&raw mut ev_sigusr2, ::core::ptr::null::<timeval>());
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx_1 = NULL;
        log_fatal(
            b"src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
            667 as ::core::ffi::c_int,
            b"signal_setup\0" as *const u8 as *const ::core::ffi::c_char,
            true_0 != 0,
            _log_ctx_1,
            b"evsignal_add\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    event_assign(
        &raw mut ev_sighup,
        pgb_event_base,
        1 as ::core::ffi::c_int,
        (EV_SIGNAL | EV_PERSIST) as ::core::ffi::c_short,
        Some(
            handle_sighup
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_short,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    err = event_add(&raw mut ev_sighup, ::core::ptr::null::<timeval>());
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx_2 = NULL;
        log_fatal(
            b"src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
            672 as ::core::ffi::c_int,
            b"signal_setup\0" as *const u8 as *const ::core::ffi::c_char,
            true_0 != 0,
            _log_ctx_2,
            b"evsignal_add\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    event_assign(
        &raw mut ev_sigquit,
        pgb_event_base,
        3 as ::core::ffi::c_int,
        (EV_SIGNAL | EV_PERSIST) as ::core::ffi::c_short,
        Some(
            handle_sigquit
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_short,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    err = event_add(&raw mut ev_sigquit, ::core::ptr::null::<timeval>());
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx_3 = NULL;
        log_fatal(
            b"src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
            677 as ::core::ffi::c_int,
            b"signal_setup\0" as *const u8 as *const ::core::ffi::c_char,
            true_0 != 0,
            _log_ctx_3,
            b"evsignal_add\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    event_assign(
        &raw mut ev_sigterm,
        pgb_event_base,
        15 as ::core::ffi::c_int,
        (EV_SIGNAL | EV_PERSIST) as ::core::ffi::c_short,
        Some(
            handle_sigterm
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_short,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    err = event_add(&raw mut ev_sigterm, ::core::ptr::null::<timeval>());
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx_4 = NULL;
        log_fatal(
            b"src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
            682 as ::core::ffi::c_int,
            b"signal_setup\0" as *const u8 as *const ::core::ffi::c_char,
            true_0 != 0,
            _log_ctx_4,
            b"evsignal_add\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    event_assign(
        &raw mut ev_sigint,
        pgb_event_base,
        2 as ::core::ffi::c_int,
        (EV_SIGNAL | EV_PERSIST) as ::core::ffi::c_short,
        Some(
            handle_sigint
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_short,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    err = event_add(&raw mut ev_sigint, ::core::ptr::null::<timeval>());
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx_5 = NULL;
        log_fatal(
            b"src/main.c\0" as *const u8 as *const ::core::ffi::c_char,
            687 as ::core::ffi::c_int,
            b"signal_setup\0" as *const u8 as *const ::core::ffi::c_char,
            true_0 != 0,
            _log_ctx_5,
            b"evsignal_add\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
}
#[c2rust::src_loc = "693:1"]
unsafe extern "C" fn go_daemon() {
    let mut pid: ::core::ffi::c_int = 0;
    let mut fd: ::core::ffi::c_int = 0;
    if cf_pidfile.is_null() || *cf_pidfile.offset(0 as ::core::ffi::c_int as isize) == 0 {
        let mut _log_ctx = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx,
            b"daemon needs pidfile configured\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    cf_quiet = 1 as ::core::ffi::c_int;
    fd = open(
        b"/dev/null\0" as *const u8 as *const ::core::ffi::c_char,
        O_RDWR,
    );
    if fd < 0 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_0,
            b"could not open /dev/null: %s\0" as *const u8 as *const ::core::ffi::c_char,
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    dup2(fd, 0 as ::core::ffi::c_int);
    dup2(fd, 1 as ::core::ffi::c_int);
    dup2(fd, 2 as ::core::ffi::c_int);
    if fd > 2 as ::core::ffi::c_int {
        close(fd);
    }
    pid = fork() as ::core::ffi::c_int;
    if pid < 0 as ::core::ffi::c_int {
        let mut _log_ctx_1 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_1,
            b"fork failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    if pid > 0 as ::core::ffi::c_int {
        _exit(0 as ::core::ffi::c_int);
    }
    pid = setsid() as ::core::ffi::c_int;
    if pid < 0 as ::core::ffi::c_int {
        let mut _log_ctx_2 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_2,
            b"setsid failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    pid = fork() as ::core::ffi::c_int;
    if pid < 0 as ::core::ffi::c_int {
        let mut _log_ctx_3 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_3,
            b"fork failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    if pid > 0 as ::core::ffi::c_int {
        _exit(0 as ::core::ffi::c_int);
    }
}
#[c2rust::src_loc = "741:1"]
unsafe extern "C" fn remove_pidfile() {
    if !cf_pidfile.is_null() {
        if *cf_pidfile.offset(0 as ::core::ffi::c_int as isize) != 0 {
            unlink(cf_pidfile);
        }
        free(cf_pidfile as *mut ::core::ffi::c_void);
        cf_pidfile = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
}
#[c2rust::src_loc = "751:1"]
unsafe extern "C" fn check_pidfile() {
    let mut buf: [::core::ffi::c_char; 129] = [0; 129];
    let mut pid: pid_t = 0 as pid_t;
    let mut fd: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    if cf_pidfile.is_null() || *cf_pidfile.offset(0 as ::core::ffi::c_int as isize) == 0 {
        return;
    }
    fd = open(cf_pidfile, O_RDONLY);
    if fd < 0 as ::core::ffi::c_int {
        if *__error() == ENOENT {
            return;
        }
        let mut _log_ctx = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx,
            b"could not open pidfile '%s': %s\0" as *const u8 as *const ::core::ffi::c_char,
            cf_pidfile,
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    res = read(
        fd,
        &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 129]>() as size_t).wrapping_sub(1 as size_t),
    ) as ::core::ffi::c_int;
    close(fd);
    if res < 0 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_0,
            b"could not read pidfile '%s': %s\0" as *const u8 as *const ::core::ffi::c_char,
            cf_pidfile,
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    if !(res == 0 as ::core::ffi::c_int) {
        buf[res as usize] = 0 as ::core::ffi::c_char;
        pid = atol(&raw mut buf as *mut ::core::ffi::c_char) as pid_t;
        if !(pid <= 0 as pid_t) {
            if !(kill(pid, 0 as ::core::ffi::c_int) >= 0 as ::core::ffi::c_int) {
                if !(*__error() != ESRCH) {
                    let mut _log_ctx_1 = NULL;
                    log_generic(
                        LG_INFO,
                        _log_ctx_1,
                        b"stale pidfile, removing\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    err = unlink(cf_pidfile);
                    if err != 0 as ::core::ffi::c_int {
                        let mut _log_ctx_2 = NULL;
                        log_generic(
                            LG_FATAL,
                            _log_ctx_2,
                            b"could not remove stale pidfile: %s\0" as *const u8
                                as *const ::core::ffi::c_char,
                            strerror(*__error()),
                        );
                        exit(1 as ::core::ffi::c_int);
                    }
                    return;
                }
            }
        }
    }
    let mut _log_ctx_3 = NULL;
    log_generic(
        LG_FATAL,
        _log_ctx_3,
        b"pidfile '%s' exists, another instance running?\0" as *const u8
            as *const ::core::ffi::c_char,
        cf_pidfile,
    );
    exit(1 as ::core::ffi::c_int);
}
#[c2rust::src_loc = "797:1"]
unsafe extern "C" fn write_pidfile() {
    let mut buf: [::core::ffi::c_char; 64] = [0; 64];
    let mut pid: pid_t = 0;
    let mut res: ::core::ffi::c_int = 0;
    let mut fd: ::core::ffi::c_int = 0;
    if cf_pidfile.is_null() || *cf_pidfile.offset(0 as ::core::ffi::c_int as isize) == 0 {
        return;
    }
    pid = getpid();
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
        b"%u\n\0" as *const u8 as *const ::core::ffi::c_char,
        pid as ::core::ffi::c_uint,
    );
    fd = open(
        cf_pidfile,
        O_WRONLY | O_CREAT | O_EXCL,
        0o644 as ::core::ffi::c_int,
    );
    if fd < 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx,
            b"could not open pidfile '%s': %s\0" as *const u8 as *const ::core::ffi::c_char,
            cf_pidfile,
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    res = safe_write(
        fd,
        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(&raw mut buf as *mut ::core::ffi::c_char),
    ) as ::core::ffi::c_int;
    if res < 0 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_0,
            b"could not write pidfile '%s': %s\0" as *const u8 as *const ::core::ffi::c_char,
            cf_pidfile,
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    close(fd);
    atexit(Some(remove_pidfile as unsafe extern "C" fn() -> ()));
}
#[c2rust::src_loc = "822:1"]
unsafe extern "C" fn check_limits() {
    let mut lim = rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };
    let mut total_users = statlist_count(&raw mut user_list);
    let mut fd_count: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    let mut item = ::core::ptr::null_mut::<List>();
    let mut db = ::core::ptr::null_mut::<PgDatabase>();
    let mut _log_ctx = NULL;
    if (cf_verbose > 1 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        log_generic(
            LG_NOISE,
            _log_ctx,
            b"event: %d, SBuf: %d, PgSocket: %d, IOBuf: %d\0" as *const u8
                as *const ::core::ffi::c_char,
            ::core::mem::size_of::<event>() as ::core::ffi::c_int,
            ::core::mem::size_of::<SBuf>() as ::core::ffi::c_int,
            ::core::mem::size_of::<PgSocket>() as ::core::ffi::c_int,
            (12 as ::core::ffi::c_ulong).wrapping_add(cf_sbuf_len as ::core::ffi::c_ulong)
                as ::core::ffi::c_int,
        );
    }
    err = getrlimit(RLIMIT_NOFILE, &raw mut lim);
    if err < 0 as ::core::ffi::c_int {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_ERROR,
            _log_ctx_0,
            b"could not get RLIMIT_NOFILE: %s\0" as *const u8 as *const ::core::ffi::c_char,
            strerror(*__error()),
        );
        return;
    }
    fd_count = cf_max_client_conn + 10 as ::core::ffi::c_int;
    item = database_list.head.next;
    while item != &raw mut database_list.head {
        db = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgDatabase;
        if !(*db).forced_user_credentials.is_null() {
            fd_count += if (*db).pool_size >= 0 as ::core::ffi::c_int {
                (*db).pool_size
            } else {
                cf_default_pool_size
            };
        } else {
            fd_count += (if (*db).pool_size >= 0 as ::core::ffi::c_int {
                (*db).pool_size
            } else {
                cf_default_pool_size
            }) * total_users;
        }
        item = (*item).next;
    }
    let mut _log_ctx_1 = NULL;
    log_generic(
        LG_INFO,
        _log_ctx_1,
        b"kernel file descriptor limit: %d (hard: %d); max_client_conn: %d, max expected fd use: %d\0"
            as *const u8 as *const ::core::ffi::c_char,
        lim.rlim_cur as ::core::ffi::c_int,
        lim.rlim_max as ::core::ffi::c_int,
        cf_max_client_conn,
        fd_count,
    );
}
#[c2rust::src_loc = "856:1"]
unsafe extern "C" fn check_old_process_unix() -> bool {
    let mut sa_un = sockaddr_un {
        sun_len: 0,
        sun_family: 0,
        sun_path: [0; 104],
    };
    let mut len: socklen_t = ::core::mem::size_of::<sockaddr_un>() as socklen_t;
    let mut domain = AF_UNIX;
    let mut res: ::core::ffi::c_int = 0;
    let mut fd: ::core::ffi::c_int = 0;
    if cf_unix_socket_dir.is_null()
        || *cf_unix_socket_dir == 0
        || 0 as ::core::ffi::c_int > 0 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    memset(
        &raw mut sa_un as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        len as size_t,
    );
    sa_un.sun_family = domain as sa_family_t;
    snprintf(
        &raw mut sa_un.sun_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 104]>() as size_t,
        b"%s/.s.PGSQL.%d\0" as *const u8 as *const ::core::ffi::c_char,
        cf_unix_socket_dir,
        cf_listen_port,
    );
    fd = socket(domain, SOCK_STREAM, 0 as ::core::ffi::c_int);
    if fd < 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx,
            b"could not create socket: %s\0" as *const u8 as *const ::core::ffi::c_char,
            strerror(*__error()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    res = safe_connect(fd, &raw mut sa_un as *mut sockaddr, len);
    safe_close(fd);
    if res < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[c2rust::src_loc = "881:1"]
unsafe extern "C" fn main_loop_once() {
    let mut err: ::core::ffi::c_int = 0;
    reset_time_cache();
    err = event_base_loop(pgb_event_base, EVLOOP_ONCE);
    if err < 0 as ::core::ffi::c_int {
        if *__error() != EINTR {
            let mut _log_ctx = NULL;
            log_generic(
                LG_WARNING,
                _log_ctx,
                b"event_loop failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
                strerror(*__error()),
            );
        }
    }
    ldap_poll();
    pam_poll();
    per_loop_maint();
    reuse_just_freed_objects();
    rescue_timers();
    per_loop_pooler_maint();
    if !adns.is_null() {
        adns_per_loop(adns);
    }
}
#[c2rust::src_loc = "903:1"]
unsafe extern "C" fn takeover_part1() {
    let mut evtmp = ::core::ptr::null_mut::<event_base>();
    evtmp = pgb_event_base;
    pgb_event_base = event_base_new();
    if cf_unix_socket_dir.is_null() || *cf_unix_socket_dir == 0 {
        let mut _log_ctx = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx,
            b"cannot reboot if unix dir not configured\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    if *cf_unix_socket_dir.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '@' as i32
    {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_0,
            b"cannot reboot with abstract Unix socket\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    if 0 as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        let mut _log_ctx_1 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_1,
            b"cannot reboot under service manager\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    takeover_init();
    while cf_reboot != 0 {
        main_loop_once();
    }
    event_base_free(pgb_event_base);
    pgb_event_base = evtmp;
}
#[c2rust::src_loc = "933:1"]
unsafe extern "C" fn dns_setup() {
    if !adns.is_null() {
        return;
    }
    adns = adns_create_context();
    if adns.is_null() {
        let mut _log_ctx = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx,
            b"dns setup failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
}
#[c2rust::src_loc = "1024:1"]
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    let mut did_takeover = false_0 != 0;
    let mut arg_username = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut long_idx: ::core::ffi::c_int = 0;
    static mut long_options: [option; 8] = [
        option {
            name: b"quiet\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'q' as i32,
        },
        option {
            name: b"verbose\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'v' as i32,
        },
        option {
            name: b"help\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'h' as i32,
        },
        option {
            name: b"daemon\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'd' as i32,
        },
        option {
            name: b"version\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'V' as i32,
        },
        option {
            name: b"reboot\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'R' as i32,
        },
        option {
            name: b"user\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 'u' as i32,
        },
        option {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
    ];
    setprogname(usual_basename(
        *argv.offset(0 as ::core::ffi::c_int as isize),
    ));
    loop {
        c = getopt_long(
            argc,
            argv as *const *mut ::core::ffi::c_char,
            b"qvhdVRu:\0" as *const u8 as *const ::core::ffi::c_char,
            &raw const long_options as *const option,
            &raw mut long_idx,
        );
        if !(c != -(1 as ::core::ffi::c_int)) {
            break;
        }
        match c {
            82 => {
                cf_reboot = 1 as ::core::ffi::c_int;
            }
            118 => {
                cf_verbose += 1;
            }
            86 => {
                printf(
                    b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    PACKAGE_STRING.as_ptr(),
                );
                printf(
                    b"libevent %s\nadns: %s\ntls: %s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    event_get_version(),
                    adns_get_backend(),
                    tls_backend_version(),
                );
                return 0 as ::core::ffi::c_int;
            }
            100 => {
                cf_daemon = 1 as ::core::ffi::c_int;
            }
            113 => {
                cf_quiet = 1 as ::core::ffi::c_int;
            }
            117 => {
                arg_username = optarg;
            }
            104 => {
                usage(*argv.offset(0 as ::core::ffi::c_int as isize));
            }
            _ => {
                fprintf(
                    __stderrp,
                    b"Try \"%s --help\" for more information.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                );
                exit(1 as ::core::ffi::c_int);
            }
        }
    }
    if optind + 1 as ::core::ffi::c_int != argc {
        fprintf(
            __stderrp,
            b"%s: no configuration file specified\n\0" as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        fprintf(
            __stderrp,
            b"Try \"%s --help\" for more information.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        exit(1 as ::core::ffi::c_int);
    }
    cf_config_file = xstrdup(*argv.offset(optind as isize));
    init_objects();
    load_config();
    main_config.loaded = true_0 != 0;
    init_var_lookup(cf_track_extra_parameters);
    init_caches();
    logging_prefix_cb = Some(
        log_socket_prefix
            as unsafe extern "C" fn(
                LogLevel,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_char,
                ::core::ffi::c_uint,
            ) -> ::core::ffi::c_int,
    ) as logging_prefix_fn_t;
    if !sbuf_tls_setup() {
        let mut _log_ctx = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx,
            b"TLS setup failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    if !arg_username.is_null() {
        free(global_username as *mut ::core::ffi::c_void);
        global_username = xstrdup(arg_username);
    }
    if !global_username.is_null() && *global_username as ::core::ffi::c_int != 0 {
        change_user(global_username);
    }
    if getuid() == 0 as uid_t {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_0,
            b"PgBouncer should not run as root\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    admin_setup();
    if cf_reboot != 0 {
        let mut _log_ctx_1 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_1,
            b"Online restart is deprecated, use so_reuseport instead\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        if check_old_process_unix() {
            takeover_part1();
            did_takeover = true_0 != 0;
        } else {
            let mut _log_ctx_2 = NULL;
            log_generic(
                LG_INFO,
                _log_ctx_2,
                b"old process not found, try to continue normally\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            cf_reboot = 0 as ::core::ffi::c_int;
            check_pidfile();
        }
    } else {
        if check_old_process_unix() {
            let mut _log_ctx_3 = NULL;
            log_generic(
                LG_FATAL,
                _log_ctx_3,
                b"unix socket is in use, cannot continue\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            exit(1 as ::core::ffi::c_int);
        }
        check_pidfile();
    }
    if cf_daemon != 0 {
        go_daemon();
    }
    if !getenv(b"NOTIFY_SOCKET\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        let mut _log_ctx_4 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_4,
            b"apparently running under systemd with notify socket, but systemd support was not built\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    check_limits();
    srandom((time(::core::ptr::null_mut::<time_t>()) ^ getpid() as time_t) as ::core::ffi::c_uint);
    pgb_event_base = event_base_new();
    if pgb_event_base.is_null() {
        let mut _log_ctx_5 = NULL;
        log_generic(
            LG_FATAL,
            _log_ctx_5,
            b"event_base_new() failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    dns_setup();
    signal_setup();
    janitor_setup();
    stats_setup();
    auth_ldap_init();
    pam_init();
    if did_takeover {
        takeover_finish();
    } else {
        pooler_setup();
    }
    write_pidfile();
    let mut _log_ctx_6 = NULL;
    log_generic(
        LG_INFO,
        _log_ctx_6,
        b"process up: %s, libevent %s (%s), adns: %s, tls: %s\0" as *const u8
            as *const ::core::ffi::c_char,
        b"PgBouncer 1.25.1\0" as *const u8 as *const ::core::ffi::c_char,
        event_get_version(),
        event_base_get_method(pgb_event_base),
        adns_get_backend(),
        tls_backend_version(),
    );
    while cf_shutdown != SHUTDOWN_IMMEDIATE as ::core::ffi::c_int {
        main_loop_once();
    }
    return 0 as ::core::ffi::c_int;
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}
unsafe extern "C" fn run_static_initializers() {
    bouncer_params = [
        CfKey {
            key_name: b"admin_users\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_admin_users as uintptr_t,
            def_value: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"application_name_add_host\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_application_name_add_host as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"auth_dbname\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_authdb
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_auth_dbname as uintptr_t,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        CfKey {
            key_name: b"auth_file\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_auth_file as uintptr_t,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        CfKey {
            key_name: b"auth_hba_file\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_auth_hba_file as uintptr_t,
            def_value: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"auth_ident_file\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_auth_ident_file as uintptr_t,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        CfKey {
            key_name: b"auth_ldap_options\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_auth_ldap_options as uintptr_t,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        CfKey {
            key_name: b"auth_query\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_auth_query as uintptr_t,
            def_value: b"SELECT rolname, CASE WHEN rolvaliduntil < now() THEN NULL ELSE rolpassword END FROM pg_authid WHERE rolname=$1 AND rolcanlogin\0"
                as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"auth_type\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_lookup
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_lookup
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: &raw const auth_type_map as *const CfLookup
                    as *const ::core::ffi::c_void,
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_auth_type as uintptr_t,
            def_value: b"md5\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"auth_user\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_auth_user as uintptr_t,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        CfKey {
            key_name: b"autodb_idle_timeout\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_autodb_idle_timeout as uintptr_t,
            def_value: b"3600\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"cancel_wait_timeout\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_cancel_wait_timeout as uintptr_t,
            def_value: b"10\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"client_idle_timeout\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_idle_timeout as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"client_login_timeout\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_login_timeout as uintptr_t,
            def_value: b"60\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"client_tls13_ciphers\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_tls13_ciphers as uintptr_t,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        CfKey {
            key_name: b"client_tls_ca_file\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_tls_ca_file as uintptr_t,
            def_value: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"client_tls_cert_file\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_tls_cert_file as uintptr_t,
            def_value: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"client_tls_ciphers\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_tls_ciphers as uintptr_t,
            def_value: b"default\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"client_tls_dheparams\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_tls_dheparams as uintptr_t,
            def_value: b"auto\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"client_tls_ecdhcurve\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_tls_ecdhecurve as uintptr_t,
            def_value: b"auto\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"client_tls_key_file\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_tls_key_file as uintptr_t,
            def_value: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"client_tls_protocols\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_tls_protocols as uintptr_t,
            def_value: b"secure\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"client_tls_sslmode\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_lookup
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_lookup
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: &raw const sslmode_map as *const CfLookup
                    as *const ::core::ffi::c_void,
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_client_tls_sslmode as uintptr_t,
            def_value: b"disable\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"conffile\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_config_file as uintptr_t,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        CfKey {
            key_name: b"default_pool_size\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_default_pool_size as uintptr_t,
            def_value: b"20\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"disable_pqexec\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_disable_pqexec as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"dns_max_ttl\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_dns_max_ttl as uintptr_t,
            def_value: b"15\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"dns_nxdomain_ttl\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_dns_nxdomain_ttl as uintptr_t,
            def_value: b"15\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"dns_zone_check_period\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_dns_zone_check_period as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"idle_transaction_timeout\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_idle_transaction_timeout as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"ignore_startup_parameters\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_ignore_startup_params as uintptr_t,
            def_value: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"job_name\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_jobname as uintptr_t,
            def_value: b"pgbouncer\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"listen_addr\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_listen_addr as uintptr_t,
            def_value: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"listen_backlog\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_listen_backlog as uintptr_t,
            def_value: b"128\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"listen_port\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_listen_port as uintptr_t,
            def_value: b"6432\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"log_connections\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_log_connections as uintptr_t,
            def_value: b"1\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"log_disconnections\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_log_disconnections as uintptr_t,
            def_value: b"1\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"log_pooler_errors\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_log_pooler_errors as uintptr_t,
            def_value: b"1\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"log_stats\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_log_stats as uintptr_t,
            def_value: b"1\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"logfile\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_logfile as uintptr_t,
            def_value: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"max_client_conn\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_max_client_conn as uintptr_t,
            def_value: b"100\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"max_db_client_connections\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_max_db_client_connections as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"max_db_connections\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_max_db_connections as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"max_packet_size\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_uint
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_uint
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_max_packet_size as uintptr_t,
            def_value: b"2147483647\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"max_prepared_statements\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_max_prepared_statements as uintptr_t,
            def_value: b"200\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"max_user_client_connections\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_max_user_client_connections as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"max_user_connections\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_max_user_connections as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"min_pool_size\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_min_pool_size as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"peer_id\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_peer_id as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"pidfile\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_pidfile as uintptr_t,
            def_value: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"pkt_buf\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_sbuf_len as uintptr_t,
            def_value: b"4096\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"pool_mode\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_lookup
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_lookup
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: &raw const pool_mode_map as *const CfLookup
                    as *const ::core::ffi::c_void,
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_pool_mode as uintptr_t,
            def_value: b"session\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"query_timeout\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_query_timeout as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"query_wait_notify\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_query_wait_notify as uintptr_t,
            def_value: b"5\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"query_wait_timeout\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_query_wait_timeout as uintptr_t,
            def_value: b"120\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"reserve_pool_size\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_res_pool_size as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"reserve_pool_timeout\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_res_pool_timeout as uintptr_t,
            def_value: b"5\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"resolv_conf\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_resolv_conf as uintptr_t,
            def_value: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"sbuf_loopcnt\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_sbuf_loopcnt as uintptr_t,
            def_value: b"5\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"scram_iterations\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_scram_iterations as uintptr_t,
            def_value: b"4096\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"server_check_delay\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_check_delay as uintptr_t,
            def_value: b"30\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"server_check_query\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_check_query as uintptr_t,
            def_value: b"<empty>\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"server_connect_timeout\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_connect_timeout as uintptr_t,
            def_value: b"15\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"server_fast_close\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_fast_close as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"server_idle_timeout\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_idle_timeout as uintptr_t,
            def_value: b"600\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"server_lifetime\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_lifetime as uintptr_t,
            def_value: b"3600\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"server_login_retry\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_login_retry as uintptr_t,
            def_value: b"15\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"server_reset_query\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_reset_query as uintptr_t,
            def_value: b"DISCARD ALL\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"server_reset_query_always\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_reset_query_always as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"server_round_robin\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_round_robin as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"server_tls13_ciphers\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_tls13_ciphers as uintptr_t,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        CfKey {
            key_name: b"server_tls_ca_file\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_tls_ca_file as uintptr_t,
            def_value: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"server_tls_cert_file\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_tls_cert_file as uintptr_t,
            def_value: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"server_tls_ciphers\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_tls_ciphers as uintptr_t,
            def_value: b"default\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"server_tls_key_file\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_tls_key_file as uintptr_t,
            def_value: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"server_tls_protocols\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_tls_protocols as uintptr_t,
            def_value: b"secure\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"server_tls_sslmode\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_lookup
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_lookup
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: &raw const sslmode_map as *const CfLookup
                    as *const ::core::ffi::c_void,
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_server_tls_sslmode as uintptr_t,
            def_value: b"prefer\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"so_reuseport\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_so_reuseport as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"stats_period\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_stats_period as uintptr_t,
            def_value: b"60\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"stats_users\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_stats_users as uintptr_t,
            def_value: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"suspend_timeout\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_suspend_timeout as uintptr_t,
            def_value: b"10\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"syslog\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_syslog as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"syslog_facility\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_syslog_facility as uintptr_t,
            def_value: b"daemon\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"syslog_ident\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_syslog_ident as uintptr_t,
            def_value: b"pgbouncer\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"tcp_defer_accept\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    set_defer_accept
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_tcp_defer_accept as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"tcp_keepalive\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_tcp_keepalive as uintptr_t,
            def_value: b"1\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"tcp_keepcnt\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_tcp_keepcnt as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"tcp_keepidle\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_tcp_keepidle as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"tcp_keepintvl\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_tcp_keepintvl as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"tcp_socket_buffer\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_tcp_socket_buffer as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"tcp_user_timeout\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_tcp_user_timeout as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"track_extra_parameters\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_track_extra_parameters as uintptr_t,
            def_value: b"IntervalStyle\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"transaction_timeout\0" as *const u8
                as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_time_usec
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_transaction_timeout as uintptr_t,
            def_value: b"0\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"unix_socket_dir\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_unix_socket_dir as uintptr_t,
            def_value: b"/tmp\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"unix_socket_group\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_unix_socket_group as uintptr_t,
            def_value: b"\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"unix_socket_mode\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_unix_socket_mode as uintptr_t,
            def_value: b"0777\0" as *const u8 as *const ::core::ffi::c_char,
        },
        CfKey {
            key_name: b"user\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_str
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut global_username as uintptr_t,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        CfKey {
            key_name: b"verbose\0" as *const u8 as *const ::core::ffi::c_char,
            op: CfOps {
                setter: Some(
                    cf_set_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                            *const ::core::ffi::c_char,
                        ) -> bool,
                ),
                getter: Some(
                    cf_get_int
                        as unsafe extern "C" fn(
                            *mut CfValue,
                        ) -> *const ::core::ffi::c_char,
                ),
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0 as ::core::ffi::c_int | CF_VAL_ABS,
            key_ofs: &raw mut cf_verbose as uintptr_t,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        CfKey {
            key_name: ::core::ptr::null::<::core::ffi::c_char>(),
            op: CfOps {
                setter: None,
                getter: None,
                op_extra: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            flags: 0,
            key_ofs: 0,
            def_value: ::core::ptr::null::<::core::ffi::c_char>(),
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
