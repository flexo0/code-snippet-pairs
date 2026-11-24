fn main() {
    let _ = match Some(14) {
        //~^ manual_filter
        Some(x) => {
            if unsafe { f(x) } {
                Some(x)
            } else {
                None
            }
        },
        None => None,
    };
    let _ = match Some(15) {
        //~^ manual_filter
        Some(x) => unsafe { if f(x) { Some(x) } else { None } },
        None => None,
    };
}
