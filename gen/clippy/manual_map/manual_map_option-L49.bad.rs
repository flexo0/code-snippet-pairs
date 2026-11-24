fn main() {
    match &Some([0, 1]) {
        //~^ manual_map
        Some(x) => Some(x[0]),
        &None => None,
    };
}
