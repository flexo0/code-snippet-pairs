fn main() {
    if let Some(stripped) = s.strip_prefix("ab") {
        //~^ ERROR: stripping a prefix manually
        println!("{stripped}{}", stripped);
    }
}
