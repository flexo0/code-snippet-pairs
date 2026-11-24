fn main() {
    let _ = Some(0).map(|x| unsafe { f(x) });
    let _ = Some(0).map(|x| unsafe { f(x) });
}
