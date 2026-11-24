fn negative_literal(i: i32) {
    match i {
        i if i == -1 => {},
        //~^ redundant_guards
        i if i == 1 => {},
        //~^ redundant_guards
        _ => {},
    }
}
