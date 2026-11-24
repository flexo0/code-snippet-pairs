fn issue15655() {
    let _ = 5u32.saturating_sub(1u32); //~ manual_saturating_arithmetic
}
