fn wrongly_unmangled_macros() {
    let _ = if test_big!(a) > test_little!(b) {
        //~^ implicit_saturating_sub
        test_big!(a) - test_little!(b)
    } else {
        0
    };
}
