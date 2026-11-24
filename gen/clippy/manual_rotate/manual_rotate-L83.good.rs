fn wrongly_unmangled_macros() {
    let _ = x.inner.rotate_left(test_expr!(y));
}
