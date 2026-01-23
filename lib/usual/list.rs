#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/list.h:19"]
pub mod list_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "35:1"]
    pub struct List {
        pub next: *mut List,
        pub prev: *mut List,
    }
    #[c2rust::src_loc = "140:1"]
    pub type list_cmp_f =
        Option<unsafe extern "C" fn(*const List, *const List) -> ::core::ffi::c_int>;
    #[inline]
    #[c2rust::src_loc = "52:1"]
    pub unsafe extern "C" fn list_empty(mut list: *const List) -> ::core::ffi::c_int {
        return ((*list).next == list as *mut List) as ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:19"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::_types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:19"]
pub mod _types_h {
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}
pub use self::_null_h::NULL;
pub use self::_types_h::__DARWIN_NULL;
pub use self::list_h::{list_cmp_f, list_empty, List};
#[c2rust::src_loc = "22:1"]
unsafe extern "C" fn merge(
    mut cmp_func: list_cmp_f,
    mut p: *mut List,
    mut q: *mut List,
) -> *mut List {
    let mut res: [List; 1] = [List {
        next: ::core::ptr::null_mut::<List>(),
        prev: ::core::ptr::null_mut::<List>(),
    }; 1];
    let mut tail = &raw mut res as *mut List;
    let mut e = ::core::ptr::null_mut::<List>();
    while !p.is_null() && !q.is_null() {
        if cmp_func.expect("non-null function pointer")(p, q) <= 0 as ::core::ffi::c_int {
            e = p;
            p = (*p).next;
        } else {
            e = q;
            q = (*q).next;
        }
        (*tail).next = e;
        tail = e;
    }
    (*tail).next = if !p.is_null() { p } else { q };
    return (*(&raw mut res as *mut List)).next;
}
#[no_mangle]
#[c2rust::src_loc = "46:1"]
pub unsafe extern "C" fn list_sort(mut list: *mut List, mut cmp_func: list_cmp_f) {
    let mut i: ::core::ffi::c_int = 0;
    let mut top = 0 as ::core::ffi::c_int;
    let mut p = ::core::ptr::null_mut::<List>();
    let mut stack: [*mut List; 64] = [::core::ptr::null_mut::<List>(); 64];
    if list_empty(list) != 0 {
        return;
    }
    while (*list).next != list {
        p = (*list).next;
        (*list).next = (*p).next;
        (*p).next = ::core::ptr::null_mut::<List>();
        i = 0 as ::core::ffi::c_int;
        while i < top && !stack[i as usize].is_null() {
            p = merge(cmp_func, stack[i as usize], p);
            stack[i as usize] = ::core::ptr::null_mut::<List>();
            i += 1;
        }
        stack[i as usize] = p;
        if i == top {
            top += 1;
        }
    }
    p = ::core::ptr::null_mut::<List>();
    i = 0 as ::core::ffi::c_int;
    while i < top {
        p = merge(cmp_func, stack[i as usize], p);
        i += 1;
    }
    (*list).next = p;
    p = list;
    while !(*p).next.is_null() {
        (*(*p).next).prev = p;
        p = (*p).next;
    }
    (*list).prev = p;
    (*p).next = list;
}
