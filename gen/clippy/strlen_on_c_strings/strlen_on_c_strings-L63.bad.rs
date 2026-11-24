fn msrv_1_78() {
    let _ = unsafe { libc::strlen(cstr.as_ptr()) };
}
