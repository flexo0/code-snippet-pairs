fn main() {
    if let Some(_) = Some(16) {
        Some(16)
    } else if let Some(x) = Some(16) {
        //~^ manual_filter
        // Lint starting from here
        if x % 2 == 0 { Some(x) } else { None }
    } else {
        None
    };
}
