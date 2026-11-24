fn mutable_closure_in_loop() {
    for _ in 0..5 {
        Some(1).map(&mut closure);
        //~^ redundant_closure

        let mut value = 0;
        let mut in_loop = |n| value += n;
        Some(1).map(in_loop);
        //~^ redundant_closure
    }
}
