fn main() {
    let _ = (0..).filter(|&n| to_res(n).is_ok()).map(|a| to_res(a).unwrap_or(1));
}
