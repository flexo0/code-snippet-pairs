fn issue_10449() {
    let _x: u8 = unsafe { *(f as *const u8) };
}
