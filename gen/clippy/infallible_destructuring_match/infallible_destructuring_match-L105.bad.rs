fn never_enum() {
    let data = match wrapper {
        //~^ infallible_destructuring_match
        Ok(i) => i,
    };
}
