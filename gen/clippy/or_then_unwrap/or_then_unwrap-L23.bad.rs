fn main() {
    let _ = option.or(Some("fallback")).unwrap(); // should trigger lint
}
