fn main() {
    match Some(3) {
        //~^ manual_filter
        Some(x) => {
            if x > 0 {
                Some(x)
            } else {
                None
            }
        },
        None => None,
    };
}
