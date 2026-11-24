fn issue_10449() {
    let _x: u8 = unsafe { *std::mem::transmute::<fn(), *const u8>(f) };
}
