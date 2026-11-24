fn main() {
    let _ = (0..).filter(|&n| to_opt(n).is_some()).map(|a| to_opt(a).expect("hi"));
}
