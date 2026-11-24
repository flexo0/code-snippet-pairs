fn wrongly_unmangled_macros() {
    test_slice!(foo).swap(0, 1);
}
