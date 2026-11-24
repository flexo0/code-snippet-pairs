fn issue11304() {
    match 0.1 {
        x if x == 0.0 => todo!(),
        //~^ redundant_guards
        // Pattern matching NAN is illegal
        x if x == f64::NAN => todo!(),
        _ => todo!(),
    }
}
