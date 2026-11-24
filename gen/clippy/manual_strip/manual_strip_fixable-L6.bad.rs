fn main() {
    if s.starts_with("ab") {
        let stripped = &s["ab".len()..];
        //~^ ERROR: stripping a prefix manually
        println!("{stripped}{}", &s["ab".len()..]);
    }
}
