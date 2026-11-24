fn main() {
    let _ = match None::<()> {
        //~^ redundant_pattern_matching
        Some(_) => false,
        None => true,
    };
}
