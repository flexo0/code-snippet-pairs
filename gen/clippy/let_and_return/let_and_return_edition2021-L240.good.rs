fn issue12801() {
    fn other_ops() {
        let _ = || {
            
            (if true { 2 } else { 3 } << 4)
            //~^ let_and_return
        };
    }
}
