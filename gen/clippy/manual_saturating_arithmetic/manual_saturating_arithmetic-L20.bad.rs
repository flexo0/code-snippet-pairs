fn main() {
    let _ = 1u32.checked_sub(1).unwrap_or(u32::min_value());
}
