fn main() {
    let _: Option<_> = (Some(Some(1))).map(|x| x).flatten();
}
