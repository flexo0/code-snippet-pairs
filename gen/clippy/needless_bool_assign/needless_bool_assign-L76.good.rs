fn wrongly_unmangled_macros(must_keep: fn(usize, usize) -> bool, x: usize, y: usize) {
    dot_0!(skip) = !invoke!(must_keep, x, y);
}
