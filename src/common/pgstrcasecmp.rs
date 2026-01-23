#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:28"]
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
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:28"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/runetype.h:28"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_ctype.h:28"]
pub mod _ctype_h {
    #[c2rust::src_loc = "83:9"]
    pub const _CTYPE_U: ::core::ffi::c_long = 0x8000 as ::core::ffi::c_long;
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
    #[c2rust::src_loc = "277:1"]
    pub unsafe extern "C" fn isupper(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        return __istype(_c as __darwin_ct_rune_t, _CTYPE_U as ::core::ffi::c_ulong);
    }
    #[inline]
    #[c2rust::src_loc = "296:1"]
    pub unsafe extern "C" fn tolower(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        return __tolower(_c as __darwin_ct_rune_t) as ::core::ffi::c_int;
    }
    use super::_types_h::{__darwin_ct_rune_t, __uint32_t};
    use super::runetype_h::_DefaultRuneLocale;
    extern "C" {
        #[c2rust::src_loc = "153:1"]
        pub fn __maskrune(_: __darwin_ct_rune_t, _: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "194:1"]
        pub fn __tolower(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/ctype.h:28"]
pub mod ctype_h {
    #[inline]
    #[c2rust::src_loc = "110:1"]
    pub unsafe extern "C" fn safe_isupper(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        return isupper(c as ::core::ffi::c_uchar as ::core::ffi::c_int);
    }
    #[inline]
    #[c2rust::src_loc = "120:1"]
    pub unsafe extern "C" fn safe_tolower(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        return tolower(c as ::core::ffi::c_uchar as ::core::ffi::c_int);
    }
    use super::_ctype_h::{isupper, tolower};
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/postgres_compat.h:28"]
pub mod postgres_compat_h {
    #[c2rust::src_loc = "32:9"]
    pub const HIGHBIT: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
}
pub use self::_ctype_h::{__istype, __maskrune, __tolower, isascii, isupper, tolower, _CTYPE_U};
pub use self::_size_t_h::size_t;
pub use self::_types_h::{
    __darwin_ct_rune_t, __darwin_rune_t, __darwin_size_t, __darwin_wchar_t, __uint32_t,
};
pub use self::ctype_h::{safe_isupper, safe_tolower};
pub use self::postgres_compat_h::HIGHBIT;
pub use self::runetype_h::{
    _DefaultRuneLocale, _RuneCharClass, _RuneEntry, _RuneLocale, _RuneRange,
};
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn pg_strncasecmp(
    mut s1: *const ::core::ffi::c_char,
    mut s2: *const ::core::ffi::c_char,
    mut n: size_t,
) -> ::core::ffi::c_int {
    loop {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 > 0 as size_t) {
            break;
        }
        let fresh1 = s1;
        s1 = s1.offset(1);
        let mut ch1 = *fresh1 as ::core::ffi::c_uchar;
        let fresh2 = s2;
        s2 = s2.offset(1);
        let mut ch2 = *fresh2 as ::core::ffi::c_uchar;
        if ch1 as ::core::ffi::c_int != ch2 as ::core::ffi::c_int {
            if ch1 as ::core::ffi::c_int >= 'A' as i32 && ch1 as ::core::ffi::c_int <= 'Z' as i32 {
                ch1 =
                    (ch1 as ::core::ffi::c_int + ('a' as i32 - 'A' as i32)) as ::core::ffi::c_uchar;
            } else if ch1 as ::core::ffi::c_int & HIGHBIT != 0
                && safe_isupper(ch1 as ::core::ffi::c_int) != 0
            {
                ch1 = safe_tolower(ch1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            }
            if ch2 as ::core::ffi::c_int >= 'A' as i32 && ch2 as ::core::ffi::c_int <= 'Z' as i32 {
                ch2 =
                    (ch2 as ::core::ffi::c_int + ('a' as i32 - 'A' as i32)) as ::core::ffi::c_uchar;
            } else if ch2 as ::core::ffi::c_int & HIGHBIT != 0
                && safe_isupper(ch2 as ::core::ffi::c_int) != 0
            {
                ch2 = safe_tolower(ch2 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            }
            if ch1 as ::core::ffi::c_int != ch2 as ::core::ffi::c_int {
                return ch1 as ::core::ffi::c_int - ch2 as ::core::ffi::c_int;
            }
        }
        if ch1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}
