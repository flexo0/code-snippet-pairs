fn issue1231() {
    unsafe { &*(raw as *const u8) };
}
