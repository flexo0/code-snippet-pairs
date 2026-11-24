pub fn main() {
    let _ = Some("42").and_then(|s| if s.len() < 42 { Some(0) } else { Some(s.len()) });
}
