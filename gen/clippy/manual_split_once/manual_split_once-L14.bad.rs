fn main() {
    let (_, _) = "key=value".splitn(2, '=').next_tuple().unwrap();
}
