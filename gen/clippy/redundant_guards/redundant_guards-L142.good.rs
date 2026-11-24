fn negative_literal(i: i32) {
    match i {
        -1 => {},
        //~^ redundant_guards
        1 => {},
        //~^ redundant_guards
        _ => {},
    }
}
