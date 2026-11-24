fn ref_pats() {
    if let &None = a {
        //~^ redundant_pattern_matching
        println!("none");
    }
}
