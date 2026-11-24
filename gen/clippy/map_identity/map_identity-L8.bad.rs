fn main() {
    let _: Vec<_> = x.iter().map(not_identity).map(|x| return x).collect();
}
