//! Memory buffer utilities
//!
//! Consolidated mbuf inline functions from c2rust headers.

use crate::types::{
    false_0, free, memchr, memcpy, memset, realloc, size_t, true_0, uint16_t, uint32_t, uint64_t,
    uint8_t, MBuf,
};

/// Ensure buffer has room for additional `len` bytes.
/// Returns false if buffer is read-only, fixed, or allocation fails.
#[no_mangle]
pub unsafe extern "C" fn mbuf_make_room(buf: *mut MBuf, len: ::core::ffi::c_uint) -> bool {
    let mut new_alloc = (*buf).alloc_len;
    if (*buf).reader as ::core::ffi::c_int != 0 || (*buf).fixed as ::core::ffi::c_int != 0 {
        return false_0 != 0;
    }
    if (*buf).write_pos.wrapping_add(len) <= (*buf).alloc_len {
        return true_0 != 0;
    }
    if new_alloc == 0 as ::core::ffi::c_uint {
        new_alloc = 128 as ::core::ffi::c_uint;
    }
    while new_alloc < (*buf).write_pos.wrapping_add(len) {
        new_alloc = new_alloc.wrapping_mul(2 as ::core::ffi::c_uint);
    }
    let ptr = realloc((*buf).data as *mut ::core::ffi::c_void, new_alloc as size_t);
    if ptr.is_null() {
        return false_0 != 0;
    }
    (*buf).data = ptr as *mut uint8_t;
    (*buf).alloc_len = new_alloc;
    true_0 != 0
}

/// Initialize buffer as a fixed-size writer (no dynamic allocation).
#[inline]
pub unsafe fn mbuf_init_fixed_writer(
    buf: *mut MBuf,
    ptr: *mut ::core::ffi::c_void,
    len: ::core::ffi::c_uint,
) {
    (*buf).data = ptr as *mut uint8_t;
    (*buf).read_pos = 0 as ::core::ffi::c_uint;
    (*buf).write_pos = 0 as ::core::ffi::c_uint;
    (*buf).alloc_len = len;
    (*buf).reader = false_0 != 0;
    (*buf).fixed = true_0 != 0;
}

/// Initialize buffer as a fixed-size reader.
#[inline]
pub unsafe fn mbuf_init_fixed_reader(
    buf: *mut MBuf,
    ptr: *const ::core::ffi::c_void,
    len: ::core::ffi::c_uint,
) {
    (*buf).data = ptr as *mut uint8_t;
    (*buf).read_pos = 0 as ::core::ffi::c_uint;
    (*buf).write_pos = len;
    (*buf).alloc_len = len;
    (*buf).reader = true_0 != 0;
    (*buf).fixed = true_0 != 0;
}

/// Initialize buffer for dynamic allocation.
#[inline]
pub unsafe fn mbuf_init_dynamic(buf: *mut MBuf) {
    (*buf).data = ::core::ptr::null_mut::<uint8_t>();
    (*buf).read_pos = 0 as ::core::ffi::c_uint;
    (*buf).write_pos = 0 as ::core::ffi::c_uint;
    (*buf).alloc_len = 0 as ::core::ffi::c_uint;
    (*buf).reader = false_0 != 0;
    (*buf).fixed = false_0 != 0;
}

/// Free buffer data (if not fixed) and zero the struct.
#[inline]
pub unsafe fn mbuf_free(buf: *mut MBuf) {
    if !(*buf).data.is_null() {
        if !(*buf).fixed {
            free((*buf).data as *mut ::core::ffi::c_void);
        }
        memset(
            buf as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<MBuf>() as size_t,
        );
    }
}

/// Reset read position to start.
#[inline]
pub unsafe fn mbuf_rewind_reader(buf: *mut MBuf) {
    (*buf).read_pos = 0 as ::core::ffi::c_uint;
}

/// Reset both read and write positions (for writers only).
#[inline]
pub unsafe fn mbuf_rewind_writer(buf: *mut MBuf) {
    if !(*buf).reader {
        (*buf).read_pos = 0 as ::core::ffi::c_uint;
        (*buf).write_pos = 0 as ::core::ffi::c_uint;
    }
}

/// Copy buffer metadata (not a deep copy of data).
#[inline]
pub unsafe fn mbuf_copy(src: *const MBuf, dst: *mut MBuf) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<MBuf>() as size_t,
    );
}

/// Slice an existing buffer (creates a read-only view).
#[inline]
pub unsafe fn mbuf_slice(src: *mut MBuf, len: ::core::ffi::c_uint, dst: *mut MBuf) -> bool {
    if (*src).read_pos.wrapping_add(len) > (*src).write_pos {
        return false_0 != 0;
    }
    (*dst).data = (*src).data.offset((*src).read_pos as isize);
    (*dst).read_pos = 0 as ::core::ffi::c_uint;
    (*dst).write_pos = len;
    (*dst).alloc_len = len;
    (*dst).reader = true_0 != 0;
    (*dst).fixed = true_0 != 0;
    (*src).read_pos = (*src).read_pos.wrapping_add(len);
    true_0 != 0
}

/// Return bytes available for reading.
#[inline]
pub unsafe fn mbuf_avail_for_read(buf: *const MBuf) -> ::core::ffi::c_uint {
    (*buf).write_pos.wrapping_sub((*buf).read_pos)
}

/// Return total bytes written.
#[inline]
pub unsafe fn mbuf_written(buf: *const MBuf) -> ::core::ffi::c_uint {
    (*buf).write_pos
}

/// Read a big-endian u16.
#[inline]
pub unsafe fn mbuf_get_uint16be(buf: *mut MBuf, dst_p: *mut uint16_t) -> bool {
    if (*buf).read_pos.wrapping_add(2 as ::core::ffi::c_uint) > (*buf).write_pos {
        return false_0 != 0;
    }
    let fresh0 = (*buf).read_pos;
    (*buf).read_pos = (*buf).read_pos.wrapping_add(1);
    let a = *(*buf).data.offset(fresh0 as isize) as ::core::ffi::c_uint;
    let fresh1 = (*buf).read_pos;
    (*buf).read_pos = (*buf).read_pos.wrapping_add(1);
    let b = *(*buf).data.offset(fresh1 as isize) as ::core::ffi::c_uint;
    *dst_p = (a << 8 as ::core::ffi::c_int | b) as uint16_t;
    true_0 != 0
}

/// Read a big-endian u32.
#[inline]
pub unsafe fn mbuf_get_uint32be(buf: *mut MBuf, dst_p: *mut uint32_t) -> bool {
    if (*buf).read_pos.wrapping_add(4 as ::core::ffi::c_uint) > (*buf).write_pos {
        return false_0 != 0;
    }
    let fresh2 = (*buf).read_pos;
    (*buf).read_pos = (*buf).read_pos.wrapping_add(1);
    let a = *(*buf).data.offset(fresh2 as isize) as ::core::ffi::c_uint;
    let fresh3 = (*buf).read_pos;
    (*buf).read_pos = (*buf).read_pos.wrapping_add(1);
    let b = *(*buf).data.offset(fresh3 as isize) as ::core::ffi::c_uint;
    let fresh4 = (*buf).read_pos;
    (*buf).read_pos = (*buf).read_pos.wrapping_add(1);
    let c = *(*buf).data.offset(fresh4 as isize) as ::core::ffi::c_uint;
    let fresh5 = (*buf).read_pos;
    (*buf).read_pos = (*buf).read_pos.wrapping_add(1);
    let d = *(*buf).data.offset(fresh5 as isize) as ::core::ffi::c_uint;
    *dst_p = (a << 24 as ::core::ffi::c_int
        | b << 16 as ::core::ffi::c_int
        | c << 8 as ::core::ffi::c_int
        | d) as uint32_t;
    true_0 != 0
}

/// Read a big-endian u64.
#[inline]
pub unsafe fn mbuf_get_uint64be(buf: *mut MBuf, dst_p: *mut uint64_t) -> bool {
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    if !mbuf_get_uint32be(buf, &raw mut a) || !mbuf_get_uint32be(buf, &raw mut b) {
        return false_0 != 0;
    }
    *dst_p = (a as uint64_t) << 32 as ::core::ffi::c_int | b as uint64_t;
    true_0 != 0
}

/// Read a single byte.
#[inline]
pub unsafe fn mbuf_get_byte(buf: *mut MBuf, dst_p: *mut uint8_t) -> bool {
    if (*buf).read_pos >= (*buf).write_pos {
        return false_0 != 0;
    }
    let fresh = (*buf).read_pos;
    (*buf).read_pos = (*buf).read_pos.wrapping_add(1);
    *dst_p = *(*buf).data.offset(fresh as isize);
    true_0 != 0
}

/// Read a single char.
#[inline]
pub unsafe fn mbuf_get_char(buf: *mut MBuf, dst_p: *mut ::core::ffi::c_char) -> bool {
    if (*buf).read_pos.wrapping_add(1 as ::core::ffi::c_uint) > (*buf).write_pos {
        return false_0 != 0;
    }
    let fresh = (*buf).read_pos;
    (*buf).read_pos = (*buf).read_pos.wrapping_add(1);
    *dst_p = *(*buf).data.offset(fresh as isize) as ::core::ffi::c_char;
    true_0 != 0
}

/// Read raw bytes, returning pointer to data in buffer.
#[inline]
pub unsafe fn mbuf_get_bytes(
    buf: *mut MBuf,
    len: ::core::ffi::c_uint,
    dst_p: *mut *const uint8_t,
) -> bool {
    if (*buf).read_pos.wrapping_add(len) > (*buf).write_pos {
        return false_0 != 0;
    }
    *dst_p = (*buf).data.offset((*buf).read_pos as isize);
    (*buf).read_pos = (*buf).read_pos.wrapping_add(len);
    true_0 != 0
}

/// Read chars (same as bytes but char pointer).
#[inline]
pub unsafe fn mbuf_get_chars(
    buf: *mut MBuf,
    len: ::core::ffi::c_uint,
    dst_p: *mut *const ::core::ffi::c_char,
) -> bool {
    if (*buf).read_pos.wrapping_add(len) > (*buf).write_pos {
        return false_0 != 0;
    }
    *dst_p = ((*buf).data as *mut ::core::ffi::c_char).offset((*buf).read_pos as isize);
    (*buf).read_pos = (*buf).read_pos.wrapping_add(len);
    true_0 != 0
}

/// Read null-terminated string.
#[inline]
pub unsafe fn mbuf_get_string(buf: *mut MBuf, dst_p: *mut *const ::core::ffi::c_char) -> bool {
    let res: *const ::core::ffi::c_char =
        ((*buf).data as *mut ::core::ffi::c_char).offset((*buf).read_pos as isize);
    let nul = memchr(
        res as *const ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        mbuf_avail_for_read(buf) as size_t,
    ) as *const uint8_t;
    if nul.is_null() {
        return false_0 != 0;
    }
    *dst_p = res;
    (*buf).read_pos =
        nul.offset(1 as ::core::ffi::c_int as isize)
            .offset_from((*buf).data) as ::core::ffi::c_long as ::core::ffi::c_uint;
    true_0 != 0
}

/// Write a single byte.
#[inline]
pub unsafe fn mbuf_write_byte(buf: *mut MBuf, val: uint8_t) -> bool {
    if (*buf).write_pos.wrapping_add(1 as ::core::ffi::c_uint) > (*buf).alloc_len
        && !mbuf_make_room(buf, 1 as ::core::ffi::c_uint)
    {
        return false_0 != 0;
    }
    let fresh6 = (*buf).write_pos;
    (*buf).write_pos = (*buf).write_pos.wrapping_add(1);
    *(*buf).data.offset(fresh6 as isize) = val;
    true_0 != 0
}

/// Write raw bytes.
#[inline]
pub unsafe fn mbuf_write(
    buf: *mut MBuf,
    ptr: *const ::core::ffi::c_void,
    len: ::core::ffi::c_uint,
) -> bool {
    if (*buf).write_pos.wrapping_add(len) > (*buf).alloc_len && !mbuf_make_room(buf, len) {
        return false_0 != 0;
    }
    if len > 0 as ::core::ffi::c_uint {
        memcpy(
            (*buf).data.offset((*buf).write_pos as isize) as *mut ::core::ffi::c_void,
            ptr,
            len as size_t,
        );
    }
    (*buf).write_pos = (*buf).write_pos.wrapping_add(len);
    true_0 != 0
}

/// Write contents of another mbuf (raw, not advancing src read pos).
#[inline]
pub unsafe fn mbuf_write_raw_mbuf(dst: *mut MBuf, src: *mut MBuf) -> bool {
    mbuf_write(
        dst,
        (*src).data as *const ::core::ffi::c_void,
        (*src).write_pos,
    )
}
