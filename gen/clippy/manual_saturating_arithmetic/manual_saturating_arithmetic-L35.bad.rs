fn main() {
    let _ = 1i128
        //~^ manual_saturating_arithmetic
        .checked_add(1)
        .unwrap_or(170_141_183_460_469_231_731_687_303_715_884_105_727);
    let _ = 1i32.checked_add(-1).unwrap_or(i32::min_value());
}
