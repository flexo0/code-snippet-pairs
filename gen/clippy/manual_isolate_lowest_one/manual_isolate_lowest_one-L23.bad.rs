fn main() {
    let _ = nz_u32.get() & nz_u32.get().wrapping_neg();
}
