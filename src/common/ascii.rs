pub unsafe fn is_valid_ascii(s: *const ::core::ffi::c_uchar, len: usize) -> bool {
    for i in 0..len {
        if *s.add(i) > 127 {
            return false;
        }
    }

    true
}
