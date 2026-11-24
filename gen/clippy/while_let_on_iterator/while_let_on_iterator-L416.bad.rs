fn issue_8113() {
    while let Some(x) = x[0].next() {
        //~^ while_let_on_iterator
        println!("{}", x);
    }
}
