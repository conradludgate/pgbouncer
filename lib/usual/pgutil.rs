pub mod _types_h {

    pub type __uint32_t = u32;

    pub type __darwin_ct_rune_t = ::core::ffi::c_int;

    pub type __darwin_size_t = usize;

    pub type __darwin_wchar_t = ::libc::wchar_t;

    pub type __darwin_rune_t = __darwin_wchar_t;
}

pub mod _size_t_h {

    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}

pub mod cxalloc_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

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

    pub struct CxMem {
        pub ops: *const CxOps,
        pub ctx: *mut ::core::ffi::c_void,
    }
    use super::_size_t_h::size_t;
    extern "C" {

        pub fn cx_alloc(cx: *const CxMem, len: size_t) -> *mut ::core::ffi::c_void;

        pub fn cx_free(cx: *const CxMem, ptr: *mut ::core::ffi::c_void);
    }
}

pub mod string_h {
    use super::cxalloc_h::CxMem;
    extern "C" {

        pub type StrList;

        pub fn strlist_new(ca: *const CxMem) -> *mut StrList;

        pub fn strlist_free(slist: *mut StrList);

        pub fn strlist_append_ref(slist: *mut StrList, str: *mut ::core::ffi::c_char) -> bool;
    }
}

pub mod pgutil_kwlookup_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct pgkw_t {
        pub pgkw_str16: [::core::ffi::c_char; 6],
        pub pgkw_str22: [::core::ffi::c_char; 5],
        pub pgkw_str24: [::core::ffi::c_char; 3],
        pub pgkw_str27: [::core::ffi::c_char; 6],
        pub pgkw_str28: [::core::ffi::c_char; 4],
        pub pgkw_str29: [::core::ffi::c_char; 3],
        pub pgkw_str30: [::core::ffi::c_char; 5],
        pub pgkw_str31: [::core::ffi::c_char; 6],
        pub pgkw_str32: [::core::ffi::c_char; 5],
        pub pgkw_str33: [::core::ffi::c_char; 5],
        pub pgkw_str34: [::core::ffi::c_char; 3],
        pub pgkw_str36: [::core::ffi::c_char; 5],
        pub pgkw_str37: [::core::ffi::c_char; 5],
        pub pgkw_str39: [::core::ffi::c_char; 6],
        pub pgkw_str41: [::core::ffi::c_char; 7],
        pub pgkw_str42: [::core::ffi::c_char; 4],
        pub pgkw_str43: [::core::ffi::c_char; 5],
        pub pgkw_str44: [::core::ffi::c_char; 6],
        pub pgkw_str45: [::core::ffi::c_char; 5],
        pub pgkw_str46: [::core::ffi::c_char; 6],
        pub pgkw_str47: [::core::ffi::c_char; 6],
        pub pgkw_str48: [::core::ffi::c_char; 3],
        pub pgkw_str51: [::core::ffi::c_char; 6],
        pub pgkw_str52: [::core::ffi::c_char; 5],
        pub pgkw_str53: [::core::ffi::c_char; 5],
        pub pgkw_str54: [::core::ffi::c_char; 6],
        pub pgkw_str55: [::core::ffi::c_char; 8],
        pub pgkw_str56: [::core::ffi::c_char; 6],
        pub pgkw_str57: [::core::ffi::c_char; 10],
        pub pgkw_str58: [::core::ffi::c_char; 8],
        pub pgkw_str60: [::core::ffi::c_char; 6],
        pub pgkw_str62: [::core::ffi::c_char; 7],
        pub pgkw_str63: [::core::ffi::c_char; 8],
        pub pgkw_str64: [::core::ffi::c_char; 11],
        pub pgkw_str66: [::core::ffi::c_char; 3],
        pub pgkw_str67: [::core::ffi::c_char; 4],
        pub pgkw_str68: [::core::ffi::c_char; 8],
        pub pgkw_str69: [::core::ffi::c_char; 7],
        pub pgkw_str70: [::core::ffi::c_char; 10],
        pub pgkw_str71: [::core::ffi::c_char; 11],
        pub pgkw_str72: [::core::ffi::c_char; 7],
        pub pgkw_str73: [::core::ffi::c_char; 6],
        pub pgkw_str75: [::core::ffi::c_char; 5],
        pub pgkw_str76: [::core::ffi::c_char; 5],
        pub pgkw_str77: [::core::ffi::c_char; 7],
        pub pgkw_str78: [::core::ffi::c_char; 3],
        pub pgkw_str79: [::core::ffi::c_char; 9],
        pub pgkw_str80: [::core::ffi::c_char; 9],
        pub pgkw_str83: [::core::ffi::c_char; 5],
        pub pgkw_str84: [::core::ffi::c_char; 8],
        pub pgkw_str85: [::core::ffi::c_char; 4],
        pub pgkw_str86: [::core::ffi::c_char; 9],
        pub pgkw_str87: [::core::ffi::c_char; 4],
        pub pgkw_str88: [::core::ffi::c_char; 4],
        pub pgkw_str89: [::core::ffi::c_char; 4],
        pub pgkw_str90: [::core::ffi::c_char; 3],
        pub pgkw_str91: [::core::ffi::c_char; 4],
        pub pgkw_str92: [::core::ffi::c_char; 4],
        pub pgkw_str93: [::core::ffi::c_char; 6],
        pub pgkw_str95: [::core::ffi::c_char; 8],
        pub pgkw_str96: [::core::ffi::c_char; 6],
        pub pgkw_str97: [::core::ffi::c_char; 8],
        pub pgkw_str98: [::core::ffi::c_char; 5],
        pub pgkw_str99: [::core::ffi::c_char; 5],
        pub pgkw_str100: [::core::ffi::c_char; 8],
        pub pgkw_str101: [::core::ffi::c_char; 8],
        pub pgkw_str102: [::core::ffi::c_char; 9],
        pub pgkw_str103: [::core::ffi::c_char; 5],
        pub pgkw_str104: [::core::ffi::c_char; 7],
        pub pgkw_str106: [::core::ffi::c_char; 8],
        pub pgkw_str107: [::core::ffi::c_char; 7],
        pub pgkw_str108: [::core::ffi::c_char; 4],
        pub pgkw_str109: [::core::ffi::c_char; 5],
        pub pgkw_str111: [::core::ffi::c_char; 9],
        pub pgkw_str112: [::core::ffi::c_char; 11],
        pub pgkw_str115: [::core::ffi::c_char; 4],
        pub pgkw_str116: [::core::ffi::c_char; 4],
        pub pgkw_str117: [::core::ffi::c_char; 6],
        pub pgkw_str119: [::core::ffi::c_char; 9],
        pub pgkw_str120: [::core::ffi::c_char; 7],
        pub pgkw_str122: [::core::ffi::c_char; 15],
        pub pgkw_str123: [::core::ffi::c_char; 10],
        pub pgkw_str125: [::core::ffi::c_char; 6],
        pub pgkw_str126: [::core::ffi::c_char; 9],
        pub pgkw_str127: [::core::ffi::c_char; 7],
        pub pgkw_str128: [::core::ffi::c_char; 4],
        pub pgkw_str129: [::core::ffi::c_char; 13],
        pub pgkw_str130: [::core::ffi::c_char; 6],
        pub pgkw_str132: [::core::ffi::c_char; 8],
        pub pgkw_str133: [::core::ffi::c_char; 10],
        pub pgkw_str134: [::core::ffi::c_char; 11],
        pub pgkw_str135: [::core::ffi::c_char; 8],
        pub pgkw_str136: [::core::ffi::c_char; 9],
        pub pgkw_str137: [::core::ffi::c_char; 13],
        pub pgkw_str138: [::core::ffi::c_char; 10],
        pub pgkw_str139: [::core::ffi::c_char; 13],
        pub pgkw_str140: [::core::ffi::c_char; 6],
        pub pgkw_str142: [::core::ffi::c_char; 5],
        pub pgkw_str144: [::core::ffi::c_char; 6],
        pub pgkw_str145: [::core::ffi::c_char; 8],
        pub pgkw_str146: [::core::ffi::c_char; 11],
        pub pgkw_str147: [::core::ffi::c_char; 13],
        pub pgkw_str149: [::core::ffi::c_char; 6],
        pub pgkw_str152: [::core::ffi::c_char; 5],
        pub pgkw_str153: [::core::ffi::c_char; 13],
        pub pgkw_str154: [::core::ffi::c_char; 10],
        pub pgkw_str155: [::core::ffi::c_char; 6],
        pub pgkw_str157: [::core::ffi::c_char; 4],
        pub pgkw_str160: [::core::ffi::c_char; 8],
        pub pgkw_str162: [::core::ffi::c_char; 8],
        pub pgkw_str163: [::core::ffi::c_char; 7],
        pub pgkw_str164: [::core::ffi::c_char; 8],
        pub pgkw_str165: [::core::ffi::c_char; 5],
        pub pgkw_str166: [::core::ffi::c_char; 6],
        pub pgkw_str168: [::core::ffi::c_char; 5],
        pub pgkw_str170: [::core::ffi::c_char; 10],
        pub pgkw_str175: [::core::ffi::c_char; 14],
        pub pgkw_str177: [::core::ffi::c_char; 8],
        pub pgkw_str181: [::core::ffi::c_char; 10],
        pub pgkw_str183: [::core::ffi::c_char; 15],
        pub pgkw_str184: [::core::ffi::c_char; 5],
        pub pgkw_str185: [::core::ffi::c_char; 8],
        pub pgkw_str186: [::core::ffi::c_char; 10],
        pub pgkw_str188: [::core::ffi::c_char; 9],
        pub pgkw_str192: [::core::ffi::c_char; 8],
        pub pgkw_str198: [::core::ffi::c_char; 13],
        pub pgkw_str200: [::core::ffi::c_char; 13],
        pub pgkw_str202: [::core::ffi::c_char; 18],
        pub pgkw_str204: [::core::ffi::c_char; 5],
        pub pgkw_str205: [::core::ffi::c_char; 6],
        pub pgkw_str206: [::core::ffi::c_char; 10],
        pub pgkw_str207: [::core::ffi::c_char; 4],
        pub pgkw_str208: [::core::ffi::c_char; 9],
        pub pgkw_str213: [::core::ffi::c_char; 7],
        pub pgkw_str218: [::core::ffi::c_char; 16],
        pub pgkw_str219: [::core::ffi::c_char; 8],
        pub pgkw_str220: [::core::ffi::c_char; 5],
        pub pgkw_str224: [::core::ffi::c_char; 10],
        pub pgkw_str227: [::core::ffi::c_char; 7],
        pub pgkw_str236: [::core::ffi::c_char; 6],
        pub pgkw_str237: [::core::ffi::c_char; 10],
        pub pgkw_str265: [::core::ffi::c_char; 8],
        pub pgkw_str266: [::core::ffi::c_char; 5],
        pub pgkw_str272: [::core::ffi::c_char; 9],
        pub pgkw_str273: [::core::ffi::c_char; 14],
        pub pgkw_str279: [::core::ffi::c_char; 7],
        pub pgkw_str289: [::core::ffi::c_char; 7],
        pub pgkw_str311: [::core::ffi::c_char; 7],
    }

    pub const MAX_HASH_VALUE: C2RustUnnamed = 311;

    pub const MIN_WORD_LENGTH: C2RustUnnamed = 2;

    pub const MAX_WORD_LENGTH: C2RustUnnamed = 17;

    pub type C2RustUnnamed = ::core::ffi::c_uint;

    pub const MIN_HASH_VALUE: C2RustUnnamed = 16;

    pub const TOTAL_KEYWORDS: C2RustUnnamed = 148;

    pub unsafe extern "C" fn pg_keyword_lookup_hash(
        mut str: *const ::core::ffi::c_char,
        mut len: size_t,
    ) -> ::core::ffi::c_uint {
        pub static mut asso_values: [::core::ffi::c_ushort; 256] = [
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            38 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            125 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            31 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            64 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            10 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            96 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            125 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            26 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            5 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            13 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            63 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            10 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            12 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            70 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            5 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            3 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            71 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            65 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            77 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            3 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
            312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        ];
        let mut hval = len as ::core::ffi::c_uint;
        let mut current_block_3: u64;
        match hval {
            6..=8 => {
                current_block_3 = 3907860993030533457;
            }
            2..=5 => {
                current_block_3 = 17908814879015121377;
            }
            1 => {
                current_block_3 = 8833495243305235038;
            }
            _ => {
                hval = hval.wrapping_add(
                    asso_values[*str.offset(8 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_uint,
                );
                current_block_3 = 3907860993030533457;
            }
        }
        if current_block_3 == 3907860993030533457 {
            hval = hval.wrapping_add(
                asso_values
                    [*str.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_uint,
            );
            current_block_3 = 17908814879015121377;
        }
        if current_block_3 == 17908814879015121377 {
            hval = hval.wrapping_add(
                asso_values
                    [*str.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_uint,
            );
        }
        hval = hval.wrapping_add(
            asso_values
                [*str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_uint,
        );
        hval.wrapping_add(
            asso_values[*str.add(len.wrapping_sub(1 as size_t)) as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_uint,
        )
    }
    #[no_mangle]

    pub unsafe extern "C" fn pg_keyword_lookup_real(
        mut str: *const ::core::ffi::c_char,
        mut len: size_t,
    ) -> *const ::core::ffi::c_char {
        pub static mut pgkw_contents: pgkw_t = unsafe {
            pgkw_t {
                pgkw_str16: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"treat\0",
                ),
                pgkw_str22: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"true\0"),
                pgkw_str24: ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"or\0"),
                pgkw_str27: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"order\0",
                ),
                pgkw_str28: ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"not\0"),
                pgkw_str29: ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"to\0"),
                pgkw_str30: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"left\0"),
                pgkw_str31: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"least\0",
                ),
                pgkw_str32: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"real\0"),
                pgkw_str33: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"join\0"),
                pgkw_str34: ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"on\0"),
                pgkw_str36: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"none\0"),
                pgkw_str37: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"else\0"),
                pgkw_str39: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"right\0",
                ),
                pgkw_str41: ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(
                    *b"select\0",
                ),
                pgkw_str42: ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"int\0"),
                pgkw_str43: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"time\0"),
                pgkw_str44: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"inout\0",
                ),
                pgkw_str45: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"some\0"),
                pgkw_str46: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"inner\0",
                ),
                pgkw_str47: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"limit\0",
                ),
                pgkw_str48: ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"in\0"),
                pgkw_str51: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"nchar\0",
                ),
                pgkw_str52: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"into\0"),
                pgkw_str53: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"like\0"),
                pgkw_str54: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"ilike\0",
                ),
                pgkw_str55: ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(
                    *b"notnull\0",
                ),
                pgkw_str56: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"table\0",
                ),
                pgkw_str57: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"localtime\0",
                ),
                pgkw_str58: ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(
                    *b"integer\0",
                ),
                pgkw_str60: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"cross\0",
                ),
                pgkw_str62: ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(
                    *b"create\0",
                ),
                pgkw_str63: ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(
                    *b"collate\0",
                ),
                pgkw_str64: ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(
                    *b"references\0",
                ),
                pgkw_str66: ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"is\0"),
                pgkw_str67: ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"all\0"),
                pgkw_str68: ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(
                    *b"analyze\0",
                ),
                pgkw_str69: ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(
                    *b"column\0",
                ),
                pgkw_str70: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"intersect\0",
                ),
                pgkw_str71: ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(
                    *b"constraint\0",
                ),
                pgkw_str72: ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(
                    *b"except\0",
                ),
                pgkw_str73: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"grant\0",
                ),
                pgkw_str75: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"trim\0"),
                pgkw_str76: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"cast\0"),
                pgkw_str77: ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(
                    *b"isnull\0",
                ),
                pgkw_str78: ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"as\0"),
                pgkw_str79: ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(
                    *b"national\0",
                ),
                pgkw_str80: ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(
                    *b"coalesce\0",
                ),
                pgkw_str83: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"case\0"),
                pgkw_str84: ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(
                    *b"analyse\0",
                ),
                pgkw_str85: ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"row\0"),
                pgkw_str86: ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(
                    *b"greatest\0",
                ),
                pgkw_str87: ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"end\0"),
                pgkw_str88: ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"new\0"),
                pgkw_str89: ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"out\0"),
                pgkw_str90: ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"do\0"),
                pgkw_str91: ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"asc\0"),
                pgkw_str92: ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"old\0"),
                pgkw_str93: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"outer\0",
                ),
                pgkw_str95: ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(
                    *b"similar\0",
                ),
                pgkw_str96: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"union\0",
                ),
                pgkw_str97: ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(
                    *b"default\0",
                ),
                pgkw_str98: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"null\0"),
                pgkw_str99: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"user\0"),
                pgkw_str100: ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(
                    *b"leading\0",
                ),
                pgkw_str101: ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(
                    *b"extract\0",
                ),
                pgkw_str102: ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(
                    *b"trailing\0",
                ),
                pgkw_str103: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(
                    *b"only\0",
                ),
                pgkw_str104: ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(
                    *b"exists\0",
                ),
                pgkw_str106: ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(
                    *b"natural\0",
                ),
                pgkw_str107: ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(
                    *b"unique\0",
                ),
                pgkw_str108: ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"dec\0"),
                pgkw_str109: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(
                    *b"desc\0",
                ),
                pgkw_str111: ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(
                    *b"distinct\0",
                ),
                pgkw_str112: ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(
                    *b"deferrable\0",
                ),
                pgkw_str115: ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"and\0"),
                pgkw_str116: ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"for\0"),
                pgkw_str117: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"float\0",
                ),
                pgkw_str119: ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(
                    *b"smallint\0",
                ),
                pgkw_str120: ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(
                    *b"offset\0",
                ),
                pgkw_str122: ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(
                    *b"localtimestamp\0",
                ),
                pgkw_str123: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"precision\0",
                ),
                pgkw_str125: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"array\0",
                ),
                pgkw_str126: ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(
                    *b"position\0",
                ),
                pgkw_str127: ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(
                    *b"freeze\0",
                ),
                pgkw_str128: ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"any\0"),
                pgkw_str129: ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(
                    *b"session_user\0",
                ),
                pgkw_str130: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"setof\0",
                ),
                pgkw_str132: ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(
                    *b"decimal\0",
                ),
                pgkw_str133: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"xmlforest\0",
                ),
                pgkw_str134: ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(
                    *b"asymmetric\0",
                ),
                pgkw_str135: ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(
                    *b"xmlroot\0",
                ),
                pgkw_str136: ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(
                    *b"xmlparse\0",
                ),
                pgkw_str137: ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(
                    *b"current_time\0",
                ),
                pgkw_str138: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"xmlconcat\0",
                ),
                pgkw_str139: ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(
                    *b"current_role\0",
                ),
                pgkw_str140: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"group\0",
                ),
                pgkw_str142: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(
                    *b"then\0",
                ),
                pgkw_str144: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"xmlpi\0",
                ),
                pgkw_str145: ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(
                    *b"numeric\0",
                ),
                pgkw_str146: ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(
                    *b"xmlelement\0",
                ),
                pgkw_str147: ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(
                    *b"concurrently\0",
                ),
                pgkw_str149: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"false\0",
                ),
                pgkw_str152: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(
                    *b"over\0",
                ),
                pgkw_str153: ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(
                    *b"xmlserialize\0",
                ),
                pgkw_str154: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"returning\0",
                ),
                pgkw_str155: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"using\0",
                ),
                pgkw_str157: ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"bit\0"),
                pgkw_str160: ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(
                    *b"placing\0",
                ),
                pgkw_str162: ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(
                    *b"between\0",
                ),
                pgkw_str163: ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(
                    *b"bigint\0",
                ),
                pgkw_str164: ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(
                    *b"primary\0",
                ),
                pgkw_str165: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(
                    *b"char\0",
                ),
                pgkw_str166: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"check\0",
                ),
                pgkw_str168: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(
                    *b"from\0",
                ),
                pgkw_str170: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"symmetric\0",
                ),
                pgkw_str175: ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(
                    *b"authorization\0",
                ),
                pgkw_str177: ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(
                    *b"verbose\0",
                ),
                pgkw_str181: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"timestamp\0",
                ),
                pgkw_str183: ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(
                    *b"current_schema\0",
                ),
                pgkw_str184: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(
                    *b"full\0",
                ),
                pgkw_str185: ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(
                    *b"foreign\0",
                ),
                pgkw_str186: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"xmlexists\0",
                ),
                pgkw_str188: ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(
                    *b"interval\0",
                ),
                pgkw_str192: ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(
                    *b"boolean\0",
                ),
                pgkw_str198: ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(
                    *b"current_date\0",
                ),
                pgkw_str200: ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(
                    *b"current_user\0",
                ),
                pgkw_str202: ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(
                    *b"current_timestamp\0",
                ),
                pgkw_str204: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(
                    *b"when\0",
                ),
                pgkw_str205: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"where\0",
                ),
                pgkw_str206: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"character\0",
                ),
                pgkw_str207: ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"off\0"),
                pgkw_str208: ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(
                    *b"overlaps\0",
                ),
                pgkw_str213: ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(
                    *b"values\0",
                ),
                pgkw_str218: ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(
                    *b"current_catalog\0",
                ),
                pgkw_str219: ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(
                    *b"varchar\0",
                ),
                pgkw_str220: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(
                    *b"with\0",
                ),
                pgkw_str224: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"substring\0",
                ),
                pgkw_str227: ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(
                    *b"window\0",
                ),
                pgkw_str236: ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(
                    *b"fetch\0",
                ),
                pgkw_str237: ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                    *b"initially\0",
                ),
                pgkw_str265: ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(
                    *b"overlay\0",
                ),
                pgkw_str266: ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(
                    *b"both\0",
                ),
                pgkw_str272: ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(
                    *b"variadic\0",
                ),
                pgkw_str273: ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(
                    *b"xmlattributes\0",
                ),
                pgkw_str279: ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(
                    *b"nullif\0",
                ),
                pgkw_str289: ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(
                    *b"having\0",
                ),
                pgkw_str311: ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(
                    *b"binary\0",
                ),
            }
        };
        if len <= MAX_WORD_LENGTH as ::core::ffi::c_int as size_t
            && len >= MIN_WORD_LENGTH as ::core::ffi::c_int as size_t
        {
            let mut key = pg_keyword_lookup_hash(str, len);
            if key <= MAX_HASH_VALUE as ::core::ffi::c_int as ::core::ffi::c_uint {
                let mut o = super::wordlist[key as usize];
                if o >= 0 as ::core::ffi::c_int {
                    let mut s =
                        (&raw const pgkw_contents as *const ::core::ffi::c_char).offset(o as isize);
                    if *str as ::core::ffi::c_int == *s as ::core::ffi::c_int
                        && strcmp(
                            str.offset(1 as ::core::ffi::c_int as isize),
                            s.offset(1 as ::core::ffi::c_int as isize),
                        ) == 0
                    {
                        return s;
                    }
                }
            }
        }
        ::core::ptr::null::<::core::ffi::c_char>()
    }
    use super::_size_t_h::size_t;
    use super::_string_h::strcmp;
}

pub mod runetype_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

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

    pub struct _RuneCharClass {
        pub __name: [::core::ffi::c_char; 14],
        pub __mask: __uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _RuneRange {
        pub __nranges: ::core::ffi::c_int,
        pub __ranges: *mut _RuneEntry,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _RuneEntry {
        pub __min: __darwin_rune_t,
        pub __max: __darwin_rune_t,
        pub __map: __darwin_rune_t,
        pub __types: *mut __uint32_t,
    }
    use super::_types_h::{__darwin_rune_t, __darwin_size_t, __uint32_t};
    extern "C" {

        pub static mut _DefaultRuneLocale: _RuneLocale;
    }
}

pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {

        pub fn memcpy(
            __dst: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;

        pub fn strchr(
            __s: *const ::core::ffi::c_char,
            __c: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_char;

        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;

        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}

pub mod _strings_h {
    use super::_size_t_h::size_t;
    extern "C" {

        pub fn strncasecmp(
            _: *const ::core::ffi::c_char,
            _: *const ::core::ffi::c_char,
            _: size_t,
        ) -> ::core::ffi::c_int;
    }
}

pub mod _null_h {

    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}

pub mod ctype_h {
    #[inline]

    pub unsafe extern "C" fn safe_isspace(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        isspace(c as ::core::ffi::c_uchar as ::core::ffi::c_int)
    }
    use super::_ctype_h::isspace;
}

pub mod _ctype_h {

    pub const _CTYPE_S: ::core::ffi::c_long = 0x4000 as ::core::ffi::c_long;
    #[inline]

    pub unsafe extern "C" fn isascii(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        (_c & !(0x7f as ::core::ffi::c_int) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
    }
    #[inline]

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

    pub unsafe extern "C" fn isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        __istype(_c as __darwin_ct_rune_t, _CTYPE_S as ::core::ffi::c_ulong)
    }
    use super::_types_h::__darwin_ct_rune_t;
    use super::runetype_h::_DefaultRuneLocale;
    extern "C" {

        pub fn __maskrune(_: __darwin_ct_rune_t, _: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    }
}

pub mod sys__types_h {

    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}

pub mod stdbool_h {

    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::_ctype_h::{__istype, __maskrune, isascii, isspace, _CTYPE_S};
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
use self::_string_h::{memcpy, strchr, strlen};
use self::_strings_h::strncasecmp;
pub use self::_types_h::{
    __darwin_ct_rune_t, __darwin_rune_t, __darwin_size_t, __darwin_wchar_t, __uint32_t,
};
pub use self::ctype_h::safe_isspace;
pub use self::cxalloc_h::{cx_alloc, cx_free, CxMem, CxOps};
pub use self::pgutil_kwlookup_h::{
    pg_keyword_lookup_hash, pg_keyword_lookup_real, pgkw_t, C2RustUnnamed, MAX_HASH_VALUE,
    MAX_WORD_LENGTH, MIN_HASH_VALUE, MIN_WORD_LENGTH, TOTAL_KEYWORDS,
};
pub use self::runetype_h::{
    _DefaultRuneLocale, _RuneCharClass, _RuneEntry, _RuneLocale, _RuneRange,
};
pub use self::stdbool_h::{false_0, true_0};
use self::string_h::{strlist_append_ref, strlist_free, strlist_new, StrList};
pub use self::sys__types_h::__DARWIN_NULL;
static mut wordlist: [::core::ffi::c_int; 312] = [0; 312];
#[no_mangle]

pub unsafe extern "C" fn pg_quote_literal(
    mut _dst: *mut ::core::ffi::c_char,
    mut _src: *const ::core::ffi::c_char,
    mut dstlen: ::core::ffi::c_int,
) -> bool {
    let mut dst = _dst;
    let mut end = _dst
        .offset(dstlen as isize)
        .offset(-(2 as ::core::ffi::c_int as isize));
    let mut src = _src;
    let mut stdquote = true_0 != 0;
    if dstlen < 3 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if _src.is_null() {
        if dstlen < 5 as ::core::ffi::c_int {
            return false_0 != 0;
        }
        memcpy(
            _dst as *mut ::core::ffi::c_void,
            b"NULL\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            5 as size_t,
        );
        return true_0 != 0;
    }
    '_retry: loop {
        let fresh0 = dst;
        dst = dst.offset(1);
        *fresh0 = '\'' as i32 as ::core::ffi::c_char;
        loop {
            if !(*src as ::core::ffi::c_int != 0 && dst < end) {
                break '_retry;
            }
            if *src as ::core::ffi::c_int == '\'' as i32 {
                let fresh1 = dst;
                dst = dst.offset(1);
                *fresh1 = '\'' as i32 as ::core::ffi::c_char;
            } else if *src as ::core::ffi::c_int == '\\' as i32 {
                if stdquote {
                    break;
                }
                let fresh2 = dst;
                dst = dst.offset(1);
                *fresh2 = '\\' as i32 as ::core::ffi::c_char;
            }
            let fresh3 = src;
            src = src.offset(1);
            let fresh4 = dst;
            dst = dst.offset(1);
            *fresh4 = *fresh3;
        }
        dst = _dst;
        src = _src;
        let fresh6 = dst;
        dst = dst.offset(1);
        *fresh6 = 'E' as i32 as ::core::ffi::c_char;
        stdquote = false_0 != 0;
    }
    if *src as ::core::ffi::c_int != 0 || dst > end {
        return false_0 != 0;
    }
    let fresh5 = dst;
    dst = dst.offset(1);
    *fresh5 = '\'' as i32 as ::core::ffi::c_char;
    *dst = 0 as ::core::ffi::c_char;
    true_0 != 0
}
#[inline]

unsafe extern "C" fn id_start(mut c: ::core::ffi::c_uchar) -> bool {
    c as ::core::ffi::c_int >= 'a' as i32 && c as ::core::ffi::c_int <= 'z' as i32
        || c as ::core::ffi::c_int == '_' as i32
}
#[inline]

unsafe extern "C" fn id_body(mut c: ::core::ffi::c_uchar) -> bool {
    id_start(c) as ::core::ffi::c_int != 0
        || c as ::core::ffi::c_int >= '0' as i32 && c as ::core::ffi::c_int <= '9' as i32
}
#[no_mangle]

pub unsafe extern "C" fn pg_quote_ident(
    mut _dst: *mut ::core::ffi::c_char,
    mut _src: *const ::core::ffi::c_char,
    mut dstlen: ::core::ffi::c_int,
) -> bool {
    let mut current_block: u64;
    let mut dst = _dst;
    let mut end = _dst
        .offset(dstlen as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    let mut src = _src;
    if dstlen < 1 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if !id_start(*src as ::core::ffi::c_uchar) {
        current_block = 6923518307576458708;
    } else {
        current_block = 10680521327981672866;
    }
    loop {
        match current_block {
            6923518307576458708 => {
                dst = _dst;
                break;
            }
            _ => {
                if *src as ::core::ffi::c_int != 0 && dst < end {
                    if !id_body(*src as ::core::ffi::c_uchar) {
                        current_block = 6923518307576458708;
                        continue;
                    }
                    let fresh7 = src;
                    src = src.offset(1);
                    let fresh8 = dst;
                    dst = dst.offset(1);
                    *fresh8 = *fresh7;
                    current_block = 10680521327981672866;
                } else {
                    if *src != 0 {
                        return false_0 != 0;
                    }
                    *dst = 0 as ::core::ffi::c_char;
                    if !pg_is_reserved_word(_dst) {
                        return true_0 != 0;
                    }
                    current_block = 6923518307576458708;
                }
            }
        }
    }
    src = _src;
    end = _dst
        .offset(dstlen as isize)
        .offset(-(2 as ::core::ffi::c_int as isize));
    if dstlen < 3 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    let fresh9 = dst;
    dst = dst.offset(1);
    *fresh9 = '"' as i32 as ::core::ffi::c_char;
    while *src as ::core::ffi::c_int != 0 && dst < end {
        if *src as ::core::ffi::c_int == '"' as i32 {
            let fresh10 = dst;
            dst = dst.offset(1);
            *fresh10 = *src;
        }
        let fresh11 = src;
        src = src.offset(1);
        let fresh12 = dst;
        dst = dst.offset(1);
        *fresh12 = *fresh11;
    }
    if *src != 0 {
        return false_0 != 0;
    }
    let fresh13 = dst;
    dst = dst.offset(1);
    *fresh13 = '"' as i32 as ::core::ffi::c_char;
    *dst = 0 as ::core::ffi::c_char;
    true_0 != 0
}
#[no_mangle]

pub unsafe extern "C" fn pg_quote_fqident(
    mut _dst: *mut ::core::ffi::c_char,
    mut _src: *const ::core::ffi::c_char,
    mut dstlen: ::core::ffi::c_int,
) -> bool {
    let mut dot: *const ::core::ffi::c_char = strchr(_src, '.' as i32);
    let mut scmbuf: [::core::ffi::c_char; 128] = [0; 128];
    let mut scm = ::core::ptr::null::<::core::ffi::c_char>();
    let mut scmlen: ::core::ffi::c_int = 0;
    if !dot.is_null() {
        scmlen = dot.offset_from(_src) as ::core::ffi::c_long as ::core::ffi::c_int;
        if scmlen >= ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as ::core::ffi::c_int {
            return false_0 != 0;
        }
        memcpy(
            &raw mut scmbuf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            _src as *const ::core::ffi::c_void,
            scmlen as size_t,
        );
        scmbuf[scmlen as usize] = 0 as ::core::ffi::c_char;
        scm = &raw mut scmbuf as *mut ::core::ffi::c_char;
        _src = dot.offset(1 as ::core::ffi::c_int as isize);
    } else {
        scm = b"public\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if !pg_quote_ident(_dst, scm, dstlen) {
        return false_0 != 0;
    }
    scmlen = strlen(_dst) as ::core::ffi::c_int;
    *_dst.offset(scmlen as isize) = '.' as i32 as ::core::ffi::c_char;
    _dst = _dst.offset((scmlen + 1 as ::core::ffi::c_int) as isize);
    dstlen -= scmlen + 1 as ::core::ffi::c_int;
    if !pg_quote_ident(_dst, _src, dstlen) {
        return false_0 != 0;
    }
    true_0 != 0
}

unsafe extern "C" fn parse_value(
    mut arr: *mut StrList,
    mut val: *const ::core::ffi::c_char,
    mut vend: *const ::core::ffi::c_char,
    mut cx: *const CxMem,
) -> bool {
    let mut len: ::core::ffi::c_int = 0;
    let mut s = ::core::ptr::null::<::core::ffi::c_char>();
    let mut str = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut c: ::core::ffi::c_uint = 0;
    while val < vend && safe_isspace(*val as ::core::ffi::c_int) != 0 {
        val = val.offset(1);
    }
    while vend > val
        && safe_isspace(*vend.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
            != 0
    {
        vend = vend.offset(-1);
    }
    if val == vend {
        return false_0 != 0;
    }
    s = val;
    len = vend.offset_from(val) as ::core::ffi::c_long as ::core::ffi::c_int;
    if len == 4 as ::core::ffi::c_int
        && strncasecmp(
            val,
            b"null\0" as *const u8 as *const ::core::ffi::c_char,
            len as size_t,
        ) == 0
    {
        return strlist_append_ref(arr, ::core::ptr::null_mut::<::core::ffi::c_char>());
    }
    str = cx_alloc(cx, (len + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
    p = str;
    if str.is_null() {
        return false_0 != 0;
    }
    while s < vend {
        let fresh17 = s;
        s = s.offset(1);
        c = *fresh17 as ::core::ffi::c_uint;
        if c == '"' as i32 as ::core::ffi::c_uint {
            loop {
                let fresh18 = s;
                s = s.offset(1);
                c = *fresh18 as ::core::ffi::c_uint;
                if c == '"' as i32 as ::core::ffi::c_uint {
                    break;
                }
                if c == '\\' as i32 as ::core::ffi::c_uint {
                    let fresh19 = s;
                    s = s.offset(1);
                    let fresh20 = p;
                    p = p.offset(1);
                    *fresh20 = *fresh19;
                } else {
                    let fresh21 = p;
                    p = p.offset(1);
                    *fresh21 = c as ::core::ffi::c_char;
                }
            }
        } else if c == '\\' as i32 as ::core::ffi::c_uint {
            let fresh22 = s;
            s = s.offset(1);
            let fresh23 = p;
            p = p.offset(1);
            *fresh23 = *fresh22;
        } else {
            let fresh24 = p;
            p = p.offset(1);
            *fresh24 = c as ::core::ffi::c_char;
        }
    }
    let fresh25 = p;
    p = p.offset(1);
    *fresh25 = 0 as ::core::ffi::c_char;
    if !strlist_append_ref(arr, str) {
        cx_free(cx, str as *mut ::core::ffi::c_void);
        return false_0 != 0;
    }
    true_0 != 0
}
#[no_mangle]

pub unsafe extern "C" fn pg_parse_array(
    mut pgarr: *const ::core::ffi::c_char,
    mut cx: *const CxMem,
) -> *mut StrList {
    let mut current_block: u64;
    let mut s = pgarr;
    let mut lst = ::core::ptr::null_mut::<StrList>();
    let mut val = ::core::ptr::null::<::core::ffi::c_char>();
    let mut c: ::core::ffi::c_uint = 0;
    if *s as ::core::ffi::c_int == '[' as i32 {
        s = strchr(s, ']' as i32);
        if s.is_null()
            || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '=' as i32
        {
            return ::core::ptr::null_mut::<StrList>();
        }
        s = s.offset(2 as ::core::ffi::c_int as isize);
    }
    let fresh14 = s;
    s = s.offset(1);
    if *fresh14 as ::core::ffi::c_int != '{' as i32 {
        return ::core::ptr::null_mut::<StrList>();
    }
    lst = strlist_new(cx);
    if lst.is_null() {
        return ::core::ptr::null_mut::<StrList>();
    }
    's_48: loop {
        if *s == 0 {
            current_block = 9853141518545631134;
            break;
        }
        if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '}' as i32 {
            if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            {
                current_block = 398846161464620410;
                break;
            }
            if !val.is_null() && !parse_value(lst, val, s, cx) {
                current_block = 398846161464620410;
                break;
            }
            return lst;
        } else {
            if val.is_null() {
                val = s;
            }
            if *s as ::core::ffi::c_int == ',' as i32 {
                if !parse_value(lst, val, s, cx) {
                    current_block = 398846161464620410;
                    break;
                }
                s = s.offset(1);
                val = s;
            } else {
                let fresh15 = s;
                s = s.offset(1);
                c = *fresh15 as ::core::ffi::c_uint;
                if c == '"' as i32 as ::core::ffi::c_uint {
                    loop {
                        let fresh16 = s;
                        s = s.offset(1);
                        c = *fresh16 as ::core::ffi::c_uint;
                        if c == '"' as i32 as ::core::ffi::c_uint {
                            break;
                        }
                        if c == '\\' as i32 as ::core::ffi::c_uint {
                            if *s == 0 {
                                current_block = 398846161464620410;
                                break 's_48;
                            }
                            s = s.offset(1);
                        } else if *s == 0 {
                            current_block = 398846161464620410;
                            break 's_48;
                        }
                    }
                } else {
                    if c != '\\' as i32 as ::core::ffi::c_uint {
                        continue;
                    }
                    if *s == 0 {
                        current_block = 398846161464620410;
                        break;
                    }
                    s = s.offset(1);
                }
            }
        }
    }
    if current_block == 9853141518545631134
        && (*s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int == '}' as i32)
    {
        return lst;
    }
    strlist_free(lst);
    ::core::ptr::null_mut::<StrList>()
}
#[no_mangle]

pub unsafe extern "C" fn pg_is_reserved_word(mut str: *const ::core::ffi::c_char) -> bool {
    let mut kw = pg_keyword_lookup_real(str, strlen(str));
    !kw.is_null()
}
unsafe extern "C" fn run_static_initializers() {
    wordlist = [
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str16
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str22
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str24
            as *mut [::core::ffi::c_char; 3] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str27
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str28
            as *mut [::core::ffi::c_char; 4] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str29
            as *mut [::core::ffi::c_char; 3] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str30
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str31
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str32
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str33
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str34
            as *mut [::core::ffi::c_char; 3] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str36
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str37
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str39
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str41
            as *mut [::core::ffi::c_char; 7] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str42
            as *mut [::core::ffi::c_char; 4] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str43
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str44
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str45
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str46
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str47
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str48
            as *mut [::core::ffi::c_char; 3] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str51
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str52
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str53
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str54
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str55
            as *mut [::core::ffi::c_char; 8] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str56
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str57
            as *mut [::core::ffi::c_char; 10] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str58
            as *mut [::core::ffi::c_char; 8] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str60
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str62
            as *mut [::core::ffi::c_char; 7] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str63
            as *mut [::core::ffi::c_char; 8] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str64
            as *mut [::core::ffi::c_char; 11] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str66
            as *mut [::core::ffi::c_char; 3] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str67
            as *mut [::core::ffi::c_char; 4] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str68
            as *mut [::core::ffi::c_char; 8] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str69
            as *mut [::core::ffi::c_char; 7] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str70
            as *mut [::core::ffi::c_char; 10] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str71
            as *mut [::core::ffi::c_char; 11] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str72
            as *mut [::core::ffi::c_char; 7] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str73
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str75
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str76
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str77
            as *mut [::core::ffi::c_char; 7] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str78
            as *mut [::core::ffi::c_char; 3] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str79
            as *mut [::core::ffi::c_char; 9] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str80
            as *mut [::core::ffi::c_char; 9] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str83
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str84
            as *mut [::core::ffi::c_char; 8] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str85
            as *mut [::core::ffi::c_char; 4] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str86
            as *mut [::core::ffi::c_char; 9] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str87
            as *mut [::core::ffi::c_char; 4] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str88
            as *mut [::core::ffi::c_char; 4] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str89
            as *mut [::core::ffi::c_char; 4] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str90
            as *mut [::core::ffi::c_char; 3] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str91
            as *mut [::core::ffi::c_char; 4] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str92
            as *mut [::core::ffi::c_char; 4] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str93
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str95
            as *mut [::core::ffi::c_char; 8] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str96
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str97
            as *mut [::core::ffi::c_char; 8] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str98
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str99
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str100
            as *mut [::core::ffi::c_char; 8] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str101
            as *mut [::core::ffi::c_char; 8] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str102
            as *mut [::core::ffi::c_char; 9] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str103
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str104
            as *mut [::core::ffi::c_char; 7] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str106
            as *mut [::core::ffi::c_char; 8] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str107
            as *mut [::core::ffi::c_char; 7] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str108
            as *mut [::core::ffi::c_char; 4] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str109
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str111
            as *mut [::core::ffi::c_char; 9] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str112
            as *mut [::core::ffi::c_char; 11] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str115
            as *mut [::core::ffi::c_char; 4] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str116
            as *mut [::core::ffi::c_char; 4] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str117
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str119
            as *mut [::core::ffi::c_char; 9] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str120
            as *mut [::core::ffi::c_char; 7] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str122
            as *mut [::core::ffi::c_char; 15] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str123
            as *mut [::core::ffi::c_char; 10] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str125
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str126
            as *mut [::core::ffi::c_char; 9] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str127
            as *mut [::core::ffi::c_char; 7] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str128
            as *mut [::core::ffi::c_char; 4] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str129
            as *mut [::core::ffi::c_char; 13] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str130
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str132
            as *mut [::core::ffi::c_char; 8] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str133
            as *mut [::core::ffi::c_char; 10] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str134
            as *mut [::core::ffi::c_char; 11] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str135
            as *mut [::core::ffi::c_char; 8] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str136
            as *mut [::core::ffi::c_char; 9] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str137
            as *mut [::core::ffi::c_char; 13] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str138
            as *mut [::core::ffi::c_char; 10] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str139
            as *mut [::core::ffi::c_char; 13] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str140
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str142
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str144
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str145
            as *mut [::core::ffi::c_char; 8] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str146
            as *mut [::core::ffi::c_char; 11] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str147
            as *mut [::core::ffi::c_char; 13] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str149
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str152
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str153
            as *mut [::core::ffi::c_char; 13] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str154
            as *mut [::core::ffi::c_char; 10] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str155
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str157
            as *mut [::core::ffi::c_char; 4] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str160
            as *mut [::core::ffi::c_char; 8] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str162
            as *mut [::core::ffi::c_char; 8] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str163
            as *mut [::core::ffi::c_char; 7] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str164
            as *mut [::core::ffi::c_char; 8] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str165
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str166
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str168
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str170
            as *mut [::core::ffi::c_char; 10] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str175
            as *mut [::core::ffi::c_char; 14] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str177
            as *mut [::core::ffi::c_char; 8] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str181
            as *mut [::core::ffi::c_char; 10] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str183
            as *mut [::core::ffi::c_char; 15] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str184
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str185
            as *mut [::core::ffi::c_char; 8] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str186
            as *mut [::core::ffi::c_char; 10] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str188
            as *mut [::core::ffi::c_char; 9] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str192
            as *mut [::core::ffi::c_char; 8] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str198
            as *mut [::core::ffi::c_char; 13] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str200
            as *mut [::core::ffi::c_char; 13] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str202
            as *mut [::core::ffi::c_char; 18] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str204
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str205
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str206
            as *mut [::core::ffi::c_char; 10] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str207
            as *mut [::core::ffi::c_char; 4] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str208
            as *mut [::core::ffi::c_char; 9] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str213
            as *mut [::core::ffi::c_char; 7] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str218
            as *mut [::core::ffi::c_char; 16] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str219
            as *mut [::core::ffi::c_char; 8] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str220
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str224
            as *mut [::core::ffi::c_char; 10] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str227
            as *mut [::core::ffi::c_char; 7] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str236
            as *mut [::core::ffi::c_char; 6] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str237
            as *mut [::core::ffi::c_char; 10] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str265
            as *mut [::core::ffi::c_char; 8] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str266
            as *mut [::core::ffi::c_char; 5] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str272
            as *mut [::core::ffi::c_char; 9] as size_t as ::core::ffi::c_int,
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str273
            as *mut [::core::ffi::c_char; 14] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str279
            as *mut [::core::ffi::c_char; 7] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str289
            as *mut [::core::ffi::c_char; 7] as size_t as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        &raw const (*(::core::ptr::null::<pgkw_t>() as *mut pgkw_t)).pgkw_str311
            as *mut [::core::ffi::c_char; 7] as size_t as ::core::ffi::c_int,
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
