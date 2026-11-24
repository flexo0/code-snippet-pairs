fn main() {
    if true {
        Some(0)
    } else { Some(0).map(|x| x + 1) };
}
