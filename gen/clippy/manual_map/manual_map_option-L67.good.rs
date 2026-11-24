fn main() {
    Some(String::new()).as_ref().map(|x| x.len());
}
