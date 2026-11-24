pub fn main() {
    let _ = Some("42").map(|s| if s.len() < 42 { 0 } else { s.len() });
}
