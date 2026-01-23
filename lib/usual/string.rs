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
    #[c2rust::src_loc = "118:1"]
    pub type __darwin_ssize_t = isize;
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_rsize_t.h:19"]
pub mod _rsize_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type rsize_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_errno_t.h:19"]
pub mod _errno_t_h {
    #[c2rust::src_loc = "30:1"]
    pub type errno_t = ::core::ffi::c_int;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types.h:19"]
pub mod include__types_h {
    #[c2rust::src_loc = "43:1"]
    pub type __darwin_nl_item = ::core::ffi::c_int;
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
        #[c2rust::src_loc = "111:1"]
        pub fn cx_free(cx: *const CxMem, ptr: *mut ::core::ffi::c_void);
        #[c2rust::src_loc = "121:1"]
        pub fn cx_alloc0(cx: *const CxMem, len: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "127:1"]
        pub fn cx_strdup(
            cx: *const CxMem,
            str: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/string.h:19"]
pub mod string_h {
    #[c2rust::src_loc = "35:1"]
    pub type str_cb =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char) -> bool>;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/statlist.h:30"]
pub mod statlist_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:1"]
    pub struct StatList {
        pub head: List,
        pub cur_count: ::core::ffi::c_int,
    }
    #[inline]
    #[c2rust::src_loc = "62:1"]
    pub unsafe extern "C" fn statlist_append(mut list: *mut StatList, mut item: *mut List) {
        list_append(&raw mut (*list).head, item);
        (*list).cur_count += 1;
    }
    #[inline]
    #[c2rust::src_loc = "78:1"]
    pub unsafe extern "C" fn statlist_init(
        mut list: *mut StatList,
        mut _name: *const ::core::ffi::c_char,
    ) {
        list_init(&raw mut (*list).head);
        (*list).cur_count = 0 as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "95:1"]
    pub unsafe extern "C" fn statlist_pop(mut list: *mut StatList) -> *mut List {
        let mut item = list_pop(&raw mut (*list).head);
        if !item.is_null() {
            (*list).cur_count -= 1;
        }
        return item;
    }
    #[inline]
    #[c2rust::src_loc = "120:1"]
    pub unsafe extern "C" fn statlist_empty(mut list: *const StatList) -> bool {
        return list_empty(&raw const (*list).head) != 0;
    }
    use super::list_h::{list_append, list_empty, list_init, list_pop, List};
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/list.h:30"]
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
    #[c2rust::src_loc = "52:1"]
    pub unsafe extern "C" fn list_empty(mut list: *const List) -> ::core::ffi::c_int {
        return ((*list).next == list as *mut List) as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "68:1"]
    pub unsafe extern "C" fn list_append(mut list: *mut List, mut item: *mut List) -> *mut List {
        (*item).next = list;
        (*item).prev = (*list).prev;
        (*(*list).prev).next = item;
        (*list).prev = item;
        return item;
    }
    #[inline]
    #[c2rust::src_loc = "78:1"]
    pub unsafe extern "C" fn list_del(mut item: *mut List) -> *mut List {
        (*(*item).prev).next = (*item).next;
        (*(*item).next).prev = (*item).prev;
        (*item).prev = item;
        (*item).next = (*item).prev;
        return item;
    }
    #[inline]
    #[c2rust::src_loc = "87:1"]
    pub unsafe extern "C" fn list_pop(mut list: *mut List) -> *mut List {
        if list_empty(list) != 0 {
            return ::core::ptr::null_mut::<List>();
        }
        return list_del((*list).next);
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/mbuf.h:29"]
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
    #[inline]
    #[c2rust::src_loc = "55:1"]
    pub unsafe extern "C" fn mbuf_init_dynamic(mut buf: *mut MBuf) {
        (*buf).data = ::core::ptr::null_mut::<uint8_t>();
        (*buf).read_pos = 0 as ::core::ffi::c_uint;
        (*buf).write_pos = 0 as ::core::ffi::c_uint;
        (*buf).alloc_len = 0 as ::core::ffi::c_uint;
        (*buf).reader = false_0 != 0;
        (*buf).fixed = false_0 != 0;
    }
    #[inline]
    #[c2rust::src_loc = "66:1"]
    pub unsafe extern "C" fn mbuf_free(mut buf: *mut MBuf) {
        if !(*buf).data.is_null() {
            if !(*buf).fixed {
                free((*buf).data as *mut ::core::ffi::c_void);
            }
            memset(
                buf as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<MBuf>() as size_t,
            );
        }
    }
    #[inline]
    #[c2rust::src_loc = "86:1"]
    pub unsafe extern "C" fn mbuf_rewind_writer(mut buf: *mut MBuf) {
        if !(*buf).reader {
            (*buf).read_pos = 0 as ::core::ffi::c_uint;
            (*buf).write_pos = 0 as ::core::ffi::c_uint;
        }
    }
    #[inline]
    #[c2rust::src_loc = "252:1"]
    pub unsafe extern "C" fn mbuf_write_byte(mut buf: *mut MBuf, mut val: uint8_t) -> bool {
        if (*buf).write_pos.wrapping_add(1 as ::core::ffi::c_uint) > (*buf).alloc_len
            && !mbuf_make_room(buf, 1 as ::core::ffi::c_uint)
        {
            return false_0 != 0;
        }
        let fresh0 = (*buf).write_pos;
        (*buf).write_pos = (*buf).write_pos.wrapping_add(1);
        *(*buf).data.offset(fresh0 as isize) = val;
        return true_0 != 0;
    }
    #[inline]
    #[c2rust::src_loc = "263:1"]
    pub unsafe extern "C" fn mbuf_write(
        mut buf: *mut MBuf,
        mut ptr: *const ::core::ffi::c_void,
        mut len: ::core::ffi::c_uint,
    ) -> bool {
        if (*buf).write_pos.wrapping_add(len) > (*buf).alloc_len && !mbuf_make_room(buf, len) {
            return false_0 != 0;
        }
        if len > 0 as ::core::ffi::c_uint {
            memcpy(
                (*buf).data.offset((*buf).write_pos as isize) as *mut ::core::ffi::c_void,
                ptr,
                len as size_t,
            );
        }
        (*buf).write_pos = (*buf).write_pos.wrapping_add(len);
        return true_0 != 0;
    }
    use super::_malloc_h::free;

    use super::_size_t_h::size_t;
    use super::_string_h::{memcpy, memset};
    use super::_uint8_t_h::uint8_t;
    use super::stdbool_h::{false_0, true_0};
    extern "C" {
        #[c2rust::src_loc = "248:1"]
        pub fn mbuf_make_room(buf: *mut MBuf, len: ::core::ffi::c_uint) -> bool;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/runetype.h:31"]
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
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/bytemap.h:32"]
pub mod bytemap_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:1"]
    pub struct Bitmap256 {
        pub bmap: [uint32_t; 8],
    }
    #[c2rust::src_loc = "27:9"]
    pub const BITMAP256_SHIFT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    #[c2rust::src_loc = "28:9"]
    pub const BITMAP256_MASK: ::core::ffi::c_int =
        ((1 as ::core::ffi::c_int) << BITMAP256_SHIFT) - 1 as ::core::ffi::c_int;
    #[inline]
    #[c2rust::src_loc = "40:1"]
    pub unsafe extern "C" fn bitmap256_init(mut bmap: *mut Bitmap256) {
        memset(
            bmap as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Bitmap256>() as size_t,
        );
    }
    #[inline]
    #[c2rust::src_loc = "48:1"]
    pub unsafe extern "C" fn bitmap256_set(mut bmap: *mut Bitmap256, mut byte: uint8_t) {
        (*bmap).bmap[(byte as ::core::ffi::c_int >> BITMAP256_SHIFT) as usize] |=
            ((1 as ::core::ffi::c_int) << (byte as ::core::ffi::c_int & BITMAP256_MASK))
                as uint32_t;
    }
    #[inline]
    #[c2rust::src_loc = "56:1"]
    pub unsafe extern "C" fn bitmap256_is_set(
        mut bmap: *const Bitmap256,
        mut byte: uint8_t,
    ) -> bool {
        return (*bmap).bmap[(byte as ::core::ffi::c_int >> BITMAP256_SHIFT) as usize]
            & ((1 as ::core::ffi::c_int) << (byte as ::core::ffi::c_int & BITMAP256_MASK))
                as uint32_t
            != 0;
    }
    use super::_size_t_h::size_t;
    use super::_string_h::memset;
    use super::_uint32_t_h::uint32_t;
    use super::_uint8_t_h::uint8_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_locale_t.h:21"]
pub mod _locale_t_h {
    #[c2rust::src_loc = "37:1"]
    pub type locale_t = *mut _xlocale;
    extern "C" {
        #[c2rust::src_loc = "36:1"]
        pub type _xlocale;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_nl_item.h:26"]
pub mod _nl_item_h {
    #[c2rust::src_loc = "32:1"]
    pub type nl_item = __darwin_nl_item;
    use super::include__types_h::__darwin_nl_item;
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:19"]
pub mod _string_h {
    use super::_errno_t_h::errno_t;
    use super::_rsize_t_h::rsize_t;
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn memchr(
            __s: *const ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
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
        #[c2rust::src_loc = "80:1"]
        pub fn memmove(
            __dst: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __len: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "83:1"]
        pub fn memset(
            __b: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __len: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "89:1"]
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "96:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
        #[c2rust::src_loc = "108:1"]
        pub fn strrchr(
            __s: *const ::core::ffi::c_char,
            __c: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "139:1"]
        pub fn strerror_r(
            __errnum: ::core::ffi::c_int,
            __strerrbuf: *mut ::core::ffi::c_char,
            __buflen: size_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "164:1"]
        pub fn strnlen(__s1: *const ::core::ffi::c_char, __n: size_t) -> size_t;
        #[c2rust::src_loc = "176:1"]
        pub fn memset_s(
            __s: *mut ::core::ffi::c_void,
            __smax: rsize_t,
            __c: ::core::ffi::c_int,
            __n: rsize_t,
        ) -> errno_t;
        #[c2rust::src_loc = "202:1"]
        pub fn strlcpy(
            __dst: *mut ::core::ffi::c_char,
            __source: *const ::core::ffi::c_char,
            __size: size_t,
        ) -> ::core::ffi::c_ulong;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:19"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/ctype.h:31"]
pub mod ctype_h {
    #[inline]
    #[c2rust::src_loc = "105:1"]
    pub unsafe extern "C" fn safe_isspace(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        return isspace(c as ::core::ffi::c_uchar as ::core::ffi::c_int);
    }
    use super::_ctype_h::isspace;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_ctype.h:31"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/xlocale/_stdlib.h:23"]
pub mod _stdlib_h {
    use super::_locale_t_h::locale_t;
    extern "C" {
        #[c2rust::src_loc = "48:1"]
        pub fn strtod_l(
            _: *const ::core::ffi::c_char,
            _: *mut *mut ::core::ffi::c_char,
            _: locale_t,
        ) -> ::core::ffi::c_double;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_locale.h:21"]
pub mod _locale_h {
    #[c2rust::src_loc = "75:9"]
    pub const LC_ALL_MASK: ::core::ffi::c_int = LC_COLLATE_MASK
        | LC_CTYPE_MASK
        | LC_MESSAGES_MASK
        | LC_MONETARY_MASK
        | LC_NUMERIC_MASK
        | LC_TIME_MASK;
    #[c2rust::src_loc = "81:9"]
    pub const LC_COLLATE_MASK: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "82:9"]
    pub const LC_CTYPE_MASK: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "83:9"]
    pub const LC_MESSAGES_MASK: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "84:9"]
    pub const LC_MONETARY_MASK: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "85:9"]
    pub const LC_NUMERIC_MASK: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
    #[c2rust::src_loc = "86:9"]
    pub const LC_TIME_MASK: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
    use super::_locale_t_h::locale_t;
    extern "C" {
        #[c2rust::src_loc = "100:1"]
        pub fn newlocale(
            _: ::core::ffi::c_int,
            _: *const ::core::ffi::c_char,
            _: locale_t,
        ) -> locale_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_langinfo.h:26"]
pub mod _langinfo_h {
    #[c2rust::src_loc = "98:9"]
    pub const RADIXCHAR: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    use super::_nl_item_h::nl_item;
    extern "C" {
        #[c2rust::src_loc = "116:1"]
        pub fn nl_langinfo(_: nl_item) -> *mut ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:19"]
pub mod sys__types_h {
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:19"]
pub mod _malloc_h {
    extern "C" {
        #[c2rust::src_loc = "56:1"]
        pub fn free(_: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdlib.h:19"]
pub mod include__stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "184:1"]
        pub fn strtod(
            _: *const ::core::ffi::c_char,
            _: *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_double;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/errno.h:19"]
pub mod errno_h {
    #[c2rust::src_loc = "112:9"]
    pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
    #[c2rust::src_loc = "126:9"]
    pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
    #[c2rust::src_loc = "181:9"]
    pub const ENAMETOOLONG: ::core::ffi::c_int = 63 as ::core::ffi::c_int;
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
pub use self::_ctype_h::{__istype, __maskrune, isascii, isspace, _CTYPE_S};
pub use self::_errno_t_h::errno_t;
pub use self::_langinfo_h::{nl_langinfo, RADIXCHAR};
pub use self::_locale_h::{
    newlocale, LC_ALL_MASK, LC_COLLATE_MASK, LC_CTYPE_MASK, LC_MESSAGES_MASK, LC_MONETARY_MASK,
    LC_NUMERIC_MASK, LC_TIME_MASK,
};
pub use self::_locale_t_h::{_xlocale, locale_t};

pub use self::_nl_item_h::nl_item;
pub use self::_null_h::NULL;
pub use self::_rsize_t_h::rsize_t;
pub use self::_size_t_h::size_t;
pub use self::_ssize_t_h::ssize_t;
use self::_stdio_h::snprintf;
use self::_stdlib_h::strtod_l;
use self::_string_h::{
    memchr, memcmp, memcpy, memmove, memset_s, strcmp, strerror_r, strlcpy, strlen, strnlen,
    strrchr,
};
pub use self::_types_h::{
    __darwin_ct_rune_t, __darwin_rune_t, __darwin_size_t, __darwin_ssize_t, __darwin_wchar_t,
    __uint32_t,
};
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::bytemap_h::{
    bitmap256_init, bitmap256_is_set, bitmap256_set, Bitmap256, BITMAP256_MASK, BITMAP256_SHIFT,
};
pub use self::ctype_h::safe_isspace;
pub use self::cxalloc_h::{cx_alloc, cx_alloc0, cx_free, cx_strdup, CxMem, CxOps};
pub use self::errno_h::{__error, EINVAL, ENAMETOOLONG, ERANGE};
use self::include__stdlib_h::strtod;
pub use self::include__types_h::__darwin_nl_item;
pub use self::list_h::{list_append, list_del, list_empty, list_init, list_pop, List};
pub use self::mbuf_h::{
    mbuf_free, mbuf_init_dynamic, mbuf_make_room, mbuf_rewind_writer, mbuf_write, mbuf_write_byte,
    MBuf,
};
pub use self::runetype_h::{
    _DefaultRuneLocale, _RuneCharClass, _RuneEntry, _RuneLocale, _RuneRange,
};
pub use self::statlist_h::{
    statlist_append, statlist_empty, statlist_init, statlist_pop, StatList,
};
pub use self::stdbool_h::{false_0, true_0};
pub use self::string_h::str_cb;
pub use self::sys__types_h::__DARWIN_NULL;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "38:1"]
pub struct StrList {
    pub list: StatList,
    pub ca: *const CxMem,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "43:1"]
pub struct StrItem {
    pub node: List,
    pub str_0: *mut ::core::ffi::c_char,
}
#[no_mangle]
#[c2rust::src_loc = "48:1"]
pub unsafe extern "C" fn strlist_empty(mut slist: *mut StrList) -> bool {
    return statlist_empty(&raw mut (*slist).list);
}
#[no_mangle]
#[c2rust::src_loc = "53:1"]
pub unsafe extern "C" fn strlist_append(
    mut slist: *mut StrList,
    mut str: *const ::core::ffi::c_char,
) -> bool {
    let mut nstr = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ok: bool = false;
    if !str.is_null() {
        nstr = cx_strdup((*slist).ca, str) as *mut ::core::ffi::c_char;
        if nstr.is_null() {
            return false_0 != 0;
        }
    }
    ok = strlist_append_ref(slist, nstr);
    if !ok {
        cx_free((*slist).ca, nstr as *mut ::core::ffi::c_void);
    }
    return ok;
}
#[no_mangle]
#[c2rust::src_loc = "68:1"]
pub unsafe extern "C" fn strlist_append_ref(
    mut slist: *mut StrList,
    mut str: *mut ::core::ffi::c_char,
) -> bool {
    let mut item =
        cx_alloc((*slist).ca, ::core::mem::size_of::<StrItem>() as size_t) as *mut StrItem;
    if item.is_null() {
        return false_0 != 0;
    }
    list_init(&raw mut (*item).node);
    (*item).str_0 = str;
    statlist_append(&raw mut (*slist).list, &raw mut (*item).node);
    return true_0 != 0;
}
#[no_mangle]
#[c2rust::src_loc = "79:1"]
pub unsafe extern "C" fn strlist_pop(mut slist: *mut StrList) -> *mut ::core::ffi::c_char {
    let mut item = ::core::ptr::null_mut::<StrItem>();
    let mut el = ::core::ptr::null_mut::<List>();
    let mut str = ::core::ptr::null_mut::<::core::ffi::c_char>();
    el = statlist_pop(&raw mut (*slist).list);
    if el.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    item = (el as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
        as *mut StrItem;
    str = (*item).str_0;
    cx_free((*slist).ca, item as *mut ::core::ffi::c_void);
    return str;
}
#[no_mangle]
#[c2rust::src_loc = "95:1"]
pub unsafe extern "C" fn strlist_new(mut ca: *const CxMem) -> *mut StrList {
    let mut slist = cx_alloc0(ca, ::core::mem::size_of::<StrList>() as size_t) as *mut StrList;
    if slist.is_null() {
        return ::core::ptr::null_mut::<StrList>();
    }
    statlist_init(
        &raw mut (*slist).list,
        b"strlist\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*slist).ca = ca;
    return slist;
}
#[no_mangle]
#[c2rust::src_loc = "105:1"]
pub unsafe extern "C" fn strlist_free(mut slist: *mut StrList) {
    let mut s = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if slist.is_null() {
        return;
    }
    while !strlist_empty(slist) {
        s = strlist_pop(slist);
        if !s.is_null() {
            cx_free((*slist).ca, s as *mut ::core::ffi::c_void);
        }
    }
    cx_free((*slist).ca, slist as *mut ::core::ffi::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "118:1"]
pub unsafe extern "C" fn strlist_foreach(
    mut slist: *const StrList,
    mut func: str_cb,
    mut arg: *mut ::core::ffi::c_void,
) -> bool {
    let mut el = ::core::ptr::null_mut::<List>();
    let mut item = ::core::ptr::null_mut::<StrItem>();
    el = (*slist).list.head.next;
    while el != &raw const (*slist).list.head as *mut List {
        item = (el as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut StrItem;
        if !func.expect("non-null function pointer")(arg, (*item).str_0) {
            return false_0 != 0;
        }
        el = (*el).next;
    }
    return true_0 != 0;
}
#[inline]
#[c2rust::src_loc = "134:1"]
unsafe extern "C" fn skip_ws(mut p: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char {
    while *p as ::core::ffi::c_int != 0 && safe_isspace(*p as ::core::ffi::c_int) != 0 {
        p = p.offset(1);
    }
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "142:1"]
pub unsafe extern "C" fn parse_word_list(
    mut s: *const ::core::ffi::c_char,
    mut cb_func: str_cb,
    mut cb_arg: *mut ::core::ffi::c_void,
) -> bool {
    let mut current_block: u64;
    let mut buf = MBuf {
        data: ::core::ptr::null_mut::<uint8_t>(),
        read_pos: 0,
        write_pos: 0,
        alloc_len: 0,
        reader: false,
        fixed: false,
    };
    let mut p = s;
    let mut start = ::core::ptr::null::<::core::ffi::c_char>();
    let mut end = ::core::ptr::null::<::core::ffi::c_char>();
    mbuf_init_dynamic(&raw mut buf);
    loop {
        if !(*p != 0) {
            current_block = 7149356873433890176;
            break;
        }
        p = skip_ws(p);
        start = p;
        while *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int != ',' as i32 {
            p = p.offset(1);
        }
        end = p;
        while end > start
            && safe_isspace(*end.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int)
                != 0
        {
            end = end.offset(-1);
        }
        if *p != 0 {
            if !(*p as ::core::ffi::c_int == ',' as i32) {
                current_block = 4041987952866317976;
                break;
            }
            p = p.offset(1);
        }
        if !mbuf_write(
            &raw mut buf,
            start as *const ::core::ffi::c_void,
            end.offset_from(start) as ::core::ffi::c_long as ::core::ffi::c_uint,
        ) {
            current_block = 16309619385779902555;
            break;
        }
        if !mbuf_write_byte(&raw mut buf, 0 as uint8_t) {
            current_block = 16309619385779902555;
            break;
        }
        if !cb_func.expect("non-null function pointer")(
            cb_arg,
            buf.data as *const ::core::ffi::c_char,
        ) {
            current_block = 16309619385779902555;
            break;
        }
        mbuf_rewind_writer(&raw mut buf);
    }
    match current_block {
        7149356873433890176 => {
            mbuf_free(&raw mut buf);
            return true_0 != 0;
        }
        4041987952866317976 => {
            *__error() = EINVAL;
        }
        _ => {}
    }
    mbuf_free(&raw mut buf);
    return false_0 != 0;
}
#[no_mangle]
#[c2rust::src_loc = "219:1"]
pub unsafe extern "C" fn usual_strpcpy(
    mut dst: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut n: size_t,
) -> *mut ::core::ffi::c_char {
    if n == 0 as size_t {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    while n > 0 as size_t {
        *dst = *src;
        if *dst as ::core::ffi::c_int == '\0' as i32 {
            return dst;
        }
        n = n.wrapping_sub(1);
        dst = dst.offset(1);
        src = src.offset(1);
    }
    *dst.offset(-(1 as ::core::ffi::c_int) as isize) = '\0' as i32 as ::core::ffi::c_char;
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
#[c2rust::src_loc = "231:1"]
pub unsafe extern "C" fn usual_strpcat(
    mut dst: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut n: size_t,
) -> *mut ::core::ffi::c_char {
    let mut dstlen = strnlen(dst, n);
    if dstlen < n {
        return usual_strpcpy(dst.offset(dstlen as isize), src, n.wrapping_sub(dstlen));
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
#[c2rust::src_loc = "240:1"]
pub unsafe extern "C" fn usual_mempcpy(
    mut dst: *mut ::core::ffi::c_void,
    mut src: *const ::core::ffi::c_void,
    mut n: size_t,
) -> *mut ::core::ffi::c_void {
    memcpy(dst, src, n);
    return (dst as *mut ::core::ffi::c_char).offset(n as isize) as *mut ::core::ffi::c_void;
}
#[no_mangle]
#[c2rust::src_loc = "248:1"]
pub unsafe extern "C" fn usual_memrchr(
    mut s: *const ::core::ffi::c_void,
    mut c: ::core::ffi::c_int,
    mut n: size_t,
) -> *mut ::core::ffi::c_void {
    let mut p = s as *const uint8_t;
    loop {
        let fresh1 = n;
        n = n.wrapping_sub(1);
        if !(fresh1 != 0) {
            break;
        }
        if *p.offset(n as isize) as ::core::ffi::c_int == c {
            return p.offset(n as isize) as *mut ::core::ffi::c_void;
        }
    }
    return NULL;
}
#[no_mangle]
#[c2rust::src_loc = "295:1"]
pub unsafe extern "C" fn usual_explicit_bzero(mut buf: *mut ::core::ffi::c_void, mut len: size_t) {
    memset_s(buf, len as rsize_t, 0 as ::core::ffi::c_int, len as rsize_t);
}
#[no_mangle]
#[c2rust::src_loc = "326:1"]
pub unsafe extern "C" fn usual_basename(
    mut path: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut p = ::core::ptr::null::<::core::ffi::c_char>();
    let mut p2 = ::core::ptr::null::<::core::ffi::c_char>();
    static mut buf: [::core::ffi::c_char; 256] = [0; 256];
    let mut len: ::core::ffi::c_uint = 0;
    if path.is_null()
        || *path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        return memcpy(
            &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            b".\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            2 as size_t,
        ) as *const ::core::ffi::c_char;
    }
    p = strrchr(path, '/' as i32);
    if p.is_null() {
        return path;
    }
    if *p.offset(1 as ::core::ffi::c_int as isize) != 0 {
        return p.offset(1 as ::core::ffi::c_int as isize);
    }
    p2 = p;
    while p2 > path {
        if *p2.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int != '/' as i32 {
            len = p2.offset_from(path) as ::core::ffi::c_long as ::core::ffi::c_uint;
            if len as usize
                > (::core::mem::size_of::<[::core::ffi::c_char; 256]>() as usize)
                    .wrapping_sub(1 as usize)
            {
                len = (::core::mem::size_of::<[::core::ffi::c_char; 256]>() as usize)
                    .wrapping_sub(1 as usize) as ::core::ffi::c_uint;
            }
            memcpy(
                &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                p2.offset(-(len as isize)) as *const ::core::ffi::c_void,
                len as size_t,
            );
            buf[len as usize] = 0 as ::core::ffi::c_char;
            return usual_basename(&raw mut buf as *mut ::core::ffi::c_char);
        }
        p2 = p2.offset(-1);
    }
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "356:1"]
pub unsafe extern "C" fn usual_dirname(
    mut path: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut p = ::core::ptr::null::<::core::ffi::c_char>();
    let mut len: size_t = 0;
    static mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
    if path.is_null()
        || *path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        return memcpy(
            &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            b".\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            2 as size_t,
        ) as *const ::core::ffi::c_char;
    }
    len = strlen(path);
    while len != 0
        && *path.offset(len.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int == '/' as i32
    {
        len = len.wrapping_sub(1);
    }
    if len == 0 {
        return memcpy(
            &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            b"/\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            2 as size_t,
        ) as *const ::core::ffi::c_char;
    }
    p = usual_memrchr(path as *const ::core::ffi::c_void, '/' as i32, len)
        as *const ::core::ffi::c_char;
    if p.is_null() {
        return memcpy(
            &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            b".\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            2 as size_t,
        ) as *const ::core::ffi::c_char;
    }
    len = p.offset_from(path) as ::core::ffi::c_long as size_t;
    while len != 0
        && *path.offset(len.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int == '/' as i32
    {
        len = len.wrapping_sub(1);
    }
    if len == 0 {
        return memcpy(
            &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            b"/\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            2 as size_t,
        ) as *const ::core::ffi::c_char;
    }
    if len
        > (::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as usize).wrapping_sub(1 as usize)
    {
        *__error() = ENAMETOOLONG;
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    memcpy(
        &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        path as *const ::core::ffi::c_void,
        len,
    );
    buf[len as usize] = 0 as ::core::ffi::c_char;
    return &raw mut buf as *mut ::core::ffi::c_char;
}
#[no_mangle]
#[c2rust::src_loc = "403:1"]
pub unsafe extern "C" fn usual_strerror_r(
    mut e: ::core::ffi::c_int,
    mut dst: *mut ::core::ffi::c_char,
    mut dstlen: size_t,
) -> *const ::core::ffi::c_char {
    if strerror_r(e, dst, dstlen) != 0 as ::core::ffi::c_int {
        strlcpy(
            dst,
            b"ERR\0" as *const u8 as *const ::core::ffi::c_char,
            dstlen,
        );
    }
    return dst;
}
#[no_mangle]
#[c2rust::src_loc = "423:1"]
pub unsafe extern "C" fn mempbrk(
    mut data: *const ::core::ffi::c_void,
    mut dlen: size_t,
    mut find: *const ::core::ffi::c_void,
    mut flen: size_t,
) -> *mut ::core::ffi::c_void {
    let mut s = data as *const uint8_t;
    let mut fb = find as *const uint8_t;
    let mut i: size_t = 0;
    let mut bmap = Bitmap256 { bmap: [0; 8] };
    if flen == 0 as size_t {
        return NULL;
    }
    if flen == 1 as size_t {
        return memchr(
            data,
            *fb.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            dlen,
        );
    }
    bitmap256_init(&raw mut bmap);
    i = 0 as size_t;
    while i < flen {
        bitmap256_set(&raw mut bmap, *fb.offset(i as isize));
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < dlen {
        if bitmap256_is_set(&raw mut bmap, *s.offset(i as isize)) {
            return s.offset(i as isize) as *mut ::core::ffi::c_void;
        }
        i = i.wrapping_add(1);
    }
    return NULL;
}
#[no_mangle]
#[c2rust::src_loc = "445:1"]
pub unsafe extern "C" fn memspn(
    mut data: *const ::core::ffi::c_void,
    mut dlen: size_t,
    mut accept: *const ::core::ffi::c_void,
    mut alen: size_t,
) -> size_t {
    let mut s = data as *const uint8_t;
    let mut fb = accept as *const uint8_t;
    let mut i: size_t = 0;
    let mut bmap = Bitmap256 { bmap: [0; 8] };
    if alen == 0 as size_t {
        return 0 as size_t;
    }
    if alen == 1 as size_t {
        i = 0 as size_t;
        while i < dlen {
            if *s.offset(i as isize) as ::core::ffi::c_int
                != *fb.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            {
                break;
            }
            i = i.wrapping_add(1);
        }
        return i;
    }
    bitmap256_init(&raw mut bmap);
    i = 0 as size_t;
    while i < alen {
        bitmap256_set(&raw mut bmap, *fb.offset(i as isize));
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < dlen {
        if !bitmap256_is_set(&raw mut bmap, *s.offset(i as isize)) {
            break;
        }
        i = i.wrapping_add(1);
    }
    return i;
}
#[no_mangle]
#[c2rust::src_loc = "471:1"]
pub unsafe extern "C" fn memcspn(
    mut data: *const ::core::ffi::c_void,
    mut dlen: size_t,
    mut reject: *const ::core::ffi::c_void,
    mut rlen: size_t,
) -> size_t {
    let mut p = ::core::ptr::null::<::core::ffi::c_void>();
    p = mempbrk(data, dlen, reject, rlen);
    if !p.is_null() {
        return (p as *mut ::core::ffi::c_char).offset_from(data as *mut ::core::ffi::c_char)
            as ::core::ffi::c_long as size_t;
    }
    return dlen;
}
#[no_mangle]
#[c2rust::src_loc = "481:1"]
pub unsafe extern "C" fn strtod_dot(
    mut s: *const ::core::ffi::c_char,
    mut tokend: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_double {
    let mut dp = ::core::ptr::null::<::core::ffi::c_char>();
    let mut buf: [::core::ffi::c_char; 128] = [0; 128];
    let mut dst = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut tmp = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut end = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut dot = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_uint = 0;
    let mut dplen: ::core::ffi::c_uint = 0;
    let mut res: ::core::ffi::c_double = 0.;
    dp = nl_langinfo(RADIXCHAR);
    if memcmp(
        dp as *const ::core::ffi::c_void,
        b".\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        2 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        return strtod(s, tokend);
    }
    static mut c_locale: locale_t = ::core::ptr::null::<_xlocale>() as *mut _xlocale;
    if c_locale.is_null() {
        c_locale = newlocale(
            LC_ALL_MASK,
            b"C\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<_xlocale>(),
        );
    }
    if !c_locale.is_null() {
        return strtod_l(s, tokend, c_locale);
    }
    while *s as ::core::ffi::c_int != 0 && safe_isspace(*s as ::core::ffi::c_int) != 0 {
        s = s.offset(1);
    }
    dot = ::core::ptr::null_mut::<::core::ffi::c_char>();
    dst = &raw mut buf as *mut ::core::ffi::c_char;
    end = (&raw mut buf as *mut ::core::ffi::c_char)
        .offset(::core::mem::size_of::<[::core::ffi::c_char; 128]>() as usize as isize)
        .offset(-(5 as ::core::ffi::c_int as isize));
    dplen = (if *dp.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0 {
        strlen(dp)
    } else {
        1 as size_t
    }) as ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_uint;
    while *s.offset(i as isize) != 0 {
        if *s.offset(i as isize) as ::core::ffi::c_int >= '0' as i32
            && *s.offset(i as isize) as ::core::ffi::c_int <= '9' as i32
        {
            let fresh2 = dst;
            dst = dst.offset(1);
            *fresh2 = *s.offset(i as isize);
        } else if *s.offset(i as isize) as ::core::ffi::c_int == '.' as i32 {
            dot = dst;
            memcpy(
                dst as *mut ::core::ffi::c_void,
                dp as *const ::core::ffi::c_void,
                dplen as size_t,
            );
            dst = dst.offset(dplen as isize);
        } else {
            if !(*s.offset(i as isize) as ::core::ffi::c_int == '-' as i32
                || *s.offset(i as isize) as ::core::ffi::c_int == '+' as i32
                || *s.offset(i as isize) as ::core::ffi::c_int == 'e' as i32
                || *s.offset(i as isize) as ::core::ffi::c_int == 'E' as i32)
            {
                break;
            }
            let fresh3 = dst;
            dst = dst.offset(1);
            *fresh3 = *s.offset(i as isize);
        }
        if dst >= end {
            *__error() = ERANGE;
            return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        i = i.wrapping_add(1);
    }
    *dst = '\0' as i32 as ::core::ffi::c_char;
    if dot.is_null() {
        return strtod(s, tokend);
    }
    tmp = ::core::ptr::null_mut::<::core::ffi::c_char>();
    res = strtod(&raw mut buf as *mut ::core::ffi::c_char, &raw mut tmp);
    if !tmp.is_null() && !tokend.is_null() {
        *tokend =
            (s as *mut ::core::ffi::c_char)
                .offset(tmp.offset_from(&raw mut buf as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as isize);
        if !dot.is_null() && tmp > dot && dplen > 1 as ::core::ffi::c_uint {
            *tokend = (*tokend).offset(-(dplen.wrapping_sub(1 as ::core::ffi::c_uint) as isize));
        }
    }
    return res;
}
#[no_mangle]
#[c2rust::src_loc = "550:1"]
pub unsafe extern "C" fn dtostr_dot(
    mut buf: *mut ::core::ffi::c_char,
    mut buflen: size_t,
    mut val: ::core::ffi::c_double,
) -> ssize_t {
    let mut dp = ::core::ptr::null::<::core::ffi::c_char>();
    let mut len: ssize_t = 0;
    let mut dplen: ssize_t = 0;
    let mut p = ::core::ptr::null_mut::<::core::ffi::c_char>();
    len = snprintf(
        buf,
        buflen,
        b"%.17g\0" as *const u8 as *const ::core::ffi::c_char,
        val,
    ) as ssize_t;
    if len >= buflen as ssize_t || len < 0 as ssize_t {
        return len;
    }
    dp = nl_langinfo(RADIXCHAR);
    if memcmp(
        dp as *const ::core::ffi::c_void,
        b".\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        2 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        return len;
    }
    dplen = (if *dp.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0 {
        strlen(dp)
    } else {
        1 as size_t
    }) as ssize_t;
    p = memchr(
        buf as *const ::core::ffi::c_void,
        *dp.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        len as size_t,
    ) as *mut ::core::ffi::c_char;
    if !p.is_null() {
        *p = '.' as i32 as ::core::ffi::c_char;
        if *dp.offset(1 as ::core::ffi::c_int as isize) != 0 {
            memmove(
                p.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
                p.offset(dplen as isize) as *const ::core::ffi::c_void,
                strlen(p.offset(dplen as isize)).wrapping_add(1 as size_t),
            );
            len -= dplen - 1 as ssize_t;
        }
    }
    return len;
}
#[no_mangle]
#[c2rust::src_loc = "688:1"]
pub unsafe extern "C" fn strcmpeq(
    mut str_left: *const ::core::ffi::c_char,
    mut str_right: *const ::core::ffi::c_char,
) -> bool {
    if str_left.is_null() && str_right.is_null() {
        return true_0 != 0;
    }
    if str_left.is_null() || str_right.is_null() {
        return false_0 != 0;
    }
    return strcmp(str_left, str_right) == 0 as ::core::ffi::c_int;
}
