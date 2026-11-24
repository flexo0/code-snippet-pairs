fn main() {
    let _ = (0..).find(|&n| to_opt(n).is_some()).map(|a| to_opt(a).expect("hi"));
}
