fn main() {
    match Some(0) {
        //~^ manual_map
        Some(_) => Some(2),
        None::<u32> => None,
    };
}
