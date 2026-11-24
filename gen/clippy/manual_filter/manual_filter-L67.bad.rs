fn main() {
    match Some(5) {
        //~^ manual_filter
        Some(x) => {
            if x > 0 {
                Some(x)
            } else {
                None
            }
        },
        _ => None,
    };
}
