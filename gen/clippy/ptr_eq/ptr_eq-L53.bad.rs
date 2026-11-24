fn main() {
    let _ = x as *const u32 != y as *mut u32 as *const u32;
}
