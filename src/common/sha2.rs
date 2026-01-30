//! SHA-2 implementation for PostgreSQL
//!
//! Refactored to use shared types module.

use super::types::{
    memcpy, memset, pg_sha224_ctx, pg_sha256_ctx, pg_sha384_ctx, pg_sha512_ctx, size_t, uint32_t,
    uint64_t, uint8_t, PG_SHA224_DIGEST_LENGTH, PG_SHA256_BLOCK_LENGTH, PG_SHA256_DIGEST_LENGTH,
    PG_SHA384_BLOCK_LENGTH, PG_SHA384_DIGEST_LENGTH, PG_SHA512_BLOCK_LENGTH,
    PG_SHA512_DIGEST_LENGTH,
};

pub const PG_SHA256_SHORT_BLOCK_LENGTH: ::core::ffi::c_int =
    PG_SHA256_BLOCK_LENGTH - 8 as ::core::ffi::c_int;
pub const PG_SHA512_SHORT_BLOCK_LENGTH: ::core::ffi::c_int =
    PG_SHA512_BLOCK_LENGTH - 16 as ::core::ffi::c_int;

static mut K256: [uint32_t; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

static mut sha224_initial_hash_value: [uint32_t; 8] = [
    0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939, 0xffc00b31, 0x68581511, 0x64f98fa7, 0xbefa4fa4,
];

static mut sha256_initial_hash_value: [uint32_t; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

static mut K512: [uint64_t; 80] = [
    0x428a2f98d728ae22,
    0x7137449123ef65cd,
    0xb5c0fbcfec4d3b2f,
    0xe9b5dba58189dbbc,
    0x3956c25bf348b538,
    0x59f111f1b605d019,
    0x923f82a4af194f9b,
    0xab1c5ed5da6d8118,
    0xd807aa98a3030242,
    0x12835b0145706fbe,
    0x243185be4ee4b28c,
    0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f,
    0x80deb1fe3b1696b1,
    0x9bdc06a725c71235,
    0xc19bf174cf692694,
    0xe49b69c19ef14ad2,
    0xefbe4786384f25e3,
    0x0fc19dc68b8cd5b5,
    0x240ca1cc77ac9c65,
    0x2de92c6f592b0275,
    0x4a7484aa6ea6e483,
    0x5cb0a9dcbd41fbd4,
    0x76f988da831153b5,
    0x983e5152ee66dfab,
    0xa831c66d2db43210,
    0xb00327c898fb213f,
    0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2,
    0xd5a79147930aa725,
    0x06ca6351e003826f,
    0x142929670a0e6e70,
    0x27b70a8546d22ffc,
    0x2e1b21385c26c926,
    0x4d2c6dfc5ac42aed,
    0x53380d139d95b3df,
    0x650a73548baf63de,
    0x766a0abb3c77b2a8,
    0x81c2c92e47edaee6,
    0x92722c851482353b,
    0xa2bfe8a14cf10364,
    0xa81a664bbc423001,
    0xc24b8b70d0f89791,
    0xc76c51a30654be30,
    0xd192e819d6ef5218,
    0xd69906245565a910,
    0xf40e35855771202a,
    0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8,
    0x1e376c085141ab53,
    0x2748774cdf8eeb99,
    0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63,
    0x4ed8aa4ae3418acb,
    0x5b9cca4f7763e373,
    0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc,
    0x78a5636f43172f60,
    0x84c87814a1f0ab72,
    0x8cc702081a6439ec,
    0x90befffa23631e28,
    0xa4506cebde82bde9,
    0xbef9a3f7b2c67915,
    0xc67178f2e372532b,
    0xca273eceea26619c,
    0xd186b8c721c0c207,
    0xeada7dd6cde0eb1e,
    0xf57d4f7fee6ed178,
    0x06f067aa72176fba,
    0x0a637dc5a2c898a6,
    0x113f9804bef90dae,
    0x1b710b35131c471b,
    0x28db77f523047d84,
    0x32caab7b40c72493,
    0x3c9ebe0a15c9bebc,
    0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6,
    0x597f299cfc657e2a,
    0x5fcb6fab3ad6faec,
    0x6c44198c4a475817,
];

static mut sha384_initial_hash_value: [uint64_t; 8] = [
    0xcbbb9d5dc1059ed8,
    0x629a292a367cd507,
    0x9159015a3070dd17,
    0x152fecd8f70e5939,
    0x67332667ffc00b31,
    0x8eb44a8768581511,
    0xdb0c2e0d64f98fa7,
    0x47b5481dbefa4fa4,
];

static mut sha512_initial_hash_value: [uint64_t; 8] = [
    0x6a09e667f3bcc908,
    0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b,
    0xa54ff53a5f1d36f1,
    0x510e527fade682d1,
    0x9b05688c2b3e6c1f,
    0x1f83d9abfb41bd6b,
    0x5be0cd19137e2179,
];

#[no_mangle]
pub unsafe extern "C" fn pg_sha256_init(mut context: *mut pg_sha256_ctx) {
    if context.is_null() {
        return;
    }
    memcpy(
        &raw mut (*context).state as *mut uint32_t as *mut ::core::ffi::c_void,
        &raw const sha256_initial_hash_value as *const uint32_t as *const ::core::ffi::c_void,
        PG_SHA256_DIGEST_LENGTH as size_t,
    );
    memset(
        &raw mut (*context).buffer as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        PG_SHA256_BLOCK_LENGTH as size_t,
    );
    (*context).bitcount = 0;
}

unsafe extern "C" fn SHA256_Transform(mut context: *mut pg_sha256_ctx, mut data: *const uint8_t) {
    let mut a: uint32_t;
    let mut b: uint32_t;
    let mut c: uint32_t;
    let mut d: uint32_t;
    let mut e: uint32_t;
    let mut f: uint32_t;
    let mut g: uint32_t;
    let mut h: uint32_t;
    let mut s0: uint32_t;
    let mut s1: uint32_t;
    let mut T1: uint32_t;
    let mut T2: uint32_t;
    let mut W256: *mut uint32_t;
    let mut j: ::core::ffi::c_int;
    W256 = &raw mut (*context).buffer as *mut uint8_t as *mut uint32_t;
    a = (*context).state[0];
    b = (*context).state[1];
    c = (*context).state[2];
    d = (*context).state[3];
    e = (*context).state[4];
    f = (*context).state[5];
    g = (*context).state[6];
    h = (*context).state[7];
    j = 0;
    loop {
        *W256.offset(j as isize) = *data.offset(3) as uint32_t
            | (*data.offset(2) as uint32_t) << 8
            | (*data.offset(1) as uint32_t) << 16
            | (*data as uint32_t) << 24;
        data = data.offset(4);
        T1 = h
            .wrapping_add((e >> 6 | e << 26) ^ (e >> 11 | e << 21) ^ (e >> 25 | e << 7))
            .wrapping_add(e & f ^ !e & g)
            .wrapping_add(K256[j as usize])
            .wrapping_add(*W256.offset(j as isize));
        T2 = ((a >> 2 | a << 30) ^ (a >> 13 | a << 19) ^ (a >> 22 | a << 10))
            .wrapping_add(a & b ^ a & c ^ b & c);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(T1);
        d = c;
        c = b;
        b = a;
        a = T1.wrapping_add(T2);
        j += 1;
        if j >= 16 {
            break;
        }
    }
    loop {
        s0 = *W256.offset(((j + 1) & 0xf) as isize);
        s0 = (s0 >> 7 | s0 << 25) ^ (s0 >> 18 | s0 << 14) ^ s0 >> 3;
        s1 = *W256.offset(((j + 14) & 0xf) as isize);
        s1 = (s1 >> 17 | s1 << 15) ^ (s1 >> 19 | s1 << 13) ^ s1 >> 10;
        let fresh0 = &mut *W256.offset((j & 0xf) as isize);
        *fresh0 = (*fresh0).wrapping_add(
            s1.wrapping_add(*W256.offset(((j + 9) & 0xf) as isize))
                .wrapping_add(s0),
        );
        T1 = h
            .wrapping_add((e >> 6 | e << 26) ^ (e >> 11 | e << 21) ^ (e >> 25 | e << 7))
            .wrapping_add(e & f ^ !e & g)
            .wrapping_add(K256[j as usize])
            .wrapping_add(*fresh0);
        T2 = ((a >> 2 | a << 30) ^ (a >> 13 | a << 19) ^ (a >> 22 | a << 10))
            .wrapping_add(a & b ^ a & c ^ b & c);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(T1);
        d = c;
        c = b;
        b = a;
        a = T1.wrapping_add(T2);
        j += 1;
        if j >= 64 {
            break;
        }
    }
    (*context).state[0] = (*context).state[0].wrapping_add(a);
    (*context).state[1] = (*context).state[1].wrapping_add(b);
    (*context).state[2] = (*context).state[2].wrapping_add(c);
    (*context).state[3] = (*context).state[3].wrapping_add(d);
    (*context).state[4] = (*context).state[4].wrapping_add(e);
    (*context).state[5] = (*context).state[5].wrapping_add(f);
    (*context).state[6] = (*context).state[6].wrapping_add(g);
    (*context).state[7] = (*context).state[7].wrapping_add(h);
}

#[no_mangle]
pub unsafe extern "C" fn pg_sha256_update(
    mut context: *mut pg_sha256_ctx,
    mut data: *const uint8_t,
    mut len: size_t,
) {
    let mut freespace: size_t;
    let mut usedspace: size_t;
    if len == 0 {
        return;
    }
    usedspace =
        ((*context).bitcount >> 3).wrapping_rem(PG_SHA256_BLOCK_LENGTH as uint64_t) as size_t;
    if usedspace > 0 {
        freespace = (PG_SHA256_BLOCK_LENGTH as size_t).wrapping_sub(usedspace);
        if len >= freespace {
            memcpy(
                (&raw mut (*context).buffer as *mut uint8_t).add(usedspace)
                    as *mut ::core::ffi::c_void,
                data as *const ::core::ffi::c_void,
                freespace,
            );
            (*context).bitcount = (*context)
                .bitcount
                .wrapping_add((freespace << 3) as uint64_t);
            len = len.wrapping_sub(freespace);
            data = data.add(freespace);
            SHA256_Transform(context, &raw mut (*context).buffer as *mut uint8_t);
        } else {
            memcpy(
                (&raw mut (*context).buffer as *mut uint8_t).add(usedspace)
                    as *mut ::core::ffi::c_void,
                data as *const ::core::ffi::c_void,
                len,
            );
            (*context).bitcount += ((len << 3) as uint64_t);
            return;
        }
    }
    while len >= PG_SHA256_BLOCK_LENGTH as size_t {
        SHA256_Transform(context, data);
        (*context).bitcount = (*context)
            .bitcount
            .wrapping_add((PG_SHA256_BLOCK_LENGTH << 3) as uint64_t);
        len = len.wrapping_sub(PG_SHA256_BLOCK_LENGTH as size_t);
        data = data.offset(PG_SHA256_BLOCK_LENGTH as isize);
    }
    if len > 0 {
        memcpy(
            &raw mut (*context).buffer as *mut uint8_t as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            len,
        );
        (*context).bitcount += ((len << 3) as uint64_t);
    }
}

unsafe extern "C" fn SHA256_Last(mut context: *mut pg_sha256_ctx) {
    let mut usedspace: ::core::ffi::c_uint;
    usedspace = ((*context).bitcount >> 3).wrapping_rem(PG_SHA256_BLOCK_LENGTH as uint64_t)
        as ::core::ffi::c_uint;
    let mut tmp: uint64_t = (*context).bitcount;
    tmp = tmp >> 32 | tmp << 32;
    tmp = (tmp & 0xff00ff00ff00ff00) >> 8 | (tmp & 0x00ff00ff00ff00ff) << 8;
    (*context).bitcount = (tmp & 0xffff0000ffff0000) >> 16 | (tmp & 0x0000ffff0000ffff) << 16;
    if usedspace > 0 {
        let fresh1 = usedspace;
        usedspace = usedspace.wrapping_add(1);
        (*context).buffer[fresh1 as usize] = 0x80;
        if usedspace <= PG_SHA256_SHORT_BLOCK_LENGTH as ::core::ffi::c_uint {
            memset(
                (&raw mut (*context).buffer as *mut uint8_t).offset(usedspace as isize)
                    as *mut ::core::ffi::c_void,
                0,
                (PG_SHA256_SHORT_BLOCK_LENGTH as ::core::ffi::c_uint).wrapping_sub(usedspace)
                    as size_t,
            );
        } else {
            if usedspace < PG_SHA256_BLOCK_LENGTH as ::core::ffi::c_uint {
                memset(
                    (&raw mut (*context).buffer as *mut uint8_t).offset(usedspace as isize)
                        as *mut ::core::ffi::c_void,
                    0,
                    (PG_SHA256_BLOCK_LENGTH as ::core::ffi::c_uint).wrapping_sub(usedspace)
                        as size_t,
                );
            }
            SHA256_Transform(context, &raw mut (*context).buffer as *mut uint8_t);
            memset(
                &raw mut (*context).buffer as *mut uint8_t as *mut ::core::ffi::c_void,
                0,
                PG_SHA256_SHORT_BLOCK_LENGTH as size_t,
            );
        }
    } else {
        memset(
            &raw mut (*context).buffer as *mut uint8_t as *mut ::core::ffi::c_void,
            0,
            PG_SHA256_SHORT_BLOCK_LENGTH as size_t,
        );
        *(&raw mut (*context).buffer as *mut uint8_t) = 0x80;
    }
    *((&raw mut (*context).buffer as *mut uint8_t).offset(PG_SHA256_SHORT_BLOCK_LENGTH as isize)
        as *mut uint64_t) = (*context).bitcount;
    SHA256_Transform(context, &raw mut (*context).buffer as *mut uint8_t);
}

#[no_mangle]
pub unsafe extern "C" fn pg_sha256_final(
    mut context: *mut pg_sha256_ctx,
    mut digest: *mut uint8_t,
) {
    if !digest.is_null() {
        SHA256_Last(context);
        let mut j: ::core::ffi::c_int = 0;
        while j < 8 {
            let mut tmp: uint32_t = (*context).state[j as usize];
            tmp = tmp >> 16 | tmp << 16;
            (*context).state[j as usize] =
                ((tmp as u64 & 0xff00ff00) >> 8 | (tmp as u64 & 0x00ff00ff) << 8) as uint32_t;
            j += 1;
        }
        memcpy(
            digest as *mut ::core::ffi::c_void,
            &raw mut (*context).state as *mut uint32_t as *const ::core::ffi::c_void,
            PG_SHA256_DIGEST_LENGTH as size_t,
        );
    }
    memset(
        context as *mut ::core::ffi::c_void,
        0,
        ::core::mem::size_of::<pg_sha256_ctx>() as size_t,
    );
}

#[no_mangle]
pub unsafe extern "C" fn pg_sha512_init(mut context: *mut pg_sha512_ctx) {
    if context.is_null() {
        return;
    }
    memcpy(
        &raw mut (*context).state as *mut uint64_t as *mut ::core::ffi::c_void,
        &raw const sha512_initial_hash_value as *const uint64_t as *const ::core::ffi::c_void,
        PG_SHA512_DIGEST_LENGTH as size_t,
    );
    memset(
        &raw mut (*context).buffer as *mut uint8_t as *mut ::core::ffi::c_void,
        0,
        PG_SHA512_BLOCK_LENGTH as size_t,
    );
    (*context).bitcount[1] = 0;
    (*context).bitcount[0] = (*context).bitcount[1];
}

unsafe extern "C" fn SHA512_Transform(mut context: *mut pg_sha512_ctx, mut data: *const uint8_t) {
    let mut a: uint64_t;
    let mut b: uint64_t;
    let mut c: uint64_t;
    let mut d: uint64_t;
    let mut e: uint64_t;
    let mut f: uint64_t;
    let mut g: uint64_t;
    let mut h: uint64_t;
    let mut s0: uint64_t;
    let mut s1: uint64_t;
    let mut T1: uint64_t;
    let mut T2: uint64_t;
    let W512 = &raw mut (*context).buffer as *mut uint8_t as *mut uint64_t;
    let mut j: ::core::ffi::c_int;
    a = (*context).state[0];
    b = (*context).state[1];
    c = (*context).state[2];
    d = (*context).state[3];
    e = (*context).state[4];
    f = (*context).state[5];
    g = (*context).state[6];
    h = (*context).state[7];
    j = 0;
    loop {
        *W512.offset(j as isize) = *data.offset(7) as uint64_t
            | (*data.offset(6) as uint64_t) << 8
            | (*data.offset(5) as uint64_t) << 16
            | (*data.offset(4) as uint64_t) << 24
            | (*data.offset(3) as uint64_t) << 32
            | (*data.offset(2) as uint64_t) << 40
            | (*data.offset(1) as uint64_t) << 48
            | (*data as uint64_t) << 56;
        data = data.offset(8);
        T1 = h
            .wrapping_add((e >> 14 | e << 50) ^ (e >> 18 | e << 46) ^ (e >> 41 | e << 23))
            .wrapping_add(e & f ^ !e & g)
            .wrapping_add(K512[j as usize])
            .wrapping_add(*W512.offset(j as isize));
        T2 = ((a >> 28 | a << 36) ^ (a >> 34 | a << 30) ^ (a >> 39 | a << 25))
            .wrapping_add(a & b ^ a & c ^ b & c);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(T1);
        d = c;
        c = b;
        b = a;
        a = T1.wrapping_add(T2);
        j += 1;
        if j >= 16 {
            break;
        }
    }
    loop {
        s0 = *W512.offset(((j + 1) & 0xf) as isize);
        s0 = (s0 >> 1 | s0 << 63) ^ (s0 >> 8 | s0 << 56) ^ s0 >> 7;
        s1 = *W512.offset(((j + 14) & 0xf) as isize);
        s1 = (s1 >> 19 | s1 << 45) ^ (s1 >> 61 | s1 << 3) ^ s1 >> 6;
        let fresh2 = &mut *W512.offset((j & 0xf) as isize);
        *fresh2 = (*fresh2).wrapping_add(
            s1.wrapping_add(*W512.offset(((j + 9) & 0xf) as isize))
                .wrapping_add(s0),
        );
        T1 = h
            .wrapping_add((e >> 14 | e << 50) ^ (e >> 18 | e << 46) ^ (e >> 41 | e << 23))
            .wrapping_add(e & f ^ !e & g)
            .wrapping_add(K512[j as usize])
            .wrapping_add(*fresh2);
        T2 = ((a >> 28 | a << 36) ^ (a >> 34 | a << 30) ^ (a >> 39 | a << 25))
            .wrapping_add(a & b ^ a & c ^ b & c);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(T1);
        d = c;
        c = b;
        b = a;
        a = T1.wrapping_add(T2);
        j += 1;
        if j >= 80 {
            break;
        }
    }
    (*context).state[0] = (*context).state[0].wrapping_add(a);
    (*context).state[1] = (*context).state[1].wrapping_add(b);
    (*context).state[2] = (*context).state[2].wrapping_add(c);
    (*context).state[3] = (*context).state[3].wrapping_add(d);
    (*context).state[4] = (*context).state[4].wrapping_add(e);
    (*context).state[5] = (*context).state[5].wrapping_add(f);
    (*context).state[6] = (*context).state[6].wrapping_add(g);
    (*context).state[7] = (*context).state[7].wrapping_add(h);
}

#[no_mangle]
pub unsafe extern "C" fn pg_sha512_update(
    mut context: *mut pg_sha512_ctx,
    mut data: *const uint8_t,
    mut len: size_t,
) {
    let mut freespace: size_t;
    let mut usedspace: size_t;
    if len == 0 {
        return;
    }
    usedspace =
        ((*context).bitcount[0] >> 3).wrapping_rem(PG_SHA512_BLOCK_LENGTH as uint64_t) as size_t;
    if usedspace > 0 {
        freespace = (PG_SHA512_BLOCK_LENGTH as size_t).wrapping_sub(usedspace);
        if len >= freespace {
            memcpy(
                (&raw mut (*context).buffer as *mut uint8_t).add(usedspace)
                    as *mut ::core::ffi::c_void,
                data as *const ::core::ffi::c_void,
                freespace,
            );
            (*context).bitcount[0] =
                (*context).bitcount[0].wrapping_add((freespace << 3) as uint64_t);
            if (*context).bitcount[0] < (freespace << 3) as uint64_t {
                (*context).bitcount[1] = (*context).bitcount[1].wrapping_add(1);
            }
            len = len.wrapping_sub(freespace);
            data = data.add(freespace);
            SHA512_Transform(context, &raw mut (*context).buffer as *mut uint8_t);
        } else {
            memcpy(
                (&raw mut (*context).buffer as *mut uint8_t).add(usedspace)
                    as *mut ::core::ffi::c_void,
                data as *const ::core::ffi::c_void,
                len,
            );
            (*context).bitcount[0] = (*context).bitcount[0].wrapping_add((len << 3) as uint64_t);
            if (*context).bitcount[0] < (len << 3) as uint64_t {
                (*context).bitcount[1] = (*context).bitcount[1].wrapping_add(1);
            }
            return;
        }
    }
    while len >= PG_SHA512_BLOCK_LENGTH as size_t {
        SHA512_Transform(context, data);
        (*context).bitcount[0] = (*context).bitcount[0].wrapping_add((128 << 3) as uint64_t);
        if (*context).bitcount[0] < (128 << 3) as uint64_t {
            (*context).bitcount[1] = (*context).bitcount[1].wrapping_add(1);
        }
        len = len.wrapping_sub(PG_SHA512_BLOCK_LENGTH as size_t);
        data = data.offset(PG_SHA512_BLOCK_LENGTH as isize);
    }
    if len > 0 {
        memcpy(
            &raw mut (*context).buffer as *mut uint8_t as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            len,
        );
        (*context).bitcount[0] = (*context).bitcount[0].wrapping_add((len << 3) as uint64_t);
        if (*context).bitcount[0] < (len << 3) as uint64_t {
            (*context).bitcount[1] = (*context).bitcount[1].wrapping_add(1);
        }
    }
}

unsafe extern "C" fn SHA512_Last(mut context: *mut pg_sha512_ctx) {
    let mut usedspace: ::core::ffi::c_uint;
    usedspace = ((*context).bitcount[0] >> 3).wrapping_rem(PG_SHA512_BLOCK_LENGTH as uint64_t)
        as ::core::ffi::c_uint;
    let mut tmp: uint64_t = (*context).bitcount[0];
    tmp = tmp >> 32 | tmp << 32;
    tmp = (tmp & 0xff00ff00ff00ff00) >> 8 | (tmp & 0x00ff00ff00ff00ff) << 8;
    (*context).bitcount[0] = (tmp & 0xffff0000ffff0000) >> 16 | (tmp & 0x0000ffff0000ffff) << 16;
    let mut tmp_0: uint64_t = (*context).bitcount[1];
    tmp_0 = tmp_0 >> 32 | tmp_0 << 32;
    tmp_0 = (tmp_0 & 0xff00ff00ff00ff00) >> 8 | (tmp_0 & 0x00ff00ff00ff00ff) << 8;
    (*context).bitcount[1] =
        (tmp_0 & 0xffff0000ffff0000) >> 16 | (tmp_0 & 0x0000ffff0000ffff) << 16;
    if usedspace > 0 {
        let fresh3 = usedspace;
        usedspace = usedspace.wrapping_add(1);
        (*context).buffer[fresh3 as usize] = 0x80;
        if usedspace <= PG_SHA512_SHORT_BLOCK_LENGTH as ::core::ffi::c_uint {
            memset(
                (&raw mut (*context).buffer as *mut uint8_t).offset(usedspace as isize)
                    as *mut ::core::ffi::c_void,
                0,
                (PG_SHA512_SHORT_BLOCK_LENGTH as ::core::ffi::c_uint).wrapping_sub(usedspace)
                    as size_t,
            );
        } else {
            if usedspace < PG_SHA512_BLOCK_LENGTH as ::core::ffi::c_uint {
                memset(
                    (&raw mut (*context).buffer as *mut uint8_t).offset(usedspace as isize)
                        as *mut ::core::ffi::c_void,
                    0,
                    (PG_SHA512_BLOCK_LENGTH as ::core::ffi::c_uint).wrapping_sub(usedspace)
                        as size_t,
                );
            }
            SHA512_Transform(context, &raw mut (*context).buffer as *mut uint8_t);
            memset(
                &raw mut (*context).buffer as *mut uint8_t as *mut ::core::ffi::c_void,
                0,
                (PG_SHA512_BLOCK_LENGTH - 2) as size_t,
            );
        }
    } else {
        memset(
            &raw mut (*context).buffer as *mut uint8_t as *mut ::core::ffi::c_void,
            0,
            PG_SHA512_SHORT_BLOCK_LENGTH as size_t,
        );
        *(&raw mut (*context).buffer as *mut uint8_t) = 0x80;
    }
    *((&raw mut (*context).buffer as *mut uint8_t).offset(PG_SHA512_SHORT_BLOCK_LENGTH as isize)
        as *mut uint64_t) = (*context).bitcount[1];
    *((&raw mut (*context).buffer as *mut uint8_t)
        .offset((PG_SHA512_SHORT_BLOCK_LENGTH + 8) as isize) as *mut uint64_t) =
        (*context).bitcount[0];
    SHA512_Transform(context, &raw mut (*context).buffer as *mut uint8_t);
}

#[no_mangle]
pub unsafe extern "C" fn pg_sha512_final(
    mut context: *mut pg_sha512_ctx,
    mut digest: *mut uint8_t,
) {
    if !digest.is_null() {
        SHA512_Last(context);
        let mut j: ::core::ffi::c_int = 0;
        while j < 8 {
            let mut tmp: uint64_t = (*context).state[j as usize];
            tmp = tmp >> 32 | tmp << 32;
            tmp = (tmp & 0xff00ff00ff00ff00) >> 8 | (tmp & 0x00ff00ff00ff00ff) << 8;
            (*context).state[j as usize] =
                (tmp & 0xffff0000ffff0000) >> 16 | (tmp & 0x0000ffff0000ffff) << 16;
            j += 1;
        }
        memcpy(
            digest as *mut ::core::ffi::c_void,
            &raw mut (*context).state as *mut uint64_t as *const ::core::ffi::c_void,
            PG_SHA512_DIGEST_LENGTH as size_t,
        );
    }
    memset(
        context as *mut ::core::ffi::c_void,
        0,
        ::core::mem::size_of::<pg_sha512_ctx>() as size_t,
    );
}

#[no_mangle]
pub unsafe extern "C" fn pg_sha384_init(mut context: *mut pg_sha384_ctx) {
    if context.is_null() {
        return;
    }
    memcpy(
        &raw mut (*context).state as *mut uint64_t as *mut ::core::ffi::c_void,
        &raw const sha384_initial_hash_value as *const uint64_t as *const ::core::ffi::c_void,
        PG_SHA512_DIGEST_LENGTH as size_t,
    );
    memset(
        &raw mut (*context).buffer as *mut uint8_t as *mut ::core::ffi::c_void,
        0,
        PG_SHA384_BLOCK_LENGTH as size_t,
    );
    (*context).bitcount[1] = 0;
    (*context).bitcount[0] = (*context).bitcount[1];
}

#[no_mangle]
pub unsafe extern "C" fn pg_sha384_update(
    mut context: *mut pg_sha384_ctx,
    mut data: *const uint8_t,
    mut len: size_t,
) {
    pg_sha512_update(context as *mut pg_sha512_ctx, data, len);
}

#[no_mangle]
pub unsafe extern "C" fn pg_sha384_final(
    mut context: *mut pg_sha384_ctx,
    mut digest: *mut uint8_t,
) {
    if !digest.is_null() {
        SHA512_Last(context as *mut pg_sha512_ctx);
        let mut j: ::core::ffi::c_int = 0;
        while j < 6 {
            let mut tmp: uint64_t = (*context).state[j as usize];
            tmp = tmp >> 32 | tmp << 32;
            tmp = (tmp & 0xff00ff00ff00ff00) >> 8 | (tmp & 0x00ff00ff00ff00ff) << 8;
            (*context).state[j as usize] =
                (tmp & 0xffff0000ffff0000) >> 16 | (tmp & 0x0000ffff0000ffff) << 16;
            j += 1;
        }
        memcpy(
            digest as *mut ::core::ffi::c_void,
            &raw mut (*context).state as *mut uint64_t as *const ::core::ffi::c_void,
            PG_SHA384_DIGEST_LENGTH as size_t,
        );
    }
    memset(
        context as *mut ::core::ffi::c_void,
        0,
        ::core::mem::size_of::<pg_sha384_ctx>() as size_t,
    );
}

#[no_mangle]
pub unsafe extern "C" fn pg_sha224_init(mut context: *mut pg_sha224_ctx) {
    if context.is_null() {
        return;
    }
    memcpy(
        &raw mut (*context).state as *mut uint32_t as *mut ::core::ffi::c_void,
        &raw const sha224_initial_hash_value as *const uint32_t as *const ::core::ffi::c_void,
        PG_SHA256_DIGEST_LENGTH as size_t,
    );
    memset(
        &raw mut (*context).buffer as *mut uint8_t as *mut ::core::ffi::c_void,
        0,
        PG_SHA256_BLOCK_LENGTH as size_t,
    );
    (*context).bitcount = 0;
}

#[no_mangle]
pub unsafe extern "C" fn pg_sha224_update(
    mut context: *mut pg_sha224_ctx,
    mut data: *const uint8_t,
    mut len: size_t,
) {
    pg_sha256_update(context as *mut pg_sha256_ctx, data, len);
}

#[no_mangle]
pub unsafe extern "C" fn pg_sha224_final(
    mut context: *mut pg_sha224_ctx,
    mut digest: *mut uint8_t,
) {
    if !digest.is_null() {
        SHA256_Last(context as *mut pg_sha256_ctx);
        let mut j: ::core::ffi::c_int = 0;
        while j < 8 {
            let mut tmp: uint32_t = (*context).state[j as usize];
            tmp = tmp >> 16 | tmp << 16;
            (*context).state[j as usize] =
                ((tmp as u64 & 0xff00ff00) >> 8 | (tmp as u64 & 0x00ff00ff) << 8) as uint32_t;
            j += 1;
        }
        memcpy(
            digest as *mut ::core::ffi::c_void,
            &raw mut (*context).state as *mut uint32_t as *const ::core::ffi::c_void,
            PG_SHA224_DIGEST_LENGTH as size_t,
        );
    }
    memset(
        context as *mut ::core::ffi::c_void,
        0,
        ::core::mem::size_of::<pg_sha224_ctx>() as size_t,
    );
}
