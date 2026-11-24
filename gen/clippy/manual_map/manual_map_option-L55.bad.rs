fn main() {
    match &Some(0) {
        //~^ manual_map
        &Some(x) => Some(x * 2),
        None => None,
    };
}
