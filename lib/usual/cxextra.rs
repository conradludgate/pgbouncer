#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:6"]
pub mod _types_h {
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:6"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:6"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/cxalloc.h:6"]
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
        #[c2rust::src_loc = "104:1"]
        pub fn cx_realloc(
            cx: *const CxMem,
            ptr: *mut ::core::ffi::c_void,
            len: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "111:1"]
        pub fn cx_free(cx: *const CxMem, ptr: *mut ::core::ffi::c_void);
        #[c2rust::src_loc = "118:1"]
        pub fn cx_destroy(cx: *const CxMem);
        #[c2rust::src_loc = "143:1"]
        pub static cx_libc_allocator: CxMem;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/list.h:7"]
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
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:6"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:8"]
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
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_param.h:6"]
pub mod _param_h {
    #[c2rust::src_loc = "17:9"]
    pub const __DARWIN_ALIGNBYTES: usize =
        (::core::mem::size_of::<__darwin_size_t>() as usize).wrapping_sub(1 as usize);
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:6"]
pub mod sys__types_h {
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdlib.h:6"]
pub mod _stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "160:1"]
        pub fn exit(_: ::core::ffi::c_int) -> !;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/llvm/21.1.8/lib/clang/21/include/stdbool.h:6"]
pub mod stdbool_h {
    #[c2rust::src_loc = "25:9"]
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/bits.h:8"]
pub mod bits_h {
    #[inline]
    #[c2rust::src_loc = "35:1"]
    pub unsafe extern "C" fn is_power_of_2(mut n: ::core::ffi::c_uint) -> bool {
        return n > 0 as ::core::ffi::c_uint && n & n.wrapping_sub(1 as ::core::ffi::c_uint) == 0;
    }
}
pub use self::_null_h::NULL;
pub use self::_param_h::__DARWIN_ALIGNBYTES;
pub use self::_size_t_h::size_t;
use self::_stdlib_h::exit;
use self::_string_h::{memcpy, memset};
pub use self::_types_h::__darwin_size_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::bits_h::is_power_of_2;
pub use self::cxalloc_h::{
    cx_alloc, cx_destroy, cx_free, cx_libc_allocator, cx_realloc, CxMem, CxOps,
};
pub use self::list_h::{list_append, list_del, list_init, List};
pub use self::stdbool_h::true_0;
pub use self::sys__types_h::__DARWIN_NULL;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "75:1"]
pub struct CxPool {
    pub this: CxMem,
    pub parent: *const CxMem,
    pub last: *mut CxPoolSeg,
    pub last_ptr: *mut ::core::ffi::c_uchar,
    pub align: ::core::ffi::c_uint,
    pub allow_free_first: bool,
    pub first_seg: CxPoolSeg,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "68:1"]
pub struct CxPoolSeg {
    pub prev: *mut CxPoolSeg,
    pub seg_start: *mut ::core::ffi::c_uchar,
    pub seg_pos: *mut ::core::ffi::c_uchar,
    pub seg_end: *mut ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "270:1"]
pub struct CxTree {
    pub this: CxMem,
    pub real: *const CxMem,
    pub alloc_list: List,
    pub subtree_node: List,
    pub subtree_list: List,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "279:1"]
pub struct CxTreeItem {
    pub node: List,
}
#[inline]
#[c2rust::src_loc = "16:1"]
unsafe extern "C" fn p_move(
    mut p: *const ::core::ffi::c_void,
    mut ofs: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    return (p as *mut ::core::ffi::c_char).offset(ofs as isize) as *mut ::core::ffi::c_void;
}
#[c2rust::src_loc = "26:1"]
unsafe extern "C" fn nofail_alloc(
    mut next: *mut ::core::ffi::c_void,
    mut len: size_t,
) -> *mut ::core::ffi::c_void {
    let mut p = cx_alloc(next as *const CxMem, len);
    if p.is_null() {
        exit(1 as ::core::ffi::c_int);
    }
    return p;
}
#[c2rust::src_loc = "34:1"]
unsafe extern "C" fn nofail_realloc(
    mut next: *mut ::core::ffi::c_void,
    mut ptr: *mut ::core::ffi::c_void,
    mut len: size_t,
) -> *mut ::core::ffi::c_void {
    let mut p = cx_realloc(next as *const CxMem, ptr, len);
    if p.is_null() {
        exit(1 as ::core::ffi::c_int);
    }
    return p;
}
#[c2rust::src_loc = "42:1"]
unsafe extern "C" fn nofail_free(
    mut next: *mut ::core::ffi::c_void,
    mut ptr: *mut ::core::ffi::c_void,
) {
    cx_free(next as *const CxMem, ptr);
}
#[c2rust::src_loc = "47:1"]
unsafe extern "C" fn nofail_destroy(mut next: *mut ::core::ffi::c_void) {
    cx_destroy(next as *const CxMem);
}
#[no_mangle]
#[c2rust::src_loc = "52:1"]
pub static mut cx_nofail_ops: CxOps = unsafe {
    CxOps {
        c_alloc: Some(
            nofail_alloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> *mut ::core::ffi::c_void,
        ),
        c_realloc: Some(
            nofail_realloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> *mut ::core::ffi::c_void,
        ),
        c_free: Some(
            nofail_free
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
        ),
        c_destroy: Some(nofail_destroy as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    }
};
#[no_mangle]
#[c2rust::src_loc = "59:1"]
pub static mut cx_libc_nofail: CxMem = unsafe {
    CxMem {
        ops: &raw const cx_nofail_ops,
        ctx: &raw const cx_libc_allocator as *mut ::core::ffi::c_void,
    }
};
#[c2rust::src_loc = "85:9"]
pub const POOL_HDR: __darwin_size_t =
    ::core::mem::size_of::<CxPoolSeg>().wrapping_add(__DARWIN_ALIGNBYTES) & !__DARWIN_ALIGNBYTES;
#[c2rust::src_loc = "87:1"]
unsafe extern "C" fn new_seg(mut pool: *mut CxPool, mut nsize: size_t) -> *mut CxPoolSeg {
    let mut seg = ::core::ptr::null_mut::<CxPoolSeg>();
    let mut ptr = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut alloc: size_t = POOL_HDR.wrapping_add(nsize);
    seg = cx_alloc((*pool).parent as *const CxMem, alloc) as *mut CxPoolSeg;
    if seg.is_null() {
        return ::core::ptr::null_mut::<CxPoolSeg>();
    }
    ptr = seg as *mut ::core::ffi::c_uchar;
    (*seg).seg_start = ((ptr.offset(
        (::core::mem::size_of::<CxPoolSeg>().wrapping_add(
            (::core::mem::size_of::<__darwin_size_t>() as __darwin_size_t)
                .wrapping_sub(1 as __darwin_size_t),
        ) & !(::core::mem::size_of::<__darwin_size_t>() as __darwin_size_t)
            .wrapping_sub(1 as __darwin_size_t)) as isize,
    ) as uintptr_t)
        .wrapping_add((*pool).align as uintptr_t)
        .wrapping_sub(1 as ::core::ffi::c_int as uintptr_t)
        & !((*pool).align as uintptr_t).wrapping_sub(1 as ::core::ffi::c_int as uintptr_t))
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_uchar;
    (*seg).seg_pos = (*seg).seg_start;
    (*seg).seg_end = (seg as *mut ::core::ffi::c_uchar).offset(alloc as isize);
    (*seg).prev = (*pool).last;
    (*pool).last = seg;
    (*pool).last_ptr = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    return seg;
}
#[c2rust::src_loc = "106:1"]
unsafe extern "C" fn pool_alloc(
    mut ctx: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    let mut pool = ctx as *mut CxPool;
    let mut seg = (*pool).last;
    let mut ptr = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut nsize: ::core::ffi::c_uint = 0;
    size = ((size as uintptr_t)
        .wrapping_add((*pool).align as uintptr_t)
        .wrapping_sub(1 as ::core::ffi::c_int as uintptr_t)
        & !((*pool).align as uintptr_t).wrapping_sub(1 as ::core::ffi::c_int as uintptr_t))
        as size_t;
    if !seg.is_null() && (*seg).seg_pos.offset(size as isize) <= (*seg).seg_end {
        ptr = (*seg).seg_pos as *mut ::core::ffi::c_void;
        (*seg).seg_pos = (*seg).seg_pos.offset(size as isize);
        (*pool).last_ptr = ptr as *mut ::core::ffi::c_uchar;
        return ptr;
    } else {
        nsize = (if !seg.is_null() {
            2 as ::core::ffi::c_long
                * (*seg).seg_end.offset_from((*seg).seg_start) as ::core::ffi::c_long
        } else {
            512 as ::core::ffi::c_long
        }) as ::core::ffi::c_uint;
        while (nsize as size_t) < size {
            nsize = nsize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
        seg = new_seg(pool, nsize as size_t);
        if seg.is_null() {
            return NULL;
        }
        ptr = (*seg).seg_pos as *mut ::core::ffi::c_void;
        (*seg).seg_pos = (*seg).seg_pos.offset(size as isize);
        (*pool).last_ptr = ptr as *mut ::core::ffi::c_uchar;
        return ptr;
    };
}
#[c2rust::src_loc = "134:1"]
unsafe extern "C" fn pool_free(
    mut ctx: *mut ::core::ffi::c_void,
    mut ptr: *mut ::core::ffi::c_void,
) {
    let mut pool = ctx as *mut CxPool;
    let mut cur = (*pool).last;
    if (*pool).last_ptr != ptr as *mut ::core::ffi::c_uchar {
        return;
    }
    (*cur).seg_pos = ptr as *mut ::core::ffi::c_uchar;
    (*pool).last_ptr = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
}
#[c2rust::src_loc = "145:1"]
unsafe extern "C" fn pool_guess_old_len(
    mut pool: *mut CxPool,
    mut ptr: *mut ::core::ffi::c_uchar,
) -> size_t {
    let mut seg = (*pool).last;
    let mut cstart = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    while !seg.is_null() {
        cstart = ((seg.offset(1 as ::core::ffi::c_int as isize) as uintptr_t)
            .wrapping_add((*pool).align as uintptr_t)
            .wrapping_sub(1 as ::core::ffi::c_int as uintptr_t)
            & !((*pool).align as uintptr_t).wrapping_sub(1 as ::core::ffi::c_int as uintptr_t))
            as *mut ::core::ffi::c_void as *mut ::core::ffi::c_uchar;
        if ptr >= cstart && ptr < (*seg).seg_pos {
            return (*seg).seg_pos.offset_from(ptr) as ::core::ffi::c_long as size_t;
        }
        seg = (*seg).prev;
    }
    return 0 as size_t;
}
#[c2rust::src_loc = "160:1"]
unsafe extern "C" fn pool_realloc(
    mut ctx: *mut ::core::ffi::c_void,
    mut ptr: *mut ::core::ffi::c_void,
    mut len: size_t,
) -> *mut ::core::ffi::c_void {
    let mut pool = ctx as *mut CxPool;
    let mut seg = (*pool).last;
    let mut p = ptr as *mut ::core::ffi::c_uchar;
    let mut olen: size_t = 0;
    if (*pool).last_ptr != ptr as *mut ::core::ffi::c_uchar {
        olen = pool_guess_old_len(pool, ptr as *mut ::core::ffi::c_uchar);
        p = pool_alloc(ctx, len) as *mut ::core::ffi::c_uchar;
        if p.is_null() {
            return NULL;
        }
        if olen > len {
            olen = len;
        }
        memcpy(p as *mut ::core::ffi::c_void, ptr, olen);
        return p as *mut ::core::ffi::c_void;
    }
    olen = (*seg).seg_pos.offset_from(p) as ::core::ffi::c_long as size_t;
    if (*seg).seg_pos.offset(-(olen as isize)).offset(len as isize) <= (*seg).seg_end {
        (*seg).seg_pos = p.offset(len as isize);
        return p as *mut ::core::ffi::c_void;
    } else {
        p = pool_alloc(ctx, len) as *mut ::core::ffi::c_uchar;
        if p.is_null() {
            return NULL;
        }
        memcpy(p as *mut ::core::ffi::c_void, ptr, olen);
        return p as *mut ::core::ffi::c_void;
    };
}
#[c2rust::src_loc = "191:1"]
unsafe extern "C" fn pool_destroy(mut ctx: *mut ::core::ffi::c_void) {
    let mut pool = ctx as *mut CxPool;
    let mut cur = ::core::ptr::null_mut::<CxPoolSeg>();
    let mut prev = ::core::ptr::null_mut::<CxPoolSeg>();
    if pool.is_null() {
        return;
    }
    cur = (*pool).last;
    while !cur.is_null() {
        prev = (*cur).prev;
        if prev.is_null() {
            break;
        }
        cx_free(
            (*pool).parent as *const CxMem,
            cur as *mut ::core::ffi::c_void,
        );
        cur = prev;
    }
    if (*pool).allow_free_first {
        cx_free(
            (*pool).parent as *const CxMem,
            pool as *mut ::core::ffi::c_void,
        );
    }
}
#[c2rust::src_loc = "208:1"]
static mut pool_ops: CxOps = unsafe {
    CxOps {
        c_alloc: Some(
            pool_alloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> *mut ::core::ffi::c_void,
        ),
        c_realloc: Some(
            pool_realloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> *mut ::core::ffi::c_void,
        ),
        c_free: Some(
            pool_free
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
        ),
        c_destroy: Some(pool_destroy as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    }
};
#[no_mangle]
#[c2rust::src_loc = "219:1"]
pub unsafe extern "C" fn cx_new_pool_from_area(
    mut parent: *const CxMem,
    mut buf: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut allow_free: bool,
    mut align: ::core::ffi::c_uint,
) -> *const CxMem {
    let mut head = ::core::ptr::null_mut::<CxPool>();
    if size < ::core::mem::size_of::<CxPool>() as usize {
        return ::core::ptr::null::<CxMem>();
    }
    if align == 0 as ::core::ffi::c_uint {
        align = 8 as ::core::ffi::c_uint;
    } else if !is_power_of_2(align) {
        return ::core::ptr::null::<CxMem>();
    }
    head = buf as *mut CxPool;
    memset(
        head as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<CxPool>() as size_t,
    );
    (*head).parent = parent as *const CxMem;
    (*head).this.ops = &raw const pool_ops;
    (*head).this.ctx = head as *mut ::core::ffi::c_void;
    (*head).last = &raw mut (*head).first_seg;
    (*head).allow_free_first = allow_free;
    (*head).align = align;
    (*head).first_seg.seg_start = ((head.offset(1 as ::core::ffi::c_int as isize) as uintptr_t)
        .wrapping_add(align as uintptr_t)
        .wrapping_sub(1 as ::core::ffi::c_int as uintptr_t)
        & !(align as uintptr_t).wrapping_sub(1 as ::core::ffi::c_int as uintptr_t))
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_uchar;
    (*head).first_seg.seg_pos = (*head).first_seg.seg_start;
    (*head).first_seg.seg_end = (head as *mut ::core::ffi::c_uchar).offset(size as isize);
    return &raw mut (*head).this;
}
#[no_mangle]
#[c2rust::src_loc = "247:1"]
pub unsafe extern "C" fn cx_new_pool(
    mut parent: *const CxMem,
    mut initial_size: size_t,
    mut align: ::core::ffi::c_uint,
) -> *const CxMem {
    let mut area = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut size: size_t = 0;
    if initial_size < 1024 as size_t {
        initial_size = 1024 as size_t;
    }
    size =
        (::core::mem::size_of::<CxPool>() as usize).wrapping_add(initial_size as usize) as size_t;
    area = cx_alloc(parent, size);
    if area.is_null() {
        return ::core::ptr::null::<CxMem>();
    }
    return cx_new_pool_from_area(parent, area, size, true_0 != 0, align);
}
#[c2rust::src_loc = "268:9"]
pub const TREE_HDR: ::core::ffi::c_int = ::core::mem::size_of::<CxTreeItem>() as ::core::ffi::c_int;
#[c2rust::src_loc = "283:1"]
unsafe extern "C" fn tree_alloc(
    mut ctx: *mut ::core::ffi::c_void,
    mut len: size_t,
) -> *mut ::core::ffi::c_void {
    let mut tree = ctx as *mut CxTree;
    let mut item = ::core::ptr::null_mut::<CxTreeItem>();
    item = cx_alloc((*tree).real, (TREE_HDR as size_t).wrapping_add(len)) as *mut CxTreeItem;
    if item.is_null() {
        return NULL;
    }
    list_init(&raw mut (*item).node);
    list_append(&raw mut (*tree).alloc_list, &raw mut (*item).node);
    return p_move(item as *const ::core::ffi::c_void, TREE_HDR);
}
#[c2rust::src_loc = "297:1"]
unsafe extern "C" fn tree_realloc(
    mut ctx: *mut ::core::ffi::c_void,
    mut ptr: *mut ::core::ffi::c_void,
    mut len: size_t,
) -> *mut ::core::ffi::c_void {
    let mut t = ctx as *mut CxTree;
    let mut item = ::core::ptr::null_mut::<CxTreeItem>();
    let mut item2 = ::core::ptr::null_mut::<CxTreeItem>();
    item = p_move(ptr, -TREE_HDR) as *mut CxTreeItem;
    list_del(&raw mut (*item).node);
    item2 = cx_realloc(
        (*t).real,
        item as *mut ::core::ffi::c_void,
        (TREE_HDR as size_t).wrapping_add(len),
    ) as *mut CxTreeItem;
    if !item2.is_null() {
        list_append(&raw mut (*t).alloc_list, &raw mut (*item2).node);
        return p_move(item2 as *const ::core::ffi::c_void, TREE_HDR);
    } else {
        list_append(&raw mut (*t).alloc_list, &raw mut (*item).node);
        return NULL;
    };
}
#[c2rust::src_loc = "314:1"]
unsafe extern "C" fn tree_free(
    mut ctx: *mut ::core::ffi::c_void,
    mut ptr: *mut ::core::ffi::c_void,
) {
    let mut t = ctx as *mut CxTree;
    let mut item = ::core::ptr::null_mut::<CxTreeItem>();
    item = p_move(ptr, -TREE_HDR) as *mut CxTreeItem;
    list_del(&raw mut (*item).node);
    cx_free((*t).real, item as *mut ::core::ffi::c_void);
}
#[c2rust::src_loc = "324:1"]
unsafe extern "C" fn tree_destroy(mut ctx: *mut ::core::ffi::c_void) {
    let mut tree = ctx as *mut CxTree;
    let mut sub = ::core::ptr::null_mut::<CxTree>();
    let mut item = ::core::ptr::null_mut::<CxTreeItem>();
    let mut el = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    list_del(&raw mut (*tree).subtree_node);
    el = (*tree).alloc_list.next;
    tmp = (*(*tree).alloc_list.next).next;
    while el != &raw mut (*tree).alloc_list {
        list_del(el);
        item = (el as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut CxTreeItem;
        cx_free((*tree).real, item as *mut ::core::ffi::c_void);
        el = tmp;
        tmp = (*tmp).next;
    }
    el = (*tree).subtree_list.next;
    tmp = (*(*tree).subtree_list.next).next;
    while el != &raw mut (*tree).subtree_list {
        sub = (el as *mut ::core::ffi::c_char).offset(-(40 as ::core::ffi::c_ulong as isize))
            as *mut CxTree;
        tree_destroy(sub as *mut ::core::ffi::c_void);
        el = tmp;
        tmp = (*tmp).next;
    }
    cx_free((*tree).real, tree as *mut ::core::ffi::c_void);
}
#[c2rust::src_loc = "350:1"]
static mut tree_ops: CxOps = unsafe {
    CxOps {
        c_alloc: Some(
            tree_alloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> *mut ::core::ffi::c_void,
        ),
        c_realloc: Some(
            tree_realloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> *mut ::core::ffi::c_void,
        ),
        c_free: Some(
            tree_free
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
        ),
        c_destroy: Some(tree_destroy as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    }
};
#[no_mangle]
#[c2rust::src_loc = "358:1"]
pub unsafe extern "C" fn cx_new_tree(mut cx: *const CxMem) -> *const CxMem {
    let mut t = ::core::ptr::null_mut::<CxTree>();
    let mut parent = ::core::ptr::null_mut::<CxTree>();
    let mut real = cx;
    if (*cx).ops == &raw const tree_ops {
        parent = (*cx).ctx as *mut CxTree;
        real = (*parent).real;
    }
    t = cx_alloc(real, ::core::mem::size_of::<CxTree>() as size_t) as *mut CxTree;
    if t.is_null() {
        return ::core::ptr::null::<CxMem>();
    }
    (*t).real = real;
    (*t).this.ops = &raw const tree_ops;
    (*t).this.ctx = t as *mut ::core::ffi::c_void;
    list_init(&raw mut (*t).alloc_list);
    list_init(&raw mut (*t).subtree_node);
    list_init(&raw mut (*t).subtree_list);
    if !parent.is_null() {
        list_append(&raw mut (*parent).subtree_list, &raw mut (*t).subtree_node);
    }
    return &raw mut (*t).this;
}
