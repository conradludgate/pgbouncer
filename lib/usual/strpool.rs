pub mod _types_h {

    pub type __darwin_size_t = usize;

    pub type __darwin_ssize_t = isize;
}

pub mod _size_t_h {

    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}

pub mod _ssize_t_h {

    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
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

pub mod cbtree_h {

    pub type cbtree_walker_func =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> bool>;

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

        pub type CBTree;

        pub fn cbtree_create(
            obj_key_cb: cbtree_getkey_func,
            obj_free_cb: cbtree_walker_func,
            cb_ctx: *mut ::core::ffi::c_void,
            cx: *const CxMem,
        ) -> *mut CBTree;

        pub fn cbtree_destroy(tree: *mut CBTree);

        pub fn cbtree_insert(tree: *mut CBTree, obj: *mut ::core::ffi::c_void) -> bool;

        pub fn cbtree_delete(
            tree: *mut CBTree,
            key: *const ::core::ffi::c_void,
            klen: size_t,
        ) -> bool;

        pub fn cbtree_lookup(
            tree: *mut CBTree,
            key: *const ::core::ffi::c_void,
            klen: size_t,
        ) -> *mut ::core::ffi::c_void;

        pub fn cbtree_walk(
            tree: *mut CBTree,
            cb_func: cbtree_walker_func,
            cb_arg: *mut ::core::ffi::c_void,
        ) -> bool;
    }
}

pub mod strpool_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct PStr {
        pub pool: *mut StrPool,
        pub len: size_t,
        pub refcnt: ::core::ffi::c_int,
        pub str_0: [::core::ffi::c_char; 0],
    }
    use super::StrPool;
    use super::_size_t_h::size_t;
}

pub mod _null_h {

    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}

pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {

        pub fn memcpy(
            __dst: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;

        pub fn memset(
            __b: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __len: size_t,
        ) -> *mut ::core::ffi::c_void;

        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}

pub mod sys__types_h {

    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}

pub mod stdbool_h {

    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
}
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
pub use self::_ssize_t_h::ssize_t;
use self::_string_h::{memcpy, memset, strlen};
pub use self::_types_h::{__darwin_size_t, __darwin_ssize_t};
pub use self::cbtree_h::{
    cbtree_create, cbtree_delete, cbtree_destroy, cbtree_getkey_func, cbtree_insert, cbtree_lookup,
    cbtree_walk, cbtree_walker_func, CBTree,
};
pub use self::cxalloc_h::{cx_alloc, cx_free, CxMem, CxOps};
pub use self::stdbool_h::true_0;
pub use self::strpool_h::PStr;
pub use self::sys__types_h::__DARWIN_NULL;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct StrPool {
    pub ca: *const CxMem,
    pub tree: *mut CBTree,
    pub count: ::core::ffi::c_int,
}

unsafe extern "C" fn get_key(
    mut _ctx: *mut ::core::ffi::c_void,
    mut obj: *mut ::core::ffi::c_void,
    mut dst_p: *mut *const ::core::ffi::c_void,
) -> size_t {
    let mut s = obj as *mut PStr;
    *dst_p = &raw mut (*s).str_0 as *mut ::core::ffi::c_char as *const ::core::ffi::c_void;
    (*s).len
}

unsafe extern "C" fn free_str(
    mut _arg: *mut ::core::ffi::c_void,
    mut obj: *mut ::core::ffi::c_void,
) -> bool {
    let mut p = obj as *mut PStr;
    let mut sp = (*p).pool;
    memset(
        p as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (20 as size_t).wrapping_add(1 as size_t),
    );
    cx_free((*sp).ca, obj);
    true_0 != 0
}
#[no_mangle]

pub unsafe extern "C" fn strpool_create(mut ca: *const CxMem) -> *mut StrPool {
    let mut sp = ::core::ptr::null_mut::<StrPool>();
    sp = cx_alloc(ca, ::core::mem::size_of::<StrPool>() as size_t) as *mut StrPool;
    if sp.is_null() {
        return ::core::ptr::null_mut::<StrPool>();
    }
    (*sp).count = 0 as ::core::ffi::c_int;
    (*sp).ca = ca;
    (*sp).tree = cbtree_create(
        Some(
            get_key
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                    *mut *const ::core::ffi::c_void,
                ) -> size_t,
        ),
        None,
        NULL,
        ca,
    );
    if (*sp).tree.is_null() {
        cx_free(ca, sp as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<StrPool>();
    }
    sp
}
#[no_mangle]

pub unsafe extern "C" fn strpool_free(mut sp: *mut StrPool) {
    if !sp.is_null() {
        cbtree_walk(
            (*sp).tree,
            Some(
                free_str
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
            ),
            sp as *mut ::core::ffi::c_void,
        );
        cbtree_destroy((*sp).tree);
        cx_free((*sp).ca, sp as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]

pub unsafe extern "C" fn strpool_total(mut sp: *mut StrPool) -> ::core::ffi::c_int {
    (*sp).count
}
#[no_mangle]

pub unsafe extern "C" fn strpool_get(
    mut sp: *mut StrPool,
    mut str: *const ::core::ffi::c_char,
    mut len: ssize_t,
) -> *mut PStr {
    let mut cstr = ::core::ptr::null_mut::<PStr>();
    let mut ok: bool = false;
    if len < 0 as ssize_t {
        len = strlen(str) as ssize_t;
    }
    cstr = cbtree_lookup((*sp).tree, str as *const ::core::ffi::c_void, len as size_t) as *mut PStr;
    if !cstr.is_null() {
        (*cstr).refcnt += 1;
        return cstr;
    }
    cstr = cx_alloc(
        (*sp).ca,
        (::core::mem::size_of::<PStr>() as size_t)
            .wrapping_add(len as size_t)
            .wrapping_add(1 as size_t),
    ) as *mut PStr;
    if cstr.is_null() {
        return ::core::ptr::null_mut::<PStr>();
    }
    (*cstr).pool = sp;
    (*cstr).refcnt = 1 as ::core::ffi::c_int;
    (*cstr).len = len as size_t;
    memcpy(
        &raw mut (*cstr).str_0 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        str as *const ::core::ffi::c_void,
        (len + 1 as ssize_t) as size_t,
    );
    ok = cbtree_insert((*sp).tree, cstr as *mut ::core::ffi::c_void);
    if !ok {
        cx_free((*sp).ca, cstr as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<PStr>();
    }
    (*sp).count += 1;
    cstr
}
#[no_mangle]

pub unsafe extern "C" fn strpool_incref(mut s: *mut PStr) {
    if !s.is_null() {
        (*s).refcnt += 1;
    }
}
#[no_mangle]

pub unsafe extern "C" fn strpool_decref(mut s: *mut PStr) {
    let mut sp = ::core::ptr::null_mut::<StrPool>();
    if s.is_null() {
        return;
    }
    (*s).refcnt -= 1;
    if (*s).refcnt > 0 as ::core::ffi::c_int {
        return;
    }
    sp = (*s).pool;
    (*sp).count -= 1;
    cbtree_delete(
        (*sp).tree,
        &raw mut (*s).str_0 as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        (*s).len,
    );
    free_str(NULL, s as *mut ::core::ffi::c_void);
}
