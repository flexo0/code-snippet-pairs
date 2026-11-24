fn main() {
    if true {
        Some(0)
    } else if let Some(x) = Some(0) {
        //~^ manual_map
        Some(x + 1)
    } else {
        None
    };
}
