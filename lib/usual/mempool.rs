#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:19"]
pub mod _types_h {
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:19"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:19"]
pub mod _malloc_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "55:1"]
        pub fn calloc(__count: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "56:1"]
        pub fn free(_: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:19"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_param.h:19"]
pub mod _param_h {
    #[c2rust::src_loc = "17:9"]
    pub const __DARWIN_ALIGNBYTES: usize =
        ::core::mem::size_of::<__darwin_size_t>().wrapping_sub(1_usize);
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:19"]
pub mod sys__types_h {
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}
use self::_malloc_h::{calloc, free};
pub use self::_null_h::NULL;
pub use self::_param_h::__DARWIN_ALIGNBYTES;
pub use self::_size_t_h::size_t;
pub use self::_types_h::__darwin_size_t;
pub use self::sys__types_h::__DARWIN_NULL;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "31:1"]
pub struct MemPool {
    pub prev: *mut MemPool,
    pub size: ::core::ffi::c_uint,
    pub used: ::core::ffi::c_uint,
}
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn mempool_alloc(
    mut pool: *mut *mut MemPool,
    mut size: ::core::ffi::c_uint,
) -> *mut ::core::ffi::c_void {
    let mut cur = *pool;
    let mut ptr = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut nsize: ::core::ffi::c_uint = 0;
    size = ((size as __darwin_size_t).wrapping_add(__DARWIN_ALIGNBYTES) & !__DARWIN_ALIGNBYTES)
        as ::core::ffi::c_uint;
    if !cur.is_null() && (*cur).used.wrapping_add(size) <= (*cur).size {
        ptr = (cur.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char)
            .offset((*cur).used as isize) as *mut ::core::ffi::c_void;
        (*cur).used = (*cur).used.wrapping_add(size);
        ptr
    } else {
        nsize = if !cur.is_null() {
            (2 as ::core::ffi::c_uint).wrapping_mul((*cur).size)
        } else {
            512 as ::core::ffi::c_uint
        };
        while nsize < size {
            nsize = nsize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
        cur = calloc(
            1 as size_t,
            (::core::mem::size_of::<MemPool>() as size_t).wrapping_add(nsize as size_t),
        ) as *mut MemPool;
        if cur.is_null() {
            return NULL;
        }
        (*cur).used = size;
        (*cur).size = nsize;
        (*cur).prev = *pool;
        *pool = cur;
        cur.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void
    }
}
#[no_mangle]
#[c2rust::src_loc = "63:1"]
pub unsafe extern "C" fn mempool_destroy(mut pool: *mut *mut MemPool) {
    let mut cur = ::core::ptr::null_mut::<MemPool>();
    let mut tmp = ::core::ptr::null_mut::<MemPool>();
    if pool.is_null() {
        return;
    }
    cur = *pool;
    *pool = ::core::ptr::null_mut::<MemPool>();
    while !cur.is_null() {
        tmp = (*cur).prev;
        free(cur as *mut ::core::ffi::c_void);
        cur = tmp;
    }
}
