fn main() {
    let _ = Some(14).filter(|&x| unsafe { f(x) });
    let _ = Some(15).filter(|&x| unsafe { f(x) });
}
