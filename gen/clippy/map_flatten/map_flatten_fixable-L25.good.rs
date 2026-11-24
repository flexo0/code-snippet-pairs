fn main() {
    let _: Option<_> = (Some(Some(1))).and_then(|x| x);
}
