fn binop_with_macros() {
    (compare_to!(0) < has_is_empty.len()).then(|| println!("This can happen."));
}
