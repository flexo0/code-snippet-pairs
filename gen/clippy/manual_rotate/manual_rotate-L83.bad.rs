fn wrongly_unmangled_macros() {
    let _ = (x.inner << test_expr!(y)) | (x.inner >> (32 - test_expr!(y)));
}
