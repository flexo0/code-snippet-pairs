fn main() {
    unsafe {
        let _ = match Some(0) {
            //~^ manual_map
            Some(x) => Some(f(x)),
            None => None,
        };
    }
}
