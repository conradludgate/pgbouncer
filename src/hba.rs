#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:19"]
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
    #[c2rust::src_loc = "61:1"]
    pub type __darwin_ct_rune_t = ::core::ffi::c_int;
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
    #[c2rust::src_loc = "103:1"]
    pub type __darwin_wchar_t = ::libc::wchar_t;
    #[c2rust::src_loc = "108:1"]
    pub type __darwin_rune_t = __darwin_wchar_t;
    #[c2rust::src_loc = "117:1"]
    pub type __darwin_socklen_t = __uint32_t;
    #[c2rust::src_loc = "118:1"]
    pub type __darwin_ssize_t = isize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:19"]
pub mod sys__types_h {
    #[c2rust::src_loc = "83:1"]
    pub type __darwin_off_t = __int64_t;
    #[c2rust::src_loc = "84:1"]
    pub type __darwin_pid_t = __int32_t;
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_uid_t = __uint32_t;
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    use super::_types_h::{__int32_t, __int64_t, __uint32_t};
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_in_addr_t.h:19"]
pub mod _in_addr_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type in_addr_t = __uint32_t;
    use super::_types_h::__uint32_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_in_port_t.h:19"]
pub mod _in_port_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type in_port_t = __uint16_t;
    use super::_types_h::__uint16_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_pid_t.h:19"]
pub mod _pid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pid_t = __darwin_pid_t;
    use super::sys__types_h::__darwin_pid_t;
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ssize_t.h:19"]
pub mod _ssize_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:19"]
pub mod _uint8_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint8_t = u8;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:19"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = u32;
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
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    use super::sys__types_h::__darwin_off_t;
    extern "C" {
        #[c2rust::src_loc = "103:1"]
        pub type __sFILEX;
        #[c2rust::src_loc = "233:1"]
        pub fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "243:1"]
        pub fn fopen(
            __filename: *const ::core::ffi::c_char,
            __mode: *const ::core::ffi::c_char,
        ) -> *mut FILE;
        #[c2rust::src_loc = "456:1"]
        pub fn getline(
            __linep: *mut *mut ::core::ffi::c_char,
            __linecapp: *mut size_t,
            __stream: *mut FILE,
        ) -> ssize_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/runetype.h:19"]
pub mod runetype_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:9"]
    pub struct _RuneEntry {
        pub __min: __darwin_rune_t,
        pub __max: __darwin_rune_t,
        pub __map: __darwin_rune_t,
        pub __types: *mut __uint32_t,
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
    #[c2rust::src_loc = "75:9"]
    pub struct _RuneCharClass {
        pub __name: [::core::ffi::c_char; 14],
        pub __mask: __uint32_t,
    }
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
    use super::_types_h::{__darwin_rune_t, __darwin_size_t, __uint32_t};
    extern "C" {
        #[c2rust::src_loc = "114:1"]
        pub static mut _DefaultRuneLocale: _RuneLocale;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/list.h:19"]
pub mod list_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "35:1"]
    pub struct List {
        pub next: *mut List,
        pub prev: *mut List,
    }
    #[inline]
    #[c2rust::src_loc = "46:1"]
    pub unsafe extern "C" fn list_init(mut list: *mut List) {
        (*list).prev = list;
        (*list).next = (*list).prev;
    }
    #[inline]
    #[c2rust::src_loc = "68:1"]
    pub unsafe extern "C" fn list_append(mut list: *mut List, mut item: *mut List) -> *mut List {
        (*item).next = list;
        (*item).prev = (*list).prev;
        (*(*list).prev).next = item;
        (*list).prev = item;
        item
    }
    #[inline]
    #[c2rust::src_loc = "78:1"]
    pub unsafe extern "C" fn list_del(mut item: *mut List) -> *mut List {
        (*(*item).prev).next = (*item).next;
        (*(*item).next).prev = (*item).prev;
        (*item).prev = item;
        (*item).next = (*item).prev;
        item
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_sa_family_t.h:19"]
pub mod _sa_family_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type sa_family_t = __uint8_t;
    use super::_types_h::__uint8_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_socklen_t.h:19"]
pub mod _socklen_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type socklen_t = __darwin_socklen_t;
    use super::_types_h::__darwin_socklen_t;
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
    #[c2rust::src_loc = "361:9"]
    pub const AF_UNIX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "365:9"]
    pub const AF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "396:9"]
    pub const AF_INET6: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
    use super::_sa_family_t_h::sa_family_t;
    use super::_types_h::__uint8_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet/in.h:19"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet6/in6.h:19"]
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
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/bouncer.h:19"]
pub mod bouncer_h {
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
    #[c2rust::src_loc = "239:2"]
    pub const AUTH_TYPE_LDAP: auth_type = 6;
    #[c2rust::src_loc = "241:2"]
    pub const AUTH_TYPE_SCRAM_SHA_256: auth_type = 8;
    #[c2rust::src_loc = "237:2"]
    pub const AUTH_TYPE_CERT: auth_type = 4;
    #[c2rust::src_loc = "242:2"]
    pub const AUTH_TYPE_PEER: auth_type = 9;
    #[c2rust::src_loc = "235:2"]
    pub const AUTH_TYPE_PLAIN: auth_type = 2;
    #[c2rust::src_loc = "236:2"]
    pub const AUTH_TYPE_MD5: auth_type = 3;
    #[c2rust::src_loc = "243:2"]
    pub const AUTH_TYPE_REJECT: auth_type = 10;
    #[c2rust::src_loc = "234:2"]
    pub const AUTH_TYPE_TRUST: auth_type = 1;
    #[c2rust::src_loc = "232:1"]
    pub type auth_type = ::core::ffi::c_uint;
    #[c2rust::src_loc = "240:2"]
    pub const AUTH_TYPE_PAM: auth_type = 7;
    #[c2rust::src_loc = "238:2"]
    pub const AUTH_TYPE_HBA: auth_type = 5;
    #[c2rust::src_loc = "233:2"]
    pub const AUTH_TYPE_ANY: auth_type = 0;
    #[inline]
    #[c2rust::src_loc = "299:1"]
    pub unsafe extern "C" fn pga_family(mut a: *const PgAddr) -> ::core::ffi::c_uint {
        (*a).sa.sa_family as ::core::ffi::c_uint
    }
    #[inline]
    #[c2rust::src_loc = "303:1"]
    pub unsafe extern "C" fn pga_is_unix(mut a: *const PgAddr) -> bool {
        (*a).sa.sa_family as ::core::ffi::c_int == AF_UNIX
    }
    use super::_pid_t_h::pid_t;

    use super::_uid_t_h::uid_t;
    use super::in6_h::sockaddr_in6;
    use super::in_h::sockaddr_in;
    use super::socket_h::{sockaddr, AF_UNIX};
    extern "C" {}
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
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/cxalloc.h:19"]
pub mod cxalloc_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "53:1"]
    pub struct CxOps {
        pub c_alloc: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
        >,
        pub c_realloc: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                size_t,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub c_free:
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>,
        pub c_destroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "79:1"]
    pub struct CxMem {
        pub ops: *const CxOps,
        pub ctx: *mut ::core::ffi::c_void,
    }
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "96:1"]
        pub fn cx_alloc(cx: *const CxMem, len: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "118:1"]
        pub fn cx_destroy(cx: *const CxMem);
        #[c2rust::src_loc = "121:1"]
        pub fn cx_alloc0(cx: *const CxMem, len: size_t) -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/hba.h:19"]
pub mod hba_h {
    #[c2rust::src_loc = "25:1"]
    pub type RuleType = ::core::ffi::c_uint;
    #[c2rust::src_loc = "29:2"]
    pub const RULE_HOSTNOSSL: RuleType = 3;
    #[c2rust::src_loc = "28:2"]
    pub const RULE_HOSTSSL: RuleType = 2;
    #[c2rust::src_loc = "27:2"]
    pub const RULE_HOST: RuleType = 1;
    #[c2rust::src_loc = "26:2"]
    pub const RULE_LOCAL: RuleType = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "32:1"]
    pub struct HBAAddress {
        pub flags: ::core::ffi::c_uint,
        pub family: ::core::ffi::c_int,
        pub addr: [uint8_t; 16],
        pub mask: [uint8_t; 16],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:1"]
    pub struct HBAName {
        pub flags: ::core::ffi::c_uint,
        pub name_set: *mut StrSet,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "44:1"]
    pub struct HBARule {
        pub node: List,
        pub rule_type: RuleType,
        pub rule_method: ::core::ffi::c_int,
        pub address: HBAAddress,
        pub db_name: HBAName,
        pub user_name: HBAName,
        pub identmap: *mut IdentMap,
        pub hba_linenr: ::core::ffi::c_int,
        pub auth_options: *mut ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "67:1"]
    pub struct IdentMap {
        pub node: List,
        pub map_name: *mut ::core::ffi::c_char,
        pub mappings: List,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "56:1"]
    pub struct HBA {
        pub rules: List,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:1"]
    pub struct Mapping {
        pub node: List,
        pub system_user_name: *mut ::core::ffi::c_char,
        pub postgres_user_name: *mut ::core::ffi::c_char,
        pub name_flags: ::core::ffi::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "73:1"]
    pub struct Ident {
        pub maps: List,
    }
    #[c2rust::src_loc = "19:9"]
    pub const ADDRESS_ALL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "21:9"]
    pub const NAME_ALL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "22:9"]
    pub const NAME_SAMEUSER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "23:9"]
    pub const NAME_REPLICATION: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    use super::StrSet;
    use super::_uint8_t_h::uint8_t;
    use super::list_h::List;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/cbtree.h:22"]
pub mod cbtree_h {
    #[c2rust::src_loc = "29:1"]
    pub type cbtree_walker_func =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> bool>;
    #[c2rust::src_loc = "27:1"]
    pub type cbtree_getkey_func = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut *const ::core::ffi::c_void,
        ) -> size_t,
    >;
    use super::_size_t_h::size_t;
    use super::cxalloc_h::CxMem;
    extern "C" {
        #[c2rust::src_loc = "32:1"]
        pub type CBTree;
        #[c2rust::src_loc = "42:1"]
        pub fn cbtree_create(
            obj_key_cb: cbtree_getkey_func,
            obj_free_cb: cbtree_walker_func,
            cb_ctx: *mut ::core::ffi::c_void,
            cx: *const CxMem,
        ) -> *mut CBTree;
        #[c2rust::src_loc = "53:1"]
        pub fn cbtree_insert(tree: *mut CBTree, obj: *mut ::core::ffi::c_void) -> bool;
        #[c2rust::src_loc = "67:1"]
        pub fn cbtree_lookup(
            tree: *mut CBTree,
            key: *const ::core::ffi::c_void,
            klen: size_t,
        ) -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:19"]
pub mod _malloc_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "54:1"]
        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "55:1"]
        pub fn calloc(__count: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "56:1"]
        pub fn free(_: *mut ::core::ffi::c_void);
        #[c2rust::src_loc = "57:1"]
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t)
            -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_ctype.h:19"]
pub mod _ctype_h {
    #[c2rust::src_loc = "82:9"]
    pub const _CTYPE_S: ::core::ffi::c_long = 0x4000 as ::core::ffi::c_long;
    #[inline]
    #[c2rust::src_loc = "139:1"]
    pub unsafe extern "C" fn isascii(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        (_c & !(0x7f as ::core::ffi::c_int) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
    }
    #[inline]
    #[c2rust::src_loc = "157:1"]
    pub unsafe extern "C" fn __istype(
        mut _c: __darwin_ct_rune_t,
        mut _f: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int {
        if isascii(_c as ::core::ffi::c_int) != 0 {
            (_DefaultRuneLocale.__runetype[_c as usize] as ::core::ffi::c_ulong & _f != 0)
                as ::core::ffi::c_int
        } else {
            (__maskrune(_c, _f) != 0) as ::core::ffi::c_int
        }
    }
    #[inline]
    #[c2rust::src_loc = "271:1"]
    pub unsafe extern "C" fn isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        __istype(_c as __darwin_ct_rune_t, _CTYPE_S as ::core::ffi::c_ulong)
    }
    use super::_types_h::__darwin_ct_rune_t;
    use super::runetype_h::_DefaultRuneLocale;
    extern "C" {
        #[c2rust::src_loc = "153:1"]
        pub fn __maskrune(_: __darwin_ct_rune_t, _: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/ctype.h:19"]
pub mod ctype_h {
    #[inline]
    #[c2rust::src_loc = "105:1"]
    pub unsafe extern "C" fn safe_isspace(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        isspace(c as ::core::ffi::c_uchar as ::core::ffi::c_int)
    }
    use super::_ctype_h::isspace;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:19"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arpa/inet.h:19"]
pub mod inet_h {
    use super::_socklen_t_h::socklen_t;
    extern "C" {
        #[c2rust::src_loc = "80:1"]
        pub fn inet_ntop(
            _: ::core::ffi::c_int,
            _: *const ::core::ffi::c_void,
            _: *mut ::core::ffi::c_char,
            __size: socklen_t,
        ) -> *const ::core::ffi::c_char;
        #[c2rust::src_loc = "81:1"]
        pub fn inet_pton(
            _: ::core::ffi::c_int,
            _: *const ::core::ffi::c_char,
            _: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:19"]
pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "75:1"]
        pub fn memcmp(
            __s1: *const ::core::ffi::c_void,
            __s2: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> ::core::ffi::c_int;
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
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/cxextra.h:21"]
pub mod cxextra_h {
    use super::_size_t_h::size_t;
    use super::cxalloc_h::CxMem;
    extern "C" {
        #[c2rust::src_loc = "40:1"]
        pub fn cx_new_pool(
            parent: *const CxMem,
            initial_size: size_t,
            align: ::core::ffi::c_uint,
        ) -> *const CxMem;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdlib.h:19"]
pub mod _stdlib_h {
    extern "C" {
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
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/string.h:19"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "146:1"]
        pub fn usual_dirname(path: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    }
}
pub use self::_ctype_h::{__istype, __maskrune, isascii, isspace, _CTYPE_S};
pub use self::_in_addr_t_h::in_addr_t;
pub use self::_in_port_t_h::in_port_t;
use self::_malloc_h::{calloc, free, malloc, realloc};
pub use self::_null_h::NULL;
pub use self::_pid_t_h::pid_t;
pub use self::_sa_family_t_h::sa_family_t;
pub use self::_size_t_h::size_t;
pub use self::_socklen_t_h::socklen_t;
pub use self::_ssize_t_h::ssize_t;
pub use self::_stdio_h::{__sFILE, __sFILEX, __sbuf, fclose, fopen, fpos_t, getline, FILE};
use self::_stdlib_h::strtoul;
use self::_string_h::{memcmp, memcpy, memset, strchr, strcmp, strdup, strerror, strlen, strncmp};
pub use self::_types_h::{
    __darwin_ct_rune_t, __darwin_rune_t, __darwin_size_t, __darwin_socklen_t, __darwin_ssize_t,
    __darwin_wchar_t, __int32_t, __int64_t, __uint16_t, __uint32_t, __uint8_t,
};
pub use self::_uid_t_h::uid_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::bouncer_h::{
    auth_type, pga_family, pga_is_unix, sockaddr_ucreds, PgAddr, ReplicationType, AUTH_TYPE_ANY,
    AUTH_TYPE_CERT, AUTH_TYPE_HBA, AUTH_TYPE_LDAP, AUTH_TYPE_MD5, AUTH_TYPE_PAM, AUTH_TYPE_PEER,
    AUTH_TYPE_PLAIN, AUTH_TYPE_REJECT, AUTH_TYPE_SCRAM_SHA_256, AUTH_TYPE_TRUST,
    REPLICATION_LOGICAL, REPLICATION_NONE, REPLICATION_PHYSICAL,
};
pub use self::cbtree_h::{
    cbtree_create, cbtree_getkey_func, cbtree_insert, cbtree_lookup, cbtree_walker_func, CBTree,
};
pub use self::ctype_h::safe_isspace;
pub use self::cxalloc_h::{cx_alloc, cx_alloc0, cx_destroy, CxMem, CxOps};
use self::cxextra_h::cx_new_pool;
use self::errno_h::__error;
pub use self::hba_h::{
    HBAAddress, HBAName, HBARule, Ident, IdentMap, Mapping, RuleType, ADDRESS_ALL, HBA, NAME_ALL,
    NAME_REPLICATION, NAME_SAMEUSER, RULE_HOST, RULE_HOSTNOSSL, RULE_HOSTSSL, RULE_LOCAL,
};
pub use self::in6_h::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use self::in_h::{in_addr, sockaddr_in};
use self::inet_h::{inet_ntop, inet_pton};
pub use self::list_h::{list_append, list_del, list_init, List};
pub use self::logging_h::{
    log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE, LG_STATS, LG_WARNING,
};
pub use self::runetype_h::{
    _DefaultRuneLocale, _RuneCharClass, _RuneEntry, _RuneLocale, _RuneRange,
};
pub use self::socket_h::{sockaddr, AF_INET, AF_INET6, AF_UNIX};
pub use self::stdbool_h::{false_0, true_0};
use self::string_h::usual_dirname;
pub use self::sys__types_h::{__darwin_off_t, __darwin_pid_t, __darwin_uid_t, __DARWIN_NULL};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "36:1"]
pub struct StrSet {
    pub pool: *const CxMem,
    pub count: ::core::ffi::c_uint,
    pub alloc: ::core::ffi::c_uint,
    pub nodes: *mut *mut StrSetNode,
    pub cbtree: *mut CBTree,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "31:1"]
pub struct StrSetNode {
    pub s_len: ::core::ffi::c_uint,
    pub s_val: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "151:1"]
pub struct TokParser {
    pub pos: *const ::core::ffi::c_char,
    pub cur_tok: TokType,
    pub cur_tok_str: *mut ::core::ffi::c_char,
    pub buf: *mut ::core::ffi::c_char,
    pub buflen: size_t,
}
#[c2rust::src_loc = "143:1"]
pub type TokType = ::core::ffi::c_uint;
#[c2rust::src_loc = "148:2"]
pub const TOK_EOL: TokType = 4;
#[c2rust::src_loc = "147:2"]
pub const TOK_FAIL: TokType = 3;
#[c2rust::src_loc = "146:2"]
pub const TOK_COMMA: TokType = 2;
#[c2rust::src_loc = "145:2"]
pub const TOK_IDENT: TokType = 1;
#[c2rust::src_loc = "144:2"]
pub const TOK_STRING: TokType = 0;
#[no_mangle]
#[c2rust::src_loc = "49:1"]
pub unsafe extern "C" fn strset_new(mut cx: *const CxMem) -> *mut StrSet {
    let mut set = ::core::ptr::null_mut::<StrSet>();
    let mut pool = ::core::ptr::null::<CxMem>();
    pool = cx_new_pool(cx, 1024 as size_t, 0 as ::core::ffi::c_uint);
    if pool.is_null() {
        return ::core::ptr::null_mut::<StrSet>();
    }
    set = cx_alloc(pool, ::core::mem::size_of::<StrSet>() as size_t) as *mut StrSet;
    if set.is_null() {
        return ::core::ptr::null_mut::<StrSet>();
    }
    (*set).pool = pool;
    (*set).cbtree = ::core::ptr::null_mut::<CBTree>();
    (*set).count = 0 as ::core::ffi::c_uint;
    (*set).alloc = 10 as ::core::ffi::c_uint;
    (*set).nodes = cx_alloc0(
        pool,
        ((*set).alloc as size_t).wrapping_mul(::core::mem::size_of::<*mut StrSet>() as size_t),
    ) as *mut *mut StrSetNode;
    if (*set).nodes.is_null() {
        cx_destroy(pool);
        return ::core::ptr::null_mut::<StrSet>();
    }
    set
}
#[c2rust::src_loc = "72:1"]
unsafe extern "C" fn strset_node_key(
    mut _ctx: *mut ::core::ffi::c_void,
    mut obj: *mut ::core::ffi::c_void,
    mut ptr_p: *mut *const ::core::ffi::c_void,
) -> size_t {
    let mut node = obj as *mut StrSetNode;
    *ptr_p = &raw mut (*node).s_val as *mut ::core::ffi::c_char as *const ::core::ffi::c_void;
    (*node).s_len as size_t
}
#[no_mangle]
#[c2rust::src_loc = "79:1"]
pub unsafe extern "C" fn strset_add(
    mut set: *mut StrSet,
    mut str: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_uint,
) -> bool {
    let mut node = ::core::ptr::null_mut::<StrSetNode>();
    let mut i: ::core::ffi::c_uint = 0;
    let mut ok: bool = false;
    if strset_contains(set, str, len) {
        return true_0 != 0;
    }
    node = cx_alloc(
        (*set).pool,
        (4 as size_t)
            .wrapping_add(len as size_t)
            .wrapping_add(1 as size_t),
    ) as *mut StrSetNode;
    if node.is_null() {
        return false_0 != 0;
    }
    (*node).s_len = len;
    memcpy(
        &raw mut (*node).s_val as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        str as *const ::core::ffi::c_void,
        len as size_t,
    );
    *(&raw mut (*node).s_val as *mut ::core::ffi::c_char).offset(len as isize) =
        0 as ::core::ffi::c_char;
    if (*set).count < (*set).alloc {
        let fresh2 = (*set).count;
        (*set).count = (*set).count.wrapping_add(1);
        let fresh3 = &mut *(*set).nodes.offset(fresh2 as isize);
        *fresh3 = node;
        return true_0 != 0;
    }
    if (*set).cbtree.is_null() {
        (*set).cbtree = cbtree_create(
            Some(
                strset_node_key
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                        *mut *const ::core::ffi::c_void,
                    ) -> size_t,
            ),
            None,
            set as *mut ::core::ffi::c_void,
            (*set).pool,
        );
        if (*set).cbtree.is_null() {
            return false_0 != 0;
        }
        i = 0 as ::core::ffi::c_uint;
        while i < (*set).count {
            ok = cbtree_insert(
                (*set).cbtree,
                *(*set).nodes.offset(i as isize) as *mut ::core::ffi::c_void,
            );
            if !ok {
                return false_0 != 0;
            }
            i = i.wrapping_add(1);
        }
    }
    ok = cbtree_insert((*set).cbtree, node as *mut ::core::ffi::c_void);
    if !ok {
        return false_0 != 0;
    }
    (*set).count = (*set).count.wrapping_add(1);
    true_0 != 0
}
#[no_mangle]
#[c2rust::src_loc = "117:1"]
pub unsafe extern "C" fn strset_contains(
    mut set: *mut StrSet,
    mut str: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_uint,
) -> bool {
    let mut i: ::core::ffi::c_uint = 0;
    let mut node = ::core::ptr::null_mut::<StrSetNode>();
    if !(*set).cbtree.is_null() {
        return !cbtree_lookup(
            (*set).cbtree,
            str as *const ::core::ffi::c_void,
            len as size_t,
        )
        .is_null();
    }
    i = 0 as ::core::ffi::c_uint;
    while i < (*set).count {
        node = *(*set).nodes.offset(i as isize);
        if ((*node).s_len == len)
            && memcmp(
                &raw mut (*node).s_val as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                str as *const ::core::ffi::c_void,
                len as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            return true_0 != 0;
        }
        i = i.wrapping_add(1);
    }
    false_0 != 0
}
#[no_mangle]
#[c2rust::src_loc = "133:1"]
pub unsafe extern "C" fn strset_free(mut set: *mut StrSet) {
    if !set.is_null() {
        cx_destroy((*set).pool);
    }
}
#[c2rust::src_loc = "160:1"]
unsafe extern "C" fn tok_buf_check(mut p: *mut TokParser, mut len: size_t) -> bool {
    let mut tmplen: size_t = 0;
    let mut tmp = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*p).buflen >= len {
        return true_0 != 0;
    }
    tmplen = len.wrapping_mul(2 as size_t);
    tmp = realloc((*p).buf as *mut ::core::ffi::c_void, tmplen) as *mut ::core::ffi::c_char;
    if tmp.is_null() {
        return false_0 != 0;
    }
    (*p).buf = tmp;
    (*p).buflen = tmplen;
    true_0 != 0
}
#[c2rust::src_loc = "175:1"]
unsafe extern "C" fn next_token(mut p: *mut TokParser) -> TokType {
    let mut s = ::core::ptr::null::<::core::ffi::c_char>();
    let mut s2 = ::core::ptr::null::<::core::ffi::c_char>();
    let mut dst = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*p).cur_tok as ::core::ffi::c_uint == TOK_EOL as ::core::ffi::c_int as ::core::ffi::c_uint {
        return TOK_EOL;
    }
    (*p).cur_tok_str = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*p).cur_tok = TOK_FAIL;
    while *(*p).pos.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        && safe_isspace(
            *(*p).pos.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                as ::core::ffi::c_int,
        ) != 0
    {
        (*p).pos = (*p).pos.offset(1);
    }
    if *(*p).pos.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '#' as i32
        || *(*p).pos.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32
    {
        (*p).cur_tok = TOK_EOL;
        (*p).pos = ::core::ptr::null::<::core::ffi::c_char>();
    } else if *(*p).pos.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ',' as i32
    {
        (*p).cur_tok = TOK_COMMA;
        (*p).pos = (*p).pos.offset(1);
    } else if *(*p).pos.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '"' as i32
    {
        s = (*p).pos.offset(1 as ::core::ffi::c_int as isize);
        while *s.offset(0 as ::core::ffi::c_int as isize) != 0 {
            if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '"' as i32 {
                if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '"' as i32 {
                    break;
                }
                s = s.offset(1);
            }
            s = s.offset(1);
        }
        if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '"' as i32
            || !tok_buf_check(p, s.offset_from((*p).pos) as ::core::ffi::c_long as size_t)
        {
            return TOK_FAIL;
        }
        dst = (*p).buf;
        s2 = (*p).pos.offset(1 as ::core::ffi::c_int as isize);
        while s2 < s {
            let fresh0 = dst;
            dst = dst.offset(1);
            *fresh0 = *s2;
            if *s2 as ::core::ffi::c_int == '"' as i32 {
                s2 = s2.offset(1);
            }
            s2 = s2.offset(1);
        }
        *dst = 0 as ::core::ffi::c_char;
        (*p).pos = s.offset(1 as ::core::ffi::c_int as isize);
        (*p).cur_tok = TOK_STRING;
        (*p).cur_tok_str = (*p).buf;
    } else {
        s = (*p).pos.offset(1 as ::core::ffi::c_int as isize);
        while *s != 0 {
            if *s as ::core::ffi::c_int == ',' as i32
                || *s as ::core::ffi::c_int == '#' as i32
                || *s as ::core::ffi::c_int == '"' as i32
            {
                break;
            }
            if safe_isspace(*s as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
                break;
            }
            s = s.offset(1);
        }
        if !tok_buf_check(
            p,
            (s.offset_from((*p).pos) as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as size_t,
        ) {
            return TOK_FAIL;
        }
        memcpy(
            (*p).buf as *mut ::core::ffi::c_void,
            (*p).pos as *const ::core::ffi::c_void,
            s.offset_from((*p).pos) as ::core::ffi::c_long as size_t,
        );
        *(*p)
            .buf
            .offset(s.offset_from((*p).pos) as ::core::ffi::c_long as isize) =
            0 as ::core::ffi::c_char;
        (*p).pos = s;
        (*p).cur_tok = TOK_IDENT;
        (*p).cur_tok_str = (*p).buf;
    }
    (*p).cur_tok
}
#[c2rust::src_loc = "231:1"]
unsafe extern "C" fn eat_all(mut p: *mut TokParser) {
    (*p).cur_tok = TOK_EOL;
}
#[c2rust::src_loc = "236:1"]
unsafe extern "C" fn eat(mut p: *mut TokParser, mut ttype: TokType) -> bool {
    if (*p).cur_tok as ::core::ffi::c_uint == ttype as ::core::ffi::c_uint {
        next_token(p);
        return true_0 != 0;
    }
    false_0 != 0
}
#[c2rust::src_loc = "246:1"]
unsafe extern "C" fn check_kw(mut p: *mut TokParser, mut kw: *const ::core::ffi::c_char) -> bool {
    if (*p).cur_tok as ::core::ffi::c_uint == TOK_IDENT as ::core::ffi::c_int as ::core::ffi::c_uint
        && strcmp(kw, (*p).cur_tok_str) == 0 as ::core::ffi::c_int
    {
        return true_0 != 0;
    }
    false_0 != 0
}
#[c2rust::src_loc = "254:1"]
unsafe extern "C" fn eat_kw(mut p: *mut TokParser, mut kw: *const ::core::ffi::c_char) -> bool {
    if (*p).cur_tok as ::core::ffi::c_uint == TOK_IDENT as ::core::ffi::c_int as ::core::ffi::c_uint
        && strcmp(kw, (*p).cur_tok_str) == 0 as ::core::ffi::c_int
    {
        next_token(p);
        return true_0 != 0;
    }
    false_0 != 0
}
#[c2rust::src_loc = "263:1"]
unsafe extern "C" fn expect(
    mut tp: *mut TokParser,
    mut ttype: TokType,
    mut str_p: *mut *const ::core::ffi::c_char,
) -> bool {
    if (*tp).cur_tok as ::core::ffi::c_uint == ttype as ::core::ffi::c_uint {
        *str_p = (*tp).buf;
        return true_0 != 0;
    }
    false_0 != 0
}
#[c2rust::src_loc = "272:1"]
unsafe extern "C" fn path_join(
    mut p1: *const ::core::ffi::c_char,
    mut p2: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut len1: size_t = 0;
    let mut len2: size_t = 0;
    let mut res = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pos = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if *p2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32
        || *p1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        || memcmp(
            p1 as *const ::core::ffi::c_void,
            b".\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            2 as size_t,
        ) == 0
    {
        return strdup(p2);
    }
    len1 = strlen(p1);
    len2 = strlen(p2);
    res = malloc(
        len1.wrapping_add(len2)
            .wrapping_add(2 as size_t)
            .wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    if !res.is_null() {
        memcpy(
            res as *mut ::core::ffi::c_void,
            p1 as *const ::core::ffi::c_void,
            len1,
        );
        pos = res.add(len1);
        if *pos.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int != '/' as i32 {
            let fresh4 = pos;
            pos = pos.offset(1);
            *fresh4 = '/' as i32 as ::core::ffi::c_char;
        }
        memcpy(
            pos as *mut ::core::ffi::c_void,
            p2 as *const ::core::ffi::c_void,
            len2.wrapping_add(1 as size_t),
        );
    }
    res
}
#[c2rust::src_loc = "292:1"]
unsafe extern "C" fn path_join_dirname(
    mut parent: *const ::core::ffi::c_char,
    mut fn_0: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut tmp = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut res = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut basedir = ::core::ptr::null::<::core::ffi::c_char>();
    if *fn_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32 {
        return strdup(fn_0);
    }
    tmp = strdup(parent);
    if tmp.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    basedir = usual_dirname(tmp);
    res = path_join(basedir, fn_0);
    free(tmp as *mut ::core::ffi::c_void);
    res
}
#[c2rust::src_loc = "307:1"]
unsafe extern "C" fn init_parser(mut p: *mut TokParser) {
    memset(
        p as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<TokParser>() as size_t,
    );
}
#[c2rust::src_loc = "312:1"]
unsafe extern "C" fn parse_from_string(mut p: *mut TokParser, mut str: *const ::core::ffi::c_char) {
    (*p).pos = str;
    (*p).cur_tok = TOK_COMMA;
    (*p).cur_tok_str = ::core::ptr::null_mut::<::core::ffi::c_char>();
    next_token(p);
}
#[c2rust::src_loc = "320:1"]
unsafe extern "C" fn free_parser(mut p: *mut TokParser) {
    free((*p).buf as *mut ::core::ffi::c_void);
    (*p).buf = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[c2rust::src_loc = "329:1"]
unsafe extern "C" fn parse_namefile(
    mut hname: *mut HBAName,
    mut fn_0: *const ::core::ffi::c_char,
    mut is_db: bool,
) -> bool {
    let mut f = ::core::ptr::null_mut::<FILE>();
    let mut len: ssize_t = 0;
    let mut ln = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut buflen: size_t = 0 as size_t;
    let mut ok = false_0 != 0;
    let mut tp = TokParser {
        pos: ::core::ptr::null::<::core::ffi::c_char>(),
        cur_tok: TOK_STRING,
        cur_tok_str: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        buf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        buflen: 0,
    };
    init_parser(&raw mut tp);
    f = fopen(fn_0, b"r\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if f.is_null() {
        return false_0 != 0;
    }
    loop {
        len = getline(&raw mut ln, &raw mut buflen, f);
        if len < 0 as ssize_t {
            ok = true_0 != 0;
            break;
        } else {
            parse_from_string(&raw mut tp, ln);
            if !parse_names(hname, &raw mut tp, is_db, fn_0) {
                break;
            }
        }
    }
    free_parser(&raw mut tp);
    free(ln as *mut ::core::ffi::c_void);
    fclose(f);
    ok
}
#[c2rust::src_loc = "360:1"]
unsafe extern "C" fn parse_ident_name(
    mut ident_name: *mut *const ::core::ffi::c_char,
    mut tp: *mut TokParser,
    mut is_name_all: *mut bool,
) -> bool {
    if eat_kw(tp, b"all\0" as *const u8 as *const ::core::ffi::c_char) {
        *is_name_all = true_0 != 0;
        return true_0 != 0;
    }
    if !expect(tp, TOK_IDENT, ident_name) && !expect(tp, TOK_STRING, ident_name) {
        return false_0 != 0;
    }
    true_0 != 0
}
#[c2rust::src_loc = "376:1"]
unsafe extern "C" fn parse_names(
    mut hname: *mut HBAName,
    mut tp: *mut TokParser,
    mut is_db: bool,
    mut parent_filename: *const ::core::ffi::c_char,
) -> bool {
    let mut current_block: u64;
    let mut tok = ::core::ptr::null::<::core::ffi::c_char>();
    loop {
        if eat_kw(tp, b"all\0" as *const u8 as *const ::core::ffi::c_char) {
            (*hname).flags |= NAME_ALL as ::core::ffi::c_uint;
        } else {
            if is_db {
                if eat_kw(tp, b"sameuser\0" as *const u8 as *const ::core::ffi::c_char) {
                    (*hname).flags |= NAME_SAMEUSER as ::core::ffi::c_uint;
                    current_block = 13429877768294522094;
                } else {
                    if eat_kw(tp, b"samerole\0" as *const u8 as *const ::core::ffi::c_char) {
                        let mut _log_ctx = NULL;
                        log_generic(
                            LG_WARNING,
                            _log_ctx,
                            b"samerole is not supported\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        return false_0 != 0;
                    }
                    if eat_kw(
                        tp,
                        b"samegroup\0" as *const u8 as *const ::core::ffi::c_char,
                    ) {
                        let mut _log_ctx_0 = NULL;
                        log_generic(
                            LG_WARNING,
                            _log_ctx_0,
                            b"samegroup is not supported\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        return false_0 != 0;
                    }
                    if eat_kw(
                        tp,
                        b"replication\0" as *const u8 as *const ::core::ffi::c_char,
                    ) {
                        (*hname).flags |= NAME_REPLICATION as ::core::ffi::c_uint;
                        current_block = 13429877768294522094;
                    } else {
                        current_block = 5948590327928692120;
                    }
                }
            } else {
                current_block = 5948590327928692120;
            }
            match current_block {
                13429877768294522094 => {}
                _ => {
                    if expect(tp, TOK_IDENT, &raw mut tok) {
                        if *tok.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == '+' as i32
                        {
                            return false_0 != 0;
                        }
                        if *tok.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == '@' as i32
                        {
                            let mut ok: bool = false;
                            let mut fn_0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                            fn_0 = path_join_dirname(
                                parent_filename,
                                tok.offset(1 as ::core::ffi::c_int as isize),
                            );
                            if fn_0.is_null() {
                                return false_0 != 0;
                            }
                            ok = parse_namefile(hname, fn_0, is_db);
                            free(fn_0 as *mut ::core::ffi::c_void);
                            if !ok {
                                return false_0 != 0;
                            }
                            next_token(tp);
                            current_block = 13429877768294522094;
                        } else {
                            current_block = 3275366147856559585;
                        }
                    } else {
                        if expect(tp, TOK_STRING, &raw mut tok) {
                        } else {
                            return false_0 != 0;
                        }
                        current_block = 3275366147856559585;
                    }
                    match current_block {
                        13429877768294522094 => {}
                        _ => {
                            if (*hname).name_set.is_null() {
                                (*hname).name_set =
                                    strset_new(::core::ptr::null::<CxMem>()) as *mut StrSet;
                                if (*hname).name_set.is_null() {
                                    return false_0 != 0;
                                }
                            }
                            if !strset_add(
                                (*hname).name_set as *mut StrSet,
                                tok,
                                strlen(tok) as ::core::ffi::c_uint,
                            ) {
                                return false_0 != 0;
                            }
                            next_token(tp);
                        }
                    }
                }
            }
        }
        if !eat(tp, TOK_COMMA) {
            break;
        }
    }
    true_0 != 0
}
#[c2rust::src_loc = "447:1"]
unsafe extern "C" fn rule_free(mut rule: *mut HBARule) {
    strset_free((*rule).db_name.name_set as *mut StrSet);
    strset_free((*rule).user_name.name_set as *mut StrSet);
    free((*rule).auth_options as *mut ::core::ffi::c_void);
    free(rule as *mut ::core::ffi::c_void);
}
#[c2rust::src_loc = "455:1"]
unsafe extern "C" fn parse_addr(
    mut haddress: *mut HBAAddress,
    mut addr: *const ::core::ffi::c_char,
) -> bool {
    if inet_pton(
        AF_INET6,
        addr,
        &raw mut (*haddress).addr as *mut uint8_t as *mut ::core::ffi::c_void,
    ) != 0
    {
        (*haddress).family = AF_INET6;
    } else if inet_pton(
        AF_INET,
        addr,
        &raw mut (*haddress).addr as *mut uint8_t as *mut ::core::ffi::c_void,
    ) != 0
    {
        (*haddress).family = AF_INET;
    } else {
        return false_0 != 0;
    }
    true_0 != 0
}
#[c2rust::src_loc = "467:1"]
unsafe extern "C" fn parse_nmask(
    mut haddress: *mut HBAAddress,
    mut nmask: *const ::core::ffi::c_char,
) -> bool {
    let mut end = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut bits: ::core::ffi::c_ulong = 0;
    let mut i: ::core::ffi::c_uint = 0;
    *__error() = 0 as ::core::ffi::c_int;
    bits = strtoul(nmask, &raw mut end, 10 as ::core::ffi::c_int);
    if *__error() != 0 || *end as ::core::ffi::c_int != 0 {
        return false_0 != 0;
    }
    if (*haddress).family == AF_INET && bits > 32 as ::core::ffi::c_ulong {
        return false_0 != 0;
    }
    if (*haddress).family == AF_INET6 && bits > 128 as ::core::ffi::c_ulong {
        return false_0 != 0;
    }
    i = 0 as ::core::ffi::c_uint;
    while (i as ::core::ffi::c_ulong) < bits.wrapping_div(8 as ::core::ffi::c_ulong) {
        (*haddress).mask[i as usize] = 255 as uint8_t;
        i = i.wrapping_add(1);
    }
    if bits.wrapping_rem(8 as ::core::ffi::c_ulong) != 0 {
        (*haddress).mask[i as usize] = ((255 as ::core::ffi::c_int)
            << (8 as ::core::ffi::c_ulong)
                .wrapping_sub(bits.wrapping_rem(8 as ::core::ffi::c_ulong)))
            as uint8_t;
    }
    true_0 != 0
}
#[c2rust::src_loc = "490:1"]
unsafe extern "C" fn bad_mask(mut haddress: *mut HBAAddress) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut bytes = if (*haddress).family == AF_INET {
        4 as ::core::ffi::c_int
    } else {
        16 as ::core::ffi::c_int
    };
    let mut res: uint8_t = 0 as uint8_t;
    i = 0 as ::core::ffi::c_int;
    while i < bytes {
        res = (res as ::core::ffi::c_int
            | (*haddress).addr[i as usize] as ::core::ffi::c_int
                & (255 as ::core::ffi::c_int ^ (*haddress).mask[i as usize] as ::core::ffi::c_int))
            as uint8_t;
        i += 1;
    }
    res != 0
}
#[c2rust::src_loc = "499:1"]
unsafe extern "C" fn match_map(
    mut rule: *mut HBARule,
    mut ident: *mut Ident,
    mut mapname: *const ::core::ffi::c_char,
) -> bool {
    let mut el = ::core::ptr::null_mut::<List>();
    let mut map = ::core::ptr::null_mut::<IdentMap>();
    if ident.is_null() {
        return false_0 != 0;
    }
    el = (*ident).maps.next;
    while el != &raw mut (*ident).maps {
        map = (el as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut IdentMap;
        if strcmp((*map).map_name, mapname) == 0 as ::core::ffi::c_int {
            (*rule).identmap = map as *mut IdentMap;
            return true_0 != 0;
        }
        el = (*el).next;
    }
    false_0 != 0
}
#[c2rust::src_loc = "519:1"]
unsafe extern "C" fn parse_map_definition(
    mut rule: *mut HBARule,
    mut ident: *mut Ident,
    mut tp: *mut TokParser,
    mut linenr: ::core::ffi::c_int,
) -> bool {
    let mut str = ::core::ptr::null::<::core::ffi::c_char>();
    let mut val = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !expect(tp, TOK_IDENT, &raw mut str) {
        return true_0 != 0;
    }
    val = strchr(str, '=' as i32);
    if val.is_null()
        || strncmp(
            str,
            b"map=\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) != 0 as ::core::ffi::c_int
    {
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            b"hba line %d: Ident map %s is malformed. It is not in map=value format.\0" as *const u8
                as *const ::core::ffi::c_char,
            linenr,
            str,
        );
        return false_0 != 0;
    }
    val = val.offset(1);
    next_token(tp);
    if !match_map(rule, ident, val) {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_0,
            b"hba line %d: Ident map %s is not found in ident config file\0" as *const u8
                as *const ::core::ffi::c_char,
            linenr,
            val,
        );
        return false_0 != 0;
    }
    true_0 != 0
}
#[c2rust::src_loc = "545:1"]
unsafe extern "C" fn mapping_free(mut mapping: *mut Mapping) {
    free((*mapping).system_user_name as *mut ::core::ffi::c_void);
    free((*mapping).postgres_user_name as *mut ::core::ffi::c_void);
    free(mapping as *mut ::core::ffi::c_void);
}
#[c2rust::src_loc = "552:1"]
unsafe extern "C" fn ident_map_free(mut ident_map: *mut IdentMap) {
    let mut el = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut mapping = ::core::ptr::null_mut::<Mapping>();
    if ident_map.is_null() {
        return;
    }
    el = (*ident_map).mappings.next;
    tmp = (*(*ident_map).mappings.next).next;
    while el != &raw mut (*ident_map).mappings {
        mapping = (el as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut Mapping;
        list_del(&raw mut (*mapping).node);
        mapping_free(mapping);
        el = tmp;
        tmp = (*tmp).next;
    }
    free((*ident_map).map_name as *mut ::core::ffi::c_void);
    free(ident_map as *mut ::core::ffi::c_void);
}
#[c2rust::src_loc = "570:1"]
unsafe extern "C" fn find_ident_map(
    mut ident: *mut Ident,
    mut mapname: *const ::core::ffi::c_char,
    mut ident_map: *mut *mut IdentMap,
) -> bool {
    let mut el = ::core::ptr::null_mut::<List>();
    el = (*ident).maps.next;
    while el != &raw mut (*ident).maps {
        *ident_map = (el as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut IdentMap;
        if strcmp((**ident_map).map_name, mapname) == 0 {
            return true_0 != 0;
        }
        el = (*el).next;
    }
    false_0 != 0
}
#[c2rust::src_loc = "584:1"]
unsafe extern "C" fn parse_ident_line(
    mut ident: *mut Ident,
    mut tp: *mut TokParser,
    mut linenr: ::core::ffi::c_int,
) -> bool {
    let mut current_block: u64;
    let mut map_name = ::core::ptr::null::<::core::ffi::c_char>();
    let mut map_name_copy = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut system_user_name = ::core::ptr::null::<::core::ffi::c_char>();
    let mut postgres_user_name = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ident_map = ::core::ptr::null_mut::<IdentMap>();
    let mut mapping = ::core::ptr::null_mut::<Mapping>();
    let mut is_name_all = false_0 != 0;
    if eat(tp, TOK_EOL) {
        return true_0 != 0;
    }
    mapping = calloc(1 as size_t, ::core::mem::size_of::<Mapping>() as size_t) as *mut Mapping;
    if mapping.is_null() {
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            b"ident: no mem for parsing mapping\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    if expect(tp, TOK_IDENT, &raw mut map_name) {
        map_name_copy = strdup(map_name);
        if map_name_copy.is_null() {
            let mut _log_ctx_0 = NULL;
            log_generic(
                LG_WARNING,
                _log_ctx_0,
                b"ident: no mem for map_name\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            next_token(tp);
            if !expect(tp, TOK_IDENT, &raw mut system_user_name) {
                if !expect(tp, TOK_STRING, &raw mut system_user_name) {
                    current_block = 6603408307643950352;
                } else {
                    current_block = 12124785117276362961;
                }
            } else {
                current_block = 12124785117276362961;
            }
            match current_block {
                6603408307643950352 => {}
                _ => {
                    (*mapping).system_user_name = strdup(system_user_name);
                    if (*mapping).system_user_name.is_null() {
                        let mut _log_ctx_1 = NULL;
                        log_generic(
                            LG_WARNING,
                            _log_ctx_1,
                            b"ident: no mem for system_user_name\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    } else {
                        next_token(tp);
                        if parse_ident_name(&raw mut postgres_user_name, tp, &raw mut is_name_all) {
                            if is_name_all {
                                (*mapping).name_flags |= NAME_ALL as ::core::ffi::c_uint;
                                current_block = 11385396242402735691;
                            } else {
                                (*mapping).postgres_user_name = strdup(postgres_user_name);
                                if (*mapping).postgres_user_name.is_null() {
                                    let mut _log_ctx_2 = NULL;
                                    log_generic(
                                        LG_WARNING,
                                        _log_ctx_2,
                                        b"ident: no mem for postgres_user_name\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    current_block = 6603408307643950352;
                                } else {
                                    current_block = 11385396242402735691;
                                }
                            }
                            match current_block {
                                6603408307643950352 => {}
                                _ => {
                                    next_token(tp);
                                    if !eat(tp, TOK_EOL) {
                                        let mut _log_ctx_3 = NULL;
                                        log_generic(
                                            LG_WARNING,
                                            _log_ctx_3,
                                            b"ident line %d: unsupported parameters\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            linenr,
                                        );
                                    } else {
                                        if find_ident_map(ident, map_name_copy, &raw mut ident_map)
                                        {
                                            list_append(
                                                &raw mut (*ident_map).mappings,
                                                &raw mut (*mapping).node,
                                            );
                                            free(map_name_copy as *mut ::core::ffi::c_void);
                                            map_name_copy =
                                                ::core::ptr::null_mut::<::core::ffi::c_char>();
                                            current_block = 11777552016271000781;
                                        } else {
                                            ident_map = calloc(
                                                1 as size_t,
                                                ::core::mem::size_of::<IdentMap>() as size_t,
                                            )
                                                as *mut IdentMap;
                                            if ident_map.is_null() {
                                                let mut _log_ctx_4 = NULL;
                                                log_generic(
                                                    LG_WARNING,
                                                    _log_ctx_4,
                                                    b"ident: no mem for parsing ident_map\0"
                                                        as *const u8
                                                        as *const ::core::ffi::c_char,
                                                );
                                                current_block = 6603408307643950352;
                                            } else {
                                                (*ident_map).map_name = map_name_copy;
                                                map_name_copy =
                                                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                                                list_init(&raw mut (*ident_map).mappings);
                                                list_append(
                                                    &raw mut (*ident_map).mappings,
                                                    &raw mut (*mapping).node,
                                                );
                                                list_append(
                                                    &raw mut (*ident).maps,
                                                    &raw mut (*ident_map).node,
                                                );
                                                current_block = 11777552016271000781;
                                            }
                                        }
                                        match current_block {
                                            6603408307643950352 => {}
                                            _ => return true_0 != 0,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    mapping_free(mapping);
    ident_map_free(ident_map);
    free(map_name_copy as *mut ::core::ffi::c_void);
    false_0 != 0
}
#[c2rust::src_loc = "682:1"]
unsafe extern "C" fn parse_line(
    mut hba: *mut HBA,
    mut ident: *mut Ident,
    mut tp: *mut TokParser,
    mut linenr: ::core::ffi::c_int,
    mut parent_filename: *const ::core::ffi::c_char,
) -> bool {
    let mut current_block: u64;
    let mut addr = ::core::ptr::null::<::core::ffi::c_char>();
    let mut mask = ::core::ptr::null::<::core::ffi::c_char>();
    let mut rtype = RULE_LOCAL;
    let mut nmask = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rule = ::core::ptr::null_mut::<HBARule>();
    if eat_kw(tp, b"local\0" as *const u8 as *const ::core::ffi::c_char) {
        rtype = RULE_LOCAL;
    } else if eat_kw(tp, b"host\0" as *const u8 as *const ::core::ffi::c_char) {
        rtype = RULE_HOST;
    } else if eat_kw(tp, b"hostssl\0" as *const u8 as *const ::core::ffi::c_char) {
        rtype = RULE_HOSTSSL;
    } else if eat_kw(
        tp,
        b"hostnossl\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        rtype = RULE_HOSTNOSSL;
    } else if eat(tp, TOK_EOL) {
        return true_0 != 0;
    } else {
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            b"hba line %d: unknown type\0" as *const u8 as *const ::core::ffi::c_char,
            linenr,
        );
        return false_0 != 0;
    }
    rule = calloc(1 as size_t, ::core::mem::size_of::<HBARule>() as size_t) as *mut HBARule;
    if rule.is_null() {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx_0,
            b"hba: no mem for rule\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return false_0 != 0;
    }
    (*rule).rule_type = rtype;
    if parse_names(&raw mut (*rule).db_name, tp, true_0 != 0, parent_filename)
        && parse_names(&raw mut (*rule).user_name, tp, true_0 != 0, parent_filename)
    {
        if rtype as ::core::ffi::c_uint == RULE_LOCAL as ::core::ffi::c_int as ::core::ffi::c_uint {
            (*rule).address.family = AF_UNIX;
            current_block = 981995395831942902;
        } else if eat_kw(tp, b"all\0" as *const u8 as *const ::core::ffi::c_char) {
            (*rule).address.flags |= ADDRESS_ALL as ::core::ffi::c_uint;
            current_block = 981995395831942902;
        } else if !expect(tp, TOK_IDENT, &raw mut addr) {
            let mut _log_ctx_1 = NULL;
            log_generic(
                LG_WARNING,
                _log_ctx_1,
                b"hba line %d: did not find address - %d - '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                linenr,
                (*tp).cur_tok as ::core::ffi::c_uint,
                (*tp).buf,
            );
            current_block = 10970100966140617357;
        } else {
            nmask = strchr(addr, '/' as i32);
            if !nmask.is_null() {
                let fresh1 = nmask;
                nmask = nmask.offset(1);
                *fresh1 = 0 as ::core::ffi::c_char;
            }
            if !parse_addr(&raw mut (*rule).address, addr) {
                let mut _log_ctx_2 = NULL;
                log_generic(
                    LG_WARNING,
                    _log_ctx_2,
                    b"hba line %d: failed to parse address - %s\0" as *const u8
                        as *const ::core::ffi::c_char,
                    linenr,
                    addr,
                );
                current_block = 10970100966140617357;
            } else {
                if !nmask.is_null() {
                    if !parse_nmask(&raw mut (*rule).address, nmask) {
                        let mut _log_ctx_3 = NULL;
                        log_generic(
                            LG_WARNING,
                            _log_ctx_3,
                            b"hba line %d: invalid mask\0" as *const u8
                                as *const ::core::ffi::c_char,
                            linenr,
                        );
                        current_block = 10970100966140617357;
                    } else {
                        next_token(tp);
                        current_block = 2116367355679836638;
                    }
                } else {
                    next_token(tp);
                    if !expect(tp, TOK_IDENT, &raw mut mask) {
                        let mut _log_ctx_4 = NULL;
                        log_generic(
                            LG_WARNING,
                            _log_ctx_4,
                            b"hba line %d: did not find mask\0" as *const u8
                                as *const ::core::ffi::c_char,
                            linenr,
                        );
                        current_block = 10970100966140617357;
                    } else if inet_pton(
                        (*rule).address.family,
                        mask,
                        &raw mut (*rule).address.mask as *mut uint8_t as *mut ::core::ffi::c_void,
                    ) == 0
                    {
                        let mut _log_ctx_5 = NULL;
                        log_generic(
                            LG_WARNING,
                            _log_ctx_5,
                            b"hba line %d: failed to parse mask: %s\0" as *const u8
                                as *const ::core::ffi::c_char,
                            linenr,
                            mask,
                        );
                        current_block = 10970100966140617357;
                    } else {
                        next_token(tp);
                        current_block = 2116367355679836638;
                    }
                }
                match current_block {
                    10970100966140617357 => {}
                    _ => {
                        if bad_mask(&raw mut (*rule).address) {
                            let mut buf1: [::core::ffi::c_char; 128] = [0; 128];
                            let mut buf2: [::core::ffi::c_char; 128] = [0; 128];
                            let mut _log_ctx_6 = NULL;
                            log_generic(
                                LG_WARNING,
                                _log_ctx_6,
                                b"address does not match mask in %s line #%d: %s / %s\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char,
                                parent_filename,
                                linenr,
                                inet_ntop(
                                    (*rule).address.family,
                                    &raw mut (*rule).address.addr as *mut uint8_t
                                        as *const ::core::ffi::c_void,
                                    &raw mut buf1 as *mut ::core::ffi::c_char,
                                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
                                        as socklen_t,
                                ),
                                inet_ntop(
                                    (*rule).address.family,
                                    &raw mut (*rule).address.mask as *mut uint8_t
                                        as *const ::core::ffi::c_void,
                                    &raw mut buf2 as *mut ::core::ffi::c_char,
                                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
                                        as socklen_t,
                                ),
                            );
                        }
                        current_block = 981995395831942902;
                    }
                }
            }
        }
        match current_block {
            10970100966140617357 => {}
            _ => {
                if eat_kw(tp, b"trust\0" as *const u8 as *const ::core::ffi::c_char) {
                    (*rule).rule_method = AUTH_TYPE_TRUST as ::core::ffi::c_int;
                    current_block = 200744462051969938;
                } else if eat_kw(tp, b"reject\0" as *const u8 as *const ::core::ffi::c_char) {
                    (*rule).rule_method = AUTH_TYPE_REJECT as ::core::ffi::c_int;
                    current_block = 200744462051969938;
                } else if eat_kw(tp, b"md5\0" as *const u8 as *const ::core::ffi::c_char) {
                    (*rule).rule_method = AUTH_TYPE_MD5 as ::core::ffi::c_int;
                    current_block = 200744462051969938;
                } else if eat_kw(tp, b"password\0" as *const u8 as *const ::core::ffi::c_char) {
                    (*rule).rule_method = AUTH_TYPE_PLAIN as ::core::ffi::c_int;
                    current_block = 200744462051969938;
                } else if eat_kw(tp, b"peer\0" as *const u8 as *const ::core::ffi::c_char) {
                    (*rule).rule_method = AUTH_TYPE_PEER as ::core::ffi::c_int;
                    current_block = 200744462051969938;
                } else if eat_kw(tp, b"cert\0" as *const u8 as *const ::core::ffi::c_char) {
                    (*rule).rule_method = AUTH_TYPE_CERT as ::core::ffi::c_int;
                    current_block = 200744462051969938;
                } else if eat_kw(
                    tp,
                    b"scram-sha-256\0" as *const u8 as *const ::core::ffi::c_char,
                ) {
                    (*rule).rule_method = AUTH_TYPE_SCRAM_SHA_256 as ::core::ffi::c_int;
                    current_block = 200744462051969938;
                } else if check_kw(tp, b"ldap\0" as *const u8 as *const ::core::ffi::c_char) {
                    (*rule).rule_method = AUTH_TYPE_LDAP as ::core::ffi::c_int;
                    current_block = 200744462051969938;
                } else {
                    let mut _log_ctx_7 = NULL;
                    log_generic(
                        LG_WARNING,
                        _log_ctx_7,
                        b"hba line %d: unsupported method: buf=%s\0" as *const u8
                            as *const ::core::ffi::c_char,
                        linenr,
                        (*tp).buf,
                    );
                    current_block = 10970100966140617357;
                }
                match current_block {
                    10970100966140617357 => {}
                    _ => {
                        if (*rule).rule_method == AUTH_TYPE_LDAP as ::core::ffi::c_int {
                            (*rule).auth_options = strdup((*tp).pos);
                            if (*rule).auth_options.is_null() {
                                let mut _log_ctx_8 = NULL;
                                log_generic(
                                    LG_WARNING,
                                    _log_ctx_8,
                                    b"hba line %d: cannot get auth_options: buf=%s\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    linenr,
                                    (*tp).pos,
                                );
                                current_block = 10970100966140617357;
                            } else {
                                eat_all(tp);
                                current_block = 6014157347423944569;
                            }
                        } else {
                            current_block = 6014157347423944569;
                        }
                        match current_block {
                            10970100966140617357 => {}
                            _ => {
                                if parse_map_definition(rule, ident, tp, linenr) {
                                    if !eat(tp, TOK_EOL) {
                                        let mut _log_ctx_9 = NULL;
                                        log_generic(
                                            LG_WARNING,
                                            _log_ctx_9,
                                            b"hba line %d: unsupported parameters\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            linenr,
                                        );
                                    } else {
                                        (*rule).hba_linenr = linenr;
                                        list_append(&raw mut (*hba).rules, &raw mut (*rule).node);
                                        return true_0 != 0;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    rule_free(rule);
    false_0 != 0
}
#[no_mangle]
#[c2rust::src_loc = "807:1"]
pub unsafe extern "C" fn ident_load_map(mut fn_0: *const ::core::ffi::c_char) -> *mut Ident {
    let mut ident = ::core::ptr::null_mut::<Ident>();
    let mut f = ::core::ptr::null_mut::<FILE>();
    let mut ln = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut lnbuf: size_t = 0 as size_t;
    let mut len: ssize_t = 0;
    let mut linenr: ::core::ffi::c_int = 0;
    let mut tp = TokParser {
        pos: ::core::ptr::null::<::core::ffi::c_char>(),
        cur_tok: TOK_STRING,
        cur_tok_str: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        buf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        buflen: 0,
    };
    if fn_0.is_null() {
        return ::core::ptr::null_mut::<Ident>();
    }
    init_parser(&raw mut tp);
    ident = malloc(::core::mem::size_of::<Ident>() as size_t) as *mut Ident;
    if !ident.is_null() {
        list_init(&raw mut (*ident).maps);
        f = fopen(fn_0, b"r\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
        if f.is_null() {
            let mut _log_ctx = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx,
                b"could not open ident config file %s: %s\0" as *const u8
                    as *const ::core::ffi::c_char,
                fn_0,
                strerror(*__error()),
            );
        } else {
            linenr = 1 as ::core::ffi::c_int;
            loop {
                len = getline(&raw mut ln, &raw mut lnbuf, f);
                if len < 0 as ssize_t {
                    break;
                }
                parse_from_string(&raw mut tp, ln);
                if !parse_ident_line(ident, &raw mut tp, linenr) {
                    let mut _log_ctx_0 = NULL;
                    log_generic(
                        LG_WARNING,
                        _log_ctx_0,
                        b"could not parse ident config line %d\0" as *const u8
                            as *const ::core::ffi::c_char,
                        linenr,
                    );
                }
                linenr += 1;
            }
        }
    }
    free_parser(&raw mut tp);
    free(ln as *mut ::core::ffi::c_void);
    if !f.is_null() {
        fclose(f);
    }
    ident
}
#[no_mangle]
#[c2rust::src_loc = "860:1"]
pub unsafe extern "C" fn hba_load_rules(
    mut fn_0: *const ::core::ffi::c_char,
    mut ident: *mut Ident,
) -> *mut HBA {
    let mut hba = ::core::ptr::null_mut::<HBA>();
    let mut f = ::core::ptr::null_mut::<FILE>();
    let mut ln = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut lnbuf: size_t = 0 as size_t;
    let mut len: ssize_t = 0;
    let mut linenr: ::core::ffi::c_int = 0;
    let mut tp = TokParser {
        pos: ::core::ptr::null::<::core::ffi::c_char>(),
        cur_tok: TOK_STRING,
        cur_tok_str: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        buf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        buflen: 0,
    };
    init_parser(&raw mut tp);
    hba = malloc(::core::mem::size_of::<HBA>() as size_t) as *mut HBA;
    if !hba.is_null() {
        list_init(&raw mut (*hba).rules);
        f = fopen(fn_0, b"r\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
        if f.is_null() {
            let mut _log_ctx = NULL;
            log_generic(
                LG_ERROR,
                _log_ctx,
                b"could not open hba config file %s: %s\0" as *const u8
                    as *const ::core::ffi::c_char,
                fn_0,
                strerror(*__error()),
            );
        } else {
            linenr = 1 as ::core::ffi::c_int;
            loop {
                len = getline(&raw mut ln, &raw mut lnbuf, f);
                if len < 0 as ssize_t {
                    break;
                }
                parse_from_string(&raw mut tp, ln);
                if !parse_line(hba, ident, &raw mut tp, linenr, fn_0) {
                    let mut _log_ctx_0 = NULL;
                    log_generic(
                        LG_WARNING,
                        _log_ctx_0,
                        b"could not parse hba config line %d\0" as *const u8
                            as *const ::core::ffi::c_char,
                        linenr,
                    );
                }
                linenr += 1;
            }
        }
    }
    free_parser(&raw mut tp);
    free(ln as *mut ::core::ffi::c_void);
    if !f.is_null() {
        fclose(f);
    }
    hba
}
#[no_mangle]
#[c2rust::src_loc = "904:1"]
pub unsafe extern "C" fn ident_free(mut ident: *mut Ident) {
    let mut el = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut map = ::core::ptr::null_mut::<IdentMap>();
    if ident.is_null() {
        return;
    }
    el = (*ident).maps.next;
    tmp = (*(*ident).maps.next).next;
    while el != &raw mut (*ident).maps {
        map = (el as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut IdentMap;
        list_del(&raw mut (*map).node);
        ident_map_free(map);
        el = tmp;
        tmp = (*tmp).next;
    }
    free(ident as *mut ::core::ffi::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "920:1"]
pub unsafe extern "C" fn hba_free(mut hba: *mut HBA) {
    let mut el = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut rule = ::core::ptr::null_mut::<HBARule>();
    if hba.is_null() {
        return;
    }
    el = (*hba).rules.next;
    tmp = (*(*hba).rules.next).next;
    while el != &raw mut (*hba).rules {
        rule = (el as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut HBARule;
        list_del(&raw mut (*rule).node);
        rule_free(rule);
        el = tmp;
        tmp = (*tmp).next;
    }
    free(hba as *mut ::core::ffi::c_void);
}
#[c2rust::src_loc = "934:1"]
unsafe extern "C" fn name_match(
    mut hname: *mut HBAName,
    mut name: *const ::core::ffi::c_char,
    mut namelen: ::core::ffi::c_uint,
    mut pair: *const ::core::ffi::c_char,
) -> bool {
    if (*hname).flags & NAME_ALL as ::core::ffi::c_uint != 0 {
        return true_0 != 0;
    }
    if (*hname).flags & NAME_SAMEUSER as ::core::ffi::c_uint != 0
        && strcmp(name, pair) == 0 as ::core::ffi::c_int
    {
        return true_0 != 0;
    }
    if !(*hname).name_set.is_null() {
        return strset_contains((*hname).name_set as *mut StrSet, name, namelen);
    }
    false_0 != 0
}
#[c2rust::src_loc = "945:1"]
unsafe extern "C" fn match_inet4(mut haddress: *const HBAAddress, mut addr: *mut PgAddr) -> bool {
    let mut src = ::core::ptr::null::<uint32_t>();
    let mut base = ::core::ptr::null::<uint32_t>();
    let mut mask = ::core::ptr::null::<uint32_t>();
    if pga_family(addr) != AF_INET as ::core::ffi::c_uint {
        return false_0 != 0;
    }
    src = &raw mut (*addr).sin.sin_addr.s_addr as *mut uint32_t;
    base = &raw const (*haddress).addr as *const uint8_t as *mut uint32_t;
    mask = &raw const (*haddress).mask as *const uint8_t as *mut uint32_t;
    *src.offset(0 as ::core::ffi::c_int as isize) & *mask.offset(0 as ::core::ffi::c_int as isize)
        == *base.offset(0 as ::core::ffi::c_int as isize)
}
#[c2rust::src_loc = "956:1"]
unsafe extern "C" fn match_inet6(mut haddress: *const HBAAddress, mut addr: *mut PgAddr) -> bool {
    let mut src = ::core::ptr::null::<uint32_t>();
    let mut base = ::core::ptr::null::<uint32_t>();
    let mut mask = ::core::ptr::null::<uint32_t>();
    if pga_family(addr) != AF_INET6 as ::core::ffi::c_uint {
        return false_0 != 0;
    }
    src = &raw mut (*addr).sin6.sin6_addr.__u6_addr.__u6_addr8 as *mut __uint8_t as *mut uint32_t;
    base = &raw const (*haddress).addr as *const uint8_t as *mut uint32_t;
    mask = &raw const (*haddress).mask as *const uint8_t as *mut uint32_t;
    *src.offset(0 as ::core::ffi::c_int as isize) & *mask.offset(0 as ::core::ffi::c_int as isize)
        == *base.offset(0 as ::core::ffi::c_int as isize)
        && *src.offset(1 as ::core::ffi::c_int as isize)
            & *mask.offset(1 as ::core::ffi::c_int as isize)
            == *base.offset(1 as ::core::ffi::c_int as isize)
        && *src.offset(2 as ::core::ffi::c_int as isize)
            & *mask.offset(2 as ::core::ffi::c_int as isize)
            == *base.offset(2 as ::core::ffi::c_int as isize)
        && *src.offset(3 as ::core::ffi::c_int as isize)
            & *mask.offset(3 as ::core::ffi::c_int as isize)
            == *base.offset(3 as ::core::ffi::c_int as isize)
}
#[c2rust::src_loc = "968:1"]
unsafe extern "C" fn address_match(mut haddress: *const HBAAddress, mut addr: *mut PgAddr) -> bool {
    if (*haddress).flags & ADDRESS_ALL as ::core::ffi::c_uint != 0 {
        return true_0 != 0;
    }
    match (*haddress).family {
        AF_INET => match_inet4(haddress, addr),
        AF_INET6 => match_inet6(haddress, addr),
        _ => false_0 != 0,
    }
}
#[no_mangle]
#[c2rust::src_loc = "982:1"]
pub unsafe extern "C" fn hba_eval(
    mut hba: *mut HBA,
    mut addr: *mut PgAddr,
    mut is_tls: bool,
    mut replication: ReplicationType,
    mut dbname: *const ::core::ffi::c_char,
    mut username: *const ::core::ffi::c_char,
) -> *mut HBARule {
    let mut el = ::core::ptr::null_mut::<List>();
    let mut rule = ::core::ptr::null_mut::<HBARule>();
    let mut dbnamelen = strlen(dbname) as ::core::ffi::c_uint;
    let mut unamelen = strlen(username) as ::core::ffi::c_uint;
    if hba.is_null() {
        return ::core::ptr::null_mut::<HBARule>();
    }
    let mut current_block_4: u64;
    el = (*hba).rules.next;
    while el != &raw mut (*hba).rules {
        rule = (el as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut HBARule;
        if pga_is_unix(addr) {
            if (*rule).rule_type as ::core::ffi::c_uint
                != RULE_LOCAL as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                current_block_4 = 10680521327981672866;
            } else {
                current_block_4 = 3512920355445576850;
            }
        } else if (*rule).rule_type as ::core::ffi::c_uint
            == RULE_LOCAL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            current_block_4 = 10680521327981672866;
        } else if (*rule).rule_type as ::core::ffi::c_uint
            == RULE_HOSTSSL as ::core::ffi::c_int as ::core::ffi::c_uint
            && !is_tls
        {
            current_block_4 = 10680521327981672866;
        } else if (*rule).rule_type as ::core::ffi::c_uint
            == RULE_HOSTNOSSL as ::core::ffi::c_int as ::core::ffi::c_uint
            && is_tls as ::core::ffi::c_int != 0
        {
            current_block_4 = 10680521327981672866;
        } else if !address_match(&raw mut (*rule).address, addr) {
            current_block_4 = 10680521327981672866;
        } else {
            current_block_4 = 3512920355445576850;
        }
        if current_block_4 == 3512920355445576850 {
            if replication as ::core::ffi::c_uint
                == REPLICATION_PHYSICAL as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if (*rule).db_name.flags & NAME_REPLICATION as ::core::ffi::c_uint == 0 {
                    current_block_4 = 10680521327981672866;
                } else {
                    current_block_4 = 15904375183555213903;
                }
            } else if !name_match(&raw mut (*rule).db_name, dbname, dbnamelen, username) {
                current_block_4 = 10680521327981672866;
            } else {
                current_block_4 = 15904375183555213903;
            }
            match current_block_4 {
                10680521327981672866 => {}
                _ => {
                    if name_match(&raw mut (*rule).user_name, username, unamelen, dbname) {
                        return rule;
                    }
                }
            }
        }
        el = (*el).next;
    }
    ::core::ptr::null_mut::<HBARule>()
}
