fn needs_deref(a: &u64, b: &u64) {
    let _ = a % b == 0; //~ manual_is_multiple_of
}
