fn issue_8113() {
    for x in x[0].by_ref() {
        //~^ while_let_on_iterator
        println!("{}", x);
    }
}
