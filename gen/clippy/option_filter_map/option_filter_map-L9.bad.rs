fn main() {
    let _ = Some(Some(1)).filter(|o| o.is_some()).map(|o| o.unwrap());
}
