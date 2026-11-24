fn main() {
    let _ = option.unwrap_or("fallback"); // should trigger lint
}
