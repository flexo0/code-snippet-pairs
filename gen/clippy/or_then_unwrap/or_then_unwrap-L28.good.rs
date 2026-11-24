fn main() {
    let _ = result.unwrap_or("fallback"); // should trigger lint
}
