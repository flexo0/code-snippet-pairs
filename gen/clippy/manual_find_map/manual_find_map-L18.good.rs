fn main() {
    let _ = (1..5)
        .find_map(|y| *to_ref(to_opt(y)));
}
