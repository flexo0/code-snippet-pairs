fn binop_with_macros() {
    (!has_is_empty.is_empty()).then(|| println!("This can happen."));
}
