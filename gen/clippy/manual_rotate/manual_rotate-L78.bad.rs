fn wrongly_unmangled_macros() {
    let _ = (test_expr!(x) << 3) | (test_expr!(x) >> 29);
}
