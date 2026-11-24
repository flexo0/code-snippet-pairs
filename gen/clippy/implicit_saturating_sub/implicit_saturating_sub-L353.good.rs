fn wrongly_unmangled_macros() {
    let _ = test_big!(a).saturating_sub(test_little!(b));
}
