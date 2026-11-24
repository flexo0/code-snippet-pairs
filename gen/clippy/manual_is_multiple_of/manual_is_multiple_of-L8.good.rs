fn f(a: u64, b: u64) {
    let _ = a.is_multiple_of(b); //~ manual_is_multiple_of
    let _ = (a + 1).is_multiple_of(b + 1); //~ manual_is_multiple_of
    let _ = !a.is_multiple_of(b); //~ manual_is_multiple_of
    let _ = !(a + 1).is_multiple_of(b + 1); //~ manual_is_multiple_of
}
