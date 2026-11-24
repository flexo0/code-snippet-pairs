fn in_parens() {
    if true {
        (if true {
            println!("In parens, linted");
        })
    }
}
