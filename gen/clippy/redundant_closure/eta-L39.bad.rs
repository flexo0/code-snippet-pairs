fn main() {
    let d = Some(1u8).map(|a| foo((|b| foo2(b))(a))); //is adjusted?
}
