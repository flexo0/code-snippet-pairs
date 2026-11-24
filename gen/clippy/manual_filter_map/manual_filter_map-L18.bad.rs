fn main() {
    let _ = (1..5)
        .filter(|&x| to_ref(to_opt(x)).is_some())
        //~^ manual_filter_map
        .map(|y| to_ref(to_opt(y)).unwrap());
}
