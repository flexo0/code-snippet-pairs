fn main() {
    let _ = result.or::<&str>(Ok("fallback")).unwrap(); // should trigger lint
}
