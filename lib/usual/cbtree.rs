#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:26"]
pub mod _types_h {
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:26"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:26"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/cxalloc.h:26"]
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
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/cbtree.h:26"]
pub mod cbtree_h {
    #[c2rust::src_loc = "27:1"]
    pub type cbtree_getkey_func = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut *const ::core::ffi::c_void,
        ) -> size_t,
    >;
    #[c2rust::src_loc = "29:1"]
    pub type cbtree_walker_func =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> bool>;
    use super::_size_t_h::size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:26"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:28"]
pub mod _string_h {
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "75:1"]
        pub fn memcmp(
            __s1: *const ::core::ffi::c_void,
            __s2: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "83:1"]
        pub fn memset(
            __b: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __len: size_t,
        ) -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:26"]
pub mod sys__types_h {
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdint.h:26"]
pub mod stdint_h {
    #[c2rust::src_loc = "154:9"]
    pub const UINTPTR_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
    #[c2rust::src_loc = "173:9"]
    pub const SIZE_MAX: ::core::ffi::c_ulong = UINTPTR_MAX;
}
#[c2rust::header_src = "/opt/homebrew/Cellar/llvm/21.1.8/lib/clang/21/include/stdbool.h:26"]
pub mod stdbool_h {
    #[c2rust::src_loc = "25:9"]
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/bits.h:28"]
pub mod bits_h {
    #[inline]
    #[c2rust::src_loc = "107:1"]
    pub unsafe extern "C" fn usual_fls(mut x: ::core::ffi::c_int) -> ::core::ffi::c_int {
        return (if x == 0 as ::core::ffi::c_int {
            0 as usize
        } else {
            (8 as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                .wrapping_sub((x as ::core::ffi::c_uint).leading_zeros() as i32 as usize)
        }) as ::core::ffi::c_int;
    }
}
pub use self::_null_h::NULL;
pub use self::_size_t_h::size_t;
use self::_string_h::{memcmp, memset};
pub use self::_types_h::__darwin_size_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::bits_h::usual_fls;
pub use self::cbtree_h::{cbtree_getkey_func, cbtree_walker_func};
pub use self::cxalloc_h::{cx_alloc, cx_free, CxMem, CxOps};
pub use self::stdbool_h::{false_0, true_0};
pub use self::stdint_h::{SIZE_MAX, UINTPTR_MAX};
pub use self::sys__types_h::__DARWIN_NULL;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "45:1"]
pub struct CBTree {
    pub root: *mut Node,
    pub obj_key_cb: cbtree_getkey_func,
    pub obj_free_cb: cbtree_walker_func,
    pub cb_ctx: *mut ::core::ffi::c_void,
    pub cx: *const CxMem,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "40:1"]
pub struct Node {
    pub child: [*mut Node; 2],
    pub bitpos: size_t,
}
#[c2rust::src_loc = "54:9"]
pub const SAME_KEY: ::core::ffi::c_ulong = SIZE_MAX;
#[c2rust::src_loc = "56:9"]
pub const MAX_KEY: ::core::ffi::c_ulong = SIZE_MAX.wrapping_div(8 as ::core::ffi::c_ulong);
#[inline]
#[c2rust::src_loc = "63:1"]
unsafe extern "C" fn is_node(mut ptr: *mut ::core::ffi::c_void) -> bool {
    return ptr as uintptr_t & 1 as ::core::ffi::c_int as uintptr_t == 0 as uintptr_t;
}
#[inline]
#[c2rust::src_loc = "69:1"]
unsafe extern "C" fn set_external(mut obj: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    return (obj as uintptr_t | 1 as ::core::ffi::c_int as uintptr_t) as *mut ::core::ffi::c_void;
}
#[inline]
#[c2rust::src_loc = "75:1"]
unsafe extern "C" fn get_external(
    mut extval: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    return (extval as uintptr_t & !(1 as ::core::ffi::c_int) as uintptr_t)
        as *mut ::core::ffi::c_void;
}
#[inline]
#[c2rust::src_loc = "81:1"]
unsafe extern "C" fn get_bit(
    mut bitpos: size_t,
    mut key: *const ::core::ffi::c_uchar,
    mut klen: size_t,
) -> ::core::ffi::c_uint {
    let mut pos: size_t = bitpos.wrapping_div(8 as size_t);
    let mut bit =
        (7 as size_t).wrapping_sub(bitpos.wrapping_rem(8 as size_t)) as ::core::ffi::c_uint;
    return (pos < klen
        && *key.offset(pos as isize) as ::core::ffi::c_int & (1 as ::core::ffi::c_int) << bit != 0)
        as ::core::ffi::c_int as ::core::ffi::c_uint;
}
#[inline]
#[c2rust::src_loc = "89:1"]
unsafe extern "C" fn get_key(
    mut tree: *mut CBTree,
    mut obj: *mut ::core::ffi::c_void,
    mut key_p: *mut *const ::core::ffi::c_void,
) -> size_t {
    return (*tree).obj_key_cb.expect("non-null function pointer")((*tree).cb_ctx, obj, key_p);
}
#[inline]
#[c2rust::src_loc = "95:1"]
unsafe extern "C" fn key_matches(
    mut tree: *mut CBTree,
    mut obj: *mut ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_void,
    mut klen: size_t,
) -> bool {
    let mut o_key = ::core::ptr::null::<::core::ffi::c_void>();
    let mut o_klen: size_t = 0;
    o_klen = get_key(tree, obj, &raw mut o_key);
    return o_klen == klen && memcmp(key, o_key, klen) == 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "104:1"]
unsafe extern "C" fn find_crit_bit(
    mut a: *const ::core::ffi::c_uchar,
    mut alen: size_t,
    mut b: *const ::core::ffi::c_uchar,
    mut blen: size_t,
) -> size_t {
    let mut current_block: u64;
    let mut av: ::core::ffi::c_uchar = 0;
    let mut bv: ::core::ffi::c_uchar = 0;
    let mut c: ::core::ffi::c_uchar = 0;
    let mut pos: ::core::ffi::c_uchar = 0;
    let mut i: size_t = 0;
    let mut minlen: size_t = if alen > blen { blen } else { alen };
    let mut maxlen: size_t = if alen > blen { alen } else { blen };
    i = 0 as size_t;
    loop {
        if !(i < minlen) {
            current_block = 2473556513754201174;
            break;
        }
        av = *a.offset(i as isize);
        bv = *b.offset(i as isize);
        if av as ::core::ffi::c_int != bv as ::core::ffi::c_int {
            current_block = 6683740114474407808;
            break;
        }
        i = i.wrapping_add(1);
    }
    loop {
        match current_block {
            2473556513754201174 => {
                if i < maxlen {
                    av = (if i < alen {
                        *a.offset(i as isize) as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uchar;
                    bv = (if i < blen {
                        *b.offset(i as isize) as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uchar;
                    if av as ::core::ffi::c_int != bv as ::core::ffi::c_int {
                        current_block = 6683740114474407808;
                        continue;
                    }
                    i = i.wrapping_add(1);
                    current_block = 2473556513754201174;
                } else {
                    return SAME_KEY as size_t;
                }
            }
            _ => {
                c = (av as ::core::ffi::c_int ^ bv as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                pos = (8 as ::core::ffi::c_int - usual_fls(c as ::core::ffi::c_int))
                    as ::core::ffi::c_uchar;
                return i.wrapping_mul(8 as size_t).wrapping_add(pos as size_t);
            }
        }
    }
}
#[c2rust::src_loc = "144:1"]
unsafe extern "C" fn raw_lookup(
    mut tree: *mut CBTree,
    mut key: *const ::core::ffi::c_void,
    mut klen: size_t,
) -> *mut ::core::ffi::c_void {
    let mut node = (*tree).root;
    let mut bit: ::core::ffi::c_uint = 0;
    while is_node(node as *mut ::core::ffi::c_void) {
        bit = get_bit((*node).bitpos, key as *const ::core::ffi::c_uchar, klen);
        node = (*node).child[bit as usize];
    }
    return get_external(node as *mut ::core::ffi::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "156:1"]
pub unsafe extern "C" fn cbtree_lookup(
    mut tree: *mut CBTree,
    mut key: *const ::core::ffi::c_void,
    mut klen: size_t,
) -> *mut ::core::ffi::c_void {
    let mut obj = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if (*tree).root.is_null() {
        return NULL;
    }
    obj = raw_lookup(tree, key, klen);
    if key_matches(tree, obj, key, klen) {
        return obj;
    }
    return NULL;
}
#[c2rust::src_loc = "179:1"]
unsafe extern "C" fn new_node(mut tree: *mut CBTree) -> *mut Node {
    let mut node = cx_alloc((*tree).cx, ::core::mem::size_of::<Node>() as size_t) as *mut Node;
    if node.is_null() {
        return ::core::ptr::null_mut::<Node>();
    }
    memset(
        node as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Node>() as size_t,
    );
    return node;
}
#[c2rust::src_loc = "189:1"]
unsafe extern "C" fn insert_first(
    mut tree: *mut CBTree,
    mut obj: *mut ::core::ffi::c_void,
) -> bool {
    (*tree).root = set_external(obj) as *mut Node;
    return true_0 != 0;
}
#[c2rust::src_loc = "196:1"]
unsafe extern "C" fn insert_at(
    mut tree: *mut CBTree,
    mut newbit: size_t,
    mut key: *const ::core::ffi::c_void,
    mut klen: size_t,
    mut obj: *mut ::core::ffi::c_void,
) -> bool {
    let mut pos: *mut *mut Node = &raw mut (*tree).root;
    let mut node = ::core::ptr::null_mut::<Node>();
    let mut bit: ::core::ffi::c_uint = 0;
    while is_node(*pos as *mut ::core::ffi::c_void) as ::core::ffi::c_int != 0
        && (**pos).bitpos < newbit
    {
        bit = get_bit((**pos).bitpos, key as *const ::core::ffi::c_uchar, klen);
        pos = (&raw mut (**pos).child as *mut *mut Node).offset(bit as isize) as *mut *mut Node;
    }
    bit = get_bit(newbit, key as *const ::core::ffi::c_uchar, klen);
    node = new_node(tree);
    if node.is_null() {
        return false_0 != 0;
    }
    (*node).bitpos = newbit;
    (*node).child[bit as usize] = set_external(obj) as *mut Node;
    (*node).child[(bit ^ 1 as ::core::ffi::c_uint) as usize] = *pos;
    *pos = node;
    return true_0 != 0;
}
#[no_mangle]
#[c2rust::src_loc = "220:1"]
pub unsafe extern "C" fn cbtree_insert(
    mut tree: *mut CBTree,
    mut obj: *mut ::core::ffi::c_void,
) -> bool {
    let mut key = ::core::ptr::null::<::core::ffi::c_void>();
    let mut old_key = ::core::ptr::null::<::core::ffi::c_void>();
    let mut newbit: size_t = 0;
    let mut klen: size_t = 0;
    let mut old_klen: size_t = 0;
    let mut old_obj = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if (*tree).root.is_null() {
        return insert_first(tree, obj);
    }
    klen = get_key(tree, obj, &raw mut key);
    if klen > MAX_KEY as size_t {
        return false_0 != 0;
    }
    old_obj = raw_lookup(tree, key, klen);
    old_klen = get_key(tree, old_obj, &raw mut old_key);
    newbit = find_crit_bit(
        key as *const ::core::ffi::c_uchar,
        klen,
        old_key as *const ::core::ffi::c_uchar,
        old_klen,
    );
    if newbit == SAME_KEY as size_t {
        return false_0 != 0;
    }
    return insert_at(tree, newbit, key, klen, obj);
}
#[no_mangle]
#[c2rust::src_loc = "251:1"]
pub unsafe extern "C" fn cbtree_delete(
    mut tree: *mut CBTree,
    mut key: *const ::core::ffi::c_void,
    mut klen: size_t,
) -> bool {
    let mut obj = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut tmp = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut bit = 0 as ::core::ffi::c_uint;
    let mut pos: *mut *mut Node = &raw mut (*tree).root;
    let mut prev_pos = ::core::ptr::null_mut::<*mut Node>();
    if (*tree).root.is_null() {
        return false_0 != 0;
    }
    while is_node(*pos as *mut ::core::ffi::c_void) {
        bit = get_bit((**pos).bitpos, key as *const ::core::ffi::c_uchar, klen);
        prev_pos = pos;
        pos = (&raw mut (**pos).child as *mut *mut Node).offset(bit as isize) as *mut *mut Node;
    }
    obj = get_external(*pos as *mut ::core::ffi::c_void);
    if !key_matches(tree, obj, key, klen) {
        return false_0 != 0;
    }
    if (*tree).obj_free_cb.is_some() {
        (*tree).obj_free_cb.expect("non-null function pointer")((*tree).cb_ctx, obj);
    }
    if !prev_pos.is_null() {
        tmp = *prev_pos as *mut ::core::ffi::c_void;
        *prev_pos = (**prev_pos).child[(bit ^ 1 as ::core::ffi::c_uint) as usize];
        cx_free((*tree).cx, tmp);
    } else {
        (*tree).root = ::core::ptr::null_mut::<Node>();
    }
    return true_0 != 0;
}
#[no_mangle]
#[c2rust::src_loc = "293:1"]
pub unsafe extern "C" fn cbtree_create(
    mut obj_key_cb: cbtree_getkey_func,
    mut obj_free_cb: cbtree_walker_func,
    mut cb_ctx: *mut ::core::ffi::c_void,
    mut cx: *const CxMem,
) -> *mut CBTree {
    let mut tree = cx_alloc(cx, ::core::mem::size_of::<CBTree>() as size_t) as *mut CBTree;
    if tree.is_null() {
        return ::core::ptr::null_mut::<CBTree>();
    }
    (*tree).root = ::core::ptr::null_mut::<Node>();
    (*tree).cb_ctx = cb_ctx;
    (*tree).obj_key_cb = obj_key_cb;
    (*tree).obj_free_cb = obj_free_cb;
    (*tree).cx = cx;
    return tree;
}
#[c2rust::src_loc = "310:1"]
unsafe extern "C" fn destroy_node(mut tree: *mut CBTree, mut node: *mut Node) {
    if is_node(node as *mut ::core::ffi::c_void) {
        destroy_node(tree, (*node).child[0 as ::core::ffi::c_int as usize]);
        destroy_node(tree, (*node).child[1 as ::core::ffi::c_int as usize]);
        cx_free((*tree).cx, node as *mut ::core::ffi::c_void);
    } else if (*tree).obj_free_cb.is_some() {
        let mut obj = get_external(node as *mut ::core::ffi::c_void);
        (*tree).obj_free_cb.expect("non-null function pointer")((*tree).cb_ctx, obj);
    }
}
#[no_mangle]
#[c2rust::src_loc = "323:1"]
pub unsafe extern "C" fn cbtree_destroy(mut tree: *mut CBTree) {
    if !(*tree).root.is_null() {
        destroy_node(tree, (*tree).root);
    }
    (*tree).root = ::core::ptr::null_mut::<Node>();
    cx_free((*tree).cx, tree as *mut ::core::ffi::c_void);
}
#[c2rust::src_loc = "335:1"]
unsafe extern "C" fn walk(
    mut node: *mut Node,
    mut cb_func: cbtree_walker_func,
    mut cb_arg: *mut ::core::ffi::c_void,
) -> bool {
    if !is_node(node as *mut ::core::ffi::c_void) {
        return cb_func.expect("non-null function pointer")(
            cb_arg,
            get_external(node as *mut ::core::ffi::c_void),
        );
    }
    return walk(
        (*node).child[0 as ::core::ffi::c_int as usize],
        cb_func,
        cb_arg,
    ) as ::core::ffi::c_int
        != 0
        && walk(
            (*node).child[1 as ::core::ffi::c_int as usize],
            cb_func,
            cb_arg,
        ) as ::core::ffi::c_int
            != 0;
}
#[no_mangle]
#[c2rust::src_loc = "343:1"]
pub unsafe extern "C" fn cbtree_walk(
    mut tree: *mut CBTree,
    mut cb_func: cbtree_walker_func,
    mut cb_arg: *mut ::core::ffi::c_void,
) -> bool {
    if (*tree).root.is_null() {
        return true_0 != 0;
    }
    return walk((*tree).root, cb_func, cb_arg);
}
