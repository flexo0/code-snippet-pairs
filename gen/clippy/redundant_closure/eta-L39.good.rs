fn main() {
    let d = Some(1u8).map(|a| foo(foo2(a))); //is adjusted?
}
