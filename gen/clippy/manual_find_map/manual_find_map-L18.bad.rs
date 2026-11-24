fn main() {
    let _ = (1..5)
        .find(|&x| to_ref(to_opt(x)).is_some())
        //~^ manual_find_map
        .map(|y| to_ref(to_opt(y)).unwrap());
}
