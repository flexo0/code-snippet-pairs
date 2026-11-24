fn mutable_closure_in_loop() {
    for _ in 0..5 {
        Some(1).map(|n| closure(n));
        //~^ redundant_closure

        let mut value = 0;
        let mut in_loop = |n| value += n;
        Some(1).map(|n| in_loop(n));
        //~^ redundant_closure
    }
}
