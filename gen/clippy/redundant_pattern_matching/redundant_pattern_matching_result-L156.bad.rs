fn issue10726() {
    match x {
        //~^ redundant_pattern_matching
        Ok(_) => true,
        _ => false,
    };
}
