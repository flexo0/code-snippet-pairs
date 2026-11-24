fn issue12801() {
    fn other_ops() {
        let _ = || {
            let s = if true { 2 } else { 3 } << 4;
            s
            //~^ let_and_return
        };
    }
}
