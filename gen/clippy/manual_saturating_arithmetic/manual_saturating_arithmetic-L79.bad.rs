fn issue15655() {
    let _ = 5u32.checked_sub(1u32).unwrap_or_default(); //~ manual_saturating_arithmetic
}
