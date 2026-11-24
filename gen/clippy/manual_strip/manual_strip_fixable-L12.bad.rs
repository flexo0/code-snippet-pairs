fn main() {
    if s.ends_with("bc") {
        let stripped = &s[..s.len() - "bc".len()];
        //~^ ERROR: stripping a suffix manually
        println!("{stripped}{}", &s[..s.len() - "bc".len()]);
    }
}
