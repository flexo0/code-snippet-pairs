fn main() {
    let _ = (1..5)
        .filter_map(|y| *to_ref(to_opt(y)));
}
