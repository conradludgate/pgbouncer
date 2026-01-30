//! Base64 encoding/decoding for PostgreSQL
//!
//! Refactored to use shared types module.

use super::types::{int8_t, memset, size_t, uint32_t, uint8_t};

#[c2rust::src_loc = "23:1"]
static mut _base64: [::core::ffi::c_char; 65] = unsafe {
    ::core::mem::transmute::<[u8; 65], [::core::ffi::c_char; 65]>(
        *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\0",
    )
};

#[c2rust::src_loc = "26:1"]
static mut b64lookup: [int8_t; 128] = [
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    62 as ::core::ffi::c_int as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    63 as ::core::ffi::c_int as int8_t,
    52 as ::core::ffi::c_int as int8_t,
    53 as ::core::ffi::c_int as int8_t,
    54 as ::core::ffi::c_int as int8_t,
    55 as ::core::ffi::c_int as int8_t,
    56 as ::core::ffi::c_int as int8_t,
    57 as ::core::ffi::c_int as int8_t,
    58 as ::core::ffi::c_int as int8_t,
    59 as ::core::ffi::c_int as int8_t,
    60 as ::core::ffi::c_int as int8_t,
    61 as ::core::ffi::c_int as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    0 as ::core::ffi::c_int as int8_t,
    1 as ::core::ffi::c_int as int8_t,
    2 as ::core::ffi::c_int as int8_t,
    3 as ::core::ffi::c_int as int8_t,
    4 as ::core::ffi::c_int as int8_t,
    5 as ::core::ffi::c_int as int8_t,
    6 as ::core::ffi::c_int as int8_t,
    7 as ::core::ffi::c_int as int8_t,
    8 as ::core::ffi::c_int as int8_t,
    9 as ::core::ffi::c_int as int8_t,
    10 as ::core::ffi::c_int as int8_t,
    11 as ::core::ffi::c_int as int8_t,
    12 as ::core::ffi::c_int as int8_t,
    13 as ::core::ffi::c_int as int8_t,
    14 as ::core::ffi::c_int as int8_t,
    15 as ::core::ffi::c_int as int8_t,
    16 as ::core::ffi::c_int as int8_t,
    17 as ::core::ffi::c_int as int8_t,
    18 as ::core::ffi::c_int as int8_t,
    19 as ::core::ffi::c_int as int8_t,
    20 as ::core::ffi::c_int as int8_t,
    21 as ::core::ffi::c_int as int8_t,
    22 as ::core::ffi::c_int as int8_t,
    23 as ::core::ffi::c_int as int8_t,
    24 as ::core::ffi::c_int as int8_t,
    25 as ::core::ffi::c_int as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    26 as ::core::ffi::c_int as int8_t,
    27 as ::core::ffi::c_int as int8_t,
    28 as ::core::ffi::c_int as int8_t,
    29 as ::core::ffi::c_int as int8_t,
    30 as ::core::ffi::c_int as int8_t,
    31 as ::core::ffi::c_int as int8_t,
    32 as ::core::ffi::c_int as int8_t,
    33 as ::core::ffi::c_int as int8_t,
    34 as ::core::ffi::c_int as int8_t,
    35 as ::core::ffi::c_int as int8_t,
    36 as ::core::ffi::c_int as int8_t,
    37 as ::core::ffi::c_int as int8_t,
    38 as ::core::ffi::c_int as int8_t,
    39 as ::core::ffi::c_int as int8_t,
    40 as ::core::ffi::c_int as int8_t,
    41 as ::core::ffi::c_int as int8_t,
    42 as ::core::ffi::c_int as int8_t,
    43 as ::core::ffi::c_int as int8_t,
    44 as ::core::ffi::c_int as int8_t,
    45 as ::core::ffi::c_int as int8_t,
    46 as ::core::ffi::c_int as int8_t,
    47 as ::core::ffi::c_int as int8_t,
    48 as ::core::ffi::c_int as int8_t,
    49 as ::core::ffi::c_int as int8_t,
    50 as ::core::ffi::c_int as int8_t,
    51 as ::core::ffi::c_int as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
    -(1 as ::core::ffi::c_int) as int8_t,
];

#[no_mangle]
#[c2rust::src_loc = "44:1"]
pub unsafe extern "C" fn pg_b64_encode(
    mut src: *const uint8_t,
    mut len: ::core::ffi::c_int,
    mut dst: *mut ::core::ffi::c_char,
    mut dstlen: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut p = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s = ::core::ptr::null::<uint8_t>();
    let mut end = src.offset(len as isize);
    let mut pos = 2 as ::core::ffi::c_int;
    let mut buf: uint32_t = 0 as uint32_t;
    s = src;
    p = dst;
    loop {
        if s >= end {
            current_block = 7651349459974463963;
            break;
        }
        buf |= ((*s as ::core::ffi::c_int) << (pos << 3 as ::core::ffi::c_int)) as uint32_t;
        pos -= 1;
        s = s.offset(1);
        if pos >= 0 as ::core::ffi::c_int {
            continue;
        }
        if p.offset_from(dst) as ::core::ffi::c_long + 4 as ::core::ffi::c_long
            > dstlen as ::core::ffi::c_long
        {
            current_block = 5042550662117864494;
            break;
        }
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = _base64[(buf >> 18 as ::core::ffi::c_int & 0x3f as uint32_t) as usize];
        let fresh1 = p;
        p = p.offset(1);
        *fresh1 = _base64[(buf >> 12 as ::core::ffi::c_int & 0x3f as uint32_t) as usize];
        let fresh2 = p;
        p = p.offset(1);
        *fresh2 = _base64[(buf >> 6 as ::core::ffi::c_int & 0x3f as uint32_t) as usize];
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = _base64[(buf & 0x3f as uint32_t) as usize];
        pos = 2 as ::core::ffi::c_int;
        buf = 0 as uint32_t;
    }
    if current_block == 7651349459974463963 {
        if pos != 2 as ::core::ffi::c_int {
            if p.offset_from(dst) as ::core::ffi::c_long + 4 as ::core::ffi::c_long
                > dstlen as ::core::ffi::c_long
            {
                current_block = 5042550662117864494;
            } else {
                let fresh4 = p;
                p = p.offset(1);
                *fresh4 = _base64[(buf >> 18 as ::core::ffi::c_int & 0x3f as uint32_t) as usize];
                let fresh5 = p;
                p = p.offset(1);
                *fresh5 = _base64[(buf >> 12 as ::core::ffi::c_int & 0x3f as uint32_t) as usize];
                let fresh6 = p;
                p = p.offset(1);
                *fresh6 = (if pos == 0 as ::core::ffi::c_int {
                    _base64[(buf >> 6 as ::core::ffi::c_int & 0x3f as uint32_t) as usize]
                        as ::core::ffi::c_int
                } else {
                    '=' as i32
                }) as ::core::ffi::c_char;
                let fresh7 = p;
                p = p.offset(1);
                *fresh7 = '=' as i32 as ::core::ffi::c_char;
                current_block = 15652330335145281839;
            }
        } else {
            current_block = 15652330335145281839;
        }
        match current_block {
            5042550662117864494 => {}
            _ => {
                return p.offset_from(dst) as ::core::ffi::c_long as ::core::ffi::c_int;
            }
        }
    }
    memset(
        dst as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        dstlen as size_t,
    );
    -(1 as ::core::ffi::c_int)
}

#[no_mangle]
#[c2rust::src_loc = "111:1"]
pub unsafe extern "C" fn pg_b64_decode(
    mut src: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dstlen: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut srcend = src.offset(len as isize);
    let mut s = src;
    let mut p = dst;
    let mut c: ::core::ffi::c_char = 0;
    let mut b = 0 as ::core::ffi::c_int;
    let mut buf: uint32_t = 0 as uint32_t;
    let mut pos = 0 as ::core::ffi::c_int;
    let mut end = 0 as ::core::ffi::c_int;
    loop {
        if s >= srcend {
            current_block = 3437258052017859086;
            break;
        }
        let fresh8 = s;
        s = s.offset(1);
        c = *fresh8;
        if c as ::core::ffi::c_int == ' ' as i32
            || c as ::core::ffi::c_int == '\t' as i32
            || c as ::core::ffi::c_int == '\n' as i32
            || c as ::core::ffi::c_int == '\r' as i32
        {
            current_block = 353454654129660769;
            break;
        }
        if c as ::core::ffi::c_int == '=' as i32 {
            if end == 0 {
                if pos == 2 as ::core::ffi::c_int {
                    end = 1 as ::core::ffi::c_int;
                } else if pos == 3 as ::core::ffi::c_int {
                    end = 2 as ::core::ffi::c_int;
                } else {
                    current_block = 353454654129660769;
                    break;
                }
            }
            b = 0 as ::core::ffi::c_int;
        } else {
            b = -(1 as ::core::ffi::c_int);
            if c as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                && (c as ::core::ffi::c_int) < 127 as ::core::ffi::c_int
            {
                b = b64lookup[c as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int;
            }
            if b < 0 as ::core::ffi::c_int {
                current_block = 353454654129660769;
                break;
            }
        }
        buf = (buf << 6 as ::core::ffi::c_int).wrapping_add(b as uint32_t);
        pos += 1;
        if pos != 4 as ::core::ffi::c_int {
            continue;
        }
        if p.offset_from(dst) as ::core::ffi::c_long + 1 as ::core::ffi::c_long
            > dstlen as ::core::ffi::c_long
        {
            current_block = 353454654129660769;
            break;
        }
        let fresh9 = p;
        p = p.offset(1);
        *fresh9 = (buf >> 16 as ::core::ffi::c_int & 255 as uint32_t) as uint8_t;
        if end == 0 as ::core::ffi::c_int || end > 1 as ::core::ffi::c_int {
            if p.offset_from(dst) as ::core::ffi::c_long + 1 as ::core::ffi::c_long
                > dstlen as ::core::ffi::c_long
            {
                current_block = 353454654129660769;
                break;
            }
            let fresh10 = p;
            p = p.offset(1);
            *fresh10 = (buf >> 8 as ::core::ffi::c_int & 255 as uint32_t) as uint8_t;
        }
        if end == 0 as ::core::ffi::c_int || end > 2 as ::core::ffi::c_int {
            if p.offset_from(dst) as ::core::ffi::c_long + 1 as ::core::ffi::c_long
                > dstlen as ::core::ffi::c_long
            {
                current_block = 353454654129660769;
                break;
            }
            let fresh11 = p;
            p = p.offset(1);
            *fresh11 = (buf & 255 as uint32_t) as uint8_t;
        }
        buf = 0 as uint32_t;
        pos = 0 as ::core::ffi::c_int;
    }
    if current_block == 3437258052017859086 && (pos == 0 as ::core::ffi::c_int) {
        return p.offset_from(dst) as ::core::ffi::c_long as ::core::ffi::c_int;
    }
    memset(
        dst as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        dstlen as size_t,
    );
    -(1 as ::core::ffi::c_int)
}

#[no_mangle]
#[c2rust::src_loc = "219:1"]
pub unsafe extern "C" fn pg_b64_enc_len(mut srclen: ::core::ffi::c_int) -> ::core::ffi::c_int {
    (srclen + 2 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
}

#[no_mangle]
#[c2rust::src_loc = "234:1"]
pub unsafe extern "C" fn pg_b64_dec_len(mut srclen: ::core::ffi::c_int) -> ::core::ffi::c_int {
    (srclen * 3 as ::core::ffi::c_int) >> 2 as ::core::ffi::c_int
}
