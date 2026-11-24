fn main() {
    let _ = ["foo"].into_iter().map(|s| s == "foo").all(std::convert::identity);
}
