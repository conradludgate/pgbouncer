#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = *mut ::core::ffi::c_char;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:19"]
pub mod _types_h {
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
    #[c2rust::src_loc = "95:1"]
    pub type __darwin_va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:19"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_va_list.h:19"]
pub mod _va_list_h {
    #[c2rust::src_loc = "44:1"]
    pub type va_list = __darwin_va_list;
    use super::_types_h::__darwin_va_list;
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
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:19"]
pub mod _stdio_h {
    use super::_size_t_h::size_t;

    extern "C" {
        #[c2rust::src_loc = "438:1"]
        pub fn vsnprintf(
            __str: *mut ::core::ffi::c_char,
            __size: size_t,
            __format: *const ::core::ffi::c_char,
            _: ::core::ffi::VaList,
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
        #[c2rust::src_loc = "57:1"]
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t)
            -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:19"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:22"]
pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {
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
        #[c2rust::src_loc = "96:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:19"]
pub mod sys__types_h {
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_abort.h:19"]
pub mod _abort_h {
    extern "C" {
        #[c2rust::src_loc = "33:1"]
        pub fn abort() -> !;
    }
}
use self::_abort_h::abort;
use self::_malloc_h::{free, malloc, realloc};
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
use self::_stdio_h::vsnprintf;
use self::_string_h::{memcpy, memset, strlen};
pub use self::_types_h::{__darwin_size_t, __darwin_va_list};
pub use self::_va_list_h::va_list;
pub use self::cxalloc_h::{CxMem, CxOps};
pub use self::internal::__builtin_va_list;
pub use self::sys__types_h::__DARWIN_NULL;
#[no_mangle]
#[c2rust::src_loc = "28:1"]
pub unsafe extern "C" fn cx_alloc(
    mut cx: *const CxMem,
    mut len: size_t,
) -> *mut ::core::ffi::c_void {
    if len == 0 {
        return NULL;
    }
    if cx.is_null() {
        cx = &raw const cx_libc_allocator;
    }
    return (*(*cx).ops).c_alloc.expect("non-null function pointer")((*cx).ctx, len);
}
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn cx_realloc(
    mut cx: *const CxMem,
    mut ptr: *mut ::core::ffi::c_void,
    mut len: size_t,
) -> *mut ::core::ffi::c_void {
    if cx.is_null() {
        cx = &raw const cx_libc_allocator;
    }
    if ptr.is_null() {
        return cx_alloc(cx, len);
    }
    if len == 0 {
        cx_free(cx, ptr);
        return NULL;
    }
    return (*(*cx).ops).c_realloc.expect("non-null function pointer")((*cx).ctx, ptr, len);
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn cx_free(mut cx: *const CxMem, mut ptr: *mut ::core::ffi::c_void) {
    if cx.is_null() {
        cx = &raw const cx_libc_allocator;
    }
    if !ptr.is_null() {
        (*(*cx).ops).c_free.expect("non-null function pointer")((*cx).ctx, ptr);
    }
}
#[no_mangle]
#[c2rust::src_loc = "58:1"]
pub unsafe extern "C" fn cx_destroy(mut cx: *const CxMem) {
    if cx.is_null() {
        return;
    }
    if (*(*cx).ops).c_destroy.is_none() {
        abort();
    }
    (*(*cx).ops).c_destroy.expect("non-null function pointer")((*cx).ctx);
}
#[no_mangle]
#[c2rust::src_loc = "67:1"]
pub unsafe extern "C" fn cx_alloc0(
    mut cx: *const CxMem,
    mut len: size_t,
) -> *mut ::core::ffi::c_void {
    let mut p = cx_alloc(cx, len);
    if !p.is_null() {
        memset(p, 0 as ::core::ffi::c_int, len);
    }
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "75:1"]
pub unsafe extern "C" fn cx_memdup(
    mut cx: *const CxMem,
    mut src: *const ::core::ffi::c_void,
    mut len: size_t,
) -> *mut ::core::ffi::c_void {
    let mut p = cx_alloc(cx, len);
    if !p.is_null() {
        memcpy(p, src, len);
    }
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "83:1"]
pub unsafe extern "C" fn cx_strdup(
    mut cx: *const CxMem,
    mut s: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    return cx_memdup(
        cx,
        s as *const ::core::ffi::c_void,
        strlen(s).wrapping_add(1 as size_t),
    );
}
#[no_mangle]
#[c2rust::src_loc = "88:1"]
pub unsafe extern "C" fn cx_sprintf(
    mut cx: *const CxMem,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ...
) -> *mut ::core::ffi::c_char {
    let mut res = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    res = cx_vsprintf(cx, fmt, ap.as_va_list());
    return res;
}
#[no_mangle]
#[c2rust::src_loc = "98:1"]
pub unsafe extern "C" fn cx_vsprintf(
    mut cx: *const CxMem,
    mut fmt: *const ::core::ffi::c_char,
    mut ap: ::core::ffi::VaList,
) -> *mut ::core::ffi::c_char {
    let mut res = ::core::ptr::null_mut::<::core::ffi::c_char>();
    cx_vasprintf(cx, &raw mut res, fmt, ap.as_va_list());
    return res;
}
#[no_mangle]
#[c2rust::src_loc = "105:1"]
pub unsafe extern "C" fn cx_asprintf(
    mut cx: *const CxMem,
    mut dst_p: *mut *mut ::core::ffi::c_char,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ...
) -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int = 0;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    res = cx_vasprintf(cx, dst_p, fmt, ap.as_va_list());
    return res;
}
#[no_mangle]
#[c2rust::src_loc = "115:1"]
pub unsafe extern "C" fn cx_vasprintf(
    mut cx: *const CxMem,
    mut dst_p: *mut *mut ::core::ffi::c_char,
    mut fmt: *const ::core::ffi::c_char,
    mut ap: ::core::ffi::VaList,
) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_char; 128] = [0; 128];
    let mut dst = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut res: ::core::ffi::c_int = 0;
    let mut res2: ::core::ffi::c_int = 0;
    *dst_p = ::core::ptr::null_mut::<::core::ffi::c_char>();
    res = vsnprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
        fmt,
        ap.as_va_list(),
    );
    if res < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    dst = cx_alloc(cx, (res + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
    if dst.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (res as size_t) < ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as usize {
        memcpy(
            dst as *mut ::core::ffi::c_void,
            &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            (res + 1 as ::core::ffi::c_int) as size_t,
        );
    } else {
        res2 = vsnprintf(
            dst,
            (res + 1 as ::core::ffi::c_int) as size_t,
            fmt,
            ap.as_va_list(),
        );
        if res2 != res {
            cx_free(cx, dst as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
    }
    *dst_p = dst;
    return res;
}
#[c2rust::src_loc = "146:1"]
unsafe extern "C" fn libc_alloc(
    mut _ctx: *mut ::core::ffi::c_void,
    mut len: size_t,
) -> *mut ::core::ffi::c_void {
    return malloc(len);
}
#[c2rust::src_loc = "151:1"]
unsafe extern "C" fn libc_realloc(
    mut _ctx: *mut ::core::ffi::c_void,
    mut ptr: *mut ::core::ffi::c_void,
    mut len: size_t,
) -> *mut ::core::ffi::c_void {
    return realloc(ptr, len);
}
#[c2rust::src_loc = "156:1"]
unsafe extern "C" fn libc_free(
    mut _ctx: *mut ::core::ffi::c_void,
    mut ptr: *mut ::core::ffi::c_void,
) {
    free(ptr);
}
#[c2rust::src_loc = "161:1"]
static mut libc_alloc_ops: CxOps = unsafe {
    CxOps {
        c_alloc: Some(
            libc_alloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> *mut ::core::ffi::c_void,
        ),
        c_realloc: Some(
            libc_realloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> *mut ::core::ffi::c_void,
        ),
        c_free: Some(
            libc_free
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
        ),
        c_destroy: None,
    }
};
#[no_mangle]
#[c2rust::src_loc = "167:1"]
pub static mut cx_libc_allocator: CxMem = unsafe {
    CxMem {
        ops: &raw const libc_alloc_ops,
        ctx: NULL,
    }
};
