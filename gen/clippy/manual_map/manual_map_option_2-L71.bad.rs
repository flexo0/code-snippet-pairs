fn main() {
    let _ = match Some(0) {
        //~^ manual_map
        Some(x) => unsafe { Some(f(x)) },
        None => None,
    };
    let _ = match Some(0) {
        //~^ manual_map
        Some(x) => Some(unsafe { f(x) }),
        None => None,
    };
}
