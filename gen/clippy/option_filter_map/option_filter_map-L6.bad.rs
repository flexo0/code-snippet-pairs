fn main() {
    let _ = Some(Some(1)).filter(Option::is_some).map(Option::unwrap);
}
