fn main() {
    match &&Some(String::new()) {
        //~^ manual_map
        Some(x) => Some(x.len()),
        _ => None,
    };
}
