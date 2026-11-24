fn main() {
    fn _f2(x: &String) -> &str {
        &**{ x }
    }
}
