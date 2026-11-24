fn main() {
    match Some(42) {
        //~^ redundant_pattern_matching
        Some(_) => true,
        None => false,
    };
}
