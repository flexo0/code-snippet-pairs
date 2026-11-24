fn wrongly_unmangled_macros(must_keep: fn(usize, usize) -> bool, x: usize, y: usize) {
    if invoke!(must_keep, x, y) {
        dot_0!(skip) = false;
    } else {
        dot_0!(skip) = true;
    }
}
