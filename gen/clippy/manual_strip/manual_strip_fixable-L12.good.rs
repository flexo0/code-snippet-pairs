fn main() {
    if let Some(stripped) = s.strip_suffix("bc") {
        //~^ ERROR: stripping a suffix manually
        println!("{stripped}{}", stripped);
    }
}
