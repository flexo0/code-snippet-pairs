fn main() {
    if let Some(_) = Some(16) {
        Some(16)
    } else { Some(16).filter(|&x| x % 2 == 0) };
}
