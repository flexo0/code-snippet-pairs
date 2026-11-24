fn f(a: u64, b: u64) {
    let _ = a % b == 0; //~ manual_is_multiple_of
    let _ = (a + 1) % (b + 1) == 0; //~ manual_is_multiple_of
    let _ = a % b != 0; //~ manual_is_multiple_of
    let _ = (a + 1) % (b + 1) != 0; //~ manual_is_multiple_of
}
