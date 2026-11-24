fn main() {
    match Some(6) {
        //~^ manual_filter
        Some(ref x) => {
            if x > &0 {
                Some(x)
            } else {
                None
            }
        },
        _ => None,
    };
}
