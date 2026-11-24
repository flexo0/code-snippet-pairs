fn wrongly_unmangled_macros() {
    let _ = test_expr!(x).rotate_left(3);
}
