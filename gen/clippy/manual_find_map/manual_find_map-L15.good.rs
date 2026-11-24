fn main() {
    let _ = (0..).find_map(|a| to_res(a).ok());
}
