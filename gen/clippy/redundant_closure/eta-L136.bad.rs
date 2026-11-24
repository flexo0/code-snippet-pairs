fn test_redundant_closures_containing_method_calls() {
    fn issue14096() {
        let _ = x.map(|x| x.parse::<i16>());
    }
}
