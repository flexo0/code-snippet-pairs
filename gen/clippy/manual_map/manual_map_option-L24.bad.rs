fn main() {
    if let Some(x) = Some(0) {
        //~^ manual_map
        Some(!x)
    } else {
        None
    };
}
