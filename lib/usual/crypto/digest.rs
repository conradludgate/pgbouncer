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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:19"]
pub mod _uint8_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint8_t = u8;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint64_t.h:19"]
pub mod _uint64_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint64_t = u64;
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
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/crypto/digest.h:19"]
pub mod digest_h {
    #[c2rust::src_loc = "30:1"]
    pub type DigestInitFunc = unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ();
    #[c2rust::src_loc = "31:1"]
    pub type DigestUpdateFunc = unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_void,
        ::core::ffi::c_uint,
    ) -> ();
    #[c2rust::src_loc = "32:1"]
    pub type DigestFinalFunc = unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> ();
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:1"]
    pub struct DigestInfo {
        pub init: Option<DigestInitFunc>,
        pub update: Option<DigestUpdateFunc>,
        pub final_0: Option<DigestFinalFunc>,
        pub state_len: ::core::ffi::c_short,
        pub result_len: ::core::ffi::c_short,
        pub block_len: ::core::ffi::c_short,
    }
    use super::_uint8_t_h::uint8_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:19"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:21"]
pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "83:1"]
        pub fn memset(
            __b: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __len: size_t,
        ) -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:19"]
pub mod sys__types_h {
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
use self::_string_h::memset;
pub use self::_types_h::__darwin_size_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::cxalloc_h::{cx_alloc, cx_free, CxMem, CxOps};
pub use self::digest_h::{DigestFinalFunc, DigestInfo, DigestInitFunc, DigestUpdateFunc};
pub use self::sys__types_h::__DARWIN_NULL;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "23:1"]
pub struct DigestContext {
    pub impl_0: *const DigestInfo,
    pub cx: *const CxMem,
    pub state: [uint64_t; 1],
}
#[no_mangle]
#[c2rust::src_loc = "29:1"]
pub unsafe extern "C" fn digest_new(
    mut impl_0: *const DigestInfo,
    mut cx: *const CxMem,
) -> *mut DigestContext {
    let mut ctx = ::core::ptr::null_mut::<DigestContext>();
    let mut alloc: ::core::ffi::c_uint = 0;
    alloc = (16 as ::core::ffi::c_ulong).wrapping_add((*impl_0).state_len as ::core::ffi::c_ulong)
        as ::core::ffi::c_uint;
    ctx = cx_alloc(cx, alloc as size_t) as *mut DigestContext;
    if ctx.is_null() {
        return ::core::ptr::null_mut::<DigestContext>();
    }
    (*ctx).impl_0 = impl_0;
    (*ctx).cx = cx;
    (*impl_0).init.expect("non-null function pointer")(
        &raw mut (*ctx).state as *mut uint64_t as *mut ::core::ffi::c_void,
    );
    return ctx;
}
#[no_mangle]
#[c2rust::src_loc = "45:1"]
pub unsafe extern "C" fn digest_update(
    mut ctx: *mut DigestContext,
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    (*(*ctx).impl_0).update.expect("non-null function pointer")(
        &raw mut (*ctx).state as *mut uint64_t as *mut ::core::ffi::c_void,
        data,
        len as ::core::ffi::c_uint,
    );
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn digest_final(mut ctx: *mut DigestContext, mut res: *mut uint8_t) {
    (*(*ctx).impl_0).final_0.expect("non-null function pointer")(
        &raw mut (*ctx).state as *mut uint64_t as *mut ::core::ffi::c_void,
        res,
    );
}
#[no_mangle]
#[c2rust::src_loc = "55:1"]
pub unsafe extern "C" fn digest_reset(mut ctx: *mut DigestContext) {
    (*(*ctx).impl_0).init.expect("non-null function pointer")(
        &raw mut (*ctx).state as *mut uint64_t as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]
#[c2rust::src_loc = "60:1"]
pub unsafe extern "C" fn digest_free(mut ctx: *mut DigestContext) {
    let mut cx = (*ctx).cx;
    let mut alloc = (16 as ::core::ffi::c_ulong)
        .wrapping_add((*(*ctx).impl_0).state_len as ::core::ffi::c_ulong)
        as ::core::ffi::c_uint;
    memset(
        ctx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        alloc as size_t,
    );
    cx_free(cx, ctx as *mut ::core::ffi::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "69:1"]
pub unsafe extern "C" fn digest_block_len(mut ctx: *mut DigestContext) -> ::core::ffi::c_uint {
    return (*(*ctx).impl_0).block_len as ::core::ffi::c_uint;
}
#[no_mangle]
#[c2rust::src_loc = "74:1"]
pub unsafe extern "C" fn digest_result_len(mut ctx: *mut DigestContext) -> ::core::ffi::c_uint {
    return (*(*ctx).impl_0).result_len as ::core::ffi::c_uint;
}
