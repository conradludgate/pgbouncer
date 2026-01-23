#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:19"]
pub mod _types_h {
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:19"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:19"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
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
        #[c2rust::src_loc = "111:1"]
        pub fn cx_free(cx: *const CxMem, ptr: *mut ::core::ffi::c_void);
        #[c2rust::src_loc = "121:1"]
        pub fn cx_alloc0(cx: *const CxMem, len: size_t) -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/slab.h:19"]
pub mod slab_h {
    #[c2rust::src_loc = "41:1"]
    pub type slab_init_fn = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    #[c2rust::src_loc = "67:1"]
    pub type slab_stat_fn = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            ::core::ffi::c_uint,
            ::core::ffi::c_uint,
            ::core::ffi::c_uint,
        ) -> (),
    >;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/statlist.h:23"]
pub mod statlist_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:1"]
    pub struct StatList {
        pub head: List,
        pub cur_count: ::core::ffi::c_int,
    }
    #[inline]
    #[c2rust::src_loc = "55:1"]
    pub unsafe extern "C" fn statlist_prepend(mut list: *mut StatList, mut item: *mut List) {
        list_prepend(&raw mut (*list).head, item);
        (*list).cur_count += 1;
    }
    #[inline]
    #[c2rust::src_loc = "62:1"]
    pub unsafe extern "C" fn statlist_append(mut list: *mut StatList, mut item: *mut List) {
        list_append(&raw mut (*list).head, item);
        (*list).cur_count += 1;
    }
    #[inline]
    #[c2rust::src_loc = "69:1"]
    pub unsafe extern "C" fn statlist_remove(mut list: *mut StatList, mut item: *mut List) {
        list_del(item);
        (*list).cur_count -= 1;
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
    #[c2rust::src_loc = "88:1"]
    pub unsafe extern "C" fn statlist_count(mut list: *const StatList) -> ::core::ffi::c_int {
        (*list).cur_count
    }
    #[inline]
    #[c2rust::src_loc = "95:1"]
    pub unsafe extern "C" fn statlist_pop(mut list: *mut StatList) -> *mut List {
        let mut item = list_pop(&raw mut (*list).head);
        if !item.is_null() {
            (*list).cur_count -= 1;
        }
        item
    }
    use super::list_h::{list_append, list_del, list_init, list_pop, list_prepend, List};
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/list.h:23"]
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
        std::ptr::eq((*list).next, list) as ::core::ffi::c_int
    }
    #[inline]
    #[c2rust::src_loc = "58:1"]
    pub unsafe extern "C" fn list_prepend(mut list: *mut List, mut item: *mut List) -> *mut List {
        (*item).next = (*list).next;
        (*item).prev = list;
        (*(*list).next).prev = item;
        (*list).next = item;
        item
    }
    #[inline]
    #[c2rust::src_loc = "68:1"]
    pub unsafe extern "C" fn list_append(mut list: *mut List, mut item: *mut List) -> *mut List {
        (*item).next = list;
        (*item).prev = (*list).prev;
        (*(*list).prev).next = item;
        (*list).prev = item;
        item
    }
    #[inline]
    #[c2rust::src_loc = "78:1"]
    pub unsafe extern "C" fn list_del(mut item: *mut List) -> *mut List {
        (*(*item).prev).next = (*item).next;
        (*(*item).next).prev = (*item).prev;
        (*item).prev = item;
        (*item).next = (*item).prev;
        item
    }
    #[inline]
    #[c2rust::src_loc = "87:1"]
    pub unsafe extern "C" fn list_pop(mut list: *mut List) -> *mut List {
        if list_empty(list) != 0 {
            return ::core::ptr::null_mut::<List>();
        }
        list_del((*list).next)
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_param.h:19"]
pub mod _param_h {
    #[c2rust::src_loc = "17:9"]
    pub const __DARWIN_ALIGNBYTES: usize =
        ::core::mem::size_of::<__darwin_size_t>().wrapping_sub(1_usize);
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:21"]
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
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:19"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:19"]
pub mod sys__types_h {
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}
pub use self::_null_h::NULL;
pub use self::_param_h::__DARWIN_ALIGNBYTES;
pub use self::_size_t_h::size_t;
use self::_string_h::{memcpy, memset, strlen};
pub use self::_types_h::__darwin_size_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::cxalloc_h::{cx_alloc0, cx_free, CxMem, CxOps};
pub use self::list_h::{
    list_append, list_del, list_empty, list_init, list_pop, list_prepend, List,
};
pub use self::slab_h::{slab_init_fn, slab_stat_fn};
pub use self::statlist_h::{
    statlist_append, statlist_count, statlist_init, statlist_pop, statlist_prepend,
    statlist_remove, StatList,
};
pub use self::sys__types_h::__DARWIN_NULL;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "30:1"]
pub struct Slab {
    pub head: List,
    pub freelist: StatList,
    pub fraglist: StatList,
    pub name: [::core::ffi::c_char; 32],
    pub final_size: ::core::ffi::c_uint,
    pub total_count: ::core::ffi::c_uint,
    pub init_func: slab_init_fn,
    pub cx: *const CxMem,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "45:1"]
pub struct SlabFrag {
    pub head: List,
}
#[c2rust::src_loc = "50:1"]
static mut slab_list: StatList = StatList {
    head: List {
        next: ::core::ptr::null::<List>() as *mut List,
        prev: ::core::ptr::null::<List>() as *mut List,
    },
    cur_count: 0,
};
#[c2rust::src_loc = "52:1"]
unsafe extern "C" fn slab_list_append(mut slab: *mut Slab) {
    statlist_append(&raw mut slab_list, &raw mut (*slab).head);
}
#[c2rust::src_loc = "59:1"]
unsafe extern "C" fn slab_list_remove(mut slab: *mut Slab) {
    statlist_remove(&raw mut slab_list, &raw mut (*slab).head);
}
#[c2rust::src_loc = "67:1"]
unsafe extern "C" fn init_slab(
    mut slab: *mut Slab,
    mut name: *const ::core::ffi::c_char,
    mut obj_size: ::core::ffi::c_uint,
    mut align: ::core::ffi::c_uint,
    mut init_func: slab_init_fn,
    mut cx: *const CxMem,
) {
    let mut slen = strlen(name) as ::core::ffi::c_uint;
    list_init(&raw mut (*slab).head);
    statlist_init(&raw mut (*slab).freelist, name);
    statlist_init(&raw mut (*slab).fraglist, name);
    (*slab).total_count = 0 as ::core::ffi::c_uint;
    (*slab).init_func = init_func;
    (*slab).cx = cx;
    if slen as usize >= ::core::mem::size_of::<[::core::ffi::c_char; 32]>() {
        slen = ::core::mem::size_of::<[::core::ffi::c_char; 32]>().wrapping_sub(1_usize)
            as ::core::ffi::c_uint;
    }
    memcpy(
        &raw mut (*slab).name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        name as *const ::core::ffi::c_void,
        slen as size_t,
    );
    (*slab).name[slen as usize] = 0 as ::core::ffi::c_char;
    if (align as usize) < ::core::mem::size_of::<::core::ffi::c_long>() {
        align = 0 as ::core::ffi::c_uint;
    }
    if align == 0 as ::core::ffi::c_uint {
        (*slab).final_size = ((obj_size as __darwin_size_t).wrapping_add(__DARWIN_ALIGNBYTES)
            & !__DARWIN_ALIGNBYTES) as ::core::ffi::c_uint;
    } else {
        (*slab).final_size = ((obj_size as uintptr_t)
            .wrapping_add(align as uintptr_t)
            .wrapping_sub(1 as ::core::ffi::c_int as uintptr_t)
            & !(align as uintptr_t).wrapping_sub(1 as ::core::ffi::c_int as uintptr_t))
            as ::core::ffi::c_uint;
    }
    if ((*slab).final_size as usize) < ::core::mem::size_of::<List>() {
        (*slab).final_size = ::core::mem::size_of::<List>() as ::core::ffi::c_uint;
    }
    slab_list_append(slab);
}
#[no_mangle]
#[c2rust::src_loc = "103:1"]
pub unsafe extern "C" fn slab_create(
    mut name: *const ::core::ffi::c_char,
    mut obj_size: ::core::ffi::c_uint,
    mut align: ::core::ffi::c_uint,
    mut init_func: slab_init_fn,
    mut cx: *const CxMem,
) -> *mut Slab {
    let mut slab = ::core::ptr::null_mut::<Slab>();
    slab = cx_alloc0(cx, ::core::mem::size_of::<Slab>() as size_t) as *mut Slab;
    if !slab.is_null() {
        init_slab(slab, name, obj_size, align, init_func, cx);
    }
    slab
}
#[no_mangle]
#[c2rust::src_loc = "117:1"]
pub unsafe extern "C" fn slab_destroy(mut slab: *mut Slab) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut frag = ::core::ptr::null_mut::<SlabFrag>();
    if slab.is_null() {
        return;
    }
    slab_list_remove(slab);
    item = (*slab).fraglist.head.next;
    tmp = (*(*slab).fraglist.head.next).next;
    while item != &raw mut (*slab).fraglist.head {
        frag = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut SlabFrag;
        cx_free((*slab).cx, frag as *mut ::core::ffi::c_void);
        item = tmp;
        tmp = (*tmp).next;
    }
    cx_free((*slab).cx, slab as *mut ::core::ffi::c_void);
}
#[c2rust::src_loc = "134:1"]
unsafe extern "C" fn grow(mut slab: *mut Slab) {
    let mut count: ::core::ffi::c_uint = 0;
    let mut i: ::core::ffi::c_uint = 0;
    let mut size: ::core::ffi::c_uint = 0;
    let mut area = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut frag = ::core::ptr::null_mut::<SlabFrag>();
    count = (*slab).total_count;
    if count < 50 as ::core::ffi::c_uint {
        count = ((16 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_div((*slab).final_size);
    }
    if count < 50 as ::core::ffi::c_uint {
        count = 50 as ::core::ffi::c_uint;
    }
    size = count.wrapping_mul((*slab).final_size);
    frag = cx_alloc0(
        (*slab).cx,
        (size as size_t).wrapping_add(::core::mem::size_of::<SlabFrag>() as size_t),
    ) as *mut SlabFrag;
    if frag.is_null() {
        return;
    }
    list_init(&raw mut (*frag).head);
    area = (frag as *mut ::core::ffi::c_char).add(::core::mem::size_of::<SlabFrag>());
    i = 0 as ::core::ffi::c_uint;
    while i < count {
        let mut obj =
            area.offset(i.wrapping_mul((*slab).final_size) as isize) as *mut ::core::ffi::c_void;
        let mut head = obj as *mut List;
        list_init(head);
        statlist_append(&raw mut (*slab).freelist, head);
        i = i.wrapping_add(1);
    }
    (*slab).total_count = (*slab).total_count.wrapping_add(count);
    statlist_append(&raw mut (*slab).fraglist, &raw mut (*frag).head);
}
#[no_mangle]
#[c2rust::src_loc = "169:1"]
pub unsafe extern "C" fn slab_alloc(mut slab: *mut Slab) -> *mut ::core::ffi::c_void {
    let mut item = statlist_pop(&raw mut (*slab).freelist);
    if item.is_null() {
        grow(slab);
        item = statlist_pop(&raw mut (*slab).freelist);
    }
    if !item.is_null() {
        if (*slab).init_func.is_some() {
            (*slab).init_func.expect("non-null function pointer")(item as *mut ::core::ffi::c_void);
        } else {
            memset(
                item as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (*slab).final_size as size_t,
            );
        }
    }
    item as *mut ::core::ffi::c_void
}
#[no_mangle]
#[c2rust::src_loc = "186:1"]
pub unsafe extern "C" fn slab_free(mut slab: *mut Slab, mut obj: *mut ::core::ffi::c_void) {
    let mut item = obj as *mut List;
    list_init(item);
    statlist_prepend(&raw mut (*slab).freelist, item);
}
#[no_mangle]
#[c2rust::src_loc = "194:1"]
pub unsafe extern "C" fn slab_total_count(mut slab: *const Slab) -> ::core::ffi::c_int {
    (*slab).total_count as ::core::ffi::c_int
}
#[no_mangle]
#[c2rust::src_loc = "200:1"]
pub unsafe extern "C" fn slab_free_count(mut slab: *const Slab) -> ::core::ffi::c_int {
    statlist_count(&raw const (*slab).freelist)
}
#[no_mangle]
#[c2rust::src_loc = "206:1"]
pub unsafe extern "C" fn slab_active_count(mut slab: *const Slab) -> ::core::ffi::c_int {
    slab_total_count(slab) - slab_free_count(slab)
}
#[c2rust::src_loc = "211:1"]
unsafe extern "C" fn run_slab_stats(
    mut slab: *mut Slab,
    mut cb_func: slab_stat_fn,
    mut cb_arg: *mut ::core::ffi::c_void,
) {
    let mut free = statlist_count(&raw mut (*slab).freelist) as ::core::ffi::c_uint;
    cb_func.expect("non-null function pointer")(
        cb_arg,
        &raw mut (*slab).name as *mut ::core::ffi::c_char,
        (*slab).final_size,
        free,
        (*slab).total_count,
    );
}
#[no_mangle]
#[c2rust::src_loc = "218:1"]
pub unsafe extern "C" fn slab_stats(
    mut cb_func: slab_stat_fn,
    mut cb_arg: *mut ::core::ffi::c_void,
) {
    let mut slab = ::core::ptr::null_mut::<Slab>();
    let mut item = ::core::ptr::null_mut::<List>();
    item = slab_list.head.next;
    while item != &raw mut slab_list.head {
        slab = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut Slab;
        run_slab_stats(slab, cb_func, cb_arg);
        item = (*item).next;
    }
}
unsafe extern "C" fn run_static_initializers() {
    slab_list = StatList {
        head: List {
            next: &raw mut slab_list.head,
            prev: &raw mut slab_list.head,
        },
        cur_count: 0 as ::core::ffi::c_int,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
