fn main() {
    unsafe {
        let _ = Some(0).map(|x| f(x));
    }
}
