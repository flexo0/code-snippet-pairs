fn ref_pats() {
    if a.is_none() {
        //~^ redundant_pattern_matching
        println!("none");
    }
}
