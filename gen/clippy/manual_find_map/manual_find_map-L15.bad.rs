fn main() {
    let _ = (0..).find(|&n| to_res(n).is_ok()).map(|a| to_res(a).unwrap_or(1));
}
