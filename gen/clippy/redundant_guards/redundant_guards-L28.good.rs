fn issue11304() {
    match FloatWrapper(0.1) {
        FloatWrapper(0.0) => todo!(),
        //~^ redundant_guards
        _ => todo!(),
    }
}
