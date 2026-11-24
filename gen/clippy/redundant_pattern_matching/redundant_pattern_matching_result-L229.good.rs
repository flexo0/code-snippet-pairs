fn wrongly_unmangled_macros() {
    let _ = x.is_ok() && test_guard!(42);
}
