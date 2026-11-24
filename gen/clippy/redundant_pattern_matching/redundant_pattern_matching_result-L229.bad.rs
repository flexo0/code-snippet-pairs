fn wrongly_unmangled_macros() {
    let _ = matches!(x, Ok(_) if test_guard!(42));
}
