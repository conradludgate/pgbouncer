#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:23"]
pub mod _types_h {
    #[c2rust::src_loc = "32:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "34:1"]
    pub type __uint16_t = u16;
    #[c2rust::src_loc = "35:1"]
    pub type __int32_t = i32;
    #[c2rust::src_loc = "36:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "77:1"]
    pub type __darwin_ptrdiff_t = isize;
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
    #[c2rust::src_loc = "118:1"]
    pub type __darwin_ssize_t = isize;
    #[c2rust::src_loc = "119:1"]
    pub type __darwin_time_t = ::core::ffi::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h:23"]
pub mod _uintptr_t_h {
    #[c2rust::src_loc = "34:1"]
    pub type uintptr_t = usize;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:23"]
pub mod sys__types_h {
    #[c2rust::src_loc = "84:1"]
    pub type __darwin_pid_t = __int32_t;
    #[c2rust::src_loc = "86:1"]
    pub type __darwin_suseconds_t = __int32_t;
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_uid_t = __uint32_t;
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    use super::_types_h::{__int32_t, __uint32_t};
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_in_addr_t.h:23"]
pub mod _in_addr_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type in_addr_t = __uint32_t;
    use super::_types_h::__uint32_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_in_port_t.h:23"]
pub mod _in_port_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type in_port_t = __uint16_t;
    use super::_types_h::__uint16_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_pid_t.h:23"]
pub mod _pid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type pid_t = __darwin_pid_t;
    use super::sys__types_h::__darwin_pid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uid_t.h:23"]
pub mod _uid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uid_t = __darwin_uid_t;
    use super::sys__types_h::__darwin_uid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:23"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ssize_t.h:23"]
pub mod _ssize_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ptrdiff_t.h:23"]
pub mod _ptrdiff_t_h {
    #[c2rust::src_loc = "51:1"]
    pub type ptrdiff_t = __darwin_ptrdiff_t;
    use super::_types_h::__darwin_ptrdiff_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint8_t.h:23"]
pub mod _uint8_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint8_t = u8;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint16_t.h:23"]
pub mod _uint16_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint16_t = u16;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint32_t.h:23"]
pub mod _uint32_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint32_t = u32;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types/_uint64_t.h:23"]
pub mod _uint64_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uint64_t = u64;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_timeval.h:23"]
pub mod _timeval_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "34:1"]
    pub struct timeval {
        pub tv_sec: __darwin_time_t,
        pub tv_usec: __darwin_suseconds_t,
    }
    use super::_types_h::__darwin_time_t;
    use super::sys__types_h::__darwin_suseconds_t;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/tls/tls.h:23"]
pub mod tls_h {
    extern "C" {
        #[c2rust::src_loc = "66:1"]
        pub type tls;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/time.h:23"]
pub mod time_h {
    #[c2rust::src_loc = "40:1"]
    pub type usec_t = uint64_t;
    #[c2rust::src_loc = "43:9"]
    pub const USEC: usec_t = 1000000 as ::core::ffi::c_int as usec_t;
    use super::_uint64_t_h::uint64_t;
    extern "C" {
        #[c2rust::src_loc = "54:1"]
        pub fn get_cached_time() -> usec_t;
    }
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
    #[c2rust::src_loc = "52:1"]
    pub unsafe extern "C" fn list_empty(mut list: *const List) -> ::core::ffi::c_int {
        return ((*list).next == list as *mut List) as ::core::ffi::c_int;
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
    #[c2rust::src_loc = "88:1"]
    pub unsafe extern "C" fn statlist_count(mut list: *const StatList) -> ::core::ffi::c_int {
        return (*list).cur_count;
    }
    #[inline]
    #[c2rust::src_loc = "120:1"]
    pub unsafe extern "C" fn statlist_empty(mut list: *const StatList) -> bool {
        return list_empty(&raw const (*list).head) != 0;
    }
    use super::list_h::{list_append, list_del, list_empty, List};
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/aatree.h:23"]
pub mod aatree_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "41:1"]
    pub struct AATree {
        pub root: *mut AANode,
        pub count: ::core::ffi::c_int,
        pub node_cmp: aatree_cmp_f,
        pub release_cb: aatree_walker_f,
    }
    #[c2rust::src_loc = "36:1"]
    pub type aatree_walker_f =
        Option<unsafe extern "C" fn(*mut AANode, *mut ::core::ffi::c_void) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "57:1"]
    pub struct AANode {
        pub left: *mut AANode,
        pub right: *mut AANode,
        pub level: ::core::ffi::c_int,
    }
    #[c2rust::src_loc = "33:1"]
    pub type aatree_cmp_f =
        Option<unsafe extern "C" fn(uintptr_t, *mut AANode) -> ::core::ffi::c_int>;
    #[c2rust::src_loc = "66:1"]
    pub type AATreeWalkType = ::core::ffi::c_uint;
    #[c2rust::src_loc = "69:2"]
    pub const AA_WALK_POST_ORDER: AATreeWalkType = 2;
    #[c2rust::src_loc = "68:2"]
    pub const AA_WALK_PRE_ORDER: AATreeWalkType = 1;
    #[c2rust::src_loc = "67:2"]
    pub const AA_WALK_IN_ORDER: AATreeWalkType = 0;
    use super::_uintptr_t_h::uintptr_t;
    extern "C" {
        #[c2rust::src_loc = "85:1"]
        pub fn aatree_walk(
            tree: *mut AATree,
            wtype: AATreeWalkType,
            walker: aatree_walker_f,
            arg: *mut ::core::ffi::c_void,
        );
        #[c2rust::src_loc = "88:1"]
        pub fn aatree_destroy(tree: *mut AATree);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_sa_family_t.h:23"]
pub mod _sa_family_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type sa_family_t = __uint8_t;
    use super::_types_h::__uint8_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/socket.h:23"]
pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "414:1"]
    pub struct sockaddr {
        pub sa_len: __uint8_t,
        pub sa_family: sa_family_t,
        pub sa_data: [::core::ffi::c_char; 14],
    }
    use super::_sa_family_t_h::sa_family_t;
    use super::_types_h::__uint8_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet/in.h:23"]
pub mod in_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "301:1"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "374:1"]
    pub struct sockaddr_in {
        pub sin_len: __uint8_t,
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [::core::ffi::c_char; 8],
    }
    use super::_in_addr_t_h::in_addr_t;
    use super::_in_port_t_h::in_port_t;
    use super::_sa_family_t_h::sa_family_t;
    use super::_types_h::__uint8_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/netinet6/in6.h:23"]
pub mod in6_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "152:9"]
    pub struct in6_addr {
        pub __u6_addr: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "153:2"]
    pub union C2RustUnnamed {
        pub __u6_addr8: [__uint8_t; 16],
        pub __u6_addr16: [__uint16_t; 8],
        pub __u6_addr32: [__uint32_t; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "170:1"]
    pub struct sockaddr_in6 {
        pub sin6_len: __uint8_t,
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: __uint32_t,
        pub sin6_addr: in6_addr,
        pub sin6_scope_id: __uint32_t,
    }
    use super::_in_port_t_h::in_port_t;
    use super::_sa_family_t_h::sa_family_t;
    use super::_types_h::{__uint16_t, __uint32_t, __uint8_t};
}
#[c2rust::header_src = "/opt/homebrew/Cellar/libevent/2.1.12_1/include/event2/event.h:23"]
pub mod event_h {
    #[c2rust::src_loc = "1014:1"]
    pub type event_callback_fn = Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_short,
            *mut ::core::ffi::c_void,
        ) -> (),
    >;
    #[c2rust::src_loc = "934:9"]
    pub const EV_PERSIST: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    use super::_timeval_h::timeval;
    use super::event_struct_h::event;
    extern "C" {
        #[c2rust::src_loc = "217:1"]
        pub type event_base;
        #[c2rust::src_loc = "860:1"]
        pub fn event_base_loopbreak(_: *mut event_base) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1132:1"]
        pub fn event_assign(
            _: *mut event,
            _: *mut event_base,
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_short,
            _: event_callback_fn,
            _: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1233:1"]
        pub fn event_add(ev: *mut event, timeout: *const timeval) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/libevent/2.1.12_1/include/event2/event_struct.h:23"]
pub mod event_struct_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "123:1"]
    pub struct event {
        pub ev_evcallback: event_callback,
        pub ev_timeout_pos: C2RustUnnamed_5,
        pub ev_fd: ::core::ffi::c_int,
        pub ev_base: *mut event_base,
        pub ev_: C2RustUnnamed_0,
        pub ev_events: ::core::ffi::c_short,
        pub ev_res: ::core::ffi::c_short,
        pub ev_timeout: timeval,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "135:2"]
    pub union C2RustUnnamed_0 {
        pub ev_io: C2RustUnnamed_3,
        pub ev_signal: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "143:3"]
    pub struct C2RustUnnamed_1 {
        pub ev_signal_next: C2RustUnnamed_2,
        pub ev_ncalls: ::core::ffi::c_short,
        pub ev_pncalls: *mut ::core::ffi::c_short,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "144:4"]
    pub struct C2RustUnnamed_2 {
        pub le_next: *mut event,
        pub le_prev: *mut *mut event,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "137:3"]
    pub struct C2RustUnnamed_3 {
        pub ev_io_next: C2RustUnnamed_4,
        pub ev_timeout: timeval,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "138:4"]
    pub struct C2RustUnnamed_4 {
        pub le_next: *mut event,
        pub le_prev: *mut *mut event,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "127:2"]
    pub union C2RustUnnamed_5 {
        pub ev_next_with_common_timeout: C2RustUnnamed_6,
        pub min_heap_idx: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "128:3"]
    pub struct C2RustUnnamed_6 {
        pub tqe_next: *mut event,
        pub tqe_prev: *mut *mut event,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "107:1"]
    pub struct event_callback {
        pub evcb_active_next: C2RustUnnamed_8,
        pub evcb_flags: ::core::ffi::c_short,
        pub evcb_pri: uint8_t,
        pub evcb_closure: uint8_t,
        pub evcb_cb_union: C2RustUnnamed_7,
        pub evcb_arg: *mut ::core::ffi::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:9"]
    pub union C2RustUnnamed_7 {
        pub evcb_callback: Option<
            unsafe extern "C" fn(
                ::core::ffi::c_int,
                ::core::ffi::c_short,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
        pub evcb_selfcb:
            Option<unsafe extern "C" fn(*mut event_callback, *mut ::core::ffi::c_void) -> ()>,
        pub evcb_evfinalize:
            Option<unsafe extern "C" fn(*mut event, *mut ::core::ffi::c_void) -> ()>,
        pub evcb_cbfinalize:
            Option<unsafe extern "C" fn(*mut event_callback, *mut ::core::ffi::c_void) -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "108:2"]
    pub struct C2RustUnnamed_8 {
        pub tqe_next: *mut event_callback,
        pub tqe_prev: *mut *mut event_callback,
    }
    use super::_timeval_h::timeval;
    use super::_uint8_t_h::uint8_t;
    use super::event_h::event_base;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/cryptohash.h:23"]
pub mod cryptohash_h {
    #[c2rust::src_loc = "19:9"]
    pub type pg_cryptohash_type = ::core::ffi::c_uint;
    #[c2rust::src_loc = "24:2"]
    pub const PG_SHA512: pg_cryptohash_type = 3;
    #[c2rust::src_loc = "23:2"]
    pub const PG_SHA384: pg_cryptohash_type = 2;
    #[c2rust::src_loc = "22:2"]
    pub const PG_SHA256: pg_cryptohash_type = 1;
    #[c2rust::src_loc = "21:2"]
    pub const PG_SHA224: pg_cryptohash_type = 0;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/uthash.h:23"]
pub mod uthash_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1072:9"]
    pub struct UT_hash_bucket {
        pub hh_head: *mut UT_hash_handle,
        pub count: ::core::ffi::c_uint,
        pub expand_mult: ::core::ffi::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1129:9"]
    pub struct UT_hash_handle {
        pub tbl: *mut UT_hash_table,
        pub prev: *mut ::core::ffi::c_void,
        pub next: *mut ::core::ffi::c_void,
        pub hh_prev: *mut UT_hash_handle,
        pub hh_next: *mut UT_hash_handle,
        pub key: *const ::core::ffi::c_void,
        pub keylen: ::core::ffi::c_uint,
        pub hashv: ::core::ffi::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1096:9"]
    pub struct UT_hash_table {
        pub buckets: *mut UT_hash_bucket,
        pub num_buckets: ::core::ffi::c_uint,
        pub log2_num_buckets: ::core::ffi::c_uint,
        pub num_items: ::core::ffi::c_uint,
        pub tail: *mut UT_hash_handle,
        pub hho: ptrdiff_t,
        pub ideal_chain_maxlen: ::core::ffi::c_uint,
        pub nonideal_items: ::core::ffi::c_uint,
        pub ineff_expands: ::core::ffi::c_uint,
        pub noexpand: ::core::ffi::c_uint,
        pub signature: uint32_t,
    }
    use super::_ptrdiff_t_h::ptrdiff_t;
    use super::_uint32_t_h::uint32_t;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/bouncer.h:23"]
pub mod bouncer_h {
    #[c2rust::src_loc = "70:1"]
    pub type SocketState = ::core::ffi::c_uint;
    #[c2rust::src_loc = "88:2"]
    pub const SV_TESTED: SocketState = 16;
    #[c2rust::src_loc = "87:2"]
    pub const SV_USED: SocketState = 15;
    #[c2rust::src_loc = "86:2"]
    pub const SV_ACTIVE_CANCEL: SocketState = 14;
    #[c2rust::src_loc = "85:2"]
    pub const SV_ACTIVE: SocketState = 13;
    #[c2rust::src_loc = "84:2"]
    pub const SV_IDLE: SocketState = 12;
    #[c2rust::src_loc = "83:2"]
    pub const SV_BEING_CANCELED: SocketState = 11;
    #[c2rust::src_loc = "82:2"]
    pub const SV_LOGIN: SocketState = 10;
    #[c2rust::src_loc = "81:2"]
    pub const SV_JUSTFREE: SocketState = 9;
    #[c2rust::src_loc = "80:2"]
    pub const SV_FREE: SocketState = 8;
    #[c2rust::src_loc = "78:2"]
    pub const CL_ACTIVE_CANCEL: SocketState = 7;
    #[c2rust::src_loc = "77:2"]
    pub const CL_WAITING_CANCEL: SocketState = 6;
    #[c2rust::src_loc = "76:2"]
    pub const CL_ACTIVE: SocketState = 5;
    #[c2rust::src_loc = "75:2"]
    pub const CL_WAITING_LOGIN: SocketState = 4;
    #[c2rust::src_loc = "74:2"]
    pub const CL_WAITING: SocketState = 3;
    #[c2rust::src_loc = "73:2"]
    pub const CL_LOGIN: SocketState = 2;
    #[c2rust::src_loc = "72:2"]
    pub const CL_JUSTFREE: SocketState = 1;
    #[c2rust::src_loc = "71:2"]
    pub const CL_FREE: SocketState = 0;
    #[c2rust::src_loc = "91:1"]
    pub type PauseMode = ::core::ffi::c_uint;
    #[c2rust::src_loc = "94:2"]
    pub const P_SUSPEND: PauseMode = 2;
    #[c2rust::src_loc = "93:2"]
    pub const P_PAUSE: PauseMode = 1;
    #[c2rust::src_loc = "92:2"]
    pub const P_NONE: PauseMode = 0;
    #[c2rust::src_loc = "97:1"]
    pub type ShutDownMode = ::core::ffi::c_uint;
    #[c2rust::src_loc = "119:2"]
    pub const SHUTDOWN_IMMEDIATE: ShutDownMode = 3;
    #[c2rust::src_loc = "117:2"]
    pub const SHUTDOWN_WAIT_FOR_CLIENTS: ShutDownMode = 2;
    #[c2rust::src_loc = "106:2"]
    pub const SHUTDOWN_WAIT_FOR_SERVERS: ShutDownMode = 1;
    #[c2rust::src_loc = "99:2"]
    pub const SHUTDOWN_NONE: ShutDownMode = 0;
    #[c2rust::src_loc = "131:1"]
    pub type PacketCallbackFlag = ::core::ffi::c_uint;
    #[c2rust::src_loc = "144:2"]
    pub const CB_HANDLE_COMPLETE_PACKET: PacketCallbackFlag = 2;
    #[c2rust::src_loc = "139:2"]
    pub const CB_WANT_COMPLETE_PACKET: PacketCallbackFlag = 1;
    #[c2rust::src_loc = "133:2"]
    pub const CB_NONE: PacketCallbackFlag = 0;
    #[c2rust::src_loc = "147:1"]
    pub type LoadBalanceHosts = ::core::ffi::c_uint;
    #[c2rust::src_loc = "149:2"]
    pub const LOAD_BALANCE_HOSTS_ROUND_ROBIN: LoadBalanceHosts = 1;
    #[c2rust::src_loc = "148:2"]
    pub const LOAD_BALANCE_HOSTS_DISABLE: LoadBalanceHosts = 0;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "663:1"]
    pub struct PgSocket {
        pub head: List,
        pub cancel_head: List,
        pub link: *mut PgSocket,
        pub pool: *mut PgPool,
        pub login_user_credentials: *mut PgCredentials,
        pub id: ::core::ffi::c_ulonglong,
        pub client_auth_type: ::core::ffi::c_int,
        pub outstanding_requests: StatList,
        #[bitfield(name = "state", ty = "SocketState", bits = "0..=7")]
        #[bitfield(name = "contributes_db_client_count", ty = "bool", bits = "8..=8")]
        #[bitfield(name = "user_connection_counted", ty = "bool", bits = "9..=9")]
        #[bitfield(name = "ready", ty = "bool", bits = "10..=10")]
        #[bitfield(name = "idle_tx", ty = "bool", bits = "11..=11")]
        #[bitfield(name = "close_needed", ty = "bool", bits = "12..=12")]
        #[bitfield(name = "setting_vars", ty = "bool", bits = "13..=13")]
        #[bitfield(name = "exec_on_connect", ty = "bool", bits = "14..=14")]
        #[bitfield(name = "resetting", ty = "bool", bits = "15..=15")]
        #[bitfield(name = "copy_mode", ty = "bool", bits = "16..=16")]
        #[bitfield(name = "wait_for_welcome", ty = "bool", bits = "17..=17")]
        #[bitfield(name = "welcome_sent", ty = "bool", bits = "18..=18")]
        #[bitfield(name = "wait_for_user_conn", ty = "bool", bits = "19..=19")]
        #[bitfield(name = "wait_for_user", ty = "bool", bits = "20..=20")]
        #[bitfield(name = "wait_for_auth", ty = "bool", bits = "21..=21")]
        #[bitfield(name = "suspended", ty = "bool", bits = "22..=22")]
        #[bitfield(name = "sent_wait_notification", ty = "bool", bits = "23..=23")]
        #[bitfield(name = "admin_user", ty = "bool", bits = "24..=24")]
        #[bitfield(name = "own_user", ty = "bool", bits = "25..=25")]
        #[bitfield(name = "wait_for_response", ty = "bool", bits = "26..=26")]
        #[bitfield(name = "wait_sslchar", ty = "bool", bits = "27..=27")]
        #[bitfield(name = "query_failed", ty = "bool", bits = "28..=28")]
        pub state_contributes_db_client_count_user_connection_counted_ready_idle_tx_close_needed_setting_vars_exec_on_connect_resetting_copy_mode_wait_for_welcome_welcome_sent_wait_for_user_conn_wait_for_user_wait_for_auth_suspended_sent_wait_notification_admin_user_own_user_wait_for_response_wait_sslchar_query_failed:
            [u8; 4],
        pub replication: ReplicationType,
        pub startup_options: *mut ::core::ffi::c_char,
        pub connect_time: usec_t,
        pub request_time: usec_t,
        pub query_start: usec_t,
        pub xact_start: usec_t,
        pub wait_start: usec_t,
        pub cancel_key: [uint8_t; 8],
        pub canceling_clients: StatList,
        pub canceled_server: *mut PgSocket,
        pub remote_addr: PgAddr,
        pub local_addr: PgAddr,
        pub host: *mut ::core::ffi::c_char,
        pub c2rust_unnamed: C2RustUnnamed_9,
        pub scram_state: ScramState,
        pub vars: VarCache,
        pub client_prepared_statements: *mut PgClientPreparedStatement,
        pub server_prepared_statements: *mut PgServerPreparedStatement,
        pub packet_cb_state: CallbackState,
        pub sbuf: SBuf,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "768:2"]
    pub struct CallbackState {
        #[bitfield(name = "flag", ty = "PacketCallbackFlag", bits = "0..=7")]
        pub flag: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
        pub pkt: PktHdr,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "732:2"]
    pub struct ScramState {
        pub client_nonce: *mut ::core::ffi::c_char,
        pub client_first_message_bare: *mut ::core::ffi::c_char,
        pub client_final_message_without_proof: *mut ::core::ffi::c_char,
        pub server_nonce: *mut ::core::ffi::c_char,
        pub server_first_message: *mut ::core::ffi::c_char,
        pub iterations: ::core::ffi::c_int,
        pub hash_type: pg_cryptohash_type,
        pub key_length: ::core::ffi::c_int,
        pub salt: *mut uint8_t,
        pub saltlen: ::core::ffi::c_int,
        pub SaltedPassword: *mut uint8_t,
        pub cbind_flag: ::core::ffi::c_char,
        pub adhoc: bool,
        pub encoded_salt: *mut ::core::ffi::c_char,
        pub ClientKey: [uint8_t; 32],
        pub StoredKey: [uint8_t; 32],
        pub ServerKey: [uint8_t; 32],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "727:2"]
    pub union C2RustUnnamed_9 {
        pub dns_token: *mut DNSToken,
        pub db: *mut PgDatabase,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "570:1"]
    pub struct PgDatabase {
        pub head: List,
        pub name: [::core::ffi::c_char; 64],
        pub peer_id: ::core::ffi::c_int,
        pub pool: *mut PgPool,
        pub host: *mut ::core::ffi::c_char,
        pub port: ::core::ffi::c_int,
        pub pool_size: ::core::ffi::c_int,
        pub min_pool_size: ::core::ffi::c_int,
        pub res_pool_size: ::core::ffi::c_int,
        pub pool_mode: ::core::ffi::c_int,
        pub max_db_client_connections: ::core::ffi::c_int,
        pub max_db_connections: ::core::ffi::c_int,
        pub server_lifetime: usec_t,
        pub connect_query: *mut ::core::ffi::c_char,
        pub load_balance_hosts: LoadBalanceHosts,
        pub startup_params: *mut PktBuf,
        pub dbname: *const ::core::ffi::c_char,
        pub auth_dbname: *mut ::core::ffi::c_char,
        pub forced_user_credentials: *mut PgCredentials,
        pub auth_user_credentials: *mut PgCredentials,
        pub auth_query: *mut ::core::ffi::c_char,
        pub db_paused: bool,
        pub db_wait_close: bool,
        pub db_dead: bool,
        pub db_auto: bool,
        pub db_disabled: bool,
        pub admin: bool,
        pub fake: bool,
        pub inactive_time: usec_t,
        pub active_stamp: ::core::ffi::c_uint,
        pub connection_count: ::core::ffi::c_int,
        pub client_connection_count: ::core::ffi::c_int,
        pub user_tree: AATree,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "509:1"]
    pub struct PgCredentials {
        pub tree_node: AANode,
        pub name: [::core::ffi::c_char; 128],
        pub passwd: [::core::ffi::c_char; 2048],
        pub mock_auth: bool,
        pub dynamic_passwd: bool,
        pub global_user: *mut PgGlobalUser,
        pub scram_ClientKey: [uint8_t; 32],
        pub scram_ServerKey: [uint8_t; 32],
        pub scram_StoredKey: [uint8_t; 32],
        pub scram_Iiterations: ::core::ffi::c_int,
        pub scram_SaltKey: *mut ::core::ffi::c_char,
        pub use_scram_keys: bool,
        pub adhoc_scram_secrets_cached: bool,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "549:1"]
    pub struct PgGlobalUser {
        pub credentials: PgCredentials,
        pub head: List,
        pub pool_list: List,
        pub pool_mode: ::core::ffi::c_int,
        pub pool_size: ::core::ffi::c_int,
        pub res_pool_size: ::core::ffi::c_int,
        pub transaction_timeout: usec_t,
        pub idle_transaction_timeout: usec_t,
        pub query_timeout: usec_t,
        pub client_idle_timeout: usec_t,
        pub max_user_connections: ::core::ffi::c_int,
        pub max_user_client_connections: ::core::ffi::c_int,
        pub connection_count: ::core::ffi::c_int,
        pub client_connection_count: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "345:1"]
    pub struct PgPool {
        pub head: List,
        pub map_head: List,
        pub db: *mut PgDatabase,
        pub user_credentials: *mut PgCredentials,
        pub active_client_list: StatList,
        pub waiting_client_list: StatList,
        pub waiting_cancel_req_list: StatList,
        pub active_cancel_req_list: StatList,
        pub active_server_list: StatList,
        pub active_cancel_server_list: StatList,
        pub being_canceled_server_list: StatList,
        pub idle_server_list: StatList,
        pub used_server_list: StatList,
        pub tested_server_list: StatList,
        pub new_server_list: StatList,
        pub stats: PgStats,
        pub newer_stats: PgStats,
        pub older_stats: PgStats,
        pub welcome_msg: *mut PktBuf,
        pub orig_vars: VarCache,
        pub last_lifetime_disconnect: usec_t,
        pub last_connect_time: usec_t,
        #[bitfield(name = "last_connect_failed", ty = "bool", bits = "0..=0")]
        pub last_connect_failed: [u8; 1],
        pub last_connect_failed_message: [::core::ffi::c_char; 100],
        #[bitfield(name = "last_login_failed", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "welcome_msg_ready", ty = "bool", bits = "1..=1")]
        pub last_login_failed_welcome_msg_ready: [u8; 1],
        pub rrcounter: uint16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "320:1"]
    pub struct PgStats {
        pub server_assignment_count: uint64_t,
        pub xact_count: uint64_t,
        pub query_count: uint64_t,
        pub server_bytes: uint64_t,
        pub client_bytes: uint64_t,
        pub xact_time: usec_t,
        pub query_time: usec_t,
        pub wait_time: usec_t,
        pub ps_server_parse_count: uint64_t,
        pub ps_client_parse_count: uint64_t,
        pub ps_bind_count: uint64_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "292:1"]
    pub union PgAddr {
        pub sa: sockaddr,
        pub sin: sockaddr_in,
        pub sin6: sockaddr_in6,
        pub scred: sockaddr_ucreds,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "282:1"]
    pub struct sockaddr_ucreds {
        pub sin: sockaddr_in,
        pub uid: uid_t,
        pub pid: pid_t,
    }
    #[c2rust::src_loc = "650:1"]
    pub type ReplicationType = ::core::ffi::c_uint;
    #[c2rust::src_loc = "653:2"]
    pub const REPLICATION_PHYSICAL: ReplicationType = 2;
    #[c2rust::src_loc = "652:2"]
    pub const REPLICATION_LOGICAL: ReplicationType = 1;
    #[c2rust::src_loc = "651:2"]
    pub const REPLICATION_NONE: ReplicationType = 0;
    #[inline]
    #[c2rust::src_loc = "917:1"]
    pub unsafe extern "C" fn first_socket(mut slist: *mut StatList) -> *mut PgSocket {
        if statlist_empty(slist) {
            return ::core::ptr::null_mut::<PgSocket>();
        }
        return ((*slist).head.next as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut PgSocket;
    }

    use super::_pid_t_h::pid_t;
    use super::_uid_t_h::uid_t;
    use super::_uint16_t_h::uint16_t;
    use super::_uint64_t_h::uint64_t;
    use super::_uint8_t_h::uint8_t;
    use super::aatree_h::{AANode, AATree};
    use super::cryptohash_h::pg_cryptohash_type;
    use super::dnslookup_h::{DNSContext, DNSToken};
    use super::event_h::event_base;
    use super::in6_h::sockaddr_in6;
    use super::in_h::sockaddr_in;
    use super::list_h::List;
    use super::pktbuf_h::PktBuf;
    use super::prepare_h::{PgClientPreparedStatement, PgServerPreparedStatement};
    use super::proto_h::PktHdr;
    use super::sbuf_h::SBuf;
    use super::socket_h::sockaddr;
    use super::statlist_h::{statlist_empty, StatList};
    use super::time_h::usec_t;
    use super::varcache_h::VarCache;
    extern "C" {
        #[c2rust::src_loc = "66:1"]
        pub static mut pgb_event_base: *mut event_base;
        #[c2rust::src_loc = "798:1"]
        pub static mut cf_query_wait_notify: ::core::ffi::c_ulong;
        #[c2rust::src_loc = "822:1"]
        pub static mut cf_autodb_idle_timeout: usec_t;
        #[c2rust::src_loc = "824:1"]
        pub static mut cf_suspend_timeout: usec_t;
        #[c2rust::src_loc = "826:1"]
        pub static mut cf_server_idle_timeout: usec_t;
        #[c2rust::src_loc = "829:1"]
        pub static mut cf_server_check_query: *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "830:1"]
        pub static mut empty_server_check_query: bool;
        #[c2rust::src_loc = "831:1"]
        pub static mut cf_server_check_delay: usec_t;
        #[c2rust::src_loc = "832:1"]
        pub static mut cf_server_fast_close: ::core::ffi::c_int;
        #[c2rust::src_loc = "833:1"]
        pub static mut cf_server_connect_timeout: usec_t;
        #[c2rust::src_loc = "835:1"]
        pub static mut cf_query_timeout: usec_t;
        #[c2rust::src_loc = "836:1"]
        pub static mut cf_query_wait_timeout: usec_t;
        #[c2rust::src_loc = "837:1"]
        pub static mut cf_cancel_wait_timeout: usec_t;
        #[c2rust::src_loc = "838:1"]
        pub static mut cf_client_idle_timeout: usec_t;
        #[c2rust::src_loc = "839:1"]
        pub static mut cf_client_login_timeout: usec_t;
        #[c2rust::src_loc = "840:1"]
        pub static mut cf_idle_transaction_timeout: usec_t;
        #[c2rust::src_loc = "841:1"]
        pub static mut cf_transaction_timeout: usec_t;
        #[c2rust::src_loc = "842:1"]
        pub static mut any_user_level_timeout_set: bool;
        #[c2rust::src_loc = "843:1"]
        pub static mut any_user_level_client_timeout_set: bool;
        #[c2rust::src_loc = "868:1"]
        pub static mut cf_pause_mode: ::core::ffi::c_int;
        #[c2rust::src_loc = "869:1"]
        pub static mut cf_shutdown: ::core::ffi::c_int;
        #[c2rust::src_loc = "870:1"]
        pub static mut cf_reboot: ::core::ffi::c_int;
        #[c2rust::src_loc = "912:1"]
        pub static mut g_suspend_start: usec_t;
        #[c2rust::src_loc = "914:1"]
        pub static mut adns: *mut DNSContext;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/sbuf.h:23"]
pub mod sbuf_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "71:1"]
    pub struct SBuf {
        pub ev: event,
        pub wait_type: uint8_t,
        pub pkt_action: uint8_t,
        pub tls_state: uint8_t,
        pub sock: ::core::ffi::c_int,
        pub pkt_remain: ::core::ffi::c_uint,
        pub skip_remain: ::core::ffi::c_uint,
        pub extra_packets: MBuf,
        pub extra_packet_queue_after: bool,
        pub proto_cb: sbuf_cb_t,
        pub dst: *mut SBuf,
        pub io: *mut IOBuf,
        pub ops: *const SBufIO,
        pub tls: *mut tls,
        pub tls_host: *const ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "58:1"]
    pub struct SBufIO {
        pub sbufio_peek:
            Option<unsafe extern "C" fn(*mut SBuf, *mut ::core::ffi::c_void, size_t) -> ssize_t>,
        pub sbufio_recv:
            Option<unsafe extern "C" fn(*mut SBuf, *mut ::core::ffi::c_void, size_t) -> ssize_t>,
        pub sbufio_send:
            Option<unsafe extern "C" fn(*mut SBuf, *const ::core::ffi::c_void, size_t) -> ssize_t>,
        pub sbufio_close: Option<unsafe extern "C" fn(*mut SBuf) -> ::core::ffi::c_int>,
    }
    #[c2rust::src_loc = "54:1"]
    pub type sbuf_cb_t = Option<unsafe extern "C" fn(*mut SBuf, SBufEvent, *mut MBuf) -> bool>;
    #[c2rust::src_loc = "24:9"]
    pub type SBufEvent = ::core::ffi::c_uint;
    #[c2rust::src_loc = "32:2"]
    pub const SBUF_EV_TLS_READY: SBufEvent = 7;
    #[c2rust::src_loc = "31:2"]
    pub const SBUF_EV_PKT_CALLBACK: SBufEvent = 6;
    #[c2rust::src_loc = "30:2"]
    pub const SBUF_EV_FLUSH: SBufEvent = 5;
    #[c2rust::src_loc = "29:2"]
    pub const SBUF_EV_CONNECT_OK: SBufEvent = 4;
    #[c2rust::src_loc = "28:2"]
    pub const SBUF_EV_CONNECT_FAILED: SBufEvent = 3;
    #[c2rust::src_loc = "27:2"]
    pub const SBUF_EV_SEND_FAILED: SBufEvent = 2;
    #[c2rust::src_loc = "26:2"]
    pub const SBUF_EV_RECV_FAILED: SBufEvent = 1;
    #[c2rust::src_loc = "25:2"]
    pub const SBUF_EV_READ: SBufEvent = 0;
    #[inline]
    #[c2rust::src_loc = "144:1"]
    pub unsafe extern "C" fn sbuf_is_empty(mut sbuf: *mut SBuf) -> bool {
        return iobuf_empty((*sbuf).io) as ::core::ffi::c_int != 0
            && (*sbuf).pkt_remain == 0 as ::core::ffi::c_uint;
    }
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    use super::_uint8_t_h::uint8_t;
    use super::event_struct_h::event;
    use super::iobuf_h::{iobuf_empty, IOBuf};
    use super::mbuf_h::MBuf;
    use super::tls_h::tls;
    extern "C" {
        #[c2rust::src_loc = "121:1"]
        pub fn sbuf_pause(sbuf: *mut SBuf) -> bool;
        #[c2rust::src_loc = "122:1"]
        pub fn sbuf_continue(sbuf: *mut SBuf);
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/iobuf.h:23"]
pub mod iobuf_h {
    #[c2rust::src_loc = "55:1"]
    pub type IOBuf = iobuf;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:1"]
    pub struct iobuf {
        pub done_pos: ::core::ffi::c_uint,
        pub parse_pos: ::core::ffi::c_uint,
        pub recv_pos: ::core::ffi::c_uint,
        pub buf: [uint8_t; 0],
    }
    #[inline]
    #[c2rust::src_loc = "65:1"]
    pub unsafe extern "C" fn iobuf_empty(mut io: *const IOBuf) -> bool {
        return io.is_null() || (*io).done_pos == (*io).recv_pos;
    }
    use super::_uint8_t_h::uint8_t;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/mbuf.h:23"]
pub mod mbuf_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "14:1"]
    pub struct MBuf {
        pub data: *mut uint8_t,
        pub read_pos: ::core::ffi::c_uint,
        pub write_pos: ::core::ffi::c_uint,
        pub alloc_len: ::core::ffi::c_uint,
        pub reader: bool,
        pub fixed: bool,
    }
    use super::_uint8_t_h::uint8_t;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/proto.h:23"]
pub mod proto_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "34:1"]
    pub struct PktHdr {
        pub type_0: ::core::ffi::c_uint,
        pub len: ::core::ffi::c_uint,
        pub data: MBuf,
    }
    use super::mbuf_h::MBuf;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/prepare.h:23"]
pub mod prepare_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:9"]
    pub struct PgServerPreparedStatement {
        pub query_id: uint64_t,
        pub hh: UT_hash_handle,
        pub ps: *mut PgPreparedStatement,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:9"]
    pub struct PgPreparedStatement {
        pub hh: UT_hash_handle,
        pub query_id: uint64_t,
        pub use_count: uint32_t,
        pub query_and_parameters_len: size_t,
        pub stmt_name_len: uint8_t,
        pub stmt_name: [::core::ffi::c_char; 31],
        pub query_and_parameters: [::core::ffi::c_char; 0],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "21:9"]
    pub struct PgClientPreparedStatement {
        pub hh: UT_hash_handle,
        pub ps: *mut PgPreparedStatement,
        pub stmt_name: [::core::ffi::c_char; 0],
    }
    use super::_size_t_h::size_t;
    use super::_uint32_t_h::uint32_t;
    use super::_uint64_t_h::uint64_t;
    use super::_uint8_t_h::uint8_t;
    use super::uthash_h::UT_hash_handle;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/varcache.h:23"]
pub mod varcache_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "14:1"]
    pub struct VarCache {
        pub var_list: *mut *mut PStr,
    }
    use super::strpool_h::PStr;
    extern "C" {
        #[c2rust::src_loc = "24:1"]
        pub fn varcache_clean(cache: *mut VarCache);
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/strpool.h:23"]
pub mod strpool_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:1"]
    pub struct PStr {
        pub pool: *mut StrPool,
        pub len: size_t,
        pub refcnt: ::core::ffi::c_int,
        pub str_0: [::core::ffi::c_char; 0],
    }
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "34:1"]
        pub type StrPool;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/pktbuf.h:23"]
pub mod pktbuf_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "26:1"]
    pub struct PktBuf {
        pub buf: *mut uint8_t,
        pub buf_len: ::core::ffi::c_int,
        pub write_pos: ::core::ffi::c_int,
        pub pktlen_pos: ::core::ffi::c_int,
        pub send_pos: ::core::ffi::c_int,
        pub ev: *mut event,
        pub queued_dst: *mut PgSocket,
        #[bitfield(name = "failed", ty = "bool", bits = "0..=0")]
        #[bitfield(name = "sending", ty = "bool", bits = "1..=1")]
        #[bitfield(name = "fixed_buf", ty = "bool", bits = "2..=2")]
        pub failed_sending_fixed_buf: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
    }
    use super::_uint8_t_h::uint8_t;
    use super::bouncer_h::PgSocket;
    use super::event_struct_h::event;
    extern "C" {
        #[c2rust::src_loc = "44:1"]
        pub fn pktbuf_dynamic(start_len: ::core::ffi::c_int) -> *mut PktBuf;
        #[c2rust::src_loc = "45:1"]
        pub fn pktbuf_static(buf: *mut PktBuf, data: *mut uint8_t, len: ::core::ffi::c_int);
        #[c2rust::src_loc = "47:1"]
        pub fn pktbuf_free(buf: *mut PktBuf);
        #[c2rust::src_loc = "56:1"]
        pub fn pktbuf_send_immediate(buf: *mut PktBuf, sk: *mut PgSocket) -> bool;
        #[c2rust::src_loc = "57:1"]
        pub fn pktbuf_send_queued(buf: *mut PktBuf, sk: *mut PgSocket) -> bool;
        #[c2rust::src_loc = "76:1"]
        pub fn pktbuf_write_generic(
            buf: *mut PktBuf,
            type_0: ::core::ffi::c_int,
            fmt: *const ::core::ffi::c_char,
            ...
        );
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/dnslookup.h:23"]
pub mod dnslookup_h {
    extern "C" {
        #[c2rust::src_loc = "20:1"]
        pub type DNSToken;
        #[c2rust::src_loc = "19:1"]
        pub type DNSContext;
        #[c2rust::src_loc = "35:1"]
        pub fn adns_zone_cache_maint(ctx: *mut DNSContext);
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/logging.h:23"]
pub mod logging_h {
    #[c2rust::src_loc = "47:1"]
    pub type LogLevel = ::core::ffi::c_uint;
    #[c2rust::src_loc = "54:2"]
    pub const LG_NOISE: LogLevel = 6;
    #[c2rust::src_loc = "53:2"]
    pub const LG_DEBUG: LogLevel = 5;
    #[c2rust::src_loc = "52:2"]
    pub const LG_INFO: LogLevel = 4;
    #[c2rust::src_loc = "51:2"]
    pub const LG_STATS: LogLevel = 3;
    #[c2rust::src_loc = "50:2"]
    pub const LG_WARNING: LogLevel = 2;
    #[c2rust::src_loc = "49:2"]
    pub const LG_ERROR: LogLevel = 1;
    #[c2rust::src_loc = "48:2"]
    pub const LG_FATAL: LogLevel = 0;
    extern "C" {
        #[c2rust::src_loc = "85:1"]
        pub static mut cf_verbose: ::core::ffi::c_int;
        #[c2rust::src_loc = "117:1"]
        pub fn log_generic(
            level: LogLevel,
            ctx: *mut ::core::ffi::c_void,
            s: *const ::core::ffi::c_char,
            ...
        );
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/objects.h:23"]
pub mod objects_h {
    use super::bouncer_h::{PgCredentials, PgDatabase, PgPool, PgSocket, SocketState};

    use super::statlist_h::StatList;
    extern "C" {
        #[c2rust::src_loc = "27:8"]
        pub type Slab;
        #[c2rust::src_loc = "21:1"]
        pub static mut pool_list: StatList;
        #[c2rust::src_loc = "22:1"]
        pub static mut peer_pool_list: StatList;
        #[c2rust::src_loc = "23:1"]
        pub static mut database_list: StatList;
        #[c2rust::src_loc = "24:1"]
        pub static mut peer_list: StatList;
        #[c2rust::src_loc = "25:1"]
        pub static mut autodatabase_idle_list: StatList;
        #[c2rust::src_loc = "26:1"]
        pub static mut login_client_list: StatList;
        #[c2rust::src_loc = "29:1"]
        pub static mut db_cache: *mut Slab;
        #[c2rust::src_loc = "30:1"]
        pub static mut peer_cache: *mut Slab;
        #[c2rust::src_loc = "31:1"]
        pub static mut peer_pool_cache: *mut Slab;
        #[c2rust::src_loc = "32:1"]
        pub static mut pool_cache: *mut Slab;
        #[c2rust::src_loc = "34:1"]
        pub static mut credentials_cache: *mut Slab;
        #[c2rust::src_loc = "37:1"]
        pub static mut var_list_cache: *mut Slab;
        #[c2rust::src_loc = "48:1"]
        pub fn get_pool(db: *mut PgDatabase, user_credentials: *mut PgCredentials) -> *mut PgPool;
        #[c2rust::src_loc = "55:1"]
        pub fn life_over(server: *mut PgSocket) -> bool;
        #[c2rust::src_loc = "56:1"]
        pub fn release_server(server: *mut PgSocket) -> bool;
        #[c2rust::src_loc = "61:1"]
        pub fn disconnect_server(
            server: *mut PgSocket,
            notify: bool,
            reason: *const ::core::ffi::c_char,
            ...
        );
        #[c2rust::src_loc = "62:1"]
        pub fn disconnect_client(
            client: *mut PgSocket,
            notify: bool,
            reason: *const ::core::ffi::c_char,
            ...
        );
        #[c2rust::src_loc = "84:1"]
        pub fn launch_new_connection(pool: *mut PgPool, evict_if_needed: bool);
        #[c2rust::src_loc = "97:1"]
        pub fn activate_client(client: *mut PgSocket);
        #[c2rust::src_loc = "100:1"]
        pub fn change_server_state(server: *mut PgSocket, newstate: SocketState);
        #[c2rust::src_loc = "102:1"]
        pub fn get_active_client_count() -> ::core::ffi::c_int;
        #[c2rust::src_loc = "103:1"]
        pub fn get_active_server_count() -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:23"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/server.h:23"]
pub mod server_h {
    use super::bouncer_h::{PgDatabase, PgPool};
    use super::time_h::usec_t;
    extern "C" {
        #[c2rust::src_loc = "24:1"]
        pub fn pool_pool_size(pool: *mut PgPool) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "25:1"]
        pub fn pool_min_pool_size(pool: *mut PgPool) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "26:1"]
        pub fn pool_server_lifetime(pool: *mut PgPool) -> usec_t;
        #[c2rust::src_loc = "27:1"]
        pub fn database_min_pool_size(db: *mut PgDatabase) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "28:1"]
        pub fn pool_res_pool_size(pool: *mut PgPool) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/slab.h:25"]
pub mod slab_h {
    use super::objects_h::Slab;
    extern "C" {
        #[c2rust::src_loc = "55:1"]
        pub fn slab_free(slab: *mut Slab, obj: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/malloc/_malloc.h:23"]
pub mod _malloc_h {
    extern "C" {
        #[c2rust::src_loc = "56:1"]
        pub fn free(_: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdlib.h:23"]
pub mod _stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "160:1"]
        pub fn exit(_: ::core::ffi::c_int) -> !;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/errno.h:23"]
pub mod errno_h {
    extern "C" {
        #[c2rust::src_loc = "80:1"]
        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/llvm/21.1.8/lib/clang/21/include/stdbool.h:23"]
pub mod stdbool_h {
    #[c2rust::src_loc = "25:9"]
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_string.h:23"]
pub mod _string_h {
    extern "C" {
        #[c2rust::src_loc = "95:1"]
        pub fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/common/protocol.h:23"]
pub mod protocol_h {
    #[c2rust::src_loc = "26:9"]
    pub const PqMsg_Query: ::core::ffi::c_int = 'Q' as i32;
    #[c2rust::src_loc = "49:9"]
    pub const PqMsg_NoticeResponse: ::core::ffi::c_int = 'N' as i32;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/admin.h:23"]
pub mod admin_h {
    extern "C" {
        #[c2rust::src_loc = "23:1"]
        pub fn admin_pause_done();
        #[c2rust::src_loc = "24:1"]
        pub fn admin_wait_close_done();
    }
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/include/pooler.h:23"]
pub mod pooler_h {
    extern "C" {
        #[c2rust::src_loc = "21:1"]
        pub fn resume_pooler();
        #[c2rust::src_loc = "26:1"]
        pub fn cleanup_unix_sockets();
    }
}
pub use self::_in_addr_t_h::in_addr_t;
pub use self::_in_port_t_h::in_port_t;
use self::_malloc_h::free;
pub use self::_null_h::NULL;
pub use self::_pid_t_h::pid_t;
pub use self::_ptrdiff_t_h::ptrdiff_t;
pub use self::_sa_family_t_h::sa_family_t;
pub use self::_size_t_h::size_t;
pub use self::_ssize_t_h::ssize_t;
use self::_stdlib_h::exit;
use self::_string_h::strerror;
pub use self::_timeval_h::timeval;
pub use self::_types_h::{
    __darwin_ptrdiff_t, __darwin_size_t, __darwin_ssize_t, __darwin_time_t, __int32_t, __uint16_t,
    __uint32_t, __uint8_t,
};
pub use self::_uid_t_h::uid_t;
pub use self::_uint16_t_h::uint16_t;
pub use self::_uint32_t_h::uint32_t;
pub use self::_uint64_t_h::uint64_t;
pub use self::_uint8_t_h::uint8_t;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::aatree_h::{
    aatree_cmp_f, aatree_destroy, aatree_walk, aatree_walker_f, AANode, AATree, AATreeWalkType,
    AA_WALK_IN_ORDER, AA_WALK_POST_ORDER, AA_WALK_PRE_ORDER,
};
use self::admin_h::{admin_pause_done, admin_wait_close_done};
pub use self::bouncer_h::{
    adns, any_user_level_client_timeout_set, any_user_level_timeout_set, cf_autodb_idle_timeout,
    cf_cancel_wait_timeout, cf_client_idle_timeout, cf_client_login_timeout,
    cf_idle_transaction_timeout, cf_pause_mode, cf_query_timeout, cf_query_wait_notify,
    cf_query_wait_timeout, cf_reboot, cf_server_check_delay, cf_server_check_query,
    cf_server_connect_timeout, cf_server_fast_close, cf_server_idle_timeout, cf_shutdown,
    cf_suspend_timeout, cf_transaction_timeout, empty_server_check_query, first_socket,
    g_suspend_start, pgb_event_base, sockaddr_ucreds, C2RustUnnamed_9, CallbackState,
    LoadBalanceHosts, PacketCallbackFlag, PauseMode, PgAddr, PgCredentials, PgDatabase,
    PgGlobalUser, PgPool, PgSocket, PgStats, ReplicationType, ScramState, ShutDownMode,
    SocketState, CB_HANDLE_COMPLETE_PACKET, CB_NONE, CB_WANT_COMPLETE_PACKET, CL_ACTIVE,
    CL_ACTIVE_CANCEL, CL_FREE, CL_JUSTFREE, CL_LOGIN, CL_WAITING, CL_WAITING_CANCEL,
    CL_WAITING_LOGIN, LOAD_BALANCE_HOSTS_DISABLE, LOAD_BALANCE_HOSTS_ROUND_ROBIN, P_NONE, P_PAUSE,
    P_SUSPEND, REPLICATION_LOGICAL, REPLICATION_NONE, REPLICATION_PHYSICAL, SHUTDOWN_IMMEDIATE,
    SHUTDOWN_NONE, SHUTDOWN_WAIT_FOR_CLIENTS, SHUTDOWN_WAIT_FOR_SERVERS, SV_ACTIVE,
    SV_ACTIVE_CANCEL, SV_BEING_CANCELED, SV_FREE, SV_IDLE, SV_JUSTFREE, SV_LOGIN, SV_TESTED,
    SV_USED,
};
pub use self::cryptohash_h::{pg_cryptohash_type, PG_SHA224, PG_SHA256, PG_SHA384, PG_SHA512};
use self::dnslookup_h::adns_zone_cache_maint;
use self::errno_h::__error;
pub use self::event_h::{
    event_add, event_assign, event_base, event_base_loopbreak, event_callback_fn, EV_PERSIST,
};
pub use self::event_struct_h::{
    event, event_callback, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
};
pub use self::in6_h::{in6_addr, sockaddr_in6, C2RustUnnamed};
pub use self::in_h::{in_addr, sockaddr_in};
pub use self::iobuf_h::{iobuf, iobuf_empty, IOBuf};
pub use self::list_h::{list_append, list_del, list_empty, List};
pub use self::logging_h::{
    cf_verbose, log_generic, LogLevel, LG_DEBUG, LG_ERROR, LG_FATAL, LG_INFO, LG_NOISE, LG_STATS,
    LG_WARNING,
};
pub use self::mbuf_h::MBuf;
use self::objects_h::{
    activate_client, autodatabase_idle_list, change_server_state, credentials_cache, database_list,
    db_cache, disconnect_client, disconnect_server, get_active_client_count,
    get_active_server_count, get_pool, launch_new_connection, life_over, login_client_list,
    peer_cache, peer_list, peer_pool_cache, peer_pool_list, pool_cache, pool_list, release_server,
    var_list_cache,
};
pub use self::pktbuf_h::{
    pktbuf_dynamic, pktbuf_free, pktbuf_send_immediate, pktbuf_send_queued, pktbuf_static,
    pktbuf_write_generic, PktBuf,
};
use self::pooler_h::{cleanup_unix_sockets, resume_pooler};
pub use self::prepare_h::{
    PgClientPreparedStatement, PgPreparedStatement, PgServerPreparedStatement,
};
pub use self::proto_h::PktHdr;
pub use self::protocol_h::{PqMsg_NoticeResponse, PqMsg_Query};
pub use self::sbuf_h::{
    sbuf_cb_t, sbuf_continue, sbuf_is_empty, sbuf_pause, SBuf, SBufEvent, SBufIO,
    SBUF_EV_CONNECT_FAILED, SBUF_EV_CONNECT_OK, SBUF_EV_FLUSH, SBUF_EV_PKT_CALLBACK, SBUF_EV_READ,
    SBUF_EV_RECV_FAILED, SBUF_EV_SEND_FAILED, SBUF_EV_TLS_READY,
};
use self::server_h::{
    database_min_pool_size, pool_min_pool_size, pool_pool_size, pool_res_pool_size,
    pool_server_lifetime,
};
use self::slab_h::slab_free;
pub use self::socket_h::sockaddr;
pub use self::statlist_h::{
    statlist_append, statlist_count, statlist_empty, statlist_remove, StatList,
};
pub use self::stdbool_h::{false_0, true_0};
pub use self::strpool_h::{PStr, StrPool};
pub use self::sys__types_h::{__darwin_pid_t, __darwin_suseconds_t, __darwin_uid_t, __DARWIN_NULL};
pub use self::time_h::{get_cached_time, usec_t, USEC};

pub use self::uthash_h::{UT_hash_bucket, UT_hash_handle, UT_hash_table};
pub use self::varcache_h::{varcache_clean, VarCache};
#[c2rust::src_loc = "28:1"]
static mut full_maint_period: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
};
#[c2rust::src_loc = "29:1"]
static mut full_maint_ev: event = event {
    ev_evcallback: event_callback {
        evcb_active_next: C2RustUnnamed_8 {
            tqe_next: ::core::ptr::null::<event_callback>() as *mut event_callback,
            tqe_prev: ::core::ptr::null::<*mut event_callback>() as *mut *mut event_callback,
        },
        evcb_flags: 0,
        evcb_pri: 0,
        evcb_closure: 0,
        evcb_cb_union: C2RustUnnamed_7 {
            evcb_callback: None,
        },
        evcb_arg: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    },
    ev_timeout_pos: C2RustUnnamed_5 {
        ev_next_with_common_timeout: C2RustUnnamed_6 {
            tqe_next: ::core::ptr::null::<event>() as *mut event,
            tqe_prev: ::core::ptr::null::<*mut event>() as *mut *mut event,
        },
    },
    ev_fd: 0,
    ev_base: ::core::ptr::null::<event_base>() as *mut event_base,
    ev_: C2RustUnnamed_0 {
        ev_io: C2RustUnnamed_3 {
            ev_io_next: C2RustUnnamed_4 {
                le_next: ::core::ptr::null::<event>() as *mut event,
                le_prev: ::core::ptr::null::<*mut event>() as *mut *mut event,
            },
            ev_timeout: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        },
    },
    ev_events: 0,
    ev_res: 0,
    ev_timeout: timeval {
        tv_sec: 0,
        tv_usec: 0,
    },
};
#[c2rust::src_loc = "34:1"]
unsafe extern "C" fn close_server_list(
    mut sk_list: *mut StatList,
    mut reason: *const ::core::ffi::c_char,
) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut server = ::core::ptr::null_mut::<PgSocket>();
    item = (*sk_list).head.next;
    tmp = (*(*sk_list).head.next).next;
    while item != &raw mut (*sk_list).head {
        server = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgSocket;
        disconnect_server(
            server,
            true_0 != 0,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            reason,
        );
        item = tmp;
        tmp = (*tmp).next;
    }
}
#[c2rust::src_loc = "45:1"]
unsafe extern "C" fn close_client_list(
    mut sk_list: *mut StatList,
    mut reason: *const ::core::ffi::c_char,
) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut client = ::core::ptr::null_mut::<PgSocket>();
    item = (*sk_list).head.next;
    tmp = (*(*sk_list).head.next).next;
    while item != &raw mut (*sk_list).head {
        client = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgSocket;
        disconnect_client(
            client,
            true_0 != 0,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            reason,
        );
        item = tmp;
        tmp = (*tmp).next;
    }
}
#[no_mangle]
#[c2rust::src_loc = "56:1"]
pub unsafe extern "C" fn suspend_socket(mut sk: *mut PgSocket, mut force_suspend: bool) -> bool {
    if (*sk).suspended() {
        return true_0 != 0;
    }
    if sbuf_is_empty(&raw mut (*sk).sbuf) {
        if sbuf_pause(&raw mut (*sk).sbuf) {
            (*sk).set_suspended((true_0 != 0) as bool);
        }
    }
    if (*sk).suspended() as ::core::ffi::c_int != 0 || !force_suspend {
        return (*sk).suspended();
    }
    if (*sk).state() as ::core::ffi::c_int >= SV_FREE as ::core::ffi::c_int {
        disconnect_server(
            sk,
            true_0 != 0,
            b"suspend_timeout\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        disconnect_client(
            sk,
            true_0 != 0,
            b"suspend_timeout\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return true_0 != 0;
}
#[c2rust::src_loc = "77:1"]
unsafe extern "C" fn suspend_socket_list(
    mut list: *mut StatList,
    mut force_suspend: bool,
) -> ::core::ffi::c_int {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut sk = ::core::ptr::null_mut::<PgSocket>();
    let mut active = 0 as ::core::ffi::c_int;
    item = (*list).head.next;
    tmp = (*(*list).head.next).next;
    while item != &raw mut (*list).head {
        sk = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgSocket;
        if !suspend_socket(sk, force_suspend) {
            active += 1;
        }
        item = tmp;
        tmp = (*tmp).next;
    }
    return active;
}
#[c2rust::src_loc = "92:1"]
unsafe extern "C" fn resume_socket_list(mut list: *mut StatList) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut sk = ::core::ptr::null_mut::<PgSocket>();
    item = (*list).head.next;
    tmp = (*(*list).head.next).next;
    while item != &raw mut (*list).head {
        sk = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgSocket;
        if (*sk).suspended() {
            (*sk).set_suspended((false_0 != 0) as bool);
            sbuf_continue(&raw mut (*sk).sbuf);
        }
        item = tmp;
        tmp = (*tmp).next;
    }
}
#[c2rust::src_loc = "107:1"]
unsafe extern "C" fn resume_sockets() {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    item = pool_list.head.next;
    while item != &raw mut pool_list.head {
        pool = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgPool;
        if !(*(*pool).db).admin {
            resume_socket_list(&raw mut (*pool).active_client_list);
            resume_socket_list(&raw mut (*pool).active_server_list);
            resume_socket_list(&raw mut (*pool).idle_server_list);
            resume_socket_list(&raw mut (*pool).used_server_list);
        }
        item = (*item).next;
    }
}
#[no_mangle]
#[c2rust::src_loc = "124:1"]
pub unsafe extern "C" fn resume_all() {
    resume_sockets();
    resume_pooler();
}
#[c2rust::src_loc = "133:1"]
unsafe extern "C" fn launch_recheck(mut pool: *mut PgPool) {
    let mut q: *const ::core::ffi::c_char = cf_server_check_query;
    let mut need_check = true_0 != 0;
    let mut server = ::core::ptr::null_mut::<PgSocket>();
    let mut res = true_0 != 0;
    loop {
        server = first_socket(&raw mut (*pool).used_server_list);
        if server.is_null() {
            return;
        }
        if (*server).ready() {
            break;
        }
        disconnect_server(
            server,
            true_0 != 0,
            b"idle server got dirty\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if q.is_null()
        || *q.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        need_check = false_0 != 0;
    } else if cf_server_check_delay > 0 as usec_t {
        let mut now = get_cached_time();
        if now.wrapping_sub((*server).request_time) < cf_server_check_delay {
            need_check = false_0 != 0;
        }
    }
    if need_check {
        if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_DEBUG,
                server as *mut ::core::ffi::c_void,
                b"P: checking: %s\0" as *const u8 as *const ::core::ffi::c_char,
                q,
            );
        }
        change_server_state(server, SV_TESTED);
        if empty_server_check_query {
            let mut _data: [uint8_t; 512] = [0; 512];
            let mut _buf = PktBuf {
                buf: ::core::ptr::null_mut::<uint8_t>(),
                buf_len: 0,
                write_pos: 0,
                pktlen_pos: 0,
                send_pos: 0,
                ev: ::core::ptr::null_mut::<event>(),
                queued_dst: ::core::ptr::null_mut::<PgSocket>(),
                failed_sending_fixed_buf: [0; 1],
                c2rust_padding: [0; 7],
            };
            pktbuf_static(
                &raw mut _buf,
                &raw mut _data as *mut uint8_t,
                ::core::mem::size_of::<[uint8_t; 512]>() as ::core::ffi::c_int,
            );
            pktbuf_write_generic(
                &raw mut _buf,
                PqMsg_Query,
                b"s\0" as *const u8 as *const ::core::ffi::c_char,
                b"\0\0" as *const u8 as *const ::core::ffi::c_char,
            );
            res = pktbuf_send_immediate(&raw mut _buf, server);
        } else {
            let mut _data_0: [uint8_t; 512] = [0; 512];
            let mut _buf_0 = PktBuf {
                buf: ::core::ptr::null_mut::<uint8_t>(),
                buf_len: 0,
                write_pos: 0,
                pktlen_pos: 0,
                send_pos: 0,
                ev: ::core::ptr::null_mut::<event>(),
                queued_dst: ::core::ptr::null_mut::<PgSocket>(),
                failed_sending_fixed_buf: [0; 1],
                c2rust_padding: [0; 7],
            };
            pktbuf_static(
                &raw mut _buf_0,
                &raw mut _data_0 as *mut uint8_t,
                ::core::mem::size_of::<[uint8_t; 512]>() as ::core::ffi::c_int,
            );
            pktbuf_write_generic(
                &raw mut _buf_0,
                PqMsg_Query,
                b"s\0" as *const u8 as *const ::core::ffi::c_char,
                q,
            );
            res = pktbuf_send_immediate(&raw mut _buf_0, server);
        }
        if !res {
            disconnect_server(
                server,
                false_0 != 0,
                b"test query failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else {
        release_server(server);
    };
}
#[c2rust::src_loc = "178:1"]
unsafe extern "C" fn per_loop_activate(mut pool: *mut PgPool) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut client = ::core::ptr::null_mut::<PgSocket>();
    let mut sv_tested: ::core::ffi::c_int = 0;
    let mut sv_used: ::core::ffi::c_int = 0;
    if !statlist_empty(&raw mut (*pool).waiting_cancel_req_list) {
        launch_new_connection(pool, true_0 != 0);
        return;
    }
    sv_tested = statlist_count(&raw mut (*pool).tested_server_list);
    sv_used = statlist_count(&raw mut (*pool).used_server_list);
    item = (*pool).waiting_client_list.head.next;
    tmp = (*(*pool).waiting_client_list.head.next).next;
    while item != &raw mut (*pool).waiting_client_list.head {
        let mut buf = ::core::ptr::null_mut::<PktBuf>();
        let mut res: bool = false;
        client = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgSocket;
        if (*client).state() as ::core::ffi::c_int == CL_WAITING as ::core::ffi::c_int
            && !(*client).sent_wait_notification()
            && (*client).welcome_sent() as ::core::ffi::c_int != 0
            && get_cached_time()
                .wrapping_sub((*client).wait_start)
                .wrapping_div(USEC)
                > cf_query_wait_notify as usec_t
            && cf_query_wait_notify > 0 as ::core::ffi::c_ulong
        {
            buf = pktbuf_dynamic(256 as ::core::ffi::c_int);
            if buf.is_null() {
                let mut _log_ctx = NULL;
                log_generic(
                    LG_FATAL,
                    _log_ctx,
                    b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
                );
                exit(1 as ::core::ffi::c_int);
            }
            pktbuf_write_generic(
                buf,
                PqMsg_NoticeResponse,
                b"sscss\0" as *const u8 as *const ::core::ffi::c_char,
                b"SNOTICE\0" as *const u8 as *const ::core::ffi::c_char,
                b"C00000\0" as *const u8 as *const ::core::ffi::c_char,
                'M' as i32,
                b"No server connection available in postgres backend, client being queued\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
            );
            res = pktbuf_send_queued(buf, client);
            if !res {
                let mut _log_ctx_0 = NULL;
                log_generic(
                    LG_WARNING,
                    _log_ctx_0,
                    b"Sending queue warning failed\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            (*client).set_sent_wait_notification((true_0 != 0) as bool);
        }
        if (*client).replication as u64 != 0 {
            launch_new_connection(pool, true_0 != 0);
        } else if !statlist_empty(&raw mut (*pool).idle_server_list) {
            if (*client).wait_for_welcome() as ::core::ffi::c_int != 0
                && !(*pool).welcome_msg_ready()
            {
                launch_new_connection(pool, true_0 != 0);
            } else {
                activate_client(client);
            }
        } else if sv_tested > 0 as ::core::ffi::c_int {
            sv_tested -= 1;
        } else if sv_used > 0 as ::core::ffi::c_int {
            launch_recheck(pool);
            sv_used -= 1;
        } else {
            launch_new_connection(pool, true_0 != 0);
            break;
        }
        item = tmp;
        tmp = (*tmp).next;
    }
}
#[c2rust::src_loc = "251:1"]
unsafe extern "C" fn per_loop_pause(mut pool: *mut PgPool) -> ::core::ffi::c_int {
    let mut active = 0 as ::core::ffi::c_int;
    if (*(*pool).db).admin {
        return 0 as ::core::ffi::c_int;
    }
    close_server_list(
        &raw mut (*pool).idle_server_list,
        b"pause mode\0" as *const u8 as *const ::core::ffi::c_char,
    );
    close_server_list(
        &raw mut (*pool).used_server_list,
        b"pause mode\0" as *const u8 as *const ::core::ffi::c_char,
    );
    close_server_list(
        &raw mut (*pool).new_server_list,
        b"pause mode\0" as *const u8 as *const ::core::ffi::c_char,
    );
    active += statlist_count(&raw mut (*pool).active_server_list);
    active += statlist_count(&raw mut (*pool).tested_server_list);
    return active;
}
#[c2rust::src_loc = "271:1"]
unsafe extern "C" fn per_loop_suspend(
    mut pool: *mut PgPool,
    mut force_suspend: bool,
) -> ::core::ffi::c_int {
    let mut active = 0 as ::core::ffi::c_int;
    if (*(*pool).db).admin {
        return 0 as ::core::ffi::c_int;
    }
    active += suspend_socket_list(&raw mut (*pool).active_client_list, force_suspend);
    active += suspend_socket_list(&raw mut (*pool).waiting_client_list, force_suspend);
    if active != 0 {
        per_loop_activate(pool);
    }
    if active == 0 {
        active += suspend_socket_list(&raw mut (*pool).active_server_list, force_suspend);
        active += suspend_socket_list(&raw mut (*pool).idle_server_list, force_suspend);
        close_server_list(
            &raw mut (*pool).tested_server_list,
            b"close unsafe file descriptors on suspend\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        close_server_list(
            &raw mut (*pool).used_server_list,
            b"close unsafe file descriptors on suspend\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    return active;
}
#[c2rust::src_loc = "300:1"]
unsafe extern "C" fn count_close_needed(mut server_list: *mut StatList) -> ::core::ffi::c_int {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut server = ::core::ptr::null_mut::<PgSocket>();
    let mut count = 0 as ::core::ffi::c_int;
    item = (*server_list).head.next;
    while item != &raw mut (*server_list).head {
        server = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgSocket;
        if (*server).close_needed() {
            count += 1;
        }
        item = (*item).next;
    }
    return count;
}
#[c2rust::src_loc = "318:1"]
unsafe extern "C" fn per_loop_wait_close(mut pool: *mut PgPool) -> ::core::ffi::c_int {
    let mut count = 0 as ::core::ffi::c_int;
    if (*(*pool).db).admin {
        return 0 as ::core::ffi::c_int;
    }
    count += count_close_needed(&raw mut (*pool).active_server_list);
    count += count_close_needed(&raw mut (*pool).idle_server_list);
    count += count_close_needed(&raw mut (*pool).new_server_list);
    count += count_close_needed(&raw mut (*pool).tested_server_list);
    count += count_close_needed(&raw mut (*pool).used_server_list);
    return count;
}
#[no_mangle]
#[c2rust::src_loc = "337:1"]
pub unsafe extern "C" fn per_loop_maint() {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut active_count = 0 as ::core::ffi::c_int;
    let mut waiting_count = 0 as ::core::ffi::c_int;
    let mut partial_pause = false_0 != 0;
    let mut partial_wait = false_0 != 0;
    let mut force_suspend = false_0 != 0;
    if cf_pause_mode == P_SUSPEND as ::core::ffi::c_int && cf_suspend_timeout > 0 as usec_t {
        let mut stime: usec_t = get_cached_time().wrapping_sub(g_suspend_start);
        if stime >= cf_suspend_timeout {
            force_suspend = true_0 != 0;
        }
    }
    item = pool_list.head.next;
    while item != &raw mut pool_list.head {
        pool = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgPool;
        if !(*(*pool).db).admin {
            match cf_pause_mode {
                0 => {
                    if (*(*pool).db).db_paused {
                        partial_pause = true_0 != 0;
                        active_count += per_loop_pause(pool);
                    } else {
                        per_loop_activate(pool);
                    }
                }
                1 => {
                    active_count += per_loop_pause(pool);
                }
                2 => {
                    active_count += per_loop_suspend(pool, force_suspend);
                }
                _ => {}
            }
            if (*(*pool).db).db_wait_close {
                partial_wait = true_0 != 0;
                waiting_count += per_loop_wait_close(pool);
            }
        }
        item = (*item).next;
    }
    let mut current_block_28: u64;
    match cf_pause_mode {
        2 => {
            if force_suspend {
                close_client_list(
                    &raw mut login_client_list,
                    b"suspend_timeout\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
                active_count += statlist_count(&raw mut login_client_list);
            }
            current_block_28 = 13345507216710712890;
        }
        1 => {
            current_block_28 = 13345507216710712890;
        }
        0 => {
            if partial_pause as ::core::ffi::c_int != 0 && active_count == 0 {
                admin_pause_done();
            }
            current_block_28 = 17500079516916021833;
        }
        _ => {
            current_block_28 = 17500079516916021833;
        }
    }
    match current_block_28 {
        13345507216710712890 => {
            if active_count == 0 {
                admin_pause_done();
            }
        }
        _ => {}
    }
    if partial_wait as ::core::ffi::c_int != 0 && waiting_count == 0 {
        admin_wait_close_done();
    }
}
#[c2rust::src_loc = "403:1"]
unsafe extern "C" fn pool_client_maint(mut pool: *mut PgPool) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut now = get_cached_time();
    let mut client = ::core::ptr::null_mut::<PgSocket>();
    let mut user = ::core::ptr::null_mut::<PgGlobalUser>();
    let mut age: usec_t = 0;
    let mut effective_client_idle_timeout: usec_t = 0;
    if cf_client_idle_timeout > 0 as usec_t
        || any_user_level_client_timeout_set as ::core::ffi::c_int != 0
    {
        item = (*pool).active_client_list.head.next;
        tmp = (*(*pool).active_client_list.head.next).next;
        while item != &raw mut (*pool).active_client_list.head {
            client = (item as *mut ::core::ffi::c_char)
                .offset(-(0 as ::core::ffi::c_ulong as isize))
                as *mut PgSocket;
            if (*client).link.is_null() {
                user = (*(*client).login_user_credentials).global_user;
                effective_client_idle_timeout = cf_client_idle_timeout;
                if (*user).client_idle_timeout > 0 as usec_t {
                    effective_client_idle_timeout = (*user).client_idle_timeout;
                }
                if now.wrapping_sub((*client).request_time) > effective_client_idle_timeout {
                    disconnect_client(
                        client,
                        true_0 != 0,
                        b"client_idle_timeout\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            }
            item = tmp;
            tmp = (*tmp).next;
        }
    }
    if cf_query_timeout > 0 as usec_t || cf_query_wait_timeout > 0 as usec_t {
        item = (*pool).waiting_client_list.head.next;
        tmp = (*(*pool).waiting_client_list.head.next).next;
        while item != &raw mut (*pool).waiting_client_list.head {
            client = (item as *mut ::core::ffi::c_char)
                .offset(-(0 as ::core::ffi::c_ulong as isize))
                as *mut PgSocket;
            if (*client).query_start == 0 as usec_t {
                age = now.wrapping_sub((*client).request_time);
            } else {
                age = now.wrapping_sub((*client).query_start);
            }
            if cf_shutdown == SHUTDOWN_WAIT_FOR_SERVERS as ::core::ffi::c_int {
                disconnect_client(
                    client,
                    true_0 != 0,
                    b"server shutting down\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else if cf_query_timeout > 0 as usec_t && age > cf_query_timeout {
                disconnect_client(
                    client,
                    true_0 != 0,
                    b"query_timeout\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else if cf_query_wait_timeout > 0 as usec_t && age > cf_query_wait_timeout {
                disconnect_client(
                    client,
                    true_0 != 0,
                    b"query_wait_timeout\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            item = tmp;
            tmp = (*tmp).next;
        }
    }
    if cf_cancel_wait_timeout > 0 as usec_t {
        item = (*pool).waiting_cancel_req_list.head.next;
        tmp = (*(*pool).waiting_cancel_req_list.head.next).next;
        while item != &raw mut (*pool).waiting_cancel_req_list.head {
            client = (item as *mut ::core::ffi::c_char)
                .offset(-(0 as ::core::ffi::c_ulong as isize))
                as *mut PgSocket;
            age = now.wrapping_sub((*client).request_time);
            if age > cf_cancel_wait_timeout {
                disconnect_client(
                    client,
                    false_0 != 0,
                    b"cancel_wait_timeout\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            item = tmp;
            tmp = (*tmp).next;
        }
    }
    if cf_client_login_timeout > 0 as usec_t && !(*pool).welcome_msg_ready() {
        item = (*pool).waiting_client_list.head.next;
        tmp = (*(*pool).waiting_client_list.head.next).next;
        while item != &raw mut (*pool).waiting_client_list.head {
            client = (item as *mut ::core::ffi::c_char)
                .offset(-(0 as ::core::ffi::c_ulong as isize))
                as *mut PgSocket;
            if (*client).wait_for_welcome() {
                age = now.wrapping_sub((*client).connect_time);
                if age > cf_client_login_timeout {
                    disconnect_client(
                        client,
                        true_0 != 0,
                        b"client_login_timeout (server down)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            }
            item = tmp;
            tmp = (*tmp).next;
        }
    }
}
#[c2rust::src_loc = "480:1"]
unsafe extern "C" fn peer_pool_client_maint(mut pool: *mut PgPool) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut now = get_cached_time();
    let mut client = ::core::ptr::null_mut::<PgSocket>();
    let mut age: usec_t = 0;
    if cf_cancel_wait_timeout > 0 as usec_t {
        item = (*pool).waiting_cancel_req_list.head.next;
        tmp = (*(*pool).waiting_cancel_req_list.head.next).next;
        while item != &raw mut (*pool).waiting_cancel_req_list.head {
            client = (item as *mut ::core::ffi::c_char)
                .offset(-(0 as ::core::ffi::c_ulong as isize))
                as *mut PgSocket;
            age = now.wrapping_sub((*client).request_time);
            if age > cf_cancel_wait_timeout {
                disconnect_client(
                    client,
                    false_0 != 0,
                    b"cancel_wait_timeout\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            item = tmp;
            tmp = (*tmp).next;
        }
    }
}
#[c2rust::src_loc = "499:1"]
unsafe extern "C" fn check_unused_servers(
    mut pool: *mut PgPool,
    mut slist: *mut StatList,
    mut idle_test: bool,
) {
    let mut now = get_cached_time();
    let mut server_lifetime = pool_server_lifetime(pool);
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut idle: usec_t = 0;
    let mut age: usec_t = 0;
    let mut server = ::core::ptr::null_mut::<PgSocket>();
    item = (*slist).head.next;
    tmp = (*(*slist).head.next).next;
    while item != &raw mut (*slist).head {
        server = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgSocket;
        age = now.wrapping_sub((*server).connect_time);
        idle = now.wrapping_sub((*server).request_time);
        if (*server).close_needed() {
            disconnect_server(
                server,
                true_0 != 0,
                b"database configuration changed\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else if (*server).state() as ::core::ffi::c_int == SV_IDLE as ::core::ffi::c_int
            && !(*server).ready()
        {
            disconnect_server(
                server,
                true_0 != 0,
                b"SV_IDLE server got dirty\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else if (*server).state() as ::core::ffi::c_int == SV_USED as ::core::ffi::c_int
            && !(*server).ready()
        {
            disconnect_server(
                server,
                true_0 != 0,
                b"SV_USED server got dirty\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else if cf_server_idle_timeout > 0 as usec_t
            && idle > cf_server_idle_timeout
            && (pool_min_pool_size(pool) == 0 as ::core::ffi::c_int
                || statlist_count(&raw mut (*pool).active_server_list)
                    + statlist_count(&raw mut (*pool).being_canceled_server_list)
                    + statlist_count(&raw mut (*pool).idle_server_list)
                    + statlist_count(&raw mut (*pool).tested_server_list)
                    + statlist_count(&raw mut (*pool).used_server_list)
                    > pool_min_pool_size(pool))
        {
            disconnect_server(
                server,
                true_0 != 0,
                b"server idle timeout\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else if age >= server_lifetime {
            if life_over(server) {
                disconnect_server(
                    server,
                    true_0 != 0,
                    b"server lifetime over\0" as *const u8 as *const ::core::ffi::c_char,
                );
                (*pool).last_lifetime_disconnect = now;
            }
        } else if cf_pause_mode == P_PAUSE as ::core::ffi::c_int {
            disconnect_server(
                server,
                true_0 != 0,
                b"pause mode\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else if idle_test as ::core::ffi::c_int != 0
            && *cf_server_check_query as ::core::ffi::c_int != 0
        {
            if idle > cf_server_check_delay {
                change_server_state(server, SV_USED);
            }
        }
        item = tmp;
        tmp = (*tmp).next;
    }
}
#[c2rust::src_loc = "542:1"]
unsafe extern "C" fn check_pool_size(mut pool: *mut PgPool) {
    let mut server = ::core::ptr::null_mut::<PgSocket>();
    let mut cur = statlist_count(&raw mut (*pool).active_server_list)
        + statlist_count(&raw mut (*pool).being_canceled_server_list)
        + statlist_count(&raw mut (*pool).idle_server_list)
        + statlist_count(&raw mut (*pool).tested_server_list)
        + statlist_count(&raw mut (*pool).used_server_list);
    let mut many = cur - (pool_pool_size(pool) + pool_res_pool_size(pool));
    if pool_pool_size(pool) > 0 as ::core::ffi::c_int {
        while many > 0 as ::core::ffi::c_int {
            server = first_socket(&raw mut (*pool).used_server_list);
            if server.is_null() {
                server = first_socket(&raw mut (*pool).idle_server_list);
            }
            if server.is_null() {
                break;
            }
            disconnect_server(
                server,
                true_0 != 0,
                b"too many servers in the pool\0" as *const u8 as *const ::core::ffi::c_char,
            );
            many -= 1;
            cur -= 1;
        }
    }
    if cur < pool_min_pool_size(pool)
        && cur < pool_pool_size(pool)
        && cf_pause_mode == P_NONE as ::core::ffi::c_int
        && cf_reboot == 0 as ::core::ffi::c_int
        && (statlist_count(&raw mut (*pool).active_client_list)
            + statlist_count(&raw mut (*pool).waiting_client_list)
            > 0 as ::core::ffi::c_int
            || !(*(*pool).db).forced_user_credentials.is_null())
    {
        let mut _log_ctx = NULL;
        if (cf_verbose > 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0
        {
            log_generic(
                LG_DEBUG,
                _log_ctx,
                b"launching new connection to satisfy min_pool_size\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        launch_new_connection(pool, false_0 != 0);
    }
}
#[c2rust::src_loc = "575:1"]
unsafe extern "C" fn pool_server_maint(mut pool: *mut PgPool) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut now = get_cached_time();
    let mut server = ::core::ptr::null_mut::<PgSocket>();
    check_unused_servers(
        pool,
        &raw mut (*pool).used_server_list,
        0 as ::core::ffi::c_int != 0,
    );
    check_unused_servers(
        pool,
        &raw mut (*pool).tested_server_list,
        0 as ::core::ffi::c_int != 0,
    );
    check_unused_servers(
        pool,
        &raw mut (*pool).idle_server_list,
        1 as ::core::ffi::c_int != 0,
    );
    item = (*pool).active_server_list.head.next;
    tmp = (*(*pool).active_server_list.head.next).next;
    while item != &raw mut (*pool).active_server_list.head {
        server = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgSocket;
        if cf_server_fast_close != 0
            && (*server).ready() as ::core::ffi::c_int != 0
            && (*server).close_needed() as ::core::ffi::c_int != 0
        {
            disconnect_server(
                server,
                true_0 != 0,
                b"database configuration changed\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if (*server).replication as ::core::ffi::c_uint != 0
            && (*server).close_needed() as ::core::ffi::c_int != 0
        {
            disconnect_server(
                server,
                true_0 != 0,
                b"database configuration changed\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        item = tmp;
        tmp = (*tmp).next;
    }
    if cf_query_timeout > 0 as usec_t
        || cf_idle_transaction_timeout > 0 as usec_t
        || cf_transaction_timeout > 0 as usec_t
        || any_user_level_timeout_set as ::core::ffi::c_int != 0
    {
        item = (*pool).active_server_list.head.next;
        tmp = (*(*pool).active_server_list.head.next).next;
        while item != &raw mut (*pool).active_server_list.head {
            let mut age_client: usec_t = 0;
            let mut age_server: usec_t = 0;
            let mut age_transaction: usec_t = 0;
            let mut effective_query_timeout: usec_t = 0;
            let mut effective_idle_transaction_timeout: usec_t = 0;
            let mut user_query_timeout: usec_t = 0;
            let mut user_idle_transaction_timeout: usec_t = 0;
            let mut user_transaction_timeout: usec_t = 0;
            let mut effective_transaction_timeout: usec_t = 0;
            server = (item as *mut ::core::ffi::c_char)
                .offset(-(0 as ::core::ffi::c_ulong as isize))
                as *mut PgSocket;
            if !(*server).ready() {
                age_client = now.wrapping_sub((*(*server).link).request_time);
                age_server = now.wrapping_sub((*server).request_time);
                age_transaction = now.wrapping_sub((*(*server).link).xact_start);
                user_idle_transaction_timeout =
                    (*(*(*server).login_user_credentials).global_user).idle_transaction_timeout;
                user_transaction_timeout =
                    (*(*(*server).login_user_credentials).global_user).transaction_timeout;
                user_query_timeout =
                    (*(*(*server).login_user_credentials).global_user).query_timeout;
                effective_idle_transaction_timeout = cf_idle_transaction_timeout;
                effective_query_timeout = cf_query_timeout;
                effective_transaction_timeout = cf_transaction_timeout;
                if user_idle_transaction_timeout > 0 as usec_t {
                    effective_idle_transaction_timeout = user_idle_transaction_timeout;
                }
                if user_query_timeout > 0 as usec_t {
                    effective_query_timeout = user_query_timeout;
                }
                if user_transaction_timeout > 0 as usec_t {
                    effective_transaction_timeout = user_transaction_timeout;
                }
                if effective_query_timeout > 0 as usec_t && age_client > effective_query_timeout {
                    disconnect_server(
                        server,
                        true_0 != 0,
                        b"query timeout\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else if effective_idle_transaction_timeout > 0 as usec_t
                    && (*server).idle_tx() as ::core::ffi::c_int != 0
                    && age_server > effective_idle_transaction_timeout
                {
                    disconnect_server(
                        server,
                        true_0 != 0,
                        b"idle transaction timeout\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else if effective_transaction_timeout > 0 as usec_t
                    && age_transaction > effective_transaction_timeout
                {
                    disconnect_server(
                        server,
                        true_0 != 0,
                        b"transaction timeout\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            }
            item = tmp;
            tmp = (*tmp).next;
        }
    }
    if cf_server_connect_timeout > 0 as usec_t {
        item = (*pool).new_server_list.head.next;
        tmp = (*(*pool).new_server_list.head.next).next;
        while item != &raw mut (*pool).new_server_list.head {
            let mut age: usec_t = 0;
            server = (item as *mut ::core::ffi::c_char)
                .offset(-(0 as ::core::ffi::c_ulong as isize))
                as *mut PgSocket;
            age = now.wrapping_sub((*server).connect_time);
            if age > cf_server_connect_timeout {
                disconnect_server(
                    server,
                    true_0 != 0,
                    b"connect timeout\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            item = tmp;
            tmp = (*tmp).next;
        }
    }
    check_pool_size(pool);
}
#[c2rust::src_loc = "682:1"]
unsafe extern "C" fn peer_pool_server_maint(mut pool: *mut PgPool) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut now = get_cached_time();
    let mut server = ::core::ptr::null_mut::<PgSocket>();
    if cf_server_connect_timeout > 0 as usec_t || cf_cancel_wait_timeout > 0 as usec_t {
        item = (*pool).new_server_list.head.next;
        tmp = (*(*pool).new_server_list.head.next).next;
        while item != &raw mut (*pool).new_server_list.head {
            let mut age: usec_t = 0;
            server = (item as *mut ::core::ffi::c_char)
                .offset(-(0 as ::core::ffi::c_ulong as isize))
                as *mut PgSocket;
            age = now.wrapping_sub((*server).connect_time);
            if cf_server_connect_timeout > 0 as usec_t && age > cf_server_connect_timeout {
                disconnect_server(
                    server,
                    true_0 != 0,
                    b"connect timeout\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else if cf_cancel_wait_timeout > 0 as usec_t && age > cf_cancel_wait_timeout {
                disconnect_server(
                    server,
                    true_0 != 0,
                    b"cancel_wait_timeout\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            item = tmp;
            tmp = (*tmp).next;
        }
    }
}
#[c2rust::src_loc = "712:1"]
unsafe extern "C" fn cleanup_client_logins() {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut client = ::core::ptr::null_mut::<PgSocket>();
    let mut age: usec_t = 0;
    let mut now = get_cached_time();
    if cf_client_login_timeout <= 0 as usec_t {
        return;
    }
    item = login_client_list.head.next;
    tmp = (*login_client_list.head.next).next;
    while item != &raw mut login_client_list.head {
        client = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgSocket;
        age = now.wrapping_sub((*client).connect_time);
        if age > cf_client_login_timeout {
            disconnect_client(
                client,
                true_0 != 0,
                b"client_login_timeout\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        item = tmp;
        tmp = (*tmp).next;
    }
}
#[c2rust::src_loc = "730:1"]
unsafe extern "C" fn cleanup_inactive_autodatabases() {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut db = ::core::ptr::null_mut::<PgDatabase>();
    let mut age: usec_t = 0;
    let mut now = get_cached_time();
    if cf_autodb_idle_timeout <= 0 as usec_t {
        return;
    }
    item = autodatabase_idle_list.head.next;
    tmp = (*autodatabase_idle_list.head.next).next;
    while item != &raw mut autodatabase_idle_list.head {
        db = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgDatabase;
        if !(*db).db_paused {
            age = now.wrapping_sub((*db).inactive_time);
            if !(age > cf_autodb_idle_timeout) {
                break;
            }
            kill_database(db);
        }
        item = tmp;
        tmp = (*tmp).next;
    }
}
#[c2rust::src_loc = "755:1"]
unsafe extern "C" fn do_full_maint(
    mut _sock: ::core::ffi::c_int,
    mut _flags: ::core::ffi::c_short,
    mut _arg: *mut ::core::ffi::c_void,
) {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut db = ::core::ptr::null_mut::<PgDatabase>();
    static mut seq: ::core::ffi::c_uint = 0;
    seq = seq.wrapping_add(1);
    if cf_pause_mode == P_SUSPEND as ::core::ffi::c_int {
        return;
    }
    item = database_list.head.next;
    tmp = (*database_list.head.next).next;
    while item != &raw mut database_list.head {
        db = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgDatabase;
        if database_min_pool_size(db) > 0 as ::core::ffi::c_int
            && !(*db).forced_user_credentials.is_null()
        {
            get_pool(db, (*db).forced_user_credentials);
        }
        item = tmp;
        tmp = (*tmp).next;
    }
    item = pool_list.head.next;
    tmp = (*pool_list.head.next).next;
    while item != &raw mut pool_list.head {
        pool = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgPool;
        if !(*(*pool).db).admin {
            pool_server_maint(pool);
            pool_client_maint(pool);
            if (*(*pool).db).db_auto as ::core::ffi::c_int != 0
                && (*(*pool).db).inactive_time == 0 as usec_t
            {
                if statlist_count(&raw mut (*pool).active_client_list)
                    + statlist_count(&raw mut (*pool).waiting_client_list)
                    > 0 as ::core::ffi::c_int
                    || statlist_count(&raw mut (*pool).active_server_list)
                        + statlist_count(&raw mut (*pool).being_canceled_server_list)
                        + statlist_count(&raw mut (*pool).idle_server_list)
                        + statlist_count(&raw mut (*pool).tested_server_list)
                        + statlist_count(&raw mut (*pool).used_server_list)
                        + statlist_count(&raw mut (*pool).new_server_list)
                        + statlist_count(&raw mut (*pool).active_cancel_server_list)
                        > 0 as ::core::ffi::c_int
                {
                    (*(*pool).db).active_stamp = seq;
                }
            }
        }
        item = tmp;
        tmp = (*tmp).next;
    }
    item = peer_pool_list.head.next;
    tmp = (*peer_pool_list.head.next).next;
    while item != &raw mut peer_pool_list.head {
        pool = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgPool;
        peer_pool_server_maint(pool);
        peer_pool_client_maint(pool);
        item = tmp;
        tmp = (*tmp).next;
    }
    item = database_list.head.next;
    tmp = (*database_list.head.next).next;
    while item != &raw mut database_list.head {
        db = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgDatabase;
        if (*db).db_auto as ::core::ffi::c_int != 0 && (*db).inactive_time == 0 as usec_t {
            if !((*db).active_stamp == seq) {
                (*db).inactive_time = get_cached_time();
                statlist_remove(&raw mut database_list, &raw mut (*db).head);
                statlist_append(&raw mut autodatabase_idle_list, &raw mut (*db).head);
            }
        }
        item = tmp;
        tmp = (*tmp).next;
    }
    cleanup_inactive_autodatabases();
    cleanup_client_logins();
    if cf_shutdown == SHUTDOWN_WAIT_FOR_SERVERS as ::core::ffi::c_int
        && get_active_server_count() == 0 as ::core::ffi::c_int
    {
        let mut _log_ctx = NULL;
        log_generic(
            LG_INFO,
            _log_ctx,
            b"server connections dropped, exiting\0" as *const u8 as *const ::core::ffi::c_char,
        );
        cf_shutdown = SHUTDOWN_IMMEDIATE as ::core::ffi::c_int;
        cleanup_unix_sockets();
        event_base_loopbreak(pgb_event_base);
        return;
    }
    if cf_shutdown == SHUTDOWN_WAIT_FOR_CLIENTS as ::core::ffi::c_int
        && get_active_client_count() == 0 as ::core::ffi::c_int
    {
        let mut _log_ctx_0 = NULL;
        log_generic(
            LG_INFO,
            _log_ctx_0,
            b"client connections dropped, exiting\0" as *const u8 as *const ::core::ffi::c_char,
        );
        cf_shutdown = SHUTDOWN_IMMEDIATE as ::core::ffi::c_int;
        cleanup_unix_sockets();
        event_base_loopbreak(pgb_event_base);
        return;
    }
    adns_zone_cache_maint(adns);
}
#[no_mangle]
#[c2rust::src_loc = "845:1"]
pub unsafe extern "C" fn janitor_setup() {
    event_assign(
        &raw mut full_maint_ev,
        pgb_event_base,
        -(1 as ::core::ffi::c_int),
        EV_PERSIST as ::core::ffi::c_short,
        Some(
            do_full_maint
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_short,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        NULL,
    );
    if event_add(&raw mut full_maint_ev, &raw mut full_maint_period) < 0 as ::core::ffi::c_int {
        let mut _log_ctx = NULL;
        log_generic(
            LG_WARNING,
            _log_ctx,
            b"event_add failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
            strerror(*__error()),
        );
    }
}
#[no_mangle]
#[c2rust::src_loc = "853:1"]
pub unsafe extern "C" fn kill_pool(mut pool: *mut PgPool) {
    let mut reason = b"database removed\0" as *const u8 as *const ::core::ffi::c_char;
    close_client_list(&raw mut (*pool).active_client_list, reason);
    close_client_list(&raw mut (*pool).waiting_client_list, reason);
    close_client_list(&raw mut (*pool).active_cancel_req_list, reason);
    close_client_list(&raw mut (*pool).waiting_cancel_req_list, reason);
    close_server_list(&raw mut (*pool).active_server_list, reason);
    close_server_list(&raw mut (*pool).active_cancel_server_list, reason);
    close_server_list(&raw mut (*pool).being_canceled_server_list, reason);
    close_server_list(&raw mut (*pool).idle_server_list, reason);
    close_server_list(&raw mut (*pool).used_server_list, reason);
    close_server_list(&raw mut (*pool).tested_server_list, reason);
    close_server_list(&raw mut (*pool).new_server_list, reason);
    pktbuf_free((*pool).welcome_msg as *mut PktBuf);
    list_del(&raw mut (*pool).map_head);
    statlist_remove(&raw mut pool_list, &raw mut (*pool).head);
    varcache_clean(&raw mut (*pool).orig_vars);
    slab_free(
        var_list_cache,
        (*pool).orig_vars.var_list as *mut ::core::ffi::c_void,
    );
    slab_free(pool_cache, pool as *mut ::core::ffi::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "880:1"]
pub unsafe extern "C" fn kill_peer_pool(mut pool: *mut PgPool) {
    let mut reason = b"peer removed\0" as *const u8 as *const ::core::ffi::c_char;
    close_client_list(&raw mut (*pool).active_cancel_req_list, reason);
    close_client_list(&raw mut (*pool).waiting_cancel_req_list, reason);
    close_server_list(&raw mut (*pool).active_cancel_server_list, reason);
    close_server_list(&raw mut (*pool).new_server_list, reason);
    pktbuf_free((*pool).welcome_msg as *mut PktBuf);
    list_del(&raw mut (*pool).map_head);
    statlist_remove(&raw mut peer_pool_list, &raw mut (*pool).head);
    varcache_clean(&raw mut (*pool).orig_vars);
    slab_free(
        var_list_cache,
        (*pool).orig_vars.var_list as *mut ::core::ffi::c_void,
    );
    slab_free(peer_pool_cache, pool as *mut ::core::ffi::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "899:1"]
pub unsafe extern "C" fn kill_database(mut db: *mut PgDatabase) {
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut _log_ctx = NULL;
    log_generic(
        LG_WARNING,
        _log_ctx,
        b"dropping database '%s' as it does not exist anymore or inactive auto-database\0"
            as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*db).name as *mut ::core::ffi::c_char,
    );
    item = pool_list.head.next;
    tmp = (*pool_list.head.next).next;
    while item != &raw mut pool_list.head {
        pool = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgPool;
        if (*pool).db == db {
            kill_pool(pool);
        }
        item = tmp;
        tmp = (*tmp).next;
    }
    pktbuf_free((*db).startup_params as *mut PktBuf);
    free((*db).host as *mut ::core::ffi::c_void);
    if !(*db).forced_user_credentials.is_null() {
        slab_free(
            credentials_cache,
            (*db).forced_user_credentials as *mut ::core::ffi::c_void,
        );
    }
    free((*db).connect_query as *mut ::core::ffi::c_void);
    if (*db).inactive_time != 0 {
        statlist_remove(&raw mut autodatabase_idle_list, &raw mut (*db).head);
    } else {
        statlist_remove(&raw mut database_list, &raw mut (*db).head);
    }
    if !(*db).auth_dbname.is_null() {
        free((*db).auth_dbname as *mut ::core::ffi::c_void);
    }
    if !(*db).auth_query.is_null() {
        free((*db).auth_query as *mut ::core::ffi::c_void);
    }
    clear_user_tree_cached_scram_keys(&raw mut (*db).user_tree);
    aatree_destroy(&raw mut (*db).user_tree);
    slab_free(db_cache, db as *mut ::core::ffi::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "936:1"]
pub unsafe extern "C" fn kill_peer(mut db: *mut PgDatabase) {
    let mut pool = ::core::ptr::null_mut::<PgPool>();
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut _log_ctx = NULL;
    log_generic(
        LG_WARNING,
        _log_ctx,
        b"dropping peer %s as it does not exist anymore\0" as *const u8
            as *const ::core::ffi::c_char,
        &raw mut (*db).name as *mut ::core::ffi::c_char,
    );
    item = peer_pool_list.head.next;
    tmp = (*peer_pool_list.head.next).next;
    while item != &raw mut peer_pool_list.head {
        pool = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgPool;
        if (*pool).db == db {
            kill_peer_pool(pool);
        }
        item = tmp;
        tmp = (*tmp).next;
    }
    free((*db).host as *mut ::core::ffi::c_void);
    statlist_remove(&raw mut peer_list, &raw mut (*db).head);
    slab_free(peer_cache, db as *mut ::core::ffi::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "957:1"]
pub unsafe extern "C" fn config_postprocess() {
    let mut item = ::core::ptr::null_mut::<List>();
    let mut tmp = ::core::ptr::null_mut::<List>();
    let mut db = ::core::ptr::null_mut::<PgDatabase>();
    item = database_list.head.next;
    tmp = (*database_list.head.next).next;
    while item != &raw mut database_list.head {
        db = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgDatabase;
        if (*db).db_dead {
            kill_database(db);
        }
        item = tmp;
        tmp = (*tmp).next;
    }
    item = peer_list.head.next;
    tmp = (*peer_list.head.next).next;
    while item != &raw mut peer_list.head {
        db = (item as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut PgDatabase;
        if (*db).db_dead {
            kill_peer(db);
        }
        item = tmp;
        tmp = (*tmp).next;
    }
}
#[c2rust::src_loc = "979:1"]
unsafe extern "C" fn clean_cached_scram(mut n: *mut AANode, mut _arg: *mut ::core::ffi::c_void) {
    let mut user = (n as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
        as *mut PgCredentials;
    if !(*user).scram_SaltKey.is_null() {
        free((*user).scram_SaltKey as *mut ::core::ffi::c_void);
        (*user).scram_SaltKey = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*user).adhoc_scram_secrets_cached = false_0 != 0;
    }
}
#[no_mangle]
#[c2rust::src_loc = "989:1"]
pub unsafe extern "C" fn clear_user_tree_cached_scram_keys(mut tree: *mut AATree) {
    aatree_walk(
        tree,
        AA_WALK_IN_ORDER,
        Some(
            clean_cached_scram as unsafe extern "C" fn(*mut AANode, *mut ::core::ffi::c_void) -> (),
        ),
        NULL,
    );
}
unsafe extern "C" fn run_static_initializers() {
    full_maint_period = timeval {
        tv_sec: 0 as __darwin_time_t,
        tv_usec: USEC.wrapping_div(3 as usec_t) as __darwin_suseconds_t,
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
