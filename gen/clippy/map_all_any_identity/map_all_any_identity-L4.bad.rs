fn main() {
    let _ = ["foo"].into_iter().map(|s| s == "foo").any(|a| a);
}
