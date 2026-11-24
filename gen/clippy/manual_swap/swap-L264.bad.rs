fn wrongly_unmangled_macros() {
    let temp = test_slice!(foo)[0];
    //~^ manual_swap
    test_slice!(foo)[0] = test_slice!(foo)[1];
    test_slice!(foo)[1] = temp;
}
