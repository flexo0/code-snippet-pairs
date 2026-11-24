fn refutable2() {
    {
        let v = vec![1, 2, 3];
        let mut it = v.windows(2);
        while let Some([x, y]) = it.next() {
            println!("x: {}", x);
            println!("y: {}", y);
        }

        let mut it = v.windows(2);
        while let Some([x, ..]) = it.next() {
            println!("x: {}", x);
        }

        let mut it = v.windows(2);
        while let Some([.., y]) = it.next() {
            println!("y: {}", y);
        }

        let mut it = v.windows(2);
        for [..] in it {}
        //~^ while_let_on_iterator

        let v = vec![[1], [2], [3]];
        let mut it = v.iter();
        while let Some([1]) = it.next() {}

        let mut it = v.iter();
        for [_x] in it {}
        //~^ while_let_on_iterator
    }
}
