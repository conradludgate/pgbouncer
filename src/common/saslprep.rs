pub mod _types_h {

    pub type __darwin_size_t = usize;
}

pub mod _size_t_h {

    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}

pub mod saslprep_h {

    pub type pg_saslprep_rc = ::core::ffi::c_int;

    pub const SASLPREP_PROHIBITED: pg_saslprep_rc = -3;

    pub const SASLPREP_INVALID_UTF8: pg_saslprep_rc = -2;

    pub const SASLPREP_OOM: pg_saslprep_rc = -1;

    pub const SASLPREP_SUCCESS: pg_saslprep_rc = 0;
}

pub mod pg_wchar_h {

    pub type pg_wchar = ::core::ffi::c_uint;
    #[inline]

    pub unsafe extern "C" fn utf8_to_unicode(mut c: *const ::core::ffi::c_uchar) -> pg_wchar {
        if *c as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            *c as pg_wchar
        } else if *c as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
            == 0xc0 as ::core::ffi::c_int
        {
            ((*c as ::core::ffi::c_int
                & 0x1f as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int
                | *c.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x3f as ::core::ffi::c_int) as pg_wchar
        } else if *c as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
            == 0xe0 as ::core::ffi::c_int
        {
            ((*c as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int)
                << 12 as ::core::ffi::c_int
                | (*c.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x3f as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int
                | *c.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x3f as ::core::ffi::c_int) as pg_wchar
        } else if *c as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int
            == 0xf0 as ::core::ffi::c_int
        {
            ((*c as ::core::ffi::c_int
                & 0x7 as ::core::ffi::c_int)
                << 18 as ::core::ffi::c_int
                | (*c.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x3f as ::core::ffi::c_int)
                    << 12 as ::core::ffi::c_int
                | (*c.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x3f as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int
                | *c.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x3f as ::core::ffi::c_int) as pg_wchar
        } else {
            0xffffffff as pg_wchar
        }
    }
    #[inline]

    pub unsafe extern "C" fn unicode_to_utf8(
        mut c: pg_wchar,
        mut utf8string: *mut ::core::ffi::c_uchar,
    ) -> *mut ::core::ffi::c_uchar {
        if c <= 0x7f as pg_wchar {
            *utf8string = c as ::core::ffi::c_uchar;
        } else if c <= 0x7ff as pg_wchar {
            *utf8string = (0xc0 as pg_wchar
                | c >> 6 as ::core::ffi::c_int & 0x1f as pg_wchar)
                as ::core::ffi::c_uchar;
            *utf8string.offset(1 as ::core::ffi::c_int as isize) =
                (0x80 as pg_wchar | c & 0x3f as pg_wchar) as ::core::ffi::c_uchar;
        } else if c <= 0xffff as pg_wchar {
            *utf8string = (0xe0 as pg_wchar
                | c >> 12 as ::core::ffi::c_int & 0xf as pg_wchar)
                as ::core::ffi::c_uchar;
            *utf8string.offset(1 as ::core::ffi::c_int as isize) = (0x80 as pg_wchar
                | c >> 6 as ::core::ffi::c_int & 0x3f as pg_wchar)
                as ::core::ffi::c_uchar;
            *utf8string.offset(2 as ::core::ffi::c_int as isize) =
                (0x80 as pg_wchar | c & 0x3f as pg_wchar) as ::core::ffi::c_uchar;
        } else {
            *utf8string = (0xf0 as pg_wchar
                | c >> 18 as ::core::ffi::c_int & 0x7 as pg_wchar)
                as ::core::ffi::c_uchar;
            *utf8string.offset(1 as ::core::ffi::c_int as isize) = (0x80 as pg_wchar
                | c >> 12 as ::core::ffi::c_int & 0x3f as pg_wchar)
                as ::core::ffi::c_uchar;
            *utf8string.offset(2 as ::core::ffi::c_int as isize) = (0x80 as pg_wchar
                | c >> 6 as ::core::ffi::c_int & 0x3f as pg_wchar)
                as ::core::ffi::c_uchar;
            *utf8string.offset(3 as ::core::ffi::c_int as isize) =
                (0x80 as pg_wchar | c & 0x3f as pg_wchar) as ::core::ffi::c_uchar;
        }
        utf8string
    }
    extern "C" {

        pub fn pg_utf8_islegal(
            source: *const ::core::ffi::c_uchar,
            length: ::core::ffi::c_int,
        ) -> bool;

        pub fn pg_utf_mblen(s: *const ::core::ffi::c_uchar) -> ::core::ffi::c_int;
    }
}

pub mod unicode_norm_h {

    pub type UnicodeNormalizationForm = ::core::ffi::c_uint;

    pub const UNICODE_NFKD: UnicodeNormalizationForm = 3;

    pub const UNICODE_NFKC: UnicodeNormalizationForm = 2;

    pub const UNICODE_NFD: UnicodeNormalizationForm = 1;

    pub const UNICODE_NFC: UnicodeNormalizationForm = 0;
    use super::pg_wchar_h::pg_wchar;
    extern "C" {

        pub fn unicode_normalize(
            form: UnicodeNormalizationForm,
            input: *const pg_wchar,
        ) -> *mut pg_wchar;
    }
}

pub mod _malloc_h {
    use super::_size_t_h::size_t;
    extern "C" {

        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;

        pub fn free(_: *mut ::core::ffi::c_void);
    }
}

pub mod _stdlib_h {
    use super::_size_t_h::size_t;
    extern "C" {

        pub fn bsearch(
            __key: *const ::core::ffi::c_void,
            __base: *const ::core::ffi::c_void,
            __nel: size_t,
            __width: size_t,
            __compar: Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >,
        ) -> *mut ::core::ffi::c_void;
    }
}

pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {

        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;

        pub fn strdup(__s1: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    }
}

pub mod _null_h {

    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}

pub mod postgres_compat_h {

    pub const MaxAllocSize: size_t = 0x3fffffff as ::core::ffi::c_int as size_t;
    use super::_size_t_h::size_t;
}

pub mod sys__types_h {

    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}

pub mod stdbool_h {

    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}

pub mod string_h {
    extern "C" {

        pub fn pg_is_ascii(str: *const ::core::ffi::c_char) -> bool;
    }
}
use self::_malloc_h::{free, malloc};
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
use self::_stdlib_h::bsearch;
use self::_string_h::{strdup, strlen};
pub use self::_types_h::__darwin_size_t;
pub use self::pg_wchar_h::{
    pg_utf8_islegal, pg_utf_mblen, pg_wchar, unicode_to_utf8, utf8_to_unicode,
};
pub use self::postgres_compat_h::MaxAllocSize;
pub use self::saslprep_h::{
    pg_saslprep_rc, SASLPREP_INVALID_UTF8, SASLPREP_OOM, SASLPREP_PROHIBITED, SASLPREP_SUCCESS,
};
pub use self::stdbool_h::{false_0, true_0};
use self::string_h::pg_is_ascii;
pub use self::sys__types_h::__DARWIN_NULL;
pub use self::unicode_norm_h::{
    unicode_normalize, UnicodeNormalizationForm, UNICODE_NFC, UNICODE_NFD, UNICODE_NFKC,
    UNICODE_NFKD,
};

static mut non_ascii_space_ranges: [pg_wchar; 12] = [
    0xa0 as ::core::ffi::c_int as pg_wchar,
    0xa0 as ::core::ffi::c_int as pg_wchar,
    0x1680 as ::core::ffi::c_int as pg_wchar,
    0x1680 as ::core::ffi::c_int as pg_wchar,
    0x2000 as ::core::ffi::c_int as pg_wchar,
    0x200b as ::core::ffi::c_int as pg_wchar,
    0x202f as ::core::ffi::c_int as pg_wchar,
    0x202f as ::core::ffi::c_int as pg_wchar,
    0x205f as ::core::ffi::c_int as pg_wchar,
    0x205f as ::core::ffi::c_int as pg_wchar,
    0x3000 as ::core::ffi::c_int as pg_wchar,
    0x3000 as ::core::ffi::c_int as pg_wchar,
];

static mut commonly_mapped_to_nothing_ranges: [pg_wchar; 16] = [
    0xad as ::core::ffi::c_int as pg_wchar,
    0xad as ::core::ffi::c_int as pg_wchar,
    0x34f as ::core::ffi::c_int as pg_wchar,
    0x34f as ::core::ffi::c_int as pg_wchar,
    0x1806 as ::core::ffi::c_int as pg_wchar,
    0x1806 as ::core::ffi::c_int as pg_wchar,
    0x180b as ::core::ffi::c_int as pg_wchar,
    0x180d as ::core::ffi::c_int as pg_wchar,
    0x200b as ::core::ffi::c_int as pg_wchar,
    0x200d as ::core::ffi::c_int as pg_wchar,
    0x2060 as ::core::ffi::c_int as pg_wchar,
    0x2060 as ::core::ffi::c_int as pg_wchar,
    0xfe00 as ::core::ffi::c_int as pg_wchar,
    0xfe0f as ::core::ffi::c_int as pg_wchar,
    0xfeff as ::core::ffi::c_int as pg_wchar,
    0xfeff as ::core::ffi::c_int as pg_wchar,
];

static mut prohibited_output_ranges: [pg_wchar; 72] = [
    0 as ::core::ffi::c_int as pg_wchar,
    0x1f as ::core::ffi::c_int as pg_wchar,
    0x7f as ::core::ffi::c_int as pg_wchar,
    0xa0 as ::core::ffi::c_int as pg_wchar,
    0x340 as ::core::ffi::c_int as pg_wchar,
    0x341 as ::core::ffi::c_int as pg_wchar,
    0x6dd as ::core::ffi::c_int as pg_wchar,
    0x6dd as ::core::ffi::c_int as pg_wchar,
    0x70f as ::core::ffi::c_int as pg_wchar,
    0x70f as ::core::ffi::c_int as pg_wchar,
    0x1680 as ::core::ffi::c_int as pg_wchar,
    0x1680 as ::core::ffi::c_int as pg_wchar,
    0x180e as ::core::ffi::c_int as pg_wchar,
    0x180e as ::core::ffi::c_int as pg_wchar,
    0x2000 as ::core::ffi::c_int as pg_wchar,
    0x200f as ::core::ffi::c_int as pg_wchar,
    0x2028 as ::core::ffi::c_int as pg_wchar,
    0x202f as ::core::ffi::c_int as pg_wchar,
    0x205f as ::core::ffi::c_int as pg_wchar,
    0x2063 as ::core::ffi::c_int as pg_wchar,
    0x206a as ::core::ffi::c_int as pg_wchar,
    0x206f as ::core::ffi::c_int as pg_wchar,
    0x2ff0 as ::core::ffi::c_int as pg_wchar,
    0x2ffb as ::core::ffi::c_int as pg_wchar,
    0x3000 as ::core::ffi::c_int as pg_wchar,
    0x3000 as ::core::ffi::c_int as pg_wchar,
    0xd800 as ::core::ffi::c_int as pg_wchar,
    0xf8ff as ::core::ffi::c_int as pg_wchar,
    0xfdd0 as ::core::ffi::c_int as pg_wchar,
    0xfdef as ::core::ffi::c_int as pg_wchar,
    0xfeff as ::core::ffi::c_int as pg_wchar,
    0xfeff as ::core::ffi::c_int as pg_wchar,
    0xfff9 as ::core::ffi::c_int as pg_wchar,
    0xffff as ::core::ffi::c_int as pg_wchar,
    0x1d173 as ::core::ffi::c_int as pg_wchar,
    0x1d17a as ::core::ffi::c_int as pg_wchar,
    0x1fffe as ::core::ffi::c_int as pg_wchar,
    0x1ffff as ::core::ffi::c_int as pg_wchar,
    0x2fffe as ::core::ffi::c_int as pg_wchar,
    0x2ffff as ::core::ffi::c_int as pg_wchar,
    0x3fffe as ::core::ffi::c_int as pg_wchar,
    0x3ffff as ::core::ffi::c_int as pg_wchar,
    0x4fffe as ::core::ffi::c_int as pg_wchar,
    0x4ffff as ::core::ffi::c_int as pg_wchar,
    0x5fffe as ::core::ffi::c_int as pg_wchar,
    0x5ffff as ::core::ffi::c_int as pg_wchar,
    0x6fffe as ::core::ffi::c_int as pg_wchar,
    0x6ffff as ::core::ffi::c_int as pg_wchar,
    0x7fffe as ::core::ffi::c_int as pg_wchar,
    0x7ffff as ::core::ffi::c_int as pg_wchar,
    0x8fffe as ::core::ffi::c_int as pg_wchar,
    0x8ffff as ::core::ffi::c_int as pg_wchar,
    0x9fffe as ::core::ffi::c_int as pg_wchar,
    0x9ffff as ::core::ffi::c_int as pg_wchar,
    0xafffe as ::core::ffi::c_int as pg_wchar,
    0xaffff as ::core::ffi::c_int as pg_wchar,
    0xbfffe as ::core::ffi::c_int as pg_wchar,
    0xbffff as ::core::ffi::c_int as pg_wchar,
    0xcfffe as ::core::ffi::c_int as pg_wchar,
    0xcffff as ::core::ffi::c_int as pg_wchar,
    0xdfffe as ::core::ffi::c_int as pg_wchar,
    0xdffff as ::core::ffi::c_int as pg_wchar,
    0xe0001 as ::core::ffi::c_int as pg_wchar,
    0xe0001 as ::core::ffi::c_int as pg_wchar,
    0xe0020 as ::core::ffi::c_int as pg_wchar,
    0xe007f as ::core::ffi::c_int as pg_wchar,
    0xefffe as ::core::ffi::c_int as pg_wchar,
    0xeffff as ::core::ffi::c_int as pg_wchar,
    0xf0000 as ::core::ffi::c_int as pg_wchar,
    0xfffff as ::core::ffi::c_int as pg_wchar,
    0x100000 as ::core::ffi::c_int as pg_wchar,
    0x10ffff as ::core::ffi::c_int as pg_wchar,
];

static mut unassigned_codepoint_ranges: [pg_wchar; 792] = [
    0x221 as ::core::ffi::c_int as pg_wchar,
    0x221 as ::core::ffi::c_int as pg_wchar,
    0x234 as ::core::ffi::c_int as pg_wchar,
    0x24f as ::core::ffi::c_int as pg_wchar,
    0x2ae as ::core::ffi::c_int as pg_wchar,
    0x2af as ::core::ffi::c_int as pg_wchar,
    0x2ef as ::core::ffi::c_int as pg_wchar,
    0x2ff as ::core::ffi::c_int as pg_wchar,
    0x350 as ::core::ffi::c_int as pg_wchar,
    0x35f as ::core::ffi::c_int as pg_wchar,
    0x370 as ::core::ffi::c_int as pg_wchar,
    0x373 as ::core::ffi::c_int as pg_wchar,
    0x376 as ::core::ffi::c_int as pg_wchar,
    0x379 as ::core::ffi::c_int as pg_wchar,
    0x37b as ::core::ffi::c_int as pg_wchar,
    0x37d as ::core::ffi::c_int as pg_wchar,
    0x37f as ::core::ffi::c_int as pg_wchar,
    0x383 as ::core::ffi::c_int as pg_wchar,
    0x38b as ::core::ffi::c_int as pg_wchar,
    0x38b as ::core::ffi::c_int as pg_wchar,
    0x38d as ::core::ffi::c_int as pg_wchar,
    0x38d as ::core::ffi::c_int as pg_wchar,
    0x3a2 as ::core::ffi::c_int as pg_wchar,
    0x3a2 as ::core::ffi::c_int as pg_wchar,
    0x3cf as ::core::ffi::c_int as pg_wchar,
    0x3cf as ::core::ffi::c_int as pg_wchar,
    0x3f7 as ::core::ffi::c_int as pg_wchar,
    0x3ff as ::core::ffi::c_int as pg_wchar,
    0x487 as ::core::ffi::c_int as pg_wchar,
    0x487 as ::core::ffi::c_int as pg_wchar,
    0x4cf as ::core::ffi::c_int as pg_wchar,
    0x4cf as ::core::ffi::c_int as pg_wchar,
    0x4f6 as ::core::ffi::c_int as pg_wchar,
    0x4f7 as ::core::ffi::c_int as pg_wchar,
    0x4fa as ::core::ffi::c_int as pg_wchar,
    0x4ff as ::core::ffi::c_int as pg_wchar,
    0x510 as ::core::ffi::c_int as pg_wchar,
    0x530 as ::core::ffi::c_int as pg_wchar,
    0x557 as ::core::ffi::c_int as pg_wchar,
    0x558 as ::core::ffi::c_int as pg_wchar,
    0x560 as ::core::ffi::c_int as pg_wchar,
    0x560 as ::core::ffi::c_int as pg_wchar,
    0x588 as ::core::ffi::c_int as pg_wchar,
    0x588 as ::core::ffi::c_int as pg_wchar,
    0x58b as ::core::ffi::c_int as pg_wchar,
    0x590 as ::core::ffi::c_int as pg_wchar,
    0x5a2 as ::core::ffi::c_int as pg_wchar,
    0x5a2 as ::core::ffi::c_int as pg_wchar,
    0x5ba as ::core::ffi::c_int as pg_wchar,
    0x5ba as ::core::ffi::c_int as pg_wchar,
    0x5c5 as ::core::ffi::c_int as pg_wchar,
    0x5cf as ::core::ffi::c_int as pg_wchar,
    0x5eb as ::core::ffi::c_int as pg_wchar,
    0x5ef as ::core::ffi::c_int as pg_wchar,
    0x5f5 as ::core::ffi::c_int as pg_wchar,
    0x60b as ::core::ffi::c_int as pg_wchar,
    0x60d as ::core::ffi::c_int as pg_wchar,
    0x61a as ::core::ffi::c_int as pg_wchar,
    0x61c as ::core::ffi::c_int as pg_wchar,
    0x61e as ::core::ffi::c_int as pg_wchar,
    0x620 as ::core::ffi::c_int as pg_wchar,
    0x620 as ::core::ffi::c_int as pg_wchar,
    0x63b as ::core::ffi::c_int as pg_wchar,
    0x63f as ::core::ffi::c_int as pg_wchar,
    0x656 as ::core::ffi::c_int as pg_wchar,
    0x65f as ::core::ffi::c_int as pg_wchar,
    0x6ee as ::core::ffi::c_int as pg_wchar,
    0x6ef as ::core::ffi::c_int as pg_wchar,
    0x6ff as ::core::ffi::c_int as pg_wchar,
    0x6ff as ::core::ffi::c_int as pg_wchar,
    0x70e as ::core::ffi::c_int as pg_wchar,
    0x70e as ::core::ffi::c_int as pg_wchar,
    0x72d as ::core::ffi::c_int as pg_wchar,
    0x72f as ::core::ffi::c_int as pg_wchar,
    0x74b as ::core::ffi::c_int as pg_wchar,
    0x77f as ::core::ffi::c_int as pg_wchar,
    0x7b2 as ::core::ffi::c_int as pg_wchar,
    0x900 as ::core::ffi::c_int as pg_wchar,
    0x904 as ::core::ffi::c_int as pg_wchar,
    0x904 as ::core::ffi::c_int as pg_wchar,
    0x93a as ::core::ffi::c_int as pg_wchar,
    0x93b as ::core::ffi::c_int as pg_wchar,
    0x94e as ::core::ffi::c_int as pg_wchar,
    0x94f as ::core::ffi::c_int as pg_wchar,
    0x955 as ::core::ffi::c_int as pg_wchar,
    0x957 as ::core::ffi::c_int as pg_wchar,
    0x971 as ::core::ffi::c_int as pg_wchar,
    0x980 as ::core::ffi::c_int as pg_wchar,
    0x984 as ::core::ffi::c_int as pg_wchar,
    0x984 as ::core::ffi::c_int as pg_wchar,
    0x98d as ::core::ffi::c_int as pg_wchar,
    0x98e as ::core::ffi::c_int as pg_wchar,
    0x991 as ::core::ffi::c_int as pg_wchar,
    0x992 as ::core::ffi::c_int as pg_wchar,
    0x9a9 as ::core::ffi::c_int as pg_wchar,
    0x9a9 as ::core::ffi::c_int as pg_wchar,
    0x9b1 as ::core::ffi::c_int as pg_wchar,
    0x9b1 as ::core::ffi::c_int as pg_wchar,
    0x9b3 as ::core::ffi::c_int as pg_wchar,
    0x9b5 as ::core::ffi::c_int as pg_wchar,
    0x9ba as ::core::ffi::c_int as pg_wchar,
    0x9bb as ::core::ffi::c_int as pg_wchar,
    0x9bd as ::core::ffi::c_int as pg_wchar,
    0x9bd as ::core::ffi::c_int as pg_wchar,
    0x9c5 as ::core::ffi::c_int as pg_wchar,
    0x9c6 as ::core::ffi::c_int as pg_wchar,
    0x9c9 as ::core::ffi::c_int as pg_wchar,
    0x9ca as ::core::ffi::c_int as pg_wchar,
    0x9ce as ::core::ffi::c_int as pg_wchar,
    0x9d6 as ::core::ffi::c_int as pg_wchar,
    0x9d8 as ::core::ffi::c_int as pg_wchar,
    0x9db as ::core::ffi::c_int as pg_wchar,
    0x9de as ::core::ffi::c_int as pg_wchar,
    0x9de as ::core::ffi::c_int as pg_wchar,
    0x9e4 as ::core::ffi::c_int as pg_wchar,
    0x9e5 as ::core::ffi::c_int as pg_wchar,
    0x9fb as ::core::ffi::c_int as pg_wchar,
    0xa01 as ::core::ffi::c_int as pg_wchar,
    0xa03 as ::core::ffi::c_int as pg_wchar,
    0xa04 as ::core::ffi::c_int as pg_wchar,
    0xa0b as ::core::ffi::c_int as pg_wchar,
    0xa0e as ::core::ffi::c_int as pg_wchar,
    0xa11 as ::core::ffi::c_int as pg_wchar,
    0xa12 as ::core::ffi::c_int as pg_wchar,
    0xa29 as ::core::ffi::c_int as pg_wchar,
    0xa29 as ::core::ffi::c_int as pg_wchar,
    0xa31 as ::core::ffi::c_int as pg_wchar,
    0xa31 as ::core::ffi::c_int as pg_wchar,
    0xa34 as ::core::ffi::c_int as pg_wchar,
    0xa34 as ::core::ffi::c_int as pg_wchar,
    0xa37 as ::core::ffi::c_int as pg_wchar,
    0xa37 as ::core::ffi::c_int as pg_wchar,
    0xa3a as ::core::ffi::c_int as pg_wchar,
    0xa3b as ::core::ffi::c_int as pg_wchar,
    0xa3d as ::core::ffi::c_int as pg_wchar,
    0xa3d as ::core::ffi::c_int as pg_wchar,
    0xa43 as ::core::ffi::c_int as pg_wchar,
    0xa46 as ::core::ffi::c_int as pg_wchar,
    0xa49 as ::core::ffi::c_int as pg_wchar,
    0xa4a as ::core::ffi::c_int as pg_wchar,
    0xa4e as ::core::ffi::c_int as pg_wchar,
    0xa58 as ::core::ffi::c_int as pg_wchar,
    0xa5d as ::core::ffi::c_int as pg_wchar,
    0xa5d as ::core::ffi::c_int as pg_wchar,
    0xa5f as ::core::ffi::c_int as pg_wchar,
    0xa65 as ::core::ffi::c_int as pg_wchar,
    0xa75 as ::core::ffi::c_int as pg_wchar,
    0xa80 as ::core::ffi::c_int as pg_wchar,
    0xa84 as ::core::ffi::c_int as pg_wchar,
    0xa84 as ::core::ffi::c_int as pg_wchar,
    0xa8c as ::core::ffi::c_int as pg_wchar,
    0xa8c as ::core::ffi::c_int as pg_wchar,
    0xa8e as ::core::ffi::c_int as pg_wchar,
    0xa8e as ::core::ffi::c_int as pg_wchar,
    0xa92 as ::core::ffi::c_int as pg_wchar,
    0xa92 as ::core::ffi::c_int as pg_wchar,
    0xaa9 as ::core::ffi::c_int as pg_wchar,
    0xaa9 as ::core::ffi::c_int as pg_wchar,
    0xab1 as ::core::ffi::c_int as pg_wchar,
    0xab1 as ::core::ffi::c_int as pg_wchar,
    0xab4 as ::core::ffi::c_int as pg_wchar,
    0xab4 as ::core::ffi::c_int as pg_wchar,
    0xaba as ::core::ffi::c_int as pg_wchar,
    0xabb as ::core::ffi::c_int as pg_wchar,
    0xac6 as ::core::ffi::c_int as pg_wchar,
    0xac6 as ::core::ffi::c_int as pg_wchar,
    0xaca as ::core::ffi::c_int as pg_wchar,
    0xaca as ::core::ffi::c_int as pg_wchar,
    0xace as ::core::ffi::c_int as pg_wchar,
    0xacf as ::core::ffi::c_int as pg_wchar,
    0xad1 as ::core::ffi::c_int as pg_wchar,
    0xadf as ::core::ffi::c_int as pg_wchar,
    0xae1 as ::core::ffi::c_int as pg_wchar,
    0xae5 as ::core::ffi::c_int as pg_wchar,
    0xaf0 as ::core::ffi::c_int as pg_wchar,
    0xb00 as ::core::ffi::c_int as pg_wchar,
    0xb04 as ::core::ffi::c_int as pg_wchar,
    0xb04 as ::core::ffi::c_int as pg_wchar,
    0xb0d as ::core::ffi::c_int as pg_wchar,
    0xb0e as ::core::ffi::c_int as pg_wchar,
    0xb11 as ::core::ffi::c_int as pg_wchar,
    0xb12 as ::core::ffi::c_int as pg_wchar,
    0xb29 as ::core::ffi::c_int as pg_wchar,
    0xb29 as ::core::ffi::c_int as pg_wchar,
    0xb31 as ::core::ffi::c_int as pg_wchar,
    0xb31 as ::core::ffi::c_int as pg_wchar,
    0xb34 as ::core::ffi::c_int as pg_wchar,
    0xb35 as ::core::ffi::c_int as pg_wchar,
    0xb3a as ::core::ffi::c_int as pg_wchar,
    0xb3b as ::core::ffi::c_int as pg_wchar,
    0xb44 as ::core::ffi::c_int as pg_wchar,
    0xb46 as ::core::ffi::c_int as pg_wchar,
    0xb49 as ::core::ffi::c_int as pg_wchar,
    0xb4a as ::core::ffi::c_int as pg_wchar,
    0xb4e as ::core::ffi::c_int as pg_wchar,
    0xb55 as ::core::ffi::c_int as pg_wchar,
    0xb58 as ::core::ffi::c_int as pg_wchar,
    0xb5b as ::core::ffi::c_int as pg_wchar,
    0xb5e as ::core::ffi::c_int as pg_wchar,
    0xb5e as ::core::ffi::c_int as pg_wchar,
    0xb62 as ::core::ffi::c_int as pg_wchar,
    0xb65 as ::core::ffi::c_int as pg_wchar,
    0xb71 as ::core::ffi::c_int as pg_wchar,
    0xb81 as ::core::ffi::c_int as pg_wchar,
    0xb84 as ::core::ffi::c_int as pg_wchar,
    0xb84 as ::core::ffi::c_int as pg_wchar,
    0xb8b as ::core::ffi::c_int as pg_wchar,
    0xb8d as ::core::ffi::c_int as pg_wchar,
    0xb91 as ::core::ffi::c_int as pg_wchar,
    0xb91 as ::core::ffi::c_int as pg_wchar,
    0xb96 as ::core::ffi::c_int as pg_wchar,
    0xb98 as ::core::ffi::c_int as pg_wchar,
    0xb9b as ::core::ffi::c_int as pg_wchar,
    0xb9b as ::core::ffi::c_int as pg_wchar,
    0xb9d as ::core::ffi::c_int as pg_wchar,
    0xb9d as ::core::ffi::c_int as pg_wchar,
    0xba0 as ::core::ffi::c_int as pg_wchar,
    0xba2 as ::core::ffi::c_int as pg_wchar,
    0xba5 as ::core::ffi::c_int as pg_wchar,
    0xba7 as ::core::ffi::c_int as pg_wchar,
    0xbab as ::core::ffi::c_int as pg_wchar,
    0xbad as ::core::ffi::c_int as pg_wchar,
    0xbb6 as ::core::ffi::c_int as pg_wchar,
    0xbb6 as ::core::ffi::c_int as pg_wchar,
    0xbba as ::core::ffi::c_int as pg_wchar,
    0xbbd as ::core::ffi::c_int as pg_wchar,
    0xbc3 as ::core::ffi::c_int as pg_wchar,
    0xbc5 as ::core::ffi::c_int as pg_wchar,
    0xbc9 as ::core::ffi::c_int as pg_wchar,
    0xbc9 as ::core::ffi::c_int as pg_wchar,
    0xbce as ::core::ffi::c_int as pg_wchar,
    0xbd6 as ::core::ffi::c_int as pg_wchar,
    0xbd8 as ::core::ffi::c_int as pg_wchar,
    0xbe6 as ::core::ffi::c_int as pg_wchar,
    0xbf3 as ::core::ffi::c_int as pg_wchar,
    0xc00 as ::core::ffi::c_int as pg_wchar,
    0xc04 as ::core::ffi::c_int as pg_wchar,
    0xc04 as ::core::ffi::c_int as pg_wchar,
    0xc0d as ::core::ffi::c_int as pg_wchar,
    0xc0d as ::core::ffi::c_int as pg_wchar,
    0xc11 as ::core::ffi::c_int as pg_wchar,
    0xc11 as ::core::ffi::c_int as pg_wchar,
    0xc29 as ::core::ffi::c_int as pg_wchar,
    0xc29 as ::core::ffi::c_int as pg_wchar,
    0xc34 as ::core::ffi::c_int as pg_wchar,
    0xc34 as ::core::ffi::c_int as pg_wchar,
    0xc3a as ::core::ffi::c_int as pg_wchar,
    0xc3d as ::core::ffi::c_int as pg_wchar,
    0xc45 as ::core::ffi::c_int as pg_wchar,
    0xc45 as ::core::ffi::c_int as pg_wchar,
    0xc49 as ::core::ffi::c_int as pg_wchar,
    0xc49 as ::core::ffi::c_int as pg_wchar,
    0xc4e as ::core::ffi::c_int as pg_wchar,
    0xc54 as ::core::ffi::c_int as pg_wchar,
    0xc57 as ::core::ffi::c_int as pg_wchar,
    0xc5f as ::core::ffi::c_int as pg_wchar,
    0xc62 as ::core::ffi::c_int as pg_wchar,
    0xc65 as ::core::ffi::c_int as pg_wchar,
    0xc70 as ::core::ffi::c_int as pg_wchar,
    0xc81 as ::core::ffi::c_int as pg_wchar,
    0xc84 as ::core::ffi::c_int as pg_wchar,
    0xc84 as ::core::ffi::c_int as pg_wchar,
    0xc8d as ::core::ffi::c_int as pg_wchar,
    0xc8d as ::core::ffi::c_int as pg_wchar,
    0xc91 as ::core::ffi::c_int as pg_wchar,
    0xc91 as ::core::ffi::c_int as pg_wchar,
    0xca9 as ::core::ffi::c_int as pg_wchar,
    0xca9 as ::core::ffi::c_int as pg_wchar,
    0xcb4 as ::core::ffi::c_int as pg_wchar,
    0xcb4 as ::core::ffi::c_int as pg_wchar,
    0xcba as ::core::ffi::c_int as pg_wchar,
    0xcbd as ::core::ffi::c_int as pg_wchar,
    0xcc5 as ::core::ffi::c_int as pg_wchar,
    0xcc5 as ::core::ffi::c_int as pg_wchar,
    0xcc9 as ::core::ffi::c_int as pg_wchar,
    0xcc9 as ::core::ffi::c_int as pg_wchar,
    0xcce as ::core::ffi::c_int as pg_wchar,
    0xcd4 as ::core::ffi::c_int as pg_wchar,
    0xcd7 as ::core::ffi::c_int as pg_wchar,
    0xcdd as ::core::ffi::c_int as pg_wchar,
    0xcdf as ::core::ffi::c_int as pg_wchar,
    0xcdf as ::core::ffi::c_int as pg_wchar,
    0xce2 as ::core::ffi::c_int as pg_wchar,
    0xce5 as ::core::ffi::c_int as pg_wchar,
    0xcf0 as ::core::ffi::c_int as pg_wchar,
    0xd01 as ::core::ffi::c_int as pg_wchar,
    0xd04 as ::core::ffi::c_int as pg_wchar,
    0xd04 as ::core::ffi::c_int as pg_wchar,
    0xd0d as ::core::ffi::c_int as pg_wchar,
    0xd0d as ::core::ffi::c_int as pg_wchar,
    0xd11 as ::core::ffi::c_int as pg_wchar,
    0xd11 as ::core::ffi::c_int as pg_wchar,
    0xd29 as ::core::ffi::c_int as pg_wchar,
    0xd29 as ::core::ffi::c_int as pg_wchar,
    0xd3a as ::core::ffi::c_int as pg_wchar,
    0xd3d as ::core::ffi::c_int as pg_wchar,
    0xd44 as ::core::ffi::c_int as pg_wchar,
    0xd45 as ::core::ffi::c_int as pg_wchar,
    0xd49 as ::core::ffi::c_int as pg_wchar,
    0xd49 as ::core::ffi::c_int as pg_wchar,
    0xd4e as ::core::ffi::c_int as pg_wchar,
    0xd56 as ::core::ffi::c_int as pg_wchar,
    0xd58 as ::core::ffi::c_int as pg_wchar,
    0xd5f as ::core::ffi::c_int as pg_wchar,
    0xd62 as ::core::ffi::c_int as pg_wchar,
    0xd65 as ::core::ffi::c_int as pg_wchar,
    0xd70 as ::core::ffi::c_int as pg_wchar,
    0xd81 as ::core::ffi::c_int as pg_wchar,
    0xd84 as ::core::ffi::c_int as pg_wchar,
    0xd84 as ::core::ffi::c_int as pg_wchar,
    0xd97 as ::core::ffi::c_int as pg_wchar,
    0xd99 as ::core::ffi::c_int as pg_wchar,
    0xdb2 as ::core::ffi::c_int as pg_wchar,
    0xdb2 as ::core::ffi::c_int as pg_wchar,
    0xdbc as ::core::ffi::c_int as pg_wchar,
    0xdbc as ::core::ffi::c_int as pg_wchar,
    0xdbe as ::core::ffi::c_int as pg_wchar,
    0xdbf as ::core::ffi::c_int as pg_wchar,
    0xdc7 as ::core::ffi::c_int as pg_wchar,
    0xdc9 as ::core::ffi::c_int as pg_wchar,
    0xdcb as ::core::ffi::c_int as pg_wchar,
    0xdce as ::core::ffi::c_int as pg_wchar,
    0xdd5 as ::core::ffi::c_int as pg_wchar,
    0xdd5 as ::core::ffi::c_int as pg_wchar,
    0xdd7 as ::core::ffi::c_int as pg_wchar,
    0xdd7 as ::core::ffi::c_int as pg_wchar,
    0xde0 as ::core::ffi::c_int as pg_wchar,
    0xdf1 as ::core::ffi::c_int as pg_wchar,
    0xdf5 as ::core::ffi::c_int as pg_wchar,
    0xe00 as ::core::ffi::c_int as pg_wchar,
    0xe3b as ::core::ffi::c_int as pg_wchar,
    0xe3e as ::core::ffi::c_int as pg_wchar,
    0xe5c as ::core::ffi::c_int as pg_wchar,
    0xe80 as ::core::ffi::c_int as pg_wchar,
    0xe83 as ::core::ffi::c_int as pg_wchar,
    0xe83 as ::core::ffi::c_int as pg_wchar,
    0xe85 as ::core::ffi::c_int as pg_wchar,
    0xe86 as ::core::ffi::c_int as pg_wchar,
    0xe89 as ::core::ffi::c_int as pg_wchar,
    0xe89 as ::core::ffi::c_int as pg_wchar,
    0xe8b as ::core::ffi::c_int as pg_wchar,
    0xe8c as ::core::ffi::c_int as pg_wchar,
    0xe8e as ::core::ffi::c_int as pg_wchar,
    0xe93 as ::core::ffi::c_int as pg_wchar,
    0xe98 as ::core::ffi::c_int as pg_wchar,
    0xe98 as ::core::ffi::c_int as pg_wchar,
    0xea0 as ::core::ffi::c_int as pg_wchar,
    0xea0 as ::core::ffi::c_int as pg_wchar,
    0xea4 as ::core::ffi::c_int as pg_wchar,
    0xea4 as ::core::ffi::c_int as pg_wchar,
    0xea6 as ::core::ffi::c_int as pg_wchar,
    0xea6 as ::core::ffi::c_int as pg_wchar,
    0xea8 as ::core::ffi::c_int as pg_wchar,
    0xea9 as ::core::ffi::c_int as pg_wchar,
    0xeac as ::core::ffi::c_int as pg_wchar,
    0xeac as ::core::ffi::c_int as pg_wchar,
    0xeba as ::core::ffi::c_int as pg_wchar,
    0xeba as ::core::ffi::c_int as pg_wchar,
    0xebe as ::core::ffi::c_int as pg_wchar,
    0xebf as ::core::ffi::c_int as pg_wchar,
    0xec5 as ::core::ffi::c_int as pg_wchar,
    0xec5 as ::core::ffi::c_int as pg_wchar,
    0xec7 as ::core::ffi::c_int as pg_wchar,
    0xec7 as ::core::ffi::c_int as pg_wchar,
    0xece as ::core::ffi::c_int as pg_wchar,
    0xecf as ::core::ffi::c_int as pg_wchar,
    0xeda as ::core::ffi::c_int as pg_wchar,
    0xedb as ::core::ffi::c_int as pg_wchar,
    0xede as ::core::ffi::c_int as pg_wchar,
    0xeff as ::core::ffi::c_int as pg_wchar,
    0xf48 as ::core::ffi::c_int as pg_wchar,
    0xf48 as ::core::ffi::c_int as pg_wchar,
    0xf6b as ::core::ffi::c_int as pg_wchar,
    0xf70 as ::core::ffi::c_int as pg_wchar,
    0xf8c as ::core::ffi::c_int as pg_wchar,
    0xf8f as ::core::ffi::c_int as pg_wchar,
    0xf98 as ::core::ffi::c_int as pg_wchar,
    0xf98 as ::core::ffi::c_int as pg_wchar,
    0xfbd as ::core::ffi::c_int as pg_wchar,
    0xfbd as ::core::ffi::c_int as pg_wchar,
    0xfcd as ::core::ffi::c_int as pg_wchar,
    0xfce as ::core::ffi::c_int as pg_wchar,
    0xfd0 as ::core::ffi::c_int as pg_wchar,
    0xfff as ::core::ffi::c_int as pg_wchar,
    0x1022 as ::core::ffi::c_int as pg_wchar,
    0x1022 as ::core::ffi::c_int as pg_wchar,
    0x1028 as ::core::ffi::c_int as pg_wchar,
    0x1028 as ::core::ffi::c_int as pg_wchar,
    0x102b as ::core::ffi::c_int as pg_wchar,
    0x102b as ::core::ffi::c_int as pg_wchar,
    0x1033 as ::core::ffi::c_int as pg_wchar,
    0x1035 as ::core::ffi::c_int as pg_wchar,
    0x103a as ::core::ffi::c_int as pg_wchar,
    0x103f as ::core::ffi::c_int as pg_wchar,
    0x105a as ::core::ffi::c_int as pg_wchar,
    0x109f as ::core::ffi::c_int as pg_wchar,
    0x10c6 as ::core::ffi::c_int as pg_wchar,
    0x10cf as ::core::ffi::c_int as pg_wchar,
    0x10f9 as ::core::ffi::c_int as pg_wchar,
    0x10fa as ::core::ffi::c_int as pg_wchar,
    0x10fc as ::core::ffi::c_int as pg_wchar,
    0x10ff as ::core::ffi::c_int as pg_wchar,
    0x115a as ::core::ffi::c_int as pg_wchar,
    0x115e as ::core::ffi::c_int as pg_wchar,
    0x11a3 as ::core::ffi::c_int as pg_wchar,
    0x11a7 as ::core::ffi::c_int as pg_wchar,
    0x11fa as ::core::ffi::c_int as pg_wchar,
    0x11ff as ::core::ffi::c_int as pg_wchar,
    0x1207 as ::core::ffi::c_int as pg_wchar,
    0x1207 as ::core::ffi::c_int as pg_wchar,
    0x1247 as ::core::ffi::c_int as pg_wchar,
    0x1247 as ::core::ffi::c_int as pg_wchar,
    0x1249 as ::core::ffi::c_int as pg_wchar,
    0x1249 as ::core::ffi::c_int as pg_wchar,
    0x124e as ::core::ffi::c_int as pg_wchar,
    0x124f as ::core::ffi::c_int as pg_wchar,
    0x1257 as ::core::ffi::c_int as pg_wchar,
    0x1257 as ::core::ffi::c_int as pg_wchar,
    0x1259 as ::core::ffi::c_int as pg_wchar,
    0x1259 as ::core::ffi::c_int as pg_wchar,
    0x125e as ::core::ffi::c_int as pg_wchar,
    0x125f as ::core::ffi::c_int as pg_wchar,
    0x1287 as ::core::ffi::c_int as pg_wchar,
    0x1287 as ::core::ffi::c_int as pg_wchar,
    0x1289 as ::core::ffi::c_int as pg_wchar,
    0x1289 as ::core::ffi::c_int as pg_wchar,
    0x128e as ::core::ffi::c_int as pg_wchar,
    0x128f as ::core::ffi::c_int as pg_wchar,
    0x12af as ::core::ffi::c_int as pg_wchar,
    0x12af as ::core::ffi::c_int as pg_wchar,
    0x12b1 as ::core::ffi::c_int as pg_wchar,
    0x12b1 as ::core::ffi::c_int as pg_wchar,
    0x12b6 as ::core::ffi::c_int as pg_wchar,
    0x12b7 as ::core::ffi::c_int as pg_wchar,
    0x12bf as ::core::ffi::c_int as pg_wchar,
    0x12bf as ::core::ffi::c_int as pg_wchar,
    0x12c1 as ::core::ffi::c_int as pg_wchar,
    0x12c1 as ::core::ffi::c_int as pg_wchar,
    0x12c6 as ::core::ffi::c_int as pg_wchar,
    0x12c7 as ::core::ffi::c_int as pg_wchar,
    0x12cf as ::core::ffi::c_int as pg_wchar,
    0x12cf as ::core::ffi::c_int as pg_wchar,
    0x12d7 as ::core::ffi::c_int as pg_wchar,
    0x12d7 as ::core::ffi::c_int as pg_wchar,
    0x12ef as ::core::ffi::c_int as pg_wchar,
    0x12ef as ::core::ffi::c_int as pg_wchar,
    0x130f as ::core::ffi::c_int as pg_wchar,
    0x130f as ::core::ffi::c_int as pg_wchar,
    0x1311 as ::core::ffi::c_int as pg_wchar,
    0x1311 as ::core::ffi::c_int as pg_wchar,
    0x1316 as ::core::ffi::c_int as pg_wchar,
    0x1317 as ::core::ffi::c_int as pg_wchar,
    0x131f as ::core::ffi::c_int as pg_wchar,
    0x131f as ::core::ffi::c_int as pg_wchar,
    0x1347 as ::core::ffi::c_int as pg_wchar,
    0x1347 as ::core::ffi::c_int as pg_wchar,
    0x135b as ::core::ffi::c_int as pg_wchar,
    0x1360 as ::core::ffi::c_int as pg_wchar,
    0x137d as ::core::ffi::c_int as pg_wchar,
    0x139f as ::core::ffi::c_int as pg_wchar,
    0x13f5 as ::core::ffi::c_int as pg_wchar,
    0x1400 as ::core::ffi::c_int as pg_wchar,
    0x1677 as ::core::ffi::c_int as pg_wchar,
    0x167f as ::core::ffi::c_int as pg_wchar,
    0x169d as ::core::ffi::c_int as pg_wchar,
    0x169f as ::core::ffi::c_int as pg_wchar,
    0x16f1 as ::core::ffi::c_int as pg_wchar,
    0x16ff as ::core::ffi::c_int as pg_wchar,
    0x170d as ::core::ffi::c_int as pg_wchar,
    0x170d as ::core::ffi::c_int as pg_wchar,
    0x1715 as ::core::ffi::c_int as pg_wchar,
    0x171f as ::core::ffi::c_int as pg_wchar,
    0x1737 as ::core::ffi::c_int as pg_wchar,
    0x173f as ::core::ffi::c_int as pg_wchar,
    0x1754 as ::core::ffi::c_int as pg_wchar,
    0x175f as ::core::ffi::c_int as pg_wchar,
    0x176d as ::core::ffi::c_int as pg_wchar,
    0x176d as ::core::ffi::c_int as pg_wchar,
    0x1771 as ::core::ffi::c_int as pg_wchar,
    0x1771 as ::core::ffi::c_int as pg_wchar,
    0x1774 as ::core::ffi::c_int as pg_wchar,
    0x177f as ::core::ffi::c_int as pg_wchar,
    0x17dd as ::core::ffi::c_int as pg_wchar,
    0x17df as ::core::ffi::c_int as pg_wchar,
    0x17ea as ::core::ffi::c_int as pg_wchar,
    0x17ff as ::core::ffi::c_int as pg_wchar,
    0x180f as ::core::ffi::c_int as pg_wchar,
    0x180f as ::core::ffi::c_int as pg_wchar,
    0x181a as ::core::ffi::c_int as pg_wchar,
    0x181f as ::core::ffi::c_int as pg_wchar,
    0x1878 as ::core::ffi::c_int as pg_wchar,
    0x187f as ::core::ffi::c_int as pg_wchar,
    0x18aa as ::core::ffi::c_int as pg_wchar,
    0x1dff as ::core::ffi::c_int as pg_wchar,
    0x1e9c as ::core::ffi::c_int as pg_wchar,
    0x1e9f as ::core::ffi::c_int as pg_wchar,
    0x1efa as ::core::ffi::c_int as pg_wchar,
    0x1eff as ::core::ffi::c_int as pg_wchar,
    0x1f16 as ::core::ffi::c_int as pg_wchar,
    0x1f17 as ::core::ffi::c_int as pg_wchar,
    0x1f1e as ::core::ffi::c_int as pg_wchar,
    0x1f1f as ::core::ffi::c_int as pg_wchar,
    0x1f46 as ::core::ffi::c_int as pg_wchar,
    0x1f47 as ::core::ffi::c_int as pg_wchar,
    0x1f4e as ::core::ffi::c_int as pg_wchar,
    0x1f4f as ::core::ffi::c_int as pg_wchar,
    0x1f58 as ::core::ffi::c_int as pg_wchar,
    0x1f58 as ::core::ffi::c_int as pg_wchar,
    0x1f5a as ::core::ffi::c_int as pg_wchar,
    0x1f5a as ::core::ffi::c_int as pg_wchar,
    0x1f5c as ::core::ffi::c_int as pg_wchar,
    0x1f5c as ::core::ffi::c_int as pg_wchar,
    0x1f5e as ::core::ffi::c_int as pg_wchar,
    0x1f5e as ::core::ffi::c_int as pg_wchar,
    0x1f7e as ::core::ffi::c_int as pg_wchar,
    0x1f7f as ::core::ffi::c_int as pg_wchar,
    0x1fb5 as ::core::ffi::c_int as pg_wchar,
    0x1fb5 as ::core::ffi::c_int as pg_wchar,
    0x1fc5 as ::core::ffi::c_int as pg_wchar,
    0x1fc5 as ::core::ffi::c_int as pg_wchar,
    0x1fd4 as ::core::ffi::c_int as pg_wchar,
    0x1fd5 as ::core::ffi::c_int as pg_wchar,
    0x1fdc as ::core::ffi::c_int as pg_wchar,
    0x1fdc as ::core::ffi::c_int as pg_wchar,
    0x1ff0 as ::core::ffi::c_int as pg_wchar,
    0x1ff1 as ::core::ffi::c_int as pg_wchar,
    0x1ff5 as ::core::ffi::c_int as pg_wchar,
    0x1ff5 as ::core::ffi::c_int as pg_wchar,
    0x1fff as ::core::ffi::c_int as pg_wchar,
    0x1fff as ::core::ffi::c_int as pg_wchar,
    0x2053 as ::core::ffi::c_int as pg_wchar,
    0x2056 as ::core::ffi::c_int as pg_wchar,
    0x2058 as ::core::ffi::c_int as pg_wchar,
    0x205e as ::core::ffi::c_int as pg_wchar,
    0x2064 as ::core::ffi::c_int as pg_wchar,
    0x2069 as ::core::ffi::c_int as pg_wchar,
    0x2072 as ::core::ffi::c_int as pg_wchar,
    0x2073 as ::core::ffi::c_int as pg_wchar,
    0x208f as ::core::ffi::c_int as pg_wchar,
    0x209f as ::core::ffi::c_int as pg_wchar,
    0x20b2 as ::core::ffi::c_int as pg_wchar,
    0x20cf as ::core::ffi::c_int as pg_wchar,
    0x20eb as ::core::ffi::c_int as pg_wchar,
    0x20ff as ::core::ffi::c_int as pg_wchar,
    0x213b as ::core::ffi::c_int as pg_wchar,
    0x213c as ::core::ffi::c_int as pg_wchar,
    0x214c as ::core::ffi::c_int as pg_wchar,
    0x2152 as ::core::ffi::c_int as pg_wchar,
    0x2184 as ::core::ffi::c_int as pg_wchar,
    0x218f as ::core::ffi::c_int as pg_wchar,
    0x23cf as ::core::ffi::c_int as pg_wchar,
    0x23ff as ::core::ffi::c_int as pg_wchar,
    0x2427 as ::core::ffi::c_int as pg_wchar,
    0x243f as ::core::ffi::c_int as pg_wchar,
    0x244b as ::core::ffi::c_int as pg_wchar,
    0x245f as ::core::ffi::c_int as pg_wchar,
    0x24ff as ::core::ffi::c_int as pg_wchar,
    0x24ff as ::core::ffi::c_int as pg_wchar,
    0x2614 as ::core::ffi::c_int as pg_wchar,
    0x2615 as ::core::ffi::c_int as pg_wchar,
    0x2618 as ::core::ffi::c_int as pg_wchar,
    0x2618 as ::core::ffi::c_int as pg_wchar,
    0x267e as ::core::ffi::c_int as pg_wchar,
    0x267f as ::core::ffi::c_int as pg_wchar,
    0x268a as ::core::ffi::c_int as pg_wchar,
    0x2700 as ::core::ffi::c_int as pg_wchar,
    0x2705 as ::core::ffi::c_int as pg_wchar,
    0x2705 as ::core::ffi::c_int as pg_wchar,
    0x270a as ::core::ffi::c_int as pg_wchar,
    0x270b as ::core::ffi::c_int as pg_wchar,
    0x2728 as ::core::ffi::c_int as pg_wchar,
    0x2728 as ::core::ffi::c_int as pg_wchar,
    0x274c as ::core::ffi::c_int as pg_wchar,
    0x274c as ::core::ffi::c_int as pg_wchar,
    0x274e as ::core::ffi::c_int as pg_wchar,
    0x274e as ::core::ffi::c_int as pg_wchar,
    0x2753 as ::core::ffi::c_int as pg_wchar,
    0x2755 as ::core::ffi::c_int as pg_wchar,
    0x2757 as ::core::ffi::c_int as pg_wchar,
    0x2757 as ::core::ffi::c_int as pg_wchar,
    0x275f as ::core::ffi::c_int as pg_wchar,
    0x2760 as ::core::ffi::c_int as pg_wchar,
    0x2795 as ::core::ffi::c_int as pg_wchar,
    0x2797 as ::core::ffi::c_int as pg_wchar,
    0x27b0 as ::core::ffi::c_int as pg_wchar,
    0x27b0 as ::core::ffi::c_int as pg_wchar,
    0x27bf as ::core::ffi::c_int as pg_wchar,
    0x27cf as ::core::ffi::c_int as pg_wchar,
    0x27ec as ::core::ffi::c_int as pg_wchar,
    0x27ef as ::core::ffi::c_int as pg_wchar,
    0x2b00 as ::core::ffi::c_int as pg_wchar,
    0x2e7f as ::core::ffi::c_int as pg_wchar,
    0x2e9a as ::core::ffi::c_int as pg_wchar,
    0x2e9a as ::core::ffi::c_int as pg_wchar,
    0x2ef4 as ::core::ffi::c_int as pg_wchar,
    0x2eff as ::core::ffi::c_int as pg_wchar,
    0x2fd6 as ::core::ffi::c_int as pg_wchar,
    0x2fef as ::core::ffi::c_int as pg_wchar,
    0x2ffc as ::core::ffi::c_int as pg_wchar,
    0x2fff as ::core::ffi::c_int as pg_wchar,
    0x3040 as ::core::ffi::c_int as pg_wchar,
    0x3040 as ::core::ffi::c_int as pg_wchar,
    0x3097 as ::core::ffi::c_int as pg_wchar,
    0x3098 as ::core::ffi::c_int as pg_wchar,
    0x3100 as ::core::ffi::c_int as pg_wchar,
    0x3104 as ::core::ffi::c_int as pg_wchar,
    0x312d as ::core::ffi::c_int as pg_wchar,
    0x3130 as ::core::ffi::c_int as pg_wchar,
    0x318f as ::core::ffi::c_int as pg_wchar,
    0x318f as ::core::ffi::c_int as pg_wchar,
    0x31b8 as ::core::ffi::c_int as pg_wchar,
    0x31ef as ::core::ffi::c_int as pg_wchar,
    0x321d as ::core::ffi::c_int as pg_wchar,
    0x321f as ::core::ffi::c_int as pg_wchar,
    0x3244 as ::core::ffi::c_int as pg_wchar,
    0x3250 as ::core::ffi::c_int as pg_wchar,
    0x327c as ::core::ffi::c_int as pg_wchar,
    0x327e as ::core::ffi::c_int as pg_wchar,
    0x32cc as ::core::ffi::c_int as pg_wchar,
    0x32cf as ::core::ffi::c_int as pg_wchar,
    0x32ff as ::core::ffi::c_int as pg_wchar,
    0x32ff as ::core::ffi::c_int as pg_wchar,
    0x3377 as ::core::ffi::c_int as pg_wchar,
    0x337a as ::core::ffi::c_int as pg_wchar,
    0x33de as ::core::ffi::c_int as pg_wchar,
    0x33df as ::core::ffi::c_int as pg_wchar,
    0x33ff as ::core::ffi::c_int as pg_wchar,
    0x33ff as ::core::ffi::c_int as pg_wchar,
    0x4db6 as ::core::ffi::c_int as pg_wchar,
    0x4dff as ::core::ffi::c_int as pg_wchar,
    0x9fa6 as ::core::ffi::c_int as pg_wchar,
    0x9fff as ::core::ffi::c_int as pg_wchar,
    0xa48d as ::core::ffi::c_int as pg_wchar,
    0xa48f as ::core::ffi::c_int as pg_wchar,
    0xa4c7 as ::core::ffi::c_int as pg_wchar,
    0xabff as ::core::ffi::c_int as pg_wchar,
    0xd7a4 as ::core::ffi::c_int as pg_wchar,
    0xd7ff as ::core::ffi::c_int as pg_wchar,
    0xfa2e as ::core::ffi::c_int as pg_wchar,
    0xfa2f as ::core::ffi::c_int as pg_wchar,
    0xfa6b as ::core::ffi::c_int as pg_wchar,
    0xfaff as ::core::ffi::c_int as pg_wchar,
    0xfb07 as ::core::ffi::c_int as pg_wchar,
    0xfb12 as ::core::ffi::c_int as pg_wchar,
    0xfb18 as ::core::ffi::c_int as pg_wchar,
    0xfb1c as ::core::ffi::c_int as pg_wchar,
    0xfb37 as ::core::ffi::c_int as pg_wchar,
    0xfb37 as ::core::ffi::c_int as pg_wchar,
    0xfb3d as ::core::ffi::c_int as pg_wchar,
    0xfb3d as ::core::ffi::c_int as pg_wchar,
    0xfb3f as ::core::ffi::c_int as pg_wchar,
    0xfb3f as ::core::ffi::c_int as pg_wchar,
    0xfb42 as ::core::ffi::c_int as pg_wchar,
    0xfb42 as ::core::ffi::c_int as pg_wchar,
    0xfb45 as ::core::ffi::c_int as pg_wchar,
    0xfb45 as ::core::ffi::c_int as pg_wchar,
    0xfbb2 as ::core::ffi::c_int as pg_wchar,
    0xfbd2 as ::core::ffi::c_int as pg_wchar,
    0xfd40 as ::core::ffi::c_int as pg_wchar,
    0xfd4f as ::core::ffi::c_int as pg_wchar,
    0xfd90 as ::core::ffi::c_int as pg_wchar,
    0xfd91 as ::core::ffi::c_int as pg_wchar,
    0xfdc8 as ::core::ffi::c_int as pg_wchar,
    0xfdcf as ::core::ffi::c_int as pg_wchar,
    0xfdfd as ::core::ffi::c_int as pg_wchar,
    0xfdff as ::core::ffi::c_int as pg_wchar,
    0xfe10 as ::core::ffi::c_int as pg_wchar,
    0xfe1f as ::core::ffi::c_int as pg_wchar,
    0xfe24 as ::core::ffi::c_int as pg_wchar,
    0xfe2f as ::core::ffi::c_int as pg_wchar,
    0xfe47 as ::core::ffi::c_int as pg_wchar,
    0xfe48 as ::core::ffi::c_int as pg_wchar,
    0xfe53 as ::core::ffi::c_int as pg_wchar,
    0xfe53 as ::core::ffi::c_int as pg_wchar,
    0xfe67 as ::core::ffi::c_int as pg_wchar,
    0xfe67 as ::core::ffi::c_int as pg_wchar,
    0xfe6c as ::core::ffi::c_int as pg_wchar,
    0xfe6f as ::core::ffi::c_int as pg_wchar,
    0xfe75 as ::core::ffi::c_int as pg_wchar,
    0xfe75 as ::core::ffi::c_int as pg_wchar,
    0xfefd as ::core::ffi::c_int as pg_wchar,
    0xfefe as ::core::ffi::c_int as pg_wchar,
    0xff00 as ::core::ffi::c_int as pg_wchar,
    0xff00 as ::core::ffi::c_int as pg_wchar,
    0xffbf as ::core::ffi::c_int as pg_wchar,
    0xffc1 as ::core::ffi::c_int as pg_wchar,
    0xffc8 as ::core::ffi::c_int as pg_wchar,
    0xffc9 as ::core::ffi::c_int as pg_wchar,
    0xffd0 as ::core::ffi::c_int as pg_wchar,
    0xffd1 as ::core::ffi::c_int as pg_wchar,
    0xffd8 as ::core::ffi::c_int as pg_wchar,
    0xffd9 as ::core::ffi::c_int as pg_wchar,
    0xffdd as ::core::ffi::c_int as pg_wchar,
    0xffdf as ::core::ffi::c_int as pg_wchar,
    0xffe7 as ::core::ffi::c_int as pg_wchar,
    0xffe7 as ::core::ffi::c_int as pg_wchar,
    0xffef as ::core::ffi::c_int as pg_wchar,
    0xfff8 as ::core::ffi::c_int as pg_wchar,
    0x10000 as ::core::ffi::c_int as pg_wchar,
    0x102ff as ::core::ffi::c_int as pg_wchar,
    0x1031f as ::core::ffi::c_int as pg_wchar,
    0x1031f as ::core::ffi::c_int as pg_wchar,
    0x10324 as ::core::ffi::c_int as pg_wchar,
    0x1032f as ::core::ffi::c_int as pg_wchar,
    0x1034b as ::core::ffi::c_int as pg_wchar,
    0x103ff as ::core::ffi::c_int as pg_wchar,
    0x10426 as ::core::ffi::c_int as pg_wchar,
    0x10427 as ::core::ffi::c_int as pg_wchar,
    0x1044e as ::core::ffi::c_int as pg_wchar,
    0x1cfff as ::core::ffi::c_int as pg_wchar,
    0x1d0f6 as ::core::ffi::c_int as pg_wchar,
    0x1d0ff as ::core::ffi::c_int as pg_wchar,
    0x1d127 as ::core::ffi::c_int as pg_wchar,
    0x1d129 as ::core::ffi::c_int as pg_wchar,
    0x1d1de as ::core::ffi::c_int as pg_wchar,
    0x1d3ff as ::core::ffi::c_int as pg_wchar,
    0x1d455 as ::core::ffi::c_int as pg_wchar,
    0x1d455 as ::core::ffi::c_int as pg_wchar,
    0x1d49d as ::core::ffi::c_int as pg_wchar,
    0x1d49d as ::core::ffi::c_int as pg_wchar,
    0x1d4a0 as ::core::ffi::c_int as pg_wchar,
    0x1d4a1 as ::core::ffi::c_int as pg_wchar,
    0x1d4a3 as ::core::ffi::c_int as pg_wchar,
    0x1d4a4 as ::core::ffi::c_int as pg_wchar,
    0x1d4a7 as ::core::ffi::c_int as pg_wchar,
    0x1d4a8 as ::core::ffi::c_int as pg_wchar,
    0x1d4ad as ::core::ffi::c_int as pg_wchar,
    0x1d4ad as ::core::ffi::c_int as pg_wchar,
    0x1d4ba as ::core::ffi::c_int as pg_wchar,
    0x1d4ba as ::core::ffi::c_int as pg_wchar,
    0x1d4bc as ::core::ffi::c_int as pg_wchar,
    0x1d4bc as ::core::ffi::c_int as pg_wchar,
    0x1d4c1 as ::core::ffi::c_int as pg_wchar,
    0x1d4c1 as ::core::ffi::c_int as pg_wchar,
    0x1d4c4 as ::core::ffi::c_int as pg_wchar,
    0x1d4c4 as ::core::ffi::c_int as pg_wchar,
    0x1d506 as ::core::ffi::c_int as pg_wchar,
    0x1d506 as ::core::ffi::c_int as pg_wchar,
    0x1d50b as ::core::ffi::c_int as pg_wchar,
    0x1d50c as ::core::ffi::c_int as pg_wchar,
    0x1d515 as ::core::ffi::c_int as pg_wchar,
    0x1d515 as ::core::ffi::c_int as pg_wchar,
    0x1d51d as ::core::ffi::c_int as pg_wchar,
    0x1d51d as ::core::ffi::c_int as pg_wchar,
    0x1d53a as ::core::ffi::c_int as pg_wchar,
    0x1d53a as ::core::ffi::c_int as pg_wchar,
    0x1d53f as ::core::ffi::c_int as pg_wchar,
    0x1d53f as ::core::ffi::c_int as pg_wchar,
    0x1d545 as ::core::ffi::c_int as pg_wchar,
    0x1d545 as ::core::ffi::c_int as pg_wchar,
    0x1d547 as ::core::ffi::c_int as pg_wchar,
    0x1d549 as ::core::ffi::c_int as pg_wchar,
    0x1d551 as ::core::ffi::c_int as pg_wchar,
    0x1d551 as ::core::ffi::c_int as pg_wchar,
    0x1d6a4 as ::core::ffi::c_int as pg_wchar,
    0x1d6a7 as ::core::ffi::c_int as pg_wchar,
    0x1d7ca as ::core::ffi::c_int as pg_wchar,
    0x1d7cd as ::core::ffi::c_int as pg_wchar,
    0x1d800 as ::core::ffi::c_int as pg_wchar,
    0x1fffd as ::core::ffi::c_int as pg_wchar,
    0x2a6d7 as ::core::ffi::c_int as pg_wchar,
    0x2f7ff as ::core::ffi::c_int as pg_wchar,
    0x2fa1e as ::core::ffi::c_int as pg_wchar,
    0x2fffd as ::core::ffi::c_int as pg_wchar,
    0x30000 as ::core::ffi::c_int as pg_wchar,
    0x3fffd as ::core::ffi::c_int as pg_wchar,
    0x40000 as ::core::ffi::c_int as pg_wchar,
    0x4fffd as ::core::ffi::c_int as pg_wchar,
    0x50000 as ::core::ffi::c_int as pg_wchar,
    0x5fffd as ::core::ffi::c_int as pg_wchar,
    0x60000 as ::core::ffi::c_int as pg_wchar,
    0x6fffd as ::core::ffi::c_int as pg_wchar,
    0x70000 as ::core::ffi::c_int as pg_wchar,
    0x7fffd as ::core::ffi::c_int as pg_wchar,
    0x80000 as ::core::ffi::c_int as pg_wchar,
    0x8fffd as ::core::ffi::c_int as pg_wchar,
    0x90000 as ::core::ffi::c_int as pg_wchar,
    0x9fffd as ::core::ffi::c_int as pg_wchar,
    0xa0000 as ::core::ffi::c_int as pg_wchar,
    0xafffd as ::core::ffi::c_int as pg_wchar,
    0xb0000 as ::core::ffi::c_int as pg_wchar,
    0xbfffd as ::core::ffi::c_int as pg_wchar,
    0xc0000 as ::core::ffi::c_int as pg_wchar,
    0xcfffd as ::core::ffi::c_int as pg_wchar,
    0xd0000 as ::core::ffi::c_int as pg_wchar,
    0xdfffd as ::core::ffi::c_int as pg_wchar,
    0xe0000 as ::core::ffi::c_int as pg_wchar,
    0xe0000 as ::core::ffi::c_int as pg_wchar,
    0xe0002 as ::core::ffi::c_int as pg_wchar,
    0xe001f as ::core::ffi::c_int as pg_wchar,
    0xe0080 as ::core::ffi::c_int as pg_wchar,
    0xefffd as ::core::ffi::c_int as pg_wchar,
];

static mut RandALCat_codepoint_ranges: [pg_wchar; 68] = [
    0x5be as ::core::ffi::c_int as pg_wchar,
    0x5be as ::core::ffi::c_int as pg_wchar,
    0x5c0 as ::core::ffi::c_int as pg_wchar,
    0x5c0 as ::core::ffi::c_int as pg_wchar,
    0x5c3 as ::core::ffi::c_int as pg_wchar,
    0x5c3 as ::core::ffi::c_int as pg_wchar,
    0x5d0 as ::core::ffi::c_int as pg_wchar,
    0x5ea as ::core::ffi::c_int as pg_wchar,
    0x5f0 as ::core::ffi::c_int as pg_wchar,
    0x5f4 as ::core::ffi::c_int as pg_wchar,
    0x61b as ::core::ffi::c_int as pg_wchar,
    0x61b as ::core::ffi::c_int as pg_wchar,
    0x61f as ::core::ffi::c_int as pg_wchar,
    0x61f as ::core::ffi::c_int as pg_wchar,
    0x621 as ::core::ffi::c_int as pg_wchar,
    0x63a as ::core::ffi::c_int as pg_wchar,
    0x640 as ::core::ffi::c_int as pg_wchar,
    0x64a as ::core::ffi::c_int as pg_wchar,
    0x66d as ::core::ffi::c_int as pg_wchar,
    0x66f as ::core::ffi::c_int as pg_wchar,
    0x671 as ::core::ffi::c_int as pg_wchar,
    0x6d5 as ::core::ffi::c_int as pg_wchar,
    0x6dd as ::core::ffi::c_int as pg_wchar,
    0x6dd as ::core::ffi::c_int as pg_wchar,
    0x6e5 as ::core::ffi::c_int as pg_wchar,
    0x6e6 as ::core::ffi::c_int as pg_wchar,
    0x6fa as ::core::ffi::c_int as pg_wchar,
    0x6fe as ::core::ffi::c_int as pg_wchar,
    0x700 as ::core::ffi::c_int as pg_wchar,
    0x70d as ::core::ffi::c_int as pg_wchar,
    0x710 as ::core::ffi::c_int as pg_wchar,
    0x710 as ::core::ffi::c_int as pg_wchar,
    0x712 as ::core::ffi::c_int as pg_wchar,
    0x72c as ::core::ffi::c_int as pg_wchar,
    0x780 as ::core::ffi::c_int as pg_wchar,
    0x7a5 as ::core::ffi::c_int as pg_wchar,
    0x7b1 as ::core::ffi::c_int as pg_wchar,
    0x7b1 as ::core::ffi::c_int as pg_wchar,
    0x200f as ::core::ffi::c_int as pg_wchar,
    0x200f as ::core::ffi::c_int as pg_wchar,
    0xfb1d as ::core::ffi::c_int as pg_wchar,
    0xfb1d as ::core::ffi::c_int as pg_wchar,
    0xfb1f as ::core::ffi::c_int as pg_wchar,
    0xfb28 as ::core::ffi::c_int as pg_wchar,
    0xfb2a as ::core::ffi::c_int as pg_wchar,
    0xfb36 as ::core::ffi::c_int as pg_wchar,
    0xfb38 as ::core::ffi::c_int as pg_wchar,
    0xfb3c as ::core::ffi::c_int as pg_wchar,
    0xfb3e as ::core::ffi::c_int as pg_wchar,
    0xfb3e as ::core::ffi::c_int as pg_wchar,
    0xfb40 as ::core::ffi::c_int as pg_wchar,
    0xfb41 as ::core::ffi::c_int as pg_wchar,
    0xfb43 as ::core::ffi::c_int as pg_wchar,
    0xfb44 as ::core::ffi::c_int as pg_wchar,
    0xfb46 as ::core::ffi::c_int as pg_wchar,
    0xfbb1 as ::core::ffi::c_int as pg_wchar,
    0xfbd3 as ::core::ffi::c_int as pg_wchar,
    0xfd3d as ::core::ffi::c_int as pg_wchar,
    0xfd50 as ::core::ffi::c_int as pg_wchar,
    0xfd8f as ::core::ffi::c_int as pg_wchar,
    0xfd92 as ::core::ffi::c_int as pg_wchar,
    0xfdc7 as ::core::ffi::c_int as pg_wchar,
    0xfdf0 as ::core::ffi::c_int as pg_wchar,
    0xfdfc as ::core::ffi::c_int as pg_wchar,
    0xfe70 as ::core::ffi::c_int as pg_wchar,
    0xfe74 as ::core::ffi::c_int as pg_wchar,
    0xfe76 as ::core::ffi::c_int as pg_wchar,
    0xfefc as ::core::ffi::c_int as pg_wchar,
];

static mut LCat_codepoint_ranges: [pg_wchar; 720] = [
    0x41 as ::core::ffi::c_int as pg_wchar,
    0x5a as ::core::ffi::c_int as pg_wchar,
    0x61 as ::core::ffi::c_int as pg_wchar,
    0x7a as ::core::ffi::c_int as pg_wchar,
    0xaa as ::core::ffi::c_int as pg_wchar,
    0xaa as ::core::ffi::c_int as pg_wchar,
    0xb5 as ::core::ffi::c_int as pg_wchar,
    0xb5 as ::core::ffi::c_int as pg_wchar,
    0xba as ::core::ffi::c_int as pg_wchar,
    0xba as ::core::ffi::c_int as pg_wchar,
    0xc0 as ::core::ffi::c_int as pg_wchar,
    0xd6 as ::core::ffi::c_int as pg_wchar,
    0xd8 as ::core::ffi::c_int as pg_wchar,
    0xf6 as ::core::ffi::c_int as pg_wchar,
    0xf8 as ::core::ffi::c_int as pg_wchar,
    0x220 as ::core::ffi::c_int as pg_wchar,
    0x222 as ::core::ffi::c_int as pg_wchar,
    0x233 as ::core::ffi::c_int as pg_wchar,
    0x250 as ::core::ffi::c_int as pg_wchar,
    0x2ad as ::core::ffi::c_int as pg_wchar,
    0x2b0 as ::core::ffi::c_int as pg_wchar,
    0x2b8 as ::core::ffi::c_int as pg_wchar,
    0x2bb as ::core::ffi::c_int as pg_wchar,
    0x2c1 as ::core::ffi::c_int as pg_wchar,
    0x2d0 as ::core::ffi::c_int as pg_wchar,
    0x2d1 as ::core::ffi::c_int as pg_wchar,
    0x2e0 as ::core::ffi::c_int as pg_wchar,
    0x2e4 as ::core::ffi::c_int as pg_wchar,
    0x2ee as ::core::ffi::c_int as pg_wchar,
    0x2ee as ::core::ffi::c_int as pg_wchar,
    0x37a as ::core::ffi::c_int as pg_wchar,
    0x37a as ::core::ffi::c_int as pg_wchar,
    0x386 as ::core::ffi::c_int as pg_wchar,
    0x386 as ::core::ffi::c_int as pg_wchar,
    0x388 as ::core::ffi::c_int as pg_wchar,
    0x38a as ::core::ffi::c_int as pg_wchar,
    0x38c as ::core::ffi::c_int as pg_wchar,
    0x38c as ::core::ffi::c_int as pg_wchar,
    0x38e as ::core::ffi::c_int as pg_wchar,
    0x3a1 as ::core::ffi::c_int as pg_wchar,
    0x3a3 as ::core::ffi::c_int as pg_wchar,
    0x3ce as ::core::ffi::c_int as pg_wchar,
    0x3d0 as ::core::ffi::c_int as pg_wchar,
    0x3f5 as ::core::ffi::c_int as pg_wchar,
    0x400 as ::core::ffi::c_int as pg_wchar,
    0x482 as ::core::ffi::c_int as pg_wchar,
    0x48a as ::core::ffi::c_int as pg_wchar,
    0x4ce as ::core::ffi::c_int as pg_wchar,
    0x4d0 as ::core::ffi::c_int as pg_wchar,
    0x4f5 as ::core::ffi::c_int as pg_wchar,
    0x4f8 as ::core::ffi::c_int as pg_wchar,
    0x4f9 as ::core::ffi::c_int as pg_wchar,
    0x500 as ::core::ffi::c_int as pg_wchar,
    0x50f as ::core::ffi::c_int as pg_wchar,
    0x531 as ::core::ffi::c_int as pg_wchar,
    0x556 as ::core::ffi::c_int as pg_wchar,
    0x559 as ::core::ffi::c_int as pg_wchar,
    0x55f as ::core::ffi::c_int as pg_wchar,
    0x561 as ::core::ffi::c_int as pg_wchar,
    0x587 as ::core::ffi::c_int as pg_wchar,
    0x589 as ::core::ffi::c_int as pg_wchar,
    0x589 as ::core::ffi::c_int as pg_wchar,
    0x903 as ::core::ffi::c_int as pg_wchar,
    0x903 as ::core::ffi::c_int as pg_wchar,
    0x905 as ::core::ffi::c_int as pg_wchar,
    0x939 as ::core::ffi::c_int as pg_wchar,
    0x93d as ::core::ffi::c_int as pg_wchar,
    0x940 as ::core::ffi::c_int as pg_wchar,
    0x949 as ::core::ffi::c_int as pg_wchar,
    0x94c as ::core::ffi::c_int as pg_wchar,
    0x950 as ::core::ffi::c_int as pg_wchar,
    0x950 as ::core::ffi::c_int as pg_wchar,
    0x958 as ::core::ffi::c_int as pg_wchar,
    0x961 as ::core::ffi::c_int as pg_wchar,
    0x964 as ::core::ffi::c_int as pg_wchar,
    0x970 as ::core::ffi::c_int as pg_wchar,
    0x982 as ::core::ffi::c_int as pg_wchar,
    0x983 as ::core::ffi::c_int as pg_wchar,
    0x985 as ::core::ffi::c_int as pg_wchar,
    0x98c as ::core::ffi::c_int as pg_wchar,
    0x98f as ::core::ffi::c_int as pg_wchar,
    0x990 as ::core::ffi::c_int as pg_wchar,
    0x993 as ::core::ffi::c_int as pg_wchar,
    0x9a8 as ::core::ffi::c_int as pg_wchar,
    0x9aa as ::core::ffi::c_int as pg_wchar,
    0x9b0 as ::core::ffi::c_int as pg_wchar,
    0x9b2 as ::core::ffi::c_int as pg_wchar,
    0x9b2 as ::core::ffi::c_int as pg_wchar,
    0x9b6 as ::core::ffi::c_int as pg_wchar,
    0x9b9 as ::core::ffi::c_int as pg_wchar,
    0x9be as ::core::ffi::c_int as pg_wchar,
    0x9c0 as ::core::ffi::c_int as pg_wchar,
    0x9c7 as ::core::ffi::c_int as pg_wchar,
    0x9c8 as ::core::ffi::c_int as pg_wchar,
    0x9cb as ::core::ffi::c_int as pg_wchar,
    0x9cc as ::core::ffi::c_int as pg_wchar,
    0x9d7 as ::core::ffi::c_int as pg_wchar,
    0x9d7 as ::core::ffi::c_int as pg_wchar,
    0x9dc as ::core::ffi::c_int as pg_wchar,
    0x9dd as ::core::ffi::c_int as pg_wchar,
    0x9df as ::core::ffi::c_int as pg_wchar,
    0x9e1 as ::core::ffi::c_int as pg_wchar,
    0x9e6 as ::core::ffi::c_int as pg_wchar,
    0x9f1 as ::core::ffi::c_int as pg_wchar,
    0x9f4 as ::core::ffi::c_int as pg_wchar,
    0x9fa as ::core::ffi::c_int as pg_wchar,
    0xa05 as ::core::ffi::c_int as pg_wchar,
    0xa0a as ::core::ffi::c_int as pg_wchar,
    0xa0f as ::core::ffi::c_int as pg_wchar,
    0xa10 as ::core::ffi::c_int as pg_wchar,
    0xa13 as ::core::ffi::c_int as pg_wchar,
    0xa28 as ::core::ffi::c_int as pg_wchar,
    0xa2a as ::core::ffi::c_int as pg_wchar,
    0xa30 as ::core::ffi::c_int as pg_wchar,
    0xa32 as ::core::ffi::c_int as pg_wchar,
    0xa33 as ::core::ffi::c_int as pg_wchar,
    0xa35 as ::core::ffi::c_int as pg_wchar,
    0xa36 as ::core::ffi::c_int as pg_wchar,
    0xa38 as ::core::ffi::c_int as pg_wchar,
    0xa39 as ::core::ffi::c_int as pg_wchar,
    0xa3e as ::core::ffi::c_int as pg_wchar,
    0xa40 as ::core::ffi::c_int as pg_wchar,
    0xa59 as ::core::ffi::c_int as pg_wchar,
    0xa5c as ::core::ffi::c_int as pg_wchar,
    0xa5e as ::core::ffi::c_int as pg_wchar,
    0xa5e as ::core::ffi::c_int as pg_wchar,
    0xa66 as ::core::ffi::c_int as pg_wchar,
    0xa6f as ::core::ffi::c_int as pg_wchar,
    0xa72 as ::core::ffi::c_int as pg_wchar,
    0xa74 as ::core::ffi::c_int as pg_wchar,
    0xa83 as ::core::ffi::c_int as pg_wchar,
    0xa83 as ::core::ffi::c_int as pg_wchar,
    0xa85 as ::core::ffi::c_int as pg_wchar,
    0xa8b as ::core::ffi::c_int as pg_wchar,
    0xa8d as ::core::ffi::c_int as pg_wchar,
    0xa8d as ::core::ffi::c_int as pg_wchar,
    0xa8f as ::core::ffi::c_int as pg_wchar,
    0xa91 as ::core::ffi::c_int as pg_wchar,
    0xa93 as ::core::ffi::c_int as pg_wchar,
    0xaa8 as ::core::ffi::c_int as pg_wchar,
    0xaaa as ::core::ffi::c_int as pg_wchar,
    0xab0 as ::core::ffi::c_int as pg_wchar,
    0xab2 as ::core::ffi::c_int as pg_wchar,
    0xab3 as ::core::ffi::c_int as pg_wchar,
    0xab5 as ::core::ffi::c_int as pg_wchar,
    0xab9 as ::core::ffi::c_int as pg_wchar,
    0xabd as ::core::ffi::c_int as pg_wchar,
    0xac0 as ::core::ffi::c_int as pg_wchar,
    0xac9 as ::core::ffi::c_int as pg_wchar,
    0xac9 as ::core::ffi::c_int as pg_wchar,
    0xacb as ::core::ffi::c_int as pg_wchar,
    0xacc as ::core::ffi::c_int as pg_wchar,
    0xad0 as ::core::ffi::c_int as pg_wchar,
    0xad0 as ::core::ffi::c_int as pg_wchar,
    0xae0 as ::core::ffi::c_int as pg_wchar,
    0xae0 as ::core::ffi::c_int as pg_wchar,
    0xae6 as ::core::ffi::c_int as pg_wchar,
    0xaef as ::core::ffi::c_int as pg_wchar,
    0xb02 as ::core::ffi::c_int as pg_wchar,
    0xb03 as ::core::ffi::c_int as pg_wchar,
    0xb05 as ::core::ffi::c_int as pg_wchar,
    0xb0c as ::core::ffi::c_int as pg_wchar,
    0xb0f as ::core::ffi::c_int as pg_wchar,
    0xb10 as ::core::ffi::c_int as pg_wchar,
    0xb13 as ::core::ffi::c_int as pg_wchar,
    0xb28 as ::core::ffi::c_int as pg_wchar,
    0xb2a as ::core::ffi::c_int as pg_wchar,
    0xb30 as ::core::ffi::c_int as pg_wchar,
    0xb32 as ::core::ffi::c_int as pg_wchar,
    0xb33 as ::core::ffi::c_int as pg_wchar,
    0xb36 as ::core::ffi::c_int as pg_wchar,
    0xb39 as ::core::ffi::c_int as pg_wchar,
    0xb3d as ::core::ffi::c_int as pg_wchar,
    0xb3e as ::core::ffi::c_int as pg_wchar,
    0xb40 as ::core::ffi::c_int as pg_wchar,
    0xb40 as ::core::ffi::c_int as pg_wchar,
    0xb47 as ::core::ffi::c_int as pg_wchar,
    0xb48 as ::core::ffi::c_int as pg_wchar,
    0xb4b as ::core::ffi::c_int as pg_wchar,
    0xb4c as ::core::ffi::c_int as pg_wchar,
    0xb57 as ::core::ffi::c_int as pg_wchar,
    0xb57 as ::core::ffi::c_int as pg_wchar,
    0xb5c as ::core::ffi::c_int as pg_wchar,
    0xb5d as ::core::ffi::c_int as pg_wchar,
    0xb5f as ::core::ffi::c_int as pg_wchar,
    0xb61 as ::core::ffi::c_int as pg_wchar,
    0xb66 as ::core::ffi::c_int as pg_wchar,
    0xb70 as ::core::ffi::c_int as pg_wchar,
    0xb83 as ::core::ffi::c_int as pg_wchar,
    0xb83 as ::core::ffi::c_int as pg_wchar,
    0xb85 as ::core::ffi::c_int as pg_wchar,
    0xb8a as ::core::ffi::c_int as pg_wchar,
    0xb8e as ::core::ffi::c_int as pg_wchar,
    0xb90 as ::core::ffi::c_int as pg_wchar,
    0xb92 as ::core::ffi::c_int as pg_wchar,
    0xb95 as ::core::ffi::c_int as pg_wchar,
    0xb99 as ::core::ffi::c_int as pg_wchar,
    0xb9a as ::core::ffi::c_int as pg_wchar,
    0xb9c as ::core::ffi::c_int as pg_wchar,
    0xb9c as ::core::ffi::c_int as pg_wchar,
    0xb9e as ::core::ffi::c_int as pg_wchar,
    0xb9f as ::core::ffi::c_int as pg_wchar,
    0xba3 as ::core::ffi::c_int as pg_wchar,
    0xba4 as ::core::ffi::c_int as pg_wchar,
    0xba8 as ::core::ffi::c_int as pg_wchar,
    0xbaa as ::core::ffi::c_int as pg_wchar,
    0xbae as ::core::ffi::c_int as pg_wchar,
    0xbb5 as ::core::ffi::c_int as pg_wchar,
    0xbb7 as ::core::ffi::c_int as pg_wchar,
    0xbb9 as ::core::ffi::c_int as pg_wchar,
    0xbbe as ::core::ffi::c_int as pg_wchar,
    0xbbf as ::core::ffi::c_int as pg_wchar,
    0xbc1 as ::core::ffi::c_int as pg_wchar,
    0xbc2 as ::core::ffi::c_int as pg_wchar,
    0xbc6 as ::core::ffi::c_int as pg_wchar,
    0xbc8 as ::core::ffi::c_int as pg_wchar,
    0xbca as ::core::ffi::c_int as pg_wchar,
    0xbcc as ::core::ffi::c_int as pg_wchar,
    0xbd7 as ::core::ffi::c_int as pg_wchar,
    0xbd7 as ::core::ffi::c_int as pg_wchar,
    0xbe7 as ::core::ffi::c_int as pg_wchar,
    0xbf2 as ::core::ffi::c_int as pg_wchar,
    0xc01 as ::core::ffi::c_int as pg_wchar,
    0xc03 as ::core::ffi::c_int as pg_wchar,
    0xc05 as ::core::ffi::c_int as pg_wchar,
    0xc0c as ::core::ffi::c_int as pg_wchar,
    0xc0e as ::core::ffi::c_int as pg_wchar,
    0xc10 as ::core::ffi::c_int as pg_wchar,
    0xc12 as ::core::ffi::c_int as pg_wchar,
    0xc28 as ::core::ffi::c_int as pg_wchar,
    0xc2a as ::core::ffi::c_int as pg_wchar,
    0xc33 as ::core::ffi::c_int as pg_wchar,
    0xc35 as ::core::ffi::c_int as pg_wchar,
    0xc39 as ::core::ffi::c_int as pg_wchar,
    0xc41 as ::core::ffi::c_int as pg_wchar,
    0xc44 as ::core::ffi::c_int as pg_wchar,
    0xc60 as ::core::ffi::c_int as pg_wchar,
    0xc61 as ::core::ffi::c_int as pg_wchar,
    0xc66 as ::core::ffi::c_int as pg_wchar,
    0xc6f as ::core::ffi::c_int as pg_wchar,
    0xc82 as ::core::ffi::c_int as pg_wchar,
    0xc83 as ::core::ffi::c_int as pg_wchar,
    0xc85 as ::core::ffi::c_int as pg_wchar,
    0xc8c as ::core::ffi::c_int as pg_wchar,
    0xc8e as ::core::ffi::c_int as pg_wchar,
    0xc90 as ::core::ffi::c_int as pg_wchar,
    0xc92 as ::core::ffi::c_int as pg_wchar,
    0xca8 as ::core::ffi::c_int as pg_wchar,
    0xcaa as ::core::ffi::c_int as pg_wchar,
    0xcb3 as ::core::ffi::c_int as pg_wchar,
    0xcb5 as ::core::ffi::c_int as pg_wchar,
    0xcb9 as ::core::ffi::c_int as pg_wchar,
    0xcbe as ::core::ffi::c_int as pg_wchar,
    0xcbe as ::core::ffi::c_int as pg_wchar,
    0xcc0 as ::core::ffi::c_int as pg_wchar,
    0xcc4 as ::core::ffi::c_int as pg_wchar,
    0xcc7 as ::core::ffi::c_int as pg_wchar,
    0xcc8 as ::core::ffi::c_int as pg_wchar,
    0xcca as ::core::ffi::c_int as pg_wchar,
    0xccb as ::core::ffi::c_int as pg_wchar,
    0xcd5 as ::core::ffi::c_int as pg_wchar,
    0xcd6 as ::core::ffi::c_int as pg_wchar,
    0xcde as ::core::ffi::c_int as pg_wchar,
    0xcde as ::core::ffi::c_int as pg_wchar,
    0xce0 as ::core::ffi::c_int as pg_wchar,
    0xce1 as ::core::ffi::c_int as pg_wchar,
    0xce6 as ::core::ffi::c_int as pg_wchar,
    0xcef as ::core::ffi::c_int as pg_wchar,
    0xd02 as ::core::ffi::c_int as pg_wchar,
    0xd03 as ::core::ffi::c_int as pg_wchar,
    0xd05 as ::core::ffi::c_int as pg_wchar,
    0xd0c as ::core::ffi::c_int as pg_wchar,
    0xd0e as ::core::ffi::c_int as pg_wchar,
    0xd10 as ::core::ffi::c_int as pg_wchar,
    0xd12 as ::core::ffi::c_int as pg_wchar,
    0xd28 as ::core::ffi::c_int as pg_wchar,
    0xd2a as ::core::ffi::c_int as pg_wchar,
    0xd39 as ::core::ffi::c_int as pg_wchar,
    0xd3e as ::core::ffi::c_int as pg_wchar,
    0xd40 as ::core::ffi::c_int as pg_wchar,
    0xd46 as ::core::ffi::c_int as pg_wchar,
    0xd48 as ::core::ffi::c_int as pg_wchar,
    0xd4a as ::core::ffi::c_int as pg_wchar,
    0xd4c as ::core::ffi::c_int as pg_wchar,
    0xd57 as ::core::ffi::c_int as pg_wchar,
    0xd57 as ::core::ffi::c_int as pg_wchar,
    0xd60 as ::core::ffi::c_int as pg_wchar,
    0xd61 as ::core::ffi::c_int as pg_wchar,
    0xd66 as ::core::ffi::c_int as pg_wchar,
    0xd6f as ::core::ffi::c_int as pg_wchar,
    0xd82 as ::core::ffi::c_int as pg_wchar,
    0xd83 as ::core::ffi::c_int as pg_wchar,
    0xd85 as ::core::ffi::c_int as pg_wchar,
    0xd96 as ::core::ffi::c_int as pg_wchar,
    0xd9a as ::core::ffi::c_int as pg_wchar,
    0xdb1 as ::core::ffi::c_int as pg_wchar,
    0xdb3 as ::core::ffi::c_int as pg_wchar,
    0xdbb as ::core::ffi::c_int as pg_wchar,
    0xdbd as ::core::ffi::c_int as pg_wchar,
    0xdbd as ::core::ffi::c_int as pg_wchar,
    0xdc0 as ::core::ffi::c_int as pg_wchar,
    0xdc6 as ::core::ffi::c_int as pg_wchar,
    0xdcf as ::core::ffi::c_int as pg_wchar,
    0xdd1 as ::core::ffi::c_int as pg_wchar,
    0xdd8 as ::core::ffi::c_int as pg_wchar,
    0xddf as ::core::ffi::c_int as pg_wchar,
    0xdf2 as ::core::ffi::c_int as pg_wchar,
    0xdf4 as ::core::ffi::c_int as pg_wchar,
    0xe01 as ::core::ffi::c_int as pg_wchar,
    0xe30 as ::core::ffi::c_int as pg_wchar,
    0xe32 as ::core::ffi::c_int as pg_wchar,
    0xe33 as ::core::ffi::c_int as pg_wchar,
    0xe40 as ::core::ffi::c_int as pg_wchar,
    0xe46 as ::core::ffi::c_int as pg_wchar,
    0xe4f as ::core::ffi::c_int as pg_wchar,
    0xe5b as ::core::ffi::c_int as pg_wchar,
    0xe81 as ::core::ffi::c_int as pg_wchar,
    0xe82 as ::core::ffi::c_int as pg_wchar,
    0xe84 as ::core::ffi::c_int as pg_wchar,
    0xe84 as ::core::ffi::c_int as pg_wchar,
    0xe87 as ::core::ffi::c_int as pg_wchar,
    0xe88 as ::core::ffi::c_int as pg_wchar,
    0xe8a as ::core::ffi::c_int as pg_wchar,
    0xe8a as ::core::ffi::c_int as pg_wchar,
    0xe8d as ::core::ffi::c_int as pg_wchar,
    0xe8d as ::core::ffi::c_int as pg_wchar,
    0xe94 as ::core::ffi::c_int as pg_wchar,
    0xe97 as ::core::ffi::c_int as pg_wchar,
    0xe99 as ::core::ffi::c_int as pg_wchar,
    0xe9f as ::core::ffi::c_int as pg_wchar,
    0xea1 as ::core::ffi::c_int as pg_wchar,
    0xea3 as ::core::ffi::c_int as pg_wchar,
    0xea5 as ::core::ffi::c_int as pg_wchar,
    0xea5 as ::core::ffi::c_int as pg_wchar,
    0xea7 as ::core::ffi::c_int as pg_wchar,
    0xea7 as ::core::ffi::c_int as pg_wchar,
    0xeaa as ::core::ffi::c_int as pg_wchar,
    0xeab as ::core::ffi::c_int as pg_wchar,
    0xead as ::core::ffi::c_int as pg_wchar,
    0xeb0 as ::core::ffi::c_int as pg_wchar,
    0xeb2 as ::core::ffi::c_int as pg_wchar,
    0xeb3 as ::core::ffi::c_int as pg_wchar,
    0xebd as ::core::ffi::c_int as pg_wchar,
    0xebd as ::core::ffi::c_int as pg_wchar,
    0xec0 as ::core::ffi::c_int as pg_wchar,
    0xec4 as ::core::ffi::c_int as pg_wchar,
    0xec6 as ::core::ffi::c_int as pg_wchar,
    0xec6 as ::core::ffi::c_int as pg_wchar,
    0xed0 as ::core::ffi::c_int as pg_wchar,
    0xed9 as ::core::ffi::c_int as pg_wchar,
    0xedc as ::core::ffi::c_int as pg_wchar,
    0xedd as ::core::ffi::c_int as pg_wchar,
    0xf00 as ::core::ffi::c_int as pg_wchar,
    0xf17 as ::core::ffi::c_int as pg_wchar,
    0xf1a as ::core::ffi::c_int as pg_wchar,
    0xf34 as ::core::ffi::c_int as pg_wchar,
    0xf36 as ::core::ffi::c_int as pg_wchar,
    0xf36 as ::core::ffi::c_int as pg_wchar,
    0xf38 as ::core::ffi::c_int as pg_wchar,
    0xf38 as ::core::ffi::c_int as pg_wchar,
    0xf3e as ::core::ffi::c_int as pg_wchar,
    0xf47 as ::core::ffi::c_int as pg_wchar,
    0xf49 as ::core::ffi::c_int as pg_wchar,
    0xf6a as ::core::ffi::c_int as pg_wchar,
    0xf7f as ::core::ffi::c_int as pg_wchar,
    0xf7f as ::core::ffi::c_int as pg_wchar,
    0xf85 as ::core::ffi::c_int as pg_wchar,
    0xf85 as ::core::ffi::c_int as pg_wchar,
    0xf88 as ::core::ffi::c_int as pg_wchar,
    0xf8b as ::core::ffi::c_int as pg_wchar,
    0xfbe as ::core::ffi::c_int as pg_wchar,
    0xfc5 as ::core::ffi::c_int as pg_wchar,
    0xfc7 as ::core::ffi::c_int as pg_wchar,
    0xfcc as ::core::ffi::c_int as pg_wchar,
    0xfcf as ::core::ffi::c_int as pg_wchar,
    0xfcf as ::core::ffi::c_int as pg_wchar,
    0x1000 as ::core::ffi::c_int as pg_wchar,
    0x1021 as ::core::ffi::c_int as pg_wchar,
    0x1023 as ::core::ffi::c_int as pg_wchar,
    0x1027 as ::core::ffi::c_int as pg_wchar,
    0x1029 as ::core::ffi::c_int as pg_wchar,
    0x102a as ::core::ffi::c_int as pg_wchar,
    0x102c as ::core::ffi::c_int as pg_wchar,
    0x102c as ::core::ffi::c_int as pg_wchar,
    0x1031 as ::core::ffi::c_int as pg_wchar,
    0x1031 as ::core::ffi::c_int as pg_wchar,
    0x1038 as ::core::ffi::c_int as pg_wchar,
    0x1038 as ::core::ffi::c_int as pg_wchar,
    0x1040 as ::core::ffi::c_int as pg_wchar,
    0x1057 as ::core::ffi::c_int as pg_wchar,
    0x10a0 as ::core::ffi::c_int as pg_wchar,
    0x10c5 as ::core::ffi::c_int as pg_wchar,
    0x10d0 as ::core::ffi::c_int as pg_wchar,
    0x10f8 as ::core::ffi::c_int as pg_wchar,
    0x10fb as ::core::ffi::c_int as pg_wchar,
    0x10fb as ::core::ffi::c_int as pg_wchar,
    0x1100 as ::core::ffi::c_int as pg_wchar,
    0x1159 as ::core::ffi::c_int as pg_wchar,
    0x115f as ::core::ffi::c_int as pg_wchar,
    0x11a2 as ::core::ffi::c_int as pg_wchar,
    0x11a8 as ::core::ffi::c_int as pg_wchar,
    0x11f9 as ::core::ffi::c_int as pg_wchar,
    0x1200 as ::core::ffi::c_int as pg_wchar,
    0x1206 as ::core::ffi::c_int as pg_wchar,
    0x1208 as ::core::ffi::c_int as pg_wchar,
    0x1246 as ::core::ffi::c_int as pg_wchar,
    0x1248 as ::core::ffi::c_int as pg_wchar,
    0x1248 as ::core::ffi::c_int as pg_wchar,
    0x124a as ::core::ffi::c_int as pg_wchar,
    0x124d as ::core::ffi::c_int as pg_wchar,
    0x1250 as ::core::ffi::c_int as pg_wchar,
    0x1256 as ::core::ffi::c_int as pg_wchar,
    0x1258 as ::core::ffi::c_int as pg_wchar,
    0x1258 as ::core::ffi::c_int as pg_wchar,
    0x125a as ::core::ffi::c_int as pg_wchar,
    0x125d as ::core::ffi::c_int as pg_wchar,
    0x1260 as ::core::ffi::c_int as pg_wchar,
    0x1286 as ::core::ffi::c_int as pg_wchar,
    0x1288 as ::core::ffi::c_int as pg_wchar,
    0x1288 as ::core::ffi::c_int as pg_wchar,
    0x128a as ::core::ffi::c_int as pg_wchar,
    0x128d as ::core::ffi::c_int as pg_wchar,
    0x1290 as ::core::ffi::c_int as pg_wchar,
    0x12ae as ::core::ffi::c_int as pg_wchar,
    0x12b0 as ::core::ffi::c_int as pg_wchar,
    0x12b0 as ::core::ffi::c_int as pg_wchar,
    0x12b2 as ::core::ffi::c_int as pg_wchar,
    0x12b5 as ::core::ffi::c_int as pg_wchar,
    0x12b8 as ::core::ffi::c_int as pg_wchar,
    0x12be as ::core::ffi::c_int as pg_wchar,
    0x12c0 as ::core::ffi::c_int as pg_wchar,
    0x12c0 as ::core::ffi::c_int as pg_wchar,
    0x12c2 as ::core::ffi::c_int as pg_wchar,
    0x12c5 as ::core::ffi::c_int as pg_wchar,
    0x12c8 as ::core::ffi::c_int as pg_wchar,
    0x12ce as ::core::ffi::c_int as pg_wchar,
    0x12d0 as ::core::ffi::c_int as pg_wchar,
    0x12d6 as ::core::ffi::c_int as pg_wchar,
    0x12d8 as ::core::ffi::c_int as pg_wchar,
    0x12ee as ::core::ffi::c_int as pg_wchar,
    0x12f0 as ::core::ffi::c_int as pg_wchar,
    0x130e as ::core::ffi::c_int as pg_wchar,
    0x1310 as ::core::ffi::c_int as pg_wchar,
    0x1310 as ::core::ffi::c_int as pg_wchar,
    0x1312 as ::core::ffi::c_int as pg_wchar,
    0x1315 as ::core::ffi::c_int as pg_wchar,
    0x1318 as ::core::ffi::c_int as pg_wchar,
    0x131e as ::core::ffi::c_int as pg_wchar,
    0x1320 as ::core::ffi::c_int as pg_wchar,
    0x1346 as ::core::ffi::c_int as pg_wchar,
    0x1348 as ::core::ffi::c_int as pg_wchar,
    0x135a as ::core::ffi::c_int as pg_wchar,
    0x1361 as ::core::ffi::c_int as pg_wchar,
    0x137c as ::core::ffi::c_int as pg_wchar,
    0x13a0 as ::core::ffi::c_int as pg_wchar,
    0x13f4 as ::core::ffi::c_int as pg_wchar,
    0x1401 as ::core::ffi::c_int as pg_wchar,
    0x1676 as ::core::ffi::c_int as pg_wchar,
    0x1681 as ::core::ffi::c_int as pg_wchar,
    0x169a as ::core::ffi::c_int as pg_wchar,
    0x16a0 as ::core::ffi::c_int as pg_wchar,
    0x16f0 as ::core::ffi::c_int as pg_wchar,
    0x1700 as ::core::ffi::c_int as pg_wchar,
    0x170c as ::core::ffi::c_int as pg_wchar,
    0x170e as ::core::ffi::c_int as pg_wchar,
    0x1711 as ::core::ffi::c_int as pg_wchar,
    0x1720 as ::core::ffi::c_int as pg_wchar,
    0x1731 as ::core::ffi::c_int as pg_wchar,
    0x1735 as ::core::ffi::c_int as pg_wchar,
    0x1736 as ::core::ffi::c_int as pg_wchar,
    0x1740 as ::core::ffi::c_int as pg_wchar,
    0x1751 as ::core::ffi::c_int as pg_wchar,
    0x1760 as ::core::ffi::c_int as pg_wchar,
    0x176c as ::core::ffi::c_int as pg_wchar,
    0x176e as ::core::ffi::c_int as pg_wchar,
    0x1770 as ::core::ffi::c_int as pg_wchar,
    0x1780 as ::core::ffi::c_int as pg_wchar,
    0x17b6 as ::core::ffi::c_int as pg_wchar,
    0x17be as ::core::ffi::c_int as pg_wchar,
    0x17c5 as ::core::ffi::c_int as pg_wchar,
    0x17c7 as ::core::ffi::c_int as pg_wchar,
    0x17c8 as ::core::ffi::c_int as pg_wchar,
    0x17d4 as ::core::ffi::c_int as pg_wchar,
    0x17da as ::core::ffi::c_int as pg_wchar,
    0x17dc as ::core::ffi::c_int as pg_wchar,
    0x17dc as ::core::ffi::c_int as pg_wchar,
    0x17e0 as ::core::ffi::c_int as pg_wchar,
    0x17e9 as ::core::ffi::c_int as pg_wchar,
    0x1810 as ::core::ffi::c_int as pg_wchar,
    0x1819 as ::core::ffi::c_int as pg_wchar,
    0x1820 as ::core::ffi::c_int as pg_wchar,
    0x1877 as ::core::ffi::c_int as pg_wchar,
    0x1880 as ::core::ffi::c_int as pg_wchar,
    0x18a8 as ::core::ffi::c_int as pg_wchar,
    0x1e00 as ::core::ffi::c_int as pg_wchar,
    0x1e9b as ::core::ffi::c_int as pg_wchar,
    0x1ea0 as ::core::ffi::c_int as pg_wchar,
    0x1ef9 as ::core::ffi::c_int as pg_wchar,
    0x1f00 as ::core::ffi::c_int as pg_wchar,
    0x1f15 as ::core::ffi::c_int as pg_wchar,
    0x1f18 as ::core::ffi::c_int as pg_wchar,
    0x1f1d as ::core::ffi::c_int as pg_wchar,
    0x1f20 as ::core::ffi::c_int as pg_wchar,
    0x1f45 as ::core::ffi::c_int as pg_wchar,
    0x1f48 as ::core::ffi::c_int as pg_wchar,
    0x1f4d as ::core::ffi::c_int as pg_wchar,
    0x1f50 as ::core::ffi::c_int as pg_wchar,
    0x1f57 as ::core::ffi::c_int as pg_wchar,
    0x1f59 as ::core::ffi::c_int as pg_wchar,
    0x1f59 as ::core::ffi::c_int as pg_wchar,
    0x1f5b as ::core::ffi::c_int as pg_wchar,
    0x1f5b as ::core::ffi::c_int as pg_wchar,
    0x1f5d as ::core::ffi::c_int as pg_wchar,
    0x1f5d as ::core::ffi::c_int as pg_wchar,
    0x1f5f as ::core::ffi::c_int as pg_wchar,
    0x1f7d as ::core::ffi::c_int as pg_wchar,
    0x1f80 as ::core::ffi::c_int as pg_wchar,
    0x1fb4 as ::core::ffi::c_int as pg_wchar,
    0x1fb6 as ::core::ffi::c_int as pg_wchar,
    0x1fbc as ::core::ffi::c_int as pg_wchar,
    0x1fbe as ::core::ffi::c_int as pg_wchar,
    0x1fbe as ::core::ffi::c_int as pg_wchar,
    0x1fc2 as ::core::ffi::c_int as pg_wchar,
    0x1fc4 as ::core::ffi::c_int as pg_wchar,
    0x1fc6 as ::core::ffi::c_int as pg_wchar,
    0x1fcc as ::core::ffi::c_int as pg_wchar,
    0x1fd0 as ::core::ffi::c_int as pg_wchar,
    0x1fd3 as ::core::ffi::c_int as pg_wchar,
    0x1fd6 as ::core::ffi::c_int as pg_wchar,
    0x1fdb as ::core::ffi::c_int as pg_wchar,
    0x1fe0 as ::core::ffi::c_int as pg_wchar,
    0x1fec as ::core::ffi::c_int as pg_wchar,
    0x1ff2 as ::core::ffi::c_int as pg_wchar,
    0x1ff4 as ::core::ffi::c_int as pg_wchar,
    0x1ff6 as ::core::ffi::c_int as pg_wchar,
    0x1ffc as ::core::ffi::c_int as pg_wchar,
    0x200e as ::core::ffi::c_int as pg_wchar,
    0x200e as ::core::ffi::c_int as pg_wchar,
    0x2071 as ::core::ffi::c_int as pg_wchar,
    0x2071 as ::core::ffi::c_int as pg_wchar,
    0x207f as ::core::ffi::c_int as pg_wchar,
    0x207f as ::core::ffi::c_int as pg_wchar,
    0x2102 as ::core::ffi::c_int as pg_wchar,
    0x2102 as ::core::ffi::c_int as pg_wchar,
    0x2107 as ::core::ffi::c_int as pg_wchar,
    0x2107 as ::core::ffi::c_int as pg_wchar,
    0x210a as ::core::ffi::c_int as pg_wchar,
    0x2113 as ::core::ffi::c_int as pg_wchar,
    0x2115 as ::core::ffi::c_int as pg_wchar,
    0x2115 as ::core::ffi::c_int as pg_wchar,
    0x2119 as ::core::ffi::c_int as pg_wchar,
    0x211d as ::core::ffi::c_int as pg_wchar,
    0x2124 as ::core::ffi::c_int as pg_wchar,
    0x2124 as ::core::ffi::c_int as pg_wchar,
    0x2126 as ::core::ffi::c_int as pg_wchar,
    0x2126 as ::core::ffi::c_int as pg_wchar,
    0x2128 as ::core::ffi::c_int as pg_wchar,
    0x2128 as ::core::ffi::c_int as pg_wchar,
    0x212a as ::core::ffi::c_int as pg_wchar,
    0x212d as ::core::ffi::c_int as pg_wchar,
    0x212f as ::core::ffi::c_int as pg_wchar,
    0x2131 as ::core::ffi::c_int as pg_wchar,
    0x2133 as ::core::ffi::c_int as pg_wchar,
    0x2139 as ::core::ffi::c_int as pg_wchar,
    0x213d as ::core::ffi::c_int as pg_wchar,
    0x213f as ::core::ffi::c_int as pg_wchar,
    0x2145 as ::core::ffi::c_int as pg_wchar,
    0x2149 as ::core::ffi::c_int as pg_wchar,
    0x2160 as ::core::ffi::c_int as pg_wchar,
    0x2183 as ::core::ffi::c_int as pg_wchar,
    0x2336 as ::core::ffi::c_int as pg_wchar,
    0x237a as ::core::ffi::c_int as pg_wchar,
    0x2395 as ::core::ffi::c_int as pg_wchar,
    0x2395 as ::core::ffi::c_int as pg_wchar,
    0x249c as ::core::ffi::c_int as pg_wchar,
    0x24e9 as ::core::ffi::c_int as pg_wchar,
    0x3005 as ::core::ffi::c_int as pg_wchar,
    0x3007 as ::core::ffi::c_int as pg_wchar,
    0x3021 as ::core::ffi::c_int as pg_wchar,
    0x3029 as ::core::ffi::c_int as pg_wchar,
    0x3031 as ::core::ffi::c_int as pg_wchar,
    0x3035 as ::core::ffi::c_int as pg_wchar,
    0x3038 as ::core::ffi::c_int as pg_wchar,
    0x303c as ::core::ffi::c_int as pg_wchar,
    0x3041 as ::core::ffi::c_int as pg_wchar,
    0x3096 as ::core::ffi::c_int as pg_wchar,
    0x309d as ::core::ffi::c_int as pg_wchar,
    0x309f as ::core::ffi::c_int as pg_wchar,
    0x30a1 as ::core::ffi::c_int as pg_wchar,
    0x30fa as ::core::ffi::c_int as pg_wchar,
    0x30fc as ::core::ffi::c_int as pg_wchar,
    0x30ff as ::core::ffi::c_int as pg_wchar,
    0x3105 as ::core::ffi::c_int as pg_wchar,
    0x312c as ::core::ffi::c_int as pg_wchar,
    0x3131 as ::core::ffi::c_int as pg_wchar,
    0x318e as ::core::ffi::c_int as pg_wchar,
    0x3190 as ::core::ffi::c_int as pg_wchar,
    0x31b7 as ::core::ffi::c_int as pg_wchar,
    0x31f0 as ::core::ffi::c_int as pg_wchar,
    0x321c as ::core::ffi::c_int as pg_wchar,
    0x3220 as ::core::ffi::c_int as pg_wchar,
    0x3243 as ::core::ffi::c_int as pg_wchar,
    0x3260 as ::core::ffi::c_int as pg_wchar,
    0x327b as ::core::ffi::c_int as pg_wchar,
    0x327f as ::core::ffi::c_int as pg_wchar,
    0x32b0 as ::core::ffi::c_int as pg_wchar,
    0x32c0 as ::core::ffi::c_int as pg_wchar,
    0x32cb as ::core::ffi::c_int as pg_wchar,
    0x32d0 as ::core::ffi::c_int as pg_wchar,
    0x32fe as ::core::ffi::c_int as pg_wchar,
    0x3300 as ::core::ffi::c_int as pg_wchar,
    0x3376 as ::core::ffi::c_int as pg_wchar,
    0x337b as ::core::ffi::c_int as pg_wchar,
    0x33dd as ::core::ffi::c_int as pg_wchar,
    0x33e0 as ::core::ffi::c_int as pg_wchar,
    0x33fe as ::core::ffi::c_int as pg_wchar,
    0x3400 as ::core::ffi::c_int as pg_wchar,
    0x4db5 as ::core::ffi::c_int as pg_wchar,
    0x4e00 as ::core::ffi::c_int as pg_wchar,
    0x9fa5 as ::core::ffi::c_int as pg_wchar,
    0xa000 as ::core::ffi::c_int as pg_wchar,
    0xa48c as ::core::ffi::c_int as pg_wchar,
    0xac00 as ::core::ffi::c_int as pg_wchar,
    0xd7a3 as ::core::ffi::c_int as pg_wchar,
    0xd800 as ::core::ffi::c_int as pg_wchar,
    0xfa2d as ::core::ffi::c_int as pg_wchar,
    0xfa30 as ::core::ffi::c_int as pg_wchar,
    0xfa6a as ::core::ffi::c_int as pg_wchar,
    0xfb00 as ::core::ffi::c_int as pg_wchar,
    0xfb06 as ::core::ffi::c_int as pg_wchar,
    0xfb13 as ::core::ffi::c_int as pg_wchar,
    0xfb17 as ::core::ffi::c_int as pg_wchar,
    0xff21 as ::core::ffi::c_int as pg_wchar,
    0xff3a as ::core::ffi::c_int as pg_wchar,
    0xff41 as ::core::ffi::c_int as pg_wchar,
    0xff5a as ::core::ffi::c_int as pg_wchar,
    0xff66 as ::core::ffi::c_int as pg_wchar,
    0xffbe as ::core::ffi::c_int as pg_wchar,
    0xffc2 as ::core::ffi::c_int as pg_wchar,
    0xffc7 as ::core::ffi::c_int as pg_wchar,
    0xffca as ::core::ffi::c_int as pg_wchar,
    0xffcf as ::core::ffi::c_int as pg_wchar,
    0xffd2 as ::core::ffi::c_int as pg_wchar,
    0xffd7 as ::core::ffi::c_int as pg_wchar,
    0xffda as ::core::ffi::c_int as pg_wchar,
    0xffdc as ::core::ffi::c_int as pg_wchar,
    0x10300 as ::core::ffi::c_int as pg_wchar,
    0x1031e as ::core::ffi::c_int as pg_wchar,
    0x10320 as ::core::ffi::c_int as pg_wchar,
    0x10323 as ::core::ffi::c_int as pg_wchar,
    0x10330 as ::core::ffi::c_int as pg_wchar,
    0x1034a as ::core::ffi::c_int as pg_wchar,
    0x10400 as ::core::ffi::c_int as pg_wchar,
    0x10425 as ::core::ffi::c_int as pg_wchar,
    0x10428 as ::core::ffi::c_int as pg_wchar,
    0x1044d as ::core::ffi::c_int as pg_wchar,
    0x1d000 as ::core::ffi::c_int as pg_wchar,
    0x1d0f5 as ::core::ffi::c_int as pg_wchar,
    0x1d100 as ::core::ffi::c_int as pg_wchar,
    0x1d126 as ::core::ffi::c_int as pg_wchar,
    0x1d12a as ::core::ffi::c_int as pg_wchar,
    0x1d166 as ::core::ffi::c_int as pg_wchar,
    0x1d16a as ::core::ffi::c_int as pg_wchar,
    0x1d172 as ::core::ffi::c_int as pg_wchar,
    0x1d183 as ::core::ffi::c_int as pg_wchar,
    0x1d184 as ::core::ffi::c_int as pg_wchar,
    0x1d18c as ::core::ffi::c_int as pg_wchar,
    0x1d1a9 as ::core::ffi::c_int as pg_wchar,
    0x1d1ae as ::core::ffi::c_int as pg_wchar,
    0x1d1dd as ::core::ffi::c_int as pg_wchar,
    0x1d400 as ::core::ffi::c_int as pg_wchar,
    0x1d454 as ::core::ffi::c_int as pg_wchar,
    0x1d456 as ::core::ffi::c_int as pg_wchar,
    0x1d49c as ::core::ffi::c_int as pg_wchar,
    0x1d49e as ::core::ffi::c_int as pg_wchar,
    0x1d49f as ::core::ffi::c_int as pg_wchar,
    0x1d4a2 as ::core::ffi::c_int as pg_wchar,
    0x1d4a2 as ::core::ffi::c_int as pg_wchar,
    0x1d4a5 as ::core::ffi::c_int as pg_wchar,
    0x1d4a6 as ::core::ffi::c_int as pg_wchar,
    0x1d4a9 as ::core::ffi::c_int as pg_wchar,
    0x1d4ac as ::core::ffi::c_int as pg_wchar,
    0x1d4ae as ::core::ffi::c_int as pg_wchar,
    0x1d4b9 as ::core::ffi::c_int as pg_wchar,
    0x1d4bb as ::core::ffi::c_int as pg_wchar,
    0x1d4bb as ::core::ffi::c_int as pg_wchar,
    0x1d4bd as ::core::ffi::c_int as pg_wchar,
    0x1d4c0 as ::core::ffi::c_int as pg_wchar,
    0x1d4c2 as ::core::ffi::c_int as pg_wchar,
    0x1d4c3 as ::core::ffi::c_int as pg_wchar,
    0x1d4c5 as ::core::ffi::c_int as pg_wchar,
    0x1d505 as ::core::ffi::c_int as pg_wchar,
    0x1d507 as ::core::ffi::c_int as pg_wchar,
    0x1d50a as ::core::ffi::c_int as pg_wchar,
    0x1d50d as ::core::ffi::c_int as pg_wchar,
    0x1d514 as ::core::ffi::c_int as pg_wchar,
    0x1d516 as ::core::ffi::c_int as pg_wchar,
    0x1d51c as ::core::ffi::c_int as pg_wchar,
    0x1d51e as ::core::ffi::c_int as pg_wchar,
    0x1d539 as ::core::ffi::c_int as pg_wchar,
    0x1d53b as ::core::ffi::c_int as pg_wchar,
    0x1d53e as ::core::ffi::c_int as pg_wchar,
    0x1d540 as ::core::ffi::c_int as pg_wchar,
    0x1d544 as ::core::ffi::c_int as pg_wchar,
    0x1d546 as ::core::ffi::c_int as pg_wchar,
    0x1d546 as ::core::ffi::c_int as pg_wchar,
    0x1d54a as ::core::ffi::c_int as pg_wchar,
    0x1d550 as ::core::ffi::c_int as pg_wchar,
    0x1d552 as ::core::ffi::c_int as pg_wchar,
    0x1d6a3 as ::core::ffi::c_int as pg_wchar,
    0x1d6a8 as ::core::ffi::c_int as pg_wchar,
    0x1d7c9 as ::core::ffi::c_int as pg_wchar,
    0x20000 as ::core::ffi::c_int as pg_wchar,
    0x2a6d6 as ::core::ffi::c_int as pg_wchar,
    0x2f800 as ::core::ffi::c_int as pg_wchar,
    0x2fa1d as ::core::ffi::c_int as pg_wchar,
    0xf0000 as ::core::ffi::c_int as pg_wchar,
    0xffffd as ::core::ffi::c_int as pg_wchar,
    0x100000 as ::core::ffi::c_int as pg_wchar,
    0x10fffd as ::core::ffi::c_int as pg_wchar,
];

unsafe extern "C" fn codepoint_range_cmp(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut key = a as *const pg_wchar;
    let mut range = b as *const pg_wchar;
    if *key < *range {
        return -(1 as ::core::ffi::c_int);
    }
    if *key > *range.offset(1 as ::core::ffi::c_int as isize) {
        return 1 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn is_code_in_table(
    mut code: pg_wchar,
    mut map: *const pg_wchar,
    mut mapsize: ::core::ffi::c_int,
) -> bool {
    if code < *map
        || code > *map.offset((mapsize - 1 as ::core::ffi::c_int) as isize)
    {
        return false;
    }
    if !bsearch(
        &raw mut code as *const ::core::ffi::c_void,
        map as *const ::core::ffi::c_void,
        (mapsize / 2 as ::core::ffi::c_int) as size_t,
        (::core::mem::size_of::<pg_wchar>() as size_t).wrapping_mul(2 as size_t),
        Some(
            codepoint_range_cmp
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    )
    .is_null()
    {
        true
    } else {
        false
    }
}

unsafe extern "C" fn pg_utf8_string_len(
    mut source: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p = source as *const ::core::ffi::c_uchar;
    let mut l: ::core::ffi::c_int = 0;
    let mut num_chars = 0 as ::core::ffi::c_int;
    let mut len = strlen(source);
    while len != 0 {
        l = pg_utf_mblen(p);
        if len < l as size_t || !pg_utf8_islegal(p, l) {
            return -(1 as ::core::ffi::c_int);
        }
        p = p.offset(l as isize);
        len = len.wrapping_sub(l as size_t);
        num_chars += 1;
    }
    num_chars
}
#[no_mangle]

pub unsafe extern "C" fn pg_saslprep(
    mut input: *const ::core::ffi::c_char,
    mut output: *mut *mut ::core::ffi::c_char,
) -> pg_saslprep_rc {
    let mut current_block: u64;
    let mut input_chars = ::core::ptr::null_mut::<pg_wchar>();
    let mut output_chars = ::core::ptr::null_mut::<pg_wchar>();
    let mut input_size: ::core::ffi::c_int = 0;
    let mut result = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut result_size: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut contains_RandALCat: bool = false;
    let mut p = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut wp = ::core::ptr::null_mut::<pg_wchar>();
    *output = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if pg_is_ascii(input) {
        *output = strdup(input);
        if !(*output).is_null() {
            return SASLPREP_SUCCESS;
        }
    } else {
        input_size = pg_utf8_string_len(input);
        if input_size < 0 as ::core::ffi::c_int {
            return SASLPREP_INVALID_UTF8;
        }
        if (input_size as size_t)
            < MaxAllocSize.wrapping_div(::core::mem::size_of::<pg_wchar>() as size_t)
        {
            input_chars = malloc(
                ((input_size + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<pg_wchar>() as size_t),
            ) as *mut pg_wchar;
            if !input_chars.is_null() {
                p = input as *mut ::core::ffi::c_uchar;
                i = 0 as ::core::ffi::c_int;
                while i < input_size {
                    *input_chars.offset(i as isize) = utf8_to_unicode(p);
                    p = p.offset(pg_utf_mblen(p) as isize);
                    i += 1;
                }
                *input_chars.offset(i as isize) = '\0' as i32 as pg_wchar;
                count = 0 as ::core::ffi::c_int;
                i = 0 as ::core::ffi::c_int;
                while i < input_size {
                    let mut code = *input_chars.offset(i as isize);
                    if is_code_in_table(
                        code,
                        &raw const non_ascii_space_ranges as *const pg_wchar,
                        ::core::mem::size_of::<[pg_wchar; 12]>()
                            .wrapping_div(::core::mem::size_of::<pg_wchar>())
                            as ::core::ffi::c_int,
                    ) {
                        let fresh0 = count;
                        count += 1;
                        *input_chars.offset(fresh0 as isize) = 0x20 as pg_wchar;
                    } else if !is_code_in_table(
                        code,
                        &raw const commonly_mapped_to_nothing_ranges as *const pg_wchar,
                        ::core::mem::size_of::<[pg_wchar; 16]>()
                            .wrapping_div(::core::mem::size_of::<pg_wchar>())
                            as ::core::ffi::c_int,
                    ) {
                        let fresh1 = count;
                        count += 1;
                        *input_chars.offset(fresh1 as isize) = code;
                    }
                    i += 1;
                }
                *input_chars.offset(count as isize) = '\0' as i32 as pg_wchar;
                input_size = count;
                if input_size == 0 as ::core::ffi::c_int {
                    current_block = 11863746162674211658;
                } else {
                    output_chars = unicode_normalize(UNICODE_NFKC, input_chars);
                    if output_chars.is_null() {
                        current_block = 4504237218701476758;
                    } else {
                        i = 0 as ::core::ffi::c_int;
                        loop {
                            if i >= input_size {
                                current_block = 9520865839495247062;
                                break;
                            }
                            let mut code_0 = *input_chars.offset(i as isize);
                            if is_code_in_table(
                                code_0,
                                &raw const prohibited_output_ranges as *const pg_wchar,
                                ::core::mem::size_of::<[pg_wchar; 72]>()
                                    .wrapping_div(::core::mem::size_of::<pg_wchar>())
                                    as ::core::ffi::c_int,
                            ) {
                                current_block = 11863746162674211658;
                                break;
                            }
                            if is_code_in_table(
                                code_0,
                                &raw const unassigned_codepoint_ranges as *const pg_wchar,
                                ::core::mem::size_of::<[pg_wchar; 792]>()
                                    .wrapping_div(::core::mem::size_of::<pg_wchar>())
                                    as ::core::ffi::c_int,
                            ) {
                                current_block = 11863746162674211658;
                                break;
                            }
                            i += 1;
                        }
                        match current_block {
                            11863746162674211658 => {}
                            _ => {
                                contains_RandALCat = false;
                                i = 0 as ::core::ffi::c_int;
                                while i < input_size {
                                    let mut code_1 = *input_chars.offset(i as isize);
                                    if is_code_in_table(
                                        code_1,
                                        &raw const RandALCat_codepoint_ranges as *const pg_wchar,
                                        ::core::mem::size_of::<[pg_wchar; 68]>()
                                            .wrapping_div(::core::mem::size_of::<pg_wchar>())
                                            as ::core::ffi::c_int,
                                    ) {
                                        contains_RandALCat = true;
                                        break;
                                    } else {
                                        i += 1;
                                    }
                                }
                                if contains_RandALCat {
                                    let mut first =
                                        *input_chars;
                                    let mut last = *input_chars
                                        .offset((input_size - 1 as ::core::ffi::c_int) as isize);
                                    i = 0 as ::core::ffi::c_int;
                                    loop {
                                        if i >= input_size {
                                            current_block = 12381812505308290051;
                                            break;
                                        }
                                        let mut code_2 = *input_chars.offset(i as isize);
                                        if is_code_in_table(
                                            code_2,
                                            &raw const LCat_codepoint_ranges as *const pg_wchar,
                                            ::core::mem::size_of::<[pg_wchar; 720]>()
                                                .wrapping_div(::core::mem::size_of::<pg_wchar>())
                                                as ::core::ffi::c_int,
                                        ) {
                                            current_block = 11863746162674211658;
                                            break;
                                        }
                                        i += 1;
                                    }
                                    match current_block {
                                        11863746162674211658 => {}
                                        _ => {
                                            if !is_code_in_table(
                                                first,
                                                &raw const RandALCat_codepoint_ranges
                                                    as *const pg_wchar,
                                                ::core::mem::size_of::<[pg_wchar; 68]>()
                                                    .wrapping_div(
                                                        ::core::mem::size_of::<pg_wchar>(),
                                                    )
                                                    as ::core::ffi::c_int,
                                            ) || !is_code_in_table(
                                                last,
                                                &raw const RandALCat_codepoint_ranges
                                                    as *const pg_wchar,
                                                ::core::mem::size_of::<[pg_wchar; 68]>()
                                                    .wrapping_div(
                                                        ::core::mem::size_of::<pg_wchar>(),
                                                    )
                                                    as ::core::ffi::c_int,
                                            ) {
                                                current_block = 11863746162674211658;
                                            } else {
                                                current_block = 313581471991351815;
                                            }
                                        }
                                    }
                                } else {
                                    current_block = 313581471991351815;
                                }
                                match current_block {
                                    11863746162674211658 => {}
                                    _ => {
                                        result_size = 0 as ::core::ffi::c_int;
                                        wp = output_chars;
                                        while *wp != 0 {
                                            let mut buf: [::core::ffi::c_uchar; 4] = [0; 4];
                                            unicode_to_utf8(
                                                *wp,
                                                &raw mut buf as *mut ::core::ffi::c_uchar,
                                            );
                                            result_size += pg_utf_mblen(
                                                &raw mut buf as *mut ::core::ffi::c_uchar,
                                            );
                                            wp = wp.offset(1);
                                        }
                                        result = malloc(
                                            (result_size + 1 as ::core::ffi::c_int) as size_t,
                                        )
                                            as *mut ::core::ffi::c_char;
                                        if result.is_null() {
                                            current_block = 4504237218701476758;
                                        } else {
                                            p = result as *mut ::core::ffi::c_uchar;
                                            wp = output_chars;
                                            while *wp != 0 {
                                                unicode_to_utf8(*wp, p);
                                                p = p.offset(pg_utf_mblen(p) as isize);
                                                wp = wp.offset(1);
                                            }
                                            *p = '\0' as i32 as ::core::ffi::c_uchar;
                                            free(input_chars as *mut ::core::ffi::c_void);
                                            free(output_chars as *mut ::core::ffi::c_void);
                                            *output = result;
                                            return SASLPREP_SUCCESS;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                match current_block {
                    4504237218701476758 => {}
                    _ => {
                        if !input_chars.is_null() {
                            free(input_chars as *mut ::core::ffi::c_void);
                        }
                        if !output_chars.is_null() {
                            free(output_chars as *mut ::core::ffi::c_void);
                        }
                        return SASLPREP_PROHIBITED;
                    }
                }
            }
        }
    }
    if !input_chars.is_null() {
        free(input_chars as *mut ::core::ffi::c_void);
    }
    if !output_chars.is_null() {
        free(output_chars as *mut ::core::ffi::c_void);
    }
    SASLPREP_OOM
}
