fn main() {
    let _ = match Some(0) {
        //~^ manual_map
        Some(x) => Some({
            let y = (String::new(), String::new());
            (x, y.0)
        }),
        None => None,
    };
}
