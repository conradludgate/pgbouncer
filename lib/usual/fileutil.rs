#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h:19"]
pub mod _types_h {
    #[c2rust::src_loc = "34:1"]
    pub type __uint16_t = u16;
    #[c2rust::src_loc = "35:1"]
    pub type __int32_t = i32;
    #[c2rust::src_loc = "36:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "37:1"]
    pub type __int64_t = i64;
    #[c2rust::src_loc = "38:1"]
    pub type __uint64_t = u64;
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_size_t = usize;
    #[c2rust::src_loc = "118:1"]
    pub type __darwin_ssize_t = isize;
    #[c2rust::src_loc = "119:1"]
    pub type __darwin_time_t = ::core::ffi::c_long;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h:19"]
pub mod sys__types_h {
    #[c2rust::src_loc = "67:1"]
    pub type __darwin_blkcnt_t = __int64_t;
    #[c2rust::src_loc = "68:1"]
    pub type __darwin_blksize_t = __int32_t;
    #[c2rust::src_loc = "69:1"]
    pub type __darwin_dev_t = __int32_t;
    #[c2rust::src_loc = "72:1"]
    pub type __darwin_gid_t = __uint32_t;
    #[c2rust::src_loc = "74:1"]
    pub type __darwin_ino64_t = __uint64_t;
    #[c2rust::src_loc = "82:1"]
    pub type __darwin_mode_t = __uint16_t;
    #[c2rust::src_loc = "83:1"]
    pub type __darwin_off_t = __int64_t;
    #[c2rust::src_loc = "87:1"]
    pub type __darwin_uid_t = __uint32_t;
    #[c2rust::src_loc = "64:9"]
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    use super::_types_h::{__int32_t, __int64_t, __uint16_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_dev_t.h:19"]
pub mod _dev_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type dev_t = __darwin_dev_t;
    use super::sys__types_h::__darwin_dev_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_blkcnt_t.h:19"]
pub mod _blkcnt_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type blkcnt_t = __darwin_blkcnt_t;
    use super::sys__types_h::__darwin_blkcnt_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_blksize_t.h:19"]
pub mod _blksize_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type blksize_t = __darwin_blksize_t;
    use super::sys__types_h::__darwin_blksize_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_gid_t.h:19"]
pub mod _gid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type gid_t = __darwin_gid_t;
    use super::sys__types_h::__darwin_gid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_mode_t.h:19"]
pub mod _mode_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type mode_t = __darwin_mode_t;
    use super::sys__types_h::__darwin_mode_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_nlink_t.h:19"]
pub mod _nlink_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type nlink_t = __uint16_t;
    use super::_types_h::__uint16_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_off_t.h:19"]
pub mod _off_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type off_t = __darwin_off_t;
    use super::sys__types_h::__darwin_off_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uid_t.h:19"]
pub mod _uid_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type uid_t = __darwin_uid_t;
    use super::sys__types_h::__darwin_uid_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:19"]
pub mod _size_t_h {
    #[c2rust::src_loc = "50:1"]
    pub type size_t = __darwin_size_t;
    use super::_types_h::__darwin_size_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ssize_t.h:19"]
pub mod _ssize_t_h {
    #[c2rust::src_loc = "31:1"]
    pub type ssize_t = __darwin_ssize_t;
    use super::_types_h::__darwin_ssize_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h:19"]
pub mod _stdio_h {
    #[c2rust::src_loc = "86:1"]
    pub type fpos_t = __darwin_off_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "97:1"]
    pub struct __sbuf {
        pub _base: *mut ::core::ffi::c_uchar,
        pub _size: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "131:9"]
    pub struct __sFILE {
        pub _p: *mut ::core::ffi::c_uchar,
        pub _r: ::core::ffi::c_int,
        pub _w: ::core::ffi::c_int,
        pub _flags: ::core::ffi::c_short,
        pub _file: ::core::ffi::c_short,
        pub _bf: __sbuf,
        pub _lbfsize: ::core::ffi::c_int,
        pub _cookie: *mut ::core::ffi::c_void,
        pub _close: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
        pub _read: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub _seek: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void, fpos_t, ::core::ffi::c_int) -> fpos_t,
        >,
        pub _write: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub _ub: __sbuf,
        pub _extra: *mut __sFILEX,
        pub _ur: ::core::ffi::c_int,
        pub _ubuf: [::core::ffi::c_uchar; 3],
        pub _nbuf: [::core::ffi::c_uchar; 1],
        pub _lb: __sbuf,
        pub _blksize: ::core::ffi::c_int,
        pub _offset: fpos_t,
    }
    #[c2rust::src_loc = "131:1"]
    pub type FILE = __sFILE;
    use super::_size_t_h::size_t;
    use super::_ssize_t_h::ssize_t;
    use super::sys__types_h::__darwin_off_t;
    extern "C" {
        #[c2rust::src_loc = "103:1"]
        pub type __sFILEX;
        #[c2rust::src_loc = "233:1"]
        pub fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "234:1"]
        pub fn feof(_: *mut FILE) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "243:1"]
        pub fn fopen(
            __filename: *const ::core::ffi::c_char,
            __mode: *const ::core::ffi::c_char,
        ) -> *mut FILE;
        #[c2rust::src_loc = "248:1"]
        pub fn fread(
            __ptr: *mut ::core::ffi::c_void,
            __size: size_t,
            __nitems: size_t,
            __stream: *mut FILE,
        ) -> ::core::ffi::c_ulong;
        #[c2rust::src_loc = "320:1"]
        pub fn fileno(_: *mut FILE) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "456:1"]
        pub fn getline(
            __linep: *mut *mut ::core::ffi::c_char,
            __linecapp: *mut size_t,
            __stream: *mut FILE,
        ) -> ssize_t;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_timespec.h:19"]
pub mod _timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:1"]
    pub struct timespec {
        pub tv_sec: __darwin_time_t,
        pub tv_nsec: ::core::ffi::c_long,
    }
    use super::_types_h::__darwin_time_t;
}
#[c2rust::header_src = "/Users/conrad.ludgate/Documents/code/pgbouncer/lib/usual/fileutil.h:19"]
pub mod fileutil_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:1"]
    pub struct MappedFile {
        pub fd: ::core::ffi::c_int,
        pub len: ::core::ffi::c_uint,
        pub ptr: *mut ::core::ffi::c_void,
    }
    #[c2rust::src_loc = "37:1"]
    pub type procline_cb = Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ssize_t) -> bool,
    >;
    use super::_ssize_t_h::ssize_t;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/stat.h:25"]
pub mod stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "182:1"]
    pub struct stat {
        pub st_dev: dev_t,
        pub st_mode: mode_t,
        pub st_nlink: nlink_t,
        pub st_ino: __darwin_ino64_t,
        pub st_uid: uid_t,
        pub st_gid: gid_t,
        pub st_rdev: dev_t,
        pub st_atimespec: timespec,
        pub st_mtimespec: timespec,
        pub st_ctimespec: timespec,
        pub st_birthtimespec: timespec,
        pub st_size: off_t,
        pub st_blocks: blkcnt_t,
        pub st_blksize: blksize_t,
        pub st_flags: __uint32_t,
        pub st_gen: __uint32_t,
        pub st_lspare: __int32_t,
        pub st_qspare: [__int64_t; 2],
    }
    use super::_blkcnt_t_h::blkcnt_t;
    use super::_blksize_t_h::blksize_t;
    use super::_dev_t_h::dev_t;
    use super::_gid_t_h::gid_t;
    use super::_mode_t_h::mode_t;
    use super::_nlink_t_h::nlink_t;
    use super::_off_t_h::off_t;
    use super::_timespec_h::timespec;
    use super::_types_h::{__int32_t, __int64_t, __uint32_t};
    use super::_uid_t_h::uid_t;
    use super::sys__types_h::__darwin_ino64_t;
    extern "C" {
        #[c2rust::src_loc = "383:1"]
        pub fn fstat(_: ::core::ffi::c_int, _: *mut stat) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "387:1"]
        pub fn stat(_: *const ::core::ffi::c_char, _: *mut stat) -> ::core::ffi::c_int;
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
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h:19"]
pub mod _null_h {
    #[c2rust::src_loc = "49:9"]
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::sys__types_h::__DARWIN_NULL;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/mman.h:22"]
pub mod mman_h {
    #[c2rust::src_loc = "100:9"]
    pub const PROT_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "101:9"]
    pub const PROT_WRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "108:9"]
    pub const MAP_SHARED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "177:9"]
    pub const MAP_FAILED: *mut ::core::ffi::c_void =
        -(1 as ::core::ffi::c_int) as *mut ::core::ffi::c_void;
    use super::_off_t_h::off_t;
    use super::_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "249:1"]
        pub fn mmap(
            _: *mut ::core::ffi::c_void,
            _: size_t,
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_int,
            _: ::core::ffi::c_int,
            _: off_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "258:1"]
        pub fn munmap(_: *mut ::core::ffi::c_void, _: size_t) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/errno.h:19"]
pub mod errno_h {
    extern "C" {
        #[c2rust::src_loc = "80:1"]
        pub fn __error() -> *mut ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/unistd.h:19"]
pub mod unistd_h {
    extern "C" {
        #[c2rust::src_loc = "441:1"]
        pub fn close(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/opt/homebrew/Cellar/llvm/21.1.8/lib/clang/21/include/stdbool.h:19"]
pub mod stdbool_h {
    #[c2rust::src_loc = "25:9"]
    pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/fcntl.h:26"]
pub mod fcntl_h {
    #[c2rust::src_loc = "96:9"]
    pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "98:9"]
    pub const O_RDWR: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    extern "C" {
        #[c2rust::src_loc = "602:1"]
        pub fn open(
            _: *const ::core::ffi::c_char,
            _: ::core::ffi::c_int,
            ...
        ) -> ::core::ffi::c_int;
    }
}
pub use self::_blkcnt_t_h::blkcnt_t;
pub use self::_blksize_t_h::blksize_t;
pub use self::_dev_t_h::dev_t;
pub use self::_gid_t_h::gid_t;
use self::_malloc_h::{free, malloc};
pub use self::_mode_t_h::mode_t;
pub use self::_nlink_t_h::nlink_t;
pub use self::_null_h::NULL;
pub use self::_off_t_h::off_t;
pub use self::_size_t_h::size_t;
pub use self::_ssize_t_h::ssize_t;
pub use self::_stdio_h::{
    __sFILE, __sFILEX, __sbuf, fclose, feof, fileno, fopen, fpos_t, fread, getline, FILE,
};
pub use self::_timespec_h::timespec;
pub use self::_types_h::{
    __darwin_size_t, __darwin_ssize_t, __darwin_time_t, __int32_t, __int64_t, __uint16_t,
    __uint32_t, __uint64_t,
};
pub use self::_uid_t_h::uid_t;
use self::errno_h::__error;
pub use self::fcntl_h::{open, O_RDONLY, O_RDWR};
pub use self::fileutil_h::{procline_cb, MappedFile};
pub use self::mman_h::{mmap, munmap, MAP_FAILED, MAP_SHARED, PROT_READ, PROT_WRITE};
pub use self::stat_h::{fstat, stat};
pub use self::stdbool_h::{false_0, true_0};
pub use self::sys__types_h::{
    __darwin_blkcnt_t, __darwin_blksize_t, __darwin_dev_t, __darwin_gid_t, __darwin_ino64_t,
    __darwin_mode_t, __darwin_off_t, __darwin_uid_t, __DARWIN_NULL,
};
use self::unistd_h::close;
#[no_mangle]
#[c2rust::src_loc = "33:1"]
pub unsafe extern "C" fn load_file(
    mut fn_0: *const ::core::ffi::c_char,
    mut len_p: *mut size_t,
) -> *mut ::core::ffi::c_void {
    let mut st = stat {
        st_dev: 0,
        st_mode: 0,
        st_nlink: 0,
        st_ino: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_atimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_birthtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_size: 0,
        st_blocks: 0,
        st_blksize: 0,
        st_flags: 0,
        st_gen: 0,
        st_lspare: 0,
        st_qspare: [0; 2],
    };
    let mut buf = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut res: ::core::ffi::c_int = 0;
    let mut f = ::core::ptr::null_mut::<FILE>();
    let mut save_errno: ::core::ffi::c_int = 0;
    f = fopen(fn_0, b"r\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if f.is_null() {
        return NULL;
    }
    res = fstat(fileno(f), &raw mut st);
    if res < 0 as ::core::ffi::c_int {
        save_errno = *__error();
        fclose(f);
        *__error() = save_errno;
        return NULL;
    }
    buf = malloc((st.st_size + 1 as off_t) as size_t) as *mut ::core::ffi::c_char;
    if buf.is_null() {
        save_errno = *__error();
        fclose(f);
        *__error() = save_errno;
        return NULL;
    }
    res = fread(
        buf as *mut ::core::ffi::c_void,
        1 as size_t,
        st.st_size as size_t,
        f,
    ) as ::core::ffi::c_int;
    if res < 0 as ::core::ffi::c_int {
        save_errno = *__error();
        free(buf as *mut ::core::ffi::c_void);
        fclose(f);
        *__error() = save_errno;
        return NULL;
    }
    fclose(f);
    *buf.offset(res as isize) = 0 as ::core::ffi::c_char;
    if !len_p.is_null() {
        *len_p = res as size_t;
    }
    return buf as *mut ::core::ffi::c_void;
}
#[no_mangle]
#[c2rust::src_loc = "81:1"]
pub unsafe extern "C" fn foreach_line(
    mut fn_0: *const ::core::ffi::c_char,
    mut proc_line: procline_cb,
    mut arg: *mut ::core::ffi::c_void,
) -> bool {
    let mut ln = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: size_t = 0 as size_t;
    let mut res: ssize_t = 0;
    let mut f = fopen(fn_0, b"rb\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    let mut ok = false_0 != 0;
    if f.is_null() {
        return false_0 != 0;
    }
    loop {
        res = getline(&raw mut ln, &raw mut len, f);
        if res < 0 as ssize_t {
            if feof(f) != 0 {
                ok = true_0 != 0;
            }
            break;
        } else if !proc_line.expect("non-null function pointer")(arg, ln, res) {
            break;
        }
    }
    fclose(f);
    free(ln as *mut ::core::ffi::c_void);
    return ok;
}
#[no_mangle]
#[c2rust::src_loc = "109:1"]
pub unsafe extern "C" fn file_size(mut fn_0: *const ::core::ffi::c_char) -> ssize_t {
    let mut st = stat {
        st_dev: 0,
        st_mode: 0,
        st_nlink: 0,
        st_ino: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_atimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_birthtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_size: 0,
        st_blocks: 0,
        st_blksize: 0,
        st_flags: 0,
        st_gen: 0,
        st_lspare: 0,
        st_qspare: [0; 2],
    };
    if stat(fn_0, &raw mut st) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int) as ssize_t;
    }
    return st.st_size as ssize_t;
}
#[no_mangle]
#[c2rust::src_loc = "123:1"]
pub unsafe extern "C" fn map_file(
    mut m: *mut MappedFile,
    mut fname: *const ::core::ffi::c_char,
    mut rw: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut st = stat {
        st_dev: 0,
        st_mode: 0,
        st_nlink: 0,
        st_ino: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_atimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_birthtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_size: 0,
        st_blocks: 0,
        st_blksize: 0,
        st_flags: 0,
        st_gen: 0,
        st_lspare: 0,
        st_qspare: [0; 2],
    };
    (*m).fd = open(fname, if rw != 0 { O_RDWR } else { O_RDONLY });
    if (*m).fd < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if fstat((*m).fd, &raw mut st) < 0 as ::core::ffi::c_int {
        close((*m).fd);
        return -(1 as ::core::ffi::c_int);
    }
    (*m).len = st.st_size as ::core::ffi::c_uint;
    (*m).ptr = mmap(
        NULL,
        (*m).len as size_t,
        PROT_READ
            | (if rw != 0 {
                PROT_WRITE
            } else {
                0 as ::core::ffi::c_int
            }),
        MAP_SHARED,
        (*m).fd,
        0 as off_t,
    );
    if (*m).ptr == MAP_FAILED {
        close((*m).fd);
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "143:1"]
pub unsafe extern "C" fn unmap_file(mut m: *mut MappedFile) {
    munmap((*m).ptr, (*m).len as size_t);
    close((*m).fd);
    (*m).ptr = NULL;
    (*m).fd = 0 as ::core::ffi::c_int;
}
